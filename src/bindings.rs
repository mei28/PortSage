use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

#[derive(Debug, Clone)]
pub struct KeyBindings {
    pub up: Vec<KeyEvent>,
    pub down: Vec<KeyEvent>,
    pub quit: Vec<KeyEvent>,
}

impl Default for KeyBindings {
    fn default() -> Self {
        use KeyCode::*;
        Self {
            up: vec![
                KeyEvent::new(Up, KeyModifiers::NONE),
                KeyEvent::new(Char('k'), KeyModifiers::NONE),
                KeyEvent::new(Char('p'), KeyModifiers::CONTROL),
            ],
            down: vec![
                KeyEvent::new(Down, KeyModifiers::NONE),
                KeyEvent::new(Char('j'), KeyModifiers::NONE),
                KeyEvent::new(Char('n'), KeyModifiers::CONTROL),
            ],
            quit: vec![KeyEvent::new(Char('q'), KeyModifiers::NONE)],
        }
    }
}

impl KeyBindings {
    pub fn is_up(&self, key: &KeyEvent) -> bool {
        self.up.contains(key)
    }
    pub fn is_down(&self, key: &KeyEvent) -> bool {
        self.down.contains(key)
    }
    pub fn is_quit(&self, key: &KeyEvent) -> bool {
        self.quit.contains(key)
    }
}
