use ratatui::style::Color;

/// Semantic color slots used across the TUI. Each preset (Kanagawa, Tokyo
/// Night, Nord) is a `pub const Theme` built from `Color::Rgb` so palette
/// tweaks stay local to this file.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Theme {
    pub fg: Color,
    pub muted: Color,
    pub border: Color,
    pub border_active: Color,
    pub selection: Color,
    pub accent: Color,
    pub success: Color,
    pub danger: Color,
    pub pid: Color,
    pub port: Color,
    pub command: Color,
    pub header_label: Color,
    pub modal_bg: Color,
}

impl Theme {
    /// Resolve a preset by case-insensitive name. Returns `None` for unknown
    /// names so the caller can fail explicitly.
    pub fn from_name(name: &str) -> Option<Theme> {
        match name.to_ascii_lowercase().as_str() {
            "kanagawa" => Some(KANAGAWA),
            "tokyonight" | "tokyo-night" | "tokyo_night" => Some(TOKYO_NIGHT),
            "nord" => Some(NORD),
            _ => None,
        }
    }
}

pub const KANAGAWA: Theme = Theme {
    fg: Color::Rgb(0xdc, 0xd7, 0xba),
    muted: Color::Rgb(0x71, 0x7c, 0x7c),
    border: Color::Rgb(0x36, 0x36, 0x46),
    border_active: Color::Rgb(0x7e, 0x9c, 0xd8),
    selection: Color::Rgb(0x2d, 0x4f, 0x67),
    accent: Color::Rgb(0x7e, 0x9c, 0xd8),
    success: Color::Rgb(0x76, 0x94, 0x6a),
    danger: Color::Rgb(0xc3, 0x40, 0x43),
    pid: Color::Rgb(0x76, 0x94, 0x6a),
    port: Color::Rgb(0xc0, 0xa3, 0x6e),
    command: Color::Rgb(0x6a, 0x95, 0x89),
    header_label: Color::Rgb(0xc0, 0xa3, 0x6e),
    modal_bg: Color::Rgb(0x1f, 0x1f, 0x28),
};

pub const TOKYO_NIGHT: Theme = Theme {
    fg: Color::Rgb(0xc0, 0xca, 0xf5),
    muted: Color::Rgb(0x56, 0x5f, 0x89),
    border: Color::Rgb(0x29, 0x2e, 0x42),
    border_active: Color::Rgb(0x7a, 0xa2, 0xf7),
    selection: Color::Rgb(0x28, 0x34, 0x57),
    accent: Color::Rgb(0x7a, 0xa2, 0xf7),
    success: Color::Rgb(0x9e, 0xce, 0x6a),
    danger: Color::Rgb(0xf7, 0x76, 0x8e),
    pid: Color::Rgb(0x9e, 0xce, 0x6a),
    port: Color::Rgb(0xe0, 0xaf, 0x68),
    command: Color::Rgb(0x7d, 0xcf, 0xff),
    header_label: Color::Rgb(0xe0, 0xaf, 0x68),
    modal_bg: Color::Rgb(0x1a, 0x1b, 0x26),
};

pub const NORD: Theme = Theme {
    fg: Color::Rgb(0xd8, 0xde, 0xe9),
    muted: Color::Rgb(0x4c, 0x56, 0x6a),
    border: Color::Rgb(0x3b, 0x42, 0x52),
    border_active: Color::Rgb(0x88, 0xc0, 0xd0),
    selection: Color::Rgb(0x43, 0x4c, 0x5e),
    accent: Color::Rgb(0x88, 0xc0, 0xd0),
    success: Color::Rgb(0xa3, 0xbe, 0x8c),
    danger: Color::Rgb(0xbf, 0x61, 0x6a),
    pid: Color::Rgb(0xa3, 0xbe, 0x8c),
    port: Color::Rgb(0xeb, 0xcb, 0x8b),
    command: Color::Rgb(0x81, 0xa1, 0xc1),
    header_label: Color::Rgb(0xeb, 0xcb, 0x8b),
    modal_bg: Color::Rgb(0x2e, 0x34, 0x40),
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_name_resolves_known_presets() {
        assert_eq!(Theme::from_name("kanagawa"), Some(KANAGAWA));
        assert_eq!(Theme::from_name("tokyonight"), Some(TOKYO_NIGHT));
        assert_eq!(Theme::from_name("nord"), Some(NORD));
    }

    #[test]
    fn from_name_is_case_insensitive() {
        assert_eq!(Theme::from_name("Kanagawa"), Some(KANAGAWA));
        assert_eq!(Theme::from_name("NORD"), Some(NORD));
        assert_eq!(Theme::from_name("TokyoNight"), Some(TOKYO_NIGHT));
    }

    #[test]
    fn from_name_accepts_tokyo_night_variants() {
        assert_eq!(Theme::from_name("tokyo-night"), Some(TOKYO_NIGHT));
        assert_eq!(Theme::from_name("tokyo_night"), Some(TOKYO_NIGHT));
    }

    #[test]
    fn from_name_returns_none_for_unknown() {
        assert_eq!(Theme::from_name("dracula"), None);
        assert_eq!(Theme::from_name(""), None);
    }
}
