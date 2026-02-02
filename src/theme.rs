use ratatui::style::Color;
use std::sync::atomic::{AtomicBool, Ordering};

pub static MONOTONE: AtomicBool = AtomicBool::new(false);

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct Theme {
    pub name: String,
    pub bg: Color,
    pub fg: Color,
    pub title: Color,
    pub hi_fg: Color,
    pub divider: Color,
    pub selected_bg: Color,
    pub selected_fg: Color,
    pub box_color: Color,
    pub idle: Color,
    pub missed: Color,
    pub low: Color,
    pub medium: Color,
    pub high: Color,
    pub critical: Color,
    pub good: Color,
    pub warn: Color,
    pub crit: Color,
    pub key_highlight: Color,
    pub clock: Color,
    pub proc_user: Color,
    // Graph
    pub graph_low: Color,
    pub graph_mid: Color,
    pub graph_high: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self::blacksite()
    }
}

impl Theme {
    pub fn set_monotone(enabled: bool) {
        MONOTONE.store(enabled, Ordering::Relaxed);
    }

    #[allow(dead_code)]
    pub fn is_monotone_enabled() -> bool {
        MONOTONE.load(Ordering::Relaxed)
    }

    pub fn blacksite() -> Self {
        Self {
            name: "Blacksite".to_string(),
            bg: Color::Rgb(14, 15, 17),
            fg: Color::Rgb(201, 201, 201),
            title: Color::Rgb(229, 229, 229),
            hi_fg: Color::Rgb(255, 255, 255),
            divider: Color::Rgb(42, 44, 47),
            selected_bg: Color::Rgb(26, 28, 31),
            selected_fg: Color::Rgb(255, 255, 255),
            box_color: Color::Rgb(42, 44, 47),
            idle: Color::Rgb(74, 77, 82),
            missed: Color::Rgb(100, 100, 100),
            low: Color::Rgb(143, 143, 143),
            medium: Color::Rgb(199, 162, 74),
            high: Color::Rgb(201, 75, 75),
            critical: Color::Rgb(255, 59, 59),
            good: Color::Rgb(74, 122, 74),
            warn: Color::Rgb(199, 162, 74),
            crit: Color::Rgb(255, 59, 59),
            key_highlight: Color::Rgb(100, 200, 255),
            clock: Color::Rgb(143, 143, 143),
            proc_user: Color::Rgb(176, 176, 176),
            graph_low: Color::Rgb(74, 122, 74),
            graph_mid: Color::Rgb(199, 162, 74),
            graph_high: Color::Rgb(255, 59, 59),
        }
    }

    pub fn monotone() -> Self {
        Self {
            name: "Monotone".to_string(),
            bg: Color::Reset,
            fg: Color::Reset,
            title: Color::Reset,
            hi_fg: Color::White,
            divider: Color::DarkGray,
            selected_bg: Color::DarkGray,
            selected_fg: Color::White,
            box_color: Color::DarkGray,
            idle: Color::DarkGray,
            missed: Color::DarkGray,
            low: Color::Reset,
            medium: Color::White,
            high: Color::White,
            critical: Color::White,
            good: Color::Reset,
            warn: Color::White,
            crit: Color::White,
            key_highlight: Color::White,
            clock: Color::Reset,
            proc_user: Color::Reset,
            graph_low: Color::Reset,
            graph_mid: Color::White,
            graph_high: Color::White,
        }
    }

    pub fn latency_color(&self, latency: f64) -> Color {
        if latency < 50.0 {
            self.good
        } else if latency < 150.0 {
            self.warn
        } else {
            self.crit
        }
    }

    pub fn quality_color(&self, quality: &str) -> Color {
        match quality {
            "Excellent" => self.good,
            "Good" => self.good,
            "Fair" => self.warn,
            "Poor" => self.warn,
            "Critical" => self.crit,
            _ => self.idle,
        }
    }

    pub fn graph_gradient(&self, ratio: f64) -> Color {
        if ratio < 0.4 {
            self.graph_low
        } else if ratio < 0.7 {
            self.graph_mid
        } else {
            self.graph_high
        }
    }
}
