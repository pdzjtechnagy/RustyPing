# RustyPing 🦀

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Version](https://img.shields.io/badge/version-2.5.0-green.svg)

---

## ⚡ Quick Start (Portable One-Liners)
Run the latest version of RustyPing instantly without installation.

**Windows (PowerShell):**
```powershell
iwr -useb https://raw.githubusercontent.com/pdzjtechnagy/RustyPing/main/run_portable.ps1 | iex
```

**Linux (Bash):**
```bash
curl -fsSL https://raw.githubusercontent.com/pdzjtechnagy/RustyPing/main/run_portable.sh | bash
```

---

## 🚀 Features

*   **Real-time Latency Graph**: High-resolution, 1-dot wide Braille rendering for a precise timeline of network performance.
*   **Comprehensive Statistics**: Tracks Min, Max, Average, and **Jitter** (Standard Deviation).
*   **Visual Packet Loss**: Dropped packets are clearly marked as grey lines, preserving the visual timeline.
*   **CSV Export**: Log every ping result to a CSV file with timestamps for external analysis (`--log`).
*   **Integrated Tools**:
    *   **Speed Test**: Built-in upload/download bandwidth testing (powered by Cloudflare).
    *   **Port Scanner**: Fast, asynchronous TCP port scanner for common services.
*   **Adaptive UI**: Automatically switches to a compact "mini-mode" for small terminal windows (e.g., tiling window managers).
*   **Cross-Platform**: Runs natively on Windows, Linux (Debian/Ubuntu/Alpine), and macOS.

---

## 📥 Installation

### Option 1: Cargo (Recommended for Rust Users)
If you have Rust installed, this is the easiest way to get the latest version optimized for your hardware.

```bash
cargo install --path .
```

### Option 2: Pre-built Binaries (Automated)
RustyPing is automatically compiled for multiple architectures and distributions via GitHub Actions. You can find the following formats on the [Releases](https://github.com/pdzjtechnagy/RustyPing/releases) page:

*   **Linux**: `.deb` (Ubuntu/Debian), `.rpm` (Fedora), and `.tar.gz` (Generic).
*   **Windows**: `.exe` (Portable) for `x86_64`, `x86`, and `ARM64`.
*   **Architectures**: `x86_64`, `x86`, `ARM64`, `PowerPC`, and `RISC-V`.

### Option 3: Quick Install Scripts

**Linux Utility Menu (Linutil Style):**
```bash
curl -fsSL https://raw.githubusercontent.com/pdzjtechnagy/RustyPing/main/linux_install.sh | sh
```

**Windows Full Install (PowerShell):**
```powershell
iwr -useb https://raw.githubusercontent.com/pdzjtechnagy/RustyPing/main/web_install.ps1 | iex
```

---

## 🎮 Usage

### Basic Usage
Start monitoring a target immediately:

```bash
# Ping a domain
rping google.com

# Ping an IP address
rping 1.1.1.1
```

**Interactive Mode**:
If you run `rping` without any arguments, it will launch the **Interactive Startup Menu**. From here, you can:
*   Type a new target manually.
*   Select from your recent history.
*   Choose from popular default targets (e.g., Google DNS, Cloudflare).

```bash
rping
```

### Command Line Options

| Flag | Description | Example |
| :--- | :--- | :--- |
| `--log <FILE>` | Log results to a CSV file | `rping 1.1.1.1 --log latency.csv` |
| `-m`, `--monotone` | Enable high-contrast monochrome mode | `rping 8.8.8.8 -m` |
| `--list` | List recently visited targets | `rping --list` |
| `-h`, `--help` | Show help information | `rping --help` |

### Interactive Controls
While RustyPing is running, you can use the following keyboard shortcuts:

| Key | Action |
| :--- | :--- |
| **Q** | Quit the application |
| **ESC** | Open Settings Menu |
| **Enter** | Toggle **Diagnostics** Overlay |
| **S** | Run **S**peed Test |
| **P** | Run **P**ort Scan |
| **W** | Toggle **W**eb Check (HTTP/S) |
| **J** | Toggle **J**itter Panel |
| **H** | Toggle **H**istory Panel |
| **R** | **R**eset Statistics |
| **↑ / ↓** | Adjust Ping Interval (slower/faster) |
| **← / →** | Adjust Graph History Length |

---

## 📊 CSV Logging Format
When using the `--log` flag, RustyPing writes data in the following format:

```csv
Timestamp,Target,Latency(ms),Status
2026-02-01 14:00:01,1.1.1.1,12.5,Success
2026-02-01 14:00:02,1.1.1.1,12.8,Success
2026-02-01 14:00:03,1.1.1.1,0.0,Timeout
```

---

## 🛠️ Configuration
RustyPing automatically saves your preferences (ping interval, history length) and target history.
*   **Windows**: `%APPDATA%/rustyping/config.json`
*   **Linux/macOS**: `~/.config/rustyping/config.json`

---

## ❓ Troubleshooting

**Q: I see "command not found" after installing.**
A: Ensure your Cargo bin directory is in your PATH.
*   **Linux/macOS**: `export PATH="$HOME/.cargo/bin:$PATH"`
*   **Windows**: This is usually added automatically, but a restart may be required.

**Q: The graph looks broken or characters are missing.**
A: Ensure your terminal font supports Braille characters (e.g., Cascadia Code, Nerd Fonts). If issues persist, try running with `--monotone` to see if it's a color rendering issue.

---

## 📜 License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
