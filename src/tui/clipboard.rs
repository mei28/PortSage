use super::state::ClipboardMessage;
use crate::process::ProcessInfo;
use arboard::Clipboard;
use std::time::Instant;

pub fn copy_pid_to_clipboard(proc: &ProcessInfo, msg: &mut ClipboardMessage) {
    if let Ok(mut clipboard) = Clipboard::new() {
        if clipboard.set_text(proc.pid.to_string()).is_ok() {
            msg.message = Some(("✔ Copied PID to clipboard".into(), Instant::now()));
        }
    }
}
