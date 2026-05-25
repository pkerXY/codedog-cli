//! TUI 界面模块

mod app;
mod ui;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
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
            match (key.modifiers, key.code) {
                (KeyModifiers::CONTROL, KeyCode::Char('c')) | (_, KeyCode::Char('q')) => {
                    return Ok(());
                }
                (KeyModifiers::NONE, KeyCode::Char('1')) => app.select_module(0),
                (KeyModifiers::NONE, KeyCode::Char('2')) => app.select_module(1),
                (KeyModifiers::NONE, KeyCode::Char('3')) => app.select_module(2),
                (KeyModifiers::NONE, KeyCode::Char('4')) => app.select_module(3),
                (KeyModifiers::NONE, KeyCode::Tab) => app.toggle_focus(),
                (KeyModifiers::NONE, KeyCode::Enter) => app.execute(),
                (KeyModifiers::NONE, KeyCode::Char('e')) => app.edit_mode(),
                (KeyModifiers::NONE, KeyCode::Char('p')) => app.paste(),
                (KeyModifiers::NONE, KeyCode::Char('f')) => app.load_file()?,
                (KeyModifiers::NONE, KeyCode::Char('y')) => app.copy_output()?,
                (KeyModifiers::NONE, KeyCode::Char('s')) => app.save_output()?,
                (KeyModifiers::NONE, KeyCode::Char('w')) => app.toggle_format(),
                (KeyModifiers::NONE, KeyCode::Char('?')) => app.show_help(),
                (KeyModifiers::NONE, KeyCode::Esc) => app.cancel(),
                _ => app.handle_input(key.code)?,
            }
        }
    }
}
