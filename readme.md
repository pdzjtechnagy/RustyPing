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
*   **Ultra-Compact Mode**: Automatically switches to a minimal graph-only view for very small terminal windows (e.g., 20x5), perfect for tiling window managers.
*   **Target History**: Remembers your recent targets and their health stats.
*   **Professional UI**: "Blacksite" dark theme optimized for long monitoring sessions.
*   **Cross-Platform**: Works natively on Windows, Linux, and macOS.

## 🚀 Installation Guide (For Absolute Beginners)

We've made installing RustyPing as easy as possible. Choose the method that fits your comfort level.

### Option 1: The "Instant Run" (Portable) Method
Use this if you want to run RustyPing *right now* without installing anything permanently. It works exactly like the "Chris Titus Tool" - it runs from memory/temp and leaves no trace.

```powershell
iwr -useb https://raw.githubusercontent.com/pdzjtechnagy/RustyPing/main/run_portable.ps1 | iex
```

### Option 2: The "Install" Method (Windows Only)
Use this if you want RustyPing permanently installed on your system.
Open your PowerShell (press `Win + X` and select **Terminal** or **PowerShell**) and paste this single line. It handles downloading, building, and installing for you automatically.

```powershell
iwr -useb https://raw.githubusercontent.com/pdzjtechnagy/RustyPing/main/web_install.ps1 | iex
```

*Note: If you see red error text about "link.exe" or "C++ Build Tools", don't panic! It just means your computer needs a standard Microsoft tool to build software. The error message will give you the exact command to fix it.*

---

### Option 3: The "Manual" Method (Step-by-Step)
If you prefer to see exactly what's happening or are on Linux/macOS, follow these steps.

#### 1. Install Prerequisites
RustyPing is built with **Rust**, so you need the Rust toolchain installed.

*   **Windows**:
    1.  Open PowerShell as Administrator.
    2.  Run: `winget install Rustlang.Rustup`
    3.  **Important**: You also need C++ Build Tools. Run:
        ```powershell
        winget install Microsoft.VisualStudio.2022.BuildTools --override "--add Microsoft.VisualStudio.Workload.VCTools --includeRecommended --passive --norestart"
        ```
    4.  Close and reopen your terminal.

*   **Linux / macOS**:
    Open your terminal and run:
    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```

#### 2. Get the Code
Download the RustyPing source code to your computer.
```bash
git clone https://github.com/pdzjtechnagy/RustyPing.git
cd RustyPing
```

#### 3. Build & Install
Turn the code into a runnable program. This might take a minute or two as it optimizes the engine for your specific machine.
```bash
cargo install --path .
```

#### 4. Run It!
You can now run RustyPing from anywhere on your computer.
```bash
rping 1.1.1.1
```

---

## ❓ Troubleshooting & FAQ

**Q: I get "command not found" after installing.**
A: You likely need to restart your terminal so it recognizes the new `rping` command. If that doesn't work, ensure your Rust `bin` folder is in your PATH.

**Q: The installation failed with "linker not found".**
A: This is the most common issue on Windows. It means you are missing the Visual Studio Build Tools. Run the `winget` command in the "Manual Method" section above to install them.

**Q: Why do I need to compile it?**
A: Compiling from source ensures RustyPing runs at maximum speed on your specific hardware. It's like getting a suit tailored exactly to your measurements instead of buying off the rack!

## 🎮 Usage Guide


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
