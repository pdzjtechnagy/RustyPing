use crate::network::{PingMonitor, SpeedTest, PortScanner, PingResult, PingCommand, start_ping_task};
use crate::storage::{Config, TargetHistory};
use anyhow::Result;
use std::time::{Duration, Instant};
use tokio::sync::mpsc;
use std::fs::{OpenOptions, File};
use std::io::{BufWriter, Write};
use chrono::Local;

use crate::theme::Theme;

#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(dead_code)]
pub enum AppTab {
    Monitor,
    Diagnostics,
    Settings,
}

pub struct App {
    pub target: String,
    pub ping_monitor: PingMonitor,
    pub start_time: Instant,
    pub last_ping: Instant,
    
    // Background Task Channels
    pub ping_tx: mpsc::Sender<PingCommand>,
    pub ping_rx: mpsc::Receiver<PingResult>,

    // Logging
    pub log_writer: Option<BufWriter<File>>,

    // UI State
    pub theme: Theme,
    #[allow(dead_code)]
    pub current_tab: AppTab,
    pub show_settings: bool,
    pub show_diagnostics: bool,
    pub show_jitter: bool,
    pub show_history: bool,
    pub enable_web_check: bool,
    pub settings_selected: usize,
    
    // Features
    pub speedtest: Option<SpeedTest>,
    pub portscan: Option<PortScanner>,
    
    // Config
    pub config: Config,
}

impl App {
    pub async fn new(target: String, log_file: Option<String>, monotone: bool) -> Result<Self> {
        let history = TargetHistory::load()?;
        let config = history.config.clone();
        
        // Start background ping task
        let (target_addr, ping_tx, ping_rx, dns_duration) = start_ping_task(&target).await?;

        let mut ping_monitor = PingMonitor::new(
            target_addr,
            config.graph_history_length,
        );
        ping_monitor.dns_duration = dns_duration;

        // Initialize logger if requested
        let log_writer = if let Some(path) = log_file {
            let file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(&path)?;
            let mut writer = BufWriter::new(file);
            // Write header if file is empty
            if std::fs::metadata(&path)?.len() == 0 {
                writeln!(writer, "Timestamp,Target,Latency(ms),Status")?;
            }
            Some(writer)
        } else {
            None
        };

        Ok(Self {
            target,
            ping_monitor,
            ping_tx,
            ping_rx,
            log_writer,
            start_time: Instant::now(),
            theme: if monotone { Theme::monotone() } else { Theme::blacksite() },
            current_tab: AppTab::Monitor,
            show_settings: false,
            show_diagnostics: false,
            show_jitter: config.show_jitter_panel,
            show_history: config.show_history_panel,
            enable_web_check: false,
            settings_selected: 0,
            speedtest: None,
            portscan: None,
            config,
        })
    }

    pub async fn tick(&mut self) -> Result<()> {
        // Ping interval is handled by the background task
        // We just process results here

        // Process incoming ping results
        while let Ok(result) = self.ping_rx.try_recv() {
            // Log result if enabled
            if let Some(writer) = &mut self.log_writer {
                let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
                let (ms, status) = match result {
                    PingResult::Success(v) => (v, "Success"),
                    PingResult::Timeout => (0.0, "Timeout"),
                    PingResult::WebCheck { .. } => (0.0, "WebCheck"), // Skip logging detailed web stats for now
                };
                if status != "WebCheck" {
                    writeln!(writer, "{},{},{:.2},{}", timestamp, self.target, ms, status).ok();
                }
            }

            self.ping_monitor.process_result(result);
        }

        // Update speedtest if running (don't auto-close, user must press C)
        if let Some(ref mut st) = self.speedtest {
            st.update().await?;
        }

        // Update portscan if running (don't auto-close, user must press C)
        if let Some(ref mut ps) = self.portscan {
            ps.update().await?;
        }

        Ok(())
    }

    pub fn toggle_settings(&mut self) {
        self.show_settings = !self.show_settings;
        if self.show_settings {
             self.show_diagnostics = false;
        }
    }

    pub fn toggle_diagnostics(&mut self) {
        self.show_diagnostics = !self.show_diagnostics;
        if self.show_diagnostics {
            self.show_settings = false;
        }
    }

    pub fn toggle_jitter_panel(&mut self) {
        self.show_jitter = !self.show_jitter;
    }

    pub fn toggle_history_panel(&mut self) {
        self.show_history = !self.show_history;
    }

    pub fn reset_stats(&mut self) {
        self.ping_monitor.reset();
        self.start_time = Instant::now();
    }

    pub async fn toggle_web_check(&mut self) {
        self.enable_web_check = !self.enable_web_check;
        let _ = self.ping_tx.send(PingCommand::ToggleWebCheck(self.enable_web_check)).await;
    }

    pub async fn start_speedtest(&mut self) -> Result<()> {
        if self.speedtest.is_none() {
            self.speedtest = Some(SpeedTest::new(&self.target).await?);
        }
        Ok(())
    }

    pub async fn start_portscan(&mut self) -> Result<()> {
        if self.portscan.is_none() {
            self.portscan = Some(PortScanner::new(&self.target).await?);
        }
        Ok(())
    }

    pub fn increase_history(&mut self) {
        // Increase by 10 seconds
        let new_len = self.config.graph_history_length + 10;
        if new_len <= 600 { // Max 10 minutes
            self.config.graph_history_length = new_len;
            self.ping_monitor.set_max_history(new_len);
        }
    }

    pub fn decrease_history(&mut self) {
        // Decrease by 10 seconds
        let new_len = self.config.graph_history_length.saturating_sub(10);
        if new_len >= 30 { // Min 30 seconds
            self.config.graph_history_length = new_len;
            self.ping_monitor.set_max_history(new_len);
        }
    }

    pub fn increase_speed(&mut self) {
        // Decrease interval = Faster pings
        // Min 50ms
        let new_interval = self.config.ping_interval_ms.saturating_sub(50);
        if new_interval >= 50 {
            self.config.ping_interval_ms = new_interval;
        }
    }

    pub fn decrease_speed(&mut self) {
        // Increase interval = Slower pings
        // Max 5000ms (5s)
        let new_interval = self.config.ping_interval_ms + 50;
        if new_interval <= 5000 {
            self.config.ping_interval_ms = new_interval;
        }
    }

    // TODO: Implement view switching when multi-target is added
    // pub fn switch_view(&mut self, _view: u8) {
    // }

    pub fn settings_navigate_up(&mut self) {
        if self.show_settings && self.settings_selected > 0 {
            self.settings_selected -= 1;
        }
    }

    pub fn settings_navigate_down(&mut self) {
        if self.show_settings {
            self.settings_selected = (self.settings_selected + 1).min(5);
        }
    }

    pub fn settings_toggle_selected(&mut self) {
        if !self.show_settings {
            return;
        }

        match self.settings_selected {
            0 => self.show_jitter = !self.show_jitter,
            1 => self.show_history = !self.show_history,
            2 => self.config.pause_ping_during_speedtest = !self.config.pause_ping_during_speedtest,
            _ => {}
        }
    }

    pub fn runtime(&self) -> Duration {
        self.start_time.elapsed()
    }
}
