# macOS Homebrew Packaging Guide

This guide describes how to maintain the Homebrew Formula for RustyPing.

## Prerequisites

- A GitHub repository for your custom tap (e.g., `homebrew-rustyping` or just inside your main repo).
- A release tarball (`.tar.gz`) containing the macOS binary.

## Packaging Steps

1. **Create the Release Tarball**:
   ```bash
   tar -czf rustyping-macos-v2.1.2.tar.gz rustyping
   ```
   (Where `rustyping` is the compiled macOS binary).

2. **Calculate SHA256**:
   ```bash
   shasum -a 256 rustyping-macos-v2.1.2.tar.gz
   ```

3. **Update the Formula**:
   Edit `packaging/macos/rustyping.rb`:
   - Update `url` to point to the new release tarball.
   - Update `sha256` with the value from step 2.
   - Update `version`.

4. **Publish to Tap**:
   If you have a dedicated tap repo (e.g., `github.com/username/homebrew-tap`):
   - Copy `packaging/macos/rustyping.rb` to the tap repo.
   - Commit and push.

## One-Liner Installation (Client Side)

Users can install it via:

```bash
# If using a custom tap
brew tap pdzjtechnagy/tap
brew install rustyping

# Or directly from the formula URL (useful for testing)
brew install https://raw.githubusercontent.com/pdzjtechnagy/RustyPing/main/packaging/macos/rustyping.rb
```
