mod ping;
mod portscan;
mod speedtest;

pub use ping::{start_ping_task, PingCommand, PingMonitor, PingResult, WebCheckStatus};
pub use portscan::{PortResult, PortScanner, PortStatus};
pub use speedtest::{SpeedTest, SpeedTestState};

#[derive(Debug, Clone)]
pub struct NetworkStats {
    pub current_response: Option<f64>,
    pub current_avg: f64,
    pub avg_response: f64,
    pub min_response: f64,
    pub max_response: f64,
    pub uptime_pct: f64,
    pub packet_loss_pct: f64,
    pub jitter: f64,
    pub stability: f64,
    pub quality: String,
    pub total_pings: u64,
    // New Metrics
    pub dns_duration: Option<f64>,
    pub tcp_port_80: WebCheckStatus,
    pub tcp_port_443: WebCheckStatus,
}

impl Default for NetworkStats {
    fn default() -> Self {
        Self {
            current_response: None,
            current_avg: 0.0,
            avg_response: 0.0,
            min_response: 0.0,
            max_response: 0.0,
            uptime_pct: 0.0,
            packet_loss_pct: 0.0,
            jitter: 0.0,
            stability: 100.0,
            quality: "UNKNOWN".to_string(),
            total_pings: 0,
            dns_duration: None,
            tcp_port_80: WebCheckStatus::Untested,
            tcp_port_443: WebCheckStatus::Untested,
        }
    }
}
