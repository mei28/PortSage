use std::io;

use crate::process::ProcessInfo;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Cell, Row, Table},
};

use anyhow::Result;

pub fn run_tui(processes: &[ProcessInfo]) -> anyhow::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.draw(|f| {
            let size = f.size();

            let rows = processes.iter().map(|p| {
                Row::new(vec![
                    Cell::from(p.pid.to_string()).style(Style::default().fg(Color::Green)),
                    Cell::from(p.name.clone()).style(Style::default().add_modifier(Modifier::BOLD)),
                    Cell::from(p.cmd.join(" ")).style(Style::default().fg(Color::Blue)),
                ])
            });

            let table = Table::new(
                rows,
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
                    .title("PortSage - TUI"),
            )
            .column_spacing(2);

            f.render_widget(table, size);
        })?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;
    Ok(())
}
