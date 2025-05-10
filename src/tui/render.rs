use super::state::{ClipboardMessage, Mode};
use crate::process::ProcessInfo;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    prelude::*,
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
};

pub fn draw_ui(
    f: &mut Frame,
    area: Rect,
    processes: &[ProcessInfo],
    mode: &Mode,
    selected_index: usize,
    offset: usize,
    filter_input: &str,
    clipboard_message: &ClipboardMessage,
) {
    let height = area.height.saturating_sub(4) as usize;

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(2),
        ])
        .split(area);

    match mode {
        Mode::FilterInput => {
            let input = Paragraph::new(format!("filter: {filter_input}"))
                .style(Style::default().fg(Color::Cyan))
                .block(Block::default().borders(Borders::BOTTOM));
            f.render_widget(input, chunks[0]);
        }
        Mode::Detail => {
            if let Some(proc) = processes.get(selected_index) {
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
        _ => {}
    }

    // Table view
    let visible = processes
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
                Cell::from(p.name.clone()).style(Style::default().add_modifier(Modifier::BOLD)),
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
    .header(Row::new(vec!["PID", "Name", "Command"]).style(Style::default().fg(Color::Yellow)))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title("PortSage - TUI (q to quit, : to filter, ↹ Tab to detail)"),
    )
    .column_spacing(2);

    f.render_widget(table, chunks[1]);

    // Notification
    if let Some((msg, timestamp)) = &clipboard_message.message {
        if timestamp.elapsed().as_secs_f32() < 2.0 {
            let notif = Paragraph::new(msg.clone())
                .style(Style::default().fg(Color::Green))
                .block(Block::default().borders(Borders::TOP));
            f.render_widget(notif, chunks[2]);
        }
    }
}
