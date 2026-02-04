#!/bin/bash

# ==============================================================================
# RustyPing Linux Utility & Installer
# Inspired by Chris Titus's Linutil (https://github.com/ChrisTitusTech/linutil)
#
# This script provides a distro-agnostic way to install RustyPing and perform
# basic system optimizations.
#
# Usage: curl -fsSL https://raw.githubusercontent.com/pdzjtechnagy/RustyPing/main/linux_install.sh | sh
# ==============================================================================

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

REPO="pdzjtechnagy/RustyPing"
INSTALL_DIR="/usr/local/bin"
TEMP_DIR="/tmp/rustyping_install"

# --- 1. System Detection ---

detect_distro() {
    if [ -f /etc/os-release ]; then
        . /etc/os-release
        DISTRO=$ID
    elif type lsb_release >/dev/null 2>&1; then
        DISTRO=$(lsb_release -si | tr '[:upper:]' '[:lower:]')
    else
        DISTRO="unknown"
    fi
    echo "$DISTRO"
}

# --- 2. Utility Functions ---

print_banner() {
    clear
    echo -e "${CYAN}"
    echo "  ╔════════════════════════════════════════════════════════════╗"
    echo "  ║                RustyPing Linux Utility v2.5.4              ║"
    echo "  ╚════════════════════════════════════════════════════════════╝"
    echo -e "${NC}"
}

check_privileges() {
    if [ "$EUID" -ne 0 ]; then
        echo -e "${YELLOW}[!] This operation requires sudo privileges.${NC}"
        return 1
    fi
    return 0
}

# --- 3. Core Logic ---

install_rustyping() {
    echo -e "${CYAN}[*] Downloading latest RustyPing release...${NC}"
    
    # Get latest tag from GitHub API
    TAG=$(curl -s https://api.github.com/repos/$REPO/releases/latest | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')
    
    if [ -z "$TAG" ]; then
        echo -e "${RED}[-] Error: Could not fetch latest release tag.${NC}"
        exit 1
    fi

    echo -e "${GREEN}[+] Found version: $TAG${NC}"

    # Determine Architecture
    ARCH=$(uname -m)
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

    # Try to find the best matching asset from release data
    RELEASE_DATA=$(curl -s https://api.github.com/repos/$REPO/releases/latest)
    
    # 1. Try primary name
    DOWNLOAD_URL=$(echo "$RELEASE_DATA" | grep "browser_download_url" | grep "$PRIMARY_NAME" | head -n 1 | sed -E 's/.*"([^"]+)".*/\1/')
    
    # 2. Try architecture pattern + linux
    if [ -z "$DOWNLOAD_URL" ]; then
        DOWNLOAD_URL=$(echo "$RELEASE_DATA" | grep "browser_download_url" | grep -iE "$ARCH_PATTERN" | grep -i "linux" | head -n 1 | sed -E 's/.*"([^"]+)".*/\1/')
    fi
    
    # 3. Try just architecture pattern (excluding windows files)
    if [ -z "$DOWNLOAD_URL" ]; then
        DOWNLOAD_URL=$(echo "$RELEASE_DATA" | grep "browser_download_url" | grep -iE "$ARCH_PATTERN" | grep -vE "\.exe|\.msi|\.zip" | head -n 1 | sed -E 's/.*"([^"]+)".*/\1/')
    fi
    
    # 4. Last resort
    if [ -z "$DOWNLOAD_URL" ]; then
        DOWNLOAD_URL=$(echo "$RELEASE_DATA" | grep "browser_download_url" | grep -E "rping|rustyping" | grep -vE "\.exe|\.msi|\.zip" | head -n 1 | sed -E 's/.*"([^"]+)".*/\1/')
    fi

    if [ -z "$DOWNLOAD_URL" ]; then
        echo -e "${RED}[-] Error: Could not find a suitable binary for $ARCH in release $TAG.${NC}"
        echo -e "${YELLOW}[!] Available assets in $TAG:${NC}"
        echo "$RELEASE_DATA" | grep '"name":' | sed -E 's/.*"name": "([^"]+)".*/    - \1/'
        exit 1
    fi

    mkdir -p "$TEMP_DIR"
    echo -e "${CYAN}[*] Downloading from: $(basename "$DOWNLOAD_URL")...${NC}"
    
    if curl -L -o "$TEMP_DIR/rping" "$DOWNLOAD_URL"; then
        chmod +x "$TEMP_DIR/rping"
        echo -e "${YELLOW}[*] Installing to $INSTALL_DIR/rping (Requires Sudo)...${NC}"
        sudo mv "$TEMP_DIR/rping" "$INSTALL_DIR/rping"
        
        # Add capabilities for non-root ICMP if on Linux
        if [ "$(uname)" == "Linux" ]; then
            echo -e "${CYAN}[*] Setting network capabilities...${NC}"
            sudo setcap cap_net_raw+ep "$INSTALL_DIR/rping" || echo -e "${YELLOW}[!] Warning: Could not set capabilities. You may need sudo to run rping.${NC}"
        fi
        
        echo -e "${GREEN}[+] RustyPing installed successfully!${NC}"
    else
        echo -e "${RED}[-] Download failed. Please check your internet connection.${NC}"
        exit 1
    fi
}

optimize_system() {
    DISTRO=$(detect_distro)
    echo -e "${CYAN}[*] Optimizing for $DISTRO...${NC}"
    
    case "$DISTRO" in
        ubuntu|debian|kali|pop|linuxmint)
            echo -e "${YELLOW}[*] Updating APT cache and cleaning...${NC}"
            sudo apt update && sudo apt autoremove -y && sudo apt autoclean
            ;;
        fedora|rhel|centos)
            echo -e "${YELLOW}[*] Cleaning DNF cache...${NC}"
            sudo dnf clean all && sudo dnf makecache
            ;;
        arch|manjaro)
            echo -e "${YELLOW}[*] Cleaning Pacman cache...${NC}"
            sudo pacman -Sc --noconfirm
            ;;
        *)
            echo -e "${YELLOW}[!] No specific optimizations for $DISTRO yet.${NC}"
            ;;
    esac
    echo -e "${GREEN}[+] System maintenance complete.${NC}"
}

# --- 4. Main Menu ---

show_menu() {
    print_banner
    echo -e "1) ${GREEN}Install/Update RustyPing${NC}"
    echo -e "2) ${YELLOW}Run System Maintenance (Updates & Cleanup)${NC}"
    echo -e "3) ${CYAN}Show System Info${NC}"
    echo -e "q) Exit"
    echo
    read -p "Select an option: " CHOICE

    case "$CHOICE" in
        1) install_rustyping ;;
        2) optimize_system ;;
        3) 
            echo -e "${CYAN}--- System Information ---${NC}"
            uname -a
            detect_distro
            ;;
        q) exit 0 ;;
        *) echo -e "${RED}Invalid option.${NC}"; sleep 1; show_menu ;;
    esac
}

# Run the menu
show_menu
