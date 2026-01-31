// src-tauri/src/event/bus.rs
//
use crate::event::model::AppEvent;
/// 全局事件总线
/// 参考: reference/event/bus.rs
use crossbeam_channel::{unbounded, Receiver, Sender};
use once_cell::sync::Lazy;

// ============================================================
// 全局事件总线
// ============================================================

/// 全局事件总线（单例）
pub static EVENT_BUS: Lazy<EventBus<AppEvent>> = Lazy::new(|| {
    let (tx, rx) = unbounded();
    EventBus::new(tx, rx)
});

/// 事件发送器（全局可访问）
///
/// # 使用示例
///
/// ```rust
/// use crate::event::bus::EVENT_SENDER;
/// use crate::event::model::{AppEvent, NetworkEvent};
///
/// EVENT_SENDER.send(AppEvent::Network(NetworkEvent::UserOnline {
///     ip: "192.168.1.100".to_string(),
///     port: 2425,
///     nickname: "user".to_string(),
///     hostname: Some("hostname".to_string()),
///     mac_addr: Some("00:11:22:33:44:55".to_string()),
/// })).unwrap();
/// ```
pub static EVENT_SENDER: Lazy<Sender<AppEvent>> = Lazy::new(|| EVENT_BUS.sender().clone());

/// 事件接收器（全局可访问）
pub static EVENT_RECEIVER: Lazy<Receiver<AppEvent>> = Lazy::new(|| EVENT_BUS.receiver().clone());

// ============================================================
// EventBus 结构体
// ============================================================

/// 事件总线
///
/// 使用 crossbeam-channel 实现的高性能无锁通道
pub struct EventBus<T> {
    tx: Sender<T>,
    rx: Receiver<T>,
}

impl<T> EventBus<T> {
    /// 创建新的事件总线
    pub fn new(tx: Sender<T>, rx: Receiver<T>) -> Self {
        Self { tx, rx }
    }

    /// 获取发送器
    pub fn sender(&self) -> &Sender<T> {
        &self.tx
    }

    /// 获取接收器
    pub fn receiver(&self) -> &Receiver<T> {
        &self.rx
    }

    /// 尝试发送事件（非阻塞）
    #[allow(dead_code)]
    pub fn try_send(&self, event: T) -> Result<(), crossbeam_channel::TrySendError<T>> {
        self.tx.try_send(event)
    }

    /// 发送事件（阻塞）
    #[allow(dead_code)]
    pub fn send(&self, event: T) -> Result<(), crossbeam_channel::SendError<T>> {
        self.tx.send(event)
    }

    /// 尝试接收事件（非阻塞）
    #[allow(dead_code)]
    pub fn try_recv(&self) -> Result<T, crossbeam_channel::TryRecvError> {
        self.rx.try_recv()
    }

    /// 接收事件（阻塞）
    #[allow(dead_code)]
    pub fn recv(&self) -> Result<T, crossbeam_channel::RecvError> {
        self.rx.recv()
    }

    /// 超时接收事件
    #[allow(dead_code)]
    pub fn recv_timeout(&self, timeout: std::time::Duration) -> Result<T, crossbeam_channel::RecvTimeoutError> {
        self.rx.recv_timeout(timeout)
    }
}

// ============================================================
// 测试
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq)]
    enum TestEvent {
        Ping,
        Pong,
    }

    #[test]
    fn test_event_bus_send_recv() {
        let (tx, rx) = unbounded();
        let bus = EventBus::new(tx, rx);

        bus.send(TestEvent::Ping).unwrap();
        assert_eq!(bus.recv().unwrap(), TestEvent::Ping);
    }

    #[test]
    fn test_event_bus_try_send_recv() {
        let (tx, rx) = unbounded();
        let bus = EventBus::new(tx, rx);

        assert!(bus.try_send(TestEvent::Pong).is_ok());
        assert_eq!(bus.try_recv().unwrap(), TestEvent::Pong);
    }
}
