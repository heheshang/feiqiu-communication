// src/constants/feiq_protocol.rs
///
/// 飞秋（IPMsg）协议常量定义
///
/// 参考: langzime/ipmsg-rs
/// 协议规范: IP Messenger Official Documentation
///

// ============================================================
// 网络配置
// ============================================================

/// UDP 通信端口（默认 2425）
pub const FEIQ_DEFAULT_PORT: u16 = 0x0979;

/// 局域网广播地址
pub const FEIQ_BROADCAST_ADDR: &str = "255.255.255.255";

/// 协议版本号
pub const FEIQ_PROTOCOL_VERSION: &str = "1.0";

// ============================================================
// 命令字定义（Command - mode 低 8 位）
// ============================================================

// ---------- 基础通信 ----------

/// 无操作
pub const IPMSG_NOOPERATION: u32 = 0x00000000;

/// 广播上线
pub const IPMSG_BR_ENTRY: u32 = 0x00000001;

/// 广播下线
pub const IPMSG_BR_EXIT: u32 = 0x00000002;

/// 对 BR_ENTRY 的应答
pub const IPMSG_ANSENTRY: u32 = 0x00000003;

/// 广播缺席
pub const IPMSG_BR_ABSENCE: u32 = 0x00000004;

// ---------- 用户列表 ----------

/// 请求是否需要列表
pub const IPMSG_BR_ISGETLIST: u32 = 0x00000010;

/// 同意发送列表
pub const IPMSG_OKGETLIST: u32 = 0x00000011;

/// 请求用户列表
pub const IPMSG_GETLIST: u32 = 0x00000012;

/// 返回用户列表
pub const IPMSG_ANSLIST: u32 = 0x00000013;

/// 请求扩展列表
pub const IPMSG_BR_ISGETLIST2: u32 = 0x00000018;

// ---------- 消息相关 ----------

/// 发送消息
pub const IPMSG_SENDMSG: u32 = 0x00000020;

/// 接收确认
pub const IPMSG_RECVMSG: u32 = 0x00000021;

/// 消息已读
pub const IPMSG_READMSG: u32 = 0x00000030;

/// 删除消息
pub const IPMSG_DELMSG: u32 = 0x00000031;

/// 对已读的应答
pub const IPMSG_ANSREADMSG: u32 = 0x00000032;

// ---------- 用户信息 ----------

/// 请求用户信息
pub const IPMSG_GETINFO: u32 = 0x00000040;

/// 发送用户信息
pub const IPMSG_SENDINFO: u32 = 0x00000041;

// ---------- 缺席信息 ----------

/// 请求缺席信息
pub const IPMSG_GETABSENCEINFO: u32 = 0x00000050;

/// 发送缺席信息
pub const IPMSG_SENDABSENCEINFO: u32 = 0x00000051;

// ---------- 文件传输 ----------

/// 请求文件数据
pub const IPMSG_GETFILEDATA: u32 = 0x00000060;

/// 释放文件资源
pub const IPMSG_RELEASEFILES: u32 = 0x00000061;

/// 请求目录文件列表
pub const IPMSG_GETDIRFILES: u32 = 0x00000062;

// ---------- 加密通信 ----------

/// 请求公钥
pub const IPMSG_GETPUBKEY: u32 = 0x00000072;

/// 应答公钥
pub const IPMSG_ANSPUBKEY: u32 = 0x00000073;

// ============================================================
// 选项标志（Option/Flags）
// ============================================================

// ---------- 通用/全局标志 ----------

/// 缺席标志
pub const IPMSG_ABSENCEOPT: u32 = 0x00000100;

/// 服务器标志
pub const IPMSG_SERVEROPT: u32 = 0x00000200;

/// 拨号连接标志
pub const IPMSG_DIALUPOPT: u32 = 0x00010000;

/// 文件附加标志
pub const IPMSG_FILEATTACHOPT: u32 = 0x00200000;

/// 加密标志
pub const IPMSG_ENCRYPTOPT: u32 = 0x00400000;

/// UTF-8 编码标志
pub const IPMSG_UTF8OPT: u32 = 0x00800000;

// ---------- 发送命令特有标志 ----------

/// 发送确认（需要 RECVMSG 应答）
pub const IPMSG_SENDCHECKOPT: u32 = 0x00000100;

/// 私密发送（密封消息）
pub const IPMSG_SECRETOPT: u32 = 0x00000200;

/// 广播发送
pub const IPMSG_BROADCASTOPT: u32 = 0x00000400;

/// 多播发送
pub const IPMSG_MULTICASTOPT: u32 = 0x00000800;

/// 不弹窗（接收端）
pub const IPMSG_NOPOPUPOPT: u32 = 0x00001000;

/// 自动回复请求
pub const IPMSG_AUTORETOPT: u32 = 0x00002000;

/// 重试选项
pub const IPMSG_RETRYOPT: u32 = 0x00004000;

/// 带密码发送
pub const IPMSG_PASSWORDOPT: u32 = 0x00008000;

/// 不记录日志
pub const IPMSG_NOLOGOPT: u32 = 0x00020000;

// ============================================================
// 文件属性常量
// ============================================================

/// 普通文件
pub const IPMSG_FILE_REGULAR: u32 = 0x00000001;

/// 目录
pub const IPMSG_FILE_DIR: u32 = 0x00000002;

/// 返回父目录标记
pub const IPMSG_FILE_RETPARENT: u32 = 0x00000003;

/// 符号链接
pub const IPMSG_FILE_SYMLINK: u32 = 0x00000004;

// ============================================================
// 文件时间戳类型
// ============================================================

/// 创建时间
pub const IPMSG_FILE_CREATETIME: u32 = 0x00000001;

/// 修改时间
pub const IPMSG_FILE_MTIME: u32 = 0x00000002;

/// 访问时间
pub const IPMSG_FILE_ATIME: u32 = 0x00000004;

// ============================================================
// 辅助函数
// ============================================================

/// 提取命令模式（低 8 位）
#[inline]
pub const fn get_mode(command: u32) -> u32 {
    command & 0x000000ff
}

/// 提取选项标志（高 24 位）
#[inline]
pub const fn get_opt(command: u32) -> u32 {
    command & 0xffffff00
}

/// 构建完整命令字
#[inline]
pub const fn build_command(mode: u32, opt: u32) -> u32 {
    (mode & 0x000000ff) | (opt & 0xffffff00)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_parsing() {
        let cmd = IPMSG_SENDMSG | IPMSG_SENDCHECKOPT | IPMSG_UTF8OPT;
        assert_eq!(get_mode(cmd), IPMSG_SENDMSG);
        assert_eq!(get_opt(cmd), IPMSG_SENDCHECKOPT | IPMSG_UTF8OPT);
    }

    #[test]
    fn test_build_command() {
        let cmd = build_command(IPMSG_SENDMSG, IPMSG_UTF8OPT);
        assert_eq!(cmd, IPMSG_SENDMSG | IPMSG_UTF8OPT);
    }

    #[test]
    fn test_command_combinations() {
        // 发送需要确认的 UTF-8 消息
        let cmd = build_command(IPMSG_SENDMSG, IPMSG_SENDCHECKOPT | IPMSG_UTF8OPT);
        assert_eq!(get_mode(cmd), IPMSG_SENDMSG);
        assert!(get_opt(cmd) & IPMSG_SENDCHECKOPT != 0);
        assert!(get_opt(cmd) & IPMSG_UTF8OPT != 0);
    }
}
