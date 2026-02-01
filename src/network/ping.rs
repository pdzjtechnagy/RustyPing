use super::NetworkStats;
use anyhow::Result;
use std::collections::VecDeque;
use std::net::IpAddr;
use surge_ping::{Client, Config, PingIdentifier, PingSequence};
use std::time::Duration;
use tokio::sync::mpsc;

#[derive(Debug)]
pub enum PingResult {
    Success(f64),
    Timeout,
}

pub struct PingMonitor {
    target_addr: IpAddr,
    history: VecDeque<Option<f64>>,
    recent: VecDeque<f64>,
    max_history: usize,
    total_pings: u64,
    successful_pings: u64,
    failed_pings: u64,
}

impl PingMonitor {
    pub fn new(target_addr: IpAddr, max_history: usize) -> Self {
        Self {
            target_addr,
            history: VecDeque::with_capacity(max_history),
            recent: VecDeque::with_capacity(10),
            max_history,
            total_pings: 0,
            successful_pings: 0,
            failed_pings: 0,
        }
    }

    pub fn get_target_addr(&self) -> IpAddr {
        self.target_addr
    }

    pub fn set_max_history(&mut self, new_size: usize) {
        if new_size == self.max_history {
            return;
        }

        self.max_history = new_size;
        
        // If shrinking, truncate older data
        while self.history.len() > new_size {
            self.history.pop_front();
        }
        
        // If growing, we just let it fill up naturally
    }

    pub fn process_result(&mut self, result: PingResult) {
        self.total_pings += 1;
        match result {
            PingResult::Success(ms) => {
                self.successful_pings += 1;
                self.history.push_back(Some(ms));
                self.recent.push_back(ms);
                if self.recent.len() > 10 {
                    self.recent.pop_front();
                }
            },
            PingResult::Timeout => {
                self.failed_pings += 1;
                self.history.push_back(None);
            }
        }

        if self.history.len() > self.max_history {
            self.history.pop_front();
        }
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
            (100.0 - (jitter / avg_response * 100.0)).clamp(0.0, 100.0)
        } else {
            100.0
        };

        let quality = if current_response.is_none() && self.history.back().is_some() {
            // Only report OFFLINE if the MOST RECENT ping failed (history.back() is the latest pushed, which is None)
            // Wait, history.back() returns Option<&Option<f64>>.
            // If pushed None, back() is Some(None).
            // So current_response is None.
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

// Background Task Logic
pub async fn start_ping_task(target: &str) -> Result<(IpAddr, mpsc::Sender<()>, mpsc::Receiver<PingResult>)> {
    let target_addr: IpAddr = if let Ok(addr) = target.parse() {
        addr
    } else {
        use tokio::net::lookup_host;
        let mut addrs = lookup_host(format!("{target}:0")).await?;
        addrs.next().ok_or_else(|| anyhow::anyhow!("Could not resolve hostname"))?.ip()
    };

    let (cmd_tx, mut cmd_rx) = mpsc::channel(1);
    let (res_tx, res_rx) = mpsc::channel(100);

    let config = Config::default();
    let client = Client::new(&config)?;
    let addr = target_addr;

    tokio::spawn(async move {
        let mut pinger = client.pinger(addr, PingIdentifier(rand::random())).await;
        let mut seq = 0;

        while let Some(_) = cmd_rx.recv().await {
            let payload = [0; 8];
            // 2-second timeout for the ping itself
            match tokio::time::timeout(Duration::from_secs(2), pinger.ping(PingSequence(seq), &payload)).await {
                Ok(Ok((_, duration))) => {
                    let ms = duration.as_secs_f64() * 1000.0;
                    let _ = res_tx.send(PingResult::Success(ms)).await;
                }
                _ => {
                    let _ = res_tx.send(PingResult::Timeout).await;
                }
            }
            seq = seq.wrapping_add(1);
        }
    });

    Ok((target_addr, cmd_tx, res_rx))
}
