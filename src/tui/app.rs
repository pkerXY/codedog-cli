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

/// 应用状态
pub struct App {
    /// 菜单选中索引
    pub menu_index: usize,
    /// 当前选中的模块（None 表示菜单模式）
    pub selected_module: Option<Module>,
    /// 输入内容
    pub input: String,
    /// 输出内容
    pub output: String,
    /// 模块内部焦点（0=输入框, 1=执行按钮, 2=输出区）
    pub panel_focus: usize,
    /// 是否正在编辑输入
    pub editing: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            menu_index: 0,
            selected_module: None,
            input: String::new(),
            output: String::new(),
            panel_focus: 0,
            editing: false,
        }
    }

    /// 菜单项上移
    pub fn menu_up(&mut self) {
        if self.menu_index > 0 {
            self.menu_index -= 1;
        }
    }

    /// 菜单项下移
    pub fn menu_down(&mut self) {
        if self.menu_index < Module::all().len() - 1 {
            self.menu_index += 1;
        }
    }

    /// 选中当前菜单项，进入模块
    pub fn select(&mut self) {
        self.selected_module = Some(Module::all()[self.menu_index]);
        self.panel_focus = 0;
        self.editing = false;
    }

    /// 返回菜单
    pub fn back(&mut self) {
        self.selected_module = None;
        self.editing = false;
        self.input.clear();
        self.output.clear();
    }

    /// 切换模块内部焦点
    pub fn toggle_panel_focus(&mut self) {
        self.panel_focus = (self.panel_focus + 1) % 3;
    }

    /// 执行当前模块操作
    pub fn execute(&mut self) {
        match self.selected_module {
            Some(Module::Format) => {
                if let Ok(value) = serde_json::from_str::<serde_json::Value>(&self.input) {
                    if let Ok(formatted) = serde_json::to_string_pretty(&value) {
                        self.output = formatted;
                    }
                }
            }
            Some(Module::Encode) => {
                self.output = base64::Engine::encode(
                    &base64::engine::general_purpose::STANDARD,
                    self.input.as_bytes(),
                );
            }
            Some(Module::Time) => {
                self.output = chrono::Utc::now()
                    .format("%Y-%m-%d %H:%M:%S UTC")
                    .to_string();
            }
            Some(Module::Hash) => {
                use sha2::{Digest, Sha256};
                let mut hasher = Sha256::new();
                hasher.update(self.input.as_bytes());
                self.output = format!("{:x}", hasher.finalize());
            }
            None => {}
        }
    }

    /// 开始/结束编辑
    pub fn toggle_edit(&mut self) {
        if self.panel_focus == 0 {
            self.editing = !self.editing;
        }
    }

    /// 粘贴剪贴板内容
    pub fn paste(&mut self) {
        if let Ok(content) = arboard::Clipboard::new().and_then(|mut c| c.get_text()) {
            self.input = content;
        }
    }

    /// 复制输出到剪贴板
    pub fn copy_output(&mut self) -> anyhow::Result<()> {
        let mut clipboard = arboard::Clipboard::new()?;
        clipboard.set_text(&self.output)?;
        Ok(())
    }

    /// 处理键盘输入
    pub fn handle_input(&mut self, key: KeyCode) {
        if self.editing && self.panel_focus == 0 {
            match key {
                KeyCode::Char(c) => self.input.push(c),
                KeyCode::Backspace => {
                    self.input.pop();
                }
                _ => {}
            }
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
