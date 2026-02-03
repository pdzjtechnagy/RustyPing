# Debian/Ubuntu Packaging Guide

This guide describes how to manually package RustyPing for Debian-based systems.

## Packaging Steps

1. **Prepare Directory Structure**:
   ```bash
   mkdir -p build/rustyping_2.4.4_amd64/DEBIAN
   mkdir -p build/rustyping_2.4.4_amd64/usr/local/bin
   ```

2. **Copy Files**:
   ```bash
   cp target/release/rping build/rustyping_2.4.4_amd64/usr/local/bin/
   chmod 755 build/rustyping_2.4.4_amd64/usr/local/bin/rping
   cp packaging/debian/control build/rustyping_2.4.4_amd64/DEBIAN/
   ```

3. **Build .deb**:
   ```bash
   dpkg-deb --build build/rustyping_2.4.4_amd64
   ```

## Client Installation
```bash
sudo apt install ./rustyping_2.4.4_amd64.deb
```
