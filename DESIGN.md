# RustyPing UI/UX Design & Implementation Plan

## 1. Executive Summary
This document outlines the roadmap for transforming **RustyPing** from a single-screen monitoring tool into a comprehensive network diagnostic suite. The goal is to enhance usability, modularity, and visual customization while maintaining the high-performance, lightweight nature of the TUI.

## 2. UI/UX Design Principles
To guide this transformation, we adhere to the following core principles:

*   **Information Hierarchy**: Critical data (current latency, packet loss) must always be visible. Secondary data (history, jitter) should be accessible but not clutter the primary view.
*   **Keyboard-First Navigation**: As a TUI, all interactions must be drivable via keyboard shortcuts, with intuitive Vim-like (hjkl) or Arrow-key navigation.
*   **Immediate Feedback**: Every user action (mode switch, ping start) must have an immediate visual response, even if the network operation is async.
*   **Accessibility**: High-contrast modes (Monotone) and clear status indicators (symbols + colors) are mandatory for diverse terminal environments.
*   **Modularity**: Features are grouped into logical "Panels" or "Tabs" to prevent information overload.

## 3. Feature-Specific Panels & Menus

### 3.1 Network Diagnostics Panel
*Current State*: Port scanning and Speed tests are overlay modals.
*New Design*: A dedicated **Diagnostics Tab**.
*   **Sub-Panel: Ping Utility**: 
    *   Input field for quick target changes without restarting.
    *   Live scrolling log of individual ping responses (sequence, ttl, time).
*   **Sub-Panel: Port Scanner**:
    *   Dedicated form for Target IP, Start Port, End Port.
    *   Scrollable results list separating Open, Closed, and Filtered ports.
    *   Service detection (e.g., Port 80 -> HTTP).

### 3.2 Theme Management System
*Current State*: Hardcoded boolean toggle for Monotone/Color.
*New Design*: A robust **Theme Engine**.
*   **Theme Structure**: A serializable struct defining the entire color palette (Background, Foreground, Highlights, Alerts).
*   **Modes**:
    *   **Dark (Default)**: The current "Blacksite" look.
    *   **Light**: High-visibility theme for bright terminals.
    *   **Monotone**: Configurable base color (Amber, Green, White) for retro/e-ink displays.
    *   **Custom**: Load themes from `~/.config/rustyping/themes/*.toml`.

## 4. General UI/UX Improvements

### 4.1 Navigation (The "Tab System")
We will transition from a single-view app to a Tab-based architecture.
*   **Top Bar**: Always visible tabs: `[1] Monitor` | `[2] Diagnostics` | `[3] Settings`.
*   **Hotkeys**: `1-3` or `Tab` to switch views.

### 4.2 Visual Polish
*   **Status Bar**: A unified footer showing global app state (Uptime, Total Packets, Current Target).
*   **Borders**: Context-aware border colors (e.g., Red border when connection is critical).
*   **Responsiveness**: 
    *   **Full Mode**: Standard layout.
    *   **Compact Mode**: Auto-hide side panels on narrow terminals (< 80 cols).

## 5. Implementation Roadmap

### Phase 1: Architecture (Immediate)
*   [ ] Refactor `App` struct to support `AppTab` enum.
*   [ ] Create a basic Tab Bar in `ui.rs`.
*   [ ] Refactor `Theme` from static methods to a dynamic struct.

### Phase 2: Diagnostics Panel (Short Term)
*   [ ] Migrate Port Scanner from overlay to a dedicated Tab.
*   [ ] Add "Ping Log" view to Diagnostics.

### Phase 3: Advanced Theming (Medium Term)
*   [ ] Implement `ThemePalette` struct.
*   [ ] Add "Light Mode" preset.
*   [ ] Create Settings Tab UI for selecting themes.

## 6. Style Guide (Draft)

### Typography
*   **Headers**: Bold, Uppercase (e.g., **MONITOR**).
*   **Values**: Standard weight, bright color.
*   **Labels**: Dimmed color.

### Color Semantics
| Context | Default (Dark) | Monotone | Meaning |
|:---|:---|:---|:---|
| **Normal** | White/Grey | Base Color | Standard text |
| **Success** | Green | Base (Bold) | Good latency, Open port |
| **Warning** | Yellow | Base (Dim) | High latency, Filtered port |
| **Critical** | Red | Inverse/Striped | Packet loss, Closed port |
| **Selected** | Blue | Inverse | Focused item |

---
*This plan serves as the living document for the v2.3.0+ architecture.*
