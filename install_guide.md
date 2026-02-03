# RustyPing 2.0 - Complete Installation Guide

> **NOTE:** This is the manual installation and build guide. For the easiest setup, use the **Portable One-Liners** in [README.md](file:///c:/RustyPing/README.md).

## 📋 What You're Building

**RustyPing 2.4.4** - A professional network monitoring tool with:

- ✅ btop-style braille graphs
- ✅ Blacksite theme (minimal, professional)
- ✅ Smart IP history with fuzzy find
- ✅ On-demand speed tests
- ✅ Port scanning capability
- ✅ Keyboard-driven interface
- ✅ Linux/Windows Portable Launchers

## 🚀 Step-by-Step Build Guide (Windows)

### Step 1: Install MSVC Build Tools
Rust on Windows requires the **MSVC v143 - VS 2022 C++ x64/x86 build tools**.

#### 🥇 Recommended Method (Winget)
```powershell
winget install -e --id Microsoft.VisualStudio.2022.BuildTools --override "--passive --wait --add Microsoft.VisualStudio.Workload.VCTools --includeRecommended"
```

### Step 2: Install Rust
```powershell
winget install Rustlang.Rustup
# Close and reopen terminal after this!
```

### Step 3: Clone & Build
```powershell
git clone https://github.com/pdzjtechnagy/RustyPing.git
cd RustyPing
cargo build --release
```

## 🛠️ Installation Methods

### Windows (Winget)
```powershell
winget install rustyping
```

### Linux (Generic)
Use the [linux_install.sh](file:///c:/RustyPing/linux_install.sh) for a menu-driven experience:
```bash
curl -fsSL https://raw.githubusercontent.com/pdzjtechnagy/RustyPing/main/linux_install.sh | bash
```

### macOS (Homebrew)
```bash
brew tap pdzjtechnagy/tap
brew install rustyping
```

## ⚠️ Troubleshooting

### Build Error: "linker link.exe not found"
Ensure Visual Studio Build Tools are installed with the "Desktop development with C++" workload.

### Braille characters display as boxes
Use **Windows Terminal** or a modern terminal emulator with a font like **Cascadia Code** or **JetBrains Mono**.

### Permission Denied (Linux/macOS)
Some network operations (ICMP) may require `sudo` or specific capabilities:
```bash
# On Linux, you can grant the binary capability to avoid using sudo:
sudo setcap cap_net_raw+ep ./target/release/rping
```
