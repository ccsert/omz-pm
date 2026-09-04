//! 插件发现:扫描 OMZ 内置目录与自定义目录,合并去重。

use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use crate::zshrc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Source {
    Bundled,
    Custom,
}

impl Source {
    pub fn label(self) -> &'static str {
        match self {
            Source::Bundled => "内置",
            Source::Custom => "自定义",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Plugin {
    pub name: String,
    pub source: Source,
    /// 插件所在目录(用于读取 README)
    pub dir: PathBuf,
    pub enabled: bool,
}

/// OMZ 根目录:`$ZSH`,缺省 `~/.oh-my-zsh`。
pub fn zsh_root() -> PathBuf {
    env::var("ZSH")
        .ok()
        .filter(|s| !s.is_empty())
        .map(PathBuf::from)
        .unwrap_or_else(|| zshrc::home_dir().join(".oh-my-zsh"))
}

/// 自定义插件目录:`$ZSH_CUSTOM`,缺省 `$ZSH/custom`。
pub fn custom_root() -> PathBuf {
    env::var("ZSH_CUSTOM")
        .ok()
        .filter(|s| !s.is_empty())
        .map(PathBuf::from)
        .unwrap_or_else(|| zsh_root().join("custom"))
}

/// 目录算作插件的两类形态:
/// 1) 含 `*.plugin.zsh`(脚本型,OMZ 会 source);
/// 2) 仅含 `_*` 补全文件(纯补全型,OMZ 把目录加入 fpath 加载)。
fn is_plugin_dir(dir: &Path) -> bool {
    let Ok(entries) = fs::read_dir(dir) else {
        return false;
    };
    entries.filter_map(|e| e.ok()).any(|e| {
        let name = e.file_name().to_string_lossy().to_string();
        name.ends_with(".plugin.zsh") || (name.starts_with('_') && name != "_")
    })
}

/// 扫描单个目录下的插件(不排序)。
fn scan_in_dir(root: &Path, source: Source, enabled: &HashSet<String>) -> Vec<Plugin> {
    let mut out: Vec<Plugin> = Vec::new();
    let Ok(entries) = fs::read_dir(root) else {
        return out;
    };
    for entry in entries.filter_map(|e| e.ok()) {
        let path = entry.path();
        if !path.is_dir() || !is_plugin_dir(&path) {
            continue;
        }
        let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
            continue;
        };
        let name = name.to_string();
        out.push(Plugin {
            enabled: enabled.contains(&name),
            name,
            source,
            dir: path,
        });
    }
    out
}

/// 扫描全部插件并按名称排序;同名时自定义覆盖内置(与 OMZ 加载行为一致)。
pub fn scan(enabled: &HashSet<String>) -> Vec<Plugin> {
    let mut plugins = scan_in_dir(&zsh_root().join("plugins"), Source::Bundled, enabled);
    // 自定义插件后扫描,覆盖同名内置插件
    for c in scan_in_dir(&custom_root().join("plugins"), Source::Custom, enabled) {
        match plugins.iter_mut().find(|p| p.name == c.name) {
            Some(slot) => *slot = c,
            None => plugins.push(c),
        }
    }
    plugins.sort_by(|a, b| a.name.cmp(&b.name));
    plugins
}

/// 启用集中存在、但磁盘上没有对应插件目录的名字(改名/删除/拼错的残留)。
/// 这些条目只会拖慢 zsh 启动并可能报错,可整体从 zshrc 移除。
pub fn ghost_names(enabled: &HashSet<String>, plugins: &[Plugin]) -> Vec<String> {
    let mut ghosts: Vec<String> = enabled
        .iter()
        .filter(|n| !plugins.iter().any(|p| &p.name == *n))
        .cloned()
        .collect();
    ghosts.sort();
    ghosts
}

#[cfg(test)]
mod tests {
    use super::*;

    fn plugin(name: &str) -> Plugin {
        Plugin {
            name: name.to_string(),
            source: Source::Bundled,
            dir: PathBuf::from("/tmp/does-not-matter"),
            enabled: true,
        }
    }

    #[test]
    fn scan_requires_plugin_marker_files() {
        // is_plugin_dir 只认 *.plugin.zsh 与 _ 补全文件,普通目录不算
        let dir = std::env::temp_dir().join(format!("omz-pm-scan-{}", std::process::id()));
        fs::create_dir_all(dir.join("real-plugin")).unwrap();
        fs::write(dir.join("real-plugin/real.plugin.zsh"), "# t\n").unwrap();
        fs::create_dir_all(dir.join("empty-dir")).unwrap();
        let enabled: HashSet<String> = Default::default();
        let found = scan_in_dir(&dir, Source::Bundled, &enabled);
        let names: Vec<&str> = found.iter().map(|p| p.name.as_str()).collect();
        assert_eq!(names, vec!["real-plugin"]);
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn ghost_names_finds_missing() {
        let enabled: HashSet<String> = ["git", "phantom", "gone"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let plugins = vec![plugin("git")];
        assert_eq!(ghost_names(&enabled, &plugins), vec!["gone", "phantom"]);
    }

    #[test]
    fn ghost_names_empty_when_all_present() {
        let enabled: HashSet<String> = ["git"].iter().map(|s| s.to_string()).collect();
        let plugins = vec![plugin("git")];
        assert!(ghost_names(&enabled, &plugins).is_empty());
    }
}
