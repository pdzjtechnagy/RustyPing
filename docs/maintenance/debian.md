# Debian/Ubuntu Packaging Guide

This guide describes how to manually package RustyPing for Debian-based systems (Ubuntu, Kali, Pop!_OS, etc.).

## Prerequisites

- `dpkg-deb` (usually installed by default on Debian systems)
- A compiled `rustyping` binary for Linux (target `x86_64-unknown-linux-gnu` or `x86_64-unknown-linux-musl`)

## Directory Structure

We will create a temporary directory structure for the package:

```
rustyping_2.1.2_amd64/
├── DEBIAN/
│   └── control
└── usr/
    └── local/
        └── bin/
            └── rustyping
```

## Packaging Steps

1. **Prepare the Directory**:
   ```bash
   mkdir -p build/rustyping_2.1.2_amd64/DEBIAN
   mkdir -p build/rustyping_2.1.2_amd64/usr/local/bin
   ```

2. **Copy Binary**:
   Assuming you have the linux binary in `target/release/rustyping`:
   ```bash
   cp target/release/rustyping build/rustyping_2.1.2_amd64/usr/local/bin/
   chmod 755 build/rustyping_2.1.2_amd64/usr/local/bin/rustyping
   ```

3. **Create Control File**:
   Copy the `packaging/debian/control` file and update the version number if necessary.
   ```bash
   cp packaging/debian/control build/rustyping_2.1.2_amd64/DEBIAN/
   ```

4. **Build the .deb**:
   ```bash
   dpkg-deb --build build/rustyping_2.1.2_amd64
   ```
   This will output `build/rustyping_2.1.2_amd64.deb`.

## Distribution

Upload the `.deb` file to the GitHub Release.

## One-Liner Installation (Client Side)

Users can install it via:

```bash
wget https://github.com/pdzjtechnagy/RustyPing/releases/download/v2.1.2/rustyping_2.1.2_amd64.deb
sudo dpkg -i rustyping_2.1.2_amd64.deb
```
