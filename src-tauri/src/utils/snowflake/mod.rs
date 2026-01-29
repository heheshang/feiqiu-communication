// src-tauri/src/utils/snowflake/mod.rs
//
/// 雪花算法 ID 生成器
///
/// Snowflake ID 结构 (64-bit):
/// - 1 bit: 符号位 (始终为0)
/// - 41 bits: 时间戳 (毫秒级，从自定义 epoch 开始)
/// - 10 bits: 节点/机器 ID
/// - 12 bits: 序列号
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

/// 雪花算法生成器
pub struct SnowflakeGenerator {
    /// 自定义 epoch (毫秒)
    epoch: u64,
    /// 节点 ID (10 bits, 范围: 0-1023)
    node_id: u64,
    /// 序列号 (12 bits, 范围: 0-4095)
    sequence: AtomicU64,
    /// 上次生成 ID 的时间戳
    last_timestamp: AtomicU64,
}

impl SnowflakeGenerator {
    /// 创建新的生成器
    ///
    /// # 参数
    /// - `epoch`: 自定义 epoch 时间戳 (毫秒)
    /// - `node_id`: 节点 ID (必须在 0-1023 范围内)
    ///
    /// # Panics
    /// 如果 node_id 超过 10 bits (大于 1023) 会 panic
    pub fn new(epoch: u64, node_id: u64) -> Self {
        assert!(node_id <= 0x3FF, "Node ID must be between 0 and 1023");

        Self {
            epoch,
            node_id: node_id & 0x3FF,
            sequence: AtomicU64::new(0),
            last_timestamp: AtomicU64::new(0),
        }
    }

    /// 生成下一个 ID
    ///
    /// # 返回
    /// 返回一个唯一的 64-bit ID
    ///
    /// # 线程安全
    /// 此方法是线程安全的，可以多线程同时调用
    pub fn next_id(&self) -> i64 {
        loop {
            // 获取当前时间戳 (毫秒)
            let current_timestamp = self.get_millis();

            // 计算相对时间戳
            let relative_timestamp = current_timestamp - self.epoch;

            // 尝试获取上次时间戳
            let last_ts = self.last_timestamp.load(Ordering::Acquire);

            if relative_timestamp == last_ts {
                // 同一毫秒内，增加序列号
                let seq = self.sequence.fetch_add(1, Ordering::AcqRel);

                if seq >= 0xFFF {
                    // 序列号溢出，等待下一毫秒
                    std::thread::sleep(std::time::Duration::from_millis(1));
                    continue;
                }

                // 组装 ID
                return self.compose_id(relative_timestamp, seq);
            } else if relative_timestamp > last_ts {
                // 新的毫秒，重置序列号
                if self
                    .last_timestamp
                    .compare_exchange(last_ts, relative_timestamp, Ordering::AcqRel, Ordering::Acquire)
                    .is_ok()
                {
                    self.sequence.store(0, Ordering::Release);
                    return self.compose_id(relative_timestamp, 0);
                }
                // CAS 失败，重试
            } else {
                // 时钟回拨，等待时钟追上
                warn_clock_backward();
                std::thread::sleep(std::time::Duration::from_millis(1));
            }
        }
    }

    /// 组装 ID
    #[inline]
    fn compose_id(&self, timestamp: u64, sequence: u64) -> i64 {
        // ID 结构: (timestamp << 22) | (node_id << 12) | sequence
        // - timestamp: 41 bits (左移 22 位)
        // - node_id: 10 bits (左移 12 位)
        // - sequence: 12 bits (不位移)
        let id = (timestamp << 22) | (self.node_id << 12) | sequence;
        id as i64
    }

    /// 获取当前毫秒级时间戳
    #[inline]
    fn get_millis(&self) -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("System time should be after Unix epoch")
            .as_millis() as u64
    }

    /// 获取节点 ID
    pub fn node_id(&self) -> u64 {
        self.node_id
    }

    /// 获取自定义 epoch
    pub fn epoch(&self) -> u64 {
        self.epoch
    }
}

impl Default for SnowflakeGenerator {
    fn default() -> Self {
        // 默认 epoch: 2024-01-01 00:00:00 UTC
        Self::new(1704067200000, 1)
    }
}

/// 警告时钟回拨
#[cold]
fn warn_clock_backward() {
    tracing::warn!("Clock backward detected, waiting for clock to catch up");
}

// ============================================================
// 测试
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_id() {
        let gen = SnowflakeGenerator::default();
        let id1 = gen.next_id();
        let id2 = gen.next_id();
        assert!(id2 > id1, "IDs should be monotonically increasing");
    }

    #[test]
    fn test_unique_ids() {
        let gen = SnowflakeGenerator::default();
        let mut ids = std::collections::HashSet::new();

        // 生成 10000 个 ID，确保唯一性
        for _ in 0..10000 {
            ids.insert(gen.next_id());
        }

        assert_eq!(ids.len(), 10000, "All IDs should be unique");
    }

    #[test]
    fn test_node_id_clamp() {
        // 测试 node_id 超出范围
        let result = std::panic::catch_unwind(|| {
            SnowflakeGenerator::new(0, 1024); // 超过 10 bits
        });
        assert!(result.is_err(), "Should panic when node_id > 1023");
    }

    #[test]
    fn test_id_structure() {
        let gen = SnowflakeGenerator::new(1704067200000, 123); // 2024-01-01
        let id = gen.next_id();

        // 验证 ID 是正数 (符号位为 0)
        assert!(id > 0, "ID should be positive");

        // 验证 node_id 部分正确
        let node_id_part = (id as u64 >> 12) & 0x3FF;
        assert_eq!(node_id_part, 123, "Node ID part should match");
    }

    #[test]
    fn test_concurrent_generation() {
        let gen = std::sync::Arc::new(SnowflakeGenerator::default());
        let mut handles = vec![];

        // 多线程并发生成 ID
        for _ in 0..10 {
            let gen_clone = gen.clone();
            handles.push(std::thread::spawn(move || {
                let mut ids = vec![];
                for _ in 0..1000 {
                    ids.push(gen_clone.next_id());
                }
                ids
            }));
        }

        // 收集所有 ID
        let mut all_ids = std::collections::HashSet::new();
        for handle in handles {
            let ids = handle.join().unwrap();
            for id in ids {
                assert!(all_ids.insert(id), "Duplicate ID found: {}", id);
            }
        }

        assert_eq!(all_ids.len(), 10000, "Should have 10000 unique IDs");
    }
}
