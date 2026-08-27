//! 主题发现、真实渲染预览与「试穿」。

use std::io::Write as _;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use crate::plugin::{custom_root, zsh_root, Source};

#[derive(Debug, Clone)]
pub struct ThemeInfo {
    pub name: String,
    pub path: PathBuf,
    pub source: Source,
}

/// 扫描内置与自定义主题目录,同名时自定义优先。
pub fn scan() -> Vec<ThemeInfo> {
    let mut out: Vec<ThemeInfo> = Vec::new();
    let roots = [
        (zsh_root().join("themes"), Source::Bundled),
        (custom_root().join("themes"), Source::Custom),
    ];
    for (root, source) in roots {
        let Ok(entries) = std::fs::read_dir(&root) else {
            continue;
        };
        for e in entries.filter_map(|e| e.ok()) {
            let path = e.path();
            if !path.is_file() || path.extension().map(|x| x != "zsh-theme").unwrap_or(true) {
                continue;
            }
            let Some(name) = path.file_stem().and_then(|s| s.to_str()) else {
                continue;
            };
            let name = name.to_string();
            let t = ThemeInfo {
                name: name.clone(),
                path,
                source,
            };
            match out.iter_mut().find(|t| t.name == name) {
                Some(slot) => *slot = t,
                None => out.push(t),
            }
        }
    }
    out.sort_by(|a, b| a.name.cmp(&b.name));
    out
}

/// 在受限 zsh 里 source 主题文件,用 `print -P` 把 PROMPT 展开为带 ANSI 色的文本。
/// 加 3 秒超时;失败返回 None。
pub fn preview_ansi(theme: &ThemeInfo) -> Option<String> {
    let zsh_root = zsh_root();
    let file = theme.path.display().to_string();
    let root = zsh_root.display().to_string();
    let script = format!(
        "setopt prompt_subst 2>/dev/null; \
         source '{root}/lib/git.zsh' 2>/dev/null; \
         source '{root}/lib/prompt_info_functions.zsh' 2>/dev/null; \
         source '{file}' 2>/dev/null; \
         print -P -- \"$PROMPT$RPROMPT\" 2>/dev/null"
    );
    run_with_timeout("zsh", &["-f", "-c", &script], Duration::from_secs(3))
}

/// 「试穿」:用临时 ZDOTDIR 起一个完整的交互 zsh(主题生效),退出后返回。
/// 直接操作终端(挂起 TUI → 交互 shell → 恢复 TUI)。
pub fn try_on(theme_name: &str) -> std::io::Result<()> {
    crate::ui::suspend_tui()?;

    let tmp = std::env::temp_dir().join(format!("omz-pm-tryon-{}", std::process::id()));
    let _ = std::fs::create_dir_all(&tmp);
    let rc = format!(
        "# omz-pm 主题试穿\nZSH_THEME='{}'\nsource \"$ZSH/oh-my-zsh.sh\"\n",
        theme_name.replace('\'', "'\\''")
    );
    std::fs::write(tmp.join(".zshrc"), rc)?;

    println!(
        "── 主题试穿「{}」(临时会话,exit 或 Ctrl+D 返回 omz-pm)──\n",
        theme_name
    );
    let status = Command::new("zsh")
        .arg("-i")
        .env("ZDOTDIR", &tmp)
        .env("ZSH", zsh_root())
        .status();
    let _ = std::fs::remove_dir_all(&tmp);

    println!("\n(试穿结束,返回管理界面…)");
    std::io::stdout().flush().ok();
    crate::ui::resume_tui()?;
    match status {
        Ok(s) if s.success() => Ok(()),
        Ok(s) => Err(std::io::Error::other(format!("shell 退出码: {}", s))),
        Err(e) => Err(e),
    }
}

/// 彩色闪现预览:挂起 TUI,把 preview_ansi 结果原样打印到真实终端,按键返回。
pub fn flash_preview(theme: &ThemeInfo) -> std::io::Result<()> {
    crate::ui::suspend_tui()?;
    match preview_ansi(theme) {
        Some(ansi) => {
            println!("「{}」提示符效果(上下各补一行便于观察):", theme.name);
            println!("\x1b[2m$\x1b[0m {}", ansi);
            println!("\x1b[2m$\x1b[0m");
        }
        None => {
            println!(
                "「{}」预览失败(主题依赖缺失或渲染超时),可尝试试穿查看。",
                theme.name
            );
        }
    }
    println!("\n按任意键返回…");
    std::io::stdout().flush().ok();
    let _ = Command::new("sh")
        .arg("-c")
        .arg("read -r -n1 _ </dev/tty")
        .status();
    crate::ui::resume_tui()?;
    Ok(())
}

fn run_with_timeout(prog: &str, args: &[&str], timeout: Duration) -> Option<String> {
    let mut child = Command::new(prog)
        .args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .ok()?;
    let start = Instant::now();
    loop {
        match child.try_wait() {
            Ok(Some(status)) if status.success() => {
                let mut out = String::new();
                use std::io::Read;
                if let Some(mut io) = child.stdout.take() {
                    let _ = io.read_to_string(&mut out);
                }
                let trimmed = out.trim_end();
                return if trimmed.is_empty() {
                    None
                } else {
                    Some(trimmed.to_string())
                };
            }
            Ok(Some(_)) => return None,
            Ok(None) => {
                if start.elapsed() > timeout {
                    let _ = child.kill();
                    return None;
                }
                std::thread::sleep(Duration::from_millis(25));
            }
            Err(_) => return None,
        }
    }
}

/// 去除 ANSI 转义序列(TUI 内纯文本预览用)。
pub fn strip_ansi(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\x1b' {
            // 跳过 ESC [ ... 终止字母
            if chars.peek() == Some(&'[') {
                chars.next();
                for c2 in chars.by_ref() {
                    if c2.is_ascii_alphabetic() {
                        break;
                    }
                }
            }
            continue;
        }
        out.push(c);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strip_ansi_basic() {
        assert_eq!(strip_ansi("\x1b[31mred\x1b[0m plain"), "red plain");
        assert_eq!(strip_ansi("a\x1b[1;32mbc"), "abc");
        assert_eq!(strip_ansi("no escapes"), "no escapes");
    }

    #[test]
    fn scan_finds_themes() {
        // CI 上没有安装 OMZ,跳过
        if !crate::plugin::zsh_root().join("themes").exists() {
            return;
        }
        let t = scan();
        assert!(t.len() > 50, "主题过少: {}", t.len());
        assert!(t.iter().any(|x| x.name == "robbyrussell"));
    }
}
