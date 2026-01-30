use ratatui::style::Color;

/// Blacksite Theme - Minimal / Professional / Dark
pub struct Theme;

impl Theme {
    // Main colors
    pub const BG: Color = Color::Rgb(14, 15, 17);           // #0e0f11
    pub const FG: Color = Color::Rgb(201, 201, 201);        // #c9c9c9
    pub const TITLE: Color = Color::Rgb(229, 229, 229);     // #e5e5e5
    pub const HI_FG: Color = Color::Rgb(255, 255, 255);     // #ffffff
    
    // UI Elements
    pub const DIVIDER: Color = Color::Rgb(42, 44, 47);      // #2a2c2f
    pub const SELECTED_BG: Color = Color::Rgb(26, 28, 31);  // #1a1c1f
    pub const SELECTED_FG: Color = Color::Rgb(255, 255, 255); // #ffffff
    pub const BOX: Color = Color::Rgb(42, 44, 47);          // #2a2c2f
    
    // Status Colors
    pub const IDLE: Color = Color::Rgb(74, 77, 82);         // #4a4d52
    pub const LOW: Color = Color::Rgb(143, 143, 143);       // #8f8f8f
    pub const MEDIUM: Color = Color::Rgb(199, 162, 74);     // #c7a24a
    pub const HIGH: Color = Color::Rgb(201, 75, 75);        // #c94b4b
    pub const CRITICAL: Color = Color::Rgb(255, 59, 59);    // #ff3b3b
    
    // Network specific
    pub const GOOD: Color = Color::Rgb(74, 122, 74);        // #4a7a4a (restrained green)
    pub const WARN: Color = Color::Rgb(199, 162, 74);       // #c7a24a (muted yellow)
    pub const CRIT: Color = Color::Rgb(255, 59, 59);        // #ff3b3b (hard red)
    
    // Graph colors
    pub const GRAPH_BG: Color = Color::Rgb(14, 15, 17);     // #0e0f11
    pub const TEMP_START: Color = Color::Rgb(74, 122, 74);  // #4a7a4a
    pub const TEMP_MID: Color = Color::Rgb(199, 162, 74);   // #c7a24a
    pub const TEMP_END: Color = Color::Rgb(255, 59, 59);    // #ff3b3b
    
    // Misc
    pub const CLOCK: Color = Color::Rgb(143, 143, 143);     // #8f8f8f
    pub const PROC_USER: Color = Color::Rgb(176, 176, 176); // #b0b0b0

    /// Get color for latency value
    pub fn latency_color(ms: f64) -> Color {
        if ms < 30.0 {
            Self::GOOD
        } else if ms < 100.0 {
            Self::WARN
        } else {
            Self::CRIT
        }
    }

    /// Get gradient color for graph (0.0 to 1.0)
    pub fn graph_gradient(ratio: f64) -> Color {
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
        match quality {
            "EXCELLENT" => Self::GOOD,
            "GOOD" => Self::GOOD,
            "FAIR" => Self::WARN,
            "POOR" => Self::HIGH,
            "OFFLINE" => Self::CRIT,
            _ => Self::FG,
        }
    }
}
