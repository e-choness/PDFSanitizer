# GitHub Setup & Initial Release

## Step 1: Initialize GitHub Repository

If you haven't already pushed to GitHub:

```bash
cd D:\projects\PDFSanitizer

# Initialize git (already done)
git init

# Add all files
git add .

# Create initial commit
git commit -m "Initial commit: PDF Sanitizer with Tauri, Rust, and Svelte"

# Add GitHub remote (replace YOUR_USERNAME)
git remote add origin https://github.com/YOUR_USERNAME/PDFSanitizer.git

# Push to GitHub
git branch -M main
git push -u origin main
```

---

## Step 2: Verify GitHub Actions is Enabled

1. Go to your GitHub repo
2. Click **Settings** → **Actions** → **General**
3. Ensure "Actions permissions" is set to **"Allow all actions"**
4. Save

---

## Step 3: Create Your First Release

```bash
# Make sure everything is committed and pushed
git status  # Should show "nothing to commit"

# Create a version tag
git tag v1.0.0
git push origin v1.0.0
```

### What Happens Next:
1. GitHub Actions automatically starts building
2. Watch the build at: `https://github.com/YOUR_USERNAME/PDFSanitizer/actions`
3. After ~35 minutes, all platforms are built:
   - ✅ Windows `.exe`
   - ✅ macOS `.dmg`
   - ✅ Linux `.AppImage`

---

## Step 4: Download Your First Release

### Option A: During Build (While Workflow Running)
1. Go to **Actions** tab
2. Click the "Build Release" workflow
3. Scroll down to **Artifacts**
4. Download `pdf-sanitizer-x64` (Windows)

### Option B: After Build Complete (Official Release)
1. Go to **Releases** section
2. Click on `v1.0.0` tag
3. Download `.exe` for Windows

---

## Step 5: Test the Release

1. **Download** `pdf-sanitizer.exe` to your computer
2. **Run it** (no installation needed!)
3. Test the features:
   - Drag & drop PDFs
   - Configure sanitization options
   - Process files
   - Check backup folder

---

## Future Releases

Every time you want to release:

```bash
# 1. Make changes, commit, and push
git add .
git commit -m "feat: add feature X"
git push origin main

# 2. Update version in src-tauri/tauri.conf.json
# Change "version": "1.0.0" to "1.0.1" (or appropriate version)

git add src-tauri/tauri.conf.json
git commit -m "bump version to 1.0.1"
git push origin main

# 3. Create and push tag (triggers automatic build)
git tag v1.0.1
git push origin v1.0.1

# 4. Wait 30-40 minutes for builds to complete
# 5. Download from GitHub Releases page
```

---

## GitHub Actions Status

**Check build status anytime:**
- Go to **Actions** tab
- Green ✅ = Success
- Orange ⏳ = In progress
- Red ❌ = Failed

**View workflow details:**
- Click any workflow run
- Expand job logs to see what happened
- Common errors have clear error messages

---

## Important Files for CI/CD

These were created and handle the automation:

```
.github/
├── workflows/
│   ├── release.yml    # Builds & releases on tag push
│   └── test.yml       # Tests on every push/PR
```

**Do not modify** these unless you understand GitHub Actions syntax.

---

## Troubleshooting GitHub Actions

### "Workflow failed"
1. Click workflow run in Actions tab
2. Check job logs (expand red sections)
3. Common causes:
   - Rust compilation error
   - Missing dependencies
   - Syntax error in code

**Fix:** Correct the error, push to main, re-tag with new version

### "File not found in artifacts"
- Builds take 30-40 minutes
- Wait for workflow to complete (green checkmark)
- Check you're looking at the right workflow run

### "Can't push tag"
```bash
# Try with force (if tag already exists)
git push origin v1.0.0 --force

# Or use different version
git tag v1.0.1
git push origin v1.0.1
```

---

## Security Notes

**GitHub Secrets:**
- No secrets needed for basic builds
- All credentials handled by GitHub
- Release artifacts are public

**Code Safety:**
- test.yml checks code before release
- cargo clippy catches common errors
- cargo fmt ensures code style

---

## Recommended Workflow

```
Make Changes
    ↓
Test Locally (cargo tauri dev)
    ↓
Commit & Push (git push origin main)
    ↓
Wait for test.yml to pass
    ↓
Update Version Number
    ↓
Create Git Tag (git tag vX.X.X)
    ↓
Push Tag (git push origin vX.X.X)
    ↓
GitHub Actions Builds (30-40 min)
    ↓
Download from Releases Page
    ↓
Test Downloaded Executable
    ↓
Announce Release
```

---

## One-Command Release (After Setup)

Once everything is configured:

```bash
# 1. Edit version in src-tauri/tauri.conf.json
# 2. Commit changes
git add .
git commit -m "version 1.1.0: description of changes"

# 3. Tag and push (triggers automatic build)
git tag v1.1.0 && git push origin main && git push origin v1.1.0
```

Done! Wait 30-40 minutes for builds, then download from Releases.

---

## Next Steps

1. ✅ **Now:** Push your code to GitHub
2. ✅ **Now:** Enable GitHub Actions in Settings
3. ✅ **Now:** Create tag `v1.0.0` to trigger first build
4. ⏳ **Wait:** 30-40 minutes for all builds to complete
5. ✅ **Then:** Download `.exe` from Releases page
6. ✅ **Then:** Test the application
7. ✅ **Then:** Share the link with users

---

## Quick Reference

| Task | Command |
|------|---------|
| Push code | `git push origin main` |
| Create release | `git tag v1.0.0 && git push origin v1.0.0` |
| Check status | Open GitHub Actions tab |
| Download exe | Go to Releases, download `.exe` |
| Update version | Edit `src-tauri/tauri.conf.json` |

---

**You're all set!** 🚀

Your PDF Sanitizer will build automatically on every release tag. No manual builds needed!
