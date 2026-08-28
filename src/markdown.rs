//! 轻量 markdown 渲染:把 README 文本转成 ratatui 样式行,供 TUI 阅读器显示。
//!
//! 覆盖内置译文用到的子集:ATX/setext 标题、行内(粗体/斜体/行内代码/删除线/
//! 链接/`<kbd>` 等简单标签)、围栏代码块、引用块、分隔线、列表、表格(按显示
//! 宽度对齐,超宽时列内换行)。CLI `omz-pm readme` 不走这里,仍输出原文。
//!
//! 零新依赖:宽度计算与换行策略与 src/textwrap.rs 一致(CJK 占两列)。

use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use unicode_width::UnicodeWidthChar;

use crate::textwrap::display_width;

// ───────── 调色 ─────────

fn style_heading(level: usize) -> Style {
    match level {
        1 => Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
        2 => Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
        3 => Style::default().add_modifier(Modifier::BOLD),
        _ => Style::default()
            .fg(Color::DarkGray)
            .add_modifier(Modifier::BOLD),
    }
}

fn style_code() -> Style {
    Style::default().fg(Color::Magenta)
}

fn style_link() -> Style {
    Style::default()
        .fg(Color::Blue)
        .add_modifier(Modifier::UNDERLINED)
}

fn style_dim() -> Style {
    Style::default().fg(Color::DarkGray)
}

fn style_notice() -> Style {
    Style::default().fg(Color::Green)
}

// ───────── 行内解析 ─────────

/// 把一行行内文本解析成带样式的 Span。`base` 是继承样式(引用/标题内复用)。
fn inline(text: &str, base: Style) -> Vec<Span<'static>> {
    let chars: Vec<char> = text.chars().collect();
    let mut out: Vec<Span<'static>> = Vec::new();
    let mut buf = String::new();
    let mut i = 0usize;
    let n = chars.len();

    while i < n {
        let c = chars[i];
        match c {
            '\\' if i + 1 < n && "\\`*_[]()!~<>|-".contains(chars[i + 1]) => {
                buf.push(chars[i + 1]);
                i += 2;
            }
            '`' => {
                let ticks = chars[i..].iter().take_while(|&&x| x == '`').count();
                if let Some(close) = find_ticks(&chars, i + ticks, ticks) {
                    if !buf.is_empty() {
                        out.push(Span::styled(std::mem::take(&mut buf), base));
                    }
                    let inner: String = chars[i + ticks..close].iter().collect();
                    out.push(Span::styled(
                        inner.trim_matches(' ').to_string(),
                        style_code(),
                    ));
                    i = close + ticks;
                } else {
                    buf.push('`');
                    i += 1;
                }
            }
            '*' | '_' if emphasis_opens(&chars, i) => {
                let dbl = chars.get(i + 1) == Some(&c);
                let marker_len = if dbl { 2 } else { 1 };
                let close = if dbl {
                    find_seq(&chars, i + 2, c, 2)
                } else {
                    find_single_emphasis_close(&chars, i + 1, c)
                };
                if let Some(close) = close {
                    if !buf.is_empty() {
                        out.push(Span::styled(std::mem::take(&mut buf), base));
                    }
                    let inner: String = chars[i + marker_len..close].iter().collect();
                    let style = if dbl {
                        base.add_modifier(Modifier::BOLD)
                    } else {
                        base.add_modifier(Modifier::ITALIC)
                    };
                    out.extend(inline(&inner, style));
                    i = close + marker_len;
                } else {
                    buf.push(c);
                    i += 1;
                }
            }
            '~' if chars.get(i + 1) == Some(&'~') => {
                if let Some(close) = find_seq(&chars, i + 2, '~', 2) {
                    if !buf.is_empty() {
                        out.push(Span::styled(std::mem::take(&mut buf), base));
                    }
                    let inner: String = chars[i + 2..close].iter().collect();
                    out.push(Span::styled(
                        inner,
                        base.add_modifier(Modifier::CROSSED_OUT),
                    ));
                    i = close + 2;
                } else {
                    buf.push('~');
                    i += 1;
                }
            }
            '!' if chars.get(i + 1) == Some(&'[') => {
                if let Some((close_br, url_start, url_end)) = find_link(&chars, i + 1) {
                    let alt: String = chars[i + 2..close_br].iter().collect();
                    let url: String = chars[url_start..url_end].iter().collect();
                    if !buf.is_empty() {
                        out.push(Span::styled(std::mem::take(&mut buf), base));
                    }
                    let shown = if alt.trim().is_empty() { url } else { alt };
                    out.push(Span::styled(format!("🖼 {shown}"), style_dim()));
                    i = url_end + 1;
                } else {
                    buf.push('!');
                    i += 1;
                }
            }
            '[' => {
                if let Some((close_br, url_start, url_end)) = find_link(&chars, i) {
                    let label: String = chars[i + 1..close_br].iter().collect();
                    let url: String = chars[url_start..url_end].iter().collect();
                    if !buf.is_empty() {
                        out.push(Span::styled(std::mem::take(&mut buf), base));
                    }
                    let shown = if label.trim().is_empty() || label == url {
                        url
                    } else {
                        label
                    };
                    out.push(Span::styled(shown, style_link()));
                    i = url_end + 1;
                } else {
                    buf.push('[');
                    i += 1;
                }
            }
            '<' => match parse_tag(&chars, i) {
                TagMatch::Kbd(inner, next) => {
                    if !buf.is_empty() {
                        out.push(Span::styled(std::mem::take(&mut buf), base));
                    }
                    out.push(Span::styled(inner, base.add_modifier(Modifier::REVERSED)));
                    i = next;
                }
                TagMatch::Code(inner, next) => {
                    if !buf.is_empty() {
                        out.push(Span::styled(std::mem::take(&mut buf), base));
                    }
                    out.push(Span::styled(inner, style_code()));
                    i = next;
                }
                TagMatch::Url(url, next) => {
                    if !buf.is_empty() {
                        out.push(Span::styled(std::mem::take(&mut buf), base));
                    }
                    out.push(Span::styled(url, style_link()));
                    i = next;
                }
                TagMatch::Anchor(url, text, next) => {
                    if !buf.is_empty() {
                        out.push(Span::styled(std::mem::take(&mut buf), base));
                    }
                    let shown = if text.trim().is_empty() { url } else { text };
                    out.push(Span::styled(shown, style_link()));
                    i = next;
                }
                TagMatch::Drop(next) => {
                    i = next;
                }
                TagMatch::None => {
                    buf.push('<');
                    i += 1;
                }
            },
            _ => {
                buf.push(c);
                i += 1;
            }
        }
    }
    if !buf.is_empty() {
        out.push(Span::styled(buf, base));
    }
    out
}

fn find_ticks(chars: &[char], from: usize, ticks: usize) -> Option<usize> {
    let mut i = from;
    while i < chars.len() {
        if chars[i] == '`' {
            let run = chars[i..].iter().take_while(|&&x| x == '`').count();
            if run == ticks {
                return Some(i);
            }
            i += run;
        } else {
            i += 1;
        }
    }
    None
}

fn find_seq(chars: &[char], from: usize, c: char, count: usize) -> Option<usize> {
    let mut i = from;
    while i + count <= chars.len() {
        if chars[i..i + count].iter().all(|&x| x == c) {
            return Some(i);
        }
        i += 1;
    }
    None
}

/// 强调开判据:前一个字符不是字母/数字/下划线/同符号(防 snake_case 与 `a**b`),
/// 且强调内容以非空白开头。
fn emphasis_opens(chars: &[char], i: usize) -> bool {
    let c = chars[i];
    let prev_ok =
        i == 0 || !(chars[i - 1].is_alphanumeric() || chars[i - 1] == '_' || chars[i - 1] == c);
    let next = chars.get(i + if chars.get(i + 1) == Some(&c) { 2 } else { 1 });
    prev_ok && matches!(next, Some(ch) if !ch.is_whitespace())
}

/// 单字符强调的闭判据:闭合符前不是空白,后不是字母/数字/下划线。
fn find_single_emphasis_close(chars: &[char], from: usize, c: char) -> Option<usize> {
    let mut i = from;
    while i < chars.len() {
        if chars[i] == c
            && i > from
            && !chars[i - 1].is_whitespace()
            && !chars
                .get(i + 1)
                .is_some_and(|ch| ch.is_alphanumeric() || *ch == '_')
        {
            return Some(i);
        }
        i += 1;
    }
    None
}

/// `[label](url)`:返回 (']' 位置, url 起止)。
fn find_link(chars: &[char], open: usize) -> Option<(usize, usize, usize)> {
    let mut close_br = None;
    let mut i = open + 1;
    while i < chars.len() {
        match chars[i] {
            ']' => {
                close_br = Some(i);
                break;
            }
            '[' => return None,
            _ => {}
        }
        i += 1;
    }
    let close_br = close_br?;
    if chars.get(close_br + 1) != Some(&'(') {
        return None;
    }
    let mut url_end = close_br + 2;
    while url_end < chars.len() && chars[url_end] != ')' && !chars[url_end].is_whitespace() {
        url_end += 1;
    }
    if chars.get(url_end) != Some(&')') {
        return None;
    }
    Some((close_br, close_br + 2, url_end))
}

enum TagMatch {
    /// <kbd>x</kbd> → 反显
    Kbd(String, usize),
    /// <code>x</code> → 代码色
    Code(String, usize),
    /// <https://…> 自动链接
    Url(String, usize),
    /// <a href="url">text</a>
    Anchor(String, String, usize),
    /// 白名单内的普通标签,剥离
    Drop(usize),
    None,
}

fn parse_tag(chars: &[char], i: usize) -> TagMatch {
    let Some(close) = chars[i..].iter().position(|&c| c == '>').map(|off| i + off) else {
        return TagMatch::None;
    };
    let tag: String = chars[i + 1..close].iter().collect();
    let lower = tag.to_lowercase();

    if lower == "kbd" {
        if let Some(end) = find_tag_end(chars, close, "kbd") {
            let inner: String = chars[close + 1..end].iter().collect();
            return TagMatch::Kbd(inner, end + 6); // </kbd> = 6 字符
        }
    }
    if lower == "code" {
        if let Some(end) = find_tag_end(chars, close, "code") {
            let inner: String = chars[close + 1..end].iter().collect();
            return TagMatch::Code(inner, end + 7); // </code> = 7 字符
        }
    }
    if lower == "a" || lower.starts_with("a ") {
        if let Some(end) = find_tag_end(chars, close, "a") {
            let text: String = chars[close + 1..end].iter().collect();
            let href = extract_href(&tag).unwrap_or_default();
            return TagMatch::Anchor(href, text, end + 4); // </a> = 4 字符
        }
    }
    if lower.starts_with("http") {
        return TagMatch::Url(tag, close + 1);
    }
    const PLAIN: [&str; 7] = ["br", "sup", "sub", "span", "em", "strong", "p"];
    for name in PLAIN {
        if lower == name
            || lower == format!("/{name}")
            || lower == format!("{name}/")
            || lower.starts_with(&format!("{name} "))
            || lower.starts_with(&format!("{name}="))
        {
            return TagMatch::Drop(close + 1);
        }
    }
    TagMatch::None
}

fn find_tag_end(chars: &[char], from: usize, name: &str) -> Option<usize> {
    let close: String = format!("</{name}>");
    let hc: Vec<char> = close.chars().collect();
    let mut i = from;
    while i + hc.len() <= chars.len() {
        if chars[i..i + hc.len()]
            .iter()
            .collect::<String>()
            .eq_ignore_ascii_case(&close)
        {
            return Some(i);
        }
        i += 1;
    }
    None
}

fn extract_href(tag: &str) -> Option<String> {
    let pos = tag.to_lowercase().find("href")?;
    let rest = tag[pos + 4..].trim_start_matches(|c: char| c.is_whitespace() || c == '=');
    let quote = rest.chars().next()?;
    let url = if quote == '"' || quote == '\'' {
        rest[1..].split(quote).next()?
    } else {
        rest.split_whitespace().next()?
    };
    Some(url.to_string())
}

// ───────── 样式字符流与换行 ─────────

#[derive(Clone)]
struct StyledChar {
    ch: char,
    style: Style,
}

fn to_chars(spans: &[Span<'static>]) -> Vec<StyledChar> {
    spans
        .iter()
        .flat_map(|s| {
            s.content
                .chars()
                .map(move |ch| StyledChar { ch, style: s.style })
        })
        .collect()
}

/// 贪心换行:拉丁词整体不拆、CJK 逐字可断、超长词按字符硬拆(与 textwrap::wrap 同策略)。
fn wrap_chars(chars: &[StyledChar], width: usize) -> Vec<Vec<Span<'static>>> {
    if chars.is_empty() {
        return vec![Vec::new()];
    }
    // 切 token:连续窄字符一段、宽字符单列;显式换行符 = 空 token。
    let mut tokens: Vec<Vec<StyledChar>> = Vec::new();
    let mut cur: Vec<StyledChar> = Vec::new();
    for sc in chars {
        match sc.ch {
            '\n' => {
                if !cur.is_empty() {
                    tokens.push(std::mem::take(&mut cur));
                }
                tokens.push(Vec::new());
            }
            ' ' => {
                if !cur.is_empty() {
                    tokens.push(std::mem::take(&mut cur));
                }
                tokens.push(vec![sc.clone()]);
            }
            c if c.width().unwrap_or(0) >= 2 => {
                if !cur.is_empty() {
                    tokens.push(std::mem::take(&mut cur));
                }
                tokens.push(vec![sc.clone()]);
            }
            _ => cur.push(sc.clone()),
        }
    }
    if !cur.is_empty() {
        tokens.push(cur);
    }

    let mut lines: Vec<Vec<Span<'static>>> = Vec::new();
    let mut line: Vec<Span<'static>> = Vec::new();
    let mut line_w = 0usize;

    macro_rules! break_line {
        () => {{
            lines.push(std::mem::take(&mut line));
            line_w = 0;
        }};
    }

    for tok in &tokens {
        if tok.is_empty() {
            break_line!();
            continue;
        }
        if tok.len() == 1 && tok[0].ch == ' ' {
            if line_w > 0 && line_w < width {
                line.push(Span::styled(" ", tok[0].style));
                line_w += 1;
            }
            continue;
        }
        let tw: usize = tok.iter().map(|sc| sc.ch.width().unwrap_or(0)).sum();
        if tw > width {
            if line_w > 0 {
                break_line!();
            }
            for sc in tok {
                let cw = sc.ch.width().unwrap_or(0);
                if line_w + cw > width {
                    break_line!();
                }
                line.push(Span::styled(sc.ch.to_string(), sc.style));
                line_w += cw;
            }
        } else {
            if line_w + tw > width {
                break_line!();
            }
            for sc in tok {
                if let Some(last) = line.last_mut() {
                    if last.style == sc.style {
                        last.content.to_mut().push(sc.ch);
                        line_w += sc.ch.width().unwrap_or(0);
                        continue;
                    }
                }
                line.push(Span::styled(sc.ch.to_string(), sc.style));
                line_w += sc.ch.width().unwrap_or(0);
            }
        }
    }
    if !line.is_empty() || lines.is_empty() {
        lines.push(line);
    }
    lines
}

// ───────── 块级渲染 ─────────

/// 渲染整篇 markdown 为样式行。`width` 是可用显示宽度。
pub fn render(src: &str, width: usize) -> Vec<Line<'static>> {
    let width = width.max(20);
    let raw = merge_list_continuations(src);
    let mut out: Vec<Line<'static>> = Vec::new();
    let mut i = 0usize;
    // 上一行是否为非空正文(setext 标题判定用)
    let mut prev_was_paragraph = false;

    while i < raw.len() {
        let line = &raw[i];
        let trimmed = line.trim();

        // 围栏代码块:内容加淡色竖线槽,围栏行吞掉
        if trimmed.starts_with("```") || trimmed.starts_with("~~~") {
            let fence = &trimmed[..3];
            i += 1;
            let gutter = Span::styled("│ ".to_string(), style_dim());
            while i < raw.len() && !raw[i].trim().starts_with(fence) {
                // 代码内容保持原样(不过 inline 解析),但要换行避免溢出
                for l in wrap_chars(
                    &to_chars(&[Span::styled(raw[i].clone(), Style::default())]),
                    width - 2,
                ) {
                    let mut spans = vec![gutter.clone()];
                    spans.extend(l);
                    out.push(Line::from(spans));
                }
                i += 1;
            }
            i += 1; // 闭合围栏
            if raw.get(i).map_or(true, |l| !l.trim().is_empty()) {
                out.push(Line::from(String::new()));
            }
            prev_was_paragraph = false;
            continue;
        }

        // 表格
        if trimmed.starts_with('|') && trimmed.len() > 1 && trimmed[1..].contains('|') {
            let mut block: Vec<&str> = Vec::new();
            while i < raw.len() && raw[i].trim().starts_with('|') {
                block.push(raw[i].as_str());
                i += 1;
            }
            out.extend(render_table(&block, width));
            prev_was_paragraph = false;
            continue;
        }

        // 空行
        if trimmed.is_empty() {
            out.push(Line::from(String::new()));
            prev_was_paragraph = false;
            i += 1;
            continue;
        }

        // ATX 标题
        if let Some(level) = heading_level(trimmed) {
            let text = trimmed[level..].trim().trim_end_matches('#').trim();
            for l in wrap_chars(&to_chars(&inline(text, style_heading(level))), width) {
                out.push(Line::from(l));
            }
            prev_was_paragraph = false;
            i += 1;
            continue;
        }

        // setext 标题(=== / --- 紧跟正文行)
        if prev_was_paragraph && is_rule_of(trimmed, '=') {
            restyle_last(&mut out, style_heading(1));
            prev_was_paragraph = false;
            i += 1;
            continue;
        }
        if prev_was_paragraph && is_rule_of(trimmed, '-') {
            restyle_last(&mut out, style_heading(2));
            prev_was_paragraph = false;
            i += 1;
            continue;
        }

        // 分隔线
        if is_rule_of(trimmed, '*') || is_rule_of(trimmed, '-') || is_rule_of(trimmed, '_') {
            out.push(Line::from(Span::styled("─".repeat(width), style_dim())));
            prev_was_paragraph = false;
            i += 1;
            continue;
        }

        // 引用块:每条视觉行加淡色竖线前缀
        if trimmed.starts_with('>') {
            let mut quoted: Vec<StyledChar> = Vec::new();
            while i < raw.len() && raw[i].trim().starts_with('>') {
                let inner = raw[i].trim().trim_start_matches('>').trim_start();
                if !quoted.is_empty() {
                    quoted.push(StyledChar {
                        ch: '\n',
                        style: Style::default(),
                    });
                }
                quoted.extend(to_chars(&inline(inner, Style::default())));
                i += 1;
            }
            let prefix = Span::styled("▏ ".to_string(), style_dim());
            for l in wrap_chars(&quoted, width.saturating_sub(2)) {
                let mut spans = vec![prefix.clone()];
                spans.extend(l);
                out.push(Line::from(spans));
            }
            prev_was_paragraph = false;
            continue;
        }

        // 列表项(支持一层缩进,续行悬挂)
        if let Some((indent, marker, content)) = parse_list_item(line) {
            let mark = marker_text(&marker);
            let base_indent = indent + display_width(&mark);
            // 内容按扣除缩进后的宽度换行,再统一加前缀,保证任何视觉行不超宽
            let body_w = width.saturating_sub(base_indent).max(10);
            let wrapped = wrap_chars(&to_chars(&inline(&content, Style::default())), body_w);
            let first_prefix = format!("{}{}", " ".repeat(indent), mark);
            let hang = " ".repeat(base_indent);
            for (k, l) in wrapped.into_iter().enumerate() {
                let mut spans = vec![Span::styled(
                    if k == 0 {
                        first_prefix.clone()
                    } else {
                        hang.clone()
                    },
                    Style::default(),
                )];
                spans.extend(l);
                out.push(Line::from(spans));
            }
            prev_was_paragraph = false;
            i += 1;
            continue;
        }

        // ✅/📌 提示行:整行绿色(沿用旧阅读器的视觉约定)
        if trimmed.starts_with('✅') || trimmed.starts_with('📌') {
            for l in wrap_chars(&to_chars(&inline(trimmed, style_notice())), width) {
                out.push(Line::from(l));
            }
            prev_was_paragraph = false;
            i += 1;
            continue;
        }

        // 普通段落
        for l in wrap_chars(&to_chars(&inline(trimmed, Style::default())), width) {
            out.push(Line::from(l));
        }
        prev_was_paragraph = true;
        i += 1;
    }
    out
}

/// markdown 懒续行:紧跟列表项、有缩进的非空行(且不是新的块级结构)归入该项。
fn merge_list_continuations(src: &str) -> Vec<String> {
    let mut merged: Vec<String> = Vec::new();
    for line in src.lines() {
        if !merged.is_empty() {
            let t = line.trim();
            let is_item = parse_list_item(line).is_some();
            let is_new_block = t.is_empty()
                || is_item
                || heading_level(t).is_some()
                || t.starts_with('|')
                || t.starts_with('>')
                || t.starts_with("```")
                || t.starts_with("~~~");
            if parse_list_item(merged.last().unwrap()).is_some()
                && line.starts_with(' ')
                && !is_new_block
            {
                let combined = format!("{} {}", merged.last().unwrap().trim_end(), t);
                *merged.last_mut().unwrap() = combined;
                continue;
            }
        }
        merged.push(line.to_string());
    }
    merged
}

fn restyle_last(out: &mut [Line<'static>], style: Style) {
    if let Some(last) = out.last_mut() {
        for s in &mut last.spans {
            s.style = style;
        }
    }
}

fn heading_level(trimmed: &str) -> Option<usize> {
    let level = trimmed.chars().take_while(|&c| c == '#').count();
    if (1..=6).contains(&level)
        && trimmed[level..].starts_with(' ')
        && !trimmed[level..].trim().is_empty()
    {
        Some(level)
    } else {
        None
    }
}

/// 该行是否由同一字符 `c` 重复 ≥3 次构成(允许空格,如 `- - -`)。
fn is_rule_of(trimmed: &str, c: char) -> bool {
    trimmed.len() >= 3 && trimmed.chars().all(|x| x == c || x == ' ')
}

fn parse_list_item(line: &str) -> Option<(usize, String, String)> {
    let indent = line.len() - line.trim_start().len();
    let t = line.trim_start();
    match t.chars().next()? {
        '-' | '*' | '+' if t[1..].starts_with([' ', '\t']) => {
            Some((indent, t[..1].to_string(), t[1..].trim().to_string()))
        }
        d if d.is_ascii_digit() => {
            let dot = t.find(['.', ')'])?;
            let num = &t[..dot];
            if !num.is_empty()
                && num.chars().all(|c| c.is_ascii_digit())
                && t[dot + 1..].starts_with([' ', '\t'])
            {
                Some((indent, format!("{num}."), t[dot + 1..].trim().to_string()))
            } else {
                None
            }
        }
        _ => None,
    }
}

fn marker_text(marker: &str) -> String {
    if marker.starts_with(['-', '*', '+']) {
        "• ".to_string()
    } else {
        format!("{marker} ")
    }
}

// ───────── 表格 ─────────

/// 解析一行表格为单元格,`\|` 转义渲染为字面竖线。
fn split_row(line: &str) -> Vec<String> {
    let t = line.trim();
    let t = t.strip_prefix('|').unwrap_or(t);
    let t = t.strip_suffix('|').unwrap_or(t);
    let mut cells = Vec::new();
    let mut cur = String::new();
    let mut escaped = false;
    for c in t.chars() {
        if escaped {
            if c != '|' {
                cur.push('\\');
            }
            cur.push(c);
            escaped = false;
        } else if c == '\\' {
            escaped = true;
        } else if c == '|' {
            cells.push(std::mem::take(&mut cur).trim().to_string());
        } else {
            cur.push(c);
        }
    }
    if escaped {
        cur.push('\\');
    }
    cells.push(cur.trim().to_string());
    cells
}

fn is_table_separator(line: &str) -> bool {
    let cells = split_row(line);
    !cells.is_empty()
        && cells
            .iter()
            .all(|c| !c.is_empty() && c.chars().all(|x| matches!(x, '-' | ':' | ' ')))
}

/// 超宽表格的列宽收缩(水位法):从最宽的列逐级削峰,
/// 窄列(如别名列)尽量保持自然宽;压到下限 4 仍放不下才等比砍。
fn shrink_columns(natural: &[usize], avail: usize) -> Vec<usize> {
    let total = |w: &[usize]| w.iter().sum::<usize>();
    if avail == 0 || total(natural) <= avail {
        return natural.to_vec();
    }
    let mut levels: Vec<usize> = {
        let mut v = natural.to_vec();
        v.sort_unstable_by(|a, b| b.cmp(a));
        v.dedup();
        v
    };
    levels.push(4);
    let mut capped = natural.to_vec();
    for &level in &levels {
        let level = level.max(4);
        capped = natural.iter().map(|&w| w.min(level)).collect();
        if total(&capped) <= avail {
            break;
        }
    }
    if total(&capped) > avail {
        // 压到 4 还放不下:等比砍到放得下
        let scale = avail as f64 / total(&capped) as f64;
        capped = capped
            .iter()
            .map(|&w| ((w as f64 * scale) as usize).max(2))
            .collect();
    }
    // 把富余宽度还给被削的列(不超过自然宽)
    let mut extra = avail.saturating_sub(total(&capped));
    for (k, w) in capped.iter_mut().enumerate() {
        if extra == 0 {
            break;
        }
        let room = natural[k].saturating_sub(*w);
        let add = room.min(extra);
        *w += add;
        extra -= add;
    }
    capped
}

fn render_table(block: &[&str], width: usize) -> Vec<Line<'static>> {
    let header: Vec<String> = split_row(block[0]);
    let cols = header.len();
    if cols == 0 {
        return Vec::new();
    }
    let mut start = 1;
    if block.len() > 1 && is_table_separator(block[1]) {
        start = 2;
    }
    let mut body: Vec<Vec<String>> = Vec::new();
    for line in &block[start..] {
        let mut cells = split_row(line);
        cells.resize(cols, String::new());
        body.push(cells);
    }

    let parsed: Vec<(bool, Vec<Vec<Span<'static>>>)> = std::iter::once((true, header))
        .chain(body.into_iter().map(|r| (false, r)))
        .map(|(is_header, r)| {
            (
                is_header,
                r.iter()
                    .map(|c| inline(c, Style::default()))
                    .collect::<Vec<_>>(),
            )
        })
        .collect();

    // 自然列宽 = 各行该列单元格的最大显示宽度
    let mut widths = vec![0usize; cols];
    for (_, r) in &parsed {
        for (k, cell) in r.iter().enumerate() {
            let w: usize = cell.iter().map(|s| display_width(&s.content)).sum();
            widths[k] = widths[k].max(w);
        }
    }

    let sep = 3 * (cols - 1); // " │ " 分隔
    if widths.iter().sum::<usize>() + sep > width {
        widths = shrink_columns(&widths, width - sep);
    }

    let sep_span = Span::styled(" │ ", style_dim());
    let mut out: Vec<Line<'static>> = Vec::new();
    for (is_header, r) in &parsed {
        let cell_style = if *is_header {
            Style::default().add_modifier(Modifier::BOLD)
        } else {
            Style::default()
        };
        let wrapped: Vec<Vec<Vec<Span<'static>>>> = r
            .iter()
            .enumerate()
            .map(|(k, cell)| {
                let styled: Vec<Span<'static>> = cell
                    .iter()
                    .map(|s| Span::styled(s.content.clone(), cell_style.patch(s.style)))
                    .collect();
                wrap_chars(&to_chars(&styled), widths[k])
            })
            .collect();

        let height = wrapped.iter().map(Vec::len).max().unwrap_or(1);
        for k in 0..height {
            let mut spans: Vec<Span<'static>> = Vec::new();
            for (ck, col) in wrapped.iter().enumerate() {
                if ck > 0 {
                    spans.push(sep_span.clone());
                }
                let last_col = ck == cols - 1;
                match col.get(k) {
                    Some(seg) => {
                        spans.extend(seg.iter().cloned());
                        let used: usize = seg.iter().map(|s| display_width(&s.content)).sum();
                        if used < widths[ck] && !last_col {
                            spans.push(Span::styled(
                                " ".repeat(widths[ck] - used),
                                Style::default(),
                            ));
                        }
                    }
                    None => {
                        if !last_col {
                            spans.push(Span::styled(" ".repeat(widths[ck]), Style::default()));
                        }
                    }
                }
            }
            out.push(Line::from(spans));
        }
        if *is_header {
            let rule = (widths.iter().sum::<usize>() + sep).min(width);
            out.push(Line::from(Span::styled("─".repeat(rule), style_dim())));
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn texts(line: &Line<'static>) -> Vec<String> {
        line.spans.iter().map(|s| s.content.to_string()).collect()
    }

    fn joined(line: &Line<'static>) -> String {
        texts(line).concat()
    }

    #[test]
    fn heading_stripped_and_styled() {
        let out = render("## 用法\n", 40);
        assert_eq!(joined(&out[0]), "用法");
        let st = out[0].spans[0].style;
        assert!(st.fg == Some(Color::Cyan));
        assert!(st.add_modifier.contains(Modifier::BOLD));
    }

    #[test]
    fn bold_italic_code_strikethrough() {
        let out = render("**粗体** _斜体_ `code` ~~删~~\n", 60);
        let spans = &out[0].spans;
        let bold = spans.iter().find(|s| s.content == "粗体").unwrap();
        assert!(bold.style.add_modifier.contains(Modifier::BOLD));
        let ital = spans.iter().find(|s| s.content == "斜体").unwrap();
        assert!(ital.style.add_modifier.contains(Modifier::ITALIC));
        let code = spans.iter().find(|s| s.content == "code").unwrap();
        assert_eq!(code.style.fg, Some(Color::Magenta));
        let del = spans.iter().find(|s| s.content == "删").unwrap();
        assert!(del.style.add_modifier.contains(Modifier::CROSSED_OUT));
    }

    #[test]
    fn snake_case_not_italic() {
        let out = render("foo_bar_baz 与 a_b\n", 40);
        let all = joined(&out[0]);
        assert_eq!(all, "foo_bar_baz 与 a_b");
    }

    #[test]
    fn link_shows_label_only() {
        let out = render("见 [官方文档](https://example.com/very/long) 说明\n", 40);
        let all = joined(&out[0]);
        assert!(all.contains("官方文档"));
        assert!(!all.contains("example.com"));
        let link = out[0]
            .spans
            .iter()
            .find(|s| s.content == "官方文档")
            .unwrap();
        assert_eq!(link.style.fg, Some(Color::Blue));
    }

    #[test]
    fn kbd_reversed() {
        let out = render("按 <kbd>esc</kbd> 两次\n", 40);
        let kbd = out[0].spans.iter().find(|s| s.content == "esc").unwrap();
        assert!(kbd.style.add_modifier.contains(Modifier::REVERSED));
        let all = joined(&out[0]);
        assert!(!all.contains('<'));
    }

    #[test]
    fn code_fence_hidden_with_gutter() {
        let out = render("```zsh\nplugins=(... x)\n```\n", 40);
        let all: Vec<String> = out.iter().map(joined).collect();
        assert!(!all.iter().any(|l| l.contains("```")));
        assert!(all
            .iter()
            .any(|l| l.starts_with("│ ") && l.contains("plugins")));
    }

    #[test]
    fn table_aligned_with_cjk() {
        let src = "| 别名 | 命令 |\n| --- | --- |\n| `g` | `git` |\n| 中文宽 | `ls` |\n";
        let out = render(src, 40);
        let widths: Vec<usize> = out[..2].iter().map(|l| display_width(&joined(l))).collect();
        assert_eq!(widths[0], widths[1], "两行宽度应一致: {widths:?}");
        // 表头加粗
        let h = &out[0].spans[0];
        assert!(h.style.add_modifier.contains(Modifier::BOLD));
        // 表头下有分隔线
        assert!(joined(&out[1]).starts_with('─'));
    }

    #[test]
    fn table_wide_cells_wrap() {
        let src = "| 别名 | 说明 |\n| --- | --- |\n| a | 这是一条非常长的说明文字,远远超出可用宽度,需要自动换行成多行显示而不截断内容 |\n";
        let out = render(src, 40);
        for l in &out {
            assert!(display_width(&joined(l)) <= 40, "超宽: {:?}", joined(l));
        }
        let all: String = out.iter().map(joined).collect::<Vec<_>>().concat();
        assert!(all.contains("不截断内容"), "内容不得丢失");
    }

    #[test]
    fn table_escaped_pipe() {
        let src = "| 命令 | 说明 |\n| --- | --- |\n| `a\\|b` | 含竖线 |\n";
        let out = render(src, 40);
        let all: String = out.iter().map(joined).collect::<Vec<_>>().concat();
        assert!(all.contains("a|b"), "转义竖线应渲染为字面 | ");
        // ASCII | 只有数据里那一个;列分隔符是制表符 │(两个数据行各一个)
        assert_eq!(all.matches('|').count(), 1);
        assert_eq!(all.matches('│').count(), 2);
    }

    #[test]
    fn hr_and_setext() {
        let out = render("正文一段\n===\n\n---\n", 40);
        // setext: 正文变成 h1 样式
        assert!(out[0].spans[0].style.fg == Some(Color::Cyan));
        // --- 单独成行 → 分隔线(前面隔一个空行)
        assert!(out.iter().any(|l| joined(l).starts_with('─')));
    }

    #[test]
    fn blockquote_prefixed() {
        let out = render("> 引用内容\n", 40);
        assert!(joined(&out[0]).starts_with("▏ 引用内容"));
    }

    #[test]
    fn lists_with_hanging_indent() {
        let out = render(
            "- 第一项内容较长会被折行\n  续行属于同一项\n2. 有序第二项\n",
            20,
        );
        let first = joined(&out[0]);
        assert!(first.starts_with("• 第一项"));
        // 续行(懒续行合并进本项)悬挂缩进,任何行都不超宽
        for l in &out {
            assert!(display_width(&joined(l)) <= 20, "超宽: {:?}", joined(l));
        }
        assert!(
            out.iter()
                .any(|l| joined(l).starts_with("  ") && !joined(l).trim().is_empty()),
            "续行应悬挂缩进"
        );
        let all: String = out
            .iter()
            .map(joined)
            .collect::<Vec<_>>()
            .concat()
            .replace(' ', "");
        assert!(all.contains("续行属于同一项"), "内容不得丢失: {all}");
        assert!(out.iter().any(|l| joined(l).starts_with("2. 有序")));
    }

    #[test]
    fn notice_line_green() {
        let out = render("✅ 启用方式:把「x」加入 plugins 数组。\n", 40);
        assert!(out[0].spans[0].style.fg == Some(Color::Green));
    }

    #[test]
    fn image_renders_alt() {
        let out = render("![演示图](https://img.example/a.png)\n", 40);
        let all = joined(&out[0]);
        assert!(all.contains("演示图"));
        assert!(!all.contains("img.example"));
    }

    #[test]
    fn render_all_bundled_readmes_within_width() {
        // 全量回归:359 篇译文都要能渲染、无超宽行、无裸露围栏
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("data/readmes-zh");
        let mut checked = 0usize;
        for entry in std::fs::read_dir(&dir).expect("data/readmes-zh 缺失") {
            let path = entry.unwrap().path();
            if path.extension().is_none_or(|e| e != "md") {
                continue;
            }
            let src = std::fs::read_to_string(&path).unwrap();
            let name = path.file_name().unwrap().to_string_lossy().to_string();
            for l in render(&src, 100) {
                let text = joined(&l);
                assert!(display_width(&text) <= 100, "{name}: 超宽行 {:?}", text);
                assert!(!text.contains("```"), "{name}: 围栏裸露");
            }
            checked += 1;
        }
        assert!(checked >= 350, "译文数量异常: {checked}");
    }

    #[test]
    fn real_docker_readme_has_no_raw_syntax() {
        // 真实译文集成抽查:markdown 语法符号不得裸露
        let src = std::fs::read_to_string(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/data/readmes-zh/docker.md"
        ));
        if let Ok(src) = src {
            let out = render(&src, 100);
            let all: Vec<String> = out.iter().map(joined).collect();
            assert!(!all.iter().any(|l| l.contains("```")));
            assert!(!all.iter().any(|l| l.starts_with('#')));
            assert!(!all.iter().any(|l| l.starts_with("| ")));
            // 表格行全部对齐到 ≤100
            for l in &all {
                assert!(display_width(l) <= 100, "超宽: {l}");
            }
        }
    }
}
