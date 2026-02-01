use ratatui::style::Color;
use std::sync::atomic::{AtomicBool, Ordering};

static MONOTONE: AtomicBool = AtomicBool::new(false);

/// Blacksite Theme - Minimal / Professional / Dark
pub struct Theme;

#[allow(dead_code)]
impl Theme {
    pub fn set_monotone(enabled: bool) {
        MONOTONE.store(enabled, Ordering::Relaxed);
    }

    pub fn is_monotone() -> bool {
        MONOTONE.load(Ordering::Relaxed)
    }

    // Main colors
    pub fn bg() -> Color {
        if Self::is_monotone() { Color::Reset } else { Color::Rgb(14, 15, 17) }
    }
    
    pub fn fg() -> Color {
        if Self::is_monotone() { Color::Reset } else { Color::Rgb(201, 201, 201) }
    }
    
    pub fn title() -> Color {
        if Self::is_monotone() { Color::Reset } else { Color::Rgb(229, 229, 229) }
    }
    
    pub fn hi_fg() -> Color {
        if Self::is_monotone() { Color::White } else { Color::Rgb(255, 255, 255) }
    }
    
    // UI Elements
    pub fn divider() -> Color {
        if Self::is_monotone() { Color::DarkGray } else { Color::Rgb(42, 44, 47) }
    }
    
    pub fn selected_bg() -> Color {
        if Self::is_monotone() { Color::DarkGray } else { Color::Rgb(26, 28, 31) }
    }
    
    pub fn selected_fg() -> Color {
        if Self::is_monotone() { Color::White } else { Color::Rgb(255, 255, 255) }
    }
    
    pub fn box_color() -> Color {
        if Self::is_monotone() { Color::DarkGray } else { Color::Rgb(42, 44, 47) }
    }
    
    // Status Colors
    pub fn idle() -> Color {
        if Self::is_monotone() { Color::DarkGray } else { Color::Rgb(74, 77, 82) }
    }
    
    pub fn missed() -> Color {
        if Self::is_monotone() { Color::DarkGray } else { Color::Rgb(100, 100, 100) }
    }
    
    pub fn low() -> Color {
        if Self::is_monotone() { Color::Reset } else { Color::Rgb(143, 143, 143) }
    }
    
    pub fn medium() -> Color {
        if Self::is_monotone() { Color::White } else { Color::Rgb(199, 162, 74) }
    }
    
    pub fn high() -> Color {
        if Self::is_monotone() { Color::White } else { Color::Rgb(201, 75, 75) }
    }
    
    pub fn critical() -> Color {
        if Self::is_monotone() { Color::White } else { Color::Rgb(255, 59, 59) }
    }
    
    // Network specific (colors for text status)
    pub fn good() -> Color {
        if Self::is_monotone() { Color::Reset } else { Color::Rgb(74, 122, 74) }
    }
    
    pub fn warn() -> Color {
        if Self::is_monotone() { Color::White } else { Color::Rgb(199, 162, 74) }
    }
    
    pub fn crit() -> Color {
        if Self::is_monotone() { Color::White } else { Color::Rgb(255, 59, 59) }
    }
    
    // Graph colors
    // In monotone, we just return Reset for everything so it draws in default color.
    // The Braille characters themselves provide the density.
    pub const TEMP_START: Color = Color::Rgb(74, 122, 74);
    pub const TEMP_MID: Color = Color::Rgb(199, 162, 74);
    pub const TEMP_END: Color = Color::Rgb(255, 59, 59);
    
    pub fn key_highlight() -> Color {
        if Self::is_monotone() { Color::White } else { Color::Rgb(100, 200, 255) }
    }
    
    pub fn clock() -> Color {
        if Self::is_monotone() { Color::Reset } else { Color::Rgb(143, 143, 143) }
    }
    
    pub fn proc_user() -> Color {
        if Self::is_monotone() { Color::Reset } else { Color::Rgb(176, 176, 176) }
    }

    /// Get color for latency value
    pub fn latency_color(ms: f64) -> Color {
        if Self::is_monotone() {
            return Color::Reset;
        }
        
        if ms < 30.0 {
            Self::good()
        } else if ms < 100.0 {
            Self::warn()
        } else {
            Self::crit()
        }
    }

    /// Get gradient color for graph (0.0 to 1.0 ratio)
    pub fn graph_gradient(ratio: f64) -> Color {
        if Self::is_monotone() {
            return Color::Reset;
        }

        let ratio = ratio.clamp(0.0, 1.0);
        if ratio < 0.5 {
            // Interpolate between TEMP_START and TEMP_MID
            let t = ratio * 2.0;
            Self::interpolate(Self::TEMP_START, Self::TEMP_MID, t)
        } else {
            // Interpolate between TEMP_MID and TEMP_END
            let t = (ratio - 0.5) * 2.0;
            Self::interpolate(Self::TEMP_MID, Self::TEMP_END, t)
        }
    }

    fn interpolate(c1: Color, c2: Color, t: f64) -> Color {
        let t = t.clamp(0.0, 1.0);
        if let (Color::Rgb(r1, g1, b1), Color::Rgb(r2, g2, b2)) = (c1, c2) {
            Color::Rgb(
                (r1 as f64 + (r2 as f64 - r1 as f64) * t) as u8,
                (g1 as f64 + (g2 as f64 - g1 as f64) * t) as u8,
                (b1 as f64 + (b2 as f64 - b1 as f64) * t) as u8,
            )
        } else {
            c1
        }
    }

    /// Quality rating color
    pub fn quality_color(quality: &str) -> Color {
        if Self::is_monotone() {
            return Color::Reset;
        }
        match quality {
            "EXCELLENT" => Self::good(),
            "GOOD" => Self::good(),
            "FAIR" => Self::warn(),
            "POOR" => Self::high(),
            "OFFLINE" => Self::crit(),
            _ => Self::fg(),
        }
    }
}
