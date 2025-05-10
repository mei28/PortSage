use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

#[derive(Debug, Clone)]
pub struct KeyBindings {
    pub up: Vec<KeyEvent>,
    pub down: Vec<KeyEvent>,
    pub quit: Vec<KeyEvent>,
    pub detail: Vec<KeyEvent>,
    pub filter: Vec<KeyEvent>,
    pub copy: Vec<KeyEvent>,
    pub kill: Vec<KeyEvent>,
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
            quit: vec![
                KeyEvent::new(Char('q'), KeyModifiers::NONE),
                KeyEvent::new(Esc, KeyModifiers::NONE),
            ],
            detail: vec![KeyEvent::new(Tab, KeyModifiers::NONE)],
            filter: vec![KeyEvent::new(Char(':'), KeyModifiers::NONE)],
            copy: vec![KeyEvent::new(Enter, KeyModifiers::NONE)],
            kill: vec![KeyEvent::new(Char('x'), KeyModifiers::NONE)],
        }
    }
}

impl KeyBindings {
    fn matches(key: &KeyEvent, bindings: &[KeyEvent]) -> bool {
        bindings.contains(key)
    }

    pub fn is_up(&self, key: &KeyEvent) -> bool {
        Self::matches(key, &self.up)
    }

    pub fn is_down(&self, key: &KeyEvent) -> bool {
        Self::matches(key, &self.down)
    }

    pub fn is_quit(&self, key: &KeyEvent) -> bool {
        Self::matches(key, &self.quit)
    }

    pub fn is_detail(&self, key: &KeyEvent) -> bool {
        Self::matches(key, &self.detail)
    }

    pub fn is_filter(&self, key: &KeyEvent) -> bool {
        Self::matches(key, &self.filter)
    }

    pub fn is_copy(&self, key: &KeyEvent) -> bool {
        Self::matches(key, &self.copy)
    }

    pub fn is_kill(&self, key: &KeyEvent) -> bool {
        Self::matches(key, &self.kill)
    }
}
