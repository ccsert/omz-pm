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

/// 扫描全部插件并按名称排序;同名时自定义覆盖内置(与 OMZ 加载行为一致)。
pub fn scan(enabled: &HashSet<String>) -> Vec<Plugin> {
    let mut plugins: Vec<Plugin> = Vec::new();

    let roots = [
        (zsh_root().join("plugins"), Source::Bundled),
        (custom_root().join("plugins"), Source::Custom),
    ];
    for (root, source) in roots {
        let Ok(entries) = fs::read_dir(&root) else {
            continue;
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
            let plugin = Plugin {
                enabled: enabled.contains(&name),
                name,
                source,
                dir: path,
            };
            // 自定义插件在 roots 迭代中后出现,覆盖同名内置插件
            if let Some(slot) = plugins.iter_mut().find(|p| p.name == plugin.name) {
                *slot = plugin;
            } else {
                plugins.push(plugin);
            }
        }
    }

    plugins.sort_by(|a, b| a.name.cmp(&b.name));
    plugins
}
