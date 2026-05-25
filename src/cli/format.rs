//! 数据格式化命令

use clap::Args;

#[derive(Args)]
pub struct FormatArgs {
    /// 输入文件路径，不指定则从 stdin 读取
    #[arg(value_name = "FILE")]
    pub input: Option<String>,

    /// 输出格式 (json/yaml/toml)
    #[arg(short = 't', long, default_value = "json")]
    pub format: String,

    /// 就地修改文件
    #[arg(short, long)]
    pub in_place: bool,

    /// 输出文件路径
    #[arg(short, long)]
    pub output: Option<String>,

    /// 压缩输出（最小化）
    #[arg(short, long)]
    pub compact: bool,
}

pub fn handle_format(args: FormatArgs) -> anyhow::Result<()> {
    crate::core::formatter::format(&args)
}
