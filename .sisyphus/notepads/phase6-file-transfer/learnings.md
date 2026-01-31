# Phase 6: File Transfer Implementation

## Session Start

**Date**: 2026-01-30
**Phase**: 6 - File Transfer Function
**Status**: 🚀 In Progress

## Goals

Implement complete file transfer functionality for LAN messaging:

1. File request/confirmation protocol
2. Chunked file transfer (4KB chunks)
3. Transfer progress UI
4. Resume capability (断点续传)

## Protocol Specifications

### IPMsg File Transfer Commands

- `IPMSG_SENDMSG | IPMSG_FILEATTACHOPT` (0x00200020) - Send message with file attachment
- `IPMSG_GETFILEDATA` (0x00000060) - Request file data
- `IPMSG_RELEASEFILES` (0x00000061) - Release/cancel file transfer

### File Attachment Format

```
a:filename:size:mtime:attribute:
```

- Multiple files can be attached with `\a` separator

### Transfer Flow

1. **Sender**: Creates file attachment info, sends with SENDMSG|FILEATTACHOPT
2. **Receiver**: Gets file info, prompts user to accept/decline
3. **Receiver**: If accepted, sends GETFILEDATA with offset
4. **Sender**: Sends file data chunk (4KB)
5. **Repeat** until file complete
6. **Receiver**: Sends RELEASEFILES when done

## Technical Requirements

### Chunking Parameters

- Chunk size: 4KB (4096 bytes)
- Timeout: 30 seconds per chunk
- Max retries: 3
- Block size: Configurable (default 4KB)

### Database Schema

Files should be tracked in `file_storage` table:

- Transfer status (pending, transferring, completed, failed, cancelled)
- Progress tracking (bytes_transferred / total_bytes)
- Resume capability (transfer_offset)

## Implementation Plan

### Backend (Rust)

- [ ] Protocol constants in `network/feiq/constants.rs`
- [ ] File attachment parser in `network/feiq/parser.rs`
- [ ] File packer in `network/feiq/packer.rs`
- [ ] File service in `core/file/service.rs`
- [ ] IPC handlers in `ipc/file.rs`

### Frontend (React/TypeScript)

- [ ] File picker component
- [ ] File progress component
- [ ] File service (`src/services/fileService.ts`)
- [ ] File store (`src/store/fileStore.ts`)
- [ ] Integration with chat window

## Known Challenges

1. **Large files**: Need efficient chunking to avoid memory issues
2. **Network reliability**: UDP is unreliable, need ACK/retry mechanism
3. **Concurrent transfers**: Support multiple simultaneous transfers
4. **Resume after crash**: Persist transfer state to database

## References

- IPMsg Protocol Specification (file transfer section)
- Phase 6 implementation plan in `IMPLEMENTATION_PLAN.md`

---

_Last updated: 2026-01-30_
