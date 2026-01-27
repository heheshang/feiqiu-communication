// src-tauri/src/network/feiq/constants.rs
//
/// 飞秋协议常量定义
///
/// 这些常量定义了飞秋/飞鸽传书协议使用的各种命令字和选项标志。
/// 由于这是协议定义的一部分，所有常量都保留为公共 API，即使当前未使用。

/// 默认端口
#[allow(dead_code)]
pub const FEIQ_DEFAULT_PORT: u16 = 2425;

/// 广播地址
#[allow(dead_code)]
pub const FEIQ_BROADCAST_ADDR: &str = "255.255.255.255";

// ============================================================
// 命令字 (低 8 位)
// ============================================================

/// 无操作
#[allow(dead_code)]
pub const IPMSG_NOOPERATION: u32 = 0x00000000;

/// 在线广播
#[allow(dead_code)]
pub const IPMSG_BR_ENTRY: u32 = 0x00000001;

/// 离线广播
#[allow(dead_code)]
pub const IPMSG_BR_EXIT: u32 = 0x00000002;

/// 对 BR_ENTRY 的应答
#[allow(dead_code)]
pub const IPMSG_ANSENTRY: u32 = 0x00000003;

/// 广播缺席
#[allow(dead_code)]
pub const IPMSG_BR_ABSENCE: u32 = 0x00000004;

/// 请求是否需要列表
#[allow(dead_code)]
pub const IPMSG_BR_ISGETLIST: u32 = 0x00000010;

/// 同意发送列表
#[allow(dead_code)]
pub const IPMSG_OKGETLIST: u32 = 0x00000011;

/// 请求用户列表
#[allow(dead_code)]
pub const IPMSG_GETLIST: u32 = 0x00000012;

/// 返回用户列表
#[allow(dead_code)]
pub const IPMSG_ANSLIST: u32 = 0x00000013;

/// 请求扩展列表
#[allow(dead_code)]
pub const IPMSG_BR_ISGETLIST2: u32 = 0x00000018;

/// 发送消息
#[allow(dead_code)]
pub const IPMSG_SENDMSG: u32 = 0x00000020;

/// 接收确认
#[allow(dead_code)]
pub const IPMSG_RECVMSG: u32 = 0x00000021;

/// 消息已读
#[allow(dead_code)]
pub const IPMSG_READMSG: u32 = 0x00000030;

/// 删除消息
#[allow(dead_code)]
pub const IPMSG_DELMSG: u32 = 0x00000031;

/// 对已读的应答
#[allow(dead_code)]
pub const IPMSG_ANSREADMSG: u32 = 0x00000032;

/// 请求用户信息
#[allow(dead_code)]
pub const IPMSG_GETINFO: u32 = 0x00000040;

/// 发送用户信息
#[allow(dead_code)]
pub const IPMSG_SENDINFO: u32 = 0x00000041;

/// 请求文件数据
#[allow(dead_code)]
pub const IPMSG_GETFILEDATA: u32 = 0x00000060;

/// 释放文件资源
#[allow(dead_code)]
pub const IPMSG_RELEASEFILES: u32 = 0x00000061;

/// 请求目录文件列表
#[allow(dead_code)]
pub const IPMSG_GETDIRFILES: u32 = 0x00000062;

// ============================================================
// 选项标志
// ============================================================

/// 缺席标志
#[allow(dead_code)]
pub const IPMSG_ABSENCEOPT: u32 = 0x00000100;

/// 服务器标志
#[allow(dead_code)]
pub const IPMSG_SERVEROPT: u32 = 0x00000200;

/// 拨号连接标志
#[allow(dead_code)]
pub const IPMSG_DIALUPOPT: u32 = 0x00010000;

/// 发送确认（需要 RECVMSG 应答）
#[allow(dead_code)]
pub const IPMSG_SENDCHECKOPT: u32 = 0x00000100;

/// 私密发送（密封消息）
#[allow(dead_code)]
pub const IPMSG_SECRETOPT: u32 = 0x00000200;

/// 广播发送
#[allow(dead_code)]
pub const IPMSG_BROADCASTOPT: u32 = 0x00000400;

/// 多播发送
#[allow(dead_code)]
pub const IPMSG_MULTICASTOPT: u32 = 0x00000800;

/// 不弹窗（接收端）
#[allow(dead_code)]
pub const IPMSG_NOPOPUPOPT: u32 = 0x00001000;

/// 自动回复请求
#[allow(dead_code)]
pub const IPMSG_AUTORETOPT: u32 = 0x00002000;

/// 重试选项
#[allow(dead_code)]
pub const IPMSG_RETRYOPT: u32 = 0x00004000;

/// 带密码发送
#[allow(dead_code)]
pub const IPMSG_PASSWORDOPT: u32 = 0x00008000;

/// 不记录日志
#[allow(dead_code)]
pub const IPMSG_NOLOGOPT: u32 = 0x00020000;

/// 文件附加标志
#[allow(dead_code)]
pub const IPMSG_FILEATTACHOPT: u32 = 0x00200000;

/// 加密标志
#[allow(dead_code)]
pub const IPMSG_ENCRYPTOPT: u32 = 0x00400000;

/// UTF-8 编码标志
#[allow(dead_code)]
pub const IPMSG_UTF8OPT: u32 = 0x00800000;

// ============================================================
// 文件属性
// ============================================================

/// 普通文件
#[allow(dead_code)]
pub const IPMSG_FILE_REGULAR: u32 = 0x00000001;

/// 目录
#[allow(dead_code)]
pub const IPMSG_FILE_DIR: u32 = 0x00000002;

/// 返回父目录标记
#[allow(dead_code)]
pub const IPMSG_FILE_RETPARENT: u32 = 0x00000003;

/// 文件创建时间
#[allow(dead_code)]
pub const IPMSG_FILE_CREATETIME: u32 = 0x00000001;

/// 文件修改时间
#[allow(dead_code)]
pub const IPMSG_FILE_MTIME: u32 = 0x00000002;
