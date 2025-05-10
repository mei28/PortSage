mod clipboard;
mod detail;
mod filter;
mod render;
mod state;
mod view;

use crate::{bindings::KeyBindings, process::ProcessInfo};
use anyhow::Result;
use clipboard::copy_pid_to_clipboard;
use crossterm::{
    event::{self, Event, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use filter::apply_filter;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use state::{ClipboardMessage, Mode};
use std::io;
use view::draw_view;

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
            draw_view(
                f,
                &filtered_processes,
                selected_index,
                offset,
                &filter_input,
                &mode,
                &clipboard_message,
            );
        })?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key_event) = event::read()? {
                match mode {
                    Mode::Normal => match key_event {
                        _ if bindings.is_quit(&key_event) => break,
                        _ if bindings.is_down(&key_event) => {
                            if selected_index + 1 < filtered_processes.len() {
                                selected_index += 1;
                                if selected_index >= offset + 20 {
                                    offset += 1;
                                }
                            }
                        }
                        _ if bindings.is_up(&key_event) => {
                            if selected_index > 0 {
                                selected_index -= 1;
                                if selected_index < offset {
                                    offset = offset.saturating_sub(1);
                                }
                            }
                        }
                        _ if bindings.is_filter(&key_event) => {
                            mode = Mode::FilterInput;
                            filter_input.clear();
                        }
                        _ if bindings.is_detail(&key_event) => {
                            mode = Mode::Detail;
                        }
                        _ if bindings.is_copy(&key_event) => {
                            if let Some(proc) = filtered_processes.get(selected_index) {
                                copy_pid_to_clipboard(proc, &mut clipboard_message);
                            }
                        }
                        _ => {}
                    },
                    Mode::FilterInput => match key_event.code {
                        event::KeyCode::Esc | event::KeyCode::Enter => mode = Mode::Normal,
                        event::KeyCode::Char(c) => {
                            filter_input.push(c);
                            filtered_processes = apply_filter(processes, &filter_input);
                            selected_index = 0;
                            offset = 0;
                        }
                        event::KeyCode::Backspace => {
                            filter_input.pop();
                            filtered_processes = apply_filter(processes, &filter_input);
                            selected_index = 0;
                            offset = 0;
                        }
                        _ => {}
                    },
                    Mode::Detail => match key_event.code {
                        event::KeyCode::Esc | event::KeyCode::Char('q') => mode = Mode::Normal,
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
