//! 加载耗时分析:隔离计时每个插件的 source 时间。

use std::path::Path;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

pub struct BenchResult {
    pub name: String,
    /// 中位耗时(毫秒);纯补全型插件为 0
    pub ms: f64,
    #[allow(dead_code)]
    pub runs: u32,
    /// 纯补全型(无脚本可计时)
    pub completion_only: bool,
    /// 脚本 source 时报错(仍给出耗时)
    pub errored: bool,
}

/// 找到插件的入口脚本 `<name>.plugin.zsh`(或目录内任意 *.plugin.zsh)。
fn entry_script(dir: &Path, name: &str) -> Option<std::path::PathBuf> {
    let prefer = dir.join(format!("{}.plugin.zsh", name));
    if prefer.is_file() {
        return Some(prefer);
    }
    let mut files: Vec<std::path::PathBuf> = std::fs::read_dir(dir)
        .into_iter()
        .flatten()
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| {
            p.file_name()
                .and_then(|n| n.to_str())
                .map(|n| n.ends_with(".plugin.zsh"))
                .unwrap_or(false)
        })
        .collect();
    files.sort();
    files.into_iter().next()
}

/// 对单个插件计时:zsh -f 中 source,重复 runs 次取中位(首次为热身不计)。
pub fn bench_plugin(dir: &Path, name: &str, runs: u32) -> BenchResult {
    let mut res = BenchResult {
        name: name.to_string(),
        ms: 0.0,
        runs,
        completion_only: false,
        errored: false,
    };
    let Some(script) = entry_script(dir, name) else {
        res.completion_only = true;
        return res;
    };
    let root = crate::plugin::zsh_root().display().to_string();
    let file = script.display().to_string();
    let code = format!(
        "source '{root}/lib/git.zsh' 2>/dev/null; \
         source '{root}/lib/prompt_info_functions.zsh' 2>/dev/null; \
         source '{file}' 2>/dev/null"
    );

    let mut times: Vec<f64> = Vec::new();
    for i in 0..=runs {
        // i=0 为热身(冷缓存),不计入
        let mut child = match Command::new("zsh")
            .arg("-f")
            .arg("-c")
            .arg(&code)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
        {
            Ok(c) => c,
            Err(_) => {
                res.errored = true;
                break;
            }
        };
        let start = Instant::now();
        let status = wait_with_timeout(&mut child, Duration::from_secs(10));
        let elapsed = start.elapsed();
        match status {
            // 挂起超时:杀掉并标记,不让 bench 卡死
            None => {
                let _ = child.kill();
                let _ = child.wait();
                res.errored = true;
                break;
            }
            Some(s) if !s.success() => {
                res.errored = true;
                break;
            }
            Some(_) if i > 0 => times.push(elapsed.as_secs_f64() * 1000.0),
            Some(_) => {}
        }
    }
    if !times.is_empty() {
        times.sort_by(|a, b| a.partial_cmp(b).unwrap());
        res.ms = times[times.len() / 2];
    }
    res
}

/// 带超时地等待子进程;超时返回 None(调用方负责 kill)。
fn wait_with_timeout(
    child: &mut std::process::Child,
    timeout: Duration,
) -> Option<std::process::ExitStatus> {
    let start = Instant::now();
    loop {
        match child.try_wait() {
            Ok(Some(status)) => return Some(status),
            Ok(None) if start.elapsed() > timeout => return None,
            Ok(None) => std::thread::sleep(Duration::from_millis(5)),
            Err(_) => return None,
        }
    }
}

/// 中位数格式化:小于 1ms 显示两位小数,否则一位。
pub fn fmt_ms(ms: f64) -> String {
    if ms < 1.0 {
        format!("{:.2} ms", ms)
    } else {
        format!("{:.1} ms", ms)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fmt_ms_shapes() {
        assert_eq!(fmt_ms(0.42), "0.42 ms");
        assert_eq!(fmt_ms(12.34), "12.3 ms");
    }
}
