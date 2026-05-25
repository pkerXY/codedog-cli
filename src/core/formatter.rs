//! 数据格式化模块

use crate::cli::FormatArgs;

pub fn format(args: &FormatArgs) -> anyhow::Result<()> {
    let input = crate::utils::input::read_input(&args.input)?;

    let output = match args.format.as_str() {
        "json" => format_as_json(&input, args.compact)?,
        "yaml" => format_as_yaml(&input)?,
        "toml" => format_as_toml(&input)?,
        _ => anyhow::bail!("不支持的格式: {}", args.format),
    };

    if let Some(ref output_path) = args.output {
        std::fs::write(output_path, &output)?;
    } else {
        println!("{}", output);
    }

    Ok(())
}

fn format_as_json(input: &str, compact: bool) -> anyhow::Result<String> {
    let value: serde_json::Value = serde_json::from_str(input)?;

    if compact {
        Ok(serde_json::to_string(&value)?)
    } else {
        Ok(serde_json::to_string_pretty(&value)?)
    }
}

fn format_as_yaml(input: &str) -> anyhow::Result<String> {
    let value: serde_json::Value = serde_json::from_str(input)?;
    Ok(serde_yaml::to_string(&value)?)
}

fn format_as_toml(input: &str) -> anyhow::Result<String> {
    let value: serde_json::Value = serde_json::from_str(input)?;
    let toml_value = toml::Value::try_from(value)?;
    Ok(toml::to_string_pretty(&toml_value)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_json_pretty() {
        let input = r#"{"name":"dog","version":1}"#;
        let result = format_as_json(input, false).unwrap();
        assert!(result.contains('\n'));
        assert!(result.contains("  \"name\""));
    }

    #[test]
    fn test_format_json_compact() {
        let input = r#"{"name":"dog","version":1}"#;
        let result = format_as_json(input, true).unwrap();
        assert!(!result.contains('\n'));
        assert!(result.contains("\"name\":\"dog\""));
    }

    #[test]
    fn test_format_yaml() {
        let input = r#"{"name":"dog","version":1}"#;
        let result = format_as_yaml(input).unwrap();
        assert!(result.contains("name: dog"));
    }

    #[test]
    fn test_format_toml() {
        let input = r#"{"name":"dog","version":1}"#;
        let result = format_as_toml(input).unwrap();
        assert!(result.contains("name = \"dog\""));
    }
}
