//! omz-pm — Oh My Zsh 插件管理器
//!
//! 默认进入 TUI;也提供 list/info/enable/disable 子命令便于脚本化。

mod aliases;
mod bench;
mod catalog;
mod diff;
mod markdown;
mod plugin;
mod readme;
mod textwrap;
mod theme;
mod ui;
mod zshrc;

use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use catalog::Catalog;
use textwrap::{display_width, pad_to, truncate_to};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const BIN: &str = "omz-pm";

fn main() -> ExitCode {
    // 管道下游提前关闭(如 | head)时按 Unix 惯例静默退出而非 panic
    #[cfg(unix)]
    unsafe {
        libc::signal(libc::SIGPIPE, libc::SIG_DFL);
    }
    let args: Vec<String> = std::env::args().skip(1).collect();
    match run(&args) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("错误: {}", e);
            ExitCode::FAILURE
        }
    }
}

fn run(args: &[String]) -> Result<(), String> {
    // 解析全局选项 --zshrc <path>
    let mut zshrc_path: Option<PathBuf> = None;
    let mut rest: Vec<&String> = Vec::new();
    let mut i = 0;
    while i < args.len() {
        if args[i] == "--zshrc" {
            i += 1;
            let Some(p) = args.get(i) else {
                return Err("--zshrc 需要一个路径参数".into());
            };
            zshrc_path = Some(PathBuf::from(p));
        } else if let Some(p) = args[i].strip_prefix("--zshrc=") {
            zshrc_path = Some(PathBuf::from(p));
        } else {
            rest.push(&args[i]);
        }
        i += 1;
    }
    let zshrc_path = zshrc_path.unwrap_or_else(zshrc::default_zshrc_path);

    let cmd = rest.first().map(|s| s.as_str()).unwrap_or("tui");
    let sub = &rest[1.min(rest.len())..];
    match cmd {
        "tui" | "ui" => ui::run(zshrc_path).map_err(|e| format!("TUI 运行失败: {}", e)),
        "-h" | "--help" | "help" => {
            print_help();
            Ok(())
        }
        "-V" | "--version" | "version" => {
            println!("{} {}", BIN, VERSION);
            Ok(())
        }
        "list" => cmd_list(sub, &zshrc_path),
        "info" => cmd_info(sub, &zshrc_path),
        "which" => cmd_which(sub, &zshrc_path),
        "aliases" => cmd_aliases(sub, &zshrc_path),
        "readme" => cmd_readme(sub),
        "themes" => cmd_themes(&zshrc_path),
        "theme" => cmd_theme(sub, &zshrc_path),
        "bench" => cmd_bench(sub, &zshrc_path),
        "backups" => cmd_backups(sub, &zshrc_path),
        "restore" => cmd_restore(sub, &zshrc_path),
        "enable" | "disable" => cmd_toggle(cmd, sub, &zshrc_path),
        other => {
            eprintln!("未知子命令「{}」\n", other);
            print_help();
            Err("未知子命令".into())
        }
    }
}

fn load_state(zshrc_path: &std::path::Path) -> Result<(Vec<plugin::Plugin>, Catalog), String> {
    let content = fs::read_to_string(zshrc_path)
        .map_err(|e| format!("读取 {} 失败: {}", zshrc_path.display(), e))?;
    let (enabled, _) = zshrc::read_enabled(&content);
    let plugins = plugin::scan(&enabled);
    let catalog = Catalog::load();
    Ok((plugins, catalog))
}

fn find<'a>(plugins: &'a [plugin::Plugin], name: &str) -> Result<&'a plugin::Plugin, String> {
    plugins.iter().find(|p| p.name == name).ok_or_else(|| {
        let names: Vec<&str> = plugins.iter().map(|p| p.name.as_str()).collect();
        let hint = suggest_near(&names, name);
        format!(
            "没有名为「{}」的插件。{}\n用 `{} list` 查看全部插件",
            name, hint, BIN
        )
    })
}

fn suggest_near(names: &[&str], q: &str) -> String {
    let ql = q.to_lowercase();
    let hits: Vec<String> = names
        .iter()
        .filter(|n| n.to_lowercase().contains(&ql))
        .take(5)
        .map(|n| n.to_string())
        .collect();
    if hits.is_empty() {
        String::new()
    } else {
        format!("你是不是想找:{}?", hits.join(", "))
    }
}

fn cmd_list(args: &[&String], zshrc_path: &Path) -> Result<(), String> {
    let only_enabled = args.iter().any(|a| a.as_str() == "--enabled");
    let only_disabled = args.iter().any(|a| a.as_str() == "--disabled");
    let (plugins, catalog) = load_state(zshrc_path)?;

    let enabled_n = plugins.iter().filter(|p| p.enabled).count();
    println!(
        "{} v{} · zshrc: {} · 已启用 {}/{}",
        BIN,
        VERSION,
        zshrc_path.display(),
        enabled_n,
        plugins.len()
    );
    println!();

    let name_w = 26.max(
        plugins
            .iter()
            .map(|p| display_width(&p.name))
            .max()
            .unwrap_or(0)
            + 2,
    );
    for p in &plugins {
        if only_enabled && !p.enabled || only_disabled && p.enabled {
            continue;
        }
        let dot = if p.enabled { "●" } else { "○" };
        let summary = catalog
            .get(&p.name)
            .map(|e| e.summary.as_str())
            .unwrap_or("");
        let src = if p.source == plugin::Source::Custom {
            "[自定义]"
        } else {
            ""
        };
        println!(
            "{} {}  {}{}",
            dot,
            pad_to(&p.name, name_w),
            src,
            truncate_to(summary, 60),
        );
    }
    println!();
    println!(
        "提示: ● 已启用 ○ 未启用;详情见 `{} info <名称>`;交互界面直接运行 `{}`",
        BIN, BIN
    );
    Ok(())
}

fn cmd_info(args: &[&String], zshrc_path: &Path) -> Result<(), String> {
    let Some(name) = args.first() else {
        return Err(format!("用法: {} info <插件名>", BIN));
    };
    let (plugins, catalog) = load_state(zshrc_path)?;
    let p = find(&plugins, name)?;
    let status = if p.enabled {
        "已启用 ✓"
    } else {
        "未启用 ✗"
    };
    let src = match p.source {
        plugin::Source::Bundled => "内置",
        plugin::Source::Custom => "自定义",
    };
    let cat = catalog.get(name).map(|e| e.category()).unwrap_or("-");
    println!(
        "{}({})  [{} · {}]  {}",
        p.name,
        status,
        src,
        cat,
        p.dir.display()
    );
    println!();
    let w = term_width().saturating_sub(4);
    match catalog.get(name) {
        Some(e) => {
            println!("{}", e.summary);
            if !e.detail.is_empty() {
                println!();
                for line in crate::textwrap::wrap(&e.detail, w) {
                    println!("{}", line);
                }
            }
            if !e.usage.is_empty() {
                println!();
                println!("── 实战用法 ──");
                for para in e.usage.split('\n') {
                    if para.is_empty() {
                        println!();
                    } else {
                        for line in crate::textwrap::wrap(para, w) {
                            println!("{}", line);
                        }
                    }
                }
            }
            if !e.aliases.is_empty() {
                println!();
                println!("── 常用别名 ──");
                for (a, note) in &e.aliases {
                    println!("  {:<10} {}", a, note);
                }
            }
        }
        None => {
            println!("(无中文词典条目)");
            if let Some(ex) = catalog::readme_excerpt(&p.dir) {
                println!();
                println!("── English ──");
                for line in crate::textwrap::wrap(&ex, w) {
                    println!("{}", line);
                }
            }
        }
    }
    // 源码别名索引
    let defs = aliases::extract_from_dir(&p.dir);
    if !defs.is_empty() {
        println!();
        println!("── 别名索引(源码提取,共 {} 条)──", defs.len());
        for d in defs.iter().take(40) {
            println!("  {:<10} = {}", d.name, d.command);
        }
        if defs.len() > 40 {
            println!("  … 其余 {} 条略", defs.len() - 40);
        }
    }
    Ok(())
}

/// omz-pm which <别名>:反查别名来自哪个插件、含义是什么。
fn cmd_which(args: &[&String], zshrc_path: &Path) -> Result<(), String> {
    let Some(token) = args.first() else {
        return Err(format!("用法: {} which <别名或命令>", BIN));
    };
    let (plugins, catalog) = load_state(zshrc_path)?;
    let index = aliases::build_index(&plugins);
    let tok = token.as_str();

    let mut exact: Vec<(&String, &aliases::AliasDef)> = Vec::new();
    let mut partial: Vec<(&String, &aliases::AliasDef)> = Vec::new();
    for (a, pname, d) in &index {
        if a == tok {
            exact.push((pname, d));
        } else if a.contains(tok) {
            partial.push((pname, d));
        }
    }
    let hits = if exact.is_empty() { &partial } else { &exact };
    if hits.is_empty() {
        return Err(format!("没有插件定义过「{}」这个别名", token));
    }
    for (pname, d) in hits {
        let note = catalog
            .get(pname)
            .and_then(|e| e.aliases.get(&d.name))
            .cloned();
        match note {
            Some(n) => println!(
                "{}  ←  {} 插件:{}\n    原命令: {}",
                d.name, pname, n, d.command
            ),
            None => println!("{}  ←  {} 插件\n    原命令: {}", d.name, pname, d.command),
        }
    }
    Ok(())
}

/// omz-pm aliases <插件>:列出某插件提供的全部别名。
fn cmd_aliases(args: &[&String], zshrc_path: &Path) -> Result<(), String> {
    let Some(name) = args.first() else {
        return Err(format!("用法: {} aliases <插件名>", BIN));
    };
    let (plugins, catalog) = load_state(zshrc_path)?;
    let p = find(&plugins, name)?;
    let defs = aliases::extract_from_dir(&p.dir);
    if defs.is_empty() {
        println!(
            "插件 {} 没有定义任何 alias(可能是补全型/功能型插件)",
            p.name
        );
        return Ok(());
    }
    println!("{} 共定义 {} 个别名:", p.name, defs.len());
    println!();
    for d in &defs {
        let note = catalog
            .get(&p.name)
            .and_then(|e| e.aliases.get(&d.name))
            .map(|n| format!("  # {}", n))
            .unwrap_or_default();
        println!("  {:<12} {}{}", d.name, d.command, note);
    }
    Ok(())
}

/// omz-pm readme <名称>:输出翻译辅助后的 README。
fn cmd_readme(args: &[&String]) -> Result<(), String> {
    let Some(name) = args.first() else {
        return Err(format!("用法: {} readme <插件名>", BIN));
    };
    let plugins = plugin::scan(&Default::default());
    let p = find(&plugins, name)?;
    match readme::read_translated(&p.name, &p.dir) {
        Some(text) => {
            println!("{}", text);
            Ok(())
        }
        None => Err(format!("插件 {} 没有 README", p.name)),
    }
}

/// omz-pm themes:列出全部主题。
fn cmd_themes(zshrc_path: &Path) -> Result<(), String> {
    let themes = theme::scan();
    let current = fs::read_to_string(zshrc_path)
        .ok()
        .and_then(|c| zshrc::read_theme(&c))
        .unwrap_or_default();
    println!(
        "主题共 {} 个,当前:{}",
        themes.len(),
        if current.is_empty() {
            "(默认 robbyrussell 未显式设置)".to_string()
        } else {
            current.clone()
        }
    );
    println!();
    for t in &themes {
        let mark = if t.name == current { "●" } else { "○" };
        let src = if t.source == plugin::Source::Custom {
            "  [自定义]"
        } else {
            ""
        };
        println!("{} {}{}", mark, pad_to(&t.name, 28), src);
    }
    println!();
    println!(
        "提示: `{} theme <名称>` 启用;`{} theme --preview <名称>` 查看渲染效果",
        BIN, BIN
    );
    Ok(())
}

/// omz-pm theme <名称> | theme --preview <名称>
fn cmd_theme(args: &[&String], zshrc_path: &Path) -> Result<(), String> {
    let (preview, name): (bool, String) = match args.len() {
        1 => (false, args[0].clone()),
        2 if args[0] == "--preview" || args[0] == "-p" => (true, args[1].clone()),
        _ => {
            return Err(format!(
                "用法: {} theme <名称>   或   {} theme --preview <名称>",
                BIN, BIN
            ))
        }
    };
    let themes = theme::scan();
    let Some(t) = themes.iter().find(|t| t.name == name) else {
        let hint = themes
            .iter()
            .map(|t| t.name.clone())
            .filter(|n| n.contains(&name))
            .take(5)
            .collect::<Vec<_>>()
            .join(", ");
        return Err(format!(
            "没有名为「{}」的主题。{}",
            name,
            if hint.is_empty() {
                String::new()
            } else {
                format!("你是不是想找:{}?", hint)
            }
        ));
    };
    if preview {
        match theme::preview_ansi(t) {
            Some(ansi) => {
                println!("「{}」提示符效果:", t.name);
                println!("\x1b[2m$\x1b[0m {}", ansi);
                println!("\x1b[2m$\x1b[0m");
            }
            None => return Err(format!("主题「{}」渲染失败", t.name)),
        }
        return Ok(());
    }
    // 启用:diff 预览 + 确认(tty 时)
    let content = fs::read_to_string(zshrc_path)
        .map_err(|e| format!("读取 {} 失败: {}", zshrc_path.display(), e))?;
    let (new_content, warnings) = zshrc::apply_theme(&content, &t.name);
    for l in diff::unified(
        &content,
        &new_content,
        "当前 zshrc",
        &format!("主题 → {}", t.name),
    ) {
        println!("{}", l);
    }
    if is_tty() {
        print!("\n确认启用主题「{}」?[y/N] ", t.name);
        use std::io::Write;
        std::io::stdout().flush().ok();
        let mut line = String::new();
        std::io::stdin()
            .read_line(&mut line)
            .map_err(|e| e.to_string())?;
        if !line.trim().eq_ignore_ascii_case("y") {
            println!("已取消");
            return Ok(());
        }
    }
    let bak = zshrc::save_with_backup(zshrc_path, &new_content)
        .map_err(|e| format!("写入失败: {}(zshrc 未被改动)", e))?;
    for w in warnings {
        println!("⚠ {}", w);
    }
    println!(
        "✓ 主题已设为「{}」,重开终端生效。备份: {}",
        t.name,
        bak.display()
    );
    Ok(())
}

/// omz-pm bench [--runs N] [--all]:已启用插件加载耗时。
fn cmd_bench(args: &[&String], zshrc_path: &Path) -> Result<(), String> {
    let mut runs = 3u32;
    let mut all = false;
    let mut it = args.iter();
    while let Some(a) = it.next() {
        match a.as_str() {
            "--runs" | "-n" => {
                let v = it.next().ok_or("--runs 需要一个数字")?;
                runs = v.parse().map_err(|_| "--runs 需要一个数字")?;
                runs = runs.clamp(1, 10);
            }
            "--all" | "-a" => all = true,
            other => return Err(format!("未知参数「{}」", other)),
        }
    }
    let (plugins, _c) = load_state(zshrc_path)?;
    let targets: Vec<&plugin::Plugin> = plugins.iter().filter(|p| all || p.enabled).collect();
    if targets.is_empty() {
        println!("没有可分析的插件");
        return Ok(());
    }
    println!(
        "正在分析 {} 个插件(每个 source {} 次,热身 1 次不计)…",
        targets.len(),
        runs
    );
    let mut results = Vec::new();
    for p in &targets {
        results.push(bench::bench_plugin(&p.dir, &p.name, runs));
    }
    results.sort_by(|a, b| b.ms.partial_cmp(&a.ms).unwrap());
    println!();
    println!("{:<24} {:>10}  {}", "插件", "中位耗时", String::new());
    println!("{}", "-".repeat(52));
    let mut total = 0.0;
    for r in &results {
        total += r.ms;
        let note = if r.completion_only {
            "纯补全型,无脚本"
        } else if r.errored {
            "source 报错(仍给出耗时)"
        } else {
            ""
        };
        println!("{:<24} {:>10}  {}", r.name, bench::fmt_ms(r.ms), note);
    }
    println!("{}", "-".repeat(52));
    println!("{:<24} {:>10}", "合计(近似)", bench::fmt_ms(total));
    println!();
    println!("说明:为单插件隔离计时,未含插件间依赖与 zshrc 其他部分,仅供横向比较。");
    Ok(())
}

/// omz-pm backups [--clean [--keep N]]
fn cmd_backups(args: &[&String], zshrc_path: &Path) -> Result<(), String> {
    let clean = args.iter().any(|a| a.as_str() == "--clean");
    let keep = args
        .iter()
        .position(|a| a.as_str() == "--keep")
        .and_then(|i| args.get(i + 1))
        .and_then(|v| v.parse::<usize>().ok());
    let backups = zshrc::list_backups(zshrc_path);
    if backups.is_empty() {
        println!("还没有任何备份(每次保存会自动生成)");
        return Ok(());
    }
    if clean {
        let keep_n = keep.unwrap_or(0);
        let (to_delete, to_keep) = backups.split_at(backups.len().saturating_sub(keep_n));
        if to_delete.is_empty() {
            println!("没有需要清理的备份");
            return Ok(());
        }
        for p in to_delete {
            std::fs::remove_file(p).map_err(|e| format!("删除失败: {}", e))?;
            println!("已删除 {}", p.display());
        }
        println!("保留最近 {} 个备份", to_keep.len());
        return Ok(());
    }
    println!("{} 的备份(新 → 旧):", zshrc_path.display());
    println!();
    for (i, p) in backups.iter().enumerate() {
        let size = std::fs::metadata(p).map(|m| m.len()).unwrap_or(0);
        println!(
            "  [{:>2}] {}  {:>6} B",
            i,
            p.file_name().and_then(|n| n.to_str()).unwrap_or("?"),
            size
        );
    }
    println!();
    println!(
        "恢复: {} restore <序号>   清理: {} backups --clean [--keep N]",
        BIN, BIN
    );
    Ok(())
}

/// omz-pm restore <序号|路径>
fn cmd_restore(args: &[&String], zshrc_path: &Path) -> Result<(), String> {
    let Some(sel) = args.first() else {
        return Err(format!(
            "用法: {} restore <序号|路径>   先用 `{} backups` 查看列表",
            BIN, BIN
        ));
    };
    let backups = zshrc::list_backups(zshrc_path);
    let target: std::path::PathBuf = match sel.parse::<usize>() {
        Ok(idx) => {
            if idx >= backups.len() {
                return Err(format!("序号超出范围,共 {} 个备份", backups.len()));
            }
            backups[idx].clone()
        }
        Err(_) => {
            let p = PathBuf::from(sel.as_str());
            if !p.is_file() {
                return Err(format!("找不到备份文件: {}", sel));
            }
            p
        }
    };
    println!("将用 {} 覆盖 {}", target.display(), zshrc_path.display());
    if is_tty() {
        print!("确认恢复?[y/N] ");
        use std::io::Write;
        std::io::stdout().flush().ok();
        let mut line = String::new();
        std::io::stdin()
            .read_line(&mut line)
            .map_err(|e| e.to_string())?;
        if !line.trim().eq_ignore_ascii_case("y") {
            println!("已取消");
            return Ok(());
        }
    }
    let snapshot =
        zshrc::restore_backup(zshrc_path, &target).map_err(|e| format!("恢复失败: {}", e))?;
    println!("✓ 已恢复。恢复前的内容另存为: {}", snapshot.display());
    Ok(())
}

fn is_tty() -> bool {
    // 简单判断:stdin 可读且未重定向管道时 tput 存在意义不大,直接探测 /dev/tty
    std::fs::File::open("/dev/tty").is_ok()
}

fn cmd_toggle(action: &str, args: &[&String], zshrc_path: &Path) -> Result<(), String> {
    if args.is_empty() {
        return Err(format!("用法: {} {} <插件名>...", BIN, action));
    }
    let (plugins, _catalog) = load_state(zshrc_path)?;
    let mut enable: Vec<String> = Vec::new();
    let mut disable: Vec<String> = Vec::new();

    // 先校验全部名字存在再动手
    let valid: HashSet<&str> = plugins.iter().map(|p| p.name.as_str()).collect();
    for name in args {
        if !valid.contains(name.as_str()) {
            return Err(find(&plugins, name).unwrap_err());
        }
        if action == "enable" {
            enable.push(name.to_string());
        } else {
            disable.push(name.to_string());
        }
    }

    let content = fs::read_to_string(zshrc_path)
        .map_err(|e| format!("读取 {} 失败: {}", zshrc_path.display(), e))?;
    let (new_content, warnings) = zshrc::apply_changes(&content, &enable, &disable)?;
    let bak = zshrc::save_with_backup(zshrc_path, &new_content)
        .map_err(|e| format!("写入失败: {}(zshrc 未被改动)", e))?;

    for w in warnings {
        println!("⚠ {}", w);
    }
    for name in enable {
        println!("✓ 已启用 {}(重开终端生效)", name);
    }
    for name in disable {
        println!("✓ 已禁用 {}(重开终端生效)", name);
    }
    println!("备份已保存到: {}", bak.display());
    Ok(())
}

fn term_width() -> usize {
    if let Some(cols) = std::env::var("COLUMNS")
        .ok()
        .and_then(|c| c.parse::<usize>().ok())
    {
        return cols;
    }
    // 无 tty 时给个兜底宽度
    terminal_size_fallback()
}

fn terminal_size_fallback() -> usize {
    use std::process::{Command, Stdio};
    Command::new("tput")
        .arg("cols")
        .stdin(Stdio::null())
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .and_then(|s| s.trim().parse().ok())
        .unwrap_or(100)
}

fn print_help() {
    println!(
        "{bin} v{ver} — Oh My Zsh 插件管理器(内置中文说明词典)

用法:
  {bin}                    进入 TUI 交互界面(默认)
  {bin} tui                同上
  {bin} list               列出全部插件与状态
  {bin} list --enabled     只看已启用
  {bin} list --disabled    只看未启用
  {bin} info <名称>         查看插件中文说明与用法
  {bin} which <别名>        反查别名来自哪个插件、什么含义
  {bin} aliases <名称>      列出某插件定义的全部别名
  {bin} readme <名称>       输出翻译辅助后的 README
  {bin} themes              列出全部主题
  {bin} theme <名称>        启用主题(diff 确认)
  {bin} theme --preview <名> 渲染主题提示符效果
  {bin} bench [--runs N] [--all]  已启用插件加载耗时分析
  {bin} backups             查看备份列表(--clean --keep N 清理)
  {bin} restore <序号|路径>  恢复备份
  {bin} enable <名称>...    启用一个或多个插件
  {bin} disable <名称>...   禁用一个或多个插件

TUI 快捷键:
  插件视图: ↑↓/jk 移动  空格 切换  Tab 筛选  c 分类  / 搜索  r README  s 保存(diff 预览)  b 备份  T 主题视图  ? 帮助  q 退出
  主题视图: ↑↓/jk 选择  i 试穿(临时 zsh 实测)  p 彩色快览  Enter 启用  T 返回插件视图

选项:
  --zshrc <路径>            指定 zshrc 文件(默认 $ZDOTDIR/.zshrc 或 ~/.zshrc)
  -h, --help                显示本帮助
  -V, --version             显示版本

说明:
  · 插件列表扫描 $ZSH/plugins 与 $ZSH_CUSTOM/plugins
  · 每次修改都会先备份为 <zshrc>.omz-pm.bak.<时间戳>
  · 修改保存后需重新打开终端(或 source ~/.zshrc)才会生效",
        bin = BIN,
        ver = VERSION
    );
}
