//! 时间工具命令

use clap::Args;

#[derive(Args)]
pub struct TimeArgs {
    /// 输入时间戳或日期字符串，不指定则显示当前时间
    #[arg(value_name = "INPUT")]
    pub input: Option<String>,

    /// 时区 (如 Asia/Shanghai)
    #[arg(short, long, default_value = "UTC")]
    pub timezone: String,

    /// 输出格式
    #[arg(short = 'f', long, default_value = "default")]
    pub format: String,
}

pub fn handle_time(args: TimeArgs) -> anyhow::Result<()> {
    crate::core::time_utils::handle_time(&args)
}
