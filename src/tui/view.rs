use super::detail::draw_process_detail;
use super::state::{ClipboardMessage, Mode};
use crate::process::ProcessInfo;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    prelude::*,
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
};

pub fn draw_view(
    f: &mut Frame,
    processes: &[ProcessInfo],
    selected_index: usize,
    offset: usize,
    filter_input: &str,
    mode: &Mode,
    clipboard_message: &ClipboardMessage,
) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // header
            Constraint::Min(1),    // table
            Constraint::Length(3), // message
        ])
        .split(f.size());

    draw_header(f, layout[0], filter_input, mode);
    draw_table(f, layout[1], processes, selected_index, offset);
    draw_clipboard_message(f, layout[2], clipboard_message);
}

fn draw_header(f: &mut Frame, area: Rect, filter_input: &str, mode: &Mode) {
    let text = match mode {
        Mode::FilterInput => format!("Filter: {filter_input}"),
        _ => "PortSage - TUI (↑/↓/j/k: move, enter: copy pid, tab: detail, q: quit)".to_string(),
    };
    let paragraph = Paragraph::new(text)
        .style(Style::default().fg(Color::Cyan))
        .block(Block::default().borders(Borders::BOTTOM));
    f.render_widget(paragraph, area);
}

fn draw_table(
    f: &mut Frame,
    area: Rect,
    processes: &[ProcessInfo],
    selected_index: usize,
    offset: usize,
) {
    let rows = processes
        .iter()
        .skip(offset)
        .take((area.height - 2) as usize)
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
    .block(Block::default().borders(Borders::ALL))
    .column_spacing(2);

    f.render_widget(table, area);
}

fn draw_clipboard_message(f: &mut Frame, area: Rect, clipboard_message: &ClipboardMessage) {
    if let Some((msg, ts)) = &clipboard_message.message {
        if ts.elapsed().as_secs_f32() < 2.0 {
            let p = Paragraph::new(msg.clone())
                .style(Style::default().fg(Color::Green))
                .block(Block::default().borders(Borders::TOP));
            f.render_widget(p, area);
        }
    }
}
