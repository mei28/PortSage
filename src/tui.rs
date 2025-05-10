use std::{io, time::Instant};

use crate::{bindings::KeyBindings, process::ProcessInfo};
use anyhow::Result;
use arboard::Clipboard;
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
    Detail,
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
    let mut clipboard_message: Option<(String, Instant)> = None;

    loop {
        terminal.draw(|f| {
            let area = f.size();
            let height = area.height.saturating_sub(4) as usize;

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Min(0),
                    Constraint::Length(2),
                ])
                .split(area);

            // フィルター or 詳細表示
            if matches!(mode, Mode::FilterInput) {
                let input = Paragraph::new(format!("filter: {filter_input}"))
                    .style(Style::default().fg(Color::Cyan))
                    .block(Block::default().borders(Borders::BOTTOM));
                f.render_widget(input, chunks[0]);
            } else if matches!(mode, Mode::Detail) {
                if let Some(proc) = filtered_processes.get(selected_index) {
                    let detail = format!(
                        "PID: {}\nName: {}\nCmd: {}\nExe: {}",
                        proc.pid,
                        proc.name,
                        proc.cmd.join(" "),
                        proc.exe,
                    );
                    let paragraph = Paragraph::new(detail)
                        .style(Style::default().fg(Color::White))
                        .block(Block::default().title("Details").borders(Borders::ALL));
                    f.render_widget(paragraph, chunks[0]);
                }
            }

            // プロセステーブル
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
                    .title("PortSage - TUI (q to quit, : to filter, ↹ Tab to detail)"),
            )
            .column_spacing(2);

            f.render_widget(table, chunks[1]);

            // 通知があれば下部に表示
            if let Some((msg, timestamp)) = &clipboard_message {
                if timestamp.elapsed().as_secs_f32() < 2.0 {
                    let notif = Paragraph::new(msg.clone())
                        .style(Style::default().fg(Color::Green))
                        .block(Block::default().borders(Borders::TOP));
                    f.render_widget(notif, chunks[2]);
                }
            }
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
                        } else if key_event.code == KeyCode::Enter {
                            if let Some(proc) = filtered_processes.get(selected_index) {
                                if let Ok(mut clipboard) = Clipboard::new() {
                                    let _ = clipboard.set_text(proc.pid.to_string());
                                    clipboard_message = Some((
                                        "✔ Copied PID to clipboard".to_string(),
                                        Instant::now(),
                                    ));
                                }
                            }
                        } else if key_event.code == KeyCode::Tab {
                            mode = Mode::Detail;
                        }
                    }

                    Mode::FilterInput => match key_event.code {
                        KeyCode::Esc => {
                            mode = Mode::Normal;
                        }
                        KeyCode::Enter => {
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
                        _ => {}
                    },

                    Mode::Detail => {
                        if key_event.code == KeyCode::Esc {
                            mode = Mode::Normal;
                        }
                    }
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;
    Ok(())
}

fn apply_filter(processes: &[ProcessInfo], keyword: &str) -> Vec<ProcessInfo> {
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
