//! 安全读取与改写 zshrc 中的 `plugins=(…)` 声明。
//!
//! 支持单行/多行写法、行内注释、引号包裹的元素。改写时只替换
//! `plugins=` 到闭括号之间的行,文件其余部分(含前后注释)原样保留,
//! 写入前由调用方做时间戳备份。

use std::collections::HashSet;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

/// 一处 `plugins=(…)`(或 `plugins+=(…)`)声明。
#[derive(Debug, Clone)]
pub struct PluginsDecl {
    /// `plugins=` 所在行(0 起)
    pub start_line: usize,
    /// 闭括号所在行(0 起)
    pub end_line: usize,
    /// 是否为追加写法 `plugins+=(…)`(暂未参与改写逻辑,保留供诊断)
    #[allow(dead_code)]
    pub is_append: bool,
    /// 数组元素(按出现顺序,已去引号;可能含 `$var` 等非插件 token)
    pub items: Vec<String>,
}

/// 解析结果之外还需要给改写器用的私有信息。
struct DeclSpans {
    decl: PluginsDecl,
    /// 起始行从行首到 `(`(含)的字节前缀
    open_prefix: String,
    /// 闭括号之后到行尾的内容(通常是注释或空白)
    close_suffix: String,
}

/// 找到一行中 `plugins=(` 或 `plugins+=(` 闭括号后一个字节的字节下标。
fn find_plugins_open(line: &str) -> Option<(usize, bool)> {
    let trimmed_start = line.trim_start();
    if !trimmed_start.starts_with("plugins") {
        return None;
    }
    let after_name = &trimmed_start["plugins".len()..];
    let (eq_part, is_append) = if let Some(rest) = after_name.strip_prefix("+=") {
        (rest, true)
    } else {
        let rest = after_name.trim_start().strip_prefix('=')?;
        (rest, false)
    };
    let after_eq = eq_part.trim_start();
    let paren = after_eq.strip_prefix('(')?;
    let open_byte = line.len() - paren.len(); // `(` 之后一个字节的下标
    Some((open_byte, is_append))
}

/// 判断 token 是否可以不加引号直接写回。
fn is_bare_token(s: &str) -> bool {
    !s.is_empty()
        && s.chars()
            .all(|c| c.is_ascii_alphanumeric() || "._:@/+%-".contains(c))
}

/// 解析 content 中所有完整的 plugins 声明(不完整/未闭合的会被跳过)。
fn parse_spans(content: &str) -> Vec<DeclSpans> {
    let lines: Vec<&str> = content.lines().collect();
    let mut decls = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        let Some((open_byte, is_append)) = find_plugins_open(line) else {
            continue;
        };
        // 从 ( 之后开始逐字符扫描,跨行,直到 ) 或 EOF。
        let mut items: Vec<String> = Vec::new();
        let mut cur = String::new();
        let mut in_single = false;
        let mut in_double = false;
        let mut in_comment = false;
        let mut cur_has_quote = false;
        let mut end_line = None;
        let mut close_suffix = String::new();
        // 用 (行号, 字节列) 双游标扫描
        let mut li = i;
        let mut ci = open_byte;
        'scan: while li < lines.len() {
            let l = lines[li];
            let mut chars = l[ci.min(l.len())..].char_indices().peekable();
            while let Some((byte_off, c)) = chars.next() {
                let _ = byte_off;
                if in_comment {
                    if c == '\n' {
                        in_comment = false;
                    }
                    continue;
                }
                if in_single {
                    if c == '\'' {
                        in_single = false;
                    } else {
                        cur.push(c);
                    }
                    continue;
                }
                if in_double {
                    match c {
                        '"' => in_double = false,
                        '\\' => {
                            if let Some(&(_, next)) = chars.peek() {
                                cur.push(next);
                                chars.next();
                            }
                        }
                        _ => cur.push(c),
                    }
                    continue;
                }
                match c {
                    '\'' => {
                        in_single = true;
                        cur_has_quote = true;
                    }
                    '"' => {
                        in_double = true;
                        cur_has_quote = true;
                    }
                    '#' if cur.is_empty() => in_comment = true,
                    ')' => {
                        if !cur.is_empty() {
                            items.push(std::mem::take(&mut cur));
                        }
                        let close_col = ci + byte_off + 1;
                        close_suffix = l[close_col..].to_string();
                        end_line = Some(li);
                        break 'scan;
                    }
                    c if c.is_whitespace() => {
                        if !cur.is_empty() {
                            items.push(std::mem::take(&mut cur));
                        }
                    }
                    _ => cur.push(c),
                }
            }
            // 行结束:换行符等价于空白(数组元素分隔符)
            if !cur.is_empty() && !in_single && !in_double {
                items.push(std::mem::take(&mut cur));
            }
            let _ = cur_has_quote; // 引号信息仅用于去引号后的裸 token 判断
            cur_has_quote = false;
            li += 1;
            ci = 0;
            in_comment = false; // 注释不跨行
        }
        if let Some(end) = end_line {
            decls.push(DeclSpans {
                decl: PluginsDecl {
                    start_line: i,
                    end_line: end,
                    is_append,
                    items,
                },
                open_prefix: line[..open_byte].to_string(),
                close_suffix,
            });
        }
    }
    decls
}

/// 解析 content 中所有 plugins 声明(公开只读接口)。
pub fn parse_decls(content: &str) -> Vec<PluginsDecl> {
    parse_spans(content).into_iter().map(|s| s.decl).collect()
}

/// 读取生效的插件名集合(所有声明按顺序拼接,后声明覆盖同名)。
/// 返回 (集合, 声明个数)。
pub fn read_enabled(content: &str) -> (HashSet<String>, usize) {
    let decls = parse_decls(content);
    let mut set = HashSet::new();
    for d in &decls {
        for item in &d.items {
            if !item.starts_with('$') {
                set.insert(item.clone());
            }
        }
    }
    let n = decls.len();
    (set, n)
}

/// 把 items 序列化为数组元素字符串。
fn render_items(items: &[String]) -> String {
    items
        .iter()
        .map(|it| {
            if is_bare_token(it) {
                it.clone()
            } else {
                format!("'{}'", it.replace('\'', "'\\''"))
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// 序列化一个声明为多行文本。
fn render_decl_lines(
    open_prefix: &str,
    items: &[String],
    multiline: bool,
    item_indent: &str,
    close_indent: &str,
    close_suffix: &str,
) -> Vec<String> {
    if !multiline {
        return vec![format!(
            "{}{}){}",
            open_prefix,
            render_items(items),
            close_suffix
        )];
    }
    let mut out = vec![open_prefix.trim_end().to_string()];
    for it in items {
        out.push(format!(
            "{}{}",
            item_indent,
            render_items(std::slice::from_ref(it))
        ));
    }
    out.push(format!("{}){}", close_indent, close_suffix));
    out
}

/// 对 zshrc 内容应用启用/禁用变更,返回 (新内容, 警告列表)。
///
/// 规则:受影响的每个声明就地重写 —— 禁用的从中移除,
/// 新启用的追加到最后一个声明末尾;其余内容与缩进风格原样保留。
pub fn apply_changes(
    content: &str,
    enable: &[String],
    disable: &[String],
) -> Result<(String, Vec<String>), String> {
    let spans = parse_spans(content);
    let mut warnings = Vec::new();

    // 全部声明合并后的生效集合
    let effective: HashSet<String> = spans
        .iter()
        .flat_map(|s| s.decl.items.iter().cloned())
        .filter(|i| !i.starts_with('$'))
        .collect();

    let mut delta_enable: Vec<&str> = Vec::new();
    for name in enable {
        if effective.contains(name) {
            warnings.push(format!("「{}」已经启用,无需重复添加", name));
        } else if name.starts_with('$') {
            warnings.push(format!("跳过变量形式的条目「{}」,请手动处理", name));
        } else {
            delta_enable.push(name);
        }
    }

    // 计算每个声明的新列表
    let lines: Vec<&str> = content.lines().collect();
    let n_decls = spans.len();
    struct Replacement {
        start: usize,
        end: usize,
        new_lines: Vec<String>,
    }
    let mut replacements: Vec<Replacement> = Vec::new();
    let mut pending_adds: Vec<&str> = delta_enable.clone();

    for (di, s) in spans.iter().enumerate() {
        let removed: HashSet<&String> = disable.iter().collect();
        let remaining: Vec<String> = s
            .decl
            .items
            .iter()
            .filter(|it| !removed.contains(*it))
            .cloned()
            .collect();

        // 新启用的插件追加到最后一个声明
        let mut final_items = remaining;
        if di + 1 == n_decls {
            for name in std::mem::take(&mut pending_adds) {
                final_items.push(name.to_string());
            }
        }
        if final_items.len() == s.decl.items.len() && final_items == s.decl.items {
            continue; // 该声明无变化
        }
        let multiline = s.decl.start_line != s.decl.end_line;
        let item_indent = if multiline {
            lines[s.decl.start_line + 1..s.decl.end_line]
                .iter()
                .find(|l| !l.trim().is_empty())
                .map(|l| l[..l.len() - l.trim_start().len()].to_string())
                .unwrap_or_else(|| "  ".to_string())
        } else {
            String::new()
        };
        let close_indent = if multiline {
            let open_line = lines[s.decl.start_line];
            open_line[..open_line.len() - open_line.trim_start().len()].to_string()
        } else {
            String::new()
        };
        replacements.push(Replacement {
            start: s.decl.start_line,
            end: s.decl.end_line,
            new_lines: render_decl_lines(
                &s.open_prefix,
                &final_items,
                multiline,
                &item_indent,
                &close_indent,
                &s.close_suffix,
            ),
        });
    }

    for name in disable {
        if !effective.contains(name) && !name.starts_with('$') {
            warnings.push(format!("「{}」本来就没有启用,无需禁用", name));
        } else if name.starts_with('$') {
            warnings.push(format!("无法移除变量形式的条目「{}」,请手动处理", name));
        }
    }

    // 无任何 plugins 声明且需要启用时,在文件末尾追加
    let new_content = if spans.is_empty() {
        if delta_enable.is_empty() {
            content.to_string()
        } else {
            let items: Vec<String> = delta_enable.iter().map(|s| s.to_string()).collect();
            let rendered = render_items(&items);
            let mut lns: Vec<String> = content.lines().map(|l| l.to_string()).collect();
            if !lns.is_empty() && !lns.last().map(|l| l.trim().is_empty()).unwrap_or(true) {
                lns.push(String::new());
            }
            lns.push("# 由 omz-pm 添加".to_string());
            lns.push(format!("plugins=({})", rendered));
            let mut out = lns.join("\n");
            if content.ends_with('\n') && !out.ends_with('\n') {
                out.push('\n');
            }
            out
        }
    } else if replacements.is_empty() {
        content.to_string()
    } else {
        // 从后往前替换,避免行号偏移
        replacements.sort_by_key(|r| r.start);
        let mut new_lines: Vec<String> = lines.iter().map(|l| l.to_string()).collect();
        for r in replacements.into_iter().rev() {
            let n = r.end - r.start + 1;
            new_lines.splice(r.start..r.start + n, r.new_lines);
        }
        let mut out = new_lines.join("\n");
        if content.ends_with('\n') && !out.ends_with('\n') {
            out.push('\n');
        }
        out
    };
    Ok((new_content, warnings))
}

/// 当前时间格式化为 `YYYYmmdd-HHMMSS`(本地时区,由 unix 秒换算)。
fn local_timestamp() -> String {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0);
    let days = secs.div_euclid(86400);
    let sod = secs.rem_euclid(86400);
    let (y, m, d) = civil_from_days(days);
    format!(
        "{:04}{:02}{:02}-{:02}{:02}{:02}",
        y,
        m,
        d,
        sod / 3600,
        sod % 3600 / 60,
        sod % 60
    )
}

/// Howard Hinnant 的 days→(年,月,日) 算法(UTC,对备份文件名足够)。
fn civil_from_days(z: i64) -> (i64, u32, u32) {
    let z = z + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = (z - era * 146_097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    (if m <= 2 { y + 1 } else { y }, m, d)
}

/// 写入前备份,返回备份文件路径。同一秒内多次备份时追加 -N 防止覆盖。
pub fn backup(path: &Path) -> io::Result<PathBuf> {
    let ts = local_timestamp();
    let base = format!("{}.omz-pm.bak.{}", path.display(), ts);
    let mut bak = PathBuf::from(&base);
    let mut n = 1;
    while bak.exists() {
        bak = PathBuf::from(format!("{}-{}", base, n));
        n += 1;
    }
    fs::copy(path, &bak)?;
    Ok(bak)
}

/// 列出某 zshrc 的全部 omz-pm 备份(新 → 旧)。
pub fn list_backups(path: &Path) -> Vec<PathBuf> {
    let dir = path.parent().unwrap_or(Path::new("."));
    let prefix = format!(
        "{}.omz-pm.bak.",
        path.file_name().and_then(|n| n.to_str()).unwrap_or("")
    );
    let mut v: Vec<PathBuf> = fs::read_dir(dir)
        .into_iter()
        .flatten()
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| {
            p.file_name()
                .and_then(|n| n.to_str())
                .map(|n| n.starts_with(&prefix))
                .unwrap_or(false)
        })
        .collect();
    // 文件名含时间戳,名字降序即新→旧
    v.sort_by(|a, b| b.file_name().cmp(&a.file_name()));
    v
}

/// 恢复备份:先把当前 zshrc 备份一份,再用备份内容覆盖。
/// 返回 (恢复前快照路径)。
pub fn restore_backup(path: &Path, bak_file: &Path) -> io::Result<PathBuf> {
    let snapshot = backup(path)?;
    fs::copy(bak_file, path)?;
    Ok(snapshot)
}

/// 备份并原子写入(先写临时文件再 rename)。
/// 保留原文件的权限位;目标是符号链接时写入其指向的真实文件,
/// 避免用普通文件替换掉用户的 zshrc 软链。
pub fn save_with_backup(path: &Path, new_content: &str) -> io::Result<PathBuf> {
    let bak = backup(path)?;
    let target = fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf());
    let tmp = PathBuf::from(format!("{}.omz-pm.tmp", target.display()));
    fs::write(&tmp, new_content)?;
    // 临时文件以默认权限创建;rename 覆盖前拷贝原文件权限位,避免 600 → 644 回退
    if let Ok(meta) = fs::metadata(&target) {
        if let Err(e) = fs::set_permissions(&tmp, meta.permissions()) {
            let _ = fs::remove_file(&tmp);
            return Err(e);
        }
    }
    fs::rename(&tmp, &target)?;
    Ok(bak)
}

/// zshrc 默认路径:优先 `$ZDOTDIR/.zshrc`,否则 `~/.zshrc`。
pub fn default_zshrc_path() -> PathBuf {
    if let Ok(zdotdir) = std::env::var("ZDOTDIR") {
        if !zdotdir.is_empty() {
            return PathBuf::from(zdotdir).join(".zshrc");
        }
    }
    home_dir().join(".zshrc")
}

/// 若该行(已去前导空白、非注释)是 `ZSH_THEME=…` 赋值,返回 `=` 之后的原始值。
/// `ZSH_THEME_*` 等同名前缀变量不算,避免改主题时误伤。
fn theme_value_raw(t: &str) -> Option<&str> {
    let rest = t.strip_prefix("ZSH_THEME")?;
    rest.trim_start().strip_prefix('=')
}

/// 读取 `ZSH_THEME=` 设置的当前主题名(空串 = 使用默认提示符)。
pub fn read_theme(content: &str) -> Option<String> {
    for line in content.lines() {
        let t = line.trim_start();
        if t.starts_with('#') {
            continue;
        }
        if let Some(v) = theme_value_raw(t) {
            let v = v.trim();
            let v = v
                .strip_prefix('"')
                .and_then(|s| s.strip_suffix('"'))
                .or_else(|| v.strip_prefix('\'').and_then(|s| s.strip_suffix('\'')))
                .unwrap_or(v);
            return Some(v.to_string());
        }
    }
    None
}

/// 把 `ZSH_THEME=` 改为 new_theme;不存在该行时在文件末尾追加。
/// 返回 (新内容, 警告)。
pub fn apply_theme(content: &str, new_theme: &str) -> (String, Vec<String>) {
    let mut warnings = Vec::new();
    let lines: Vec<&str> = content.lines().collect();
    let mut replaced = false;
    let mut out: Vec<String> = Vec::with_capacity(lines.len() + 2);
    for line in &lines {
        let t = line.trim_start();
        if !replaced && !t.starts_with('#') && theme_value_raw(t).is_some() {
            out.push(format!("ZSH_THEME=\"{}\"", new_theme));
            replaced = true;
        } else {
            out.push((*line).to_string());
        }
    }
    if !replaced {
        if !out.is_empty() && !out.last().map(|l| l.trim().is_empty()).unwrap_or(true) {
            out.push(String::new());
        }
        out.push("# 由 omz-pm 添加".to_string());
        out.push(format!("ZSH_THEME=\"{}\"", new_theme));
        warnings.push("原 zshrc 没有 ZSH_THEME 行,已追加到文件末尾".to_string());
    }
    let mut res = out.join("\n");
    if content.ends_with('\n') && !res.ends_with('\n') {
        res.push('\n');
    }
    (res, warnings)
}

/// Unix 时间戳 → `YYYY-mm-dd HH:MM:SS`(UTC 换算,供展示)。
#[allow(dead_code)]
pub fn format_unix(secs: u64) -> String {
    let days = (secs / 86400) as i64;
    let sod = secs % 86400;
    let (y, m, d) = civil_from_days(days);
    format!(
        "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
        y,
        m,
        d,
        sod / 3600,
        sod % 3600 / 60,
        sod % 60
    )
}

pub fn home_dir() -> PathBuf {
    std::env::var("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("."))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_single_line() {
        let c = "plugins=(git docker z)\nsource $ZSH/oh-my-zsh.sh\n";
        let d = parse_decls(c);
        assert_eq!(d.len(), 1);
        assert_eq!(d[0].items, vec!["git", "docker", "z"]);
        assert_eq!(d[0].start_line, 0);
        assert_eq!(d[0].end_line, 0);
    }

    #[test]
    fn parse_multiline_with_comments_and_quotes() {
        let c = "# comment\nplugins=(\n  git   # vcs\n  'docker'\n  \"zsh-z\"\n)\n";
        let d = parse_decls(c);
        assert_eq!(d.len(), 1);
        assert_eq!(d[0].items, vec!["git", "docker", "zsh-z"]);
        assert_eq!(d[0].start_line, 1);
        assert_eq!(d[0].end_line, 5);
    }

    #[test]
    fn ignores_commented_example_line() {
        let c = "# Example format: plugins=(rails git textmate ruby lighthouse)\nplugins=(git)\n";
        let d = parse_decls(c);
        assert_eq!(d.len(), 1);
        assert_eq!(d[0].items, vec!["git"]);
    }

    #[test]
    fn reads_append_decls_too() {
        let c = "plugins=(git)\nplugins+=(docker z)\n";
        let (set, n) = read_enabled(c);
        assert_eq!(n, 2);
        assert!(set.contains("git") && set.contains("docker") && set.contains("z"));
    }

    #[test]
    fn enable_appends_preserving_other_lines() {
        let c = "# Which plugins?\nplugins=(git docker)\nsource $ZSH/oh-my-zsh.sh\n";
        let (new, w) = apply_changes(c, &["z".into()], &[]).unwrap();
        assert!(w.is_empty());
        assert!(new.contains("plugins=(git docker z)"));
        assert!(new.contains("# Which plugins?"));
        assert!(new.contains("source $ZSH/oh-my-zsh.sh"));
    }

    #[test]
    fn disable_removes_only_target() {
        let c = "plugins=(git docker z)\n";
        let (new, _) = apply_changes(c, &[], &["docker".into()]).unwrap();
        assert_eq!(new, "plugins=(git z)\n");
    }

    #[test]
    fn enable_existing_is_idempotent() {
        let c = "plugins=(git docker)\n";
        let (new, w) = apply_changes(c, &["git".into()], &[]).unwrap();
        assert_eq!(new, "plugins=(git docker)\n");
        assert!(!w.is_empty());
    }

    #[test]
    fn multiline_style_preserved() {
        let c = "plugins=(\n  git\n  docker\n)\n";
        let (new, _) = apply_changes(c, &["z".into()], &[]).unwrap();
        assert_eq!(new, "plugins=(\n  git\n  docker\n  z\n)\n");
    }

    #[test]
    fn no_decl_appends_at_end() {
        let c = "export EDITOR=vim\n";
        let (new, _) = apply_changes(c, &["git".into()], &[]).unwrap();
        assert!(new.starts_with("export EDITOR=vim\n"));
        assert!(new.trim_end().ends_with("plugins=(git)"));
    }

    #[test]
    fn trailing_comment_after_paren_preserved() {
        let c = "plugins=(git) # my plugins\n";
        let (new, _) = apply_changes(c, &["z".into()], &[]).unwrap();
        assert_eq!(new, "plugins=(git z) # my plugins\n");
    }

    #[test]
    fn dollar_tokens_preserved() {
        let c = "plugins=($custom git)\n";
        let (new, _) = apply_changes(c, &["z".into()], &[]).unwrap();
        assert!(new.contains("$custom"));
        assert!(new.contains("git z"));
    }

    #[test]
    fn quoted_weird_token_rewritten_safely() {
        let c = "plugins=('has space' git)\n";
        let (new, _) = apply_changes(c, &[], &["git".into()]).unwrap();
        assert_eq!(new, "plugins=('has space')\n");
    }

    #[test]
    fn roundtrip_enable_disable_is_identity() {
        let c = "plugins=(git docker)\n";
        let (mid, _) = apply_changes(c, &["z".into()], &[]).unwrap();
        assert_eq!(mid, "plugins=(git docker z)\n");
        let (back, _) = apply_changes(&mid, &[], &["z".into()]).unwrap();
        assert_eq!(back, c);
    }

    #[test]
    fn disable_keeps_positions_of_others() {
        let c = "plugins=(alpha beta gamma delta)\n";
        let (new, _) = apply_changes(c, &[], &["beta".into(), "gamma".into()]).unwrap();
        assert_eq!(new, "plugins=(alpha delta)\n");
    }

    #[test]
    fn multiline_edit_is_stable_and_idempotent() {
        // 多行声明被修改时整块重渲染(块内行内注释不保留);
        // 块外的行(# top / # tail / source x)与缩进风格原样保留。
        let c = "# top\nplugins=(\n  git  # vcs\n\n  docker\n) # tail\nsource x\n";
        let (mid, _) = apply_changes(c, &["z".into()], &[]).unwrap();
        assert_eq!(
            mid,
            "# top\nplugins=(\n  git\n  docker\n  z\n) # tail\nsource x\n"
        );
        let (again, _) = apply_changes(&mid, &[], &[]).unwrap();
        assert_eq!(again, mid, "无变更时不应改动文件");
        let (back, _) = apply_changes(&mid, &[], &["z".into()]).unwrap();
        assert_eq!(
            back,
            "# top\nplugins=(\n  git\n  docker\n) # tail\nsource x\n"
        );
    }

    #[test]
    fn disable_in_earlier_decl_edits_it_directly() {
        let c = "plugins=(git)\nplugins+=(docker)\n";
        let (new, _) = apply_changes(c, &[], &["git".into()]).unwrap();
        assert_eq!(new, "plugins=()\nplugins+=(docker)\n");
    }

    #[test]
    fn theme_read_and_apply() {
        let c = "ZSH_THEME=\"robbyrussell\"\nplugins=(git)\n";
        assert_eq!(read_theme(c).as_deref(), Some("robbyrussell"));
        let (new, w) = apply_theme(c, "agnoster");
        assert!(w.is_empty());
        assert!(new.starts_with("ZSH_THEME=\"agnoster\"\nplugins=(git)\n"));
        // 单引号/无引号/注释行
        assert_eq!(
            read_theme("# ZSH_THEME=\"x\"\nZSH_THEME='y'\n").as_deref(),
            Some("y")
        );
        // 不存在则追加
        let (new2, w2) = apply_theme("plugins=(git)\n", "eastwood");
        assert_eq!(w2.len(), 1);
        assert!(new2.trim_end().ends_with("ZSH_THEME=\"eastwood\""));
        // 往返
        let (back, _) = apply_theme(&new, "robbyrussell");
        assert_eq!(back, c);
    }

    #[test]
    fn theme_empty_value_is_default() {
        assert_eq!(read_theme("ZSH_THEME=\"\"\n").as_deref(), Some(""));
    }

    #[test]
    fn format_unix_basic() {
        assert_eq!(format_unix(0), "1970-01-01 00:00:00");
    }

    #[test]
    fn timestamp_format() {
        let ts = local_timestamp();
        assert_eq!(ts.len(), 15);
        assert!(ts.contains('-'));
    }

    #[test]
    fn theme_apply_skips_same_prefix_vars() {
        let c = "ZSH_THEME_DISABLE_CORRECTION=\"true\"\nZSH_THEME=\"robbyrussell\"\n";
        assert_eq!(read_theme(c).as_deref(), Some("robbyrussell"));
        let (new, w) = apply_theme(c, "agnoster");
        assert!(w.is_empty());
        assert!(
            new.contains("ZSH_THEME_DISABLE_CORRECTION=\"true\""),
            "同名前缀变量不应被改写: {new}"
        );
        assert!(new.contains("ZSH_THEME=\"agnoster\""));
    }

    #[cfg(unix)]
    #[test]
    fn save_preserves_permissions() {
        use std::os::unix::fs::PermissionsExt;
        let dir = std::env::temp_dir().join(format!("omz-pm-perm-{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        let p = dir.join("zshrc");
        fs::write(&p, "plugins=(git)\n").unwrap();
        fs::set_permissions(&p, fs::Permissions::from_mode(0o600)).unwrap();
        save_with_backup(&p, "plugins=(git z)\n").unwrap();
        let mode = fs::metadata(&p).unwrap().permissions().mode() & 0o777;
        assert_eq!(mode, 0o600, "权限位不应丢失");
        let _ = fs::remove_dir_all(dir);
    }

    #[cfg(unix)]
    #[test]
    fn save_follows_symlinked_zshrc() {
        let dir = std::env::temp_dir().join(format!("omz-pm-link-{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        let real = dir.join("real-zshrc");
        fs::write(&real, "plugins=(git)\n").unwrap();
        let link = dir.join("zshrc");
        std::os::unix::fs::symlink(&real, &link).unwrap();
        save_with_backup(&link, "plugins=(git z)\n").unwrap();
        assert!(
            fs::symlink_metadata(&link)
                .unwrap()
                .file_type()
                .is_symlink(),
            "zshrc 符号链接不应被替换成普通文件"
        );
        assert_eq!(fs::read_to_string(&real).unwrap(), "plugins=(git z)\n");
        let _ = fs::remove_dir_all(dir);
    }
}
