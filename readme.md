# RustyPing 🦀

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20Linux%20%7C%20macOS-lightgrey)
![Rust](https://img.shields.io/badge/rust-1.75%2B-orange)

**RustyPing** is a high-performance, terminal-based network monitoring tool written in Rust. Designed for professionals and enthusiasts alike, it provides real-time latency visualization, jitter analysis, and integrated tools like speed testing and port scanning—all in a sleek, "Blacksite" themed TUI (Text User Interface).

## ✨ Features

*   **Real-time Latency Graph**: High-resolution, 1-dot wide Braille rendering for precise latency tracking (similar to `btop`).
*   **Comprehensive Stats**: Tracks min, max, average latency, jitter, and packet loss in real-time.
*   **Integrated Speed Test**: Built-in upload and download bandwidth testing (powered by Cloudflare).
*   **Port Scanner**: Fast, asynchronous TCP port scanner for common services.
*   **Target History**: Remembers your recent targets and their health stats.
*   **Professional UI**: "Blacksite" dark theme optimized for long monitoring sessions.
*   **Cross-Platform**: Works natively on Windows, Linux, and macOS.

## 🚀 Installation

### One-Liner (Windows PowerShell)
The easiest way to install RustyPing on Windows is via our web installer:
```powershell
iwr https://raw.githubusercontent.com/pdzjtechnagy/rustyping/main/web_install.ps1 | iex
```

### From Source (All Platforms)
Ensure you have Rust installed (`cargo`).

```bash
git clone https://github.com/pdzjtechnagy/rustyping.git
cd rustyping
cargo install --path .
```

## 🎮 Usage

Start monitoring a target immediately:
```bash
rping google.com
rping 1.1.1.1
```

### Keyboard Controls
| Key | Action |
| :--- | :--- |
| **S** | Start **S**peed Test |
| **P** | Start **P**ort Scan |
| **R** | **R**eset Statistics |
| **ESC** | Open **S**ettings / Close Panels |
| **Q** | **Q**uit |

## 🛠️ Development

See [DEVELOPMENT.md](DEVELOPMENT.md) for details on setting up your environment, the Git workflow, and contributing to the project.

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
