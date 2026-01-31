# Phase 10: Post-Release Features (v1.1)

## Status

**📋 PLANNING** - Creating detailed task breakdown for v1.1 features

---

## TL;DR

> **Quick Summary**: 实现 v1.1 版本的高级功能，包括消息搜索、文件传输历史、主题切换、代码签名和性能优化
>
> **Deliverables**:
>
> - 消息搜索功能（全文搜索、筛选、高亮）
> - 文件传输历史界面（传输记录、状态管理）
> - 主题切换（深色/浅色模式）
> - 代码签名（macOS、Windows）
> - 性能优化（启动时间、内存占用）
>
> **Estimated Effort**: High (4-6 weeks)
> **Parallel Execution**: YES - Multiple feature tracks
> **Critical Path**: Message search → Performance testing

---

## Context

### Previous Work Completed

**Phase 9 (Release Preparation)**: 88% Complete

- ✅ Code cleanup
- ✅ Error handling
- ✅ Production build (macOS)
- ✅ User documentation (1,400+ lines)
- ✅ Release packaging (DMG)
- ✅ Testing (109 tests passing)
- ✅ Release notes (534 lines)
- ⏳ Icons & metadata (optional - functional icons exist)

**v1.0.0 Release Status**: Ready to publish

- README updated with download section
- DMG installer built (34 MB)
- All tests passing
- Documentation complete

### v1.1 Roadmap Items

From RELEASE_NOTES.md, the following features are planned for v1.1:

1. **消息搜索功能 / Message search**
   - Full-text search across chat messages
   - Filter by date, sender, chat session
   - Highlight search results
   - Quick navigation to results

2. **文件传输历史界面 / File transfer history UI**
   - View all file transfers (incoming/outgoing)
   - Transfer status (completed, pending, failed)
   - File size and timestamp
   - Re-download or open files

3. **主题切换 / Theme switching**
   - Light mode (current default)
   - Dark mode
   - System preference detection
   - Persist user choice

4. **代码签名 / Code signing**
   - macOS code signing (avoid "unidentified developer" warning)
   - Windows code signing (Authenticode)
   - Certificate management

5. **性能优化 / Performance improvements**
   - Startup time optimization (< 2s target)
   - Memory usage optimization (< 50MB target)
   - Database query optimization
   - Bundle size reduction

---

## Work Objectives

### Core Objective

实现 v1.1 版本的高级功能，提升用户体验和应用质量。重点在于消息搜索、文件传输历史、主题切换、代码签名和性能优化。

### Concrete Deliverables

1. **Message Search Module**
   - Search input in chat window
   - Full-text search across messages
   - Result highlighting and navigation
   - Search filters (date, sender, session)

2. **File Transfer History UI**
   - Dedicated file history panel
   - Transfer status indicators
   - File actions (open, re-download, delete)
   - Statistics dashboard

3. **Theme System**
   - Theme provider (React context)
   - Light/dark theme styles
   - System preference detection
   - Theme persistence in database

4. **Code Signing Infrastructure**
   - macOS signing configuration
   - Windows signing configuration
   - Automated signing in CI/CD
   - Certificate documentation

5. **Performance Optimizations**
   - Lazy loading for chat history
   - Virtual scrolling for message lists
   - Database indexing
   - Asset optimization

### Definition of Done

- [ ] All features implemented and tested
- [ ] Manual testing on macOS
- [ ] Manual testing on Windows (if possible)
- [ ] Performance benchmarks meet targets
- [ ] Code signing verified (no warnings)
- [ ] Documentation updated
- [ ] v1.1.0 release notes drafted

---

## Execution Strategy

### Parallel Execution Tracks

```
Track 1 (User Experience):
├── Task 1.1: Message search UI
├── Task 1.2: File transfer history UI
└── Task 1.3: Theme switching

Track 2 (Infrastructure):
├── Task 2.1: Code signing setup
└── Task 2.2: Performance optimization

Track 3 (Testing & Docs):
├── Task 3.1: Feature testing
└── Task 3.2: Documentation updates
```

### Dependency Matrix

| Task         | Depends On | Blocks | Can Parallelize With |
| ------------ | ---------- | ------ | -------------------- |
| 1.1 (Search) | None       | 3.1    | 1.2, 1.3, 2.1, 2.2   |
| 1.2 (Files)  | None       | 3.1    | 1.1, 1.3, 2.1, 2.2   |
| 1.3 (Theme)  | None       | 3.1    | 1.1, 1.2, 2.1, 2.2   |
| 2.1 (Sign)   | None       | None   | All tasks            |
| 2.2 (Perf)   | 1.1, 1.2   | 3.1    | 1.3, 2.1             |
| 3.1 (Test)   | All        | None   | None                 |
| 3.2 (Docs)   | All        | None   | None                 |

---

## TODOs

### Track 1: User Experience Features

#### Task 1.1: Message Search Functionality

- [ ] 1.1.1 Design search UI component

  **What to do**:
  - Add search input to chat window header
  - Design search results panel
  - Add keyboard shortcut (Cmd+F / Ctrl+F)
  - Result highlighting design

  **Must NOT do**:
  - Don't block UI during search
  - Don't search entire database without pagination

  **Recommended Agent Profile**:

  > - **Category**: `visual-engineering`
  > - **Skills**: [`frontend-ui-ux`]

  **Parallelization**:
  - Can Run In Parallel: YES
  - Parallel Group: Track 1 (with 1.2, 1.3)

  **Acceptance Criteria**:

  > - Search input visible in chat window
  > - Keyboard shortcut works (Cmd+F / Ctrl+F)
  > - Search results panel shows matches
  > - Design follows WeChat-inspired style

  **Commit**: YES
  - Message: `feat(ui): add message search UI components`
  - Files: `src/components/SearchInput.tsx`, `src/components/SearchResults.tsx`

- [ ] 1.1.2 Implement search backend

  **What to do**:
  - Add IPC command for search: `search_messages(query, filters)`
  - Implement SQL full-text search using LIKE or FTS5
  - Add database indexes on message content
  - Implement result pagination

  **Must NOT do**:
  - Don't load all messages into memory
  - Don't use client-side filtering (server-side only)

  **Recommended Agent Profile**:

  > - **Category**: `unspecified-high`
  > - **Skills**: [`git-master`]

  **Parallelization**:
  - Can Run In Parallel: NO (depends on 1.1.1 UI design)
  - Blocks: 1.1.3

  **Acceptance Criteria**:

  > - IPC command `search_messages` works
  > - SQL query uses indexes
  > - Results paginated (50 per page)
  > - Search time < 500ms for 10K messages

  **Commit**: YES
  - Message: `feat(chat): add message search backend`
  - Files: `src-tauri/src/ipc/chat.rs`, `src-tauri/src/database/handler/chat.rs`

- [ ] 1.1.3 Implement search highlighting and navigation

  **What to do**:
  - Highlight matching text in messages
  - Add "Next/Previous" navigation buttons
  - Scroll to selected result
  - Show match count

  **Must NOT do**:
  - Don't break message rendering
  - Don't slow down scrolling

  **Recommended Agent Profile**:

  > - **Category**: `visual-engineering`
  > - **Skills**: [`frontend-ui-ux`]

  **Parallelization**:
  - Can Run In Parallel: NO (depends on 1.1.2 backend)

  **Acceptance Criteria**:

  > - Matches highlighted in yellow
  > - Next/Previous buttons work
  > - Auto-scroll to result
  > - Match counter visible

  **Commit**: YES
  - Message: `feat(ui): add search result highlighting`
  - Files: `src/components/MessageItem.tsx`, `src/components/SearchResults.tsx`

- [ ] 1.1.4 Add search filters

  **What to do**:
  - Date range filter
  - Sender filter
  - Chat session filter
  - Filter combination logic

  **Must NOT do**:
  - Don't make filters too complex
  - Don't lose search performance

  **Recommended Agent Profile**:

  > - **Category**: `quick`
  > - **Skills**: [`git-master`]

  **Parallelization**:
  - Can Run In Parallel: NO (depends on 1.1.3)

  **Acceptance Criteria**:

  > - Date picker works
  > - Sender dropdown shows all contacts
  > - Session dropdown shows all chats
  > - Filters apply correctly

  **Commit**: YES
  - Message: `feat(search): add search filters`
  - Files: `src/components/SearchFilters.tsx`, `src-tauri/src/ipc/chat.rs`

#### Task 1.2: File Transfer History UI

- [ ] 1.2.1 Design file history panel

  **What to do**:
  - Create file history panel component
  - Design list view of transfers
  - Add status indicators (icons/colors)
  - Add file metadata display

  **Must NOT do**:
  - Don't duplicate existing file transfer logic

  **Recommended Agent Profile**:

  > - **Category**: `visual-engineering`
  > - **Skills**: [`frontend-ui-ux`]

  **Parallelization**:
  - Can Run In Parallel: YES
  - Parallel Group: Track 1 (with 1.1, 1.3)

  **Acceptance Criteria**:

  > - Panel accessible from main window
  > - Transfer list shows all files
  > - Status icons clear (completed/failed/pending)
  > - File info visible (name, size, date)

  **Commit**: YES
  - Message: `feat(ui): add file transfer history panel`
  - Files: `src/components/FileHistoryPanel.tsx`

- [ ] 1.2.2 Implement file history backend

  **What to do**:
  - Add IPC command: `get_file_transfers(filters)`
  - Query file_storage table
  - Add transfer status tracking
  - Implement sorting and filtering

  **Must NOT do**:
  - Don't expose file paths without validation

  **Recommended Agent Profile**:

  > - **Category**: `unspecified-low`
  > - **Skills**: [`git-master`]

  **Parallelization**:
  - Can Run In Parallel: NO (depends on 1.2.1 UI)

  **Acceptance Criteria**:

  > - IPC command returns transfer list
  > - Includes status (completed/failed/pending)
  > - Sorted by date (newest first)
  > - Filters work (all/incoming/outgoing)

  **Commit**: YES
  - Message: `feat(file): add file transfer history backend`
  - Files: `src-tauri/src/ipc/file.rs`, `src-tauri/src/database/handler/file.rs`

- [ ] 1.2.3 Add file actions

  **What to do**:
  - Open file action
  - Re-download action (for incoming)
  - Delete record action
  - Show file in Finder/Explorer

  **Must NOT do**:
  - Don't allow deleting files still in use

  **Recommended Agent Profile**:

  > - **Category**: `quick`
  > - **Skills**: [`git-master`]

  **Parallelization**:
  - Can Run In Parallel: NO (depends on 1.2.2)

  **Acceptance Criteria**:

  > - Open button launches file
  > - Re-download works (if available)
  > - Delete removes record only
  > - Show in Finder works

  **Commit**: YES
  - Message: `feat(file): add file transfer actions`
  - Files: `src/components/FileHistoryPanel.tsx`, `src-tauri/src/ipc/file.rs`

#### Task 1.3: Theme Switching

- [ ] 1.3.1 Create theme system

  **What to do**:
  - Add theme context (React)
  - Define theme tokens (colors, fonts)
  - Create light/dark theme objects
  - Add theme provider component

  **Must NOT do**:
  - Don't hardcode colors in components

  **Recommended Agent Profile**:

  > - **Category**: `visual-engineering`
  > - **Skills**: [`frontend-ui-ux`]

  **Parallelization**:
  - Can Run In Parallel: YES
  - Parallel Group: Track 1 (with 1.1, 1.2)

  **Acceptance Criteria**:

  > - Theme context provides current theme
  > - Theme toggle function available
  > - CSS variables defined for tokens
  > - Components use theme tokens

  **Commit**: YES
  - Message: `feat(theme): create theme system`
  - Files: `src/contexts/ThemeContext.tsx`, `src/themes/index.ts`

- [ ] 1.3.2 Implement theme persistence

  **What to do**:
  - Add theme preference to database
  - Add IPC commands: `get_theme()`, `set_theme()`
  - Load theme on startup
  - Save theme on change

  **Must NOT do**:
  - Don't lose user's theme choice

  **Recommended Agent Profile**:

  > - **Category**: `quick`
  > - **Skills**: [`git-master`]

  **Parallelization**:
  - Can Run In Parallel: NO (depends on 1.3.1)

  **Acceptance Criteria**:

  > - Theme saved to database
  > - Theme loads on app start
  > - IPC commands work
  > - Default theme: light

  **Commit**: YES
  - Message: `feat(theme): add theme persistence`
  - Files: `src-tauri/src/ipc/settings.rs`, `src-tauri/src/database/handler/settings.rs`

- [ ] 1.3.3 Add theme toggle UI

  **What to do**:
  - Add theme toggle button to settings
  - Add keyboard shortcut (optional)
  - Detect system preference
  - Apply theme immediately

  **Must NOT do**:
  - Don't require app restart

  **Recommended Agent Profile**:

  > - **Category**: `visual-engineering`
  > - **Skills**: [`frontend-ui-ux`]

  **Parallelization**:
  - Can Run In Parallel: NO (depends on 1.3.2)

  **Acceptance Criteria**:

  > - Toggle button visible in settings
  > - Theme switches immediately
  > - System preference detected
  > - Choice persists across restarts

  **Commit**: YES
  - Message: `feat(ui): add theme toggle button`
  - Files: `src/components/Settings.tsx`, `src/contexts/ThemeContext.tsx`

### Track 2: Infrastructure

#### Task 2.1: Code Signing

- [ ] 2.1.1 Set up macOS code signing

  **What to do**:
  - Obtain Apple Developer certificate ($99/year)
  - Configure `tauri.conf.json` for macOS signing
  - Test signing on local build
  - Verify no "unidentified developer" warning

  **Must NOT do**:
  - Don't commit certificate to repo
  - Don't use ad-hoc signing for production

  **Recommended Agent Profile**:

  > - **Category**: `unspecified-low`
  > - **Skills**: [`git-master`]

  **Parallelization**:
  - Can Run In Parallel: YES
  - Parallel Group: Track 2 (with 2.2)

  **Acceptance Criteria**:

  > - Certificate obtained (or documented)
  > - `tauri.conf.json` updated
  > - Local build signed successfully
  > - No warnings on install

  **Commit**: YES
  - Message: `chore: configure macOS code signing`
  - Files: `src-tauri/tauri.conf.json`, `docs/CODE_SIGNING.md`

- [ ] 2.1.2 Set up Windows code signing

  **What to do**:
  - Obtain code signing certificate
  - Configure `tauri.conf.json` for Windows signing
  - Test signing on Windows build
  - Verify SmartScreen reputation

  **Must NOT do**:
  - Don't commit certificate to repo

  **Recommended Agent Profile**:

  > - **Category**: `unspecified-low`
  > - **Skills**: [`git-master`]

  **Parallelization**:
  - Can Run In Parallel: YES (with 2.1.1)

  **Acceptance Criteria**:

  > - Certificate obtained (or documented)
  > - `tauri.conf.json` updated
  > - Windows build signed
  > - SmartScreen warnings reduced

  **Commit**: YES
  - Message: `chore: configure Windows code signing`
  - Files: `src-tauri/tauri.conf.json`, `docs/CODE_SIGNING.md`

- [ ] 2.1.3 Automate signing in CI/CD

  **What to do**:
  - Set up GitHub Actions for signing
  - Use secrets for certificates
  - Sign releases automatically
  - Document the process

  **Must NOT do**:
  - Don't leak certificates

  **Recommended Agent Profile**:

  > - **Category**: `unspecified-low`
  > - **Skills**: [`git-master`]

  **Parallelization**:
  - Can Run In Parallel: NO (depends on 2.1.1, 2.1.2)

  **Acceptance Criteria**:

  > - GitHub Actions workflow created
  > - Certificates stored as secrets
  > - Releases signed automatically
  > - Process documented

  **Commit**: YES
  - Message: `ci: add automated code signing`
  - Files: `.github/workflows/release.yml`, `docs/CODE_SIGNING.md`

#### Task 2.2: Performance Optimization

- [ ] 2.2.1 Optimize startup time

  **What to do**:
  - Profile current startup time
  - Lazy load Tauri plugins
  - Defer non-critical initialization
  - Optimize database queries on startup

  **Must NOT do**:
  - Don't break functionality

  **Recommended Agent Profile**:

  > - **Category**: `ultrabrain`
  > - **Skills**: [`git-master`]

  **Parallelization**:
  - Can Run In Parallel: NO (depends on 1.1, 1.2 for context)

  **Acceptance Criteria**:

  > - Startup time < 2 seconds
  > - All features still work
  > - No console errors

  **Commit**: YES
  - Message: `perf: optimize startup time`
  - Files: `src-tauri/src/main.rs`, `src-tauri/src/init.rs`

- [ ] 2.2.2 Optimize memory usage

  **What to do**:
  - Profile current memory usage
  - Implement message pagination (load 100 at a time)
  - Clear old message cache
  - Optimize image handling

  **Must NOT do**:
  - Don't lose message history

  **Recommended Agent Profile**:

  > - **Category**: `ultrabrain`
  > - **Skills**: [`git-master`]

  **Parallelization**:
  - Can Run In Parallel: NO (depends on 2.2.1)

  **Acceptance Criteria**:

  > - Memory usage < 50MB (idle)
  > - Messages scroll smoothly
  > - No memory leaks

  **Commit**: YES
  - Message: `perf: optimize memory usage`
  - Files: `src-tauri/src/core/chat/service.rs`, `src/components/ChatWindow.tsx`

- [ ] 2.2.3 Add database indexes

  **What to do**:
  - Add indexes on message content (for search)
  - Add indexes on timestamps
  - Analyze query performance
  - Update migration scripts

  **Must NOT do**:
  - Don't slow down inserts

  **Recommended Agent Profile**:

  > - **Category**: `quick`
  > - **Skills**: [`git-master`]

  **Parallelization**:
  - Can Run In Parallel: NO (depends on 2.2.2)

  **Acceptance Criteria**:

  > - Search queries use indexes
  > - Query time < 100ms
  > - No performance regression

  **Commit**: YES
  - Message: `perf: add database indexes for search`
  - Files: `src-tauri/src/database/mod.rs`

### Track 3: Testing & Documentation

#### Task 3.1: Feature Testing

- [ ] 3.1.1 Test message search

  **What to do**:
  - Test search with various queries
  - Test filters (date, sender, session)
  - Test performance with 10K+ messages
  - Test UI responsiveness

  **Must NOT do**:
  - Don't skip edge cases

  **Recommended Agent Profile**:

  > - **Category**: `unspecified-low`
  > - **Skills**: [`git-master`]

  **Parallelization**:
  - Can Run In Parallel: NO (depends on 1.1.x)

  **Acceptance Criteria**:

  > - All search variations work
  > - Filters work correctly
  > - Performance acceptable
  > - No crashes

  **Commit**: NO (testing only)

- [ ] 3.1.2 Test file transfer history

  **What to do**:
  - Test UI displays correctly
  - Test file actions (open, delete)
  - Test with various file types
  - Test status indicators

  **Must NOT do**:
  - Don't skip error cases

  **Recommended Agent Profile**:

  > - **Category**: `unspecified-low`
  > - **Skills**: [`git-master`]

  **Parallelization**:
  - Can Run In Parallel: NO (depends on 1.2.x)

  **Acceptance Criteria**:

  > - All transfers visible
  > - Actions work correctly
  > - Status accurate
  > - No crashes

  **Commit**: NO (testing only)

- [ ] 3.1.3 Test theme switching

  **What to do**:
  - Test light/dark mode toggle
  - Test system preference detection
  - Test theme persistence
  - Visual inspection of all components

  **Must NOT do**:
  - Don't skip component checks

  **Recommended Agent Profile**:

  > - **Category**: `visual-engineering`
  > - **Skills**: [`frontend-ui-ux`]

  **Parallelization**:
  - Can Run In Parallel: NO (depends on 1.3.x)

  **Acceptance Criteria**:

  > - Theme switches correctly
  > - All components themed
  > - Persistence works
  > - No visual glitches

  **Commit**: NO (testing only)

- [ ] 3.1.4 Performance benchmarks

  **What to do**:
  - Measure startup time
  - Measure memory usage
  - Measure search performance
  - Compare with v1.0 baseline

  **Must NOT do**:
  - Don't falsify metrics

  **Recommended Agent Profile**:

  > - **Category**: `unspecified-low`
  > - **Skills**: [`git-master`]

  **Parallelization**:
  - Can Run In Parallel: NO (depends on 2.2.x)

  **Acceptance Criteria**:

  > - Startup < 2s
  > - Memory < 50MB
  > - Search < 500ms
  > - Documented in README

  **Commit**: YES (if benchmarks updated)
  - Message: `docs: update performance benchmarks`
  - Files: `README.md`, `docs/PERFORMANCE.md`

#### Task 3.2: Documentation Updates

- [ ] 3.2.1 Update user guide

  **What to do**:
  - Add search feature documentation
  - Add file transfer history docs
  - Add theme switching docs
  - Update screenshots

  **Must NOT do**:
  - Don't leave outdated info

  **Recommended Agent Profile**:

  > - **Category**: `writing`
  > - **Skills**: [`git-master`]

  **Parallelization**:
  - Can Run In Parallel: NO (depends on 3.1)

  **Acceptance Criteria**:

  > - All new features documented
  > - Screenshots updated
  > - Instructions clear
  > - Bilingual (Chinese/English)

  **Commit**: YES
  - Message: `docs: update user guide for v1.1`
  - Files: `docs/USER_GUIDE.md`

- [ ] 3.2.2 Write v1.1 release notes

  **What to do**:
  - List new features
  - List bug fixes
  - List performance improvements
  - Add upgrade instructions

  **Must NOT do**:
  - Don't forget to credit contributors

  **Recommended Agent Profile**:

  > - **Category**: `writing`
  > - **Skills**: [`git-master`]

  **Parallelization**:
  - Can Run In Parallel: NO (depends on 3.2.1)

  **Acceptance Criteria**:

  > - All features listed
  > - Bilingual format
  > - Upgrade instructions clear
  > - Performance metrics included

  **Commit**: YES
  - Message: `docs: add v1.1 release notes`
  - Files: `RELEASE_NOTES.md`, `docs/Phase10_完成报告.md`

---

## Success Criteria

### Verification Commands

```bash
# 1. Build
cd src-tauri && cargo build --release
# Expected: Success, no warnings

# 2. Run tests
cargo test
# Expected: All tests pass

# 3. Test search functionality
# Open app, press Cmd+F, search for message
# Expected: Results appear, highlighted

# 4. Test file history
# Open file history panel
# Expected: All transfers visible

# 5. Test theme switching
# Toggle theme in settings
# Expected: Theme changes immediately

# 6. Performance benchmarks
# Measure startup time, memory usage
# Expected: Startup < 2s, Memory < 50MB
```

### Final Checklist

- [ ] All 3 tracks complete (UX, Infrastructure, Testing)
- [ ] `cargo test` - All tests pass
- [ ] Manual testing - All features work
- [ ] Performance benchmarks - Meet targets
- [ ] Code signing - No warnings
- [ ] Documentation - Complete and updated
- [ ] Release notes - Drafted
- [ ] v1.1.0 ready for release

---

## Notes

### Prerequisites

1. **For Code Signing**:
   - Apple Developer account ($99/year) for macOS
   - Code signing certificate for Windows
   - Access to certificate management

2. **For Performance Testing**:
   - macOS machine for profiling
   - 10K+ test messages for benchmarking

3. **For Documentation**:
   - Screenshots of new features
   - Performance metrics data

### Estimated Timeline

| Track     | Tasks        | Estimated Time |
| --------- | ------------ | -------------- |
| 1 (UX)    | 1.1-1.3      | 2-3 weeks      |
| 2 (Infra) | 2.1-2.2      | 1-2 weeks      |
| 3 (Test)  | 3.1-3.2      | 1 week         |
| **Total** | **13 tasks** | **4-6 weeks**  |

### Dependencies

- **External Certificates**: Required for code signing (Task 2.1)
- **macOS Hardware**: Required for performance profiling (Task 2.2)
- **Test Data**: Required for search testing (Task 3.1)

---

**Phase 10 Status**: 📋 **PLANNING** - Ready to begin implementation

**Next Step**: Start with Track 1, Task 1.1 (Message Search UI) or Task 2.1 (Code Signing setup)
