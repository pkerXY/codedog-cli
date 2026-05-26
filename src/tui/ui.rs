//! TUI UI 渲染

use super::app::{App, Module};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

pub fn render(f: &mut Frame, app: &App) {
    let area = f.size();

    // 创建主布局
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(1), // 标题栏
            Constraint::Min(10),   // 主内容区
            Constraint::Length(1), // 状态栏
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

    // 右侧内容区
    match app.selected_module {
        None => render_welcome(f, main_chunks[1]),
        Some(Module::Format) => render_format_panel(f, app, main_chunks[1]),
        Some(Module::Encode) => render_encode_panel(f, app, main_chunks[1]),
        Some(Module::Time) => render_time_panel(f, app, main_chunks[1]),
        Some(Module::Hash) => render_hash_panel(f, app, main_chunks[1]),
    }

    // 状态栏
    render_status(f, app, chunks[2]);
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
            let is_selected = i == app.menu_index;
            let style = if is_selected {
                Style::default().fg(Color::Yellow).bold()
            } else {
                Style::default()
            };
            let prefix = if is_selected { "> " } else { "  " };
            let text = format!("{}{}", prefix, m.name());
            ListItem::new(text).style(style)
        })
        .collect();

    let menu = List::new(items).block(
        Block::default()
            .title(" 功能菜单 ")
            .borders(Borders::ALL)
            .border_style(if app.selected_module.is_none() {
                Style::default().fg(Color::Green)
            } else {
                Style::default()
            }),
    );

    f.render_widget(menu, area);
}

fn render_welcome(f: &mut Frame, area: Rect) {
    let welcome_lines = vec![
        Line::from(""),
        Line::from("欢迎使用 CodeDog 程序员工具箱"),
        Line::from(""),
        Line::from("使用 ↑↓ 选择功能模块"),
        Line::from("按 Enter 进入模块"),
        Line::from("按 q 退出程序"),
        Line::from(""),
        Line::from("可用功能:"),
        Line::from("  • 格式化 - JSON/YAML/TOML 格式转换"),
        Line::from("  • 编码转换 - Base64/URL 编解码"),
        Line::from("  • 时间工具 - 时间戳转换"),
        Line::from("  • 哈希计算 - MD5/SHA 哈希"),
    ];

    let welcome = Paragraph::new(welcome_lines).block(
        Block::default()
            .title(" 欢迎 ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan)),
    );

    f.render_widget(welcome, area);
}

fn render_format_panel(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(45), // 输入区
            Constraint::Length(3),      // 操作按钮
            Constraint::Percentage(45), // 输出区
        ])
        .split(area);

    // 输入区
    let input_style = if app.panel_focus == 0 {
        Style::default().fg(Color::Green)
    } else {
        Style::default()
    };
    let input = Paragraph::new(app.input.as_str()).block(
        Block::default()
            .title(format!(
                " 输入 {} ",
                if app.editing { "[编辑中]" } else { "" }
            ))
            .borders(Borders::ALL)
            .border_style(input_style),
    );
    f.render_widget(input, chunks[0]);

    // 操作按钮区
    let buttons = Paragraph::new(" [Enter]格式化 [e]编辑 [p]粘贴 [Tab]切换焦点 ")
        .block(Block::default().borders(Borders::ALL))
        .style(if app.panel_focus == 1 {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default().fg(Color::DarkGray)
        });
    f.render_widget(buttons, chunks[1]);

    // 输出区
    let output_style = if app.panel_focus == 2 {
        Style::default().fg(Color::Green)
    } else {
        Style::default()
    };
    let output = Paragraph::new(app.output.as_str()).block(
        Block::default()
            .title(" 输出 ")
            .borders(Borders::ALL)
            .border_style(output_style),
    );
    f.render_widget(output, chunks[2]);
}

fn render_encode_panel(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(45), // 输入区
            Constraint::Length(3),      // 操作按钮
            Constraint::Percentage(45), // 输出区
        ])
        .split(area);

    // 输入区
    let input_style = if app.panel_focus == 0 {
        Style::default().fg(Color::Green)
    } else {
        Style::default()
    };
    let input = Paragraph::new(app.input.as_str()).block(
        Block::default()
            .title(format!(
                " 输入 {} ",
                if app.editing { "[编辑中]" } else { "" }
            ))
            .borders(Borders::ALL)
            .border_style(input_style),
    );
    f.render_widget(input, chunks[0]);

    // 操作按钮区
    let buttons = Paragraph::new(" [Enter]Base64编码 [Tab]切换焦点 ")
        .block(Block::default().borders(Borders::ALL))
        .style(if app.panel_focus == 1 {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default().fg(Color::DarkGray)
        });
    f.render_widget(buttons, chunks[1]);

    // 输出区
    let output_style = if app.panel_focus == 2 {
        Style::default().fg(Color::Green)
    } else {
        Style::default()
    };
    let output = Paragraph::new(app.output.as_str()).block(
        Block::default()
            .title(" 输出 ")
            .borders(Borders::ALL)
            .border_style(output_style),
    );
    f.render_widget(output, chunks[2]);
}

fn render_time_panel(f: &mut Frame, _app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(50), // 当前时间
            Constraint::Percentage(50), // 时间戳区
        ])
        .split(area);

    // 当前时间显示
    let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC").to_string();
    let time_lines = vec![
        Line::from(""),
        Line::from(format!("  {}", now)),
        Line::from(""),
        Line::from("  按 [Enter] 刷新"),
    ];
    let time_display = Paragraph::new(time_lines).block(
        Block::default()
            .title(" 当前时间 ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan)),
    );
    f.render_widget(time_display, chunks[0]);

    // 时间戳区
    let timestamp = chrono::Utc::now().timestamp();
    let ts_lines = vec![
        Line::from(""),
        Line::from(format!("  秒: {}", timestamp)),
        Line::from(format!("  毫秒: {}000", timestamp)),
    ];
    let ts_display = Paragraph::new(ts_lines).block(
        Block::default()
            .title(" 时间戳 ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Yellow)),
    );
    f.render_widget(ts_display, chunks[1]);
}

fn render_hash_panel(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(45), // 输入区
            Constraint::Length(3),      // 操作按钮
            Constraint::Percentage(45), // 输出区
        ])
        .split(area);

    // 输入区
    let input_style = if app.panel_focus == 0 {
        Style::default().fg(Color::Green)
    } else {
        Style::default()
    };
    let input = Paragraph::new(app.input.as_str()).block(
        Block::default()
            .title(format!(
                " 输入 {} ",
                if app.editing { "[编辑中]" } else { "" }
            ))
            .borders(Borders::ALL)
            .border_style(input_style),
    );
    f.render_widget(input, chunks[0]);

    // 操作按钮区
    let buttons = Paragraph::new(" [Enter]SHA256 [Tab]切换焦点 ")
        .block(Block::default().borders(Borders::ALL))
        .style(if app.panel_focus == 1 {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default().fg(Color::DarkGray)
        });
    f.render_widget(buttons, chunks[1]);

    // 输出区
    let output_style = if app.panel_focus == 2 {
        Style::default().fg(Color::Green)
    } else {
        Style::default()
    };
    let output = Paragraph::new(app.output.as_str()).block(
        Block::default()
            .title(" 哈希结果 ")
            .borders(Borders::ALL)
            .border_style(output_style),
    );
    f.render_widget(output, chunks[2]);
}

fn render_status(f: &mut Frame, app: &App, area: Rect) {
    let status = match app.selected_module {
        None => " [↑↓]选择 [Enter]进入 [q]退出".to_string(),
        Some(_) => {
            format!(
                " [Esc]返回 [Tab]切换焦点 [Enter]执行 {}",
                if app.editing { " [编辑中]" } else { "" }
            )
        }
    };
    let status_bar = Paragraph::new(status).style(Style::default().fg(Color::DarkGray));
    f.render_widget(status_bar, area);
}
