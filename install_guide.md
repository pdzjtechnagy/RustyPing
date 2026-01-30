# RustyPing 2.0 - Complete Installation Guide

> **NOTE:** This is the manual installation guide. For the easiest setup, see the **One-Liner** method in `README.md` or `quickstart.md`.

## 📋 What You're Building

**RustyPing 2.0** - A professional network monitoring tool with:

- ✅ btop-style braille graphs
- ✅ Blacksite theme (minimal, professional)
- ✅ Smart IP history with fuzzy find
- ✅ On-demand speed tests
- ✅ Port scanning capability
- ✅ Keyboard-driven interface

## 🚀 Step-by-Step Installation (Fresh Windows 11 25H2)

### Step 1: Install Prerequisites

```cmd
# Install Rust
winget install Rustlang.Rustup

# Install Visual Studio Build Tools (if needed)
winget install Microsoft.VisualStudio.2022.BuildTools
```

**Close and reopen your terminal after installing Rust!**

### Step 2: Create Project Structure

```cmd
mkdir C:\PINGTOOL\rustyping2
cd C:\PINGTOOL\rustyping2
mkdir src
mkdir src\network
```

### Step 3: Copy All Files

You need to copy these files from the artifacts:

#### Root Directory (`C:\PINGTOOL\rustyping2\`)

1. ✅ `Cargo.toml` - Project configuration
2. ✅ `build_rustyping.bat` - Build script
3. ✅ `RUSTYPING_README.md` - Documentation
4. ✅ `INSTALL_GUIDE.md` - This file

#### src Directory (`C:\PINGTOOL\rustyping2\src\`)

5. ✅ `main.rs` - Entry point
6. ✅ `app.rs` - Core application logic
7. ✅ `ui.rs` - TUI rendering with braille
8. ✅ `theme.rs` - Blacksite color scheme
9. ✅ `storage.rs` - History & config management
10. ⚠️ `config.rs` - **CREATE THIS** (see below)

#### src/network Directory (`C:\PINGTOOL\rustyping2\src\network\`)

11. ✅ `mod.rs` - Network module
12. ⚠️ `ping.rs` - **CREATE THIS** (see below)
13. ⚠️ `speedtest.rs` - **CREATE THIS** (see below)
14. ⚠️ `portscan.rs` - **CREATE THIS** (see below)

### Step 4: Create Missing Files

#### File: `src/config.rs`

```rust
// Placeholder - will be expanded later
pub struct Config;
```

#### File: `src/network/ping.rs`

```rust
use super::NetworkStats;
use anyhow::Result;
use std::collections::VecDeque;
use std::net::IpAddr;
use surge_ping::{Client, Config, PingIdentifier, PingSequence};
use std::time::Duration;

pub struct PingMonitor {
    client: Client,
    target: String,
    target_addr: IpAddr,
    history: VecDeque<Option<f64>>,
    recent: VecDeque<f64>,
    max_history: usize,
    total_pings: u64,
    successful_pings: u64,
    failed_pings: u64,
}

impl PingMonitor {
    pub async fn new(target: &str, max_history: usize) -> Result<Self> {
        let target_addr: IpAddr = if let Ok(addr) = target.parse() {
            addr
        } else {
            use tokio::net::lookup_host;
            let mut addrs = lookup_host(format!("{}:0", target)).await?;
            addrs.next().ok_or_else(|| anyhow::anyhow!("Could not resolve hostname"))?.ip()
        };

        let config = Config::default();
        let client = Client::new(&config)?;

        Ok(Self {
            client,
            target: target.to_string(),
            target_addr,
            history: VecDeque::with_capacity(max_history),
            recent: VecDeque::with_capacity(10),
            max_history,
            total_pings: 0,
            successful_pings: 0,
            failed_pings: 0,
        })
    }

    pub async fn ping(&mut self) -> Result<()> {
        let mut pinger = self.client.pinger(self.target_addr, PingIdentifier(rand::random())).await;
        pinger.timeout(Duration::from_secs(1));

        self.total_pings += 1;

        match pinger.ping(PingSequence(0), &[]).await {
            Ok((_, duration)) => {
                let ms = duration.as_secs_f64() * 1000.0;
                self.successful_pings += 1;
                self.history.push_back(Some(ms));
                self.recent.push_back(ms);

                if self.recent.len() > 10 {
                    self.recent.pop_front();
                }
            }
            Err(_) => {
                self.failed_pings += 1;
                self.history.push_back(None);
            }
        }

        if self.history.len() > self.max_history {
            self.history.pop_front();
        }

        Ok(())
    }

    pub fn latency_data(&self) -> &VecDeque<Option<f64>> {
        &self.history
    }

    pub fn stats(&self) -> NetworkStats {
        let valid: Vec<f64> = self.history.iter().filter_map(|&x| x).collect();
        let recent_valid: Vec<f64> = self.recent.iter().copied().collect();

        let current_response = self.history.back().and_then(|&x| x);
        let current_avg = if !recent_valid.is_empty() {
            recent_valid.iter().sum::<f64>() / recent_valid.len() as f64
        } else {
            0.0
        };

        let avg_response = if !valid.is_empty() {
            valid.iter().sum::<f64>() / valid.len() as f64
        } else {
            0.0
        };

        let min_response = valid.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let max_response = valid.iter().fold(0.0, |a, &b| a.max(b));

        let uptime_pct = if self.total_pings > 0 {
            (self.successful_pings as f64 / self.total_pings as f64) * 100.0
        } else {
            0.0
        };

        let packet_loss_pct = if self.total_pings > 0 {
            (self.failed_pings as f64 / self.total_pings as f64) * 100.0
        } else {
            0.0
        };

        // Calculate jitter (standard deviation of latency)
        let jitter = if valid.len() > 1 {
            let mean = avg_response;
            let variance = valid.iter().map(|&x| (x - mean).powi(2)).sum::<f64>() / valid.len() as f64;
            variance.sqrt()
        } else {
            0.0
        };

        let stability = (100.0 - (jitter / avg_response.max(1.0) * 100.0)).max(0.0).min(100.0);

        let quality = if current_response.is_none() {
            "OFFLINE".to_string()
        } else if current_avg < 30.0 {
            "EXCELLENT".to_string()
        } else if current_avg < 100.0 {
            "GOOD".to_string()
        } else if current_avg < 200.0 {
            "FAIR".to_string()
        } else {
            "POOR".to_string()
        };

        NetworkStats {
            current_response,
            current_avg,
            avg_response,
            min_response,
            max_response,
            uptime_pct,
            packet_loss_pct,
            jitter,
            stability,
            quality,
        }
    }

    pub fn quality(&self) -> String {
        self.stats().quality
    }

    pub fn reset(&mut self) {
        self.history.clear();
        self.recent.clear();
        self.total_pings = 0;
        self.successful_pings = 0;
        self.failed_pings = 0;
    }
}
```

#### File: `src/network/speedtest.rs`

```rust
use anyhow::Result;

pub struct SpeedTest {
    // TODO: Implement speedtest
}

impl SpeedTest {
    pub async fn new(_target: &str) -> Result<Self> {
        Ok(Self {})
    }

    pub async fn update(&mut self) -> Result<bool> {
        // Returns true when complete
        Ok(true)
    }
}
```

#### File: `src/network/portscan.rs`

```rust
use anyhow::Result;

pub struct PortScanner {
    // TODO: Implement port scanner
}

impl PortScanner {
    pub async fn new(_target: &str) -> Result<Self> {
        Ok(Self {})
    }

    pub async fn update(&mut self) -> Result<bool> {
        // Returns true when complete
        Ok(true)
    }
}
```

### Step 5: Verify File Structure

Your directory should look like:

```
C:\PINGTOOL\rustyping2\
├── Cargo.toml
├── build_rustyping.bat
├── RUSTYPING_README.md
├── INSTALL_GUIDE.md
└── src/
    ├── main.rs
    ├── app.rs
    ├── ui.rs
    ├── theme.rs
    ├── storage.rs
    ├── config.rs
    └── network/
        ├── mod.rs
        ├── ping.rs
        ├── speedtest.rs
        └── portscan.rs
```

### Step 6: Build

```cmd
cd C:\PINGTOOL\rustyping2
.\build_rustyping.bat
```

**First build takes 3-5 minutes!** It downloads and compiles all dependencies.

### Step 7: Test Run

```cmd
# Run with target
.\target\release\rping.exe 8.8.8.8

# Or run interactively
.\target\release\rping.exe
```

### Step 8: Create Launcher (Optional)

```cmd
# Create rping.bat in the rustyping2 directory
echo @.\target\release\rping.exe %* > rping.bat

# Now you can run:
.\rping.bat 8.8.8.8
```

### Step 9: Add to PATH (Optional)

```cmd
# Add C:\PINGTOOL\rustyping2 to your PATH
# Then you can run from anywhere:
rping 8.8.8.8
```

## ⚠️ Troubleshooting

### Build Error: "linker link.exe not found"

```cmd
# Install Visual Studio Build Tools
winget install Microsoft.VisualStudio.2022.BuildTools

# OR switch to GNU toolchain
rustup default stable-x86_64-pc-windows-gnu
```

### Build Error: "cannot find module"

- Check all files are in correct locations
- Verify `src/network/mod.rs` exists
- Ensure all `*.rs` files are properly named

### Runtime Error: "Permission denied"

```cmd
# Run as Administrator (ICMP requires privileges)
Start-Process powershell -Verb RunAs
cd C:\PINGTOOL\rustyping2
.\target\release\rping.exe
```

### Braille characters display as boxes

- Use Windows Terminal (not cmd.exe)
- Install a modern font (Cascadia Code, JetBrains Mono)

## ✅ Success Indicators

You'll know it works when you see:

1. ✅ **Smooth braille graph** in the latency panel
2. ✅ **Blacksite colors** (dark theme, muted colors)
3. ✅ **Real-time updates** every second
4. ✅ **History persists** between sessions (`rping` shows last targets)
5. ✅ **ESC menu** opens settings overlay

## 🎯 Next Steps

Once working:

1. Try `rping --list` to see history
2. Press `ESC` to open settings
3. Press `S` to run speed test (stub for now)
4. Press `P` to run port scan (stub for now)
5. Monitor different targets to build history

## 📝 Status: v2.0 Core Complete

✅ Working:

- Braille graphs
- Ping monitoring
- Jitter calculation
- History & fuzzy find
- Settings menu
- Blacksite theme

⚠️ Stubs (to implement later):

- Speed test (shows message, doesn't actually test)
- Port scan (shows message, doesn't actually scan)
- Multi-target tabs

---

**You now have a working btop-style network monitor!** 🎉
