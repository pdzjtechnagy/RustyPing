mod app;
mod network;
mod storage;
mod theme;
mod ui;
#[cfg(test)]
mod tests;

use app::App;
use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};
use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<()> {
    // Parse CLI arguments
    let mut target_arg = None;
    let mut monotone = false;
    let args = std::env::args().skip(1);

    for arg in args {
        match arg.as_str() {
            "--list" => {
                let history = storage::TargetHistory::load()?;
                history.print_recent();
                return Ok(());
            }
            "--select" | "-s" => {
                let history = storage::TargetHistory::load()?;
                target_arg = Some(history.fuzzy_select()?);
            }
            "--monotone" | "-m" => {
                monotone = true;
            }
            _ => {
                if !arg.starts_with('-') {
                    target_arg = Some(arg);
                }
            }
        }
    }

    // Set theme mode
    crate::theme::Theme::set_monotone(monotone);

    let target = match target_arg {
        Some(t) => t,
        None => {
            let history = storage::TargetHistory::load()?;
            if history.is_empty() {
                print!("Enter target IP or hostname: ");
                io::stdout().flush()?;
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                input.trim().to_string()
            } else {
                history.fuzzy_select()?
            }
        }
    };

    if target.is_empty() {
        eprintln!("Error: Target cannot be empty");
        return Ok(());
    }

    // Save target to history
    let mut history = storage::TargetHistory::load()?;
    history.add_target(&target);
    history.save()?;

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app
    let mut app = App::new(target).await?;

    // Run app
    let result = run_app(&mut terminal, &mut app).await;

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    // Save final stats and config
    let stats = app.ping_monitor.stats();
    let mut history = storage::TargetHistory::load()?;
    
    // Update config with any changes made during session
    history.config = app.config;
    
    history.update_stats(&app.target, stats.avg_response, stats.uptime_pct);
    history.save()?;

    result
}

async fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, app: &mut App) -> Result<()> {
    loop {
        // Render
        terminal.draw(|f| ui::draw(f, app))?;

        // Handle events
        if crossterm::event::poll(std::time::Duration::from_millis(50_u64))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        // Quit (always works)
                        KeyCode::Char('q') | KeyCode::Char('Q') => {
                            return Ok(());
                        }
                        // Close panels with C key
                        KeyCode::Char('c') | KeyCode::Char('C') => {
                            if app.speedtest.is_some() {
                                app.speedtest = None;
                            } else if app.portscan.is_some() {
                                app.portscan = None;
                            }
                        }
                        // Settings toggle
                        KeyCode::Esc => {
                            if app.show_settings {
                                app.toggle_settings();
                            } else if app.speedtest.is_none() && app.portscan.is_none() {
                                // Only toggle settings if no other modal is open
                                app.show_settings = true;
                            }
                        }
                        // Speed test
                        KeyCode::Char('s') | KeyCode::Char('S') => {
                            if !app.show_settings && app.speedtest.is_none() && app.portscan.is_none() {
                                app.start_speedtest().await?;
                            }
                        }
                        // Port scan
                        KeyCode::Char('p') | KeyCode::Char('P') => {
                            if !app.show_settings && app.speedtest.is_none() && app.portscan.is_none() {
                                app.start_portscan().await?;
                            }
                        }
                        // Other shortcuts (only when not in overlays)
                        KeyCode::Char('j') | KeyCode::Char('J') => {
                            if !app.show_settings && app.speedtest.is_none() && app.portscan.is_none() {
                                app.toggle_jitter_panel();
                            }
                        }
                        KeyCode::Char('h') | KeyCode::Char('H') => {
                            if !app.show_settings && app.speedtest.is_none() && app.portscan.is_none() {
                                app.toggle_history_panel();
                            }
                        }
                        KeyCode::Char('r') | KeyCode::Char('R') => {
                            if !app.show_settings && app.speedtest.is_none() && app.portscan.is_none() {
                                app.reset_stats();
                            }
                        }
                        
                        // Dynamic Controls (Arrow Keys)
                        KeyCode::Right => {
                            if !app.show_settings {
                                app.increase_history();
                            }
                        }
                        KeyCode::Left => {
                            if !app.show_settings {
                                app.decrease_history();
                            }
                        }
                        KeyCode::Up => {
                            if app.show_settings {
                                app.settings_navigate_up();
                            } else {
                                app.increase_speed();
                            }
                        }
                        KeyCode::Down => {
                            if app.show_settings {
                                app.settings_navigate_down();
                            } else {
                                app.decrease_speed();
                            }
                        }
                        KeyCode::Enter if app.show_settings => {
                            app.settings_toggle_selected();
                        }
                        KeyCode::Char(c) if app.show_settings && c.is_ascii_digit() => {
                            if let Some(n) = c.to_digit(10) {
                                app.settings_selected = (n as usize).min(5);
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        // Update app state
        app.tick().await?;
    }
}

