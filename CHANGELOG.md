# Changelog

All notable changes to this project will be documented in this file.

## [v2.5.0] - 2026-02-03

### Added
- **CI/CD Pipeline**: Automated multi-platform builds for Linux and Windows using GitHub Actions.
- **New Packaging**: Native `.deb` (Debian/Ubuntu) and `.rpm` (Fedora/RHEL) support.
- **Multi-Architecture Support**:
  - Linux: `x86_64`, `aarch64`, `musl-static`, `ppc64le`, `riscv64`.
  - Windows: `x86_64`, `i686`, `aarch64`.
- **Static Linking**: Switched to `rustls-tls` for Linux binaries to ensure portability across distros without system OpenSSL.

### Changed
- **Asset Discovery**: Enhanced `run_portable.sh` and `linux_install.sh` with flexible regex-based matching for release assets.
- **Documentation**: Comprehensive update of `README.md`, `INSTALL.md`, and new `docs/CI_CD.md`.
- **Startup Menu**: Updated version branding to `v2.5.0`.

### Fixed
- **Proxmox Compatibility**: Resolved "Binary not found" error in portable launcher by implementing robust architecture detection and fallback logic.
- **CI Workflow**: Optimized build matrix and artifact naming for reliable GitHub Releases.

## [v2.4.5] - 2026-02-03

### Fixed
- **Dynamic Ping Control**: Resolved logic error where ping interval changes were not sent to the background network task.
- **Test Suite**: Updated `test_network_intelligence_flow` to match new `start_ping_task` signature.

### Added
- **Linux Capability Management**: Installer now automates `setcap` for non-root ICMP access.
- **Deep Debug Audit**: Zero-warning compiler state with enhanced Clippy enforcement.

## [v2.4.4] - 2026-02-03 (Internal)
- Version bump for deep debug and verification cycle.

## [v2.4.3] - 2026-02-03

### Added
- **Linux Portable Launcher**: New `run_portable.sh` for instant Linux execution (`curl | bash`).
- **Unified Quick Start**: Added one-liner portable commands for both Windows and Linux to README.
- **Linux Utility Script**: Added `linux_install.sh` mirroring `linutil` with maintenance and optimization tools.

### Fixed
- **Version Synchronization**: Standardized versioning across all 17+ files including manifests and scripts.
- **Process Locking (Windows)**: Portable launcher now handles binary updates gracefully if an instance is running.
- **Architecture Detection**: Automated detection for `x86_64` and `aarch64` in portable scripts.

## [v2.4.2] - 2026-02-03 (Internal)
- Version bump and release preparation.

## [v2.4.1] - 2026-02-03 (Hotfix)
- Fixed `PingCommand::Stop` compilation error in tests.
- Resolved Clippy warnings in `ping.rs` and `main.rs`.

### Network Intelligence & Deep Diagnostics
Version 2.4.0 transforms RustyPing from a ping tool into a comprehensive network intelligence platform. This release introduces "Deep Diagnostics," allowing users to see beyond simple ICMP echo requests.

### Added
- **Network Intelligence Engine**:
  - **DNS Resolution Time**: Now measures and displays the time taken to resolve the target domain name.
  - **Web Check**: Background connectivity checks for HTTP (Port 80) and HTTPS (Port 443). Toggle this feature with `W`.
- **Deep Diagnostics**:
  - **Granular Error Reporting**: Distinguishes between "Connection Refused," "Timeout," and other IO errors for TCP checks.
  - **Diagnostics Overlay**: A new dedicated overlay (toggle with `Enter`) provides a focused view of DNS, HTTP/S status, and detailed ping statistics.
- **Async Architecture**:
  - TCP checks run in independent, non-blocking `tokio` tasks to ensure the main ICMP ping loop remains perfectly timed, even during TCP timeouts.

### Fixed
- **Codebase Cleanup**: Removed unused fields and optimized internal message passing.

## [v2.3.0] - 2026-02-01

### A Considerable Leap Forward
Version 2.3.0 represents a major milestone in RustyPing's development, transitioning from a simple CLI tool to a fully interactive, professional-grade network analyzer. This release focuses on data persistence, deeper metrics, and accessibility.

### Added
- **CSV Logging**: New `--log <FILE>` argument to export ping results to a CSV file in real-time. Ideal for long-term monitoring and analysis.
- **TUI Startup Menu**: Completely redesigned the startup experience. Launching without arguments now opens an interactive TUI menu to select recent targets, default providers (Google/Cloudflare), or enter a new target manually.
- **Jitter Statistics**: Added Jitter (Standard Deviation) metric to the main statistics panel, providing deeper insight into network stability.
- **Monotone Mode**: New `--monotone` (`-m`) flag for high-contrast, color-free rendering. Ideal for SSH, vintage terminals, and e-ink displays.
- **Platform Support**: 
  - **Alpine Linux**: Added static `musl` build support for zero-dependency deployment.
  - **Proxmox**: Verified compatibility for LXC containers.
- **Official Branding**: Updated all package manifests and documentation to use the official `pdzjtechnagy` handle.

### Changed
- **Network Engine**: Refactored ping logic to run in a background async task. This decouples network latency from the UI thread, ensuring a smooth 60 FPS interface even during packet loss.
- **Visual Feedback**: Failed pings now render as vertical grey lines in the graph, preserving the timeline continuity while clearly indicating packet loss.
- **Theme System**: Converted `Theme` from static constants to a dynamic system backed by `AtomicBool`, enabling runtime theme switching.

### Fixed
- **UI Freezing**: Resolved an issue where high-latency or dropped packets would cause the entire TUI to freeze.
- **Test Suite**: Fixed unit tests that were broken by the theme refactoring.
- **Clippy Lints**: Cleaned up iterator loops and redundant pattern matching for a more robust codebase.

### Packaging
- Updated Homebrew Formula for macOS.
- Updated Winget Manifest for Windows.
- Updated Debian control file.
