use super::NetworkStats;
use anyhow::Result;
use std::collections::VecDeque;
use std::net::IpAddr;
use surge_ping::{Client, Config, PingIdentifier, PingSequence};
use std::time::Duration;

pub struct PingMonitor {
    client: Client,
    _target: String,
    target_addr: IpAddr,
    history: VecDeque<Option<f64>>,
    recent: VecDeque<f64>,
    max_history: usize,
    total_pings: u64,
    successful_pings: u64,
    failed_pings: u64,
}

impl PingMonitor {
    pub async fn new(target: &str, max_history: usize) -> Result<Self> {
        let target_addr: IpAddr = if let Ok(addr) = target.parse() {
            addr
        } else {
            use tokio::net::lookup_host;
            let mut addrs = lookup_host(format!("{}:0", target)).await?;
            addrs.next().ok_or_else(|| anyhow::anyhow!("Could not resolve hostname"))?.ip()
        };

        let config = Config::default();
        let client = Client::new(&config)?;

        Ok(Self {
            client,
            _target: target.to_string(),
            target_addr,
            history: VecDeque::with_capacity(max_history),
            recent: VecDeque::with_capacity(10),
            max_history,
            total_pings: 0,
            successful_pings: 0,
            failed_pings: 0,
        })
    }

    pub async fn ping(&mut self) -> Result<()> {
        let mut pinger = self.client.pinger(self.target_addr, PingIdentifier(rand::random())).await;
        
        self.total_pings += 1;

        // Use tokio timeout to ensure we don't hang
        let payload = [0; 8];
        match tokio::time::timeout(Duration::from_secs(1), pinger.ping(PingSequence(0), &payload)).await {
            Ok(Ok((_, duration))) => {
                let ms = duration.as_secs_f64() * 1000.0;
                self.successful_pings += 1;
                self.history.push_back(Some(ms));
                self.recent.push_back(ms);

                if self.recent.len() > 10 {
                    self.recent.pop_front();
                }
            }
            _ => {
                // Timeout or Error
                self.failed_pings += 1;
                self.history.push_back(None);
            }
        }

        if self.history.len() > self.max_history {
            self.history.pop_front();
        }

        Ok(())
    }

    pub fn latency_data(&self) -> &VecDeque<Option<f64>> {
        &self.history
    }

    pub fn stats(&self) -> NetworkStats {
        let valid: Vec<f64> = self.history.iter().filter_map(|&x| x).collect();
        let recent_valid: Vec<f64> = self.recent.iter().copied().collect();

        let current_response = self.history.back().and_then(|&x| x);
        let current_avg = if !recent_valid.is_empty() {
            recent_valid.iter().sum::<f64>() / recent_valid.len() as f64
        } else {
            0.0
        };

        let avg_response = if !valid.is_empty() {
            valid.iter().sum::<f64>() / valid.len() as f64
        } else {
            0.0
        };

        let min_response = if valid.is_empty() {
            0.0
        } else {
            valid.iter().fold(f64::INFINITY, |a, &b| a.min(b))
        };
        
        let max_response = if valid.is_empty() {
            0.0_f64
        } else {
            valid.iter().fold(0.0_f64, |a, &b| a.max(b))
        };

        let uptime_pct = if self.total_pings > 0 {
            (self.successful_pings as f64 / self.total_pings as f64) * 100.0
        } else {
            0.0
        };

        let packet_loss_pct = if self.total_pings > 0 {
            (self.failed_pings as f64 / self.total_pings as f64) * 100.0
        } else {
            0.0
        };

        // Calculate jitter (standard deviation of latency)
        let jitter = if valid.len() > 1 {
            let mean = avg_response;
            let variance = valid.iter().map(|&x| (x - mean).powi(2)).sum::<f64>() / valid.len() as f64;
            variance.sqrt()
        } else {
            0.0
        };

        let stability = if avg_response > 0.0 {
            (100.0 - (jitter / avg_response * 100.0)).max(0.0).min(100.0)
        } else {
            100.0
        };

        let quality = if current_response.is_none() {
            "OFFLINE".to_string()
        } else if current_avg < 30.0 {
            "EXCELLENT".to_string()
        } else if current_avg < 100.0 {
            "GOOD".to_string()
        } else if current_avg < 200.0 {
            "FAIR".to_string()
        } else {
            "POOR".to_string()
        };

        NetworkStats {
            current_response,
            current_avg,
            avg_response,
            min_response,
            max_response,
            uptime_pct,
            packet_loss_pct,
            jitter,
            stability,
            quality,
            total_pings: self.total_pings,
        }
    }

    pub fn quality(&self) -> String {
        self.stats().quality
    }

    pub fn reset(&mut self) {
        self.history.clear();
        self.recent.clear();
        self.total_pings = 0;
        self.successful_pings = 0;
        self.failed_pings = 0;
    }
}

