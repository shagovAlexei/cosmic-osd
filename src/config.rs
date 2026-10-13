use cosmic_config::{Config, ConfigGet};

const AUDIO_CONFIG: &str = "com.system76.CosmicAudio";
const AMPLIFICATION_SINK: &str = "amplification_sink";
const AMPLIFICATION_SOURCE: &str = "amplification_source";

pub fn amplification_sink() -> bool {
    Config::new(AUDIO_CONFIG, 1)
        .ok()
        .and_then(|config| config.get::<bool>(AMPLIFICATION_SINK).ok())
        .unwrap_or(true)
}

pub fn amplification_source() -> bool {
    Config::new(AUDIO_CONFIG, 1)
        .ok()
        .and_then(|config| config.get::<bool>(AMPLIFICATION_SOURCE).ok())
        .unwrap_or(false)
}

pub const LAYOUT_CONFIG: &str = "com.system76.CosmicOsdLayout";
/// Languages with their own colors; keys are `text_color_<lang>` / `bg_color_<lang>`
/// holding "#RRGGBB" or "#RRGGBBAA" ("" = theme color).
pub const LANGS: [&str; 2] = ["ru", "en"];

#[derive(Clone, Copy, Debug, Default)]
pub struct LangColors {
    pub text: Option<[f32; 4]>,
    pub bg: Option<[f32; 4]>,
}

/// Look of the keyboard layout indicator, stored in
/// ~/.config/cosmic/com.system76.CosmicOsdLayout/v1/ and read on every show.
#[derive(Clone, Copy, Debug)]
pub struct LayoutStyle {
    pub font_size: u16,
    pub margin_right: i32,
    pub margin_bottom: i32,
    colors: [LangColors; 2],
}

impl LayoutStyle {
    pub fn colors(&self, name: &str) -> LangColors {
        LANGS
            .iter()
            .position(|l| l.eq_ignore_ascii_case(name))
            .map(|i| self.colors[i])
            .unwrap_or_default()
    }
}

pub fn parse_hex(s: &str) -> Option<[f32; 4]> {
    let h = s.trim().trim_start_matches('#');
    if !(h.len() == 6 || h.len() == 8) || !h.is_ascii() {
        return None;
    }
    let byte = |i: usize| u8::from_str_radix(&h[i..i + 2], 16).ok().map(|v| v as f32 / 255.);
    Some([byte(0)?, byte(2)?, byte(4)?, if h.len() == 8 { byte(6)? } else { 1. }])
}

pub fn layout_config() -> Option<Config> {
    Config::new(LAYOUT_CONFIG, 1).ok()
}

pub fn layout_style() -> LayoutStyle {
    let config = layout_config();
    let int = |key: &str, default| {
        config
            .as_ref()
            .and_then(|c| c.get::<i32>(key).ok())
            .unwrap_or(default)
    };
    let color = |key: String| {
        config
            .as_ref()
            .and_then(|c| c.get::<String>(&key).ok())
            .and_then(|s| parse_hex(&s))
    };
    LayoutStyle {
        font_size: int("font_size", 15).clamp(8, 200) as u16,
        margin_right: int("margin_right", 50),
        margin_bottom: int("margin_bottom", 50),
        colors: LANGS.map(|l| LangColors {
            text: color(format!("text_color_{l}")),
            bg: color(format!("bg_color_{l}")),
        }),
    }
}
