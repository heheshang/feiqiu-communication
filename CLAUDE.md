# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**飞秋通讯 (Feiqiu Communication)** - A LAN instant messaging application built with Tauri 2.0 + Rust + React, implementing the FeiQ/IPMsg protocol for peer-to-peer communication in local networks.

- **Repository**: https://github.com/heheshang/feiqiu-communication
- **Tech Stack**: Tauri 2.0, Rust (backend), React + TypeScript + Vite (frontend), SQLite + SeaORM (database)
- **Implementation Plan**: See `IMPLEMENTATION_PLAN.md` for complete phase breakdown

## Common Development Commands

### Building and Running

```bash
# Development mode (starts both frontend dev server and Tauri)
npm run tauri dev

# Production build
npm run tauri build

# Build output location (Windows)
src-tauri/target/release/feiqiu-communication.exe
```

### Rust Backend (src-tauri/)

```bash
# Navigate to Rust project
cd src-tauri

# Check compilation (faster than full build)
cargo check

# Run tests
cargo test

# Run specific test module
cargo test --lib network::feiq

# Run specific test
cargo test test_parse_entry_packet

# Format code
cargo fmt

# Lint with clippy
cargo clippy

# Build only the Rust binary
cargo build
```

### Frontend (root/)

```bash
# Install dependencies
npm install

# Start frontend dev server only
npm run dev

# Type check
npx tsc --noEmit

# Lint
npm run lint
```

## Architecture Overview

The codebase follows a layered architecture with clear separation of concerns:

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

### Key Architecture Decisions

1. **Event-Driven Architecture**: Uses `crossbeam-channel` for a global event bus (`EVENT_BUS`) that decouples components. Network events are published to the bus and consumed by interested parties.

2. **Dual Protocol Support**: The parser supports both IPMsg (standard) and FeiQ (extended) protocols. Auto-detection happens in `parser.rs` based on packet format (presence of `#` delimiter).

3. **Database Initialization**: Tables are created programmatically in `database/mod.rs` using raw SQL statements, not SeaORM migrations (migrations are in `migration.bak/` and not currently used).

4. **Async UDP Networking**: Uses `tokio` for async I/O. The UDP receiver (`network/udp/receiver.rs`) runs as a background task and publishes events to the global event bus.

## Module Structure

### Backend (src-tauri/src/)

**Core Modules:**
- `main.rs` - Tauri app entry point, event loop, initialization
- `lib.rs` - Library module exports
- `error.rs` - Centralized error types (`AppError`, `AppResult`)
- `types.rs` - Shared types between frontend/backend

**Event System** (`event/`):
- `bus.rs` - Global event bus using `crossbeam_channel`
- `model.rs` - Event types (`AppEvent`, `NetworkEvent`, `UiEvent`)
- **Usage**: `EVENT_SENDER.send(event)` to publish, `EVENT_RECEIVER.recv()` in event loop to consume

**Network Layer** (`network/`):
- `feiq/constants.rs` - IPMsg protocol constants (commands, options, default port 2425)
- `feiq/model.rs` - `FeiqPacket` struct with `ProtocolType` enum (IPMsg/FeiQ)
- `feiq/parser.rs` - Protocol parser with auto-detection
- `feiq/packer.rs` - Packet serialization
- `udp/receiver.rs` - Async UDP listener (binds to 0.0.0.0:2425)
- `udp/sender.rs` - UDP sender with broadcast support

**Database Layer** (`database/`):
- `mod.rs` - Database initialization and table creation (raw SQL, not migrations)
- `model/*.rs` - SeaORM entity definitions (user, contact, group, chat_message, chat_session, file_storage)
- `handler/*.rs` - CRUD operations for each entity

**IPC Layer** (`ipc/`):
- `mod.rs` - IPC module exports
- `*.rs` - Tauri command handlers (chat.rs, contact.rs, file.rs, group.rs)
- Commands use `#[tauri::command]` macro and can access global state via `State<'_, DbConn>`

**Business Logic** (`core/`):
- `contact/discovery.rs` - User discovery via UDP broadcast/entry/ansentry
- `chat/*.rs`, `file/*.rs`, `group/*.rs` - Business logic modules (mostly stubs currently)

**Utilities** (`utils/`):
- `snowflake/*.rs` - Snowflake ID generation (for unique IDs)
- `serde/*.rs` - Serialization helpers

## Protocol Implementation Details

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

### Key Protocol Constants (from constants.rs)
- `IPMSG_BR_ENTRY` (0x01) - Broadcast online presence
- `IPMSG_BR_EXIT` (0x02) - Broadcast offline
- `IPMSG_ANSENTRY` (0x03) - Response to entry broadcast
- `IPMSG_SENDMSG` (0x20) - Send message
- `IPMSG_RECVMSG` (0x21) - Message receive acknowledgment
- `IPMSG_UTF8OPT` (0x00800000) - UTF-8 encoding flag
- `IPMSG_SENDCHECKOPT` (0x00000020) - Require read receipt
- Default port: 2425

### Parser Logic
- Auto-detects protocol by checking for `#` in the packet (FeiQ indicator)
- IPMsg parser splits on first 5 colons to extract fields; everything after 5th colon is `extension` (may contain colons)
- FeiQ parser splits header (by `#`) and data (by `:`) separately

## Testing Strategy

- Unit tests are co-located with source files in `#[cfg(test)]` modules
- Test coverage focus: protocol parser (critical for interoperability)
- Run protocol tests: `cargo test --lib network::feiq::parser`
- All 19 protocol tests should pass

## Important Implementation Notes

### User Discovery Flow
1. On startup: broadcast `IPMSG_BR_ENTRY` packet via UDP to 255.255.255.255:2425
2. When receiving `IPMSG_BR_ENTRY`: send `IPMSG_ANSENTRY` back and add to online user list
3. When receiving `IPMSG_ANSENTRY`: add to online user list
4. On exit: broadcast `IPMSG_BR_EXIT`

### Database Schema Notes
- Tables use INTEGER PRIMARY KEY AUTOINCREMENT for simplicity (not Snowflake IDs yet)
- `session_type` in chat tables: 0 = single chat, 1 = group chat
- `status` in user table: 0 = offline, 1 = online
- `msg_type` in chat_message: 0 = text, other values for file/etc.
- `msg_no` is string (from protocol) but could be parsed to u64 via `packet.msg_no_value()`

### IPC Command Pattern
```rust
#[tauri::command]
async fn command_name(
    param: Type,
    db: State<'_, DbConn>,  // Access global database state
) -> Result<ReturnType, String> {
    // Use db.inner() to get the actual DbConn
    let result = Handler::method(db.inner(), param).await
        .map_err(|e| e.to_string())?;
    Ok(result)
}
```

### Error Handling
- Use `AppError` enum for domain-specific errors
- Use `AppResult<T>` = `Result<T, AppError>` for return types
- Convert errors with `.map_err(|e| AppError::variant(e))?` or `.map_err(|e| e.to_string())?` for IPC

## Development Workflow

1. **Adding a new IPC command:**
   - Add handler function in appropriate `ipc/*.rs` file with `#[tauri::command]`
   - Export in `ipc/mod.rs`
   - Create TypeScript wrapper in frontend `src/ipc/*.ts`
   - Call from frontend with `invoke('command_name', args)`

2. **Adding a new database table:**
   - Add SeaORM model in `database/model/*.rs`
   - Add table creation SQL in `database/mod.rs::create_tables()`
   - Add CRUD operations in `database/handler/*.rs`

3. **Adding new event type:**
   - Add variant to appropriate enum in `event/model.rs`
   - Publish via `EVENT_SENDER.send(AppEvent::Variant(...))`
   - Handle in event loop in `main.rs`

## Current Development Status

- ✅ Phase 1: Project foundation (complete)
- ✅ Phase 2: FeiQ protocol implementation (complete)
- ✅ Phase 3: Database layer (complete)
- ✅ Phase 4: Basic UI (complete)
- ⏳ Phase 5-8: Advanced features (in progress)

See completion reports in `docs/Phase*_完成报告.md` for details.
