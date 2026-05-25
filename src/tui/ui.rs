//! TUI UI 渲染

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Block, Borders, Paragraph, List, ListItem, Clear},
    Frame,
};
use super::app::{App, Focus, Module};

pub fn render(f: &mut Frame, app: &App) {
    let area = f.size();

    // 创建主布局
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(1),  // 标题栏
            Constraint::Min(10),    // 主内容区
            Constraint::Length(2),  // 状态栏
        ])
        .split(area);

    // 标题栏
    render_title(f, chunks[0]);

    // 主内容区
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(15), Constraint::Min(50)])
        .split(chunks[1]);

    // 菜单
    render_menu(f, app, main_chunks[0]);

    // 输入/输出区
    let io_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(main_chunks[1]);

    render_input(f, app, io_chunks[0]);
    render_output(f, app, io_chunks[1]);

    // 状态栏
    render_status(f, app, chunks[2]);

    // 帮助弹窗
    if app.show_help {
        render_help(f);
    }
}

fn render_title(f: &mut Frame, area: Rect) {
    let title = Paragraph::new(" CodeDog - 程序员工具箱 [q:退出]")
        .style(Style::default().fg(Color::Cyan).bold());
    f.render_widget(title, area);
}

fn render_menu(f: &mut Frame, app: &App, area: Rect) {
    let items: Vec<ListItem> = Module::all()
        .iter()
        .enumerate()
        .map(|(i, m)| {
            let is_selected = *m == app.current_module;
            let style = if is_selected {
                Style::default().fg(Color::Yellow).bold()
            } else {
                Style::default()
            };
            let prefix = if is_selected { "> " } else { "  " };
            let text = format!("{}. {}{}", i + 1, prefix, m.name());
            ListItem::new(text).style(style)
        })
        .collect();

    let menu = List::new(items)
        .block(
            Block::default()
                .title(" 功能菜单 ")
                .borders(Borders::ALL)
                .border_style(if app.focus == Focus::Menu {
                    Style::default().fg(Color::Green)
                } else {
                    Style::default()
                }),
        );

    f.render_widget(menu, area);
}

fn render_input(f: &mut Frame, app: &App, area: Rect) {
    let style = if app.focus == Focus::Input {
        Style::default().fg(Color::Green)
    } else {
        Style::default()
    };

    let input = Paragraph::new(app.input.as_str())
        .block(
            Block::default()
                .title(" 输入区域 ")
                .borders(Borders::ALL)
                .border_style(style),
        )
        .style(Style::default());

    f.render_widget(input, area);

    // 操作提示
    let hints = Paragraph::new(" [e]编辑 [p]粘贴 [c]清空 [f]文件")
        .style(Style::default().fg(Color::DarkGray));
    f.render_widget(hints, Rect::new(area.x, area.y + area.height, area.width, 1));
}

fn render_output(f: &mut Frame, app: &App, area: Rect) {
    let style = if app.focus == Focus::Output {
        Style::default().fg(Color::Green)
    } else {
        Style::default()
    };

    let output = Paragraph::new(app.output.as_str())
        .block(
            Block::default()
                .title(format!(" 输出区域 [{}] ", app.format))
                .borders(Borders::ALL)
                .border_style(style),
        );

    f.render_widget(output, area);

    // 操作提示
    let hints = Paragraph::new(" [y]复制 [s]保存 [w]切换格式")
        .style(Style::default().fg(Color::DarkGray));
    f.render_widget(hints, Rect::new(area.x, area.y + area.height, area.width, 1));
}

fn render_status(f: &mut Frame, app: &App, area: Rect) {
    let status = format!(
        " [1-4]切换模块 [Tab]切换面板 [Enter]执行 [?]帮助{}",
        if app.editing { " [编辑中]" } else { "" }
    );
    let status_bar = Paragraph::new(status)
        .style(Style::default().fg(Color::DarkGray));
    f.render_widget(status_bar, area);
}

fn render_help(f: &mut Frame) {
    let area = f.size();
    let popup_area = centered_rect(60, 50, area);
    f.render_widget(Clear, popup_area);

    let help_text = vec![
        "快捷键说明:",
        "",
        "  1/2/3/4  - 切换功能模块",
        "  Tab       - 切换面板焦点",
        "  Enter     - 执行当前操作",
        "  e         - 编辑输入内容",
        "  p         - 粘贴剪贴板",
        "  f         - 从文件加载",
        "  y         - 复制输出",
        "  s         - 保存输出",
        "  w         - 切换输出格式",
        "  q/Esc     - 退出/取消",
        "",
        "  按 ? 关闭帮助",
    ];

    let help: Vec<ListItem> = help_text
        .iter()
        .map(|&t| ListItem::new(Line::from(t)))
        .collect();

    let help_widget = List::new(help)
        .block(
            Block::default()
                .title(" 帮助 ")
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::Yellow)),
        );

    f.render_widget(help_widget, popup_area);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
