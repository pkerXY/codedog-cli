//! 时间工具模块

use chrono::{DateTime, TimeZone, Utc};
use chrono_tz::Tz;
use crate::cli::TimeArgs;

pub fn handle_time(args: &TimeArgs) -> anyhow::Result<()> {
    match &args.input {
        None => show_current_time(&args.timezone, &args.format),
        Some(input) => {
            if input.chars().all(|c| c.is_ascii_digit()) {
                convert_timestamp(input, &args.timezone, &args.format)
            } else {
                convert_date_string(input, &args.timezone, &args.format)
            }
        }
    }
}

fn show_current_time(timezone: &str, format: &str) -> anyhow::Result<()> {
    let tz: Tz = timezone.parse().map_err(|e: String| anyhow::anyhow!("{}", e))?;
    let now = Utc::now().with_timezone(&tz);

    let output = format_datetime(&now, format);
    println!("{}", output);

    Ok(())
}

fn convert_timestamp(timestamp: &str, timezone: &str, format: &str) -> anyhow::Result<()> {
    let ts: i64 = timestamp.parse()?;
    let tz: Tz = timezone.parse().map_err(|e: String| anyhow::anyhow!("{}", e))?;

    let dt = if timestamp.len() > 10 {
        // 毫秒时间戳
        Utc.timestamp_millis_opt(ts).single()
            .ok_or_else(|| anyhow::anyhow!("无效的时间戳"))?
    } else {
        // 秒时间戳
        Utc.timestamp_opt(ts, 0).single()
            .ok_or_else(|| anyhow::anyhow!("无效的时间戳"))?
    };

    let dt_with_tz = dt.with_timezone(&tz);
    let output = format_datetime(&dt_with_tz, format);
    println!("{}", output);

    Ok(())
}

fn convert_date_string(date_str: &str, timezone: &str, _format: &str) -> anyhow::Result<()> {
    let tz: Tz = timezone.parse().map_err(|e: String| anyhow::anyhow!("{}", e))?;

    // 尝试解析常见日期格式
    let dt = parse_date_string(date_str, tz)?;

    let timestamp = dt.timestamp();
    println!("{}", timestamp);

    Ok(())
}

fn parse_date_string(date_str: &str, tz: Tz) -> anyhow::Result<DateTime<Tz>> {
    // 尝试多种常见格式
    let formats = [
        "%Y-%m-%d %H:%M:%S",
        "%Y-%m-%dT%H:%M:%S",
        "%Y-%m-%d %H:%M",
        "%Y-%m-%d",
        "%Y/%m/%d %H:%M:%S",
        "%Y/%m/%d",
    ];

    for fmt in formats {
        // 先尝试解析为 NaiveDateTime，再转换为时区时间
        if let Ok(naive) = chrono::NaiveDateTime::parse_from_str(date_str, fmt) {
            return Ok(tz.from_utc_datetime(&naive));
        }
        // 尝试解析为 NaiveDate（只有日期没有时间）
        if let Ok(naive_date) = chrono::NaiveDate::parse_from_str(date_str, fmt) {
            let naive_dt = naive_date.and_hms_opt(0, 0, 0).unwrap();
            return Ok(tz.from_utc_datetime(&naive_dt));
        }
    }

    anyhow::bail!("无法解析日期: {}", date_str)
}

fn format_datetime(dt: &DateTime<Tz>, format: &str) -> String {
    match format {
        "default" => dt.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
        "iso" => dt.to_rfc3339(),
        "date" => dt.format("%Y-%m-%d").to_string(),
        "time" => dt.format("%H:%M:%S").to_string(),
        "timestamp" => dt.timestamp().to_string(),
        custom => dt.format(custom).to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Datelike;

    #[test]
    fn test_parse_date_string() {
        let tz: Tz = "UTC".parse().unwrap();
        let dt = parse_date_string("2024-01-01", tz).unwrap();
        assert_eq!(dt.year(), 2024);
        assert_eq!(dt.month(), 1);
        assert_eq!(dt.day(), 1);
    }
}
