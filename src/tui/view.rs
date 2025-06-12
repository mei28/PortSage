use super::detail::draw_process_detail;
use super::state::{ClipboardMessage, Mode};
use crate::process::ProcessInfo;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    prelude::*,
    widgets::{Block, Borders, Cell, Clear, Paragraph, Row, Table},
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

    if matches!(mode, Mode::Detail) {
        if let Some(proc) = processes.get(selected_index) {
            draw_floating_detail(f, proc);
        }
    }
    if matches!(mode, Mode::ConfirmKill) {
        draw_kill_confirm(f);
    }
}

fn draw_kill_confirm(f: &mut Frame) {
    let area = f.size();
    let width = 40;
    let height = 5;
    let x = (area.width.saturating_sub(width)) / 2;
    let y = (area.height.saturating_sub(height)) / 2;
    let dialog_area = Rect::new(x, y, width, height);

    // 背景を塗りつぶしてゴミ表示を防ぐ
    let clear = Paragraph::new("".repeat((width * height) as usize))
        .style(Style::default().bg(Color::Black));
    f.render_widget(clear, dialog_area);

    let text = "Kill this process? (y/n)";
    let paragraph = Paragraph::new(text)
        .block(
            Block::default()
                .title("Confirm Kill")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Red))
                .style(Style::default().bg(Color::Black)),
        )
        .style(Style::default().fg(Color::White));
    f.render_widget(paragraph, dialog_area);
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
                Cell::from(
                    p.ports
                        .iter()
                        .map(|p| p.to_string())
                        .collect::<Vec<_>>()
                        .join(", "),
                )
                .style(Style::default().fg(Color::Yellow)),
                Cell::from(p.cmd.join(" ")).style(Style::default().fg(Color::Blue)),
            ])
            .style(style)
        });

    let table = Table::new(
        rows,
        [
            Constraint::Length(8),  // PID
            Constraint::Length(20), // Name
            Constraint::Length(10), // Ports
            Constraint::Min(10),    // Command
        ],
    )
    .header(
        Row::new(vec!["PID", "Name", "Ports", "Command"]).style(Style::default().fg(Color::Yellow)),
    )
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

fn draw_floating_detail(f: &mut Frame, proc: &ProcessInfo) {
    let area = f.size();
    let width = area.width.saturating_sub(10).min(100);
    let height = 13;
    let x = (area.width.saturating_sub(width)) / 2;
    let y = (area.height.saturating_sub(height)) / 2;
    let detail_area = Rect::new(x, y, width, height);

    // 背景をクリアして透けを防ぐ
    f.render_widget(Clear, detail_area);

    let content = vec![
        format!("PID: {}", proc.pid),
        format!("Name: {}", proc.name),
        format!("Status: {}", proc.status),
        format!("CPU Usage: {:.2}%", proc.cpu_usage),
        format!("Memory: {} KB", proc.memory),
        format!("Virtual Memory: {} KB", proc.virtual_memory),
        format!(
            "Parent PID: {}",
            proc.parent_pid.map_or("N/A".into(), |p| p.to_string())
        ),
        format!("Start Time: {}", proc.start_time),
        format!("Exe: {}", proc.exe),
        format!("CWD: {}", proc.cwd),
        format!(
            "Ports: {}",
            proc.ports
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        ),
        format!("Cmd: {}", proc.cmd.join(" ")),
    ]
    .join("\n");

    let paragraph = Paragraph::new(content)
        .block(
            Block::default()
                .title("Process Detail")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Yellow)),
        )
        .style(Style::default().fg(Color::White).bg(Color::Black));

    f.render_widget(paragraph, detail_area);
}
