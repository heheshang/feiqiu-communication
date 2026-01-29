// src-tauri/src/network/feiq/utils.rs
//
//! 协议工具函数
//!
//! 提供 MAC 地址格式化和时间戳转换等辅助函数

use crate::network::feiq::parser::ParseError;

/// 格式化12位原始MAC地址（如5C60BA7361C6 → 5C-60-BA-73-61-C6）
pub fn format_mac_addr(raw_mac: &str) -> Result<String, ParseError> {
    if raw_mac.len() != 12 {
        return Err(ParseError::InvalidFormat(
            "MAC地址长度错误，需为12位十六进制字符串".to_string(),
        ));
    }
    // 每2个字符拆分，用-连接
    let chunks: Vec<&str> = raw_mac
        .as_bytes()
        .chunks(2)
        .map(|chunk| std::str::from_utf8(chunk).unwrap())
        .collect();
    Ok(chunks.join("-"))
}

/// 将 Unix 时间戳转换为本地时间字符串
pub fn timestamp_to_local(timestamp: i64) -> String {
    use chrono::{TimeZone, Utc};
    Utc.timestamp_opt(timestamp, 0)
        .single()
        .map(|dt| dt.with_timezone(&chrono::Local).format("%Y-%m-%d %H:%M:%S").to_string())
        .unwrap_or_else(|| "Invalid timestamp".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_mac_addr() {
        let mac = "5C60BA7361C6";
        let formatted = format_mac_addr(mac).unwrap();
        assert_eq!(formatted, "5C-60-BA-73-61-C6");
    }

    #[test]
    fn test_format_mac_addr_invalid_length() {
        let mac = "5C60BA73";
        assert!(format_mac_addr(mac).is_err());
    }

    #[test]
    fn test_timestamp_to_local() {
        let timestamp = 1765442982; // 2025-08-12 10:09:42 UTC
        let local = timestamp_to_local(timestamp);
        // 不做精确断言，因为时区不同
        assert!(local.contains("2025"));
    }
}
