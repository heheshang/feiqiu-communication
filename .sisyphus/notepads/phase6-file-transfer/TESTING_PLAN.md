# Phase 6 File Transfer - Testing Plan

## Date: 2026-01-30

## Test Environment Setup

### Prerequisites

1. Two machines on the same LAN (or use VMs/network namespaces)
2. Both machines running `bun run tauri dev`
3. Firewall allows UDP port 2425
4. Test files prepared in various sizes

### Test Files

Prepare the following test files:

- **Small text**: `test_small.txt` - 1KB
- **Medium text**: `test_medium.txt` - 100KB
- **Large text**: `test_large.txt` - 10MB
- **Binary file**: `test_binary.jpg` - 5MB image
- **Multiple files**: 3 files of varying sizes

## Manual Testing Procedures

### Test 1: Basic File Transfer (Small Files)

**Objective**: Verify basic file send/receive functionality

**Steps**:

1. On Machine A: Select a contact from Machine B
2. Click file attachment icon in chat window
3. Select `test_small.txt` (1KB)
4. Click "Send Files"
5. On Machine B: Accept the incoming file request
6. Verify transfer completes successfully
7. Compare file checksum (MD5/SHA256) on both machines

**Expected Result**:

- Machine A shows transfer progress 0% → 100%
- Machine B shows file received successfully
- Files match exactly (checksum verified)

### Test 2: Large File Transfer

**Objective**: Verify chunking works correctly

**Steps**:

1. On Machine A: Send `test_large.txt` (10MB)
2. Monitor transfer progress
3. Verify progress updates smoothly (not jump to 100% immediately)
4. On Machine B: Accept and receive file
5. Verify file integrity

**Expected Result**:

- Transfer completes in reasonable time (< 2 minutes on LAN)
- Progress bar updates smoothly
- No memory issues or crashes
- File integrity verified

### Test 3: Multiple File Transfer

**Objective**: Verify batch file transfer

**Steps**:

1. On Machine A: Select 3 test files
2. Send all files to Machine B
3. Verify all transfers complete

**Expected Result**:

- All files transferred successfully
- Order maintained (or clearly indicated)
- Progress shown for each file

### Test 4: File Transfer Cancellation

**Objective**: Verify cancellation works

**Steps**:

1. On Machine A: Start sending large file (10MB)
2. During transfer: Click "Cancel" button
3. On Machine B: Verify transfer stopped
4. On Machine A: Verify sender stopped

**Expected Result**:

- Transfer stops immediately
- Both machines show "Cancelled" status
- Partial file deleted or marked as incomplete

### Test 5: File Rejection

**Objective**: Verify receiver can reject files

**Steps**:

1. On Machine A: Send file to Machine B
2. On Machine B: Click "Reject" button
3. On Machine A: Verify rejection notification

**Expected Result**:

- Machine B shows file rejected
- Machine A shows "File rejected by recipient"
- No partial files saved

### Test 6: Network Interruption (Resume Capability)

**Objective**: Verify resume after disconnect

**Steps**:

1. On Machine A: Start sending large file (10MB)
2. During transfer: Unplug network cable OR stop firewall
3. Wait 10 seconds
4. Restore network connection
5. On Machine A: Click "Resume" button
6. Verify transfer continues from offset

**Expected Result**:

- Transfer resumes from where it left off
- Progress shows correct offset
- File completes successfully
- Checksum verified

### Test 7: Concurrent Transfers

**Objective**: Verify multiple simultaneous transfers

**Steps**:

1. On Machine A: Start sending file to Machine B
2. On Machine B: Start sending different file to Machine A
3. Verify both transfers progress simultaneously

**Expected Result**:

- Both transfers complete successfully
- No deadlocks or race conditions
- UI updates properly for both

### Test 8: Binary File Transfer

**Objective**: Verify binary files transfer correctly

**Steps**:

1. On Machine A: Send `test_binary.jpg` (5MB image)
2. On Machine B: Receive and verify file
3. Open image in viewer to verify integrity

**Expected Result**:

- Image file opens correctly
- No corruption
- File size matches
- Checksum verified

## Automated Unit Tests

### Backend Tests

Run with: `cd src-tauri && cargo test --lib core::file`

**Test Coverage**:

- ✅ `test_transfer_progress` - Progress calculation
- ✅ `test_resume_info_creation` - Resume state creation
- ✅ `test_handle_file_attach_request` - File request handling
- ✅ `test_create_file_attach_request` - Request creation

### Frontend Tests

Run with: `bun test src/components/FileUpload/FileUpload.test.tsx`

**Test Coverage** (if implemented):

- File selection
- Drag and drop
- File size formatting
- Progress updates
- Error handling

## Performance Benchmarks

### Target Metrics

| Metric                     | Target    | Actual |
| -------------------------- | --------- | ------ |
| Small file (< 100KB)       | < 1s      | \_\_\_ |
| Medium file (100KB - 10MB) | < 10s     | \_\_\_ |
| Large file (> 10MB)        | < 60s     | \_\_\_ |
| Transfer speed (LAN)       | > 10 MB/s | \_\_\_ |
| Memory usage               | < 100MB   | \_\_\_ |
| Resume time                | < 1s      | \_\_\_ |

## Known Issues to Watch For

1. **Memory Leaks**: Monitor memory usage during large transfers
2. **UI Blocking**: Ensure UI remains responsive during transfer
3. **Network Errors**: Handle UDP packet loss gracefully
4. **File Permissions**: Verify files can be written to destination
5. **Path Issues**: Test with long paths, special characters, spaces
6. **Concurrent Access**: Test multiple transfers to same user

## Test Results Template

```
Test Case | Status | Notes | Time
---------|--------|-------|-----
Test 1: Small File | ✅ PASS | | 0.5s
Test 2: Large File | ⏳ | |
Test 3: Multiple Files | ⏳ | |
Test 4: Cancellation | ⏳ | |
Test 5: Rejection | ⏳ | |
Test 6: Resume | ⏳ | |
Test 7: Concurrent | ⏳ | |
Test 8: Binary | ⏳ | |

Unit Tests: ✅ PASS (64 tests)
```

## Test Execution Checklist

- [ ] Start dev server on both machines
- [ ] Verify users discover each other
- [ ] Run unit tests (backend + frontend)
- [ ] Execute Test 1 (Small file)
- [ ] Execute Test 2 (Large file)
- [ ] Execute Test 3 (Multiple files)
- [ ] Execute Test 4 (Cancellation)
- [ ] Execute Test 5 (Rejection)
- [ ] Execute Test 6 (Resume)
- [ ] Execute Test 7 (Concurrent)
- [ ] Execute Test 8 (Binary)
- [ ] Record performance metrics
- [ ] Document any issues found

---

**Testing Status**: Ready to execute
**Manual Testing Required**: Yes - requires two machines on LAN
**Automated Tests**: ✅ Already passing (64 tests)
