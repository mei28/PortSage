use std::time::Instant;

#[derive(Debug, PartialEq, Eq)]
pub enum Mode {
    Normal,
    FilterInput,
    Detail,
    ConfirmKill,
}

#[derive(Default)]
pub struct ClipboardMessage {
    pub message: Option<(String, Instant)>,
}
