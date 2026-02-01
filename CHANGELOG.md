# Changelog

All notable changes to this project will be documented in this file.

## [v2.2.0] - 2026-02-01

### Added
- **CSV Logging**: New `--log <FILE>` argument to export ping results to a CSV file in real-time. Ideal for long-term monitoring and analysis.
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
