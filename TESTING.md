# RustyPing 2.0 - Testing Guide 🧪

## Quick Test (2 minutes)

### Step 1: Build the Program
```powershell
cd C:\RustyPing
cargo build --release
```

### Step 2: Run Basic Ping Test
```powershell
.\target\release\rping.exe 8.8.8.8
```

**What to check:**
- ✅ Graph appears with braille dots (smooth lines, not boxes)
- ✅ Header shows "EXCELLENT" or "GOOD" in green
- ✅ Numbers update every second
- ✅ Press `Q` to quit

---

## Feature Testing

### Test 1: Speed Test (30 seconds)

1. Start monitoring:
   ```powershell
   .\target\release\rping.exe 8.8.8.8
   ```

2. Press `S` key

3. **Expected behavior:**
   - Overlay appears saying "Preparing..."
   - Then shows "Downloading..." with live speed
   - After 5-10 seconds, shows final result (e.g., "45.23 Mbps")
   - Press any key to close

**If it fails:**
- Check internet connection
- Try again (Cloudflare endpoint might be temporarily unavailable)

---

### Test 2: Port Scanner (10 seconds)

1. Start monitoring:
   ```powershell
   .\target\release\rping.exe 8.8.8.8
   ```

2. Press `P` key

3. **Expected behavior:**
   - Overlay appears showing "Scanning 8.8.8.8..."
   - Progress counter updates (1/17, 2/17, etc.)
   - Ports appear as they're scanned:
     - **GREEN "OPEN"** = Port is open
     - **GRAY "CLOSED"** = Port is closed
     - **YELLOW "FILTERED"** = Timeout (firewall blocking)
   - After ~10 seconds, shows "Scan complete!"
   - Press any key to close

**Test on your router:**
```powershell
.\target\release\rping.exe 192.168.1.1
```
Then press `P` - you should see more open ports (like 80, 443, 8080)

---

### Test 3: Dynamic Scaling (30 seconds)

1. Start the program:
   ```powershell
   .\target\release\rping.exe 8.8.8.8
   ```

2. **Resize your terminal window:**
   - Make it **wider** → Jitter panel should appear on the right
   - Make it **narrower** (<100 cols) → Jitter panel should hide
   - Make it **taller** → Graph gets more space
   - Make it **shorter** → Stats panel shrinks

3. **Try minimum size:**
   - Resize to 80x20 or smaller
   - Should show error message: "Terminal too small!"

**Expected:** UI adapts smoothly, no crashes, no layout breaking

---

### Test 4: Settings Menu

1. Start monitoring:
   ```powershell
   .\target\release\rping.exe 8.8.8.8
   ```

2. Press `ESC` key

3. **Expected:**
   - Settings overlay appears
   - Use `↑/↓` to navigate
   - Press `Enter` to toggle checkboxes
   - Press `ESC` to close

4. Toggle "Show Jitter Panel" → Right panel should appear/disappear

---

### Test 5: Keyboard Shortcuts

While monitoring, test all keys:

| Key | Expected Behavior |
|-----|------------------|
| `Q` | Quit immediately |
| `ESC` | Open/close settings |
| `S` | Start speed test (if not already running) |
| `P` | Start port scan (if not already running) |
| `R` | Reset all statistics |
| `J` | Toggle jitter panel visibility |
| `H` | Toggle history panel (if implemented) |

---

## Advanced Testing

### Test Multiple Targets

```powershell
# Test Google DNS
.\target\release\rping.exe 8.8.8.8

# Test Cloudflare DNS
.\target\release\rping.exe 1.1.1.1

# Test your router
.\target\release\rping.exe 192.168.1.1

# Test a website
.\target\release\rping.exe google.com
```

**Check:** Each target should save to history automatically

---

### Test History

```powershell
# Show history
.\target\release\rping.exe --list

# Interactive selection
.\target\release\rping.exe --select
```

**Expected:** Shows your recent targets with stats

---

### Test Edge Cases

1. **No internet:**
   - Disconnect WiFi/Ethernet
   - Run: `.\target\release\rping.exe 8.8.8.8`
   - Should show "OFFLINE" in red, graph shows no data

2. **Invalid target:**
   ```powershell
   .\target\release\rping.exe invalid.hostname.xyz
   ```
   - Should show error message and exit gracefully

3. **Very slow connection:**
   - Test on mobile hotspot or slow network
   - Should show higher latency (yellow/red)
   - Graph should still update smoothly

---

## Troubleshooting Tests

### "Permission denied" error
**Fix:** Run PowerShell as Administrator

### Speed test shows error
**Possible causes:**
- No internet connection
- Firewall blocking reqwest
- Cloudflare endpoint down (try again later)

### Port scan shows all "FILTERED"
**Normal for:** Public IPs (8.8.8.8) - they block most ports
**Test on:** Your local router (192.168.1.1) - should show open ports

### Graph shows boxes instead of smooth lines
**Fix:** Use Windows Terminal (not cmd.exe)

### UI looks broken when resizing
**Report:** This is a bug - should handle resize events smoothly

---

## Success Checklist ✅

After testing, you should have verified:

- [ ] Basic ping monitoring works
- [ ] Graph renders with braille (smooth lines)
- [ ] Speed test completes and shows result
- [ ] Port scan completes and shows results
- [ ] UI scales when resizing terminal
- [ ] Settings menu opens/closes
- [ ] All keyboard shortcuts work
- [ ] History saves and loads
- [ ] No crashes or errors

---

## Quick Test Script

Run this to test everything at once:

```powershell
# Build
cargo build --release

# Test 1: Basic ping (let it run 10 seconds, then Q)
.\target\release\rping.exe 8.8.8.8

# Test 2: Speed test (press S, wait, press any key)
.\target\release\rping.exe 8.8.8.8

# Test 3: Port scan (press P, wait, press any key)
.\target\release\rping.exe 8.8.8.8

# Test 4: History
.\target\release\rping.exe --list
```

That's it! If all tests pass, everything is working. 🎉


