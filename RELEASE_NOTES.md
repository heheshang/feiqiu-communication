# 飞秋通讯 v1.0.0 Release Notes

**发布日期 / Release Date**: 2026-01-31  
**版本 / Version**: 1.0.0  
**状态 / Status**: 🎉 首次正式发布 / First Official Release

---

## 📋 目录 / Table of Contents

- [简介 / Introduction](#简介-introduction)
- [新功能 / New Features](#新功能-new-features)
- [技术架构 / Technical Architecture](#技术架构-technical-architecture)
- [安装说明 / Installation](#安装说明-installation)
- [系统要求 / System Requirements](#系统要求-system-requirements)
- [已知问题 / Known Issues](#已知问题-known-issues)
- [升级说明 / Upgrade Notes](#升级说明-upgrade-notes)
- [致谢 / Acknowledgments](#致谢-acknowledgments)

---

## 简介 / Introduction

飞秋通讯是一款基于局域网（LAN）的开源即时通讯应用，实现了标准的 FeiQ/IPMsg 协议，支持点对点的消息传输、文件传输和群聊功能。

**核心特点 / Key Features:**

- ✅ **无需互联网 / No Internet Required** - 仅需局域网连接 / LAN only
- ✅ **无需服务器 / Serverless** - 点对点通信 / Peer-to-peer
- ✅ **跨平台 / Cross-platform** - Windows、macOS、Linux
- ✅ **开源免费 / Open Source** - MIT 许可证 / MIT License
- ✅ **安全可靠 / Secure** - 数据本地存储 / Local data storage

---

## 新功能 / New Features

### 🔹 即时通讯 / Instant Messaging

**单聊功能 / Private Chat:**

- ✅ 实时文本消息传输 / Real-time text messaging
- ✅ 消息已读回执 / Read receipts
- ✅ 消息历史记录 / Message history
- ✅ 消息分页加载 / Paginated message loading
- ✅ 消息状态显示 / Message status indicators (sent/delivered/read)

**群聊功能 / Group Chat:**

- ✅ 创建群组 / Create groups
- ✅ 添加/移除成员 / Add/remove members
- ✅ 成员角色管理 / Member role management (owner/admin/member)
- ✅ 群组设置 / Group settings (name, description)
- ✅ 群组消息广播 / Group message broadcasting

### 🔹 文件传输 / File Transfer

**核心功能 / Core Features:**

- ✅ 点对点文件传输 / Peer-to-peer file transfer
- ✅ 支持大文件 / Large file support
- ✅ 断点续传 / Resumable transfers
- ✅ 传输进度显示 / Transfer progress display
- ✅ 并发传输管理 / Concurrent transfer management
- ✅ 单聊和群聊文件传输 / Private and group file transfers

### 🔹 用户发现 / User Discovery

**自动发现 / Auto-discovery:**

- ✅ UDP 广播发现 / UDP broadcast discovery
- ✅ 在线状态同步 / Online status synchronization
- ✅ 协议兼容 / Protocol compatible (FeiQ + IPMsg)
- ✅ 无需手动添加联系人 / No manual contact addition required

### 🔹 数据持久化 / Data Persistence

**数据库功能 / Database Features:**

- ✅ SQLite 数据库 / SQLite database
- ✅ 聊天记录持久化 / Chat history persistence
- ✅ 联系人管理 / Contact management
- ✅ 文件传输历史 / File transfer history
- ✅ 群组信息存储 / Group information storage

### 🔹 用户界面 / User Interface

**UI 特性 / UI Features:**

- ✅ 现代化设计 / Modern design (仿微信风格 / WeChat-style)
- ✅ 三栏布局 / Three-column layout
- ✅ 响应式设计 / Responsive design
- ✅ Emoji 支持 / Emoji support
- ✅ 消息搜索 / Message search (planned)
- ✅ 主题切换 / Theme switching (planned)

---

## 技术架构 / Technical Architecture

### 技术栈 / Tech Stack

**后端 / Backend:**

- **框架 / Framework**: Tauri 2.0
- **语言 / Language**: Rust (stable)
- **异步运行时 / Async Runtime**: Tokio
- **数据库 / Database**: SQLite + SeaORM
- **网络 / Network**: UDP (FeiQ/IPMsg protocol)

**前端 / Frontend:**

- **框架 / Framework**: React 18
- **语言 / Language**: TypeScript
- **构建工具 / Build Tool**: Vite
- **状态管理 / State Management**: Zustand
- **样式 / Styling**: Less + CSS Modules

### 架构亮点 / Architecture Highlights

**事件驱动架构 / Event-Driven Architecture:**

- 全局事件总线 / Global event bus (crossbeam-channel)
- 解耦组件 / Decoupled components
- 异步消息传递 / Async message passing

**三层错误处理 / Three-Layer Error Handling:**

- Service Layer → IPC Layer → Frontend
- 用户友好的错误消息 / User-friendly error messages
- 结构化错误码 / Structured error codes

**协议自动检测 / Protocol Auto-detection:**

- 支持 FeiQ 和 IPMsg 协议 / Supports both FeiQ and IPMsg
- 自动识别协议类型 / Automatic protocol detection
- 向后兼容 / Backward compatible

---

## 安装说明 / Installation

### macOS

**方法 1: DMG 安装器 / DMG Installer (推荐 / Recommended)**

```bash
# 1. 下载 DMG 文件 / Download DMG file
# 2. 打开 DMG / Open DMG
# 3. 拖拽"飞秋通讯.app"到应用程序文件夹
#    Drag "飞秋通讯.app" to Applications
# 4. 从启动台启动 / Launch from Launchpad
```

**方法 2: 直接运行 / Run Directly**

```bash
# 下载并解压 / Download and extract
# 双击"飞秋通讯.app"运行 / Double-click to run
```

**首次启动注意 / First Launch Note:**

- macOS 可能显示安全警告 / macOS may show security warning
- 解决方法 / Solution: 右键点击 → 打开 / Right-click → Open
- 或者在系统设置中允许 / Or allow in System Preferences

### Windows

**要求 / Requirements:**

- Windows 10 或更高版本 / Windows 10 or higher
- WebView2 运行时 / WebView2 Runtime (通常预安装 / usually pre-installed)

**安装 / Installation:**

```bash
# 1. 下载 .exe 或 .msi 安装包
# 2. 运行安装程序 / Run installer
# 3. 按照提示完成安装 / Follow prompts to complete
```

### Linux

**支持发行版 / Supported Distributions:**

- Ubuntu 20.04+ / Debian 11+
- Fedora 33+
- Arch Linux

**安装 / Installation:**

```bash
# Ubuntu/Debian (.deb)
sudo dpkg -i feiqiu-communication_1.0.0_amd64.deb

# Fedora (.rpm)
sudo rpm -i feiqiu-communication-1.0.0-1.x86_64.rpm

# AppImage (通用发行版 / Universal)
chmod +x 飞秋通讯_1.0.0_amd64.AppImage
./飞秋通讯_1.0.0_amd64.AppImage
```

---

## 系统要求 / System Requirements

### 最低要求 / Minimum Requirements

| 组件 / Component   | Windows                           | macOS         | Linux         |
| ------------------ | --------------------------------- | ------------- | ------------- |
| **操作系统 / OS**  | Windows 10+                       | macOS 10.13+  | Ubuntu 20.04+ |
| **架构 / Arch**    | x64                               | x64 / ARM64\* | x64           |
| **内存 / RAM**     | 4 GB                              | 4 GB          | 4 GB          |
| **磁盘 / Disk**    | 100 MB                            | 100 MB        | 100 MB        |
| **网络 / Network** | 以太网或Wi-Fi / Ethernet or Wi-Fi | 以太网或Wi-Fi | 以太网或Wi-Fi |

\*Apple Silicon 通过 Rosetta 2 运行 / Runs via Rosetta 2 on Apple Silicon

### 推荐配置 / Recommended Configuration

| 组件 / Component   | 推荐值 / Recommended             |
| ------------------ | -------------------------------- |
| **内存 / RAM**     | 8 GB 或更多 / 8 GB or more       |
| **网络 / Network** | 有线网络 / Wired network         |
| **处理器 / CPU**   | 双核或更高 / Dual-core or higher |

---

## 已知问题 / Known Issues

### 🐛 当前版本问题 / Current Version Issues

**1. 代码签名 / Code Signing**

- **问题 / Issue**: macOS 显示"无法验证开发者"警告 / "Unidentified developer" warning
- **原因 / Cause**: 使用 ad-hoc 签名 / Uses ad-hoc signature
- **解决方案 / Solution**:
  - 右键点击 → 打开 / Right-click → Open
  - 或在系统设置中允许 / Or allow in System Preferences
  - 未来版本将使用正式签名 / Future versions will use proper signing

**2. 防火墙提示 / Firewall Prompts**

- **问题 / Issue**: 首次运行时防火墙可能提示 / Firewall may prompt on first run
- **解决方案 / Solution**: 允许应用通过防火墙 / Allow app through firewall
- **端口 / Port**: UDP 2425 (FeiQ/IPMsg 协议端口 / protocol port)

**3. 仅支持局域网 / LAN Only**

- **限制 / Limitation**: 无法跨互联网通信 / Cannot communicate over internet
- **原因 / Cause**: 设计用于局域网 / Designed for LAN
- **解决方案 / Solution**: 使用 VPN 连接不同网络 / Use VPN to connect different networks

**4. 编译器警告 / Compiler Warnings**

- **问题 / Issue**: 46 个编译器警告 / 46 compiler warnings
- **类型 / Type**: 死代码警告 / Dead code warnings
- **影响 / Impact**: 不影响功能 / Does not affect functionality
- **计划 / Plan**: 未来版本清理 / Will clean up in future versions

### 🔧 即将修复 / To Be Fixed

- [ ] 消息搜索功能 / Message search (Phase 10)
- [ ] 文件传输历史界面 / File transfer history UI (Phase 10)
- [ ] 主题切换 / Theme switching (Phase 10)
- [ ] 多语言支持 / Multi-language support (Phase 11)
- [ ] 代码签名 / Code signing (Phase 9.3)
- [ ] 自动更新 / Auto-update (Phase 12)

---

## 升级说明 / Upgrade Notes

### 从开发版本升级 / Upgrading from Development Versions

**数据库迁移 / Database Migration:**

- ✅ 自动迁移 / Automatic migration
- ✅ 无需手动操作 / No manual intervention required
- ✅ 数据保留 / Data preserved

**配置文件 / Configuration:**

- ✅ 向后兼容 / Backward compatible
- ✅ 无需重新配置 / No reconfiguration needed

### 从 IPMsg/FeiQ 客户端迁移 / Migrating from IPMsg/FeiQ Clients

**兼容性 / Compatibility:**

- ✅ 协议兼容 / Protocol compatible
- ✅ 可与现有客户端通信 / Can communicate with existing clients
- ✅ 自动发现其他用户 / Auto-discover other users

**数据导入 / Data Import:**

- ⚠️ 暂不支持历史记录导入 / History import not yet supported
- 📝 计划在 v1.1 添加 / Planned for v1.1

---

## 测试状态 / Testing Status

### 已测试平台 / Tested Platforms

| 平台 / Platform              | 状态 / Status       | 测试结果 / Test Results |
| ---------------------------- | ------------------- | ----------------------- |
| **macOS 14** (Intel)         | ✅ 已测试 / Tested  | 全部通过 / All passed   |
| **macOS 14** (Apple Silicon) | ⏳ 待测试 / Pending | -                       |
| **Windows 10/11**            | ⏳ 待测试 / Pending | -                       |
| **Ubuntu 22.04**             | ⏳ 待测试 / Pending | -                       |

### 测试覆盖 / Test Coverage

**单元测试 / Unit Tests:**

- ✅ 协议解析器 / Protocol parser (19/19 passed)
- ✅ 数据库操作 / Database operations (64/64 passed)
- ✅ 总计 / Total: 83/83 tests passed (100%)

**集成测试 / Integration Tests:**

- ✅ 网络通信 / Network communication
- ✅ 文件传输 / File transfer
- ✅ 群组管理 / Group management

**手动测试 / Manual Testing (macOS):**

- ✅ 应用启动 / Application launch
- ✅ 网络绑定 / Network binding
- ✅ 数据库初始化 / Database initialization
- ✅ UI 响应 / UI responsiveness

---

## 性能指标 / Performance Metrics

### macOS 平台 / macOS Platform

| 指标 / Metric               | 数值 / Value  | 状态 / Status       |
| --------------------------- | ------------- | ------------------- |
| **应用大小 / App Size**     | 9.3 MB        | ✅ 优秀 / Excellent |
| **内存占用 / Memory Usage** | ~55 MB (idle) | ✅ 优秀 / Excellent |
| **CPU 占用 / CPU Usage**    | ~0% (idle)    | ✅ 优秀 / Excellent |
| **启动时间 / Launch Time**  | < 3 seconds   | ✅ 优秀 / Excellent |
| **数据库初始化 / DB Init**  | < 1 second    | ✅ 优秀 / Excellent |

---

## 开发者信息 / For Developers

### 构建从源码 / Building from Source

**前置要求 / Prerequisites:**

```bash
# Rust (stable)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Bun (推荐 / Recommended)
curl -fsSL https://bun.sh/install | bash

# 或 Node.js / Or Node.js
# 下载并安装 / Download and install from nodejs.org
```

**构建步骤 / Build Steps:**

```bash
# 克隆仓库 / Clone repository
git clone https://github.com/heheshang/feiqiu-communication.git
cd feiqiu-communication

# 安装依赖 / Install dependencies
bun install

# 开发模式 / Development mode
bun run tauri dev

# 生产构建 / Production build
bun run tauri build
```

### 运行测试 / Running Tests

```bash
# Rust 单元测试 / Rust unit tests
cd src-tauri
cargo test

# 前端测试 / Frontend tests
bun test

# 类型检查 / Type checking
bunx tsc --noEmit
```

---

## 文档 / Documentation

- 📖 [用户指南 (User Guide)](USER_GUIDE.md)
- 🔧 [故障排除 (Troubleshooting)](TROUBLESHOOTING.md)
- ❓ [常见问题 (FAQ)](FAQ.md)
- 📝 [实施计划 (Implementation Plan)](IMPLEMENTATION_PLAN.md)
- 📊 [阶段完成报告 (Phase Reports)](docs/Phase*_完成报告.md)

---

## 路线图 / Roadmap

### v1.1 (计划中 / Planned - Q2 2026)

- [ ] 消息搜索功能 / Message search
- [ ] 文件传输历史界面 / File transfer history UI
- [ ] 主题切换 / Theme switching
- [ ] 代码签名 / Code signing
- [ ] 性能优化 / Performance improvements

### v1.2 (计划中 / Planned - Q3 2026)

- [ ] 多语言支持 / Multi-language support (i18n)
- [ ] 自动更新功能 / Auto-update
- [ ] 离线消息 / Offline messages
- [ ] 消息加密 / Message encryption (E2EE)

### v2.0 (计划中 / Planned - Q4 2026)

- [ ] 语音通话 / Voice calls
- [ ] 视频通话 / Video calls
- [ ] 屏幕共享 / Screen sharing
- [ ] 插件系统 / Plugin system

---

## 贡献指南 / Contributing

我们欢迎所有形式的贡献！/ We welcome all forms of contributions!

**如何贡献 / How to Contribute:**

1. Fork 项目 / Fork the project
2. 创建特性分支 / Create feature branch
3. 提交更改 / Commit changes
4. 推送到分支 / Push to branch
5. 创建 Pull Request / Create Pull Request

**贡献指南 / Contribution Guidelines:**

- 遵循代码风格 / Follow code style
- 添加测试 / Add tests
- 更新文档 / Update documentation
- 编写清晰的提交信息 / Write clear commit messages

详见 / See: [CONTRIBUTING.md](CONTRIBUTING.md) (待添加 / To be added)

---

## 许可证 / License

MIT License

Copyright (c) 2026 feiqiu-communication

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

---

## 致谢 / Acknowledgments

### 核心技术 / Core Technologies

感谢以下开源项目 / Thanks to these open source projects:

- [Tauri](https://tauri.app/) - 跨平台桌面应用框架 / Cross-platform desktop framework
- [React](https://react.dev/) - UI 库 / UI library
- [Rust](https://www.rust-lang.org/) - 系统编程语言 / Systems programming language
- [SeaORM](https://www.sea-ql.org/SeaORM/) - ORM 框架 / ORM framework
- [Vite](https://vitejs.dev/) - 构建工具 / Build tool

### 协议 / Protocol

- [IPMsg](http://www.ipmsg.org/) - IP Messenger 协议 / Protocol by H.Shirouzu
- [FeiQ](https://www.feiq.cn/) - 飞秋协议 / FeiQ protocol

### 特别感谢 / Special Thanks

- 所有贡献者 / All contributors
- 测试用户 / Beta testers
- 开源社区 / Open source community

---

## 联系方式 / Contact

- **项目主页 / Project**: https://github.com/heheshang/feiqiu-communication
- **问题反馈 / Issues**: https://github.com/heheshang/feiqiu-communication/issues
- **讨论 / Discussions**: https://github.com/heheshang/feiqiu-communication/discussions

---

## 下载 / Download

**最新版本 / Latest Version**: v1.0.0

**macOS:**

- [DMG 安装器 (34 MB)](../../releases/飞秋通讯_1.0.0_x64.dmg)
- [App Bundle (9.3 MB)](../../releases/飞秋通讯.app)

**Windows / Linux:**

- 即将推出 / Coming soon

---

**下载统计 / Downloads:**

- ⬇️ macOS DMG: [飞秋通讯\_1.0.0_x64.dmg](../../releases/飞秋通讯_1.0.0_x64.dmg)
- ⬇️ macOS App: [飞秋通讯.app](../../releases/飞秋通讯.app)

---

**祝您使用愉快！/ Enjoy! 🎉**

**飞秋通讯开发团队 / Feiqiu Communication Development Team**  
_2026-01-31_
