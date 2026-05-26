//! 输入处理模块

use std::io::{self, Read};
use std::path::Path;

/// 输入来源类型
#[allow(dead_code)]
pub enum InputSource {
    /// 文件输入（包含文件路径）
    File(String),
    /// 文本输入
    Text(String),
    /// 标准输入
    Stdin,
}

/// 判断输入来源类型（不读取内容）
///
/// 与 `read_input` 使用相同的判断逻辑：
/// 1. 如果提供了输入参数且是有效文件路径，则为文件输入
/// 2. 如果提供了输入参数但不是文件路径，则为文本输入
/// 3. 如果没有提供输入参数，则为标准输入
pub fn classify_input(input: &Option<String>) -> InputSource {
    match input {
        Some(content) => {
            let path = Path::new(content);
            if path.exists() && path.is_file() {
                InputSource::File(content.clone())
            } else {
                InputSource::Text(content.clone())
            }
        }
        None => InputSource::Stdin,
    }
}

/// 从文件、文本或标准输入读取文本内容
///
/// 优先级：
/// 1. 如果提供了输入参数且是有效文件路径，则从文件读取
/// 2. 如果提供了输入参数但不是文件路径，则将参数作为文本内容
/// 3. 如果没有提供输入参数，则从 stdin 读取
pub fn read_input(input: &Option<String>) -> anyhow::Result<String> {
    match input {
        Some(content) => {
            // 检查是否是有效的文件路径
            let path = Path::new(content);
            if path.exists() && path.is_file() {
                let file_content = std::fs::read_to_string(content)?;
                Ok(file_content)
            } else {
                // 不是文件路径，直接作为文本内容
                Ok(content.clone())
            }
        }
        None => {
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

/// 从文件、文本或标准输入读取二进制内容
///
/// 优先级：
/// 1. 如果提供了输入参数且是有效文件路径，则从文件读取
/// 2. 如果提供了输入参数但不是文件路径，则将参数作为文本内容（转换为字节）
/// 3. 如果没有提供输入参数，则从 stdin 读取
pub fn read_input_bytes(input: &Option<String>) -> anyhow::Result<Vec<u8>> {
    match input {
        Some(content) => {
            // 检查是否是有效的文件路径
            let path = Path::new(content);
            if path.exists() && path.is_file() {
                let file_content = std::fs::read(content)?;
                Ok(file_content)
            } else {
                // 不是文件路径，直接作为文本内容
                Ok(content.as_bytes().to_vec())
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
        // 非文件路径的输入应作为文本处理
        let result = read_input(&Some("hello world".to_string())).unwrap();
        assert_eq!(result, "hello world");
    }

    #[test]
    fn test_read_input_from_text_with_special_chars() {
        // 特殊字符文本
        let result = read_input(&Some("www.yxynb.com".to_string())).unwrap();
        assert_eq!(result, "www.yxynb.com");
    }

    #[test]
    fn test_read_input_empty_text() {
        // 空字符串文本
        let result = read_input(&Some("".to_string())).unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_read_input_json_text() {
        // JSON 文本，直接返回
        let json = r#"{"name":"dog"}"#.to_string();
        let result = read_input(&Some(json.clone())).unwrap();
        assert_eq!(result, json);
    }

    #[test]
    fn test_read_input_priority_file_over_text() {
        // 创建一个名为 "abc" 的临时文件
        let mut temp_file = NamedTempFile::new().unwrap();
        write!(temp_file, "file content").unwrap();

        let path = temp_file.path().to_str().unwrap().to_string();
        // 如果路径存在且是文件，应该从文件读取
        let result = read_input(&Some(path.clone())).unwrap();
        assert_eq!(result, "file content");
    }

    #[test]
    fn test_read_input_bytes_from_file() {
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(b"binary data").unwrap();

        let path = temp_file.path().to_str().unwrap().to_string();
        let result = read_input_bytes(&Some(path)).unwrap();
        assert_eq!(result, b"binary data");
    }

    #[test]
    fn test_read_input_bytes_from_text() {
        // 非文件路径的输入应作为文本处理
        let result = read_input_bytes(&Some("abc".to_string())).unwrap();
        assert_eq!(result, b"abc");
    }

    #[test]
    fn test_read_input_bytes_from_url_like_text() {
        // URL 形式的文本（不是文件路径）
        let result = read_input_bytes(&Some("www.yxynb.com".to_string())).unwrap();
        assert_eq!(result, b"www.yxynb.com");
    }
}
