//! 中文词典:编译期嵌入全部内置插件的中文说明,
//! 支持用户在 `~/.config/omz-pm/translations.json` 覆盖/补充;
//! 词典之外的插件回退显示 README 英文摘要。

use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::path::Path;

use serde::Deserialize;

use crate::zshrc;

const EMBEDDED: &str = include_str!("../data/translations.json");

#[derive(Debug, Clone, Deserialize)]
pub struct Entry {
    /// 一句话中文简介(用于列表列)
    pub summary: String,
    /// 功能详情(用于详情面板)
    #[serde(default)]
    pub detail: String,
    /// 分类标签(如「目录与跳转」)
    #[serde(default)]
    pub cat: String,
    /// 实战用法指南(启用后怎么用)
    #[serde(default)]
    pub usage: String,
    /// 精选别名中文注解:别名 -> 一句解释
    #[serde(default)]
    pub aliases: BTreeMap<String, String>,
}

impl Entry {
    /// 分类为空时给个兜底显示。
    pub fn category(&self) -> &str {
        if self.cat.is_empty() {
            "其他工具"
        } else {
            &self.cat
        }
    }
}

#[derive(Debug, Default)]
pub struct Catalog {
    map: HashMap<String, Entry>,
}

impl Catalog {
    /// 加载内置词典并合并用户覆盖文件(若存在)。
    pub fn load() -> Catalog {
        let mut map: HashMap<String, Entry> = serde_json::from_str(EMBEDDED).unwrap_or_default();
        let user_path = user_overrides_path();
        if let Ok(text) = fs::read_to_string(&user_path) {
            match serde_json::from_str::<HashMap<String, Entry>>(&text) {
                Ok(overrides) => {
                    for (k, v) in overrides {
                        map.insert(k, v);
                    }
                }
                Err(e) => eprintln!("警告: {} 格式错误,已忽略: {}", user_path.display(), e),
            }
        }
        Catalog { map }
    }

    pub fn get(&self, name: &str) -> Option<&Entry> {
        self.map.get(name)
    }

    #[allow(dead_code)] // 测试与后续功能使用
    pub fn len(&self) -> usize {
        self.map.len()
    }

    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    /// 出现过的分类(按插件数量降序),供 TUI 分类筛选循环。
    pub fn categories(&self) -> Vec<String> {
        let mut counts: HashMap<&str, usize> = HashMap::new();
        for e in self.map.values() {
            *counts.entry(e.category()).or_insert(0) += 1;
        }
        let mut v: Vec<(String, usize)> = counts
            .into_iter()
            .map(|(c, n)| (c.to_string(), n))
            .collect();
        v.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
        v.into_iter().map(|(c, _)| c).collect()
    }
}

fn user_overrides_path() -> std::path::PathBuf {
    if let Ok(dir) = std::env::var("XDG_CONFIG_HOME") {
        if !dir.is_empty() {
            return Path::new(&dir).join("omz-pm").join("translations.json");
        }
    }
    zshrc::home_dir()
        .join(".config")
        .join("omz-pm")
        .join("translations.json")
}

/// 从插件目录提取 README 英文摘要:跳过标题/代码块,取第一段正文。
pub fn readme_excerpt(dir: &Path) -> Option<String> {
    let readme = find_readme(dir)?;
    let text = fs::read_to_string(readme).ok()?;
    let mut paragraph: Vec<String> = Vec::new();
    let mut in_fence = false;
    for line in text.lines() {
        let t = line.trim();
        if t.starts_with("```") {
            in_fence = !in_fence;
            continue;
        }
        if in_fence {
            continue;
        }
        if t.is_empty() {
            if !paragraph.is_empty() {
                break; // 第一段收集完毕
            }
            continue;
        }
        if t.starts_with('#') {
            // 标题行:若已有正文则段落结束,否则跳过文件名式标题
            if !paragraph.is_empty() {
                break;
            }
            continue;
        }
        paragraph.push(t.to_string());
        if paragraph.join(" ").len() > 700 {
            break;
        }
    }
    let mut s = paragraph.join(" ");
    // 折叠多余空白
    s = s.split_whitespace().collect::<Vec<_>>().join(" ");
    if s.is_empty() {
        None
    } else {
        Some(s)
    }
}

fn find_readme(dir: &Path) -> Option<std::path::PathBuf> {
    find_readme_path(dir)
}

/// 按常见命名查找插件目录里的 README 文件。
pub fn find_readme_path(dir: &Path) -> Option<std::path::PathBuf> {
    const NAMES: [&str; 6] = [
        "README.md",
        "readme.md",
        "Readme.md",
        "README.markdown",
        "README.MD",
        "README",
    ];
    for n in NAMES {
        let p = dir.join(n);
        if p.is_file() {
            return Some(p);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn embedded_catalog_parses_and_covers_all_bundled() {
        let c = Catalog::load();
        assert!(c.len() >= 300, "词典条目不足: {}", c.len());
        for (_, e) in c.map.iter() {
            assert!(!e.summary.is_empty(), "summary 不能为空");
        }
    }

    #[test]
    fn embedded_catalog_has_categories_and_usage() {
        let c = Catalog::load();
        assert!(!c.categories().is_empty());
        let with_usage = c.map.values().filter(|e| !e.usage.is_empty()).count();
        assert!(with_usage >= 40, "usage 条目过少: {}", with_usage);
        let git = c.get("git").unwrap();
        assert_eq!(git.category(), "版本控制");
        assert!(git.aliases.contains_key("gst"), "git 缺 gst 注解");
    }

    #[test]
    fn every_bundled_plugin_has_translation() {
        // CI 上没有安装 OMZ,跳过
        if !crate::plugin::zsh_root().join("plugins").exists() {
            return;
        }
        let c = Catalog::load();
        let plugins = crate::plugin::scan(&Default::default());
        let missing: Vec<&str> = plugins
            .iter()
            .filter(|p| p.source == crate::plugin::Source::Bundled)
            .filter(|p| c.get(&p.name).is_none())
            .map(|p| p.name.as_str())
            .collect();
        assert!(missing.is_empty(), "缺少翻译的插件: {:?}", missing);
    }
}
