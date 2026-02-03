# ⚡ Installation Guide

RustyPing is designed to be ultra-portable. You can run it instantly or install it permanently.

## ⚡ Quick Start (Portable One-Liners)

Run RustyPing instantly without installing anything. Both scripts automatically detect your architecture (`x86_64` or `arm64`) and fetch the latest version.

### Windows (PowerShell)
```powershell
iwr -useb https://raw.githubusercontent.com/pdzjtechnagy/RustyPing/main/run_portable.ps1 | iex
```

### Linux (Bash)
```bash
curl -fsSL https://raw.githubusercontent.com/pdzjtechnagy/RustyPing/main/run_portable.sh | bash
```

---

## 🛠️ Permanent Installation

### Windows (Winget)
```powershell
winget install rustyping
```

### Linux (Debian/Ubuntu/Kali)
1. Download the `.deb` from [Releases](https://github.com/pdzjtechnagy/RustyPing/releases/latest).
2. Install:
```bash
sudo apt install ./rustyping_amd64.deb
```
*Note: Using `apt` handles dependencies automatically compared to `dpkg`.*

### Linux Utility Menu (Linutil-Style)
A menu-driven installer and system optimizer for all major distributions:
```bash
curl -fsSL https://raw.githubusercontent.com/pdzjtechnagy/RustyPing/main/linux_install.sh | bash
```

### macOS (Homebrew)
```bash
brew tap pdzjtechnagy/tap
brew install rustyping
```

---

## 🏗️ Build from Source

If you prefer to build it yourself, ensure you have the [Rust toolchain](https://rustup.rs/) installed.

### Windows Prerequisites
You must have the **MSVC v143 build tools** installed (via Visual Studio 2022).
```powershell
winget install Microsoft.VisualStudio.2022.BuildTools --override "--add Microsoft.VisualStudio.Workload.VCTools --includeRecommended --passive"
```

### Compilation
```bash
git clone https://github.com/pdzjtechnagy/RustyPing.git
cd RustyPing
cargo build --release
```
The binary will be located at `target/release/rping` (or `rping.exe` on Windows).
