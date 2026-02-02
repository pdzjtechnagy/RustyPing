# Release Notes - RustyPing v2.3.1

**Release Date:** 2026-02-02

## 🔧 Maintenance & Accessibility Update

RustyPing v2.3.1 focuses on improving the installation experience and ensuring documentation is accessible to all users.

### ✨ Key Changes
*   **Web Installer Upgrade**: The `web_install.ps1` script now includes a beautiful, cyberpunk-themed UI with progress tracking and error handling.
*   **Documentation Integration**: The web installer now automatically downloads and saves the User Guides (`install_guide.md`, `quickstart.md`) to `~/.rustyping/docs/` for offline access.
*   **Installation Matrix**: Updated `install_guide.md` with a ranked list of installation methods (Winget, GUI, Script, Choco) for the MSVC Build Tools.
*   **Bug Fixes**: Resolved minor versioning inconsistencies across the codebase.

---

# Release Notes - RustyPing v2.3.0

**Release Date:** 2026-02-01

## 🚀 A Considerable Leap Forward

We are proud to announce **RustyPing v2.3.0**, a major milestone in our journey to build the ultimate terminal-based network monitoring tool. This release marks a significant transition from a simple CLI utility to a fully interactive, professional-grade network analyzer.

Version 2.3.0 focuses on three key pillars: **Data Persistence**, **Deeper Metrics**, and **Accessibility**.

---

## ✨ Key Highlights

### 📊 CSV Logging
For the first time, RustyPing allows you to export real-time latency data to a file for long-term analysis.
*   **Usage**: `rping 1.1.1.1 --log latency.csv`
*   **Format**: Timestamped CSV rows including Target, Latency (ms), and Status.
*   **Use Case**: diagnosing intermittent network drops over hours or days.

### 📉 Jitter Statistics (Standard Deviation)
Average latency doesn't tell the whole story. We've added a **Jitter** metric to the main statistics panel.
*   **What it is**: The Standard Deviation of your connection latency.
*   **Why it matters**: High jitter indicates network instability, which is critical for VoIP and gaming, even if the average ping is low.

### 🖥️ Interactive TUI Startup Menu
Launching `rping` without arguments now drops you into a beautiful, fully interactive TUI menu.
*   **History**: Select from your recently visited targets.
*   **Defaults**: Quick access to popular providers like Google (8.8.8.8) and Cloudflare (1.1.1.1).
*   **Manual Entry**: Type a new target directly in the TUI.

### 🎨 Monotone Mode
A new high-contrast mode designed for accessibility and specific hardware environments.
*   **Usage**: `rping -m` or `rping --monotone`
*   **Best For**: SSH sessions, vintage terminals, e-ink displays, or users with color vision deficiencies.

---

## 🛠️ Technical Improvements

*   **Background Network Engine**: We've completely refactored the ping logic to run in a dedicated async task. This decouples network operations from the UI rendering, ensuring a buttery-smooth **60 FPS** interface even when the network is timing out.
*   **Visual Feedback**: Packet loss is now visualized as vertical grey lines on the graph, preserving the timeline's integrity without breaking the visual flow.
*   **Platform Support**:
    *   **Alpine Linux**: Official support with static `musl` builds.
    *   **Proxmox**: Verified compatibility for LXC containers.

---

## 📦 Upgrade Instructions

### Using Cargo
If you have Rust installed, simply pull the latest version and rebuild:

```bash
git pull
cargo install --path .
```

### Windows (PowerShell)
Run the update script:

```powershell
iwr -useb https://raw.githubusercontent.com/pdzjtechnagy/RustyPing/main/web_install.ps1 | iex
```

---

*Thank you for using RustyPing! If you encounter any issues, please report them on our GitHub repository.*
