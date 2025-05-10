use std::io;

use crate::{bindings::KeyBindings, process::ProcessInfo};
use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::*,
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
};

enum Mode {
    Normal,
    FilterInput,
}

fn apply_filter<'a>(processes: &'a [ProcessInfo], keyword: &str) -> Vec<ProcessInfo> {
    let keyword = keyword.to_lowercase();
    processes
        .iter()
        .filter(|p| {
            p.pid.to_string().contains(&keyword)
                || p.name.to_lowercase().contains(&keyword)
                || p.cmd.iter().any(|c| c.to_lowercase().contains(&keyword))
        })
        .cloned()
        .collect()
}
pub fn run_tui(processes: &[ProcessInfo]) -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let bindings = KeyBindings::default();
    let mut selected_index = 0;
    let mut offset = 0;
    let mut mode = Mode::Normal;
    let mut filter_input = String::new();
    let mut filtered_processes = processes.to_vec();

    loop {
        terminal.draw(|f| {
            let area = f.size();
            let height = area.height.saturating_sub(4) as usize;

            let visible = filtered_processes
                .iter()
                .skip(offset)
                .take(height)
                .enumerate()
                .map(|(i, p)| {
                    let style = if i + offset == selected_index {
                        Style::default().bg(Color::DarkGray)
                    } else {
                        Style::default()
                    };

                    Row::new(vec![
                        Cell::from(p.pid.to_string()).style(Style::default().fg(Color::Green)),
                        Cell::from(p.name.clone())
                            .style(Style::default().add_modifier(Modifier::BOLD)),
                        Cell::from(p.cmd.join(" ")).style(Style::default().fg(Color::Blue)),
                    ])
                    .style(style)
                });

            let table = Table::new(
                visible,
                [
                    Constraint::Length(8),
                    Constraint::Length(20),
                    Constraint::Min(10),
                ],
            )
            .header(
                Row::new(vec!["PID", "Name", "Command"]).style(Style::default().fg(Color::Yellow)),
            )
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("PortSage - TUI (q to quit, : to filter)"),
            )
            .column_spacing(2);

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(3), Constraint::Min(0)])
                .split(area);

            if matches!(mode, Mode::FilterInput) {
                let input = Paragraph::new(format!("filter: {filter_input}"))
                    .style(Style::default().fg(Color::Cyan))
                    .block(Block::default().borders(Borders::BOTTOM));
                f.render_widget(input, chunks[0]);
            }

            f.render_widget(table, chunks[1]);
        })?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key_event) = event::read()? {
                match mode {
                    Mode::Normal => {
                        if bindings.is_quit(&key_event) {
                            break;
                        }

                        let total = filtered_processes.len();
                        let height = terminal.size()?.height.saturating_sub(4) as usize;

                        if bindings.is_down(&key_event) {
                            if selected_index + 1 < total {
                                selected_index += 1;
                                if selected_index >= offset + height {
                                    offset = offset.saturating_add(1);
                                }
                            }
                        } else if bindings.is_up(&key_event) {
                            if selected_index > 0 {
                                selected_index -= 1;
                                if selected_index < offset {
                                    offset = offset.saturating_sub(1);
                                }
                            }
                        } else if key_event.code == KeyCode::Char(':') {
                            mode = Mode::FilterInput;
                            filter_input.clear();
                        }
                    }

                    Mode::FilterInput => match key_event.code {
                        KeyCode::Esc => {
                            mode = Mode::Normal;
                        }
                        KeyCode::Enter => {
                            filtered_processes = processes
                                .iter()
                                .filter(|p| {
                                    let keyword = filter_input.to_lowercase();
                                    p.pid.to_string().contains(&keyword)
                                        || p.name.to_lowercase().contains(&keyword)
                                        || p.cmd.iter().any(|c| c.to_lowercase().contains(&keyword))
                                })
                                .cloned()
                                .collect();
                            selected_index = 0;
                            offset = 0;
                            mode = Mode::Normal;
                        }
                        KeyCode::Char(c) => {
                            filter_input.push(c);
                            filtered_processes = apply_filter(processes, &filter_input);
                            selected_index = 0;
                            offset = 0;
                        }
                        KeyCode::Backspace => {
                            filter_input.pop();
                            filtered_processes = apply_filter(processes, &filter_input);
                            selected_index = 0;
                            offset = 0;
                        }
                        KeyCode::Enter => {
                            mode = Mode::Normal;
                        }
                        _ => {}
                    },
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;
    Ok(())
}
