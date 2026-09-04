//! Glyph styles shared by every widget pack.
//!
//! A pack interprets these choices with its own symbol sets (ratatui
//! markers/borders, ascii sets, custom glyphs). The kernel never decides how
//! a charset is drawn: packs do.

use serde::{Deserialize, Serialize};

/// Character set used by chart widgets (history lines).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum ChartCharset {
    /// Braille dots (high detail, default).
    #[default]
    Braille,
    /// Braille dot markers.
    Dot,
    /// Solid blocks.
    Block,
    /// Half-height blocks.
    HalfBlock,
    /// Vertical bars.
    Bar,
}

/// Border look used by widget blocks.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum WidgetBorders {
    /// Keep the widget's native set (classic look).
    #[default]
    Native,
    Rounded,
    Double,
    Plain,
    /// Pure ASCII `+-|`.
    Ascii,
}
