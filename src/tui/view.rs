use super::detail::draw_process_detail;
use super::state::{ClipboardMessage, Mode};
use crate::process::ProcessInfo;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    prelude::*,
    widgets::{Block, Borders, Cell, Clear, Paragraph, Row, Table},
};

/// メインレイアウトの最大幅。これを超える幅は左右マージンで切り落とす
const MAX_CONTENT_WIDTH: u16 = 120;

/// 2 ペインレイアウト (左: リスト, 右: 詳細) を有効化する最小幅
const TWO_PANE_MIN_WIDTH: u16 = 100;

/// 2 ペイン時、左ペイン (プロセス一覧) が占める割合 (%)
const LIST_PANE_PERCENT: u16 = 60;

/// width が 2 ペイン表示に十分なら true。それ以下は単一ペインにフォールバック
fn is_two_pane(width: u16) -> bool {
    width >= TWO_PANE_MIN_WIDTH
}

/// area が max_width を超える場合に中央寄せでクランプした Rect を返す。
/// それ以下ならそのまま返す。フローティングダイアログなど画面全体を基準に
/// したい領域には適用しない。
fn viewport(area: Rect, max_width: u16) -> Rect {
    if area.width <= max_width {
        return area;
    }
    let x_offset = (area.width - max_width) / 2;
    Rect::new(area.x + x_offset, area.y, max_width, area.height)
}

pub fn draw_view(
    f: &mut Frame,
    processes: &[ProcessInfo],
    selected_index: usize,
    offset: usize,
    filter_input: &str,
    mode: &Mode,
    clipboard_message: &ClipboardMessage,
) {
    let area = viewport(f.size(), MAX_CONTENT_WIDTH);

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // header
            Constraint::Min(1),    // middle (list, or list+detail)
            Constraint::Length(3), // message
        ])
        .split(area);

    draw_header(f, layout[0], filter_input, mode);

    if is_two_pane(area.width) {
        let panes = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(LIST_PANE_PERCENT),
                Constraint::Percentage(100 - LIST_PANE_PERCENT),
            ])
            .split(layout[1]);
        draw_table(f, panes[0], processes, selected_index, offset);
        draw_right_pane(f, panes[1], processes.get(selected_index));
    } else {
        draw_table(f, layout[1], processes, selected_index, offset);
    }

    draw_clipboard_message(f, layout[2], clipboard_message);

    if matches!(mode, Mode::Detail) {
        if let Some(proc) = processes.get(selected_index) {
            draw_floating_detail(f, area, proc);
        }
    }
    if matches!(mode, Mode::ConfirmKill) {
        draw_kill_confirm(f, area);
    }
}

fn draw_right_pane(f: &mut Frame, area: Rect, proc: Option<&ProcessInfo>) {
    match proc {
        Some(p) => f.render_widget(detail_paragraph(p), area),
        None => {
            let empty = Paragraph::new("No process selected")
                .block(
                    Block::default()
                        .title("Process Detail")
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::DarkGray)),
                )
                .style(Style::default().fg(Color::DarkGray));
            f.render_widget(empty, area);
        }
    }
}

fn detail_paragraph(proc: &ProcessInfo) -> Paragraph<'static> {
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

    Paragraph::new(content)
        .block(
            Block::default()
                .title("Process Detail")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Yellow)),
        )
        .style(Style::default().fg(Color::White))
}

fn draw_kill_confirm(f: &mut Frame, area: Rect) {
    let width = 40;
    let height = 5;
    let x = area.x + (area.width.saturating_sub(width)) / 2;
    let y = area.y + (area.height.saturating_sub(height)) / 2;
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

fn draw_floating_detail(f: &mut Frame, area: Rect, proc: &ProcessInfo) {
    let width = area.width.saturating_sub(10).min(100);
    let height = 13;
    let x = area.x + (area.width.saturating_sub(width)) / 2;
    let y = area.y + (area.height.saturating_sub(height)) / 2;
    let detail_area = Rect::new(x, y, width, height);

    // 背景をクリアして透けを防ぐ
    f.render_widget(Clear, detail_area);
    f.render_widget(
        detail_paragraph(proc).style(Style::default().fg(Color::White).bg(Color::Black)),
        detail_area,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn viewport_clamps_wide_area_to_max_width_and_centers_it() {
        let area = Rect::new(0, 0, 200, 40);

        let result = viewport(area, 120);

        assert_eq!(result.x, 40);
        assert_eq!(result.y, 0);
        assert_eq!(result.width, 120);
        assert_eq!(result.height, 40);
    }

    #[test]
    fn viewport_returns_area_unchanged_when_narrower_than_max() {
        let area = Rect::new(0, 0, 80, 24);

        let result = viewport(area, 120);

        assert_eq!(result, area);
    }

    #[test]
    fn viewport_returns_area_unchanged_when_exactly_at_max() {
        let area = Rect::new(0, 0, 120, 30);

        let result = viewport(area, 120);

        assert_eq!(result, area);
    }

    #[test]
    fn is_two_pane_returns_false_below_threshold() {
        assert!(!is_two_pane(80));
        assert!(!is_two_pane(99));
    }

    #[test]
    fn is_two_pane_returns_true_at_or_above_threshold() {
        assert!(is_two_pane(100));
        assert!(is_two_pane(120));
        assert!(is_two_pane(200));
    }
}
