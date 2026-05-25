//! 编码转换模块

use crate::cli::{EncodeArgs, DecodeArgs, HashArgs};

use sha1::Sha1;
use sha2::{Digest, Sha256, Sha512};
use md5::Md5;

pub fn encode(args: &EncodeArgs) -> anyhow::Result<()> {
    let input = crate::utils::input::read_input(&args.input)?;

    let output = match args.encoding.to_lowercase().as_str() {
        "base64" => base64_encode(&input),
        "url" => url_encode(&input),
        "unicode" => unicode_encode(&input),
        _ => anyhow::bail!("不支持的编码类型: {}", args.encoding),
    };

    if let Some(ref output_path) = args.output {
        std::fs::write(output_path, &output)?;
    } else {
        println!("{}", output);
    }

    Ok(())
}

pub fn decode(args: &DecodeArgs) -> anyhow::Result<()> {
    let input = crate::utils::input::read_input(&args.input)?;

    let output = match args.encoding.to_lowercase().as_str() {
        "base64" => base64_decode(&input)?,
        "url" => url_decode(&input)?,
        "unicode" => unicode_decode(&input)?,
        _ => anyhow::bail!("不支持的解码类型: {}", args.encoding),
    };

    if let Some(ref output_path) = args.output {
        std::fs::write(output_path, &output)?;
    } else {
        println!("{}", output);
    }

    Ok(())
}

pub fn hash(args: &HashArgs) -> anyhow::Result<()> {
    let input = crate::utils::input::read_input_bytes(&args.input)?;

    let result = match args.algorithm.to_lowercase().as_str() {
        "md5" => {
            let mut hasher = Md5::new();
            hasher.update(&input);
            format!("{:x}", hasher.finalize())
        }
        "sha1" => {
            let mut hasher = Sha1::new();
            hasher.update(&input);
            format!("{:x}", hasher.finalize())
        }
        "sha256" => {
            let mut hasher = Sha256::new();
            hasher.update(&input);
            format!("{:x}", hasher.finalize())
        }
        "sha512" => {
            let mut hasher = Sha512::new();
            hasher.update(&input);
            format!("{:x}", hasher.finalize())
        }
        _ => anyhow::bail!("不支持的哈希算法: {}", args.algorithm),
    };

    println!("{}", result);
    Ok(())
}

fn base64_encode(input: &str) -> String {
    base64::Engine::encode(&base64::engine::general_purpose::STANDARD, input.as_bytes())
}

fn base64_decode(input: &str) -> anyhow::Result<String> {
    let decoded = base64::Engine::decode(&base64::engine::general_purpose::STANDARD, input)?;
    Ok(String::from_utf8(decoded)?)
}

fn url_encode(input: &str) -> String {
    urlencoding::encode(input).to_string()
}

fn url_decode(input: &str) -> anyhow::Result<String> {
    Ok(urlencoding::decode(input)?.to_string())
}

fn unicode_encode(input: &str) -> String {
    input.chars()
        .map(|c| format!("\\u{:04x}", c as u32))
        .collect()
}

fn unicode_decode(input: &str) -> anyhow::Result<String> {
    let mut result = String::new();
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '\\' && chars.peek() == Some(&'u') {
            chars.next(); // skip 'u'
            let hex: String = chars.by_ref().take(4).collect();
            let code = u32::from_str_radix(&hex, 16)?;
            result.push(char::from_u32(code).unwrap_or('?'));
        } else {
            result.push(c);
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base64_encode() {
        let result = base64_encode("hello");
        assert_eq!(result, "aGVsbG8=");
    }

    #[test]
    fn test_base64_decode() {
        let result = base64_decode("aGVsbG8=").unwrap();
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_url_encode() {
        let result = url_encode("hello world");
        assert_eq!(result, "hello%20world");
    }

    #[test]
    fn test_url_decode() {
        let result = url_decode("hello%20world").unwrap();
        assert_eq!(result, "hello world");
    }

    #[test]
    fn test_unicode_encode() {
        let result = unicode_encode("dog");
        assert_eq!(result, "\\u0064\\u006f\\u0067");
    }

    #[test]
    fn test_unicode_decode() {
        let result = unicode_decode("\\u0064\\u006f\\u0067").unwrap();
        assert_eq!(result, "dog");
    }
}
