#!/bin/bash

# ==============================================================================
# RustyPing Linux Portable Launcher v2.5.8
# This script downloads the latest release binary to a temp folder and runs it.
#
# Usage: curl -fsSL https://raw.githubusercontent.com/pdzjtechnagy/RustyPing/main/run_portable.sh | bash
# ==============================================================================

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
GRAY='\033[0;90m'
NC='\033[0m' # No Color

REPO="pdzjtechnagy/RustyPing"
TEMP_DIR="/tmp/rustyping_portable"
EXE_PATH="$TEMP_DIR/rping"

# 1. Setup Environment
mkdir -p "$TEMP_DIR"

echo -e "${CYAN}"
echo "  ╔════════════════════════════════════════════════════════════╗"
echo "  ║                RustyPing Portable Launcher                 ║"
echo "  ╚════════════════════════════════════════════════════════════╝"
echo -e "${NC}"

# 2. Check for Updates
echo -e "${GRAY}[*] Connecting to GitHub...${NC}"

# Get latest release data from GitHub API
RELEASE_DATA=$(curl -s "https://api.github.com/repos/$REPO/releases/latest")
TAG=$(echo "$RELEASE_DATA" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')

if [ -z "$TAG" ]; then
    echo -e "${RED}[-] Error: Could not fetch latest release info.${NC}"
    exit 1
fi

# Determine Architecture
ARCH=$(uname -m)
OS=$(uname -s | tr '[:upper:]' '[:lower:]')

case "$ARCH" in
    x86_64) 
        ARCH_PATTERN="amd64|x86_64|x64"
        PRIMARY_NAME="rustyping_linux_amd64"
        ;;
    aarch64|arm64) 
        ARCH_PATTERN="arm64|aarch64"
        PRIMARY_NAME="rustyping_linux_arm64"
        ;;
    *) 
        echo -e "${RED}[-] Unsupported architecture: $ARCH${NC}"
        exit 1 
        ;;
esac

# Try to find the best matching asset
echo -e "${GRAY}[*] Searching for $ARCH binary...${NC}"

# 1. Try primary name
DOWNLOAD_URL=$(echo "$RELEASE_DATA" | grep "browser_download_url" | grep "$PRIMARY_NAME" | head -n 1 | sed -E 's/.*"([^"]+)".*/\1/')

# 2. Try architecture pattern + linux
if [ -z "$DOWNLOAD_URL" ]; then
    DOWNLOAD_URL=$(echo "$RELEASE_DATA" | grep "browser_download_url" | grep -iE "$ARCH_PATTERN" | grep -i "linux" | head -n 1 | sed -E 's/.*"([^"]+)".*/\1/')
fi

# 3. Try just architecture pattern (if not windows)
if [ -z "$DOWNLOAD_URL" ]; then
    DOWNLOAD_URL=$(echo "$RELEASE_DATA" | grep "browser_download_url" | grep -iE "$ARCH_PATTERN" | grep -vE "\.exe|\.msi|\.zip" | head -n 1 | sed -E 's/.*"([^"]+)".*/\1/')
fi

# 4. Last resort: look for any asset named 'rping' or 'rustyping' that isn't a windows file
if [ -z "$DOWNLOAD_URL" ]; then
    DOWNLOAD_URL=$(echo "$RELEASE_DATA" | grep "browser_download_url" | grep -E "rping|rustyping" | grep -vE "\.exe|\.msi|\.zip" | head -n 1 | sed -E 's/.*"([^"]+)".*/\1/')
fi

if [ -z "$DOWNLOAD_URL" ]; then
    echo -e "${RED}[-] Error: Could not find a suitable binary for $ARCH in release $TAG.${NC}"
    echo -e "${YELLOW}[!] Available assets in $TAG:${NC}"
    echo "$RELEASE_DATA" | grep '"name":' | sed -E 's/.*"name": "([^"]+)".*/    - \1/'
    echo -e "${CYAN}[*] Tip: Ensure the Linux binary is uploaded to the GitHub release.${NC}"
    exit 1
fi

# Extract binary name from URL for cache check
ACTUAL_BINARY_NAME=$(basename "$DOWNLOAD_URL")

# Simple cache check (compare size if file exists)
NEEDS_DOWNLOAD=true
if [ -f "$EXE_PATH" ]; then
    LOCAL_SIZE=$(stat -c%s "$EXE_PATH")
    REMOTE_SIZE=$(echo "$RELEASE_DATA" | grep -A 10 "$ACTUAL_BINARY_NAME" | grep '"size":' | head -n 1 | sed -E 's/.*: ([0-9]+).*/\1/')
    
    if [ "$LOCAL_SIZE" -eq "$REMOTE_SIZE" ] && [ -n "$REMOTE_SIZE" ]; then
        echo -e "${GREEN}[+] Using cached version: $TAG${NC}"
        NEEDS_DOWNLOAD=false
    fi
fi

if [ "$NEEDS_DOWNLOAD" = true ]; then
    echo -e "${YELLOW}[*] Downloading RustyPing $TAG...${NC}"
    curl -L -o "$EXE_PATH" "$DOWNLOAD_URL"
    chmod +x "$EXE_PATH"
    echo -e "${GREEN}[+] Download complete!${NC}"
fi

# 3. Launch
echo -e "${CYAN}[*] Launching RustyPing...${NC}"
echo -e "${GRAY}------------------------------------------------------------${NC}"

# Run with pass-through arguments
"$EXE_PATH" "$@"

echo -e "${GRAY}------------------------------------------------------------${NC}"
echo -e "${CYAN}[+] Session ended.${NC}"
