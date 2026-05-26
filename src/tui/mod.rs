//! TUI 界面模块

mod app;
mod ui;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;

/// 启动 TUI 界面
pub fn run() -> anyhow::Result<()> {
    // 设置终端
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // 创建应用
    let mut app = app::App::new();

    // 主循环
    let res = run_app(&mut terminal, &mut app);

    // 恢复终端
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    res
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut app::App,
) -> anyhow::Result<()> {
    loop {
        terminal.draw(|f| ui::render(f, app))?;

        if let Event::Key(key) = event::read()? {
            // 只处理按键按下事件，忽略释放事件
            if key.kind != KeyEventKind::Press {
                continue;
            }

            match app.selected_module {
                // 菜单模式
                None => match (key.modifiers, key.code) {
                    (KeyModifiers::CONTROL, KeyCode::Char('c')) | (_, KeyCode::Char('q')) => {
                        return Ok(());
                    }
                    (_, KeyCode::Up) => app.menu_up(),
                    (_, KeyCode::Down) => app.menu_down(),
                    (_, KeyCode::Enter) => app.select(),
                    _ => {}
                },
                // 模块模式
                Some(_) => match (key.modifiers, key.code) {
                    (KeyModifiers::CONTROL, KeyCode::Char('c')) | (_, KeyCode::Char('q')) => {
                        return Ok(());
                    }
                    (_, KeyCode::Esc) => app.back(),
                    (_, KeyCode::Tab) => app.toggle_panel_focus(),
                    (_, KeyCode::Enter) => {
                        if app.editing {
                            app.editing = false;
                        } else {
                            app.execute();
                        }
                    }
                    (_, KeyCode::Char('e')) => app.toggle_edit(),
                    (_, KeyCode::Char('p')) => app.paste(),
                    (_, KeyCode::Char('y')) => app.copy_output()?,
                    _ => app.handle_input(key.code),
                },
            }
        }
    }
}
