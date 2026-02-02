use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetEntry {
    pub target: String,
    pub alias: Option<String>,
    pub last_used: DateTime<Utc>,
    pub total_sessions: u32,
    pub avg_latency: Option<f64>,
    pub success_rate: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub pause_ping_during_speedtest: bool,
    pub graph_history_length: usize,
    pub ping_interval_ms: u64,
    pub show_jitter_panel: bool,
    pub show_history_panel: bool,
    pub speedtest_provider: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            pause_ping_during_speedtest: true,
            graph_history_length: 200,
            ping_interval_ms: 500,
            show_jitter_panel: true,
            show_history_panel: true,
            speedtest_provider: "ookla".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TargetHistory {
    pub entries: Vec<TargetEntry>,
    pub favorites: Vec<String>,
    pub config: Config,
}

impl TargetHistory {
    fn config_path() -> Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not find config directory"))?
            .join("rustyping");
        
        fs::create_dir_all(&config_dir)?;
        Ok(config_dir.join("history.json"))
    }

    pub fn load() -> Result<Self> {
        let path = Self::config_path()?;
        
        if !path.exists() {
            return Ok(Self::default());
        }

        let contents = fs::read_to_string(path)?;
        let history: Self = serde_json::from_str(&contents)?;
        Ok(history)
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::config_path()?;
        let contents = serde_json::to_string_pretty(self)?;
        fs::write(path, contents)?;
        Ok(())
    }

    pub fn add_target(&mut self, target: &str) {
        if target == "--help" || target == "-h" || target == "?" {
            return;
        }

        // Check if target exists
        if let Some(entry) = self.entries.iter_mut().find(|e| e.target == target) {
            entry.last_used = Utc::now();
            entry.total_sessions += 1;
        } else {
            self.entries.push(TargetEntry {
                target: target.to_string(),
                alias: None,
                last_used: Utc::now(),
                total_sessions: 1,
                avg_latency: None,
                success_rate: None,
            });
        }

        // Sort by last used
        self.entries.sort_by(|a, b| b.last_used.cmp(&a.last_used));
    }

    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn print_recent(&self) {
        println!("\n┌─ Recent Targets ─────────────────────────┐");
        for (i, entry) in self.entries.iter().take(10).enumerate() {
            let alias = entry.alias.as_deref().unwrap_or("");
            let stats = if let (Some(lat), Some(sr)) = (entry.avg_latency, entry.success_rate) {
                format!(" ({lat:.1}ms, {sr:.1}%)")
            } else {
                String::new()
            };
            println!(
                "│ {}. {:<20} {:<15} {} │",
                i + 1,
                entry.target,
                alias,
                stats
            );
        }
        println!("└──────────────────────────────────────────┘\n");
    }

    pub fn update_stats(&mut self, target: &str, avg_latency: f64, success_rate: f64) {
        if let Some(entry) = self.entries.iter_mut().find(|e| e.target == target) {
            entry.avg_latency = Some(avg_latency);
            entry.success_rate = Some(success_rate);
        }
    }
}
