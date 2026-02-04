# Release Notes - RustyPing v2.5.1

**Release Date:** 2026-02-03

## 🔧 CI/CD Reliability Hotfix

RustyPing v2.5.1 addresses critical build failures in the automated Linux pipeline discovered shortly after the v2.5.0 release. This update ensures that native packages (.deb, .rpm) and multi-arch binaries are correctly generated and distributed.

### ✨ Key Fixes
*   **Pipeline Stability**: Switched to a stable `cross-rs` version and corrected environment variables for cross-compilation.
*   **Packaging Logic**: Fixed path errors in the `.deb` and `.rpm` generation steps within GitHub Actions.
*   **RPM Metadata**: Corrected binary mapping in the RPM manifest to ensure successful installation.

---

# Release Notes - RustyPing v2.5.0

**Release Date:** 2026-02-03

## 🚀 The "Professional Distribution" Update

RustyPing v2.5.0 marks a significant milestone in the project's evolution, introducing a robust CI/CD pipeline and broad architectural support. This release ensures that RustyPing can run on almost any modern system, from high-end workstations to specialized Linux environments like Proxmox and RISC-V SBCs.

### ✨ Key Highlights

*   **Automated Multi-Platform Builds**: Integrated a full GitHub Actions CI/CD pipeline.
*   **Expanded Linux Support**: Native `.deb` and `.rpm` packages for all major distributions.
*   **Static Portability**: New `musl`-based Linux builds for zero-dependency execution.
*   **Broad Architecture Support**:
    *   **Linux**: x86_64, ARM64, PowerPC 64LE, and RISC-V 64.
    *   **Windows**: x86_64, i686 (32-bit), and AArch64 (ARM64).
*   **Robust Asset Discovery**: Updated portable launchers with regex-based binary matching for reliable "one-liner" execution.
*   **Deep Debug Audit**: Comprehensive codebase review for v2.5.0 stability.

---

# Release Notes - RustyPing v2.4.5

**Release Date:** 2026-02-03

## 🛠️ The "110% Functionality" Update

RustyPing v2.4.5 is a major reliability and UX release, addressing core logic bugs and enhancing the "zero-install" experience across all platforms.

### ✨ Key Improvements
*   **Dynamic Ping Control**: Fixed a critical bug where ping interval adjustments (Arrow Keys) were not propagating to the network task.
*   **Linux Capability Support**: The installer now automatically sets `cap_net_raw+ep`, allowing ICMP pings without `sudo`.
*   **Unified Portable Launchers**: Standardized GitHub API-driven one-liners for Windows and Linux.
*   **Compiler-Level Polish**: Zero-warning build with fully optimized release profiles.

---

# Release Notes - RustyPing v2.4.3

**Release Date:** 2026-02-03

## 🚀 Unified Portable Experience

RustyPing v2.4.3 introduced a standardized portable experience across Windows and Linux, making it easier than ever to run the tool without installation.

### ✨ Key Improvements
*   **Linux Portable Launcher**: New `run_portable.sh` provides a one-liner experience for Linux users (`curl | bash`).
*   **Linutil-Style Maintenance**: New `linux_install.sh` script for distribution-aware installation and system optimization.
*   **Architecture Detection**: Both launchers now automatically detect `x86_64` and `arm64` architectures.
*   **Zero-Install Priority**: Documentation now highlights portable runs as the primary way to get started.

---

# Release Notes - RustyPing v2.4.2 (Internal)

**Release Date:** 2026-02-03

## 🔧 Infrastructure & Global Sync

RustyPing v2.4.2 was an internal milestone focused on synchronizing the ecosystem for multi-platform distribution.

### ✨ Key Improvements
*   **Global Version Audit**: Synchronized version strings across 17+ files including Winget, Debian, and Homebrew manifests.
*   **Documentation Overhaul**: Complete rewrite of `INSTALL.md` and `README.md` to reflect the new async architecture.
*   **Build System**: Optimized `release_helper.ps1` for automated artifact preparation.

---

# Release Notes - RustyPing v2.4.1 (Hotfix)

*   **Compilation Fix**: Resolved missing `Stop` variant in `PingCommand` enum.
*   **Code Quality**: Addressed multiple Clippy lints and warnings for better performance and safety.

---

# Release Notes - RustyPing v2.4.0

**Release Date:** 2026-02-03

## 🧠 Network Intelligence & Deep Diagnostics

RustyPing v2.4.0 transforms the tool from a high-performance pinger into a comprehensive network intelligence platform.

### ✨ Key Highlights

### 🔍 Deep Diagnostics Overlay
Press **Enter** at any time to open the new Diagnostics Overlay. This view provides a consolidated summary of:
*   **DNS Resolution Performance**: See exactly how long it takes to resolve your target's hostname.
*   **Service Connectivity**: Real-time status of common web ports (80/443).
*   **Extended Stats**: A focused view of ICMP metrics and packet loss.

### 🌐 Web Check (Background Monitoring)
You can now monitor more than just ICMP.
*   **Toggle**: Press **W** to enable/disable background TCP checks.
*   **Capability**: Monitors Port 80 (HTTP) and Port 443 (HTTPS) independently of your pings.
*   **Use Case**: Detect if a target is reachable via ICMP but blocking web traffic (or vice versa).

### ⚡ Async Engine Enhancements
The network engine has been further decoupled. TCP checks run in their own non-blocking tasks, ensuring that even a slow or hung web check never impacts the accuracy of your latency graph.

---

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
