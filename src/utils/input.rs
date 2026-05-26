//! 输入处理模块

use std::io::{self, Read};
use std::path::Path;

/// 从文件、标准输入或直接文本读取内容
///
/// # 参数
/// - `input`: 可选的输入参数
///   - 如果是有效文件路径，从文件读取
///   - 如果是文本内容，直接返回
///   - 如果为 None，从标准输入读取
pub fn read_input(input: &Option<String>) -> anyhow::Result<String> {
    match input {
        Some(path_or_text) => {
            // 判断是否为有效文件路径
            let path = Path::new(path_or_text);
            if path.exists() && path.is_file() {
                // 从文件读取
                let content = std::fs::read_to_string(path)?;
                Ok(content)
            } else {
                // 直接作为文本内容返回
                Ok(path_or_text.clone())
            }
        }
        None => {
            // 从标准输入读取
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer)?;
            Ok(buffer)
        }
    }
}

/// 从文件路径读取内容（仅支持文件）
///
/// 用于需要区分文件和文本的场景，如 `-i` 就地修改
pub fn read_file_content(path: &str) -> anyhow::Result<String> {
    let content = std::fs::read_to_string(path)?;
    Ok(content)
}

/// 从文件或标准输入读取二进制内容
pub fn read_input_bytes(input: &Option<String>) -> anyhow::Result<Vec<u8>> {
    match input {
        Some(path_or_text) => {
            // 判断是否为有效文件路径
            let path = Path::new(path_or_text);
            if path.exists() && path.is_file() {
                let content = std::fs::read(path)?;
                Ok(content)
            } else {
                // 作为文本处理
                Ok(path_or_text.as_bytes().to_vec())
            }
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

    #[test]
    fn test_read_input_from_text() {
        // 非文件路径，直接返回文本
        let result = read_input(&Some("hello world".to_string())).unwrap();
        assert_eq!(result, "hello world");
    }

    #[test]
    fn test_read_input_json_text() {
        // JSON 文本，直接返回
        let json = r#"{"name":"dog"}"#.to_string();
        let result = read_input(&Some(json.clone())).unwrap();
        assert_eq!(result, json);
    }
}
