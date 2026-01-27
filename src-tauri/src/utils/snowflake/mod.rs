// src-tauri/src/utils/snowflake/mod.rs
//
/// 雪花算法 ID 生成器
/// TODO: 实现分布式唯一 ID 生成

use std::sync::atomic::{AtomicU64, Ordering};

/// 雪花算法生成器
#[allow(dead_code)]
pub struct SnowflakeGenerator {
    #[allow(dead_code)]
    epoch: u64,
    #[allow(dead_code)]
    node_id: u64,
    sequence: AtomicU64,
}

#[allow(dead_code)]
impl SnowflakeGenerator {
    /// 创建新的生成器
    #[allow(dead_code)]
    pub fn new(epoch: u64, node_id: u64) -> Self {
        Self {
            epoch,
            node_id: node_id & 0x3FF, // 10 bits
            sequence: AtomicU64::new(0),
        }
    }

    /// 生成下一个 ID
    #[allow(dead_code)]
    pub fn next_id(&self) -> i64 {
        // TODO: 实现完整的雪花算法
        // 暂时返回一个简单的递增 ID
        self.sequence.fetch_add(1, Ordering::SeqCst) as i64
    }
}

impl Default for SnowflakeGenerator {
    fn default() -> Self {
        Self::new(1704067200000, 1) // 2024-01-01 00:00:00 UTC
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_id() {
        let gen = SnowflakeGenerator::default();
        let id1 = gen.next_id();
        let id2 = gen.next_id();
        assert!(id2 > id1);
    }
}
