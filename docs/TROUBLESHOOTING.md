# 故障排除指南 (Troubleshooting Guide)

本指南帮助您解决使用飞秋通讯时遇到的常见问题。

## 目录 (Table of Contents)

- [安装问题](#安装问题-installation-issues)
- [网络连接问题](#网络连接问题-network-connectivity-issues)
- [文件传输问题](#文件传输问题-file-transfer-issues)
- [性能问题](#性能问题-performance-issues)
- [崩溃和错误](#崩溃和错误-crashes-and-errors)
- [日志文件位置](#日志文件位置-log-file-locations)

---

## 安装问题 (Installation Issues)

### Windows: 应用无法启动

**症状：** 双击应用程序图标后没有反应

**可能原因和解决方案：**

#### 原因 1: 缺少 WebView2 运行时

飞秋通讯基于 Tauri 框架，需要 Microsoft WebView2 运行时。

**解决方案：**

1. 访问 [Microsoft WebView2 下载页面](https://developer.microsoft.com/en-us/microsoft-edge/webview2/)
2. 下载并安装 WebView2 运行时
3. 重启计算机
4. 重新启动飞秋通讯

#### 原因 2: 防病毒软件阻止

某些防病毒软件可能会阻止应用程序运行。

**解决方案：**

1. 打开防病毒软件设置
2. 将飞秋通讯添加到白名单
3. 或临时禁用防病毒软件进行测试

#### 原因 3: 权限不足

**解决方案：**

1. 右键点击应用程序图标
2. 选择"以管理员身份运行"
3. 如果能正常运行，建议将应用程序安装到用户目录而非 Program Files

### macOS: "应用已损坏，无法打开"

**症状：** 打开应用时提示"已损坏"

**解决方案：**

```bash
# 移除应用的隔离属性
xattr -cr /Applications/飞秋通讯.app
```

然后重新尝试打开应用。

### Linux: 缺少依赖库

**症状：** 启动时提示缺少共享库

**解决方案：**

**Ubuntu/Debian:**

```bash
sudo apt update
sudo apt install libwebkit2gtk-4.0-dev libayatana-appindicator3-dev
```

**Fedora:**

```bash
sudo dnf install webkit2gtk3 libappindicator-gtk3
```

**Arch Linux:**

```bash
sudo pacman -S webkit2gtk libappindicator-gtk3
```

---

## 网络连接问题 (Network Connectivity Issues)

### 无法发现其他用户

**症状：** 联系人列表为空，无法看到其他用户

#### 检查 1: 确认在同一局域网

飞秋通讯只能发现同一局域网内的用户。

**验证方法：**

- 打开命令提示符/终端
- 输入 `ping [对方IP地址]`
- 如果能 ping 通，说明在同一网络

#### 检查 2: 防火墙设置

防火墙可能阻止 UDP 广播。

**Windows 防火墙设置：**

1. 打开"Windows Defender 防火墙" → "允许应用通过防火墙"
2. 点击"更改设置"按钮（需要管理员权限）
3. 勾选"飞秋通讯"的"专用"和"公用"网络
4. 如果应用不在列表中，点击"允许其他应用"并添加

**或者使用命令行（以管理员身份运行 PowerShell）：**

```powershell
New-NetFirewallRule -DisplayName "Feiqiu UDP" -Direction Inbound -Protocol UDP -LocalPort 2425 -Action Allow
```

**macOS 防火墙设置：**

1. 打开"系统偏好设置" → "安全性与隐私" → "防火墙"
2. 点击"防火墙选项"
3. 添加飞秋通讯并勾选"允许传入连接"

**Linux 防火墙设置：**

```bash
# Ubuntu/Debian (ufw)
sudo ufw allow 2425/udp

# Fedora/RHEL (firewalld)
sudo firewall-cmd --add-port=2425/udp --permanent
sudo firewall-cmd --reload

# 临时禁用防火墙测试（不推荐）
sudo ufw disable
```

#### 检查 3: 端口被占用

**症状：** 启动时提示端口 2425 已被占用

**解决方案：**

**Windows:**

```powershell
# 查找占用端口的进程
netstat -ano | findstr :2425

# 结束进程（PID 为上一步查找到的进程 ID）
taskkill /PID [进程ID] /F
```

**macOS/Linux:**

```bash
# 查找占用端口的进程
lsof -i :2425

# 结束进程
kill -9 [进程ID]
```

### 无法发送或接收消息

**症状：** 可以看到其他用户，但无法发送或接收消息

#### 可能原因 1: 子网掩码设置

**解决方案：**

- 确保所有设备的子网掩码设置正确
- 通常应为 255.255.255.0
- 在网络设置中检查子网掩码配置

#### 可能原因 2: VPN 或虚拟网卡

**解决方案：**

- 临时断开 VPN 连接
- 禁用虚拟网卡（如 VMware、VirtualBox 创建的虚拟网卡）
- 重启飞秋通讯

#### 可能原因 3: IP 地址冲突

**解决方案：**

```bash
# 检查 IP 地址冲突
# Windows
ipconfig /all

# macOS/Linux
ifconfig
```

如果多台设备使用相同 IP 地址，会导致通信问题。请确保每台设备有唯一的 IP 地址。

---

## 文件传输问题 (File Transfer Issues)

### 文件传输失败

**症状：** 发送或接收文件时失败

#### 原因 1: 防火墙阻止文件传输端口

**解决方案：**
文件传输使用随机端口，需要允许飞秋通讯访问所有端口：

**Windows:**

1. 在防火墙设置中完全允许飞秋通讯
2. 或临时禁用防火墙进行测试

**macOS/Linux:**

```bash
# 完全允许应用（不推荐用于生产环境）
# 仅用于测试是否为防火墙问题
```

#### 原因 2: 磁盘空间不足

**解决方案：**

1. 检查目标磁盘剩余空间
2. 清理不需要的文件
3. 选择其他磁盘位置保存文件

#### 原因 3: 文件路径过长

**解决方案：**

- Windows 路径长度限制为 260 字符
- 尝试将文件保存在路径较短的位置
- 或使用 \\?\ 前缀绕过限制（仅限 Windows）

### 断点续传失败

**症状：** 无法恢复未完成的传输

**解决方案：**

1. **检查传输状态**
   - 在传输列表中查看传输状态
   - 只有"暂停"或"失败"的传输才能恢复

2. **验证文件存在**
   - 确保源文件（发送方）仍然存在
   - 确保部分下载的文件（接收方）未删除

3. **重新发送文件**
   - 如果断点续传仍然失败，取消传输
   - 重新发送文件

### 文件传输速度慢

**症状：** 传输速度远低于网络带宽

**可能原因和解决方案：**

#### 原因 1: 无线网络信号弱

**解决方案：**

- 靠近无线路由器
- 或使用有线网络连接

#### 原因 2: 网络拥塞

**解决方案：**

- 避免在网络高峰期传输大文件
- 暂停其他网络活动（如视频流、下载）

#### 原因 3: 磁盘性能

**解决方案：**

- 使用 SSD 而非 HDD
- 关闭不必要的后台程序
- 在传输任务管理器中降低并发传输数

---

## 性能问题 (Performance Issues)

### 应用运行缓慢

**症状：** 界面卡顿、响应慢

#### 解决方案 1: 清理聊天记录

大量聊天记录可能导致性能下降。

**操作步骤：**

1. 打开"设置" → "数据管理"
2. 选择"清理聊天记录"
3. 选择清理范围（如 30 天前）
4. 点击"清理"按钮

#### 解决方案 2: 减少群组数量

群组会增加消息处理负担。

**操作步骤：**

1. 退出不需要的群组
2. 关闭群组消息通知

#### 解决方案 3: 调整消息分页大小

**操作步骤：**

1. 打开"设置" → "界面设置"
2. 调整"每次加载消息数"
3. 建议设置为 50-100 条

### 高 CPU 或内存占用

**症状：** 任务管理器显示 CPU 或内存使用率过高

**解决方案：**

1. **重启应用**
   - 关闭并重新启动飞秋通讯
   - 清除内存缓存

2. **关闭不必要的功能**
   - 关闭自动消息同步
   - 减少文件传输并发数

3. **检查数据库大小**
   - 查看应用数据目录大小
   - 如果超过 1GB，考虑清理历史记录

---

## 崩溃和错误 (Crashes and Errors)

### 应用意外崩溃

**症状：** 应用突然关闭或无响应

#### 收集崩溃信息

**Windows:**

1. 打开"事件查看器"
2. 导航到"Windows 日志" → "应用程序"
3. 查找飞秋通讯的错误事件
4. 记录错误代码和消息

**macOS:**

1. 打开"控制台"应用
2. 在搜索框中输入"feiqiu"
3. 查找崩溃报告
4. 记录崩溃信息

**Linux:**

```bash
# 查看系统日志
journalctl -xe | grep feiqiu

# 或查看应用日志
cat ~/.config/feiqiu-communication/logs/*.log
```

#### 常见崩溃原因

**原因 1: 数据库损坏**

**解决方案：**

```bash
# 备份数据库
cp ~/.config/feiqiu-communication/feiqiu.db ~/.config/feiqiu-communication/feiqiu.db.backup

# 删除损坏的数据库（会丢失所有聊天记录）
rm ~/.config/feiqiu-communication/feiqiu.db

# 重启应用，会自动创建新数据库
```

**原因 2: 网络驱动问题**

**解决方案：**

1. 更新网卡驱动程序
2. 或尝试使用不同的网络连接

### 错误消息: "数据库操作失败"

**症状：** 操作时提示数据库错误

**解决方案：**

1. **检查磁盘空间**
   - 确保系统盘有足够的可用空间（至少 100MB）

2. **检查文件权限**
   - 确保应用数据目录有写入权限
   - Linux/macOS: `chmod +w ~/.config/feiqiu-communication/`

3. **重建数据库**
   - 如上所述，删除并重建数据库

---

## 日志文件位置 (Log File Locations)

查看日志文件可以帮助诊断问题。

### Windows

**日志位置：**

```
%APPDATA%\feiqiu-communication\logs\
```

**访问方法：**

1. 按 `Win + R` 打开运行对话框
2. 输入 `%APPDATA%\feiqiu-communication\logs\`
3. 按回车键

**最新日志文件：**

- `feiqiu-communication.log` - 当前日志
- `feiqiu-communication.log.1` - 旧日志（如果存在）

### macOS

**日志位置：**

```
~/Library/Logs/feiqiu-communication/
```

**访问方法：**

```bash
# 在终端中打开日志目录
open ~/Library/Logs/feiqiu-communication/

# 或查看最新日志
tail -f ~/Library/Logs/feiqiu-communication/feiqiu-communication.log
```

### Linux

**日志位置：**

```
~/.local/state/feiqiu-communication/logs/
```

**访问方法：**

```bash
# 查看最新日志
tail -f ~/.local/state/feiqiu-communication/logs/feiqiu-communication.log

# 或查看所有日志文件
ls -lh ~/.local/state/feiqiu-communication/logs/
```

### 提交日志报告

如果问题仍未解决，您可以：

1. **复制日志文件**
   - 压缩整个 logs 目录

2. **创建问题报告**
   - 访问: https://github.com/heheshang/feiqiu-communication/issues
   - 描述问题
   - 附上日志文件（注意删除敏感信息）

3. **包含系统信息**
   - 操作系统版本
   - 应用版本
   - 错误消息截图

---

## 获取帮助 (Getting Help)

如果您的问题未在本文档中解决：

1. **查看 [常见问题解答](FAQ.md)**
2. **阅读 [用户指南](USER_GUIDE.md)**
3. **提交问题报告** - https://github.com/heheshang/feiqiu-communication/issues
4. **联系技术支持** - feiqiu-communication@example.com

---

**版本信息：** v1.0.0
**最后更新：** 2026-01-30
