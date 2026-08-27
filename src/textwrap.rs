//! CJK 友好的文本排版辅助:按显示宽度换行、填充、截断。
//! 中文等宽字符占两列,直接用 chars().count() 会导致表格错位。

use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

pub fn display_width(s: &str) -> usize {
    s.width()
}

/// 右侧补空格到指定显示宽度。
pub fn pad_to(s: &str, width: usize) -> String {
    let w = display_width(s);
    if w >= width {
        s.to_string()
    } else {
        format!("{}{}", s, " ".repeat(width - w))
    }
}

/// 截断到指定显示宽度,超出部分以 `…` 结尾(不切断组合字符)。
pub fn truncate_to(s: &str, width: usize) -> String {
    if display_width(s) <= width {
        return s.to_string();
    }
    if width == 0 {
        return String::new();
    }
    let mut out = String::new();
    let mut cur = 0usize;
    for ch in s.chars() {
        let cw = ch.width().unwrap_or(0);
        // 需给末尾的 …(宽 1)留位
        if cur + cw + 1 > width {
            break;
        }
        out.push(ch);
        cur += cw;
    }
    out.push('…');
    out
}

/// 按显示宽度贪心换行:拉丁单词整体不拆、CJK 逐字可断、
/// 超长单词(如 URL)按字符硬拆;保留显式 `\n`。
pub fn wrap(s: &str, width: usize) -> Vec<String> {
    let mut result = Vec::new();
    if width == 0 {
        return vec![s.to_string()];
    }
    for para in s.split('\n') {
        if para.is_empty() {
            result.push(String::new());
            continue;
        }
        // 切分 token:(文本, 显示宽度, 词前是否有空格)。
        // 连续的窄字符(拉丁/数字/ASCII 标点)为一个词,
        // 宽字符(CJK 等)单独成词;原始空格记入「词前空格」标记,
        // 组装时行首的前导空格会被丢弃。
        let mut tokens: Vec<(String, usize, bool)> = Vec::new();
        let mut cur = String::new();
        let mut cur_start_space = false; // 当前词之前是否有空格
        let mut pending_space = false; // 上一个 token 之后是否出现过空格
        for ch in para.chars() {
            let w = ch.width().unwrap_or(0);
            if ch == ' ' || ch == '\t' {
                if !cur.is_empty() {
                    let ww = display_width(&cur);
                    tokens.push((std::mem::take(&mut cur), ww, cur_start_space));
                }
                pending_space = true;
            } else if w >= 2 {
                if !cur.is_empty() {
                    let ww = display_width(&cur);
                    tokens.push((std::mem::take(&mut cur), ww, cur_start_space));
                }
                tokens.push((ch.to_string(), w, pending_space));
                pending_space = false;
            } else {
                if cur.is_empty() {
                    cur_start_space = pending_space;
                    pending_space = false;
                }
                cur.push(ch);
            }
        }
        if !cur.is_empty() {
            let ww = display_width(&cur);
            tokens.push((cur, ww, cur_start_space));
        }

        let mut line = String::new();
        let mut line_w = 0usize;
        for (tok, tw, space_before) in tokens {
            // 行首永不放前导空格
            let lead = usize::from(space_before && !line.is_empty());
            let cost = tw + lead;
            if tw > width {
                // 超长 token 硬拆(独立处理,先结束当前行)
                if !line.is_empty() {
                    result.push(std::mem::take(&mut line));
                }
                let mut seg = String::new();
                let mut seg_w = 0;
                for ch in tok.chars() {
                    let cw = ch.width().unwrap_or(0);
                    if seg_w + cw > width {
                        result.push(std::mem::take(&mut seg));
                        seg_w = 0;
                    }
                    seg.push(ch);
                    seg_w += cw;
                }
                line = seg;
                line_w = seg_w;
            } else if line_w + cost > width {
                result.push(std::mem::take(&mut line));
                line = tok;
                line_w = tw;
            } else {
                if lead == 1 {
                    line.push(' ');
                }
                line.push_str(&tok);
                line_w += cost;
            }
        }
        if !line.is_empty() || result.is_empty() {
            result.push(line);
        }
    }
    if result.is_empty() {
        result.push(String::new());
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn width_of_mixed() {
        assert_eq!(display_width("abc"), 3);
        assert_eq!(display_width("中文"), 4);
        assert_eq!(display_width("a中b"), 4);
    }

    #[test]
    fn pad_uses_display_width() {
        let s = pad_to("中文", 8);
        assert_eq!(display_width(&s), 8);
        assert_eq!(s.chars().count(), 6); // 2 个汉字 + 6 个空格
    }

    #[test]
    fn truncate_cjk() {
        let t = truncate_to("中文说明很长很长", 7);
        assert_eq!(display_width(&t), 7);
        assert!(t.ends_with('…'));
    }

    #[test]
    fn truncate_short_is_noop() {
        assert_eq!(truncate_to("abc", 10), "abc");
    }

    #[test]
    fn wrap_mixed_text() {
        let lines = wrap("这是 Chinese mixed 文本 abc", 8);
        for l in &lines {
            assert!(display_width(l) <= 8, "line too wide: {:?}", l);
        }
        assert!(lines.len() >= 2);
    }

    #[test]
    fn wrap_keeps_newlines() {
        let lines = wrap("第一行\n第二行", 10);
        assert_eq!(lines, vec!["第一行", "第二行"]);
    }

    #[test]
    fn wrap_long_url_hard_breaks() {
        let lines = wrap("https://example.com/very/long/path/here", 10);
        assert!(lines.len() >= 3);
        for l in &lines {
            assert!(display_width(l) <= 10);
        }
    }

    #[test]
    fn wrap_latin_words_not_split() {
        let lines = wrap("hello world foo", 11);
        assert_eq!(lines, vec!["hello world", "foo"]);
    }
}
