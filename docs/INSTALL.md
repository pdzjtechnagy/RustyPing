# Installation Guide

## Windows

### One-Liner (PowerShell)
```powershell
iwr -useb https://raw.githubusercontent.com/pdzjtechnagy/RustyPing/main/install.ps1 | iex
```
*(Note: Requires `install.ps1` to be hosted in your repository)*

### Winget (Once submitted)
```powershell
winget install rustyping
```

### Manual
Download the `.zip` from the [Releases](https://github.com/pdzjtechnagy/RustyPing/releases) page and extract it.

---

## Linux (Debian/Ubuntu)

### One-Liner
```bash
wget https://github.com/pdzjtechnagy/RustyPing/releases/latest/download/rustyping_amd64.deb
sudo dpkg -i rustyping_amd64.deb
```

### Manual
Download the `.deb` file from the [Releases](https://github.com/pdzjtechnagy/RustyPing/releases) page.

---

## macOS

### Homebrew (One-Liner)
```bash
brew install https://raw.githubusercontent.com/pdzjtechnagy/RustyPing/main/packaging/macos/rustyping.rb
```

### Custom Tap
```bash
brew tap pdzjtechnagy/tap
brew install rustyping
```
