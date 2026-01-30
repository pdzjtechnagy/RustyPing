# RustyPing 2.0 - Quick Start 🚀

## TL;DR - Get It Running

```cmd
# 1. Install Rust
winget install Rustlang.Rustup

# 2. Close terminal, reopen, then:
mkdir C:\PINGTOOL\rustyping2
cd C:\PINGTOOL\rustyping2

# 3. Copy ALL 14 files from artifacts to correct locations

# 4. Build
.\build_rustyping.bat

# 5. Run
.\target\release\rping.exe 8.8.8.8
```

## 📦 Files You Need (14 total)

### Root (`C:\PINGTOOL\rustyping2\`)

1. `Cargo.toml`
2. `build_rustyping.bat`
3. `RUSTYPING_README.md`
4. `INSTALL_GUIDE.md`
5. `QUICKSTART.md` (this file)

### src (`C:\PINGTOOL\rustyping2\src\`)

6. `main.rs`
7. `app.rs`
8. `ui.rs`
9. `theme.rs`
10. `storage.rs`

### src/network (`C:\PINGTOOL\rustyping2\src\network\`)

11. `mod.rs`

### Files to CREATE yourself:

12. `src/config.rs` - Just put: `pub struct Config;`
13. `src/network/ping.rs` - Full code in INSTALL_GUIDE.md
14. `src/network/speedtest.rs` - Stub in INSTALL_GUIDE.md
15. `src/network/portscan.rs` - Stub in INSTALL_GUIDE.md

## 🎯 What You're Getting

**A professional network monitor with:**

- ⣿⣿⣿ Braille graphs (btop-style, high-resolution)
- 🎨 Blacksite theme (dark, minimal, professional)
- 📊 Real-time latency monitoring (<1s detection)
- 📈 Jitter calculation (connection stability)
- 💾 Smart history (fuzzy find previous targets)
- ⚙️ Settings menu (ESC key)
- ⌨️ Keyboard-driven (Q/ESC/S/P/J/H/M/R)

## 🔑 Key Commands

```
rping                    # Interactive (shows last 5)
rping 8.8.8.8            # Direct target
rping google.com         # Hostname
rping --list             # Show all history
rping --select           # Force selection menu

# Inside the TUI:
Q          Quit
ESC        Settings
S          Speed test (stub)
P          Port scan (stub)
J          Toggle jitter
H          Toggle history
R          Reset stats
```

## ⚡ Expected Build Time

- **First build:** 3-5 minutes (downloads & compiles ~108 crates)
- **Subsequent builds:** 5-10 seconds (only recompiles changes)
- **Binary size:** ~3-5 MB (release build, stripped)

## ✅ Success Checklist

After running, you should see:

```
┌─ RustyPing | 8.8.8.8 | ● EXCELLENT ─────┐
│ Runtime: 00:00:15  │  ESC:Settings     │
├─────────────────────────────────────────┤
│                                         │
│  LATENCY (last 60s)                     │
│  50ms ┤⢀⠀⠀⡠⠊⠉⠑⠒⠤⣀⠀⠀⠀⠀⠀│  <- BRAILLE!
│  40ms ┤⠈⠢⡠⠊⠀⠀⠀⠀⠀⠈⠑⠢⣀│
│  30ms ┤⠀⠀⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀│
│       └────────────────────────────     │
```

If you see blocky ASCII instead of smooth curves, your terminal doesn't support braille - use Windows Terminal!

## 🐛 Quick Fixes

**"linker link.exe not found"**

```cmd
winget install Microsoft.VisualStudio.2022.BuildTools
```

**"cannot find module config"**

```cmd
# Create missing file:
echo pub struct Config; > src\config.rs
```

**"Permission denied" when running**

```cmd
# Run as admin:
Start-Process powershell -Verb RunAs
```

**Braille shows as boxes**

- Use Windows Terminal (not cmd.exe)
- Install modern font: `winget install Cascadia.CascadiaCode`

## 📊 What Works Now (v2.0 Core)

✅ **Fully Functional:**

- Real-time ping monitoring
- Braille latency graphs
- Jitter & stability calculation
- Statistics (min/avg/max)
- Quality rating (EXCELLENT → OFFLINE)
- Blacksite theme colors
- History persistence
- Fuzzy find targets
- Settings menu (ESC)
- Keyboard controls

⚠️ **Stubs (Implement Later):**

- Speed test (press S - shows message only)
- Port scan (press P - shows message only)
- Multi-target tabs (press M - not yet implemented)

## 🎨 Why It Looks Amazing

**btop comparison:**

```
btop CPU graph:  ⢠⣀⣀⡀⠀⠀⣀⡠⠤⠤⣀
RustyPing:       ⢠⣀⣀⡀⠀⠀⣀⡠⠤⠤⣀  <- SAME!
```

Both use Unicode Braille (U+2800-28FF) for 2×4 pixel characters.
= **4x more resolution than ASCII**

## 🚀 Next Steps After Build

1. **Test it:**
   
   ```cmd
   rping 8.8.8.8
   # Should see smooth braille graph updating every second
   ```

2. **Build history:**
   
   ```cmd
   rping 1.1.1.1
   rping google.com
   rping --list    # See all targets
   ```

3. **Explore features:**
   
   - Press `ESC` → Settings menu
   - Press `J` → Toggle jitter panel
   - Press `R` → Reset statistics

4. **Make it permanent:**
   
   ```cmd
   # Add to PATH or create desktop shortcut
   copy .\target\release\rping.exe C:\Windows\System32\
   ```

## 💡 Pro Tips

- Use `rping` alone to quick-select from history
- Press `ESC` frequently - settings are your friend
- Watch jitter % - important for VoIP/gaming
- Quality score updates in real-time
- All data persists in `%APPDATA%\rustyping\history.json`

## 📝 File Checklist

Before building, verify:

```cmd
# Should return 5 files
dir Cargo.toml build_rustyping.bat *.md

# Should return 5 files  
dir src\*.rs

# Should return 4 files
dir src\network\*.rs
```

Total: **14 files minimum** to build successfully.

---

**Ready? Run `.\build_rustyping.bat` and watch the magic! 🦀**

Estimated time: 5 minutes to first successful run.
