//! CodeDog - 程序员工具箱
//!
//! 一个面向个人开发者的轻量级 CLI+TUI 工具箱

mod cli;
mod core;
mod tui;
mod utils;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "dog")]
#[command(about = "程序员工具箱", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 数据格式化 (JSON/YAML/TOML)
    Format(cli::FormatArgs),
    /// 编码转换 (Base64/URL)
    Encode(cli::EncodeArgs),
    /// 解码 (Base64/URL)
    Decode(cli::DecodeArgs),
    /// 哈希计算 (MD5/SHA)
    Hash(cli::HashArgs),
    /// 时间工具
    Time(cli::TimeArgs),
    /// 启动交互式 TUI 界面
    Tui,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Format(args) => cli::handle_format(args)?,
        Commands::Encode(args) => cli::handle_encode(args)?,
        Commands::Decode(args) => cli::handle_decode(args)?,
        Commands::Hash(args) => cli::handle_hash(args)?,
        Commands::Time(args) => cli::handle_time(args)?,
        Commands::Tui => tui::run()?,
    }

    Ok(())
}
