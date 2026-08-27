//! 轻量 unified diff(行级 LCS),用于保存前预览 zshrc 变更。

/// 生成 unified diff 文本(带 @@ 头,上下文 3 行)。
pub fn unified(a: &str, b: &str, label_a: &str, label_b: &str) -> Vec<String> {
    let old: Vec<&str> = if a.is_empty() {
        Vec::new()
    } else {
        a.lines().collect()
    };
    let new: Vec<&str> = if b.is_empty() {
        Vec::new()
    } else {
        b.lines().collect()
    };
    let n = old.len();
    let m = new.len();

    // LCS 长度表(滚动数组还原路径)
    let mut dp = vec![vec![0usize; m + 1]; n + 1];
    for i in (0..n).rev() {
        for j in (0..m).rev() {
            dp[i][j] = if old[i] == new[j] {
                dp[i + 1][j + 1] + 1
            } else {
                dp[i + 1][j].max(dp[i][j + 1])
            };
        }
    }
    // 编辑脚本:(op, 行内容),op ∈ {=, -, +}
    #[derive(PartialEq, Clone, Copy)]
    enum Op {
        Eq,
        Del,
        Ins,
    }
    let mut script: Vec<(Op, &str)> = Vec::new();
    let (mut i, mut j) = (0, 0);
    while i < n && j < m {
        if old[i] == new[j] {
            script.push((Op::Eq, old[i]));
            i += 1;
            j += 1;
        } else if dp[i + 1][j] >= dp[i][j + 1] {
            script.push((Op::Del, old[i]));
            i += 1;
        } else {
            script.push((Op::Ins, new[j]));
            j += 1;
        }
    }
    while i < n {
        script.push((Op::Del, old[i]));
        i += 1;
    }
    while j < m {
        script.push((Op::Ins, new[j]));
        j += 1;
    }

    let mut out = vec![format!("--- {}", label_a), format!("+++ {}", label_b)];

    const CTX: usize = 3;
    let len = script.len();

    // 所有变更点,按「间隔 ≤ 2*CTX 行上下文」聚组
    let changes: Vec<usize> = (0..len).filter(|&k| script[k].0 != Op::Eq).collect();
    if changes.is_empty() {
        return out;
    }
    let mut groups: Vec<(usize, usize)> = Vec::new(); // 变更点的闭区间
    for &ci in &changes {
        match groups.last_mut() {
            Some(g) if ci - g.1 - 1 <= CTX * 2 => g.1 = ci,
            _ => groups.push((ci, ci)),
        }
    }
    // 向两侧扩 CTX 行上下文(闭区间转渲染区间)
    let mut hunks: Vec<(usize, usize)> = Vec::new(); // [start, end) of script
    for (c0, c1) in groups {
        let s = c0.saturating_sub(CTX);
        let e = (c1 + CTX + 1).min(len);
        match hunks.last_mut() {
            Some(h) if s <= h.1 => h.1 = h.1.max(e),
            _ => hunks.push((s, e)),
        }
    }

    // 前缀计数,换算 hunk 起始行号与行数
    let mut old_pre = vec![0usize; len + 1];
    let mut new_pre = vec![0usize; len + 1];
    for k in 0..len {
        old_pre[k + 1] = old_pre[k] + usize::from(script[k].0 != Op::Ins);
        new_pre[k + 1] = new_pre[k] + usize::from(script[k].0 != Op::Del);
    }
    for (s, e) in hunks {
        out.push(format!(
            "@@ -{},{} +{},{} @@",
            old_pre[s] + 1,
            old_pre[e] - old_pre[s],
            new_pre[s] + 1,
            new_pre[e] - new_pre[s]
        ));
        for (op, text) in &script[s..e] {
            out.push(match op {
                Op::Eq => format!("  {}", text),
                Op::Del => format!("- {}", text),
                Op::Ins => format!("+ {}", text),
            });
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shows_add_and_remove() {
        let a = "plugins=(git docker)\nexport X=1\n";
        let b = "plugins=(git docker z)\nexport X=1\n";
        let d = unified(a, b, "old", "new");
        assert_eq!(d[0], "--- old");
        assert_eq!(d[1], "+++ new");
        assert!(d.iter().any(|l| l.starts_with("@@")));
        assert!(d.iter().any(|l| l.starts_with("+ plugins=(git docker z)")));
        assert!(d.iter().any(|l| l.starts_with("- plugins=(git docker)")));
    }

    #[test]
    fn no_change_no_hunks() {
        let d = unified("a\nb\n", "a\nb\n", "x", "y");
        assert_eq!(d.len(), 2, "只有文件头: {:?}", d);
    }

    #[test]
    fn append_at_end() {
        let d = unified("plugins=(git)\n", "plugins=(git z)\n", "x", "y");
        assert!(d.iter().any(|l| l.contains("+ plugins=(git z)")));
    }
}
