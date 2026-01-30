# RustyPing - Quick Start Guide 🚀

## First Time Setup (5 minutes)

### Option 1: One-Liner (Easiest)
Copy and paste this into PowerShell:
```powershell
iwr -useb https://raw.githubusercontent.com/pdzjtechnagy/RustyPing/main/web_install.ps1 | iex
```

### Option 2: Manual Setup

#### Step 1: Install Rust
Open PowerShell and run:
```powershell
winget install Rustlang.Rustup
```
**Close and reopen your terminal after this!**

### Step 2: Build the Program
In PowerShell, navigate to the project folder and run:
```powershell
cd C:\RustyPing
.\build.ps1
```
Wait 3-5 minutes (first time only - downloads everything).

### Step 3: Install Globally
```powershell
.\install.ps1
```
**Restart your terminal one more time!**

Done! Now you can use `rping` from anywhere.

---

## Daily Usage

### Basic Commands

**Ping a website:**
```powershell
rping google.com
```

**Ping an IP address:**
```powershell
rping 8.8.8.8
```

**Show your ping history:**
```powershell
rping --list
```

**Interactive mode (pick from recent targets):**
```powershell
rping
```
Just press Enter to use your last target, or type a number (1-5) to pick from the list.

---

## Inside the Program

### Keyboard Shortcuts

| Key | What It Does |
|-----|--------------|
| `Q` | Quit |
| `ESC` | Open settings menu |
| `S` | Run speed test |
| `P` | Port scan |
| `R` | Reset statistics |

### What You See

- **Top bar**: Target name and connection quality (EXCELLENT/GOOD/FAIR/POOR)
- **Big graph**: Real-time latency over time (braille dots = smooth lines)
- **Left panel**: Current ping, averages, min/max
- **Right panel**: Jitter (connection stability) and action buttons
- **Bottom bar**: Keyboard shortcuts and runtime

---

## Common Tasks

**Check if your internet is working:**
```
rping 8.8.8.8
```
(Green = good, Yellow = okay, Red = bad)

**Monitor your router:**
```
rping 192.168.1.1
```

**Check a specific website:**
```
rping github.com
```

**See what you've pinged before:**
```
rping --list
```

---

## Troubleshooting

**"rping: command not found"**
- Restart your terminal after running `install.ps1`
- Or use the full path: `C:\Users\YourName\.local\bin\rping.exe`

**"Permission denied"**
- Right-click PowerShell → "Run as Administrator"
- Windows needs admin rights for ping

**Graph shows boxes instead of smooth lines**
- Use Windows Terminal (not cmd.exe)
- Install from Microsoft Store if needed

---

That's it! Just run `rping` followed by an IP or website name. 🎉
