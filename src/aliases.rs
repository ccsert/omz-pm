//! 从插件源码自动提取 `alias` 定义,构成「别名 → 命令」索引,
//! 供详情面板展示与 `omz-pm which` 反查。

use std::fs;
use std::path::Path;

#[derive(Debug, Clone, PartialEq)]
pub struct AliasDef {
    pub name: String,
    pub command: String,
}

/// 扫描插件目录下所有 .zsh 文件,按文件名序提取别名定义(同名后者覆盖)。
pub fn extract_from_dir(dir: &Path) -> Vec<AliasDef> {
    let mut files: Vec<_> = fs::read_dir(dir)
        .into_iter()
        .flatten()
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.is_file() && p.extension().map(|x| x == "zsh").unwrap_or(false))
        .collect();
    files.sort();
    let mut all = Vec::new();
    for f in files {
        if let Ok(text) = fs::read_to_string(&f) {
            all.extend(extract_from_source(&text));
        }
    }
    dedup(all)
}

/// 同名只保留最后一次定义,位置取首次出现处。
fn dedup(defs: Vec<AliasDef>) -> Vec<AliasDef> {
    let mut out: Vec<AliasDef> = Vec::new();
    for d in defs {
        match out.iter_mut().find(|e| e.name == d.name) {
            Some(slot) => *slot = d,
            None => out.push(d),
        }
    }
    out
}

/// 从单份源码文本提取 `alias name=value` 定义。
/// 支持一行多条、单/双引号、无引号;忽略函数内条件定义与 `alias name`(查询形式)。
pub fn extract_from_source(src: &str) -> Vec<AliasDef> {
    let mut out = Vec::new();
    for line in src.lines() {
        let t = line.trim_start();
        // 跳过注释行与非 alias 行
        if !(t.starts_with("alias ") || t.starts_with("alias\t") || t == "alias") {
            continue;
        }
        let rest = t["alias".len()..].trim_start();
        let mut segs = split_top(rest);
        // 跳过前置 flag(如 alias -g、alias --force),与 zsh 行为一致
        while segs
            .first()
            .map(|s| s.starts_with('-') && !s.contains('='))
            .unwrap_or(false)
        {
            segs.remove(0);
        }
        if segs.len() % 2 != 0 {
            continue; // 不完整
        }
        while !segs.is_empty() {
            let name = segs.remove(0);
            let value = segs.remove(0);
            if name.is_empty() || value.is_empty() {
                continue;
            }
            out.push(AliasDef {
                name,
                command: value,
            });
        }
    }
    dedup(out)
}

/// 把 alias 参数流切分成 name value name value … 序列。
/// 切分规则:空格分隔;引号内的空格不切;value 是 `=` 后到下一个
/// "顶层空格"之前的完整串(去引号)。
fn split_top(s: &str) -> Vec<String> {
    let ch: Vec<char> = s.chars().collect();
    let mut segs: Vec<String> = Vec::new();
    let mut cur = String::new();
    let mut i = 0usize;
    while i < ch.len() {
        match ch[i] {
            '\'' => {
                i += 1;
                while i < ch.len() && ch[i] != '\'' {
                    cur.push(ch[i]);
                    i += 1;
                }
                // 收尾的 i += 1 跳过闭引号
            }
            '"' => {
                i += 1;
                while i < ch.len() && ch[i] != '"' {
                    if ch[i] == '\\'
                        && i + 1 < ch.len()
                        && matches!(ch[i + 1], '"' | '\\' | '$' | '`')
                    {
                        cur.push(ch[i + 1]);
                        i += 2;
                        continue;
                    }
                    cur.push(ch[i]);
                    i += 1;
                }
            }
            '\\' => {
                if i + 1 < ch.len() {
                    cur.push(ch[i + 1]);
                    i += 1;
                }
            }
            ' ' | '\t' => {
                if !cur.is_empty() {
                    segs.push(std::mem::take(&mut cur));
                }
            }
            c => cur.push(c),
        }
        i += 1;
    }
    if !cur.is_empty() {
        segs.push(cur);
    }

    // 从流式段组装 name/value 对
    let mut out: Vec<String> = Vec::new();
    for seg in &segs {
        match seg.split_once('=') {
            Some((n, v)) => {
                out.push(n.to_string());
                out.push(v.to_string());
            }
            None => {
                // 查询形式 `alias foo`(无 =),用空对占位后由调用方过滤
                out.push(String::new());
                out.push(String::new());
            }
        }
    }
    out
}

/// 反查:在全部插件别名索引中找 token。
pub fn build_index(plugins: &[crate::plugin::Plugin]) -> Vec<(String, String, AliasDef)> {
    let mut idx = Vec::new();
    for p in plugins {
        for d in extract_from_dir(&p.dir) {
            idx.push((d.name.clone(), p.name.clone(), d));
        }
    }
    idx
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_quoted() {
        let a = extract_from_source("alias gst='git status'\n");
        assert_eq!(
            a,
            vec![AliasDef {
                name: "gst".into(),
                command: "git status".into()
            }]
        );
    }

    #[test]
    fn double_quoted_and_escapes() {
        let a = extract_from_source("alias gp=\"git push \\\"origin\\\"\"\n");
        assert_eq!(a[0].name, "gp");
        assert_eq!(a[0].command, "git push \"origin\"");
    }

    #[test]
    fn quoted_values_keep_spaces() {
        let a = extract_from_source("alias ll='ls -G'\nalias which-command=whence\n");
        assert_eq!(a[0].name, "ll");
        assert_eq!(a[0].command, "ls -G");
        assert_eq!(a[1].name, "which-command");
        assert_eq!(a[1].command, "whence");
    }

    #[test]
    fn trailing_flag_dropped_like_zsh() {
        // zsh 中 `alias ll=ls -G` 只定义 ll=ls,-G 被当作另一个名字参数
        let a = extract_from_source("alias ll=ls -G\n");
        assert_eq!(a.len(), 1);
        assert_eq!(a[0].command, "ls");
    }

    #[test]
    fn multiple_on_one_line() {
        let a = extract_from_source("  alias -g L='| less' G='| grep'\n");
        assert_eq!(a.len(), 2);
        assert_eq!(a[0].name, "L");
        assert_eq!(a[0].command, "| less");
        assert_eq!(a[1].name, "G");
        assert_eq!(a[1].command, "| grep");
    }

    #[test]
    fn query_form_ignored() {
        let a = extract_from_source("alias gst\nalias g='git'\n");
        assert_eq!(a.len(), 1);
        assert_eq!(a[0].name, "g");
    }

    #[test]
    fn comment_and_indented_ok() {
        let a = extract_from_source("# alias not='real'\n\talias real='yes'\n");
        assert_eq!(a.len(), 1);
        assert_eq!(a[0].name, "real");
    }

    #[test]
    fn dedup_keeps_last() {
        let a = extract_from_source("alias x='one'\nalias x='two'\n");
        assert_eq!(a.len(), 1);
        assert_eq!(a[0].command, "two");
    }
}
