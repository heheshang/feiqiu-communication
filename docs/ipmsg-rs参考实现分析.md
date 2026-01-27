# ipmsg-rs 参考实现分析

> 基于 langzime/ipmsg-rs v0.7.1 的深度分析

---

## 一、项目概览

### 1.1 基本信息

| 项目 | 内容 |
|------|------|
| **仓库名称** | langzime/ipmsg-rs |
| **版本** | 0.7.1 |
| **作者** | langzime (wangyanqing@langzi.me) |
| **代码行数** | 约 1,769 行 Rust 代码 |
| **技术栈** | Rust + GTK4 + libadwaita |
| **许可证** | MIT |
| **仓库地址** | https://github.com/langzime/ipmsg-rs |

### 1.2 已实现功能

- ✅ 聊天（单聊/群聊）
- ✅ 发送文件
- ✅ 接收文件
- ✅ 在线发现（BR_ENTRY/ANSENTRY）
- ✅ 用户列表管理
- ✅ 消息已读回执

### 1.3 代码结构

```
src/
├── main.rs              (34行)    - 应用入口
├── constants/           (179行)   - 协议常量定义
│   ├── protocol.rs      (178行)   - IPMsg 协议常量
│   └── mod.rs
├── models/              (299行)   - 数据模型
│   ├── model.rs         (261行)   - 核心数据结构
│   ├── message.rs       (33行)    - 消息构造
│   ├── event.rs         (35行)    - 事件定义
│   └── mod.rs
├── core/                (395行)   - 核心业务逻辑
│   ├── mod.rs           (26行)    - 全局通道
│   ├── fileserver.rs    (195行)   - 文件发送服务
│   └── download.rs      (174行)   - 文件下载管理
├── events/              (262行)   - 事件处理
│   ├── model.rs         (260行)   - 事件循环和分发
│   └── mod.rs
├── ui/                  (521行)   - GTK4 界面
│   ├── main_win.rs      (254行)   - 主窗口
│   ├── chat_window.rs   (267行)   - 聊天窗口
│   └── mod.rs
└── util.rs              (45行)    - 工具函数
```

---

## 二、可借鉴的核心设计

### 2.1 解析器组合子模式 ⭐⭐⭐⭐⭐

**位置:** `src/util.rs:18-41`

```rust
use combine::{many1, token, satisfy, Parser};

pub fn packet_parser<Input>() -> impl Parser<Input, Output=Packet>
    where Input: Stream<Token=char>,
{
    (
        many1(satisfy(|c| c != ':')),  // 版本号
        token(':'),
        many1(satisfy(|c| c != ':')),  // 包编号
        token(':'),
        many1(satisfy(|c| c != ':')),  // 发送者名称
        token(':'),
        many1(satisfy(|c| c != ':')),  // 主机名
        token(':'),
        many1(satisfy(|c| c != ':')),  // 命令字
        token(':'),
        many(satisfy(|c| true)),       // 附加段（可选）
    ).map(|(ver, _, packet_no, _, sender, _, host, _, cmd, _, ext)| {
        let add_ext = if ext.is_empty() { None } else { Some(ext) };
        Packet::from(ver, packet_no, sender, host, cmd.parse::<u32>().unwrap(), add_ext)
    })
}
```

**优势:**
- 类型安全，编译期检查
- 声明式，易于理解
- 可组合性强
- 易于测试

**我们的实现方案:**

```rust
// src/network/feiq/parser.rs
use combine::{many1, token, satisfy, digit, Parser, Stream};
use crate::network::feiq::model::FeiqPacket;

pub fn feiq_packet_parser<Input>() -> impl Parser<Input, Output=FeiqPacket>
where
    Input: Stream<Token=char>,
{
    (
        many1(satisfy(|c| c != ':')),  // 版本号 (1.0)
        token(':'),
        many1(digit()),                // 命令字 (数字)
        token(':'),
        many1(satisfy(|c| c != ':')),  // 发送者信息
        token(':'),
        many1(satisfy(|c| c != ':')),  // 接收者信息 (可能为空)
        token(':'),
        many1(digit()),                // 消息编号
        token(':'),
        many(satisfy(|c| true)),       // 附加信息
    ).map(|(ver, _, cmd, _, sender, _, receiver, _, msg_no, _, ext)| {
        FeiqPacket {
            version: ver.into_iter().collect(),
            command: cmd.into_iter().collect::<String>().parse().unwrap(),
            sender: sender.into_iter().collect(),
            receiver: receiver.into_iter().collect(),
            msg_no: msg_no.into_iter().collect::<String>().parse().unwrap(),
            extension: if ext.is_empty() { None } else { Some(ext.into_iter().collect()) },
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_entry_packet() {
        let input = "1.0:1:admin@PC-001/192.168.1.100:2425|AA:BB:CC:DD:EE:FF::12345:";
        let result = feiq_packet_parser().parse(input);
        assert!(result.is_ok());
        let (packet, _) = result.unwrap();
        assert_eq!(packet.version, "1.0");
        assert_eq!(packet.command, 1);
    }

    #[test]
    fn test_parse_sendmsg_packet() {
        let input = "1.0:32:sender:host:12345:Hello World";
        let result = feiq_packet_parser().parse(input);
        assert!(result.is_ok());
    }
}
```

---

### 2.2 全局事件总线 ⭐⭐⭐⭐⭐

**位置:** `src/core/mod.rs`

```rust
use crossbeam_channel::unbounded;
use once_cell::sync::Lazy;

pub static GLOBAL_CHANNEL: Lazy<(
    crossbeam_channel::Sender<ModelEvent>,
    crossbeam_channel::Receiver<ModelEvent>
)> = Lazy::new(|| {
    unbounded()
});

pub static GLOBLE_SENDER: Lazy<crossbeam_channel::Sender<ModelEvent>> = Lazy::new(|| {
    GLOBAL_CHANNEL.0.clone()
});

pub static GLOBLE_RECEIVER: Lazy<crossbeam_channel::Receiver<ModelEvent>> = Lazy::new(|| {
    GLOBAL_CHANNEL.1.clone()
});
```

**事件类型定义:** `src/models/event.rs`

```rust
pub enum ModelEvent {
    ReceivedPacket { packet: Packet },
    BroadcastEntry(Packet),
    RecMsgReply { packet: Packet, from_ip: String },
    SendOneMsg { to_ip: String, packet: Packet, context: String, files: Option<ShareInfo> },
    PutDownloadTaskInPool { file: ReceivedSimpleFileInfo, save_base_path: PathBuf, download_ip: String },
    // ... 更多事件
}
```

**我们的实现方案:**

```rust
// src/event/bus.rs
use crossbeam_channel::unbounded;
use once_cell::sync::Lazy;
use crate::event::model::AppEvent;

/// 全局事件总线
pub static EVENT_BUS: Lazy<EventBus<AppEvent>> = Lazy::new(|| {
    let (tx, rx) = unbounded();
    EventBus::new(tx, rx)
});

/// 事件发送器（全局可访问）
pub static EVENT_SENDER: Lazy<crossbeam_channel::Sender<AppEvent>> =
    Lazy::new(|| EVENT_BUS.sender().clone());

/// 事件接收器（全局可访问）
pub static EVENT_RECEIVER: Lazy<crossbeam_channel::Receiver<AppEvent>> =
    Lazy::new(|| EVENT_BUS.receiver().clone());

pub struct EventBus<T> {
    tx: crossbeam_channel::Sender<T>,
    rx: crossbeam_channel::Receiver<T>,
}

impl<T> EventBus<T> {
    pub fn new(tx: crossbeam_channel::Sender<T>, rx: crossbeam_channel::Receiver<T>) -> Self {
        Self { tx, rx }
    }

    pub fn sender(&self) -> &crossbeam_channel::Sender<T> {
        &self.tx
    }

    pub fn receiver(&self) -> &crossbeam_channel::Receiver<T> {
        &self.rx
    }
}

// 使用示例
use crate::event::bus::EVENT_SENDER;
use crate::event::model::AppEvent;

EVENT_SENDER.send(AppEvent::Network(NetworkEvent::MessageReceived {
    from: "192.168.1.100".to_string(),
    content: "Hello".to_string(),
})).unwrap();
```

**事件模型定义:**

```rust
// src/event/model.rs
#[derive(Debug, Clone)]
pub enum AppEvent {
    Network(NetworkEvent),
    Ui(UiEvent),
    File(FileEvent),
    Chat(ChatEvent),
}

#[derive(Debug, Clone)]
pub enum NetworkEvent {
    PacketReceived { packet: FeiqPacket, addr: String },
    UserDiscovered { user: UserInfo },
    UserOffline { ip: String },
}

#[derive(Debug, Clone)]
pub enum UiEvent {
    ShowMessage { content: String },
    UpdateUserList { users: Vec<UserInfo> },
}
```

---

### 2.3 UDP 通信模式 ⭐⭐⭐⭐

**位置:** `src/events/model.rs:42-67`

```rust
pub fn start_daemon(socket: UdpSocket) {
    let socket_clone = socket.try_clone().unwrap();
    thread::spawn(move || {
        let mut buf = [0; 2048];  // 2KB 缓冲区
        loop {
            match socket_clone.recv_from(&mut buf) {
                Ok((amt, src)) => {
                    // 使用 GB18030 编码解析
                    let receive_str = GB18030.decode(&buf[0..amt], DecoderTrap::Strict).unwrap();
                    info!("receive raw message -> {:?} from ip -> {:?}", receive_str, src.ip());

                    // 使用 combine 解析器解析数据包
                    let result = packet_parser().parse(receive_str.as_str());
                    match result {
                        Ok((mut packet, _)) => {
                            packet.ip = src.ip().to_string();
                            GLOBLE_SENDER.send(ModelEvent::ReceivedPacket { packet }).unwrap();
                        }
                        Err(_) => {
                            error!("Invalid packet {} !", receive_str);
                        }
                    }
                },
                Err(e) => {
                    error!("couldn't receive a datagram: {}", e);
                }
            }
        }
    });
}
```

**我们的异步实现方案:**

```rust
// src/network/udp/receiver.rs
use tokio::net::UdpSocket;
use tracing::{info, error};
use crate::event::bus::EVENT_SENDER;
use crate::network::feiq::parser::feiq_packet_parser;
use crate::event::model::AppEvent;

/// 启动异步 UDP 接收器
pub async fn start_udp_receiver() -> anyhow::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:2425").await?;
    info!("UDP receiver started on port 2425");

    let mut buf = [0; 2048];

    loop {
        match socket.recv_from(&mut buf).await {
            Ok((len, addr)) => {
                let data = String::from_utf8_lossy(&buf[..len]);

                // 使用解析器解析
                match feiq_packet_parser().parse(data.as_ref()) {
                    Ok((packet, _)) => {
                        // 发送到事件总线
                        if let Err(e) = EVENT_SENDER.send(AppEvent::Network(
                            NetworkEvent::PacketReceived {
                                packet,
                                addr: addr.to_string(),
                            }
                        )) {
                            error!("Event send failed: {}", e);
                        }
                    }
                    Err(e) => {
                        error!("Failed to parse packet: {:?}", e);
                    }
                }
            }
            Err(e) => {
                error!("UDP receive error: {}", e);
            }
        }
    }
}

/// 发送 UDP 消息
pub async fn send_udp_packet(addr: &str, packet: &FeiqPacket) -> anyhow::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:0").await?;
    let data = packet.to_string();

    socket.send_to(data.as_bytes(), addr).await?;

    Ok(())
}
```

---

### 2.4 文件头格式（IPMsg 标准）⭐⭐⭐⭐

**位置:** `src/core/fileserver.rs`

```rust
// 格式: 长度:文件名:大小:属性:创建时间=值:修改时间=值:
pub fn make_header(path: &PathBuf, ret_parent: bool) -> String {
    let mut header = String::new();
    header.push(':');  // 分隔符

    if ret_parent {
        header.push_str(".");  // 返回父目录标记
    } else {
        let file_name = path.file_name().unwrap().to_str().unwrap();
        header.push_str(file_name);
    }

    header.push(':');
    let metadata = fs::metadata(&path).unwrap();
    header.push_str(format!("{:x}", metadata.len()).as_str());  // 大小（十六进制）

    header.push(':');
    let file_attr = if metadata.is_dir() { IPMSG_FILE_DIR } else { IPMSG_FILE_REGULAR };
    header.push_str(format!("{:x}", file_attr).as_str());

    header.push_str(format!(":{:x}={:x}:{:x}={:x}:",
        IPMSG_FILE_CREATETIME, timestamp,
        IPMSG_FILE_MTIME, timestamp).as_str());

    // 长度前缀（4字节十六进制）
    let length = utf8_to_gb18030(&header).len();
    header.insert_str(0, format!("{:0>4x}", length).as_str());

    header
}
```

**我们的实现方案:**

```rust
// src/core/file/protocol.rs
use std::path::Path;

const IPMSG_FILE_REGULAR: u32 = 0x00000001;
const IPMSG_FILE_DIR: u32 = 0x00000002;
const IPMSG_FILE_RETPARENT: u32 = 0x00000003;
const IPMSG_FILE_CREATETIME: u32 = 0x00000001;
const IPMSG_FILE_MTIME: u32 = 0x00000002;

#[derive(Debug, Clone)]
pub struct FileHeader {
    pub length: u16,           // 头部长度（4字节十六进制）
    pub name: String,          // 文件名
    pub size: u64,             // 文件大小（十六进制）
    pub attr: FileAttr,        // 文件属性
    pub crtime: i64,           // 创建时间
    pub mtime: i64,            // 修改时间
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FileAttr {
    Regular,
    Directory,
    RetParent,  // 返回父目录标记
}

impl FileHeader {
    /// 编码为 IPMsg 协议格式
    pub fn encode(&self) -> String {
        let attr_val = match self.attr {
            FileAttr::Regular => IPMSG_FILE_REGULAR,
            FileAttr::Directory => IPMSG_FILE_DIR,
            FileAttr::RetParent => IPMSG_FILE_RETPARENT,
        };

        format!(
            "{:04x}:{}:{:x}:{:x}:{:x}={:x}:{:x}={:x}:",
            self.length,
            self.name,
            self.size,
            attr_val,
            IPMSG_FILE_CREATETIME, self.crtime,
            IPMSG_FILE_MTIME, self.mtime
        )
    }

    /// 从 IPMsg 协议格式解析
    pub fn decode(data: &str) -> Result<Self, ParseError> {
        let parts: Vec<&str> = data.split(':').collect();
        if parts.len() < 7 {
            return Err(ParseError::InvalidFormat);
        }

        let length = u16::from_str_radix(parts[0], 16)?;
        let name = parts[1].to_string();
        let size = u64::from_str_radix(parts[2], 16)?;
        let attr_val = u32::from_str_radix(parts[3], 16)?;

        let attr = match attr_val {
            IPMSG_FILE_REGULAR => FileAttr::Regular,
            IPMSG_FILE_DIR => FileAttr::Directory,
            IPMSG_FILE_RETPARENT => FileAttr::RetParent,
            _ => return Err(ParseError::InvalidFileAttr),
        };

        let crtime = extract_time_value(parts[4])?;
        let mtime = extract_time_value(parts[5])?;

        Ok(FileHeader {
            length,
            name,
            size,
            attr,
            crtime,
            mtime,
        })
    }
}

fn extract_time_value(part: &str) -> Result<i64, ParseError> {
    let parts: Vec<&str> = part.split('=').collect();
    if parts.len() != 2 {
        return Err(ParseError::InvalidTimeFormat);
    }
    i64::from_str_radix(parts[1], 16).map_err(|_| ParseError::InvalidTimeFormat)
}

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("Invalid format")]
    InvalidFormat,
    #[error("Invalid file attribute")]
    InvalidFileAttr,
    #[error("Invalid time format")]
    InvalidTimeFormat,
    #[error("Parse int error: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_header_encode_decode() {
        let header = FileHeader {
            length: 0,
            name: "test.txt".to_string(),
            size: 1024,
            attr: FileAttr::Regular,
            crtime: 0,
            mtime: 0,
        };

        let encoded = header.encode();
        let decoded = FileHeader::decode(&encoded).unwrap();

        assert_eq!(decoded.name, "test.txt");
        assert_eq!(decoded.size, 1024);
        assert_eq!(decoded.attr, FileAttr::Regular);
    }

    #[test]
    fn test_directory_header() {
        let header = FileHeader {
            length: 0,
            name: "docs".to_string(),
            size: 0,
            attr: FileAttr::Directory,
            crtime: 0,
            mtime: 0,
        };

        let encoded = header.encode();
        assert!(encoded.contains("2"));  // IPMSG_FILE_DIR
    }
}
```

---

### 2.5 下载池管理 ⭐⭐⭐⭐

**位置:** `src/core/download.rs`

```rust
pub struct ManagerPool {
    pub file_pool: Arc<Mutex<HashMap<u32, PoolFile>>>,
}

pub struct PoolFile {
    pub status: u8,  // 0=初始, 1=下载中
    pub file_info: ReceivedSimpleFileInfo,
}
```

**我们的实现方案:**

```rust
// src/core/file/download_pool.rs
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::{DateTime, Utc};

#[derive(Clone)]
pub struct DownloadPool {
    tasks: Arc<RwLock<HashMap<u32, DownloadTask>>>,
}

#[derive(Clone, Debug)]
pub struct DownloadTask {
    pub status: TaskStatus,
    pub progress: u64,
    pub total: u64,
    pub file_info: FileInfo,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TaskStatus {
    Pending,
    Downloading,
    Completed,
    Failed(String),
    Cancelled,
}

impl DownloadPool {
    pub fn new() -> Self {
        Self {
            tasks: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// 开始下载
    pub async fn start_download(&self, file_id: u32, file_info: FileInfo) -> Result<(), DownloadError> {
        // 检查是否已存在
        {
            let tasks = self.tasks.read().await;
            if let Some(task) = tasks.get(&file_id) {
                if task.status == TaskStatus::Downloading {
                    return Err(DownloadError::AlreadyDownloading);
                }
            }
        }

        // 创建下载任务
        {
            let mut tasks = self.tasks.write().await;
            tasks.insert(file_id, DownloadTask {
                status: TaskStatus::Downloading,
                progress: 0,
                total: file_info.size,
                file_info,
                started_at: Some(Utc::now()),
                completed_at: None,
            });
        }

        // 启动下载任务
        let pool = self.clone();
        tokio::spawn(async move {
            // TODO: 执行下载逻辑
        });

        Ok(())
    }

    /// 更新下载进度
    pub async fn update_progress(&self, file_id: u32, progress: u64) {
        let mut tasks = self.tasks.write().await;
        if let Some(task) = tasks.get_mut(&file_id) {
            task.progress = progress;
        }
    }

    /// 完成下载
    pub async fn complete_download(&self, file_id: u32) {
        let mut tasks = self.tasks.write().await;
        if let Some(task) = tasks.get_mut(&file_id) {
            task.status = TaskStatus::Completed;
            task.completed_at = Some(Utc::now());
        }
    }

    /// 取消下载
    pub async fn cancel_download(&self, file_id: u32) {
        let mut tasks = self.tasks.write().await;
        if let Some(task) = tasks.get_mut(&file_id) {
            task.status = TaskStatus::Cancelled;
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DownloadError {
    #[error("Already downloading")]
    AlreadyDownloading,
    #[error("Not found")]
    NotFound,
}
```

---

## 三、架构对比

### 3.1 对比分析

| 方面 | ipmsg-rs | 我们的架构 | 建议 |
|------|----------|-----------|------|
| **异步模型** | 同步 I/O + 线程池 | Tokio 异步 I/O | ✅ 保持异步方案 |
| **协议解析** | combine 解析器组合子 | 手动字符串分割 | 🔄 引入 combine |
| **事件驱动** | crossbeam-channel 全局通道 | 未定义 | 🔄 引入事件总线 |
| **编码处理** | 硬编码 GB18030 | 可配置编码 | ✅ 保持可配置方案 |
| **文件传输** | TCP 流式传输 | 待实现 | 🔄 参考设计 |
| **UI 框架** | GTK4 (本地 GUI) | Tauri + React | ✅ 保持 Web 方案 |
| **依赖数量** | 15+ | 核心依赖 5-8 | ✅ 保持轻量级 |

### 3.2 我们的改进方向

1. **引入 combine 解析器** - 提升协议解析健壮性
2. **引入事件总线** - 统一事件管理，解耦模块
3. **参考文件头格式** - 兼容 IPMsg 标准
4. **实现下载池** - 防止重复下载，状态跟踪
5. **保持异步模型** - 相比同步方案更高效

---

## 四、依赖更新建议

### 4.1 新增依赖

```toml
[dependencies]
# 解析器组合子
combine = "4.6"

# 高性能并发通道
crossbeam-channel = "0.5"

# 线程安全的延迟初始化
once_cell = "1.19"

# 字符编码转换（可选）
encoding = "0.2"
```

### 4.2 更新后的完整依赖

```toml
[dependencies]
# 核心运行时
tokio = { version = "1.35", features = ["full"] }

# 数据库
sea-orm = { version = "0.12", features = ["sqlx-sqlite", "runtime-tokio-rustls"] }

# 序列化
serde = { version = "1.0", features = ["derive"] }

# 框架
tauri = { version = "2.0", features = [] }

# 解析器（新增）
combine = "4.6"

# 并发（新增）
crossbeam-channel = "0.5"
once_cell = "1.19"

# 编码（新增 - 可选）
encoding = "0.2"

# 日志
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }

# 错误处理
anyhow = "1.0"
thiserror = "1.0"
```

---

## 五、关键代码提取

以下是从 ipmsg-rs 中提取的关键代码，可作为参考实现：

### 5.1 协议常量

```rust
// src/constants/protocol.rs
pub const IPMSG_DEFAULT_PORT: u16 = 0x0979;  // 2425
pub const IPMSG_LIMITED_BROADCAST: &str = "255.255.255.255";

// 命令字
pub const IPMSG_NOOPERATION: u32 = 0x00000000;
pub const IPMSG_BR_ENTRY: u32 = 0x00000001;
pub const IPMSG_BR_EXIT: u32 = 0x00000002;
pub const IPMSG_ANSENTRY: u32 = 0x00000003;
pub const IPMSG_SENDMSG: u32 = 0x00000020;
pub const IPMSG_RECVMSG: u32 = 0x00000021;
pub const IPMSG_GETFILEDATA: u32 = 0x00000060;
pub const IPMSG_RELEASEFILES: u32 = 0x00000061;

// 选项标志
pub const IPMSG_SENDCHECKOPT: u32 = 0x00000100;
pub const IPMSG_FILEATTACHOPT: u32 = 0x00200000;
pub const IPMSG_UTF8OPT: u32 = 0x00800000;

// 文件属性
pub const IPMSG_FILE_REGULAR: u32 = 0x00000001;
pub const IPMSG_FILE_DIR: u32 = 0x00000002;
```

### 5.2 数据包模型

```rust
// src/models/model.rs
#[derive(Clone, Debug)]
pub struct Packet {
    pub ver: String,
    pub packet_no: String,
    pub sender_name: String,
    pub sender_host: String,
    pub command_no: u32,
    pub additional_section: Option<String>,
    pub ip: String,
}

impl Packet {
    pub fn new(command_no: u32, ext: Option<String>) -> Self {
        Packet {
            ver: "1.0".to_string(),
            packet_no: format!("{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH).unwrap()
                .as_secs()),
            sender_name: hostname::get().unwrap().to_string_lossy().to_string(),
            sender_host: hostname::get().unwrap().to_string_lossy().to_string(),
            command_no,
            additional_section: ext,
            ip: String::new(),
        }
    }
}
```

---

## 六、参考资料

- **仓库**: https://github.com/langzime/ipmsg-rs
- **IPMsg 协议规范**: IP Messenger 官方文档
- **combine 文档**: https://docs.rs/combine/
- **crossbeam 文档**: https://docs.rs/crossbeam/

---

**文档生成时间**: 2025-01-27
**基于版本**: ipmsg-rs v0.7.1
