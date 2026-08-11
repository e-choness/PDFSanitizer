# Beta Releases Guide

This guide explains how to create beta, RC (Release Candidate), and alpha releases for testing purposes.

---

## Why Beta Releases?

Beta releases allow you to:
- ✅ Test new features before official release
- ✅ Get feedback from early adopters
- ✅ Fix issues before stable release
- ✅ Iterate quickly without stable version churn
- ✅ Mark releases as pre-release (not recommended for production)

---

## Release Types

### Alpha (a)
Early experimental features, likely unstable

```
v1.1.0-alpha.1
v1.1.0-alpha.2
```

### Beta (b)
Feature-complete but may have bugs, ready for broader testing

```
v1.1.0-beta.1
v1.1.0-beta.2
```

### Release Candidate (rc)
Likely stable, final testing before production release

```
v1.1.0-rc.1
v1.1.0-rc.2
```

### Stable
Official release, recommended for production use

```
v1.1.0
```

---

## How to Create a Beta Release

### Method 1: Using GitHub Web UI (Easiest)

1. **Make your changes locally**
   ```bash
   git add .
   git commit -m "beta: add new feature X"
   git push origin main
   ```

2. **Go to GitHub**
   - Open your repository
   - Click **Actions** tab
   - Click **Build Beta Release** workflow

3. **Trigger Manual Build**
   - Click "Run workflow" dropdown button
   - Enter beta version: `v1.1.0-beta.1`
   - Click "Run workflow" (green button)

4. **Monitor Progress**
   - Watch the workflow run
   - Takes ~15-25 minutes (Windows, ubuntu-latest)
   - Status shows orange (running), green (done), red (failed)

5. **Download Beta**
   - Go to **Releases** tab
   - Find your beta version
   - Download `pdf-sanitizer.exe`
   - Release marked as "Pre-release" ⚠️

### Method 2: Using Git Commands

```bash
# 1. Make changes and push
git add .
git commit -m "beta: experimental features"
git push origin main

# 2. Go to GitHub Actions and trigger beta.yml workflow manually
# (Cannot be automated via CLI - requires manual workflow_dispatch)

# 3. Or use GitHub CLI if installed:
gh workflow run beta.yml -f beta_version=v1.1.0-beta.1

# 4. Check status
gh run list
gh run view <run-id>

# 5. Download when complete
gh release download v1.1.0-beta.1
```

---

## Beta Release Workflow Examples

### Scenario 1: Quick Testing

```bash
# Commit experimental work
git commit -m "beta: test new PDF rendering"
git push origin main

# Trigger beta.yml manually with v1.1.0-beta.1
# Wait 35 minutes
# Download and test
# If issues found, make fixes and repeat with v1.1.0-beta.2

# When ready for production:
git tag v1.1.0
git push origin v1.1.0
```

### Scenario 2: Multi-Phase Testing

```bash
# Phase 1: Alpha to friends
git commit -m "alpha: initial implementation"
git push
# Trigger: v1.1.0-alpha.1

# Phase 2: Beta to community
git commit -m "beta: bug fixes from alpha"
git push
# Trigger: v1.1.0-beta.1

# Phase 3: Release candidate
git commit -m "rc: final polish"
git push
# Trigger: v1.1.0-rc.1

# Phase 4: Stable release
git tag v1.1.0
git push origin v1.1.0
```

---

## Comparing Beta vs Stable Releases

| Feature | Beta | Stable |
|---------|------|--------|
| Trigger | Manual (workflow_dispatch) | Automatic (tag push) |
| Git Tag Required | No | Yes |
| Version Format | `v1.0.0-beta.1` | `v1.0.0` |
| Marked as Pre-release | Yes ⚠️ | No |
| Intended Use | Testing | Production |
| Time to Build | ~35 minutes | ~35 minutes |
| Workflow File | `.github/workflows/beta.yml` | `.github/workflows/release.yml` |

---

## Version Numbering

Follow semantic versioning:

```
v<MAJOR>.<MINOR>.<PATCH>[-<type>.<number>]

Examples:
v1.0.0          (initial release)
v1.1.0          (new features)
v1.0.1          (bug fix)
v2.0.0          (breaking changes)
v1.1.0-alpha.1  (first alpha)
v1.1.0-beta.1   (first beta)
v1.1.0-beta.2   (second beta)
v1.1.0-rc.1     (release candidate)
```

### Guidelines

- Increment MAJOR for breaking changes
- Increment MINOR for new features
- Increment PATCH for bug fixes
- Increment type number (alpha.1 → alpha.2) for multiple pre-releases
- Do NOT increment for different platforms (one tag for all)

---

## Testing Beta Releases

### For Beta Testers

1. **Download** the appropriate file for your platform
   - Windows: `.exe`
   - macOS: `.dmg`
   - Linux: `.AppImage`

2. **Install/Run**
   - Windows: Double-click `.exe`
   - macOS: Mount `.dmg`, drag to Applications
   - Linux: `chmod +x`, then `./app`

3. **Test Features**
   - Try the new features
   - Test common workflows
   - Check for crashes or errors

4. **Report Issues**
   - Go to GitHub Issues tab
   - Create issue with:
     - Beta version (e.g., v1.1.0-beta.1)
     - Platform (Windows/macOS/Linux)
     - Steps to reproduce
     - Error message (if any)

### For Developers

Track feedback and create next beta:

```bash
# Address reported issues
git commit -m "beta: fix issue reported in v1.1.0-beta.1"
git push origin main

# Trigger next beta
# beta.yml workflow → v1.1.0-beta.2
```

---

## Workflow Details

The `beta.yml` workflow:

✅ Builds for all three platforms
✅ Creates GitHub Release automatically
✅ Marks as "Pre-release"
✅ Adds warning message to release notes
✅ Uploads artifacts for download

**Build Time:** ~35-40 minutes (all platforms parallel)

**Artifacts:**
- Windows: `pdf-sanitizer.exe`

---

## Common Beta Release Patterns

### Pattern 1: Bug Fix Beta

```
1. User reports bug
2. You fix it
3. Trigger beta release with v1.0.1-beta.1
4. User tests fix
5. Release v1.0.1 stable
```

### Pattern 2: Feature Beta

```
1. Develop new feature
2. Trigger v1.1.0-beta.1
3. Get feedback from testers
4. Fix issues
5. Trigger v1.1.0-beta.2
6. More feedback
7. Trigger v1.1.0-rc.1 (final testing)
8. Release v1.1.0 stable
```

### Pattern 3: Experimental Beta

```
1. Experimental features added
2. Trigger v1.1.0-alpha.1
3. Internal testing
4. Trigger v1.1.0-alpha.2 with fixes
5. Promote to beta: v1.1.0-beta.1
6. Public testing
7. Release v1.1.0 stable
```

---

## FAQ

**Q: Can I use beta releases in production?**
A: Not recommended. Beta versions may have bugs. Use stable releases for production.

**Q: Can I go back to a previous beta?**
A: Yes! All versions are available in the Releases tab. Download any version you want.

**Q: How many betas can I create?**
A: As many as you want! There's no limit.

**Q: Does beta affect my GitHub free plan limits?**
A: Yes, same as stable builds (~35 min per build). Free plan has 2,000 min/month.

**Q: Can I add more target platforms to beta builds?**
A: Yes — edit `beta.yml` to add additional cargo-xwin targets or separate runners.

**Q: What if beta build fails?**
A: Check the workflow logs, fix the issue, and trigger again with same version.

**Q: Can users auto-update from beta to stable?**
A: Tauri doesn't have built-in auto-update, but you can add it as a future enhancement.

---

## Pro Tips

💡 **Tip 1:** Name your beta clearly
```bash
# Good
v1.1.0-beta.1       # First beta of 1.1.0
v1.1.0-rc.1         # Release candidate

# Confusing
v1.1beta            # Not standard format
latest-beta         # Not semantic versioning
```

💡 **Tip 2:** Document changes in release notes
The beta.yml workflow adds a default message, but you can customize it.

💡 **Tip 3:** Use alpha for internal testing
```bash
v1.1.0-alpha.1      # Share only with close collaborators
v1.1.0-beta.1       # Share with broader community
```

💡 **Tip 4:** Test each beta yourself first
Download and verify on all platforms before sharing with testers.

💡 **Tip 5:** Keep track of feedback
Update CHANGELOG.md with fixes applied between betas.

---

## Troubleshooting

### Build Failed

1. Go to Actions tab
2. Click failed workflow
3. Expand job logs
4. Look for error message
5. Fix the issue locally
6. Push fix to main
7. Trigger beta again

### Can't Find Beta in Releases

- Check "Pre-releases" are shown in Releases tab
- Scroll down - newer releases at top
- Try refreshing page

### Download Fails

- Download is large (80-150 MB)
- Check internet connection
- Try different browser
- Download from Artifacts instead (if still building)

---

## Next Steps

After testing beta releases:

1. **Ready for Production?**
   ```bash
   git tag v1.1.0
   git push origin v1.1.0
   ```

2. **Need More Testing?**
   ```bash
   # Trigger beta.yml with v1.1.0-beta.2
   ```

3. **Found Critical Issue?**
   ```bash
   # Fix and push
   # Trigger v1.1.0-beta.3 (or -rc.1 if close to release)
   ```

---

## Release Notes Template

When creating a beta release, include:

```markdown
# Beta Release: v1.1.0-beta.1

⚠️ **This is a beta release** - Use for testing purposes only.

## New Features
- Feature 1
- Feature 2

## Bug Fixes
- Fixed issue X
- Fixed issue Y

## Known Issues
- Known issue A
- Known issue B

## Testing Notes
- Please report any issues on GitHub
- Features may change before stable release
- Not recommended for production use

## Download
- Windows: `pdf-sanitizer.exe`
- macOS: `PDF Sanitizer.dmg`
- Linux: `pdf-sanitizer.AppImage`
```

---

**Happy beta testing!** 🧪✨
