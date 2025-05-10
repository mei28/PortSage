use crate::process::ProcessInfo;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};

pub fn draw_process_detail(proc: &ProcessInfo) -> Paragraph {
    let detail = format!(
        "PID: {}\n\
         Name: {}\n\
         Status: {}\n\
         CPU Usage: {:.2}%\n\
         Memory: {} KB\n\
         Virtual Memory: {} KB\n\
         Parent PID: {}\n\
         Start Time: {}\n\
         Exe: {}\n\
         CWD: {}\n\
         Cmd: {}",
        proc.pid,
        proc.name,
        proc.status,
        proc.cpu_usage,
        proc.memory,
        proc.virtual_memory,
        proc.parent_pid.map_or("N/A".to_string(), |p| p.to_string()),
        proc.start_time,
        proc.exe,
        proc.cwd,
        proc.cmd.join(" ")
    );

    Paragraph::new(detail)
        .style(Style::default().fg(Color::White))
        .block(Block::default().title("Details").borders(Borders::ALL))
}
