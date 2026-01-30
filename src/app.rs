use crate::network::{PingMonitor, SpeedTest, PortScanner};
use crate::storage::{Config, TargetHistory};
use anyhow::Result;
use std::time::{Duration, Instant};

pub struct App {
    pub target: String,
    pub ping_monitor: PingMonitor,
    pub start_time: Instant,
    pub last_ping: Instant,
    
    // UI State
    pub show_settings: bool,
    pub show_jitter: bool,
    pub show_history: bool,
    pub settings_selected: usize,
    
    // Features
    pub speedtest: Option<SpeedTest>,
    pub portscan: Option<PortScanner>,
    
    // Config
    pub config: Config,
}

impl App {
    pub async fn new(target: String) -> Result<Self> {
        let history = TargetHistory::load()?;
        let config = history.config.clone();
        
        let ping_monitor = PingMonitor::new(
            &target,
            config.graph_history_length,
        ).await?;

        Ok(Self {
            target,
            ping_monitor,
            start_time: Instant::now(),
            last_ping: Instant::now(),
            show_settings: false,
            show_jitter: config.show_jitter_panel,
            show_history: config.show_history_panel,
            settings_selected: 0,
            speedtest: None,
            portscan: None,
            config,
        })
    }

    pub async fn tick(&mut self) -> Result<()> {
        // Ping at configured interval
        if self.last_ping.elapsed() >= Duration::from_millis(self.config.ping_interval_ms) {
            self.ping_monitor.ping().await?;
            self.last_ping = Instant::now();
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
