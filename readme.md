# RustyPing 🦀

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Version](https://img.shields.io/badge/version-2.4.2-green.svg)
![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20Linux%20%7C%20macOS-lightgrey)

**RustyPing** is a high-performance, terminal-based network monitoring tool written in Rust. It combines the utility of `ping` with the visual insights of a professional network analyzer, all within a sleek, responsive TUI (Text User Interface).

Designed for network engineers, system administrators, and enthusiasts, RustyPing provides real-time visibility into your network's health with millisecond precision.

## ✨ New in v2.4.0
*   **Deep Diagnostics Overlay**: Press `Enter` to see real-time DNS resolution, HTTP/HTTPS connectivity checks, and detailed session stats.
*   **Web Checks**: Background monitoring of Port 80 and 443 to detect firewall or service-level issues (toggle with `W`).
*   **DNS Timing**: Automatic measurement of name resolution performance.

## ✨ New in v2.3.0
*   **CSV Logging**: Export real-time latency data to a file for long-term analysis.
*   **Jitter Metrics**: View the Standard Deviation of your connection latency to diagnose instability.


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

### Option 2: Pre-built Binaries
Check the [Releases](https://github.com/pdzjtechnagy/RustyPing/releases) page for pre-compiled binaries for Windows and Linux.

### Option 3: Quick Install Scripts

**Windows (PowerShell):**
```powershell
iwr -useb https://raw.githubusercontent.com/pdzjtechnagy/RustyPing/main/web_install.ps1 | iex
```

**Portable Run (Windows - No Install):**
```powershell
iwr -useb https://raw.githubusercontent.com/pdzjtechnagy/RustyPing/main/run_portable.ps1 | iex
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
