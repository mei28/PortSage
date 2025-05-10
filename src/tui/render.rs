use super::detail::draw_process_detail;
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
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(2),
        ])
        .split(area);

    if matches!(mode, Mode::FilterInput) {
        let input = Paragraph::new(format!("filter: {filter_input}"))
            .style(Style::default().fg(Color::Cyan))
            .block(Block::default().borders(Borders::BOTTOM));
        f.render_widget(input, chunks[0]);
    } else if matches!(mode, Mode::Detail) {
        if let Some(proc) = processes.get(selected_index) {
            let paragraph = draw_process_detail(proc);
            f.render_widget(paragraph, chunks[0]);
        }
    }

    let rows = processes
        .iter()
        .skip(offset)
        .take(20)
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
        rows,
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
            .title("PortSage - TUI"),
    )
    .column_spacing(2);

    f.render_widget(table, chunks[1]);

    if let Some((msg, ts)) = &clipboard_message.message {
        if ts.elapsed().as_secs_f32() < 2.0 {
            let p = Paragraph::new(msg.clone())
                .style(Style::default().fg(Color::Green))
                .block(Block::default().borders(Borders::TOP));
            f.render_widget(p, chunks[2]);
        }
    }
}
