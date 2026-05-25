//! 编码命令

use clap::Args;

#[derive(Args)]
pub struct EncodeArgs {
    /// 编码类型 (base64/url/unicode)
    #[arg(value_name = "TYPE")]
    pub encoding: String,

    /// 输入内容，不指定则从 stdin 读取
    #[arg(value_name = "INPUT")]
    pub input: Option<String>,

    /// 输出文件路径
    #[arg(short, long)]
    pub output: Option<String>,
}

pub fn handle_encode(args: EncodeArgs) -> anyhow::Result<()> {
    crate::core::encoder::encode(&args)
}
