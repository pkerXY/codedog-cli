//! TUI 应用状态

use crossterm::event::KeyCode;

/// 功能模块
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Module {
    Format,
    Encode,
    Time,
    Hash,
}

impl Module {
    pub fn name(&self) -> &'static str {
        match self {
            Module::Format => "格式化",
            Module::Encode => "编码转换",
            Module::Time => "时间工具",
            Module::Hash => "哈希计算",
        }
    }

    pub fn all() -> [Module; 4] {
        [Module::Format, Module::Encode, Module::Time, Module::Hash]
    }
}

/// 焦点位置
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Focus {
    Menu,
    Input,
    Output,
}

/// 应用状态
pub struct App {
    /// 当前模块
    pub current_module: Module,
    /// 焦点位置
    pub focus: Focus,
    /// 输入内容
    pub input: String,
    /// 输出内容
    pub output: String,
    /// 是否正在编辑
    pub editing: bool,
    /// 当前格式
    pub format: String,
    /// 帮助信息
    pub show_help: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            current_module: Module::Format,
            focus: Focus::Menu,
            input: String::new(),
            output: String::new(),
            editing: false,
            format: "json".to_string(),
            show_help: false,
        }
    }

    pub fn select_module(&mut self, index: usize) {
        self.current_module = match index {
            0 => Module::Format,
            1 => Module::Encode,
            2 => Module::Time,
            3 => Module::Hash,
            _ => return,
        };
        self.focus = Focus::Input;
    }

    pub fn toggle_focus(&mut self) {
        self.focus = match self.focus {
            Focus::Menu => Focus::Input,
            Focus::Input => Focus::Output,
            Focus::Output => Focus::Menu,
        };
    }

    pub fn execute(&mut self) {
        // TODO: 执行当前模块的操作
        match self.current_module {
            Module::Format => {
                // 尝试格式化输入
                if let Ok(value) = serde_json::from_str::<serde_json::Value>(&self.input) {
                    if let Ok(formatted) = serde_json::to_string_pretty(&value) {
                        self.output = formatted;
                    }
                }
            }
            Module::Encode => {
                self.output = base64::Engine::encode(
                    &base64::engine::general_purpose::STANDARD,
                    self.input.as_bytes(),
                );
            }
            Module::Time => {
                self.output = chrono::Utc::now()
                    .format("%Y-%m-%d %H:%M:%S UTC")
                    .to_string();
            }
            Module::Hash => {
                use sha2::{Digest, Sha256};
                let mut hasher = Sha256::new();
                hasher.update(self.input.as_bytes());
                self.output = format!("{:x}", hasher.finalize());
            }
        }
    }

    pub fn edit_mode(&mut self) {
        if self.focus == Focus::Input {
            self.editing = !self.editing;
        }
    }

    pub fn paste(&mut self) {
        if self.focus == Focus::Input {
            if let Ok(content) = arboard::Clipboard::new().and_then(|mut c| c.get_text()) {
                self.input = content;
            }
        }
    }

    pub fn load_file(&mut self) -> anyhow::Result<()> {
        // TODO: 实现文件选择对话框
        Ok(())
    }

    pub fn copy_output(&mut self) -> anyhow::Result<()> {
        let mut clipboard = arboard::Clipboard::new()?;
        clipboard.set_text(&self.output)?;
        Ok(())
    }

    pub fn save_output(&mut self) -> anyhow::Result<()> {
        // TODO: 实现文件保存对话框
        Ok(())
    }

    pub fn toggle_format(&mut self) {
        self.format = match self.format.as_str() {
            "json" => "yaml".to_string(),
            "yaml" => "toml".to_string(),
            "toml" => "json".to_string(),
            _ => "json".to_string(),
        };
    }

    pub fn show_help(&mut self) {
        self.show_help = !self.show_help;
    }

    pub fn cancel(&mut self) {
        self.editing = false;
        self.show_help = false;
    }

    pub fn handle_input(&mut self, key: KeyCode) -> anyhow::Result<()> {
        if self.editing && self.focus == Focus::Input {
            match key {
                KeyCode::Char(c) => self.input.push(c),
                KeyCode::Backspace => {
                    self.input.pop();
                }
                _ => {}
            }
        }
        Ok(())
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
