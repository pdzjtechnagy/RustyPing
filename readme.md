# RustyPing 2.0 🦀

**Professional network monitoring tool with btop-style TUI and braille graphs**

## 🎯 Features

- ✅ **Real-time ping monitoring** with <1s detection
- ✅ **Braille graphs** (btop-style, high-resolution)
- ✅ **Jitter measurement** (connection stability)
- ✅ **Packet loss tracking**
- ✅ **Quality scoring** (EXCELLENT → OFFLINE)
- ✅ **Smart history** with fuzzy find
- ✅ **Blacksite theme** (minimal, professional, dark)
- ✅ **Keyboard-driven** interface (btop-style controls)

## 🚀 Quick Installation

### ⚡ One-Liner (Recommended)

Install RustyPing directly from PowerShell (requires Git and Rust):

```powershell
iwr -useb https://raw.githubusercontent.com/pdzjtechnagy/RustyPing/main/web_install.ps1 | iex
```

### Prerequisites

- Rust (install via `winget install Rustlang.Rustup`)
- Windows Terminal (recommended for best braille rendering)

### Build & Install

```powershell
# 1. Build the release binary
.\build.ps1

# 2. Install globally (adds to PATH)
.\install.ps1

# 3. Restart your terminal, then use:
rping 8.8.8.8
```

### Manual Installation

```powershell
# Build
cargo build --release

# Copy to a directory in your PATH (e.g., %USERPROFILE%\.local\bin)
copy target\release\rping.exe %USERPROFILE%\.local\bin\rping.exe

# Add to PATH if not already there
[Environment]::SetEnvironmentVariable("Path", "$env:Path;%USERPROFILE%\.local\bin", "User")
```

## 📖 Usage

```bash
# Interactive mode (shows recent targets)
rping

# Direct target
rping 8.8.8.8
rping google.com

# Show history
rping --list

# Force selection menu
rping --select
```

## ⌨️ Keyboard Controls

| Key            | Action               |
| -------------- | -------------------- |
| `Q` / `Ctrl+C` | Quit                 |
| `ESC`          | Settings menu        |
| `S`            | Run speed test       |
| `P`            | Port scan            |
| `J`            | Toggle jitter panel  |
| `H`            | Toggle history panel |
| `R`            | Reset statistics     |
| `↑/↓`          | Navigate settings    |
| `Enter`        | Toggle setting       |

## 🎨 UI Improvements

This version includes optimized UI/UX with:

- **Enhanced visual hierarchy** - Better spacing and layout
- **Improved graph rendering** - Smooth gradients and better bounds handling
- **Professional styling** - btop-level aesthetics
- **Better empty states** - Informative messages when no data
- **Enhanced footer** - More information density
- **Polished settings overlay** - Better visual feedback

## 📁 Project Structure

```
rustyping/
├── Cargo.toml              # Dependencies & config
├── src/
│   ├── main.rs             # Entry point & CLI
│   ├── app.rs              # Core application logic
│   ├── ui.rs               # TUI rendering (braille graphs)
│   ├── theme.rs            # Blacksite color scheme
│   ├── storage.rs          # History & config management
│   └── network/
│       ├── mod.rs          # Network operations
│       ├── ping.rs         # Ping monitor
│       ├── speedtest.rs    # Speed test (stub)
│       └── portscan.rs     # Port scanner (stub)
├── build.ps1               # Build script
└── install.ps1            # Installation script
```

## 🔧 Configuration

Config stored in:

- **Windows:** `%APPDATA%\rustyping\history.json`

Settings can be toggled via the ESC menu:

- Show/hide jitter panel
- Show/hide history panel
- Pause ping during speedtest

## 🐛 Troubleshooting

**"Permission denied" error**

- Run as administrator (ICMP requires elevated privileges on Windows)

**Braille shows as boxes**

- Use Windows Terminal instead of cmd.exe

**First ping timeouts**

- Normal behavior - DNS resolution delay, second ping succeeds

**Binary not found after install**

- Restart your terminal after running install.ps1
- Verify PATH includes the install directory

## 📝 Development

```bash
# Development build
cargo build

# Run tests
cargo test

# Run with specific target
cargo run -- 8.8.8.8
```

## 🎯 Roadmap

- [ ] Implement speed test functionality
- [ ] Implement port scanner
- [ ] Multi-target monitoring
- [ ] Export session data to CSV
- [ ] Alert on threshold breach

## 📄 License

MIT License - See LICENSE file for details

## 🙏 Credits

Inspired by:

- [btop](https://github.com/aristocratos/btop) - System monitor with beautiful TUI
- [gping](https://github.com/orf/gping) - Rust-based ping grapher

Built with:

- [Ratatui](https://ratatui.rs/) - Terminal UI framework
- [surge-ping](https://docs.rs/surge-ping/) - Async ICMP pinging
- [Tokio](https://tokio.rs/) - Async runtime
