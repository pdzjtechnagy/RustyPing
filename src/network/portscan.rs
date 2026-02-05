use anyhow::Result;
use std::net::{IpAddr, SocketAddr};
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::time::timeout;
use tracing::{debug, info, trace};

#[derive(Debug, Clone)]
pub struct PortResult {
    pub port: u16,
    pub status: PortStatus,
    pub service: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PortStatus {
    Open,
    Closed,
    Filtered, // Timeout - might be filtered by firewall
}

pub struct PortScanner {
    _target: String,
    target_ip: IpAddr,
    ports: Vec<u16>,
    results: Vec<PortResult>,
    current_index: usize,
    completed: bool,
}

impl PortScanner {
    pub async fn new(target: &str) -> Result<Self> {
        // Resolve target to IP
        debug!("Scanner resolving: {}", target);
        let target_ip: IpAddr = if let Ok(addr) = target.parse() {
            addr
        } else {
            use tokio::net::lookup_host;
            let mut addrs = lookup_host(format!("{target}:0")).await?;
            let addr = addrs
                .next()
                .ok_or_else(|| anyhow::anyhow!("Could not resolve hostname"))?
                .ip();
            info!("Scanner resolved {} to {}", target, addr);
            addr
        };

        // Common ports to scan
        let ports = vec![
            21,    // FTP
            22,    // SSH
            23,    // Telnet
            25,    // SMTP
            53,    // DNS
            80,    // HTTP
            110,   // POP3
            111,   // RPC
            135,   // MSRPC
            139,   // NetBIOS
            143,   // IMAP
            443,   // HTTPS
            445,   // SMB
            993,   // IMAPS
            995,   // POP3S
            1433,  // MSSQL
            3306,  // MySQL
            3389,  // RDP
            5432,  // PostgreSQL
            5900,  // VNC
            6379,  // Redis
            8000,  // HTTP Alt
            8080,  // HTTP Alt
            8443,  // HTTPS Alt
            9200,  // ElasticSearch
            25565, // Minecraft
            27017, // MongoDB
        ];

        Ok(Self {
            _target: target.to_string(),
            target_ip,
            ports,
            results: Vec::new(),
            current_index: 0,
            completed: false,
        })
    }

    pub async fn update(&mut self) -> Result<bool> {
        if self.completed {
            return Ok(true);
        }

        // Scan ports in batches (5 at a time for responsiveness)
        let batch_size = 5;
        let end_index = (self.current_index + batch_size).min(self.ports.len());

        for i in self.current_index..end_index {
            let port = self.ports[i];
            let status = self.scan_port(port).await;

            let service = Self::identify_service(port);

            self.results.push(PortResult {
                port,
                status,
                service,
            });
        }

        self.current_index = end_index;

        if self.current_index >= self.ports.len() {
            self.completed = true;
            return Ok(true);
        }

        Ok(false)
    }

    async fn scan_port(&self, port: u16) -> PortStatus {
        let addr = SocketAddr::new(self.target_ip, port);
        trace!("Scanning port: {}", port);
        match timeout(Duration::from_millis(1500), TcpStream::connect(addr)).await {
            Ok(Ok(_)) => {
                debug!("Port {} is OPEN", port);
                PortStatus::Open
            },
            Ok(Err(e)) => {
                trace!("Port {} is CLOSED: {}", port, e);
                PortStatus::Closed
            },
            Err(_) => {
                debug!("Port {} is FILTERED (timeout)", port);
                PortStatus::Filtered
            },
        }
    }

    fn identify_service(port: u16) -> Option<String> {
        let service = match port {
            21 => "FTP",
            22 => "SSH",
            23 => "Telnet",
            25 => "SMTP",
            53 => "DNS",
            80 => "HTTP",
            110 => "POP3",
            111 => "RPC",
            135 => "MSRPC",
            139 => "NetBIOS",
            143 => "IMAP",
            443 => "HTTPS",
            445 => "SMB",
            993 => "IMAPS",
            995 => "POP3S",
            1433 => "MSSQL",
            3306 => "MySQL",
            3389 => "RDP",
            5432 => "PostgreSQL",
            5900 => "VNC",
            6379 => "Redis",
            8000 => "HTTP-Alt",
            8080 => "HTTP-Alt",
            8443 => "HTTPS-Alt",
            9200 => "ElasticSearch",
            25565 => "Minecraft",
            27017 => "MongoDB",
            _ => return None,
        };
        Some(service.to_string())
    }

    pub fn results(&self) -> &[PortResult] {
        &self.results
    }

    pub fn progress(&self) -> (usize, usize) {
        (self.current_index, self.ports.len())
    }

    pub fn is_complete(&self) -> bool {
        self.completed
    }
}
