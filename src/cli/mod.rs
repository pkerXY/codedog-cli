//! CLI 命令模块

pub mod format;
pub mod encode;
pub mod decode;
pub mod hash;
pub mod time;

pub use format::FormatArgs;
pub use encode::EncodeArgs;
pub use decode::DecodeArgs;
pub use hash::HashArgs;
pub use time::TimeArgs;

pub use format::handle_format;
pub use encode::handle_encode;
pub use decode::handle_decode;
pub use hash::handle_hash;
pub use time::handle_time;
