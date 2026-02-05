use anyhow::Result;
use futures_util::StreamExt;
use std::time::{Duration, Instant};
use tokio::sync::mpsc;
use tracing::{debug, error, info, trace};

#[derive(Debug, Clone)]
pub enum SpeedTestEvent {
    DownloadProgress { bytes: u64, speed: f64 },
    DownloadComplete { mbps: f64, avg: f64, peak: f64 },
    UploadProgress { bytes: u64 },
    UploadComplete { mbps: f64, duration: Duration },
    Error(String),
}

#[derive(Debug, Clone)]
pub enum SpeedTestState {
    Preparing,
    Downloading {
        bytes_received: u64,
        samples: Vec<(Instant, u64)>, // (time, bytes) for speed calculation
    },
    Uploading {
        bytes_sent: u64,
        download_results: (f64, f64, f64), // (mbps, avg, peak)
    },
    Complete {
        download_mbps: f64,
        upload_mbps: f64,
        total_bytes: u64,
        duration: Duration,
        avg_speed: f64,
        peak_speed: f64,
    },
    Error(String),
}

pub struct SpeedTest {
    state: SpeedTestState,
    _target: String,
    tx: Option<mpsc::Sender<SpeedTestEvent>>,
    rx: Option<mpsc::Receiver<SpeedTestEvent>>,
}

impl SpeedTest {
    pub async fn new(target: &str) -> Result<Self> {
        Ok(Self {
            state: SpeedTestState::Preparing,
            _target: target.to_string(),
            tx: None,
            rx: None,
        })
    }

    async fn update(&mut self) -> Result<bool> {
        // Initialize if in Preparing state
        if matches!(self.state, SpeedTestState::Preparing) {
            debug!("Initializing SpeedTest background tasks");
            let (tx, rx) = mpsc::channel(100);
            self.tx = Some(tx.clone());
            self.rx = Some(rx);

            // Start Download Task
            let tx_clone = tx.clone();
            tokio::spawn(async move {
                debug!("Spawning Download test task");
                run_download_test_task(tx_clone).await;
            });

            self.state = SpeedTestState::Downloading {
                bytes_received: 0,
                samples: Vec::new(),
            };
            return Ok(false);
        }

        // Process events
        if let Some(rx) = &mut self.rx {
            while let Ok(event) = rx.try_recv() {
                trace!("Received SpeedTest event: {:?}", event);
                match event {
                    SpeedTestEvent::DownloadProgress { bytes, speed } => {
                        if let SpeedTestState::Downloading {
                            bytes_received,
                            samples,
                            ..
                        } = &mut self.state
                        {
                            *bytes_received = bytes;
                            samples.push((Instant::now(), speed as u64));
                        }
                    }
                    SpeedTestEvent::DownloadComplete { mbps, avg, peak } => {
                        info!("Download complete: avg={:.2} Mbps, peak={:.2} Mbps", avg, peak);
                        // Transition to Uploading
                        self.state = SpeedTestState::Uploading {
                            bytes_sent: 0,
                            download_results: (mbps, avg, peak),
                        };

                        // Start Upload Task
                        if let Some(tx) = &self.tx {
                            let tx_clone = tx.clone();
                            tokio::spawn(async move {
                                debug!("Spawning Upload test task");
                                run_upload_test_task(tx_clone).await;
                            });
                        }
                    }
                    SpeedTestEvent::UploadProgress { bytes } => {
                        if let SpeedTestState::Uploading { bytes_sent, .. } = &mut self.state {
                            *bytes_sent = bytes;
                        }
                    }
                    SpeedTestEvent::UploadComplete { mbps, duration } => {
                        info!("Upload complete: {:.2} Mbps in {:?}", mbps, duration);
                        if let SpeedTestState::Uploading {
                            download_results, ..
                        } = &self.state
                        {
                            let (dl_mbps, dl_avg, dl_peak) = *download_results;
                            self.state = SpeedTestState::Complete {
                                download_mbps: dl_mbps,
                                upload_mbps: mbps,
                                total_bytes: 0, // Not tracked across both
                                duration,
                                avg_speed: dl_avg,
                                peak_speed: dl_peak,
                            };
                            debug!("SpeedTest transition to Complete");
                            return Ok(true);
                        }
                    }
                    SpeedTestEvent::Error(e) => {
                        error!("SpeedTest failure: {}", e);
                        self.state = SpeedTestState::Error(e);
                        return Ok(true);
                    }
                }
            }
        }

        Ok(false)
    }

    pub fn get_state(&self) -> &SpeedTestState {
        &self.state
    }

    pub fn is_complete(&self) -> bool {
        matches!(
            self.state,
            SpeedTestState::Complete { .. } | SpeedTestState::Error(_)
        )
    }
}

async fn run_download_test_task(tx: mpsc::Sender<SpeedTestEvent>) {
    let test_url = "https://speed.cloudflare.com/__down?bytes=25000000"; // 25MB
    debug!("Starting download test from: {}", test_url);

    let client_res = reqwest::Client::builder()
        .timeout(Duration::from_secs(60))
        .build();

    let client = match client_res {
        Ok(c) => c,
        Err(e) => {
            error!("Failed to create reqwest client: {}", e);
            let _ = tx
                .send(SpeedTestEvent::Error(format!(
                    "Failed to create client: {e}"
                )))
                .await;
            return;
        }
    };

    let test_start = Instant::now();
    let mut total_bytes = 0u64;
    let mut peak_speed = 0.0_f64;

    let response = match client.get(test_url).send().await {
        Ok(r) => {
            debug!("Download request successful: status={}", r.status());
            r
        }
        Err(e) => {
            error!("Download network error: {}", e);
            let _ = tx
                .send(SpeedTestEvent::Error(format!("Network error: {e}")))
                .await;
            return;
        }
    };

    if !response.status().is_success() {
        error!("Download HTTP error: {}", response.status());
        let _ = tx
            .send(SpeedTestEvent::Error(format!(
                "HTTP error: {}",
                response.status()
            )))
            .await;
        return;
    }

    let mut stream = response.bytes_stream();
    let mut last_update = Instant::now();

    while let Some(chunk_result) = stream.next().await {
        let chunk = match chunk_result {
            Ok(c) => c,
            Err(e) => {
                error!("Download stream read error: {}", e);
                let _ = tx
                    .send(SpeedTestEvent::Error(format!("Read error: {e}")))
                    .await;
                return;
            }
        };

        total_bytes += chunk.len() as u64;
        trace!("Download chunk: size={}, total={}", chunk.len(), total_bytes);

        // Update progress every 100ms to avoid flooding channel
        if last_update.elapsed() >= Duration::from_millis(100) {
            let elapsed = test_start.elapsed();
            if elapsed.as_secs_f64() > 0.0 {
                let current_speed =
                    (total_bytes as f64 * 8.0) / (elapsed.as_secs_f64() * 1_000_000.0);
                peak_speed = peak_speed.max(current_speed);
                let _ = tx
                    .send(SpeedTestEvent::DownloadProgress {
                        bytes: total_bytes,
                        speed: current_speed,
                    })
                    .await;
                last_update = Instant::now();
            }
        }
    }

    let duration = test_start.elapsed();
    let avg_speed = if duration.as_secs_f64() > 0.0 {
        (total_bytes as f64 * 8.0) / (duration.as_secs_f64() * 1_000_000.0)
    } else {
        0.0
    };

    info!("Download complete: total_bytes={}, avg_speed={:.2}Mbps, peak_speed={:.2}Mbps, duration={:?}", 
        total_bytes, avg_speed, peak_speed, duration);

    // Final calculation (Mbps)
    let download_mbps = avg_speed; // avg_speed is already in Mbps if calculated as (bytes*8)/micro/1000000? No.
                                   // Wait, previous code: (total_bytes as f64 * 8.0) / (elapsed.as_secs_f64() * 1_000_000.0)
                                   // Yes, that is Mbps.

    let _ = tx
        .send(SpeedTestEvent::DownloadComplete {
            mbps: download_mbps,
            avg: avg_speed,
            peak: peak_speed,
        })
        .await;
}

async fn run_upload_test_task(tx: mpsc::Sender<SpeedTestEvent>) {
    let test_url = "https://speed.cloudflare.com/__up";
    debug!("Starting upload test to: {}", test_url);

    let client = match reqwest::Client::builder()
        .timeout(Duration::from_secs(60))
        .build()
    {
        Ok(c) => c,
        Err(e) => {
            error!("Failed to create reqwest client for upload: {}", e);
            let _ = tx
                .send(SpeedTestEvent::Error(format!(
                    "Failed to create client: {e}"
                )))
                .await;
            return;
        }
    };

    // 10MB of random data
    let data_size = 10 * 1024 * 1024;
    debug!("Generating {} bytes of random data for upload", data_size);
    let data: Vec<u8> = (0..data_size).map(|_| rand::random::<u8>()).collect();

    let test_start = Instant::now();

    // Notify start
    let _ = tx.send(SpeedTestEvent::UploadProgress { bytes: 0 }).await;

    debug!("Sending POST request for upload test");
    let response = match client.post(test_url).body(data).send().await {
        Ok(r) => {
            debug!("Upload request successful: status={}", r.status());
            r
        }
        Err(e) => {
            error!("Upload network error: {}", e);
            let _ = tx
                .send(SpeedTestEvent::Error(format!("Network error: {e}")))
                .await;
            return;
        }
    };

    if !response.status().is_success() {
        error!("Upload HTTP error: {}", response.status());
        let _ = tx
            .send(SpeedTestEvent::Error(format!(
                "HTTP error: {}",
                response.status()
            )))
            .await;
        return;
    }

    let duration = test_start.elapsed();
    let total_bytes = data_size as u64;

    let upload_mbps = if duration.as_secs_f64() > 0.0 {
        (total_bytes as f64 * 8.0) / (duration.as_secs_f64() * 1_000_000.0)
    } else {
        0.0
    };

    info!("Upload complete: total_bytes={}, mbps={:.2}Mbps, duration={:?}", 
        total_bytes, upload_mbps, duration);

    let _ = tx
        .send(SpeedTestEvent::UploadComplete {
            mbps: upload_mbps,
            duration,
        })
        .await;
}
