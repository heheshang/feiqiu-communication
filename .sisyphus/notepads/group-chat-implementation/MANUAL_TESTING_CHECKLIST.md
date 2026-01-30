# Group Chat Feature - Manual Testing Checklist

**Date**: 2026-01-30
**Environment**: Tauri Window (NOT browser)
**App Status**: Running on http://localhost:1420/
**Process ID**: 17980

---

## 🎯 Prerequisites

- [ ] Tauri app is running (should see "飞秋通讯" window)
- [ ] Database is initialized (check `.tauri-dev.log`)
- [ ] At least one contact exists (or available via LAN discovery)

---

## 📋 Test Scenarios

### **Test 1: Tab Navigation**

**Goal**: Verify tab switching between Chats and Groups

| Step | Action                         | Expected Result                           | Actual Result | Pass/Fail |
| ---- | ------------------------------ | ----------------------------------------- | ------------- | --------- |
| 1.1  | Click "Groups" tab             | Sidebar shows group list, empty chat area |               |           |
| 1.2  | Click "Chats" tab              | Sidebar shows chat list, empty chat area  |               |           |
| 1.3  | Click "Groups" tab again       | Returns to previous state (no crash)      |               |           |
| 1.4  | Rapidly switch tabs (5+ times) | No crashes, smooth transitions            |               |           |

**Success Criteria**: ✅ No "Maximum update depth exceeded" errors
**Notes**: This was the bug we fixed - verify it's resolved

---

### **Test 2: View Groups List**

**Goal**: Verify groups display correctly

| Step | Action                         | Expected Result                          | Actual Result | Pass/Fail |
| ---- | ------------------------------ | ---------------------------------------- | ------------- | --------- |
| 2.1  | Click "Groups" tab             | See list of existing groups (if any)     |               |           |
| 2.2  | Check empty state              | See "No groups yet" message if no groups |               |           |
| 2.3  | Locate "+ Create Group" button | Button visible in sidebar header         |               |           |

**Success Criteria**: ✅ Groups list renders without errors
**Notes**: Check console for any errors

---

### **Test 3: Create a Group**

**Goal**: Create a new group with members

| Step | Action                         | Expected Result                                          | Actual Result | Pass/Fail |
| ---- | ------------------------------ | -------------------------------------------------------- | ------------- | --------- |
| 3.1  | Click "+ Create Group" button  | Opens CreateGroupDialog modal                            |               |           |
| 3.2  | Check dialog elements          | See group name input, member list, Create/Cancel buttons |               |           |
| 3.3  | Enter group name               | Input accepts text (e.g., "Test Group 1")                |               |           |
| 3.4  | Select members                 | Checkboxes work for available contacts                   |               |           |
| 3.5  | Click "Create" with no name    | Validation error or disabled button                      |               |           |
| 3.6  | Click "Create" with valid data | Dialog closes, group appears in list                     |               |           |
| 3.7  | Click "Cancel"                 | Dialog closes, no group created                          |               |           |

**Success Criteria**: ✅ Can create groups with UI feedback
**Notes**: Try creating 2-3 groups with different member combinations

---

### **Test 4: Group Selection**

**Goal**: Select a group and view its chat

| Step | Action                       | Expected Result                           | Actual Result | Pass/Fail |
| ---- | ---------------------------- | ----------------------------------------- | ------------- | --------- |
| 4.1  | Click on a group in the list | Group becomes selected (highlighted)      |               |           |
| 4.2  | Check chat area              | GroupChatWindow shows for selected group  |               |           |
| 4.3  | Check empty state            | See "Select a group" if no group selected |               |           |
| 4.4  | Switch between groups        | Chat area updates to show selected group  |               |           |

**Success Criteria**: ✅ Group selection updates UI correctly
**Notes**: Verify no crashes when switching groups

---

### **Test 5: Send Group Messages**

**Goal**: Send messages to a group

| Step | Action                 | Expected Result                             | Actual Result | Pass/Fail |
| ---- | ---------------------- | ------------------------------------------- | ------------- | --------- |
| 5.1  | Select a group         | Group chat window opens                     |               |           |
| 5.2  | Type message in input  | Text appears in message input               |               |           |
| 5.3  | Click Send button      | Message appears in chat area                |               |           |
| 5.4  | Check message format   | See sender name, timestamp, message content |               |           |
| 5.5  | Send multiple messages | All messages display in order               |               |           |
| 5.6  | Try Enter key to send  | Message sends (if implemented)              |               |           |

**Success Criteria**: ✅ Messages send and display correctly
**Notes**: Send 5-10 messages to test scrolling

---

### **Test 6: View Message History**

**Goal**: Verify previous messages load correctly

| Step | Action                     | Expected Result                 | Actual Result | Pass/Fail |
| ---- | -------------------------- | ------------------------------- | ------------- | --------- |
| 6.1  | Select group with messages | See all previous messages       |               |           |
| 6.2  | Scroll to top              | All messages accessible         |               |           |
| 6.3  | Switch to different group  | Different message history loads |               |           |
| 6.4  | Switch back                | Original messages still visible |               |           |

**Success Criteria**: ✅ Message history persists across navigation
**Notes**: Try creating multiple groups and sending messages to each

---

### **Test 7: Cross-Tab Integration**

**Goal**: Verify tabs don't interfere with each other

| Step | Action                     | Expected Result               | Actual Result | Pass/Fail |
| ---- | -------------------------- | ----------------------------- | ------------- | --------- |
| 7.1  | Select a chat in Chats tab | Chat displays in ChatWindow   |               |           |
| 7.2  | Switch to Groups tab       | See groups list (not chats)   |               |           |
| 7.3  | Select a group             | Group chat displays           |               |           |
| 7.4  | Switch back to Chats tab   | Previous chat still selected  |               |           |
| 7.5  | Switch to Groups tab again | Previous group still selected |               |           |

**Success Criteria**: ✅ Each tab maintains its own state
**Notes**: This tests the state management architecture

---

### **Test 8: Error Handling**

**Goal**: Verify graceful error handling

| Step | Action                              | Expected Result                     | Actual Result | Pass/Fail |
| ---- | ----------------------------------- | ----------------------------------- | ------------- | --------- |
| 8.1  | Try to create group with no name    | Validation error or button disabled |               |           |
| 8.2  | Try to create group with no members | Warning or error message            |               |           |
| 8.3  | Send empty message                  | Should not send (validation)        |               |           |
| 8.4  | Check console                       | No unhandled errors or crashes      |               |           |

**Success Criteria**: ✅ Errors are handled gracefully
**Notes**: Open DevTools in Tauri window (F12 or Cmd+Option+I)

---

## 🐛 Bug Report Template

If you find any bugs, document them here:

### Bug #1: [Title]

- **Description**: [What happened]
- **Steps to Reproduce**: [1. ... 2. ... 3. ...]
- **Expected Result**: [What should happen]
- **Actual Result**: [What actually happened]
- **Console Errors**: [Copy any error messages]
- **Screenshots**: [If applicable]

---

## ✅ Overall Assessment

**Test Execution Date**: ****\_\_\_****
**Tester**: ****\_\_\_****

**Feature Status**:

- [ ] ✅ Passes all tests
- [ ] ⚠️ Passes with minor issues (see notes)
- [ ] ❌ Has blocking bugs

**Issues Found**: **\_**
**Critical Bugs**: **\_**
**Minor Bugs**: **\_**

---

## 📝 Additional Notes

## **What Works Well**:

- **What Needs Improvement**:

-
- **Feature Requests**:

-
- **Tester Comments**:

-
- ***

## 🔍 How to Open DevTools in Tauri

**macOS**: `Cmd + Option + I`
**Windows/Linux**: `F12` or `Ctrl + Shift + I`

Use DevTools to:

- Check for console errors
- Inspect component state
- Monitor network requests
- View localStorage/state

---

## 🎯 Success Criteria

**Feature is considered PASSING if**:

1. ✅ All core tests (1-7) pass without crashes
2. ✅ No console errors during normal usage
3. ✅ Tab switching is smooth (no infinite loop bug)
4. ✅ Messages send and display correctly
5. ✅ State persists across tab switches

**Feature is considered FAILING if**:

1. ❌ Application crashes during any test
2. ❌ "Maximum update depth exceeded" error appears
3. ❌ Messages don't send or display
4. ❌ Data is lost across tab switches
5. ❌ Console shows unhandled exceptions
