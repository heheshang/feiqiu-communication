# 飞秋通讯 (Feiqiu Communication)

A LAN instant messaging application built with Tauri 2.0 + Rust + React, implementing the FeiQ/IPMsg protocol for peer-to-peer communication in local networks.

## Features

- **Peer-to-Peer LAN Messaging**: Direct communication between users on the same local network
- **Dual Protocol Support**: Compatible with both IPMsg (standard) and FeiQ (extended) protocols
- **User Discovery**: Automatic detection of online users in the local network
- **Real-time Chat**: Instant text messaging with read receipts
- **File Transfer**: Support for file sharing between users (in progress)
- **Group Chat**: Create and manage group conversations (in progress)
- **Modern UI**: Built with React + TypeScript + Vite
- **Cross-Platform**: Windows, macOS, and Linux support via Tauri

## Tech Stack

### Backend

- **Tauri 2.0**: Desktop application framework
- **Rust**: Systems programming language for performance and safety
- **Tokio**: Async runtime for efficient I/O operations
- **SeaORM**: ORM for database operations
- **SQLite**: Embedded database for local data storage
- **crossbeam-channel**: Lock-free multi-threading channels for event bus

### Frontend

- **React 18**: UI library
- **TypeScript**: Type-safe JavaScript
- **Vite**: Fast build tool and dev server
- **Zustand**: Lightweight state management
- **Less**: CSS preprocessor

### Network Protocol

- **IPMsg**: Standard IP Messenger protocol (port 2425)
- **FeiQ**: Extended protocol with additional features
- **UDP**: User Datagram Protocol for broadcast messaging

## Installation

### Prerequisites

- **Node.js** (v18 or higher) with [Bun](https://bun.sh/) recommended
- **Rust** (latest stable)
- **System dependencies** based on your platform:
  - **Windows**: WebView2 runtime (usually pre-installed)
  - **macOS**: Xcode command line tools
  - **Linux**: webkit2gtk, libayatana-appindicator

### Setup

```bash
# Clone the repository
git clone https://github.com/heheshang/feiqiu-communication.git
cd feiqiu_demo_1

# Install dependencies
bun install

# Run in development mode
bun run tauri dev
```

### Building for Production

```bash
# Build release binary
bun run tauri build

# Output location (Windows)
# src-tauri/target/release/feiqiu-communication.exe
```

## Development

### Project Structure

```
feiqiu_demo_1/
├── src/                    # Frontend source (React + TypeScript)
│   ├── components/         # React components
│   ├── ipc/               # IPC command wrappers
│   ├── stores/            # Zustand state management
│   └── main.tsx           # Frontend entry point
├── src-tauri/             # Backend source (Rust)
│   ├── src/
│   │   ├── core/          # Business logic layer
│   │   │   ├── chat/      # Chat functionality
│   │   │   ├── contact/   # User discovery
│   │   │   ├── file/      # File transfer
│   │   │   └── group/     # Group management
│   │   ├── database/      # Database layer
│   │   │   ├── handler/   # CRUD operations
│   │   │   └── model/     # SeaORM entities
│   │   ├── event/         # Event bus system
│   │   ├── ipc/           # Tauri command handlers
│   │   ├── network/       # Network layer
│   │   │   ├── feiq/      # Protocol implementation
│   │   │   └── udp/       # UDP networking
│   │   ├── utils/         # Utilities
│   │   ├── error.rs       # Error types
│   │   ├── types.rs       # Shared types
│   │   └── main.rs        # Application entry
│   ├── Cargo.toml         # Rust dependencies
│   └── tauri.conf.json    # Tauri configuration
└── package.json           # Node.js dependencies
```

### Common Commands

```bash
# Frontend development
bun run dev              # Start Vite dev server only
bunx tsc --noEmit        # Type check
bun run lint            # Lint TypeScript/JavaScript
bun run lint:css        # Lint styles
bun run lint:all        # Lint everything
bun run format          # Format with Prettier

# Backend development
cd src-tauri
cargo check             # Quick compilation check
cargo test              # Run tests
cargo clippy            # Lint with Clippy
cargo fmt               # Format Rust code

# Run specific tests
cargo test --lib network::feiq::parser
cargo test test_parse_entry_packet
```

## Architecture

The application follows a layered architecture with clear separation of concerns:

```
Frontend (React)
    ↓ IPC (Tauri Commands)
IPC Layer (ipc/*.rs)
    ↓
Business Logic (core/*)
    ↓
Protocol Layer (network/feiq/*)
    ↓
Network Layer (network/udp/*)
    ↓
Database Layer (database/*)

Event Bus (event/bus.rs) - connects all layers asynchronously
```

### Key Design Decisions

1. **Event-Driven Architecture**: Global event bus using `crossbeam-channel` decouples components
2. **Async UDP Networking**: Tokio-based async I/O for efficient network operations
3. **Database-First**: SQLite with SeaORM for reliable local data persistence
4. **Protocol Auto-Detection**: Automatically detects IPMsg vs FeiQ protocol from packet format

## Protocol Details

### IPMsg Protocol Format

```
版本号:命令字:发送者:接收者:消息编号:附加信息
Example: 1.0:32:sender:host:12345:Hello
```

### FeiQ Protocol Format

```
Header: 版本号#长度#MAC地址#端口#标志1#标志2#命令#类型
Data: 时间戳:包ID:主机名:用户ID:内容
Example: 1_lbt6_0#128#5C60BA7361C6#1944#0#0#4001#9:1765442982:T0170006:SHIKUN-SH:6291459:ssk
```

### Key Protocol Constants

- `IPMSG_BR_ENTRY` (0x01) - Broadcast online presence
- `IPMSG_BR_EXIT` (0x02) - Broadcast offline
- `IPMSG_ANSENTRY` (0x03) - Response to entry broadcast
- `IPMSG_SENDMSG` (0x20) - Send message
- `IPMSG_RECVMSG` (0x21) - Message receive acknowledgment
- `IPMSG_UTF8OPT` (0x00800000) - UTF-8 encoding flag
- Default port: 2425

### User Discovery Flow

1. On startup: broadcast `IPMSG_BR_ENTRY` packet via UDP to 255.255.255.255:2425
2. When receiving `IPMSG_BR_ENTRY`: send `IPMSG_ANSENTRY` back and add to online user list
3. When receiving `IPMSG_ANSENTRY`: add to online user list
4. On exit: broadcast `IPMSG_BR_EXIT`

## Development Status

- Phase 1: Project foundation (complete)
- Phase 2: FeiQ protocol implementation (complete)
- Phase 3: Database layer (complete)
- Phase 4: Basic UI (complete)
- Phase 5-8: Advanced features (in progress)

See `docs/Phase*_完成报告.md` for detailed completion reports.

## Configuration

### Application Settings

Configuration is managed through `src-tauri/tauri.conf.json`:

- Application metadata
- Window settings
- Security policies
- Build options

### Environment Variables

No environment variables required for basic operation. The application uses:

- Local IP address auto-detection
- Hostname from system
- Default port 2425 (configurable in constants)

## Troubleshooting

### Common Issues

**Application won't start**

- Ensure WebView2 is installed on Windows
- Check firewall settings for UDP port 2425
- Verify no other FeiQ/IPMsg client is using port 2425

**Users not discovered**

- Check that all devices are on the same local network
- Verify UDP broadcast is not blocked by firewall
- Ensure subnet mask allows broadcast communication

**Build failures**

- Run `cargo check` to identify Rust compilation issues
- Run `bunx tsc --noEmit` to identify TypeScript issues
- Clear node_modules and reinstall: `rm -rf node_modules && bun install`

## Contributing

Contributions are welcome! Please follow these guidelines:

1. **Code Style**

   - Rust: Run `cargo fmt` before committing
   - TypeScript/JavaScript: Run `bun run format` before committing
   - Follow existing code patterns and architecture

2. **Testing**

   - Add unit tests for new functionality
   - Run `cargo test` before committing
   - Test protocol compatibility with existing FeiQ/IPMsg clients

3. **Commits**
   - Use clear commit messages
   - Reference related issues when applicable
   - Follow conventional commit format if possible

## License

MIT License - See LICENSE file for details

## Acknowledgments

- IPMsg protocol by H.Shirouzu
- Tauri framework contributors
- Rust and React communities

## Resources

- [Tauri Documentation](https://tauri.app/)
- [IPMsg Protocol Specification](http://www.ipmsg.org/files/ipmsg_protocol_1.0_en.txt)
- [SeaORM Documentation](https://www.sea-ql.org/SeaORM/)
- [React Documentation](https://react.dev/)
