//! ratatui TUI:插件列表 + 中文详情(用法/别名)+ 搜索/分类筛选 + diff 保存确认。

use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::PathBuf;

use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, List, ListItem, ListState, Paragraph, Tabs},
    Frame, Terminal,
};
use unicode_width::UnicodeWidthStr;

use crate::{
    aliases,
    catalog::{readme_excerpt, Catalog},
    diff, markdown,
    plugin::Plugin,
    readme, textwrap,
    theme::{self, ThemeInfo},
    zshrc,
};

#[derive(Clone, Copy, PartialEq)]
enum View {
    Plugins,
    Themes,
}

#[derive(Clone, Copy, PartialEq)]
enum Direction_ {
    Next,
    Prev,
}

#[derive(Clone, Copy, PartialEq)]
enum Filter {
    All,
    Enabled,
    Disabled,
}

impl Filter {
    fn next(self) -> Self {
        match self {
            Filter::All => Filter::Enabled,
            Filter::Enabled => Filter::Disabled,
            Filter::Disabled => Filter::All,
        }
    }
    fn prev(self) -> Self {
        self.next().next()
    }
}

enum Modal {
    None,
    Help,
    QuitConfirm,
    /// 保存前 diff 预览:lines + 滚动位置 + 确认后执行的动作
    Diff {
        lines: Vec<Line<'static>>,
        scroll: u16,
        action: SaveAction,
    },
    /// README 阅读器
    Readme {
        title: String,
        lines: Vec<Line<'static>>,
        scroll: u16,
    },
    /// 备份管理
    Backups {
        items: Vec<std::path::PathBuf>,
        selected: usize,
    },
    /// 恢复备份确认
    RestoreConfirm {
        backup: std::path::PathBuf,
    },
}

/// diff 确认后要执行的动作。
#[derive(Clone)]
enum SaveAction {
    Plugins,
    Theme(String),
}

struct App {
    view: View,
    plugins: Vec<Plugin>,
    catalog: Catalog,
    excerpts: HashMap<String, String>,
    /// 词典缺失插件的源码别名索引缓存(惰性填充)
    aliases_cache: HashMap<String, Vec<aliases::AliasDef>>,
    /// 保存目标(相对最初状态的覆盖):name -> 目标启用与否
    pending: HashMap<String, bool>,
    filter: Filter,
    /// 分类筛选:None = 全部分类
    cat_idx: Option<usize>,
    categories: Vec<String>,
    search: String,
    searching: bool,
    selected: usize,
    filtered: Vec<usize>,
    detail_scroll: usize,
    modal: Modal,
    message: Option<String>,
    zshrc_path: PathBuf,
    should_quit: bool,
    /// 主题视图
    themes: Vec<ThemeInfo>,
    theme_selected: usize,
    /// 当前生效主题(read_theme 的结果)
    current_theme: Option<String>,
}

impl App {
    fn new(zshrc_path: PathBuf) -> App {
        let (enabled, _n) = read_enabled_of(&zshrc_path);
        let plugins = crate::plugin::scan(&enabled);
        let catalog = Catalog::load();
        let categories = catalog.categories();
        let mut excerpts = HashMap::new();
        for p in &plugins {
            if catalog.get(&p.name).is_none() {
                if let Some(ex) = readme_excerpt(&p.dir) {
                    excerpts.insert(p.name.clone(), ex);
                }
            }
        }
        let mut aliases_cache = HashMap::new();
        for p in &plugins {
            aliases_cache.insert(p.name.clone(), aliases::extract_from_dir(&p.dir));
        }
        let themes = theme::scan();
        let current_theme = fs::read_to_string(&zshrc_path)
            .ok()
            .and_then(|c| zshrc::read_theme(&c));
        App {
            view: View::Plugins,
            plugins,
            catalog,
            categories,
            excerpts,
            aliases_cache: HashMap::new(),
            pending: HashMap::new(),
            filter: Filter::All,
            cat_idx: None,
            search: String::new(),
            searching: false,
            selected: 0,
            filtered: Vec::new(),
            detail_scroll: 0,
            modal: Modal::None,
            message: None,
            zshrc_path,
            should_quit: false,
            themes,
            theme_selected: 0,
            current_theme,
        }
    }

    #[allow(dead_code)] // 预留:后续主题详情高亮当前项
    fn current_theme_info(&self) -> Option<&ThemeInfo> {
        let cur = self.current_theme.as_deref()?;
        self.themes.iter().find(|t| t.name == cur)
    }

    fn current_category(&self) -> Option<&String> {
        self.cat_idx.and_then(|i| self.categories.get(i))
    }

    fn category_of(&self, name: &str) -> Option<&str> {
        self.catalog.get(name).map(|e| e.category())
    }

    fn is_effective(&self, name: &str) -> bool {
        if let Some(target) = self.pending.get(name) {
            return *target;
        }
        self.plugins
            .iter()
            .find(|p| p.name == name)
            .map(|p| p.enabled)
            .unwrap_or(false)
    }

    fn refresh_filtered(&mut self) {
        let q = self.search.to_lowercase();
        let cat = self.current_category().cloned();
        self.filtered = self
            .plugins
            .iter()
            .enumerate()
            .filter(|(_, p)| match self.filter {
                Filter::All => true,
                Filter::Enabled => self.is_effective(&p.name),
                Filter::Disabled => !self.is_effective(&p.name),
            })
            .filter(|(_, p)| match &cat {
                Some(c) => self.category_of(&p.name) == Some(c.as_str()),
                None => true,
            })
            .filter(|(_, p)| {
                if q.is_empty() {
                    return true;
                }
                let in_name = p.name.to_lowercase().contains(&q);
                let in_dict = self
                    .catalog
                    .get(&p.name)
                    .map(|e| {
                        e.summary.contains(&self.search)
                            || e.detail.contains(&self.search)
                            || e.category().contains(&self.search)
                    })
                    .unwrap_or(false);
                in_name || in_dict
            })
            .map(|(i, _)| i)
            .collect();
        if self.selected >= self.filtered.len() {
            self.selected = self.filtered.len().saturating_sub(1);
        }
    }

    fn current(&self) -> Option<&Plugin> {
        self.filtered
            .get(self.selected)
            .and_then(|i| self.plugins.get(*i))
    }

    /// 该插件源码里定义的别名(启动时已全部提取)。
    fn aliases_of(&self, name: &str) -> Vec<aliases::AliasDef> {
        self.aliases_cache.get(name).cloned().unwrap_or_default()
    }

    fn toggle_current(&mut self) {
        let Some(p) = self.current() else { return };
        let name = p.name.clone();
        let target = !self.is_effective(&name);
        let original = p.enabled;
        if target == original {
            self.pending.remove(&name);
        } else {
            self.pending.insert(name, target);
        }
        self.refresh_filtered();
    }

    fn cycle_category(&mut self, dir: Direction_) {
        if self.categories.is_empty() {
            return;
        }
        self.cat_idx = match (self.cat_idx, dir) {
            (None, Direction_::Next) => Some(0),
            (None, Direction_::Prev) => Some(self.categories.len() - 1),
            (Some(i), Direction_::Next) if i + 1 < self.categories.len() => Some(i + 1),
            (Some(_), Direction_::Next) => None,
            (Some(0), Direction_::Prev) => None,
            (Some(i), Direction_::Prev) => Some(i - 1),
        };
        self.selected = 0;
        self.refresh_filtered();
    }

    fn dirty_count(&self) -> usize {
        self.pending.len()
    }

    /// 计算 diff 预览;无待保存项时返回 None。
    fn prepare_save(&self) -> Result<Option<Vec<Line<'static>>>, String> {
        if self.pending.is_empty() {
            return Ok(None);
        }
        let (mut enable, mut disable): (Vec<String>, Vec<String>) = (Vec::new(), Vec::new());
        for (name, target) in &self.pending {
            if *target {
                enable.push(name.clone());
            } else {
                disable.push(name.clone());
            }
        }
        let content = fs::read_to_string(&self.zshrc_path)
            .map_err(|e| format!("读取 {} 失败: {}", self.zshrc_path.display(), e))?;
        let (new_content, _) = zshrc::apply_changes(&content, &enable, &disable)?;
        let lines = diff::unified(&content, &new_content, "当前 zshrc", "保存后")
            .into_iter()
            .map(|l| {
                if l.starts_with('+') {
                    Line::from(l).style(Style::default().fg(Color::Green))
                } else if l.starts_with('-') {
                    Line::from(l).style(Style::default().fg(Color::Red))
                } else if l.starts_with("@@") {
                    Line::from(l).style(Style::default().fg(Color::Cyan))
                } else {
                    Line::from(l)
                }
            })
            .collect();
        Ok(Some(lines))
    }

    fn save(&mut self) -> Result<Vec<String>, String> {
        let mut enable = Vec::new();
        let mut disable = Vec::new();
        for (name, target) in &self.pending {
            if *target {
                enable.push(name.clone());
            } else {
                disable.push(name.clone());
            }
        }
        let content = fs::read_to_string(&self.zshrc_path)
            .map_err(|e| format!("读取 {} 失败: {}", self.zshrc_path.display(), e))?;
        let (new_content, warnings) = zshrc::apply_changes(&content, &enable, &disable)?;
        let bak = zshrc::save_with_backup(&self.zshrc_path, &new_content)
            .map_err(|e| format!("写入失败: {}(zshrc 未被改动)", e))?;
        for (name, target) in &self.pending {
            if let Some(p) = self.plugins.iter_mut().find(|p| &p.name == name) {
                p.enabled = *target;
            }
        }
        self.pending.clear();
        self.message = Some(format!("已保存 ✓ 备份: {}", bak.display()));
        Ok(warnings)
    }

    fn total_counts(&self) -> (usize, usize) {
        let enabled_now = self
            .plugins
            .iter()
            .filter(|p| self.is_effective(&p.name))
            .count();
        (enabled_now, self.plugins.len())
    }

    /// 计算「设为主题 X」的 diff 预览。
    fn prepare_theme_save(&self, theme_name: &str) -> Result<Vec<Line<'static>>, String> {
        let content = fs::read_to_string(&self.zshrc_path)
            .map_err(|e| format!("读取 {} 失败: {}", self.zshrc_path.display(), e))?;
        let (new_content, _) = zshrc::apply_theme(&content, theme_name);
        let lines = diff::unified(
            &content,
            &new_content,
            "当前 zshrc",
            &format!("主题 → {}", theme_name),
        )
        .into_iter()
        .map(|l| {
            if l.starts_with('+') {
                Line::from(l).style(Style::default().fg(Color::Green))
            } else if l.starts_with('-') {
                Line::from(l).style(Style::default().fg(Color::Red))
            } else if l.starts_with("@@") {
                Line::from(l).style(Style::default().fg(Color::Cyan))
            } else {
                Line::from(l)
            }
        })
        .collect();
        Ok(lines)
    }

    /// 写入主题设置。
    fn save_theme(&mut self, theme_name: &str) -> Result<(), String> {
        let content = fs::read_to_string(&self.zshrc_path)
            .map_err(|e| format!("读取 {} 失败: {}", self.zshrc_path.display(), e))?;
        let (new_content, _) = zshrc::apply_theme(&content, theme_name);
        let bak = zshrc::save_with_backup(&self.zshrc_path, &new_content)
            .map_err(|e| format!("写入失败: {}(zshrc 未被改动)", e))?;
        self.current_theme = Some(theme_name.to_string());
        self.message = Some(format!(
            "主题已设为「{}」✓ 备份: {}",
            theme_name,
            bak.display()
        ));
        Ok(())
    }

    /// 打开当前插件的 README 阅读器。
    fn open_readme(&mut self) {
        let Some(p) = self.current() else {
            self.message = Some("没有选中的插件".to_string());
            return;
        };
        match readme::read_translated(&p.name, &p.dir) {
            Some(text) => {
                let lines = markdown::render(&text, 100);
                let title = format!(" README · {} ", p.name);
                self.modal = Modal::Readme {
                    title,
                    lines,
                    scroll: 0,
                };
            }
            None => self.message = Some("该插件没有 README".to_string()),
        }
    }

    /// 恢复指定备份(会先快照当前 zshrc)。
    fn do_restore(&mut self, backup: &std::path::Path) {
        match zshrc::restore_backup(&self.zshrc_path, backup) {
            Ok(snapshot) => {
                self.message = Some(format!("已恢复 ✓ 恢复前内容已另存: {}", snapshot.display()));
            }
            Err(e) => self.message = Some(format!("恢复失败: {}", e)),
        }
    }
}

fn read_enabled_of(path: &std::path::Path) -> (std::collections::HashSet<String>, usize) {
    fs::read_to_string(path)
        .map(|c| zshrc::read_enabled(&c))
        .unwrap_or_else(|_| (Default::default(), 0))
}

pub fn run(zshrc_path: PathBuf) -> io::Result<()> {
    let mut terminal = init_terminal()?;
    let res = event_loop(&mut terminal, zshrc_path);
    restore_terminal()?;
    res
}

fn init_terminal() -> io::Result<Terminal<CrosstermBackend<io::Stdout>>> {
    crossterm::terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    crossterm::execute!(
        stdout,
        crossterm::terminal::EnterAlternateScreen,
        crossterm::event::EnableMouseCapture
    )?;
    Terminal::new(CrosstermBackend::new(stdout))
}

fn restore_terminal() -> io::Result<()> {
    crossterm::execute!(
        io::stdout(),
        crossterm::event::DisableMouseCapture,
        crossterm::terminal::LeaveAlternateScreen
    )?;
    crossterm::terminal::disable_raw_mode()?;
    Ok(())
}

type Term = Terminal<CrosstermBackend<io::Stdout>>;

fn event_loop(terminal: &mut Term, zshrc_path: PathBuf) -> io::Result<()> {
    use crossterm::event::{Event, KeyCode, KeyEventKind, KeyModifiers};
    let mut app = App::new(zshrc_path);
    app.refresh_filtered();

    loop {
        terminal.draw(|f| draw(f, &app))?;
        if let Event::Key(key) = crossterm::event::read()? {
            if key.kind != KeyEventKind::Press {
                continue;
            }
            app.message = None;
            match &mut app.modal {
                Modal::Help => match key.code {
                    KeyCode::Esc | KeyCode::Char('?') | KeyCode::Char('q') => {
                        app.modal = Modal::None;
                    }
                    _ => {}
                },
                Modal::Readme { lines, scroll, .. } => {
                    let max = lines.len().saturating_sub(1) as u16;
                    match key.code {
                        KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('r') => {
                            app.modal = Modal::None;
                        }
                        KeyCode::Down | KeyCode::Char('j') => {
                            *scroll = (*scroll + 1).min(max);
                        }
                        KeyCode::Up | KeyCode::Char('k') => {
                            *scroll = scroll.saturating_sub(1);
                        }
                        KeyCode::PageDown => *scroll = (*scroll + 20).min(max),
                        KeyCode::PageUp => *scroll = scroll.saturating_sub(20),
                        _ => {}
                    }
                }
                Modal::Backups { items, selected } => match key.code {
                    KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('b') => {
                        app.modal = Modal::None;
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        if *selected + 1 < items.len() {
                            *selected += 1;
                        }
                    }
                    KeyCode::Up | KeyCode::Char('k') => {
                        *selected = selected.saturating_sub(1);
                    }
                    KeyCode::Enter => {
                        let path = items[*selected].clone();
                        app.modal = Modal::RestoreConfirm { backup: path };
                    }
                    _ => {}
                },
                Modal::RestoreConfirm { backup } => match key.code {
                    KeyCode::Enter | KeyCode::Char('y') => {
                        let path = backup.clone();
                        app.do_restore(&path);
                        app.modal = Modal::None;
                    }
                    KeyCode::Esc | KeyCode::Char('n') | KeyCode::Char('q') => {
                        app.modal = Modal::None;
                    }
                    _ => {}
                },
                Modal::QuitConfirm => match key.code {
                    KeyCode::Char('q') => app.should_quit = true,
                    KeyCode::Esc | KeyCode::Char('n') => app.modal = Modal::None,
                    KeyCode::Char('s') => match app.save() {
                        Ok(_) => app.should_quit = true,
                        Err(e) => app.message = Some(format!("保存失败: {}", e)),
                    },
                    _ => {}
                },
                Modal::Diff {
                    lines,
                    scroll,
                    action,
                } => {
                    let max = lines.len().saturating_sub(1) as u16;
                    match key.code {
                        KeyCode::Esc | KeyCode::Char('n') | KeyCode::Char('q') => {
                            app.modal = Modal::None;
                        }
                        KeyCode::Enter | KeyCode::Char('y') => {
                            let res = match &action {
                                SaveAction::Plugins => app.save().map(|_| ()),
                                SaveAction::Theme(name) => {
                                    let name = name.clone();
                                    app.save_theme(&name)
                                }
                            };
                            match res {
                                Ok(_) => {
                                    app.refresh_filtered();
                                    app.modal = Modal::None;
                                }
                                Err(e) => {
                                    app.message = Some(e);
                                    app.modal = Modal::None;
                                }
                            }
                        }
                        KeyCode::Down | KeyCode::Char('j') => {
                            *scroll = (*scroll + 1).min(max);
                        }
                        KeyCode::Up | KeyCode::Char('k') => {
                            *scroll = scroll.saturating_sub(1);
                        }
                        KeyCode::PageDown => {
                            *scroll = (*scroll + 15).min(max);
                        }
                        KeyCode::PageUp => {
                            *scroll = scroll.saturating_sub(15);
                        }
                        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                            app.should_quit = true;
                        }
                        _ => {}
                    }
                }
                Modal::None => {
                    if app.searching {
                        match key.code {
                            KeyCode::Esc => {
                                app.searching = false;
                                app.search.clear();
                                app.refresh_filtered();
                            }
                            KeyCode::Enter => app.searching = false,
                            KeyCode::Backspace => {
                                app.search.pop();
                                app.refresh_filtered();
                            }
                            KeyCode::Char(c) => {
                                app.search.push(c);
                                app.refresh_filtered();
                            }
                            _ => {}
                        }
                        continue;
                    }
                    if key.code == KeyCode::Char('T') {
                        app.view = match app.view {
                            View::Plugins => View::Themes,
                            View::Themes => View::Plugins,
                        };
                        continue;
                    }
                    if app.view == View::Themes {
                        match key.code {
                            KeyCode::Char('q') => {
                                app.should_quit = true;
                            }
                            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                                app.should_quit = true;
                            }
                            KeyCode::Char('j') | KeyCode::Down => {
                                if app.theme_selected + 1 < app.themes.len() {
                                    app.theme_selected += 1;
                                }
                            }
                            KeyCode::Char('k') | KeyCode::Up => {
                                app.theme_selected = app.theme_selected.saturating_sub(1);
                            }
                            KeyCode::Char('i') => {
                                if let Some(t) = app.themes.get(app.theme_selected) {
                                    let name = t.name.clone();
                                    let res = theme::try_on(&name);
                                    terminal.clear()?;
                                    if let Err(e) = res {
                                        app.message = Some(format!("试穿失败: {}", e));
                                    }
                                }
                            }
                            KeyCode::Char('p') => {
                                if let Some(t) = app.themes.get(app.theme_selected) {
                                    let t = t.clone();
                                    let res = theme::flash_preview(&t);
                                    terminal.clear()?;
                                    if let Err(e) = res {
                                        app.message = Some(format!("预览失败: {}", e));
                                    }
                                }
                            }
                            KeyCode::Enter => {
                                if let Some(t) = app.themes.get(app.theme_selected) {
                                    let name = t.name.clone();
                                    match app.prepare_theme_save(&name) {
                                        Ok(lines) => {
                                            app.modal = Modal::Diff {
                                                lines,
                                                scroll: 0,
                                                action: SaveAction::Theme(name),
                                            }
                                        }
                                        Err(e) => app.message = Some(e),
                                    }
                                }
                            }
                            KeyCode::Char('?') => app.modal = Modal::Help,
                            _ => {}
                        }
                        continue;
                    }
                    match key.code {
                        KeyCode::Char('q') => {
                            if app.dirty_count() > 0 {
                                app.modal = Modal::QuitConfirm;
                            } else {
                                app.should_quit = true;
                            }
                        }
                        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                            app.should_quit = true;
                        }
                        KeyCode::Char('j') | KeyCode::Down => {
                            if app.selected + 1 < app.filtered.len() {
                                app.selected += 1;
                                app.detail_scroll = 0;
                            }
                        }
                        KeyCode::Char('k') | KeyCode::Up => {
                            app.selected = app.selected.saturating_sub(1);
                            app.detail_scroll = 0;
                        }
                        KeyCode::Char('g') | KeyCode::Home => {
                            app.selected = 0;
                            app.detail_scroll = 0;
                        }
                        KeyCode::Char('G') | KeyCode::End => {
                            app.selected = app.filtered.len().saturating_sub(1);
                            app.detail_scroll = 0;
                        }
                        KeyCode::Char(' ') | KeyCode::Enter => app.toggle_current(),
                        KeyCode::Tab => {
                            app.filter = app.filter.next();
                            app.selected = 0;
                            app.refresh_filtered();
                        }
                        KeyCode::BackTab => {
                            app.filter = app.filter.prev();
                            app.selected = 0;
                            app.refresh_filtered();
                        }
                        KeyCode::Char('c') => {
                            app.cycle_category(Direction_::Next);
                        }
                        KeyCode::Char('C') => {
                            app.cycle_category(Direction_::Prev);
                        }
                        KeyCode::Char('/') => {
                            app.searching = true;
                        }
                        KeyCode::Char('s') => match app.prepare_save() {
                            Ok(Some(lines)) => {
                                app.modal = Modal::Diff {
                                    lines,
                                    scroll: 0,
                                    action: SaveAction::Plugins,
                                }
                            }
                            Ok(None) => {
                                app.message = Some("没有待保存的更改".to_string());
                            }
                            Err(e) => app.message = Some(e),
                        },
                        KeyCode::Char('r') => app.open_readme(),
                        KeyCode::Char('b') => {
                            let items = zshrc::list_backups(&app.zshrc_path);
                            if items.is_empty() {
                                app.message = Some("还没有任何备份".to_string());
                            } else {
                                app.modal = Modal::Backups { items, selected: 0 };
                            }
                        }
                        KeyCode::PageDown => app.detail_scroll += 10,
                        KeyCode::PageUp => app.detail_scroll = app.detail_scroll.saturating_sub(10),
                        KeyCode::Char('?') => app.modal = Modal::Help,
                        _ => {}
                    }
                }
            }
        }
        if app.should_quit {
            return Ok(());
        }
    }
}

// ---------- 渲染 ----------

fn draw(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // 标题栏
            Constraint::Length(1), // Tab 行
            Constraint::Min(5),    // 主体
            Constraint::Length(1), // 底部帮助
        ])
        .split(f.area());

    draw_header(f, app, chunks[0]);
    draw_tabs(f, app, chunks[1]);
    match app.view {
        View::Plugins => draw_body(f, app, chunks[2]),
        View::Themes => draw_themes_body(f, app, chunks[2]),
    }
    draw_footer(f, app, chunks[3]);

    match &app.modal {
        Modal::Help => draw_help_modal(f),
        Modal::QuitConfirm => draw_quit_confirm(f, app),
        Modal::Diff {
            lines,
            scroll,
            action,
        } => draw_diff_modal(f, lines, *scroll, action),
        Modal::Readme {
            title,
            lines,
            scroll,
        } => draw_readme_modal(f, title, lines, *scroll),
        Modal::Backups { items, selected } => draw_backups_modal(f, items, *selected),
        Modal::RestoreConfirm { backup } => draw_restore_confirm(f, backup),
        Modal::None => {}
    }
}

fn draw_header(f: &mut Frame, app: &App, area: Rect) {
    let (en, total) = app.total_counts();
    let dirty = app.dirty_count();
    let view_tag = match app.view {
        View::Plugins => "插件",
        View::Themes => "主题",
    };
    let title = format!(" omz-pm — Oh My Zsh 插件管理器 · {} ", view_tag);
    let mut spans = vec![
        Span::styled(
            title,
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" 已启用 "),
        Span::styled(
            en.to_string(),
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(format!(" / 共 {}", total)),
    ];
    match app.view {
        View::Plugins => {
            if let Some(c) = app.current_category() {
                spans.push(Span::raw("   分类:"));
                spans.push(Span::styled(
                    c.clone(),
                    Style::default()
                        .fg(Color::Magenta)
                        .add_modifier(Modifier::BOLD),
                ));
            }
        }
        View::Themes => {
            let cur = app
                .current_theme
                .clone()
                .filter(|s| !s.is_empty())
                .unwrap_or_else(|| "(默认)".to_string());
            spans.push(Span::raw("   当前主题:"));
            spans.push(Span::styled(
                cur,
                Style::default()
                    .fg(Color::Magenta)
                    .add_modifier(Modifier::BOLD),
            ));
        }
    }
    if dirty > 0 {
        spans.push(Span::styled(
            format!("   ● {} 项待保存", dirty),
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ));
    }
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);
    f.render_widget(block, area);
    let inner = Rect {
        x: area.x + 2,
        y: area.y + 1,
        width: area.width.saturating_sub(4),
        height: 1,
    };
    f.render_widget(Line::from(spans), inner);
}

fn draw_tabs(f: &mut Frame, app: &App, area: Rect) {
    let titles = ["全部", "已启用", "未启用"]
        .iter()
        .map(|t| Line::from(Span::raw(*t)))
        .collect::<Vec<_>>();
    let selected = match app.filter {
        Filter::All => 0,
        Filter::Enabled => 1,
        Filter::Disabled => 2,
    };
    let tabs = Tabs::new(titles).select(selected).highlight_style(
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    );
    let right_txt = if app.searching {
        Line::from(vec![
            Span::styled(" 搜索:", Style::default().fg(Color::Yellow)),
            Span::raw(app.search.clone() + "▏"),
        ])
    } else if app.search.is_empty() {
        Line::from("")
    } else {
        Line::from(vec![
            Span::raw(" 搜索:"),
            Span::styled(app.search.clone(), Style::default().fg(Color::Cyan)),
        ])
    };
    let left_w = area.width / 2;
    let left = Rect {
        width: left_w,
        ..area
    };
    let right = Rect {
        x: area.x + left_w,
        width: area.width - left_w,
        ..area
    };
    f.render_widget(tabs, left);
    f.render_widget(right_txt.alignment(Alignment::Right), right);
}

fn draw_body(f: &mut Frame, app: &App, area: Rect) {
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(42), Constraint::Percentage(58)])
        .split(area);

    // ---- 左侧列表:全部条目交给 ListState 自动滚动 ----
    let list_title = match app.current_category() {
        Some(c) => format!(" 插件({} · {}) ", app.filtered.len(), c),
        None => format!(" 插件({}) ", app.filtered.len()),
    };
    let list_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title(Span::styled(list_title, Style::default().fg(Color::Blue)));
    f.render_widget(list_block, cols[0]);
    let list_area = Rect {
        x: cols[0].x + 1,
        y: cols[0].y + 1,
        width: cols[0].width.saturating_sub(2),
        height: cols[0].height.saturating_sub(2),
    };

    let items: Vec<ListItem> = app
        .filtered
        .iter()
        .filter_map(|pi| {
            let p = app.plugins.get(*pi)?;
            Some(render_list_row(p, app, list_area.width as usize))
        })
        .collect();

    let mut ls = ListState::default();
    if !app.filtered.is_empty() {
        ls.select(Some(app.selected));
    }
    let list = List::new(items).highlight_style(Style::default().bg(Color::DarkGray));
    f.render_stateful_widget(list, list_area, &mut ls);

    // ---- 右侧详情 ----
    if let Some(p) = app.current() {
        let details = build_detail_lines(app, p, cols[1].width.saturating_sub(2) as usize);
        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(Span::styled(
                format!(" {} ", p.name),
                Style::default().fg(Color::Blue),
            ));
        f.render_widget(block, cols[1]);
        let inner = Rect {
            x: cols[1].x + 1,
            y: cols[1].y + 1,
            width: cols[1].width.saturating_sub(2),
            height: cols[1].height.saturating_sub(2),
        };
        let mut para = Paragraph::new(details);
        if app.detail_scroll > 0 {
            para = para.scroll((app.detail_scroll as u16, 0));
        }
        f.render_widget(para, inner);
    } else {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(" 详情 ");
        f.render_widget(block, cols[1]);
    }
}

fn render_list_row(p: &Plugin, app: &App, width: usize) -> ListItem<'static> {
    let on = app.is_effective(&p.name);
    let dot = if on { "●" } else { "○" };
    let dot_style = if on {
        Style::default().fg(Color::Green)
    } else {
        Style::default().fg(Color::DarkGray)
    };
    let summary = app
        .catalog
        .get(&p.name)
        .map(|e| e.summary.as_str())
        .or_else(|| app.excerpts.get(&p.name).map(|s| s.as_str()))
        .unwrap_or("");
    let name_col = 24.min(width.saturating_sub(4));
    let src_tag = if p.source == crate::plugin::Source::Custom {
        "[自]"
    } else {
        ""
    };
    let rest_w = width.saturating_sub(name_col + src_tag.width() + 4);
    let mut spans = vec![
        Span::styled(dot, dot_style),
        Span::raw(" "),
        Span::styled(
            textwrap::pad_to(&p.name, name_col),
            if on {
                Style::default().add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            },
        ),
        Span::styled(src_tag, Style::default().fg(Color::Magenta)),
        Span::raw(" "),
        Span::styled(
            textwrap::truncate_to(summary, rest_w),
            Style::default().fg(Color::Gray),
        ),
    ];
    if app.pending.contains_key(&p.name) {
        spans.push(Span::styled(
            " *",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ));
    }
    ListItem::new(Line::from(spans))
}

/// 组装右侧详情的全部行(带小节标题)。
fn build_detail_lines<'a>(app: &'a App, p: &'a Plugin, w: usize) -> Vec<Line<'a>> {
    let on = app.is_effective(&p.name);
    let status = if on {
        Span::styled(
            "● 已启用",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )
    } else {
        Span::styled("○ 未启用", Style::default().fg(Color::DarkGray))
    };
    let mut lines: Vec<Line> = Vec::new();
    let mut head = vec![
        status,
        Span::raw("   "),
        Span::styled(p.source.label(), Style::default().fg(Color::Magenta)),
    ];
    if let Some(cat) = app.category_of(&p.name) {
        head.push(Span::raw("   "));
        head.push(Span::styled(
            format!("「{}」", cat),
            Style::default().fg(Color::Magenta),
        ));
    }
    lines.push(Line::from(head));
    lines.push(Line::from(""));

    match app.catalog.get(&p.name) {
        Some(entry) => {
            section(&mut lines, w, "功能说明");
            push_wrapped(&mut lines, &entry.summary, w, None);
            if !entry.detail.is_empty() {
                lines.push(Line::from(""));
                push_wrapped(&mut lines, &entry.detail, w, None);
            }
            if !entry.usage.is_empty() {
                lines.push(Line::from(""));
                section(&mut lines, w, "实战用法");
                for para in entry.usage.split('\n') {
                    if para.is_empty() {
                        lines.push(Line::from(""));
                    } else {
                        push_wrapped(&mut lines, para, w, Some(Style::default()));
                    }
                }
            }
            if !entry.aliases.is_empty() {
                lines.push(Line::from(""));
                section(&mut lines, w, "常用别名");
                for (name, note) in &entry.aliases {
                    lines.push(Line::from(vec![
                        Span::styled(
                            format!("{:<10}", name),
                            Style::default()
                                .fg(Color::Yellow)
                                .add_modifier(Modifier::BOLD),
                        ),
                        Span::raw(" "),
                        Span::raw(note.clone()),
                    ]));
                }
            }
        }
        None => {
            lines.push(Line::from(""));
            lines.push(
                Span::styled(
                    "⚠ 无中文词典条目(可能是自定义插件)",
                    Style::default().fg(Color::Yellow),
                )
                .into(),
            );
            lines.push(Line::from(""));
            if let Some(ex) = app.excerpts.get(&p.name) {
                lines.push(
                    Span::styled("── English ──", Style::default().fg(Color::DarkGray)).into(),
                );
                push_wrapped(&mut lines, ex, w, None);
            }
        }
    }

    // 源码别名索引(所有插件都可提取,放在精选注解之后)
    let src_aliases = app.aliases_of(&p.name);
    if !src_aliases.is_empty() {
        lines.push(Line::from(""));
        section(
            &mut lines,
            w,
            &format!("别名索引(源码提取,共 {} 条)", src_aliases.len()),
        );
        let curated = app
            .catalog
            .get(&p.name)
            .map(|e| e.aliases.keys().cloned().collect::<Vec<_>>())
            .unwrap_or_default();
        let shown = src_aliases
            .iter()
            .filter(|d| !curated.contains(&d.name))
            .take(30);
        for d in shown {
            lines.push(Line::from(vec![
                Span::styled(format!("{:<10}", d.name), Style::default().fg(Color::Cyan)),
                Span::styled(
                    format!("= {}", truncate_cmd(&d.command, w.saturating_sub(14))),
                    Style::default().fg(Color::Gray),
                ),
            ]));
        }
        if src_aliases.len() > curated.len() + 30 {
            lines.push(
                Span::styled(
                    format!(
                        "… 其余 {} 条见: omz-pm aliases {}",
                        src_aliases.len() - 30 - curated.len(),
                        p.name
                    ),
                    Style::default().fg(Color::DarkGray),
                )
                .into(),
            );
        }
    }

    lines
}

fn section(lines: &mut Vec<Line<'static>>, w: usize, title: &str) {
    let bar_w = w.saturating_sub(title.width() + 4).min(60);
    lines.push(Line::from(vec![
        Span::styled(
            format!(" {}", title),
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            format!(" {}", "─".repeat(bar_w)),
            Style::default().fg(Color::DarkGray),
        ),
    ]));
}

fn push_wrapped(lines: &mut Vec<Line<'static>>, text: &str, w: usize, style: Option<Style>) {
    for l in textwrap::wrap(text, w) {
        let line = Line::from(l);
        lines.push(match style {
            Some(s) => line.style(s),
            None => line,
        });
    }
}

fn truncate_cmd(cmd: &str, width: usize) -> String {
    textwrap::truncate_to(cmd, width.max(8))
}

/// 主题视图:左列主题清单,右侧当前选中主题的信息与预览。
fn draw_themes_body(f: &mut Frame, app: &App, area: Rect) {
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(38), Constraint::Percentage(62)])
        .split(area);

    let list_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title(Span::styled(
            format!(" 主题({}) ", app.themes.len()),
            Style::default().fg(Color::Blue),
        ));
    f.render_widget(list_block, cols[0]);
    let list_area = Rect {
        x: cols[0].x + 1,
        y: cols[0].y + 1,
        width: cols[0].width.saturating_sub(2),
        height: cols[0].height.saturating_sub(2),
    };
    let items: Vec<ListItem> = app
        .themes
        .iter()
        .map(|t| {
            let is_cur = app.current_theme.as_deref() == Some(t.name.as_str());
            let dot = if is_cur { "●" } else { "○" };
            let dot_style = if is_cur {
                Style::default().fg(Color::Green)
            } else {
                Style::default().fg(Color::DarkGray)
            };
            let src = if t.source == crate::plugin::Source::Custom {
                "[自]"
            } else {
                ""
            };
            ListItem::new(Line::from(vec![
                Span::styled(dot, dot_style),
                Span::raw(" "),
                Span::raw(textwrap::truncate_to(
                    &format!("{}{}", t.name, if src.is_empty() { "" } else { " " }),
                    list_area.width as usize,
                )),
                Span::styled(src, Style::default().fg(Color::Magenta)),
            ]))
        })
        .collect();
    let mut ls = ListState::default();
    if !app.themes.is_empty() {
        ls.select(Some(app.theme_selected));
    }
    f.render_stateful_widget(
        List::new(items).highlight_style(Style::default().bg(Color::DarkGray)),
        list_area,
        &mut ls,
    );

    // 右侧详情
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title(Span::styled(" 主题详情 ", Style::default().fg(Color::Blue)));
    f.render_widget(block, cols[1]);
    let inner = Rect {
        x: cols[1].x + 1,
        y: cols[1].y + 1,
        width: cols[1].width.saturating_sub(2),
        height: cols[1].height.saturating_sub(2),
    };
    let mut lines: Vec<Line> = Vec::new();
    if let Some(t) = app.themes.get(app.theme_selected) {
        let is_cur = app.current_theme.as_deref() == Some(t.name.as_str());
        let status = if is_cur {
            Span::styled(
                "● 当前主题",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            )
        } else {
            Span::styled("○ 未启用", Style::default().fg(Color::DarkGray))
        };
        lines.push(Line::from(vec![
            status,
            Span::raw("   "),
            Span::styled(
                t.name.clone(),
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::raw("  "),
            Span::styled(t.source.label(), Style::default().fg(Color::Magenta)),
        ]));
        lines.push(Line::from(""));
        lines.push(
            Span::styled(
                "按 i 试穿(进入临时 zsh 实际体验) · p 彩色快览 · Enter 启用",
                Style::default().fg(Color::DarkGray),
            )
            .into(),
        );
        lines.push(Line::from(""));
        section(&mut lines, inner.width as usize, "提示符模板预览");
        match theme::preview_ansi(t) {
            Some(ansi) => {
                let plain = theme::strip_ansi(&ansi);
                for l in textwrap::wrap(&plain, inner.width as usize) {
                    lines.push(Line::from(l));
                }
                lines.push(Line::from(""));
                lines.push(
                    Span::styled(
                        "(以上为纯文本;按 p 查看带颜色的真实效果)",
                        Style::default().fg(Color::DarkGray),
                    )
                    .into(),
                );
            }
            None => {
                lines.push(
                    Span::styled(
                        "自动渲染失败(主题依赖特殊环境),可用 i 直接试穿。",
                        Style::default().fg(Color::Yellow),
                    )
                    .into(),
                );
            }
        }
    } else {
        lines.push(Line::from("没有找到任何主题"));
    }
    f.render_widget(Paragraph::new(lines), inner);
}

fn draw_footer(f: &mut Frame, app: &App, area: Rect) {
    let msg_spans: Vec<Span> = if let Some(m) = &app.message {
        vec![Span::styled(m.clone(), Style::default().fg(Color::Green))]
    } else {
        match app.view {
            View::Themes => vec![
                Span::styled("↑↓/jk 选择", Style::default().fg(Color::DarkGray)),
                Span::raw("  "),
                Span::styled("i 试穿", Style::default().fg(Color::DarkGray)),
                Span::raw("  "),
                Span::styled("p 彩色快览", Style::default().fg(Color::DarkGray)),
                Span::raw("  "),
                Span::styled(
                    "Enter 启用(diff 确认)",
                    Style::default().fg(Color::DarkGray),
                ),
                Span::raw("  "),
                Span::styled("T 插件视图", Style::default().fg(Color::DarkGray)),
                Span::raw("  "),
                Span::styled("q 退出", Style::default().fg(Color::DarkGray)),
            ],
            View::Plugins => vec![
                Span::styled("↑↓/jk 移动", Style::default().fg(Color::DarkGray)),
                Span::raw("  "),
                Span::styled("空格 切换", Style::default().fg(Color::DarkGray)),
                Span::raw("  "),
                Span::styled("Tab 筛选", Style::default().fg(Color::DarkGray)),
                Span::raw("  "),
                Span::styled("c 分类", Style::default().fg(Color::DarkGray)),
                Span::raw("  "),
                Span::styled("/ 搜索", Style::default().fg(Color::DarkGray)),
                Span::raw("  "),
                Span::styled("r README", Style::default().fg(Color::DarkGray)),
                Span::raw("  "),
                Span::styled("s 保存(先预览 diff)", Style::default().fg(Color::DarkGray)),
                Span::raw("  "),
                Span::styled("b 备份", Style::default().fg(Color::DarkGray)),
                Span::raw("  "),
                Span::styled("T 主题", Style::default().fg(Color::DarkGray)),
                Span::raw("  "),
                Span::styled("? 帮助", Style::default().fg(Color::DarkGray)),
                Span::raw("  "),
                Span::styled("q 退出", Style::default().fg(Color::DarkGray)),
            ],
        }
    };
    f.render_widget(Line::from(msg_spans), area);
}

fn centered_rect(f: &Frame, pct_x: u16, pct_y: u16, max_w: u16, max_h: u16) -> Rect {
    let a = f.area();
    let w = (a.width * pct_x / 100).min(max_w).max(10);
    let h = (a.height * pct_y / 100).min(max_h).max(6);
    let x = a.x + (a.width.saturating_sub(w)) / 2;
    let y = a.y + (a.height.saturating_sub(h)) / 2;
    Rect {
        x,
        y,
        width: w,
        height: h,
    }
}

fn draw_help_modal(f: &mut Frame) {
    let area = centered_rect(f, 74, 76, 70, 24);
    f.render_widget(Clear, area);
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .title(" 快捷键帮助 ");
    let text = vec![
        Line::from(""),
        key_line("↑↓ / j k", "移动选择"),
        key_line("空格 / Enter", "切换 启用 ↔ 禁用"),
        key_line("Tab / Shift+Tab", "筛选:全部 / 已启用 / 未启用"),
        key_line("c", "按分类循环筛选(目录跳转/版本控制…)"),
        key_line("/", "搜索(名称、中文说明、分类)"),
        key_line("PgUp / PgDn", "滚动右侧详情"),
        key_line("s", "保存前先显示 diff 预览"),
        key_line("q", "退出(有未保存更改时会确认)"),
        Line::from(""),
        Line::from(Span::styled(
            "保存流程:s 打开 diff → Enter 确认写入(自动备份)/ Esc 取消",
            Style::default().fg(Color::DarkGray),
        )),
        Line::from(Span::styled(
            "ESC 关闭此窗口",
            Style::default().fg(Color::DarkGray),
        )),
    ];
    f.render_widget(Paragraph::new(text).block(block), area);
}

fn key_line(k: &str, d: &str) -> Line<'static> {
    Line::from(vec![
        Span::styled(
            textwrap::pad_to(&format!(" {:18}", k), 20),
            Style::default().fg(Color::Cyan),
        ),
        Span::raw(d.to_string()),
    ])
}

fn draw_quit_confirm(f: &mut Frame, app: &App) {
    let n = app.dirty_count();
    let area = centered_rect(f, 60, 30, 56, 8);
    f.render_widget(Clear, area);
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .title(" 有未保存的更改 ")
        .title_style(Style::default().fg(Color::Yellow));
    let text = vec![
        Line::from(format!("  你还有 {} 项更改没有保存。", n)),
        Line::from(""),
        Line::from(vec![
            Span::raw("  "),
            Span::styled(
                "[s] 保存并退出",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("   "),
            Span::styled("[q] 放弃更改退出", Style::default().fg(Color::Red)),
            Span::raw("   "),
            Span::styled("[Esc] 返回", Style::default().fg(Color::DarkGray)),
        ]),
    ];
    f.render_widget(Paragraph::new(text).block(block), area);
}

fn draw_diff_modal(f: &mut Frame, lines: &[Line<'static>], scroll: u16, action: &SaveAction) {
    let (title, confirm_label) = match action {
        SaveAction::Plugins => (" 保存预览(diff) ", "[Enter/y] 确认保存"),
        SaveAction::Theme(name) => (
            &format!(" 启用主题「{}」(diff) ", name)[..],
            "[Enter/y] 确认启用",
        ),
    };
    let area = centered_rect(f, 88, 88, 120, 40);
    f.render_widget(Clear, area);
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .title(Span::styled(
            title,
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ));
    f.render_widget(block, area);
    let inner = Rect {
        x: area.x + 1,
        y: area.y + 1,
        width: area.width.saturating_sub(2),
        height: area.height.saturating_sub(2),
    };
    let hint_h = 1u16;
    let list_h = inner.height.saturating_sub(hint_h);
    let list_area = Rect {
        height: list_h,
        ..inner
    };
    let hint_area = Rect {
        y: inner.y + list_h,
        height: hint_h,
        ..inner
    };
    let para = Paragraph::new(lines.to_vec()).scroll((scroll, 0));
    f.render_widget(para, list_area);
    f.render_widget(
        Line::from(vec![
            Span::styled(
                confirm_label,
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("   "),
            Span::styled("[Esc/n] 取消", Style::default().fg(Color::Red)),
            Span::raw("   "),
            Span::styled("↑↓/PgUp/PgDn 滚动", Style::default().fg(Color::DarkGray)),
        ])
        .alignment(Alignment::Center),
        hint_area,
    );
}

fn draw_readme_modal(f: &mut Frame, title: &str, lines: &[Line<'static>], scroll: u16) {
    let area = centered_rect(f, 90, 90, 130, 45);
    f.render_widget(Clear, area);
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .title(Span::styled(
            title.to_string(),
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ));
    f.render_widget(block, area);
    let inner = Rect {
        x: area.x + 1,
        y: area.y + 1,
        width: area.width.saturating_sub(2),
        height: area.height.saturating_sub(2),
    };
    let hint_h = 1u16;
    let list_area = Rect {
        height: inner.height.saturating_sub(hint_h),
        ..inner
    };
    let hint_area = Rect {
        y: inner.y + list_area.height,
        height: hint_h,
        ..inner
    };
    f.render_widget(
        Paragraph::new(lines.to_vec()).scroll((scroll, 0)),
        list_area,
    );
    f.render_widget(
        Line::from(vec![
            Span::styled("↑↓/PgUp/PgDn 滚动", Style::default().fg(Color::DarkGray)),
            Span::raw("   "),
            Span::styled("[Esc/r] 返回", Style::default().fg(Color::Green)),
        ])
        .alignment(Alignment::Right),
        hint_area,
    );
}

fn draw_backups_modal(f: &mut Frame, items: &[std::path::PathBuf], selected: usize) {
    let area = centered_rect(f, 80, 70, 100, 22);
    f.render_widget(Clear, area);
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .title(Span::styled(
            format!(" 备份管理(共 {} 个) ", items.len()),
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ));
    f.render_widget(block, area);
    let inner = Rect {
        x: area.x + 1,
        y: area.y + 1,
        width: area.width.saturating_sub(2),
        height: area.height.saturating_sub(2),
    };
    let hint_h = 1u16;
    let list_area = Rect {
        height: inner.height.saturating_sub(hint_h),
        ..inner
    };
    let hint_area = Rect {
        y: inner.y + list_area.height,
        height: hint_h,
        ..inner
    };
    let zshrc_disp = crate::zshrc::default_zshrc_path().display().to_string();
    let mut lines: Vec<Line> = Vec::new();
    for (i, p) in items.iter().enumerate() {
        let name = p
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("?")
            .to_string();
        // 文件名形如 .zshrc.omz-pm.bak.20260827-133407,提取时间戳
        let ts = name.rsplit(".omz-pm.bak.").next().unwrap_or("").to_string();
        let meta = std::fs::metadata(p).ok();
        let size = meta.as_ref().map(|m| m.len()).unwrap_or(0);
        let time_hint = parse_backup_ts(&ts);
        let cur = if i == selected { "▶ " } else { "  " };
        lines.push(Line::from(vec![
            Span::raw(cur),
            Span::styled(
                textwrap::pad_to(&ts, 20),
                if i == selected {
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                },
            ),
            Span::styled(
                format!("  {:>6} B", size),
                Style::default().fg(Color::DarkGray),
            ),
            Span::styled(
                format!("  {}", time_hint),
                Style::default().fg(Color::DarkGray),
            ),
        ]));
    }
    f.render_widget(Paragraph::new(lines), list_area);
    f.render_widget(
        Line::from(vec![
            Span::styled("[Enter] 恢复此备份", Style::default().fg(Color::Green)),
            Span::raw("   "),
            Span::styled("[Esc/b] 关闭", Style::default().fg(Color::Red)),
            Span::raw("   "),
            Span::styled(
                format!("(恢复前会先把当前 {} 另存快照)", zshrc_disp),
                Style::default().fg(Color::DarkGray),
            ),
        ])
        .alignment(Alignment::Center),
        hint_area,
    );
}

/// `20260827-133407` → `2026-08-27 13:34:07`
fn parse_backup_ts(ts: &str) -> String {
    if ts.len() == 15 {
        format!(
            "{}-{}-{} {}:{}:{}",
            &ts[0..4],
            &ts[4..6],
            &ts[6..8],
            &ts[9..11],
            &ts[11..13],
            &ts[13..15]
        )
    } else {
        ts.to_string()
    }
}

fn draw_restore_confirm(f: &mut Frame, backup: &std::path::Path) {
    let name = backup
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("?")
        .to_string();
    let area = centered_rect(f, 62, 26, 60, 7);
    f.render_widget(Clear, area);
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .title(" 确认恢复 ")
        .title_style(Style::default().fg(Color::Yellow));
    let text = vec![
        Line::from("  将用以下备份覆盖当前 zshrc:".to_string()),
        Line::from(Span::styled(
            format!("  {}", name),
            Style::default().fg(Color::Cyan),
        )),
        Line::from(""),
        Line::from(vec![
            Span::raw("  "),
            Span::styled(
                "[Enter/y] 恢复",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("   "),
            Span::styled("[Esc/n] 取消", Style::default().fg(Color::Red)),
        ]),
    ];
    f.render_widget(Paragraph::new(text).block(block), area);
}

/// 挂起 TUI(供试穿/彩色预览直接使用真实终端)。
pub fn suspend_tui() -> io::Result<()> {
    restore_terminal()?;
    Ok(())
}

/// 恢复 TUI。
pub fn resume_tui() -> io::Result<()> {
    init_terminal()?;
    Ok(())
}
