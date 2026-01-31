# GitHub v1.0.0 Release - Step-by-Step Guide

**Date**: 2026-01-31
**Status**: Ready to publish
**All artifacts prepared**: ✅

---

## Pre-Release Checklist ✅

- [x] Code complete (all major features implemented)
- [x] Tests passing (109/109 tests)
- [x] Documentation complete (1,400+ lines)
- [x] Release notes written (533 lines)
- [x] README updated (download section added)
- [x] macOS DMG built (34 MB)
- [x] macOS app bundle built (9.3 MB)
- [x] All commits pushed to main

---

## Release Creation Steps

### Step 1: Open GitHub Releases Page

**URL**: https://github.com/heheshang/feiqiu-communication/releases/new

**Actions**:

1. Open the URL in your browser
2. Make sure you're logged into GitHub
3. You should see "Create a new release" page

---

### Step 2: Fill in Release Information

#### 2.1 Choose a Tag

**Field**: Tag version

```
v1.0.0
```

**Notes**:

- Type `v1.0.0` (include the "v" prefix)
- Click "Choose a tag" dropdown
- If tag doesn't exist, GitHub will create it
- Target: Select `main` branch

#### 2.2 Release Title

**Field**: Release title

```
飞秋通讯 v1.0.0 - 首次正式发布 / First Official Release
```

**Notes**:

- Bilingual format (Chinese primary, English secondary)
- Professional and clear

#### 2.3 Release Description

**Field**: Description (large text area)

**Instructions**:

1. Open `RELEASE_NOTES.md` file in your editor
2. Select ALL content (Cmd+A / Ctrl+A)
3. Copy (Cmd+C / Ctrl+C)
4. Paste into the description field (Cmd+V / Ctrl+V)

**What to expect**:

- ~533 lines of content
- Bilingual format (Chinese/English)
- Includes:
  - Project introduction
  - Feature list
  - Installation guides
  - Technical specifications
  - Troubleshooting
  - Roadmap
  - Contributing guide
  - License

---

### Step 3: Attach Release Binaries

#### 3.1 Attach DMG Installer (Required)

**Actions**:

1. Click "Attach binaries" or "Browse files"
2. Navigate to: `releases/飞秋通讯_1.0.0_x64.dmg`
3. Select and upload

**File details**:

- Name: `飞秋通讯_1.0.0_x64.dmg`
- Size: 34 MB
- Type: macOS disk image
- Required: YES (primary distribution format)

#### 3.2 Attach App Bundle (Optional)

**Actions**:

1. Click "Attach binaries" again
2. Navigate to: `releases/飞秋通讯.app`
3. Select and upload

**File details**:

- Name: `飞秋通讯.app`
- Size: 9.3 MB
- Type: macOS application bundle
- Required: NO (optional for advanced users)

**Note**: Uploading may take 1-2 minutes per file due to size.

---

### Step 4: Configure Release Settings

#### 4.1 Set as Latest Release

**Checkbox**: ✅ "Set as the latest release"

**Action**: Check this box

**Reason**: This is the first stable release and should be the default download

#### 4.2 Pre-release (DO NOT CHECK)

**Checkbox**: ⬜ "Set as a pre-release"

**Action**: Leave this UNCHECKED

**Reason**: This is a production-ready release, not a pre-release

---

### Step 5: Publish Release

**Action**: Click the green "Publish release" button

**What happens**:

1. GitHub creates the `v1.0.0` tag
2. Release page becomes public
3. DMG becomes available for download
4. "Latest release" badge appears on repository

**Expected URL after publishing**:

```
https://github.com/heheshang/feiqiu-communication/releases/v1.0.0
```

---

## Post-Release Verification

### Verify Release Exists

**URL**: https://github.com/heheshang/feiqiu-communication/releases/v1.0.0

**Check**:

- [ ] Release page loads
- [ ] Title is correct
- [ ] Description displays correctly (533 lines)
- [ ] DMG download button works
- [ ] "Latest release" badge visible on repo main page

### Verify Tag Created

**Command**:

```bash
git tag
```

**Expected output**:

```
v1.0.0
```

### Verify Download Works

**Action**:

1. Click download button on release page
2. Verify DMG downloads (34 MB)
3. Verify file name: `飞秋通讯_1.0.0_x64.dmg`

---

## Optional Post-Release Tasks

### 1. Test DMG Installation (5 min)

**Steps**:

1. Download DMG from release page
2. Open DMG (double-click)
3. Drag app to Applications
4. Launch from Applications (not DMG)
5. Verify features work

**Expected**:

- ✅ App launches
- ✅ User discovery works
- ✅ Chat functionality works
- ⚠️ May see "unidentified developer" warning (expected, can bypass)

### 2. Update README.md (if needed)

**Add to README** (if not already there):

```markdown
## Download

**Latest Version**: v1.0.0

**macOS**: [Download DMG (34 MB)](https://github.com/heheshang/feiqiu-communication/releases/v1.0.0)
```

**Commit**:

```bash
git add README.md
git commit -m "docs: add download link for v1.0.0"
git push origin main
```

### 3. Celebrate! 🎉

**You just published your first official release!**

---

## Troubleshooting

### Issue: Tag Already Exists

**Error**: "Tag v1.0.0 already exists"

**Solution**:

```bash
# Check existing tags
git tag

# Delete local tag if needed
git tag -d v1.0.0

# Delete remote tag if needed
git push origin :refs/tags/v1.0.0

# Try release creation again
```

### Issue: DMG Upload Fails

**Symptoms**: Upload progress bar stops, error appears

**Solutions**:

1. Check file size (should be 34 MB)
2. Check internet connection
3. Try smaller file first (app bundle 9.3 MB)
4. Use GitHub CLI if available: `gh release create v1.0.0`

### Issue: Description Formatting Broken

**Symptoms**: Release notes don't display correctly

**Solution**:

1. Make sure you copied ENTIRE RELEASE_NOTES.md
2. Check for proper markdown formatting
3. Edit release after creation if needed

---

## Release Summary

**Version**: v1.0.0
**Date**: 2026-01-31
**Type**: Official Release (Production)
**Platform**: macOS (Intel x64)
**Size**: 34 MB (DMG)
**Status**: Ready to publish

---

## Next Steps After Release

### Immediate (Post-Release)

1. ✅ Verify release is accessible
2. ✅ Test DMG installation
3. ✅ Announce release (if desired)

### Short-term (Weeks 1-2)

4. Monitor user feedback
5. Fix any critical bugs
6. Plan v1.0.1 patch if needed

### Long-term (Month 1+)

7. Resume Phase 10 development (v1.1 features)
8. Build Windows/Linux versions
9. Gather user feedback for roadmap

---

**Good luck with your first release! 🚀**

_This guide was created on 2026-01-31 for 飞秋通讯 v1.0.0_
