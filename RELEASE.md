# Release Preparation Guide

This document describes the operational process for preparing and publishing a release.

## Release Workflow

The release process is automated via GitHub Actions. When you push a git tag, the workflow automatically:

1. **Runs pre-release tests** - Ensures all tests pass
2. **Builds cross-platform binaries** - Compiles for Linux (gnu, musl), macOS (x86_64, ARM64), and Windows
3. **Creates GitHub Release** - Generates draft release with artifacts

## How to Prepare a Release

### Step 1: Ensure Working Tree is Clean

```bash
# Make sure you have the latest changes
git checkout master
git pull origin master

# Check working tree status
git status
```

### Step 2: Update Version in Cargo.toml

Edit the version number in `Cargo.toml`:

```toml
[package]
version = "X.Y.Z"  # Update this to your release version
```

### Step 3: Commit Version Bump

```bash
git add Cargo.toml
git commit -m "chore: bump version to X.Y.Z"
```

### Step 4: Create and Push Git Tag

```bash
# Create an annotated tag
git tag -a vX.Y.Z -m "Release vX.Y.Z"

# Push the tag to trigger release
git push origin vX.Y.Z
```

### Step 5: Monitor the Release

1. Go to your repository's **Actions** tab
2. Watch the **Release** workflow run
3. Wait for all build jobs to complete
4. A draft GitHub Release will be created automatically

### Step 6: Review and Publish Release

1. Go to the **Releases** page
2. Edit the draft release:
   - Add release notes (optional)
   - Verify all binaries are attached
3. Click **Publish release**

## Manual Release (Optional)

If you need to run the release manually:

1. Go to **Actions** → **Release**
2. Click **Run workflow**
3. Select "Use workflow from: master"
4. Click **Run workflow**

## Troubleshooting

### Release didn't trigger
- Ensure the tag follows the format `v*` (e.g., `v1.0.0`)
- Check the Actions tab for any workflow failures

### Build failures
- Check the specific matrix job logs
- Ensure cross-compilation toolchains are available

### Artifacts missing
- Verify all build jobs completed successfully
- Check the artifact upload step in logs
