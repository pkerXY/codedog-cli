//! 哈希计算命令

use clap::Args;

#[derive(Args)]
pub struct HashArgs {
    /// 哈希算法 (md5/sha1/sha256/sha512)
    #[arg(value_name = "ALGORITHM")]
    pub algorithm: String,

    /// 输入文件路径，不指定则从 stdin 读取
    #[arg(value_name = "FILE")]
    pub input: Option<String>,
}

pub fn handle_hash(args: HashArgs) -> anyhow::Result<()> {
    crate::core::encoder::hash(&args)
}
