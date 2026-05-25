//! 输入处理模块

use std::io::{self, Read};

/// 从文件或标准输入读取文本内容
pub fn read_input(input: &Option<String>) -> anyhow::Result<String> {
    match input {
        Some(path) => {
            let content = std::fs::read_to_string(path)?;
            Ok(content)
        }
        None => {
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer)?;
            Ok(buffer)
        }
    }
}

/// 从文件或标准输入读取二进制内容
pub fn read_input_bytes(input: &Option<String>) -> anyhow::Result<Vec<u8>> {
    match input {
        Some(path) => {
            let content = std::fs::read(path)?;
            Ok(content)
        }
        None => {
            let mut buffer = Vec::new();
            io::stdin().read_to_end(&mut buffer)?;
            Ok(buffer)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_read_input_from_file() {
        let mut temp_file = NamedTempFile::new().unwrap();
        write!(temp_file, "hello world").unwrap();

        let path = temp_file.path().to_str().unwrap().to_string();
        let result = read_input(&Some(path)).unwrap();
        assert_eq!(result, "hello world");
    }
}
