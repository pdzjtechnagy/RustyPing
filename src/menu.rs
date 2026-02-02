use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph},
    Frame, Terminal,
};
use crate::storage::TargetHistory;
use crate::theme::Theme;

pub struct MenuApp {
    input: String,
    history: Vec<String>,
    defaults: Vec<String>,
    list_state: ListState,
    selected_section: SelectionSection,
    theme: Theme,
}

#[derive(PartialEq, Clone, Copy)]
enum SelectionSection {
    Input,
    History,
    Defaults,
}

impl MenuApp {
    pub fn new(history: &TargetHistory, theme: Theme) -> Self {
        let mut list_state = ListState::default();
        if !history.entries.is_empty() {
            list_state.select(Some(0));
        }

        Self {
            input: String::new(),
            history: history.entries.iter().map(|e| e.target.clone()).collect(),
            defaults: vec![
                "1.1.1.1".to_string(),
                "8.8.8.8".to_string(),
                "google.com".to_string(),
                "github.com".to_string(),
                "wikipedia.org".to_string(),
            ],
            list_state,
            selected_section: if history.entries.is_empty() {
                SelectionSection::Defaults
            } else {
                SelectionSection::History
            },
            theme,
        }
    }

    pub fn run<B: Backend>(mut self, terminal: &mut Terminal<B>) -> anyhow::Result<Option<String>> {
        // Initial selection logic
        if self.selected_section == SelectionSection::Defaults && !self.defaults.is_empty() {
             self.list_state.select(Some(0));
        }

        loop {
            terminal.draw(|f| self.ui(f))?;

            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Esc => return Ok(None),
                        KeyCode::Enter => {
                            if !self.input.is_empty() {
                                return Ok(Some(self.input.trim().to_string()));
                            }
                            if let Some(i) = self.list_state.selected() {
                                match self.selected_section {
                                    SelectionSection::History => {
                                        if i < self.history.len() {
                                            return Ok(Some(self.history[i].clone()));
                                        }
                                    }
                                    SelectionSection::Defaults => {
                                        if i < self.defaults.len() {
                                            return Ok(Some(self.defaults[i].clone()));
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                        KeyCode::Char(c) => {
                            self.input.push(c);
                            self.selected_section = SelectionSection::Input;
                            self.list_state.select(None);
                        }
                        KeyCode::Backspace => {
                            self.input.pop();
                            if self.input.is_empty() {
                                // If input cleared, jump back to lists
                                if !self.history.is_empty() {
                                    self.selected_section = SelectionSection::History;
                                    self.list_state.select(Some(0));
                                } else {
                                    self.selected_section = SelectionSection::Defaults;
                                    self.list_state.select(Some(0));
                                }
                            } else {
                                self.selected_section = SelectionSection::Input;
                                self.list_state.select(None);
                            }
                        }
                        KeyCode::Up => self.move_vertical(-1),
                        KeyCode::Down => self.move_vertical(1),
                        KeyCode::Left => self.move_horizontal(-1),
                        KeyCode::Right => self.move_horizontal(1),
                        KeyCode::Tab => self.cycle_section(),
                        _ => {}
                    }
                }
            }
        }
    }

    fn move_vertical(&mut self, delta: i32) {
        match self.selected_section {
            SelectionSection::Input => {
                if delta > 0 {
                    // Moving down from input goes to lists
                    if !self.history.is_empty() {
                        self.selected_section = SelectionSection::History;
                        self.list_state.select(Some(0));
                    } else {
                        self.selected_section = SelectionSection::Defaults;
                        self.list_state.select(Some(0));
                    }
                }
            }
            SelectionSection::History | SelectionSection::Defaults => {
                let list_len = if self.selected_section == SelectionSection::History {
                    self.history.len()
                } else {
                    self.defaults.len()
                };

                let i = self.list_state.selected().unwrap_or(0);
                if delta < 0 {
                    if i == 0 {
                        self.selected_section = SelectionSection::Input;
                        self.list_state.select(None);
                    } else {
                        self.list_state.select(Some(i - 1));
                    }
                } else {
                    if i + 1 < list_len {
                        self.list_state.select(Some(i + 1));
                    }
                }
            }
        }
    }

    fn move_horizontal(&mut self, delta: i32) {
        match self.selected_section {
            SelectionSection::History => {
                if delta > 0 {
                    self.selected_section = SelectionSection::Defaults;
                    if let Some(i) = self.list_state.selected() {
                         self.list_state.select(Some(i.min(self.defaults.len().saturating_sub(1))));
                    } else {
                        self.list_state.select(Some(0));
                    }
                }
            }
            SelectionSection::Defaults => {
                if delta < 0 && !self.history.is_empty() {
                    self.selected_section = SelectionSection::History;
                    if let Some(i) = self.list_state.selected() {
                        self.list_state.select(Some(i.min(self.history.len().saturating_sub(1))));
                   } else {
                       self.list_state.select(Some(0));
                   }
                }
            }
            _ => {}
        }
    }

    fn cycle_section(&mut self) {
        match self.selected_section {
            SelectionSection::Input => {
                if !self.history.is_empty() {
                    self.selected_section = SelectionSection::History;
                    self.list_state.select(Some(0));
                } else {
                    self.selected_section = SelectionSection::Defaults;
                    self.list_state.select(Some(0));
                }
            }
            SelectionSection::History => {
                self.selected_section = SelectionSection::Defaults;
                self.list_state.select(Some(0));
            }
            SelectionSection::Defaults => {
                self.selected_section = SelectionSection::Input;
                self.list_state.select(None);
            }
        }
    }

    fn ui(&mut self, f: &mut Frame) {
        let area = f.area();
        
        // Centered layout
        let vertical_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(2),
                Constraint::Length(25), // Height of the menu
                Constraint::Min(2),
            ])
            .split(area);

        let centered_rect = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Min(2),
                Constraint::Length(80), // Width of the menu
                Constraint::Min(2),
            ])
            .split(vertical_layout[1])[1];

        // Clear the area to avoid artifacts
        f.render_widget(Clear, centered_rect);

        // Main block
        let main_block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(self.theme.box_color))
            .title(Span::styled(" RustyPing Startup ", Style::default().fg(self.theme.title).add_modifier(Modifier::BOLD)));
        
        f.render_widget(main_block.clone(), centered_rect);

        // Inner layout
        let inner_area = main_block.inner(centered_rect);
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(4), // Header/Title
                Constraint::Length(3), // Input
                Constraint::Min(10),   // Lists
                Constraint::Length(3), // Footer
            ])
            .margin(1)
            .split(inner_area);

        // Header
        let title_text = vec![
            Line::from(vec![
                Span::styled("Rusty", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
                Span::styled("Ping", Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
                Span::raw(" v2.2.0"),
            ]),
            Line::from("High-performance network monitoring"),
        ];
        let title = Paragraph::new(title_text)
            .alignment(Alignment::Center)
            .style(Style::default().fg(self.theme.fg));
        f.render_widget(title, chunks[0]);

        // Input
        let input_style = if self.selected_section == SelectionSection::Input {
            Style::default().fg(self.theme.hi_fg).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(self.theme.fg)
        };
        
        let input_block = Block::default()
            .borders(Borders::ALL)
            .border_style(if self.selected_section == SelectionSection::Input {
                Style::default().fg(self.theme.hi_fg)
            } else {
                Style::default().fg(self.theme.box_color)
            })
            .title(" Manual Entry ");

        let input_content = if self.input.is_empty() {
            Span::styled("Type IP or hostname...", Style::default().fg(self.theme.low))
        } else {
            Span::raw(&self.input)
        };

        let input = Paragraph::new(input_content)
            .style(input_style)
            .block(input_block);
        f.render_widget(input, chunks[1]);

        // Lists Layout
        let list_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ])
            .split(chunks[2]);

        // Helper for list rendering
        let render_list = |title: &str, items: &[String], section: SelectionSection, f: &mut Frame, area: Rect, state: &mut ListState, theme: &Theme| {
            let list_items: Vec<ListItem> = items
                .iter()
                .map(|i| ListItem::new(format!("  {}", i)))
                .collect();

            let is_selected = self.selected_section == section;
            let border_style = if is_selected {
                Style::default().fg(theme.hi_fg)
            } else {
                Style::default().fg(theme.low)
            };

            let list = List::new(list_items)
                .block(Block::default()
                    .borders(Borders::ALL)
                    .title(format!(" {} ", title))
                    .border_style(border_style))
                .highlight_style(Style::default().bg(theme.selected_bg).fg(theme.selected_fg).add_modifier(Modifier::BOLD))
                .highlight_symbol(">> ");

            if is_selected {
                f.render_stateful_widget(list, area, state);
            } else {
                f.render_widget(list, area);
            }
        };

        render_list("Recent History", &self.history, SelectionSection::History, f, list_chunks[0], &mut self.list_state, &self.theme);
        render_list("Common Targets", &self.defaults, SelectionSection::Defaults, f, list_chunks[1], &mut self.list_state, &self.theme);

        // Footer
        let footer_text = Line::from(vec![
            Span::styled("TAB", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" switch lists • "),
            Span::styled("↑/↓/←/→", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" navigate • "),
            Span::styled("ENTER", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" select • "),
            Span::styled("ESC", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" quit"),
        ]);
        let footer = Paragraph::new(footer_text)
            .alignment(Alignment::Center)
            .style(Style::default().fg(self.theme.low));
        f.render_widget(footer, chunks[3]);
    }
}
