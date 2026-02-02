use crate::app::App;
use crate::network::{PortResult, PortStatus, SpeedTestState};
use crate::theme::Theme;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    symbols,
    text::{Line, Span},
    widgets::{
        canvas::{Canvas, Line as CanvasLine},
        Block, Borders, Gauge, Paragraph,
    },
    Frame,
};

pub fn draw(f: &mut Frame, app: &mut App) {
    let area = f.area();
    
    // ULTRA-COMPACT MODE (Tiny boxes, e.g., 20x5)
    // If very small, show ONLY the graph.
    if area.height < 12 || area.width < 50 {
        draw_latency_graph(f, app, area);
        // Overlay settings if open, even in tiny mode
        if app.show_settings {
            draw_settings_overlay(f, app);
        }
        return;
    }

    // Calculate dynamic constraints based on terminal size
    let header_height = 3;
    let footer_height = 1;
    
    // Check if we need to show panels
    let has_speedtest = app.speedtest.is_some();
    let has_portscan = app.portscan.is_some();
    let has_panels = has_speedtest || has_portscan;
    
    // Adjust layout based on panels and available height
    let stats_height = if area.height >= 35 {
        13 // Full stats panel
    } else if area.height >= 28 {
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
    
    // Calculate available graph height
    let used_height = header_height + stats_height + footer_height + panel_height;
    let graph_height = area.height.saturating_sub(used_height);

    // Layout chunks
    let constraints = if has_panels {
        vec![
            Constraint::Length(header_height),
            Constraint::Min(graph_height.max(5)), // Graph with minimum height
            Constraint::Length(stats_height),
            Constraint::Length(panel_height),
            Constraint::Length(footer_height),
        ]
    } else {
        vec![
            Constraint::Length(header_height),
            Constraint::Min(graph_height.max(5)), // Graph with minimum height
            Constraint::Length(stats_height),
            Constraint::Length(footer_height),
        ]
    };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(area);

    // Safe access to chunks (in case layout fails or is different)
    if let Some(chunk) = chunks.first() { draw_header(f, app, *chunk); }
    if let Some(chunk) = chunks.get(1) { draw_latency_graph(f, app, *chunk); }
    
    // Logic to handle variable chunk indices based on panels
    let stats_idx = 2;
    let panel_idx = if has_panels { 3 } else { 999 }; // 999 = invalid
    let footer_idx = if has_panels { 4 } else { 3 };

    if let Some(chunk) = chunks.get(stats_idx) {
        // Bottom section - responsive layout
        if app.show_jitter && area.width >= 100 {
            let bottom_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(*chunk);
            draw_statistics(f, app, bottom_chunks[0]);
            draw_jitter_panel(f, app, bottom_chunks[1]);
        } else {
            draw_statistics(f, app, *chunk);
        }
    }

    // Panels (speedtest/portscan)
    if has_panels {
        if let Some(panel_area) = chunks.get(panel_idx) {
            if has_speedtest && has_portscan {
                let panel_chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                    .split(*panel_area);
                draw_speedtest_panel(f, app, panel_chunks[0]);
                draw_portscan_panel(f, app, panel_chunks[1]);
            } else if has_speedtest {
                draw_speedtest_panel(f, app, *panel_area);
            } else if has_portscan {
                draw_portscan_panel(f, app, *panel_area);
            }
        }
    }

    // Footer
    if let Some(chunk) = chunks.get(footer_idx) {
        draw_footer(f, app, *chunk);
    }

    // Settings overlay (rendered last so it's on top)
    if app.show_settings {
        draw_settings_overlay(f, app);
    }
}

fn draw_header(f: &mut Frame, app: &App, area: Rect) {
    let quality = app.ping_monitor.quality();
    let status_color = app.theme.quality_color(&quality);
    let stats = app.ping_monitor.stats();

    // Enhanced header with more information
    let ip_display = app.ping_monitor.get_target_addr().to_string();
    
    let text = vec![Line::from(vec![
        Span::styled(" RustyPing ", Style::default().fg(app.theme.title).add_modifier(Modifier::BOLD)),
        Span::raw("│"),
        Span::styled(format!(" {ip_display} "), Style::default().fg(app.theme.low)),
        Span::raw("│"),
        Span::styled(format!(" Target: {} ", &app.target), Style::default().fg(app.theme.hi_fg)),
        Span::raw("│"),
        Span::styled(" ● ", Style::default().fg(status_color)),
        Span::styled(quality, Style::default().fg(status_color).add_modifier(Modifier::BOLD)),
        Span::raw(" │"),
        Span::styled(
            format!(" Packets: {} ", stats.total_pings),
            Style::default().fg(app.theme.low),
        ),
    ])];

    let header = Paragraph::new(text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(app.theme.box_color))
        )
        .style(Style::default().bg(app.theme.bg).fg(app.theme.fg));

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
                    Style::default().fg(app.theme.low),
                ),
            ]),
        ];
        let paragraph = Paragraph::new(empty_text)
            .block(
                Block::default()
                    .title("")
                    .title_style(Style::default().fg(app.theme.title))
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(app.theme.box_color)),
            )
            .style(Style::default().bg(app.theme.bg).fg(app.theme.fg));
        f.render_widget(paragraph, area);
        return;
    }

    // Calculate bounds based on successful pings
    let valid_pings: Vec<f64> = data.iter().filter_map(|&v| v).collect();
    
    let (min_latency, max_latency) = if valid_pings.is_empty() {
        (0.0, 100.0) // Default range if all pings failed
    } else {
        let min = valid_pings.iter().fold(f64::INFINITY, |a, b| a.min(*b)).min(0.0);
        let max = valid_pings.iter().fold(0.0_f64, |a, b| a.max(*b)).max(50.0);
        (min, max)
    };
    
    // Add 10% padding on top and bottom for better visibility
    let y_padding: f64 = (max_latency - min_latency).max(10.0) * 0.1;
    let y_min: f64 = (min_latency - y_padding).max(0.0);
    let y_max: f64 = max_latency + y_padding;

    // Calculate time window label
    let time_window = format!("last {}s", data.len());

    // IP Display
    let ip_display = app.ping_monitor.get_target_addr().to_string();
    let title_text = format!(" {ip_display} │ {time_window} ");

    // BRAILLE CANVAS - High-resolution rendering!
    // Right-to-Left Scrolling: Newest data is on the RIGHT side.
    
    // Calculate the actual available width in braille dots.
    // Each character cell contains 2 braille dot columns.
    // We subtract 2 from width for borders (left/right).
    let inner_width_chars = area.width.saturating_sub(2);
    let canvas_width_dots = (inner_width_chars as f64) * 2.0;

    let canvas = Canvas::default()
        .block(
            Block::default()
                .title(title_text)
                .title_style(Style::default().fg(app.theme.title).add_modifier(Modifier::BOLD))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(app.theme.box_color)),
        )
        .marker(symbols::Marker::Braille)
        // Set X bounds to match screen dot resolution exactly.
        // This ensures 1.0 unit = 1 dot column.
        .x_bounds([0.0, canvas_width_dots])
        .y_bounds([y_min, y_max])
        .paint(move |ctx| {
            // Right-Side Justification Logic
            // We align the newest data point to the absolute rightmost available dot column.
            // Coordinate System: 0.0 (Left) -> canvas_width_dots (Right)
            
            let data_len = data.len();
            let right_edge = canvas_width_dots - 1.0; // The last visible column index

            for (i, val_opt) in data.iter().enumerate() {
                // Calculate age: 0 = newest, 1 = second newest...
                // The data is pushed to back, so the last element is the newest.
                // data[i] where i = data_len - 1 is the newest.
                let age = data_len - 1 - i;
                
                // Map to screen coordinate
                let x_pos = right_edge - (age as f64);
                
                // Optimization: Skip points that are off the left side of the screen
                if x_pos < 0.0 {
                    continue;
                }

                // Ensure strict integer alignment for crisp rendering
                let x_final = x_pos.floor();

                if let Some(y) = val_opt {
                    let ratio = if y_max > 0.0 { (*y / y_max).min(1.0) } else { 0.0 };
                    let color = app.theme.graph_gradient(ratio);
                    
                    // Draw a vertical line from bottom to value
                    // Using the same X for x1 and x2 ensures a 1-dot wide vertical line.
                    ctx.draw(&CanvasLine {
                        x1: x_final, 
                        y1: y_min,
                        x2: x_final,
                        y2: *y,
                        color,
                    });
                } else {
                    // Draw missed ping line (lighter grey dot line)
                    // Full height line to indicate drop
                    ctx.draw(&CanvasLine {
                        x1: x_final,
                        y1: y_min,
                        x2: x_final,
                        y2: y_max,
                        color: app.theme.missed,
                    });
                }
            }
        });

    f.render_widget(canvas, area);
}

fn draw_statistics(f: &mut Frame, app: &App, area: Rect) {
    let stats = app.ping_monitor.stats();
    
    let current_color = if let Some(rt) = stats.current_response {
        app.theme.latency_color(rt)
    } else {
        app.theme.crit
    };

    // Enhanced statistics display with better formatting
    let text = vec![
        Line::from(vec![
            Span::styled("Current:   ", Style::default().fg(app.theme.low)),
            Span::styled(
                format!("{:>8.1} ms", stats.current_response.unwrap_or(0.0)),
                Style::default().fg(current_color).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("Avg (10):  ", Style::default().fg(app.theme.low)),
            Span::styled(
                format!("{:>8.1} ms", stats.current_avg),
                Style::default().fg(app.theme.latency_color(stats.current_avg)),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Session Statistics:", Style::default().fg(app.theme.title).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled("  Avg:    ", Style::default().fg(app.theme.low)),
            Span::styled(format!("{:>8.1} ms", stats.avg_response), Style::default().fg(app.theme.fg)),
        ]),
        Line::from(vec![
            Span::styled("  Min:    ", Style::default().fg(app.theme.low)),
            Span::styled(format!("{:>8.1} ms", stats.min_response), Style::default().fg(app.theme.good)),
        ]),
        Line::from(vec![
            Span::styled("  Max:    ", Style::default().fg(app.theme.low)),
            Span::styled(format!("{:>8.1} ms", stats.max_response), Style::default().fg(app.theme.warn)),
        ]),
        Line::from(vec![
            Span::styled("  Jitter: ", Style::default().fg(app.theme.low)),
            Span::styled(format!("{:>8.1} ms", stats.jitter), Style::default().fg(app.theme.fg)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Uptime:    ", Style::default().fg(app.theme.good)),
            Span::styled(format!("{:>6.1}%", stats.uptime_pct), Style::default().fg(app.theme.fg)),
        ]),
        Line::from(vec![
            Span::styled("Loss:      ", Style::default().fg(app.theme.crit)),
            Span::styled(format!("{:>6.1}%", stats.packet_loss_pct), Style::default().fg(app.theme.fg)),
        ]),
        Line::from(""),
        Line::from(highlighted_key(&app.theme, "R", "Reset Stats")),
    ];

    let paragraph = Paragraph::new(text)
        .block(
            Block::default()
                .title(" STATISTICS ")
                .title_style(Style::default().fg(app.theme.title).add_modifier(Modifier::BOLD))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(app.theme.box_color)),
        )
        .style(Style::default().bg(app.theme.bg).fg(app.theme.fg));

    f.render_widget(paragraph, area);
}

fn draw_jitter_panel(f: &mut Frame, app: &App, area: Rect) {
    let stats = app.ping_monitor.stats();
    
    // Calculate gauge color based on stability
    let gauge_color = if stats.stability >= 90.0 {
        app.theme.good
    } else if stats.stability >= 70.0 {
        app.theme.warn
    } else {
        app.theme.crit
    };

    let text = vec![
        Line::from(vec![
            Span::styled("Jitter:       ", Style::default().fg(app.theme.low)),
            Span::styled(
                format!("{:>8.1} ms", stats.jitter),
                Style::default().fg(app.theme.fg),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Stability:    ", Style::default().fg(app.theme.low)),
            Span::styled(
                format!("{:>6.0}%", stats.stability),
                Style::default().fg(gauge_color).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(""),
        Line::from(highlighted_key(&app.theme, "S", "Speed Test")),
        Line::from(highlighted_key(&app.theme, "P", "Port Scan")),
    ];

    let paragraph = Paragraph::new(text)
        .block(
            Block::default()
                .title(" QUALITY & ACTIONS ")
                .title_style(Style::default().fg(app.theme.title).add_modifier(Modifier::BOLD))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(app.theme.box_color)),
        )
        .style(Style::default().bg(app.theme.bg).fg(app.theme.fg));

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
        Span::styled("[Q]", Style::default().fg(app.theme.key_highlight).add_modifier(Modifier::BOLD)),
        Span::styled("uit", Style::default().fg(app.theme.low)),
        Span::raw(" │ "),
        Span::styled("[ESC]", Style::default().fg(app.theme.key_highlight).add_modifier(Modifier::BOLD)),
        Span::styled("ettings", Style::default().fg(app.theme.low)),
    ];
    
    // Only show S/P if panels aren't open
    if app.speedtest.is_none() && app.portscan.is_none() {
        spans.extend(vec![
            Span::raw(" │ "),
            Span::styled("[S]", Style::default().fg(app.theme.key_highlight).add_modifier(Modifier::BOLD)),
            Span::styled("peed", Style::default().fg(app.theme.low)),
            Span::raw(" │ "),
            Span::styled("[P]", Style::default().fg(app.theme.key_highlight).add_modifier(Modifier::BOLD)),
            Span::styled("ort", Style::default().fg(app.theme.low)),
        ]);
    } else {
        spans.extend(vec![
            Span::raw(" │ "),
            Span::styled("[C]", Style::default().fg(app.theme.key_highlight).add_modifier(Modifier::BOLD)),
            Span::styled("lose", Style::default().fg(app.theme.low)),
        ]);
    }
    
    spans.extend(vec![
        Span::raw(" │ "),
        Span::styled("Runtime: ", Style::default().fg(app.theme.low)),
        Span::styled(
            format!("{:02}:{:02}:{:02}", 
                runtime.as_secs() / 3600,
                (runtime.as_secs() % 3600) / 60,
                runtime.as_secs() % 60
            ),
            Style::default().fg(app.theme.fg).add_modifier(Modifier::BOLD),
        ),
        Span::raw(" │ "),
        Span::styled("Pkts: ", Style::default().fg(app.theme.low)),
        Span::styled(
            format!("{}", stats.total_pings),
            Style::default().fg(app.theme.fg),
        ),
        Span::raw(" │ "),
        Span::styled("Int: ", Style::default().fg(app.theme.low)),
        Span::styled(
            format!("{}ms", app.config.ping_interval_ms),
            Style::default().fg(app.theme.hi_fg),
        ),
        Span::raw(" (↑↓) │ "),
        Span::styled("Hist: ", Style::default().fg(app.theme.low)),
        Span::styled(
            format!("{}s", app.config.graph_history_length),
            Style::default().fg(app.theme.hi_fg),
        ),
        Span::raw(" (←→)"),
    ]);

    let text = Line::from(spans);
    let paragraph = Paragraph::new(text)
        .style(Style::default().bg(app.theme.bg).fg(app.theme.fg));
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
                Style::default().fg(app.theme.title),
            ),
            Span::styled(
                if app.show_jitter { "☑" } else { "☐" },
                Style::default().fg(if app.show_jitter { app.theme.good } else { app.theme.low }),
            ),
            Span::raw(" "),
            Span::styled(
                "Show Jitter Panel",
                Style::default()
                    .fg(if app.settings_selected == 0 { app.theme.hi_fg } else { app.theme.fg }),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                if app.settings_selected == 1 { " ▶ " } else { "   " },
                Style::default().fg(app.theme.title),
            ),
            Span::styled(
                if app.show_history { "☑" } else { "☐" },
                Style::default().fg(if app.show_history { app.theme.good } else { app.theme.low }),
            ),
            Span::raw(" "),
            Span::styled(
                "Show History Panel",
                Style::default()
                    .fg(if app.settings_selected == 1 { app.theme.hi_fg } else { app.theme.fg }),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                if app.settings_selected == 2 { " ▶ " } else { "   " },
                Style::default().fg(app.theme.title),
            ),
            Span::styled(
                if app.config.pause_ping_during_speedtest { "☑" } else { "☐" },
                Style::default().fg(if app.config.pause_ping_during_speedtest { app.theme.good } else { app.theme.low }),
            ),
            Span::raw(" "),
            Span::styled(
                "Pause ping during speedtest",
                Style::default()
                    .fg(if app.settings_selected == 2 { app.theme.hi_fg } else { app.theme.fg }),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                " ↑/↓ ",
                Style::default().fg(app.theme.hi_fg).add_modifier(Modifier::BOLD),
            ),
            Span::styled("Navigate  ", Style::default().fg(app.theme.low)),
            Span::styled(
                " Enter ",
                Style::default().fg(app.theme.hi_fg).add_modifier(Modifier::BOLD),
            ),
            Span::styled("Toggle  ", Style::default().fg(app.theme.low)),
            Span::styled(
                " ESC ",
                Style::default().fg(app.theme.hi_fg).add_modifier(Modifier::BOLD),
            ),
            Span::styled("Close", Style::default().fg(app.theme.low)),
        ]),
    ];

    let paragraph = Paragraph::new(text)
        .block(
            Block::default()
                .title(" SETTINGS ")
                .title_style(Style::default().fg(app.theme.title).add_modifier(Modifier::BOLD))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(app.theme.title)),
        )
        .style(Style::default().bg(app.theme.selected_bg).fg(app.theme.fg));

    f.render_widget(paragraph, area);
}

// Helper function to create highlighted key spans (btop-style)
fn highlighted_key<'a>(theme: &Theme, key: &'a str, label: &'a str) -> Vec<Span<'a>> {
    vec![
        Span::styled(
            format!("[{key}]"),
            Style::default().fg(theme.key_highlight).add_modifier(Modifier::BOLD),
        ),
        Span::raw(" "),
        Span::styled(label, Style::default().fg(theme.low)),
    ]
}

fn draw_speedtest_panel(f: &mut Frame, app: &App, area: Rect) {
    if let Some(ref st) = app.speedtest {
        let mut lines = vec![Line::from("")];
        
        match st.get_state() {
            SpeedTestState::Preparing => {
                lines.push(Line::from(vec![
                    Span::styled("Preparing speed test...", Style::default().fg(app.theme.title)),
                ]));
            }
            SpeedTestState::Downloading { bytes_received, .. } => {
                lines.push(Line::from(vec![
                    Span::styled("Download Speed Test", Style::default().fg(app.theme.title).add_modifier(Modifier::BOLD)),
                ]));
                lines.push(Line::from(""));
                lines.push(Line::from(vec![
                    Span::styled("Downloading...", Style::default().fg(app.theme.title)),
                ]));
                lines.push(Line::from(vec![
                    Span::styled("Bytes: ", Style::default().fg(app.theme.low)),
                    Span::styled(format!("{:.2} MB", *bytes_received as f64 / 1_000_000.0), Style::default().fg(app.theme.fg)),
                ]));
            }
            SpeedTestState::Uploading { bytes_sent, download_results, .. } => {
                lines.push(Line::from(vec![
                    Span::styled("Upload Speed Test", Style::default().fg(app.theme.title).add_modifier(Modifier::BOLD)),
                ]));
                lines.push(Line::from(""));
                lines.push(Line::from(vec![
                    Span::styled("Uploading...", Style::default().fg(app.theme.title)),
                ]));
                lines.push(Line::from(vec![
                    Span::styled("Download: ", Style::default().fg(app.theme.low)),
                    Span::styled(format!("{:.2} Mbps", download_results.0), Style::default().fg(app.theme.good)),
                ]));
                lines.push(Line::from(vec![
                    Span::styled("Bytes: ", Style::default().fg(app.theme.low)),
                    Span::styled(format!("{:.2} MB", *bytes_sent as f64 / 1_000_000.0), Style::default().fg(app.theme.fg)),
                ]));
            }
            SpeedTestState::Complete { download_mbps, upload_mbps, total_bytes, duration, avg_speed, peak_speed } => {
                lines.push(Line::from(vec![
                    Span::styled("Speed Test Complete", Style::default().fg(app.theme.good).add_modifier(Modifier::BOLD)),
                ]));
                lines.push(Line::from(""));
                lines.push(Line::from(vec![
                    Span::styled("Download: ", Style::default().fg(app.theme.low)),
                    Span::styled(
                        format!("{download_mbps:.2} Mbps"),
                        Style::default().fg(app.theme.good).add_modifier(Modifier::BOLD),
                    ),
                ]));
                lines.push(Line::from(vec![
                    Span::styled("Upload:   ", Style::default().fg(app.theme.low)),
                    Span::styled(
                        format!("{upload_mbps:.2} Mbps"),
                        Style::default().fg(app.theme.good).add_modifier(Modifier::BOLD),
                    ),
                ]));
                lines.push(Line::from(vec![
                    Span::styled("Peak DL:  ", Style::default().fg(app.theme.low)),
                    Span::styled(
                        format!("{peak_speed:.2} Mbps"),
                        Style::default().fg(app.theme.fg),
                    ),
                ]));
                lines.push(Line::from(vec![
                    Span::styled("Avg DL:   ", Style::default().fg(app.theme.low)),
                    Span::styled(
                        format!("{avg_speed:.2} Mbps"),
                        Style::default().fg(app.theme.fg),
                    ),
                ]));
                lines.push(Line::from(vec![
                    Span::styled("Data:     ", Style::default().fg(app.theme.low)),
                    Span::styled(
                        format!("{:.2} MB", *total_bytes as f64 / 1_000_000.0),
                        Style::default().fg(app.theme.fg),
                    ),
                ]));
                lines.push(Line::from(vec![
                    Span::styled("Duration: ", Style::default().fg(app.theme.low)),
                    Span::styled(
                        format!("{:.1}s", duration.as_secs_f64()),
                        Style::default().fg(app.theme.fg),
                    ),
                ]));
            }
            SpeedTestState::Error(msg) => {
                lines.push(Line::from(vec![
                    Span::styled("Speed Test Error", Style::default().fg(app.theme.crit).add_modifier(Modifier::BOLD)),
                ]));
                lines.push(Line::from(""));
                lines.push(Line::from(vec![
                    Span::styled("Error: ", Style::default().fg(app.theme.crit)),
                    Span::styled(msg.clone(), Style::default().fg(app.theme.fg)),
                ]));
            }
        }
        
        lines.push(Line::from(""));
        lines.push(Line::from(highlighted_key(&app.theme, "C", "Close")));
        
        let paragraph = Paragraph::new(lines)
            .block(
                Block::default()
                    .title(" SPEED TEST ")
                    .title_style(Style::default().fg(app.theme.title).add_modifier(Modifier::BOLD))
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(app.theme.box_color)),
            )
            .style(Style::default().bg(app.theme.bg).fg(app.theme.fg));
        
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
                    Style::default().fg(app.theme.title).add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(vec![
                Span::styled(
                    format!("Progress: {current}/{total} ({progress_pct:.0}%)"),
                    Style::default().fg(app.theme.low),
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
                    Style::default().fg(app.theme.fg),
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
                            Style::default().fg(app.theme.title),
                        ),
                        Span::styled(
                            "OPEN",
                            Style::default().fg(app.theme.good).add_modifier(Modifier::BOLD),
                        ),
                        Span::styled(service_text, Style::default().fg(app.theme.low)),
                    ]));
                }
                if open_ports.len() > 5 {
                    lines.push(Line::from(vec![
                        Span::styled(
                            format!("  ... {} more open", open_ports.len() - 5),
                            Style::default().fg(app.theme.low),
                        ),
                    ]));
                }
            } else if ps.is_complete() {
                lines.push(Line::from(vec![
                    Span::styled("No open ports found", Style::default().fg(app.theme.low)),
                ]));
            }
        }
        
        lines.push(Line::from(""));
        lines.push(Line::from(highlighted_key(&app.theme, "C", "Close")));

        let text = Paragraph::new(lines)
            .block(
                Block::default()
                    .title(" PORT SCAN ")
                    .title_style(Style::default().fg(app.theme.title).add_modifier(Modifier::BOLD))
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(app.theme.box_color)),
            )
            .style(Style::default().bg(app.theme.bg).fg(app.theme.fg));

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
