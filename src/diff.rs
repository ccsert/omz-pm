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

    // 编辑脚本:(op, 行内容),op ∈ {=, -, +}
    #[derive(PartialEq, Clone, Copy)]
    enum Op {
        Eq,
        Del,
        Ins,
    }
    let mut script: Vec<(Op, &str)> = Vec::new();
    let (mut i, mut j) = (0usize, 0usize);

    // 先裁掉公共前后缀:zshrc 变更通常集中在一处,DP 规模可骤降
    while i < n && j < m && old[i] == new[j] {
        script.push((Op::Eq, old[i]));
        i += 1;
        j += 1;
    }
    let (mut ei, mut ej) = (n, m);
    while ei > i && ej > j && old[ei - 1] == new[ej - 1] {
        ei -= 1;
        ej -= 1;
    }

    // 中段 LCS(滚动全量表)。规模封顶:超过上限时退化为整段删+整段插,
    // 避免 O(n·m) 内存爆炸(封顶值 ≈ 128 MB usize)。
    let mid_old = &old[i..ei];
    let mid_new = &new[j..ej];
    if mid_old.len() * mid_new.len() <= 16_000_000 {
        let mid_n = mid_old.len();
        let mid_m = mid_new.len();
        let mut dp = vec![vec![0usize; mid_m + 1]; mid_n + 1];
        for x in (0..mid_n).rev() {
            for y in (0..mid_m).rev() {
                dp[x][y] = if mid_old[x] == mid_new[y] {
                    dp[x + 1][y + 1] + 1
                } else {
                    dp[x + 1][y].max(dp[x][y + 1])
                };
            }
        }
        let (mut x, mut y) = (0, 0);
        while x < mid_n && y < mid_m {
            if mid_old[x] == mid_new[y] {
                script.push((Op::Eq, mid_old[x]));
                x += 1;
                y += 1;
            } else if dp[x + 1][y] >= dp[x][y + 1] {
                script.push((Op::Del, mid_old[x]));
                x += 1;
            } else {
                script.push((Op::Ins, mid_new[y]));
                y += 1;
            }
        }
        while x < mid_n {
            script.push((Op::Del, mid_old[x]));
            x += 1;
        }
        while y < mid_m {
            script.push((Op::Ins, mid_new[y]));
            y += 1;
        }
    } else {
        for l in mid_old {
            script.push((Op::Del, l));
        }
        for l in mid_new {
            script.push((Op::Ins, l));
        }
    }

    // 公共后缀与文件尾部剩余(某文件更长时)
    i = ei;
    j = ej;
    while i < n && j < m && old[i] == new[j] {
        script.push((Op::Eq, old[i]));
        i += 1;
        j += 1;
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

    #[test]
    fn large_files_with_single_change_stay_cheap() {
        // 公共前后缀裁剪:2 万行只改一行,不应触发全量 DP(改动前约 3 GB 内存)
        let mut a = String::new();
        let mut b = String::new();
        for k in 0..20_000 {
            a.push_str(&format!("line {k}\n"));
            b.push_str(&format!("line {}\n", if k == 10_000 { k + 1 } else { k }));
        }
        let d = unified(&a, &b, "x", "y");
        assert!(d.iter().any(|l| l.contains("- line 10000")));
        assert!(d.iter().any(|l| l.contains("+ line 10001")));
    }

    #[test]
    fn pure_insertion_and_deletion() {
        let d = unified("a\nb\n", "a\nb\nc\nd\n", "x", "y");
        assert!(d.iter().any(|l| l.contains("+ c")));
        assert!(d.iter().any(|l| l.contains("+ d")));
        let d = unified("a\nb\nc\nd\n", "a\nb\n", "x", "y");
        assert!(d.iter().any(|l| l.contains("- c")));
        assert!(d.iter().any(|l| l.contains("- d")));
    }
}
