# macOS Homebrew Packaging Guide

## Packaging Steps

1. **Create Tarball**:
   ```bash
   tar -czf rustyping-macos-v2.4.5.tar.gz rping
   ```

2. **Update Formula**:
   Edit [rustyping.rb](file:///c:/RustyPing/packaging/macos/rustyping.rb):
   - Update `version "2.4.5"`
   - Update `url` to the new release tarball.
   - Update `sha256` with the new hash.

## Installation
```bash
brew tap pdzjtechnagy/tap
brew install rustyping
```
