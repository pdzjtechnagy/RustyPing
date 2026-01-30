use crate::app::App;
use crate::network::{PortResult, PortStatus, SpeedTestState};
use crate::theme::Theme;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    symbols,
    text::{Line, Span},
    widgets::{
        canvas::{Canvas, Rectangle},
        Block, Borders, Gauge, Paragraph,
    },
    Frame,
};

pub fn draw(f: &mut Frame, app: &mut App) {
    let area = f.area();
    
    // Dynamic scaling based on terminal size (btop-style)
    let min_height = 20;
    let min_width = 80;
    
    if area.height < min_height as u16 || area.width < min_width as u16 {
        // Terminal too small - show minimal view
        draw_minimal_view(f, app, area);
        return;
    }

    // Calculate dynamic constraints based on terminal size
    let header_height = 3;
    let footer_height = 1;
    
    // Check if we need to show panels
    let has_speedtest = app.speedtest.is_some();
    let has_portscan = app.portscan.is_some();
    let has_panels = has_speedtest || has_portscan;
    
    // Adjust layout based on panels
    let stats_height = if area.height >= 30 {
        13 // Full stats panel
    } else if area.height >= 25 {
        10 // Reduced stats
    } else {
        8  // Minimal stats
    };
    
    let panel_height = if has_panels {
        if has_speedtest && has_portscan {
            12 // Both panels
        } else {
            8  // Single panel
        }
    } else {
        0
    };
    
    let graph_height = area.height.saturating_sub(header_height + stats_height + footer_height + panel_height);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(header_height),
            Constraint::Min(graph_height.max(8)), // Graph with minimum height
            Constraint::Length(stats_height),
            if has_panels { Constraint::Length(panel_height) } else { Constraint::Length(0) },
            Constraint::Length(footer_height),
        ])
        .split(area);

    // Header
    draw_header(f, app, chunks[0]);

    // Main latency graph (BRAILLE!)
    draw_latency_graph(f, app, chunks[1]);

    // Bottom section - responsive layout
    let bottom_chunks = if app.show_jitter && area.width >= 100 {
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(chunks[2])
    } else {
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(100), Constraint::Length(0)])
            .split(chunks[2])
    };

    draw_statistics(f, app, bottom_chunks[0]);
    if app.show_jitter && area.width >= 100 {
        draw_jitter_panel(f, app, bottom_chunks[1]);
    }

    // Panels (speedtest/portscan)
    if has_panels && chunks.len() > 3 {
        let panel_area = chunks[3];
        if has_speedtest && has_portscan {
            let panel_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(panel_area);
            draw_speedtest_panel(f, app, panel_chunks[0]);
            draw_portscan_panel(f, app, panel_chunks[1]);
        } else if has_speedtest {
            draw_speedtest_panel(f, app, panel_area);
        } else if has_portscan {
            draw_portscan_panel(f, app, panel_area);
        }
    }

    // Footer
    draw_footer(f, app, chunks[chunks.len() - 1]);

    // Settings overlay (rendered last so it's on top)
    if app.show_settings {
        draw_settings_overlay(f, app);
    }
}

fn draw_minimal_view(f: &mut Frame, _app: &App, area: Rect) {
    let text = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "Terminal too small!",
                Style::default().fg(Theme::CRIT).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                format!("Minimum size: 80x20 (current: {}x{})", area.width, area.height),
                Style::default().fg(Theme::LOW),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Resize terminal and restart.", Style::default().fg(Theme::FG)),
        ]),
    ];
    
    let paragraph = Paragraph::new(text)
        .block(
            Block::default()
                .title(" RustyPing ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Theme::CRIT)),
        )
        .style(Style::default().bg(Theme::BG).fg(Theme::FG));
    
    f.render_widget(paragraph, area);
}

fn draw_header(f: &mut Frame, app: &App, area: Rect) {
    let quality = app.ping_monitor.quality();
    let status_color = Theme::quality_color(&quality);
    let stats = app.ping_monitor.stats();

    // Enhanced header with more information
    let text = vec![Line::from(vec![
        Span::styled(" RustyPing ", Style::default().fg(Theme::TITLE).add_modifier(Modifier::BOLD)),
        Span::raw("│"),
        Span::styled(format!(" {} ", &app.target), Style::default().fg(Theme::HI_FG)),
        Span::raw("│"),
        Span::styled(" ● ", Style::default().fg(status_color)),
        Span::styled(quality, Style::default().fg(status_color).add_modifier(Modifier::BOLD)),
        Span::raw(" │"),
        Span::styled(
            format!(" Packets: {} ", stats.total_pings),
            Style::default().fg(Theme::LOW),
        ),
    ])];

    let header = Paragraph::new(text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Theme::BOX))
        )
        .style(Style::default().bg(Theme::BG).fg(Theme::FG));

    f.render_widget(header, area);
}

fn draw_latency_graph(f: &mut Frame, app: &App, area: Rect) {
    let data = app.ping_monitor.latency_data();
    
    // Enhanced empty state
    if data.is_empty() {
        let empty_text = vec![
            Line::from(""),
            Line::from(vec![
                Span::styled(
                    "  Waiting for ping data...",
                    Style::default().fg(Theme::LOW),
                ),
            ]),
        ];
        let paragraph = Paragraph::new(empty_text)
            .block(
                Block::default()
                    .title("LATENCY GRAPH")
                    .title_style(Style::default().fg(Theme::TITLE))
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Theme::BOX)),
            )
            .style(Style::default().bg(Theme::BG).fg(Theme::FG));
        f.render_widget(paragraph, area);
        return;
    }

    // Filter out None values and convert to (x, y) points
    let points: Vec<(f64, f64)> = data
        .iter()
        .enumerate()
        .filter_map(|(i, &val)| val.map(|v| (i as f64, v)))
        .collect();

    if points.is_empty() {
        let empty_text = vec![
            Line::from(""),
            Line::from(vec![
                Span::styled(
                    "  No successful pings yet...",
                    Style::default().fg(Theme::CRIT),
                ),
            ]),
        ];
        let paragraph = Paragraph::new(empty_text)
            .block(
                Block::default()
                    .title("LATENCY GRAPH")
                    .title_style(Style::default().fg(Theme::TITLE))
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Theme::BOX)),
            )
            .style(Style::default().bg(Theme::BG).fg(Theme::FG));
        f.render_widget(paragraph, area);
        return;
    }

    // Calculate bounds with better padding
    let max_latency: f64 = points.iter().map(|(_, y)| *y).fold(0.0_f64, |a, b| a.max(b)).max(50.0);
    let min_latency: f64 = points.iter().map(|(_, y)| *y).fold(f64::INFINITY, |a, b| a.min(b)).min(0.0);
    
    // Add 10% padding on top and bottom for better visibility
    let y_padding: f64 = (max_latency - min_latency).max(10.0) * 0.1;
    let y_min: f64 = (min_latency - y_padding).max(0.0);
    let y_max: f64 = max_latency + y_padding;

    // Calculate time window label
    let time_window = format!("last {}s", data.len());

    // BRAILLE CANVAS - High-resolution rendering!
    let canvas = Canvas::default()
        .block(
            Block::default()
                .title(format!(" LATENCY ({}) ", time_window))
                .title_style(Style::default().fg(Theme::TITLE).add_modifier(Modifier::BOLD))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Theme::BOX)),
        )
        .marker(symbols::Marker::Braille)  // <-- BRAILLE RENDERING!
        .x_bounds([0.0, data.len() as f64])
        .y_bounds([y_min, y_max])
        .paint(|ctx| {
            // Draw solid filled waveform graph (btop CPU-style) with gradient colors
            // Bars are touching for a solid waveform appearance
            for &(x, y) in &points {
                let ratio = if y_max > 0.0 { (y / y_max).min(1.0) } else { 0.0 };
                let color = Theme::graph_gradient(ratio);
                
                // Draw filled rectangle from bottom to data point
                // Bars are 1.0 wide and centered on x, so they touch (no gaps, no lines)
                ctx.draw(&Rectangle {
                    x: x - 0.5,  // Center bar on x coordinate
                    y: y_min,
                    width: 1.0,  // Full width so bars touch
                    height: y - y_min,
                    color,
                });
            }
        });

    f.render_widget(canvas, area);
}

fn draw_statistics(f: &mut Frame, app: &App, area: Rect) {
    let stats = app.ping_monitor.stats();
    
    let current_color = if let Some(rt) = stats.current_response {
        Theme::latency_color(rt)
    } else {
        Theme::CRIT
    };

    // Enhanced statistics display with better formatting
    let text = vec![
        Line::from(vec![
            Span::styled("Current:   ", Style::default().fg(Theme::LOW)),
            Span::styled(
                format!("{:>8.1} ms", stats.current_response.unwrap_or(0.0)),
                Style::default().fg(current_color).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("Avg (10):  ", Style::default().fg(Theme::LOW)),
            Span::styled(
                format!("{:>8.1} ms", stats.current_avg),
                Style::default().fg(Theme::latency_color(stats.current_avg)),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Session Statistics:", Style::default().fg(Theme::TITLE).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled("  Avg:    ", Style::default().fg(Theme::LOW)),
            Span::styled(format!("{:>8.1} ms", stats.avg_response), Style::default().fg(Theme::FG)),
        ]),
        Line::from(vec![
            Span::styled("  Min:    ", Style::default().fg(Theme::LOW)),
            Span::styled(format!("{:>8.1} ms", stats.min_response), Style::default().fg(Theme::GOOD)),
        ]),
        Line::from(vec![
            Span::styled("  Max:    ", Style::default().fg(Theme::LOW)),
            Span::styled(format!("{:>8.1} ms", stats.max_response), Style::default().fg(Theme::WARN)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Uptime:    ", Style::default().fg(Theme::GOOD)),
            Span::styled(format!("{:>6.1}%", stats.uptime_pct), Style::default().fg(Theme::FG)),
        ]),
        Line::from(vec![
            Span::styled("Loss:      ", Style::default().fg(Theme::CRIT)),
            Span::styled(format!("{:>6.1}%", stats.packet_loss_pct), Style::default().fg(Theme::FG)),
        ]),
        Line::from(""),
        Line::from(highlighted_key("R", "Reset Stats")),
    ];

    let paragraph = Paragraph::new(text)
        .block(
            Block::default()
                .title(" STATISTICS ")
                .title_style(Style::default().fg(Theme::TITLE).add_modifier(Modifier::BOLD))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Theme::BOX)),
        )
        .style(Style::default().bg(Theme::BG).fg(Theme::FG));

    f.render_widget(paragraph, area);
}

fn draw_jitter_panel(f: &mut Frame, app: &App, area: Rect) {
    let stats = app.ping_monitor.stats();
    
    // Calculate gauge color based on stability
    let gauge_color = if stats.stability >= 90.0 {
        Theme::GOOD
    } else if stats.stability >= 70.0 {
        Theme::WARN
    } else {
        Theme::CRIT
    };

    let text = vec![
        Line::from(vec![
            Span::styled("Jitter:       ", Style::default().fg(Theme::LOW)),
            Span::styled(
                format!("{:>8.1} ms", stats.jitter),
                Style::default().fg(Theme::FG),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Stability:    ", Style::default().fg(Theme::LOW)),
            Span::styled(
                format!("{:>6.0}%", stats.stability),
                Style::default().fg(gauge_color).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(""),
        Line::from(highlighted_key("S", "Speed Test")),
        Line::from(highlighted_key("P", "Port Scan")),
    ];

    let paragraph = Paragraph::new(text)
        .block(
            Block::default()
                .title(" QUALITY & ACTIONS ")
                .title_style(Style::default().fg(Theme::TITLE).add_modifier(Modifier::BOLD))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Theme::BOX)),
        )
        .style(Style::default().bg(Theme::BG).fg(Theme::FG));

    f.render_widget(paragraph, area);

    // Draw stability gauge
    let gauge_area = Rect {
        x: area.x + 1,
        y: area.y + 3,
        width: area.width.saturating_sub(2),
        height: 1,
    };

    let gauge = Gauge::default()
        .block(Block::default())
        .gauge_style(Style::default().fg(gauge_color))
        .ratio(stats.stability / 100.0)
        .label(format!("{:.0}%", stats.stability));

    f.render_widget(gauge, gauge_area);

}

fn draw_footer(f: &mut Frame, app: &App, area: Rect) {
    let runtime = app.runtime();
    let stats = app.ping_monitor.stats();
    
    // Enhanced footer with highlighted keys (btop-style)
    let mut spans = vec![
        Span::styled("[Q]", Style::default().fg(Theme::KEY_HIGHLIGHT).add_modifier(Modifier::BOLD)),
        Span::styled("uit", Style::default().fg(Theme::LOW)),
        Span::raw(" │ "),
        Span::styled("[ESC]", Style::default().fg(Theme::KEY_HIGHLIGHT).add_modifier(Modifier::BOLD)),
        Span::styled("ettings", Style::default().fg(Theme::LOW)),
    ];
    
    // Only show S/P if panels aren't open
    if app.speedtest.is_none() && app.portscan.is_none() {
        spans.extend(vec![
            Span::raw(" │ "),
            Span::styled("[S]", Style::default().fg(Theme::KEY_HIGHLIGHT).add_modifier(Modifier::BOLD)),
            Span::styled("peed", Style::default().fg(Theme::LOW)),
            Span::raw(" │ "),
            Span::styled("[P]", Style::default().fg(Theme::KEY_HIGHLIGHT).add_modifier(Modifier::BOLD)),
            Span::styled("ort", Style::default().fg(Theme::LOW)),
        ]);
    } else {
        spans.extend(vec![
            Span::raw(" │ "),
            Span::styled("[C]", Style::default().fg(Theme::KEY_HIGHLIGHT).add_modifier(Modifier::BOLD)),
            Span::styled("lose", Style::default().fg(Theme::LOW)),
        ]);
    }
    
    spans.extend(vec![
        Span::raw(" │ "),
        Span::styled("Runtime: ", Style::default().fg(Theme::LOW)),
        Span::styled(
            format!("{:02}:{:02}:{:02}", 
                runtime.as_secs() / 3600,
                (runtime.as_secs() % 3600) / 60,
                runtime.as_secs() % 60
            ),
            Style::default().fg(Theme::FG).add_modifier(Modifier::BOLD),
        ),
        Span::raw(" │ "),
        Span::styled("Pkts: ", Style::default().fg(Theme::LOW)),
        Span::styled(
            format!("{}", stats.total_pings),
            Style::default().fg(Theme::FG),
        ),
    ]);

    let text = Line::from(spans);
    let paragraph = Paragraph::new(text)
        .style(Style::default().bg(Theme::BG).fg(Theme::FG));
    f.render_widget(paragraph, area);
}

fn draw_settings_overlay(f: &mut Frame, app: &App) {
    let area = centered_rect(45, 40, f.area());
    
    // Enhanced settings overlay with better styling
    let text = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled(
                if app.settings_selected == 0 { " ▶ " } else { "   " },
                Style::default().fg(Theme::TITLE),
            ),
            Span::styled(
                if app.show_jitter { "☑" } else { "☐" },
                Style::default().fg(if app.show_jitter { Theme::GOOD } else { Theme::LOW }),
            ),
            Span::raw(" "),
            Span::styled(
                "Show Jitter Panel",
                Style::default()
                    .fg(if app.settings_selected == 0 { Theme::HI_FG } else { Theme::FG }),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                if app.settings_selected == 1 { " ▶ " } else { "   " },
                Style::default().fg(Theme::TITLE),
            ),
            Span::styled(
                if app.show_history { "☑" } else { "☐" },
                Style::default().fg(if app.show_history { Theme::GOOD } else { Theme::LOW }),
            ),
            Span::raw(" "),
            Span::styled(
                "Show History Panel",
                Style::default()
                    .fg(if app.settings_selected == 1 { Theme::HI_FG } else { Theme::FG }),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                if app.settings_selected == 2 { " ▶ " } else { "   " },
                Style::default().fg(Theme::TITLE),
            ),
            Span::styled(
                if app.config.pause_ping_during_speedtest { "☑" } else { "☐" },
                Style::default().fg(if app.config.pause_ping_during_speedtest { Theme::GOOD } else { Theme::LOW }),
            ),
            Span::raw(" "),
            Span::styled(
                "Pause ping during speedtest",
                Style::default()
                    .fg(if app.settings_selected == 2 { Theme::HI_FG } else { Theme::FG }),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                " ↑/↓ ",
                Style::default().fg(Theme::HI_FG).add_modifier(Modifier::BOLD),
            ),
            Span::styled("Navigate  ", Style::default().fg(Theme::LOW)),
            Span::styled(
                " Enter ",
                Style::default().fg(Theme::HI_FG).add_modifier(Modifier::BOLD),
            ),
            Span::styled("Toggle  ", Style::default().fg(Theme::LOW)),
            Span::styled(
                " ESC ",
                Style::default().fg(Theme::HI_FG).add_modifier(Modifier::BOLD),
            ),
            Span::styled("Close", Style::default().fg(Theme::LOW)),
        ]),
    ];

    let paragraph = Paragraph::new(text)
        .block(
            Block::default()
                .title(" SETTINGS ")
                .title_style(Style::default().fg(Theme::TITLE).add_modifier(Modifier::BOLD))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Theme::TITLE)),
        )
        .style(Style::default().bg(Theme::SELECTED_BG).fg(Theme::FG));

    f.render_widget(paragraph, area);
}

// Helper function to create highlighted key spans (btop-style)
fn highlighted_key<'a>(key: &'a str, label: &'a str) -> Vec<Span<'a>> {
    vec![
        Span::styled(
            format!("[{key}]"),
            Style::default().fg(Theme::KEY_HIGHLIGHT).add_modifier(Modifier::BOLD),
        ),
        Span::raw(" "),
        Span::styled(label, Style::default().fg(Theme::LOW)),
    ]
}

fn draw_speedtest_panel(f: &mut Frame, app: &App, area: Rect) {
    if let Some(ref st) = app.speedtest {
        let mut lines = vec![Line::from("")];
        
        match st.state() {
            SpeedTestState::Preparing => {
                lines.push(Line::from(vec![
                    Span::styled("Preparing speed test...", Style::default().fg(Theme::TITLE)),
                ]));
            }
            SpeedTestState::Downloading { bytes_received, .. } => {
                lines.push(Line::from(vec![
                    Span::styled("Download Speed Test", Style::default().fg(Theme::TITLE).add_modifier(Modifier::BOLD)),
                ]));
                lines.push(Line::from(""));
                lines.push(Line::from(vec![
                    Span::styled("Downloading...", Style::default().fg(Theme::TITLE)),
                ]));
                lines.push(Line::from(vec![
                    Span::styled("Bytes: ", Style::default().fg(Theme::LOW)),
                    Span::styled(format!("{:.2} MB", *bytes_received as f64 / 1_000_000.0), Style::default().fg(Theme::FG)),
                ]));
            }
            SpeedTestState::Uploading { bytes_sent, download_results, .. } => {
                lines.push(Line::from(vec![
                    Span::styled("Upload Speed Test", Style::default().fg(Theme::TITLE).add_modifier(Modifier::BOLD)),
                ]));
                lines.push(Line::from(""));
                lines.push(Line::from(vec![
                    Span::styled("Uploading...", Style::default().fg(Theme::TITLE)),
                ]));
                lines.push(Line::from(vec![
                    Span::styled("Download: ", Style::default().fg(Theme::LOW)),
                    Span::styled(format!("{:.2} Mbps", download_results.0), Style::default().fg(Theme::GOOD)),
                ]));
                lines.push(Line::from(vec![
                    Span::styled("Bytes: ", Style::default().fg(Theme::LOW)),
                    Span::styled(format!("{:.2} MB", *bytes_sent as f64 / 1_000_000.0), Style::default().fg(Theme::FG)),
                ]));
            }
            SpeedTestState::Complete { download_mbps, upload_mbps, total_bytes, duration, avg_speed, peak_speed } => {
                lines.push(Line::from(vec![
                    Span::styled("Speed Test Complete", Style::default().fg(Theme::GOOD).add_modifier(Modifier::BOLD)),
                ]));
                lines.push(Line::from(""));
                lines.push(Line::from(vec![
                    Span::styled("Download: ", Style::default().fg(Theme::LOW)),
                    Span::styled(
                        format!("{download_mbps:.2} Mbps"),
                        Style::default().fg(Theme::GOOD).add_modifier(Modifier::BOLD),
                    ),
                ]));
                lines.push(Line::from(vec![
                    Span::styled("Upload:   ", Style::default().fg(Theme::LOW)),
                    Span::styled(
                        format!("{upload_mbps:.2} Mbps"),
                        Style::default().fg(Theme::GOOD).add_modifier(Modifier::BOLD),
                    ),
                ]));
                lines.push(Line::from(vec![
                    Span::styled("Peak DL:  ", Style::default().fg(Theme::LOW)),
                    Span::styled(
                        format!("{peak_speed:.2} Mbps"),
                        Style::default().fg(Theme::FG),
                    ),
                ]));
                lines.push(Line::from(vec![
                    Span::styled("Avg DL:   ", Style::default().fg(Theme::LOW)),
                    Span::styled(
                        format!("{avg_speed:.2} Mbps"),
                        Style::default().fg(Theme::FG),
                    ),
                ]));
                lines.push(Line::from(vec![
                    Span::styled("Data:     ", Style::default().fg(Theme::LOW)),
                    Span::styled(
                        format!("{:.2} MB", *total_bytes as f64 / 1_000_000.0),
                        Style::default().fg(Theme::FG),
                    ),
                ]));
                lines.push(Line::from(vec![
                    Span::styled("Duration: ", Style::default().fg(Theme::LOW)),
                    Span::styled(
                        format!("{:.1}s", duration.as_secs_f64()),
                        Style::default().fg(Theme::FG),
                    ),
                ]));
            }
            SpeedTestState::Error(msg) => {
                lines.push(Line::from(vec![
                    Span::styled("Speed Test Error", Style::default().fg(Theme::CRIT).add_modifier(Modifier::BOLD)),
                ]));
                lines.push(Line::from(""));
                lines.push(Line::from(vec![
                    Span::styled("Error: ", Style::default().fg(Theme::CRIT)),
                    Span::styled(msg.clone(), Style::default().fg(Theme::FG)),
                ]));
            }
            SpeedTestState::Idle => return,
        }
        
        lines.push(Line::from(""));
        lines.push(Line::from(highlighted_key("C", "Close")));
        
        let paragraph = Paragraph::new(lines)
            .block(
                Block::default()
                    .title(" SPEED TEST ")
                    .title_style(Style::default().fg(Theme::TITLE).add_modifier(Modifier::BOLD))
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Theme::BOX)),
            )
            .style(Style::default().bg(Theme::BG).fg(Theme::FG));
        
        f.render_widget(paragraph, area);
    }
}

fn draw_portscan_panel(f: &mut Frame, app: &App, area: Rect) {
    if let Some(ref ps) = app.portscan {
        let (current, total) = ps.progress();
        let progress_pct = if total > 0 {
            (current as f64 / total as f64) * 100.0
        } else {
            0.0
        };

        let mut lines = vec![
            Line::from(vec![
                Span::styled(
                    format!("Scanning: {}", app.target),
                    Style::default().fg(Theme::TITLE).add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(vec![
                Span::styled(
                    format!("Progress: {}/{} ({:.0}%)", current, total, progress_pct),
                    Style::default().fg(Theme::LOW),
                ),
            ]),
            Line::from(""),
        ];

        // Show results
        let results = ps.results();
        let open_ports: Vec<&PortResult> = results.iter().filter(|r| r.status == PortStatus::Open).collect();
        let filtered_ports: Vec<&PortResult> = results.iter().filter(|r| r.status == PortStatus::Filtered).collect();
        
        if !results.is_empty() {
            lines.push(Line::from(vec![
                Span::styled(
                    format!("Open: {}  Filtered: {}  Closed: {}", 
                        open_ports.len(), 
                        filtered_ports.len(),
                        results.len() - open_ports.len() - filtered_ports.len()
                    ),
                    Style::default().fg(Theme::FG),
                ),
            ]));
            lines.push(Line::from(""));
            
            // Show open ports first
            if !open_ports.is_empty() {
                for result in open_ports.iter().take(5) {
                    let service_text = result
                        .service
                        .as_ref()
                        .map(|s| format!(" ({s})"))
                        .unwrap_or_default();
                    lines.push(Line::from(vec![
                        Span::styled(
                            format!("{:5} ", result.port),
                            Style::default().fg(Theme::TITLE),
                        ),
                        Span::styled(
                            "OPEN",
                            Style::default().fg(Theme::GOOD).add_modifier(Modifier::BOLD),
                        ),
                        Span::styled(service_text, Style::default().fg(Theme::LOW)),
                    ]));
                }
                if open_ports.len() > 5 {
                    lines.push(Line::from(vec![
                        Span::styled(
                            format!("  ... {} more open", open_ports.len() - 5),
                            Style::default().fg(Theme::LOW),
                        ),
                    ]));
                }
            } else if ps.is_complete() {
                lines.push(Line::from(vec![
                    Span::styled("No open ports found", Style::default().fg(Theme::LOW)),
                ]));
            }
        }
        
        lines.push(Line::from(""));
        lines.push(Line::from(highlighted_key("C", "Close")));

        let text = Paragraph::new(lines)
            .block(
                Block::default()
                    .title(" PORT SCAN ")
                    .title_style(Style::default().fg(Theme::TITLE).add_modifier(Modifier::BOLD))
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Theme::BOX)),
            )
            .style(Style::default().bg(Theme::BG).fg(Theme::FG));

        f.render_widget(text, area);
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}