# Quick Testing Guide - Group Chat Features

**Date**: 2026-01-30
**App Status**: Running (Tauri window should be open)
**Goal**: Quick verification of new features

---

## 🎯 Quick Test (5 Minutes)

### Test 1: Access Groups Tab

1. Look for Tauri window titled "飞秋通讯"
2. Click "Groups" tab in the sidebar
3. ✅ **Expected**: See groups list (or "No groups yet")

### Test 2: Create Test Group (if needed)

1. Click "+ Create Group" button
2. Enter name: "Test Group"
3. Select members (checkboxes)
4. Click "Create Group"
5. ✅ **Expected**: Group appears in list

### Test 3: Member Management

1. Click on a group to select it
2. Click "Members" button (👥 icon) in header
3. ✅ **Expected**: Members sidebar opens

**Add Member**:

1. Click ➕ button in members header
2. ✅ **Expected**: AddMemberDialog opens
3. Select users (checkboxes)
4. Click "Add X Members"
5. ✅ **Expected**: Dialog closes, members list updates

**Manage Members**:

1. Hover over a member in the list
2. ✅ **Expected**: ⬆️⬇️✕ buttons appear
3. Click ⬆️ or ⬇️ to change role
4. ✅ **Expected**: Role changes
5. Click ✕ to remove member
6. ✅ **Expected**: Confirmation dialog, member removed

### Test 4: Group Settings

1. Click ⋮ (more) button in group chat header
2. ✅ **Expected**: GroupSettingsDialog opens

**Edit Info**:

1. Change group name
2. Change description
3. Click "Save Changes"
4. ✅ **Expected**: Dialog closes, name updates in UI

**Delete Group**:

1. In settings, scroll to "Danger Zone"
2. Click "Delete Group"
3. ✅ **Expected**: Confirmation appears
4. Click "Yes, Delete Group"
5. ✅ **Expected**: Page reloads, group deleted

---

## 🐛 Report Results

**Pass/Fail for Each Test**:

- [ ] Test 1: Groups tab accessible
- [ ] Test 2: Can create group
- [ ] Test 3: Can add members
- [ ] Test 4: Can manage members (roles, remove)
- [ ] Test 5: Can edit group info
- [ ] Test 6: Can delete group

**Issues Found**:

```
(List any bugs or problems here)
```

**Console Errors**:

```
(Check Tauri console: F12 or Cmd+Option+I)
```

---

## 📝 Next Steps After Testing

**If All Tests Pass** → Proceed to Part B (Enhancements)

**If Tests Fail** → Fix bugs first, then enhancements

**Your Results**:

```
(Please fill in after manual testing)
```
