use super::NetworkStats;
use anyhow::Result;
use std::collections::VecDeque;
use std::net::IpAddr;
use surge_ping::{Client, Config, PingIdentifier, PingSequence};
use std::time::Duration;
use tokio::sync::mpsc;

use tokio::net::TcpStream;
use std::io;

#[derive(Debug, Clone, PartialEq)]
pub enum WebCheckStatus {
    Untested,
    Success(f64),
    Timeout,
    ConnectionRefused,
    Error(String),
}

#[derive(Debug)]
pub enum PingCommand {
    ToggleWebCheck(bool),
}

#[derive(Debug)]
pub enum PingResult {
    Success(f64),
    Timeout,
    WebCheck { port: u16, status: WebCheckStatus },
}

pub struct PingMonitor {
    target_addr: IpAddr,
    history: VecDeque<Option<f64>>,
    recent: VecDeque<f64>,
    max_history: usize,
    total_pings: u64,
    successful_pings: u64,
    failed_pings: u64,
    pub dns_duration: Option<f64>,
    pub tcp_80: WebCheckStatus,
    pub tcp_443: WebCheckStatus,
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
            dns_duration: None,
            tcp_80: WebCheckStatus::Untested,
            tcp_443: WebCheckStatus::Untested,
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
            },
            PingResult::WebCheck { port, status } => {
                match port {
                    80 => self.tcp_80 = status,
                    443 => self.tcp_443 = status,
                    _ => {}
                }
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
            dns_duration: self.dns_duration,
            tcp_port_80: self.tcp_80.clone(),
            tcp_port_443: self.tcp_443.clone(),
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
pub async fn start_ping_task(target: &str) -> Result<(IpAddr, mpsc::Sender<PingCommand>, mpsc::Receiver<PingResult>, Option<f64>)> {
    let start_dns = std::time::Instant::now();
    let target_addr: IpAddr = if let Ok(addr) = target.parse() {
        addr
    } else {
        use tokio::net::lookup_host;
        let mut addrs = lookup_host(format!("{target}:0")).await?;
        addrs.next().ok_or_else(|| anyhow::anyhow!("Could not resolve hostname"))?.ip()
    };
    let dns_duration = start_dns.elapsed().as_secs_f64() * 1000.0;

    let (cmd_tx, mut cmd_rx) = mpsc::channel(1);
    let (res_tx, res_rx) = mpsc::channel(100);

    let config = Config::default();
    let client = Client::new(&config)?;
    let addr = target_addr;

    tokio::spawn(async move {
        let mut pinger = client.pinger(addr, PingIdentifier(rand::random())).await;
        let mut seq = 0;
        let mut interval = tokio::time::interval(Duration::from_millis(1000));
        let mut web_check_enabled = false;
        
        loop {
            tokio::select! {
                _ = interval.tick() => {
                     let payload = [0; 8];
                     
                     // Spawn web checks if enabled (fire and forget)
                     if web_check_enabled {
                        let tx80 = res_tx.clone();
                        let tx443 = res_tx.clone();
                        let target = addr;
                        
                        tokio::spawn(async move {
                            let start = std::time::Instant::now();
                            let status = match tokio::time::timeout(Duration::from_secs(2), TcpStream::connect((target, 80))).await {
                                Ok(Ok(_)) => WebCheckStatus::Success(start.elapsed().as_secs_f64() * 1000.0),
                                Ok(Err(e)) => match e.kind() {
                                    io::ErrorKind::ConnectionRefused => WebCheckStatus::ConnectionRefused,
                                    io::ErrorKind::TimedOut => WebCheckStatus::Timeout,
                                    _ => WebCheckStatus::Error(e.to_string()),
                                },
                                Err(_) => WebCheckStatus::Timeout,
                            };
                            let _ = tx80.send(PingResult::WebCheck { port: 80, status }).await;
                        });

                        tokio::spawn(async move {
                            let start = std::time::Instant::now();
                            let status = match tokio::time::timeout(Duration::from_secs(2), TcpStream::connect((target, 443))).await {
                                Ok(Ok(_)) => WebCheckStatus::Success(start.elapsed().as_secs_f64() * 1000.0),
                                Ok(Err(e)) => match e.kind() {
                                    io::ErrorKind::ConnectionRefused => WebCheckStatus::ConnectionRefused,
                                    io::ErrorKind::TimedOut => WebCheckStatus::Timeout,
                                    _ => WebCheckStatus::Error(e.to_string()),
                                },
                                Err(_) => WebCheckStatus::Timeout,
                            };
                            let _ = tx443.send(PingResult::WebCheck { port: 443, status }).await;
                        });
                     }

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
                cmd = cmd_rx.recv() => {
                    match cmd {
                        None => break,
                        Some(PingCommand::ToggleWebCheck(enabled)) => {
                            web_check_enabled = enabled;
                        }
                    }
                }
            }
        }
    });

    Ok((target_addr, cmd_tx, res_rx, Some(dns_duration)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::timeout;

    #[tokio::test]
    async fn test_network_intelligence_flow() {
        // 1. Start the ping task against Google DNS
        // Note: this actually performs network IO, so it might flake if offline.
        let (addr, cmd_tx, mut res_rx, dns_duration) = start_ping_task("8.8.8.8").await.expect("Failed to start ping task");
        
        println!("Resolved 8.8.8.8 to {}", addr);
        assert!(dns_duration.is_some(), "DNS duration should be recorded");

        // 2. Wait for initial ICMP pings
        // Allow some time for pings to happen
        let mut success_count = 0;
        let start = std::time::Instant::now();
        while start.elapsed() < Duration::from_secs(4) {
             match timeout(Duration::from_millis(500), res_rx.recv()).await {
                Ok(Some(res)) => {
                    if let PingResult::Success(_) = res {
                        success_count += 1;
                    }
                }
                _ => {}
            }
            if success_count >= 1 { break; }
        }
        
        // It's possible pings fail due to permissions/network, but we care about the flow here.
        // If we can't ping, we might at least verify the DNS part.

        // 3. Enable Web Check
        cmd_tx.send(PingCommand::ToggleWebCheck(true)).await.expect("Failed to send command");

        // 4. Wait for Web Check results
        let mut received_80 = false;
        let mut received_443 = false;

        let check_duration = Duration::from_secs(5);
        let start_web = std::time::Instant::now();

        while start_web.elapsed() < check_duration {
            match timeout(Duration::from_millis(1500), res_rx.recv()).await {
                Ok(Some(res)) => {
                    match res {
                        PingResult::WebCheck { port, status: _ } => {
                            println!("Received Web Check result for port {}", port);
                            if port == 80 { received_80 = true; }
                            if port == 443 { received_443 = true; }
                        },
                        _ => {} 
                    }
                },
                _ => continue,
            }

            if received_80 && received_443 {
                break;
            }
        }

        assert!(received_80, "Should have received Port 80 check result");
        assert!(received_443, "Should have received Port 443 check result");

        // 5. Clean shutdown
        cmd_tx.send(PingCommand::Stop).await.expect("Failed to stop");
    }
}
