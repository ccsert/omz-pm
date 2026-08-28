//! README 阅读:优先显示内置整篇中文译文(`data/readmes-zh/` 打包嵌入),
//! 支持用户在 `~/.config/omz-pm/readmes-zh/<插件>.md` 覆盖;
//! 没有整篇译文的插件(含自定义插件)回退为「轻翻译」:对原文的小节标题、
//! 样板启用段做离线中文化。

use std::collections::HashMap;
use std::path::Path;
use std::sync::OnceLock;

use crate::catalog::find_readme_path;

/// 内置整篇中文译文,由 tools/build_readme_bundle.py 从 data/readmes-zh/ 打包生成。
const EMBEDDED_READMES: &str = include_str!("../data/readmes_zh.json");

static BUNDLED: OnceLock<HashMap<String, String>> = OnceLock::new();

fn bundled() -> &'static HashMap<String, String> {
    BUNDLED.get_or_init(|| serde_json::from_str(EMBEDDED_READMES).unwrap_or_default())
}

/// 用户整篇译文覆盖:~/.config/omz-pm/readmes-zh/<插件>.md。
fn user_override(name: &str) -> Option<String> {
    let config = match std::env::var("XDG_CONFIG_HOME") {
        Ok(d) if !d.is_empty() => Path::new(&d).to_path_buf(),
        _ => crate::zshrc::home_dir().join(".config"),
    };
    let path = config
        .join("omz-pm")
        .join("readmes-zh")
        .join(format!("{name}.md"));
    std::fs::read_to_string(path)
        .ok()
        .filter(|t| !t.trim().is_empty())
}

/// 常见小节标题词典(命中即中文化,保留原文在括号里)。
const HEADERS: &[(&str, &str)] = &[
    ("aliases", "别名"),
    ("alias", "别名"),
    ("functions", "函数"),
    ("function", "函数"),
    ("usage", "用法"),
    ("options", "选项"),
    ("installation", "安装"),
    ("install", "安装"),
    ("requirements", "依赖要求"),
    ("requirement", "依赖要求"),
    ("configuration", "配置"),
    ("config", "配置"),
    ("settings", "设置"),
    ("customization", "自定义"),
    ("customizing", "自定义"),
    ("key bindings", "键位绑定"),
    ("keybindings", "键位绑定"),
    ("bindings", "键位绑定"),
    ("widgets", "控件"),
    ("completions", "补全"),
    ("completion", "补全"),
    ("commands", "命令"),
    ("features", "特性"),
    ("examples", "示例"),
    ("example", "示例"),
    ("troubleshooting", "故障排查"),
    ("faq", "常见问题"),
    ("notes", "注意"),
    ("changelog", "更新日志"),
    ("credits", "致谢"),
    ("getting started", "快速开始"),
    ("setup", "设置"),
    ("development", "开发"),
    ("testing", "测试"),
    ("styles", "样式"),
    ("colors", "配色"),
    ("variables", "变量"),
    ("reference", "参考"),
    ("see also", "另见"),
    ("themes", "主题"),
    ("license", "许可证"),
    ("contributing", "参与贡献"),
];

/// 阅读 README:优先用户覆盖译文,其次内置整篇译文,
/// 都没有时读取原文件做轻翻译。
pub fn read_translated(plugin_name: &str, dir: &Path) -> Option<String> {
    if let Some(t) = user_override(plugin_name) {
        return Some(t);
    }
    if let Some(t) = bundled().get(plugin_name) {
        return Some(t.clone());
    }
    let path = find_readme_path(dir)?;
    let text = std::fs::read_to_string(path).ok()?;
    Some(translate(plugin_name, &text))
}

pub fn translate(plugin_name: &str, text: &str) -> String {
    let mut out: Vec<String> = Vec::new();
    let mut in_fence = false;
    let mut skip_example = false;
    let enable_note = format!(
        "✅ 启用方式:把「{}」加入 zshrc 的 plugins 数组(可用 omz-pm 一键完成)。",
        plugin_name
    );

    for line in text.lines() {
        let t = line.trim();
        if t.starts_with("```") {
            in_fence = !in_fence;
            if skip_example {
                // 示例代码块的结束围栏也吞掉
                if !in_fence {
                    skip_example = false;
                }
                continue;
            }
            out.push(line.to_string());
            continue;
        }
        if skip_example {
            continue;
        }
        if in_fence {
            out.push(line.to_string());
            continue;
        }

        // 样板「To use it, add ... to the plugins array」等启用说明段落
        let lower = t.to_lowercase();
        let is_enable_para = lower.contains("plugins array")
            || lower.contains("list of plugins")
            || ((lower.contains("to use it") || lower.contains("to use,"))
                && lower.contains("add"));
        if is_enable_para && (lower.contains("add") || lower.contains("enable")) {
            out.push(enable_note.clone());
            // 后面若紧跟示例代码块则整块略过
            skip_example = true;
            continue;
        }
        // 依赖说明行
        if lower.starts_with("**requires") || lower.starts_with("requires") {
            let rest = t
                .trim_start_matches("**Requires:**")
                .trim_start_matches("**requires**")
                .trim_start_matches("Requires:")
                .trim_start_matches("requires:")
                .trim();
            out.push(format!("📌 依赖:{}", rest));
            continue;
        }

        // 小节标题翻译
        if t.starts_with('#') {
            let level = t.chars().take_while(|c| *c == '#').count();
            let hv = t[level..].trim();
            let translated = translate_header(hv);
            out.push(format!("{} {}", "#".repeat(level), translated));
            continue;
        }

        out.push(line.to_string());
    }
    out.join("\n")
}

fn translate_header(h: &str) -> String {
    let key = h.trim().trim_end_matches(':').to_lowercase();
    for (en, zh) in HEADERS {
        if *en == key {
            return format!("{}({})", zh, h.trim());
        }
    }
    h.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bundled_readmes_parse_and_cover_dictionary() {
        let dict: HashMap<String, serde_json::Value> =
            serde_json::from_str(include_str!("../data/translations.json")).unwrap();
        assert!(dict.len() >= 300, "词典条目异常: {}", dict.len());
        let b = bundled();
        let missing: Vec<&String> = dict.keys().filter(|k| !b.contains_key(*k)).collect();
        assert!(missing.is_empty(), "缺整篇译文的插件: {:?}", missing);
    }

    #[test]
    fn bundled_readmes_are_complete_translations() {
        for (name, text) in bundled() {
            assert!(!text.trim().is_empty(), "{} 译文为空", name);
            // 按行数围栏(6 反引号行会被子串计数多数一次)
            let fences = text
                .lines()
                .filter(|l| l.trim_start().starts_with("```"))
                .count();
            assert!(fences % 2 == 0, "{} 代码围栏不闭合", name);
            assert!(
                !text.contains("plugins array"),
                "{} 遗留英文样板启用段",
                name
            );
        }
    }

    #[test]
    fn header_translated() {
        let r = translate("x", "## Aliases\n\n| `g` | `git` |\n");
        assert!(r.contains("## 别名(Aliases)"), "{}", r);
        assert!(r.contains("| `g` | `git` |"), "表格应保留");
    }

    #[test]
    fn boilerplate_replaced() {
        let r = translate(
            "sudo",
            "To use it, add `sudo` to the plugins array in your zshrc file:\n\n```zsh\nplugins=(... sudo)\n```\n\n## Options\n",
        );
        assert!(r.contains("✅ 启用方式"), "{}", r);
        assert!(
            !r.contains("plugins=(... sudo)"),
            "示例代码块应被略过: {}",
            r
        );
        assert!(r.contains("## 选项(Options)"));
    }

    #[test]
    fn requires_translated() {
        let r = translate("sfffe", "**Requires:** `ack`\n");
        assert!(r.contains("📌 依赖:`ack`"), "{}", r);
    }

    #[test]
    fn unknown_header_kept() {
        let r = translate("x", "## My Weird Header\n");
        assert!(r.contains("My Weird Header"));
    }

    #[test]
    fn fence_content_untouched() {
        let r = translate("x", "```zsh\n# To use it, add nothing\n```\n");
        assert!(r.contains("# To use it, add nothing"));
    }
}
