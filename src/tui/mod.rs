mod clipboard;
mod filter;
mod render;
mod state;

use crate::{bindings::KeyBindings, process::ProcessInfo};
use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;
use std::time::Instant;

use clipboard::copy_pid_to_clipboard;
use filter::apply_filter;
use render::draw_ui;
use state::{ClipboardMessage, Mode};

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
    let mut clipboard_message = ClipboardMessage::default();

    loop {
        terminal.draw(|f| {
            draw_ui(
                f,
                f.size(),
                &filtered_processes,
                &mode,
                selected_index,
                offset,
                &filter_input,
                &clipboard_message,
            );
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
                                    offset += 1;
                                }
                            }
                        } else if bindings.is_up(&key_event) {
                            if selected_index > 0 {
                                selected_index -= 1;
                                if selected_index < offset {
                                    offset -= 1;
                                }
                            }
                        } else if key_event.code == KeyCode::Char(':') {
                            mode = Mode::FilterInput;
                            filter_input.clear();
                        } else if key_event.code == KeyCode::Enter {
                            if let Some(proc) = filtered_processes.get(selected_index) {
                                copy_pid_to_clipboard(proc, &mut clipboard_message);
                            }
                        } else if key_event.code == KeyCode::Tab {
                            mode = Mode::Detail;
                        }
                    }

                    Mode::FilterInput => match key_event.code {
                        KeyCode::Esc => mode = Mode::Normal,
                        KeyCode::Enter => mode = Mode::Normal,
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
