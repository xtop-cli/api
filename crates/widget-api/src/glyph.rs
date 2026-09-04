//! Glyph styles shared by every widget pack, with the canonical mappings to
//! ratatui's drawing primitives.
//!
//! Packs MUST NOT re-implement these mappings — they import [`to_color`],
//! [`border_for`] and [`marker_for`] here, so the same configuration draws
//! identically in every pack (single source of truth, DR-2). The kernel only
//! stores the contract enums ([`ChartCharset`], [`WidgetBorders`]); turning
//! them into glyphs is this module's job.

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

/// Convert an `[r, g, b]` palette entry into a ratatui color.
///
/// Returns `Color::Rgb` verbatim; palette entries are already 24-bit RGB
/// triples, so no quantization is applied.
pub fn to_color(palette_entry: [u8; 3]) -> ratatui::style::Color {
    let [r, g, b] = palette_entry;
    ratatui::style::Color::Rgb(r, g, b)
}

/// Pure ASCII `+ - |` border frame.
///
/// ratatui's `symbols::border` module no longer ships an ASCII set (its
/// `PLAIN` is the single-line box-drawing frame), so the contract provides
/// the canonical one. Both `WidgetBorders::Plain` and `WidgetBorders::Ascii`
/// map here; packs must not hand-roll their own copy.
pub const ASCII_BORDER: ratatui::symbols::border::Set<'static> = ratatui::symbols::border::Set {
    top_left: "+",
    top_right: "+",
    bottom_left: "+",
    bottom_right: "+",
    vertical_left: "|",
    vertical_right: "|",
    horizontal_top: "-",
    horizontal_bottom: "-",
};

/// Border set for a [`WidgetBorders`] choice.
///
/// Mapping decisions (one border look per ratatui set, no pack-specific
/// divergence):
///
/// - `Native` → `border::PLAIN`, the standard single-line box-drawing frame
///   (`┌─┐│└┘`) and ratatui's own default border set — the classic look a
///   widget draws when it does not override anything.
/// - `Rounded` → `border::ROUNDED` (box drawing with rounded corners).
/// - `Double` → `border::DOUBLE` (double-line box drawing).
/// - `Plain` → [`ASCII_BORDER`] (pure ASCII `+ - |` frame).
/// - `Ascii` → [`ASCII_BORDER`] (same frame; `Ascii` is an explicit config
///   spelling of the same intent).
pub fn border_for(borders: WidgetBorders) -> ratatui::symbols::border::Set<'static> {
    match borders {
        WidgetBorders::Native => ratatui::symbols::border::PLAIN,
        WidgetBorders::Rounded => ratatui::symbols::border::ROUNDED,
        WidgetBorders::Double => ratatui::symbols::border::DOUBLE,
        WidgetBorders::Plain => ASCII_BORDER,
        WidgetBorders::Ascii => ASCII_BORDER,
    }
}

/// Chart marker for a [`ChartCharset`] choice.
///
/// The mapping mirrors the ratatui marker of the same name:
/// `Braille` → [`ratatui::symbols::Marker::Braille`],
/// `Dot` → [`ratatui::symbols::Marker::Dot`],
/// `Block` → [`ratatui::symbols::Marker::Block`],
/// `HalfBlock` → [`ratatui::symbols::Marker::HalfBlock`],
/// `Bar` → [`ratatui::symbols::Marker::Bar`]. Packs that need a different
/// glyph for the same config must not re-implement the table; they diverge
/// deliberately and document why.
pub fn marker_for(charset: ChartCharset) -> ratatui::symbols::Marker {
    match charset {
        ChartCharset::Braille => ratatui::symbols::Marker::Braille,
        ChartCharset::Dot => ratatui::symbols::Marker::Dot,
        ChartCharset::Block => ratatui::symbols::Marker::Block,
        ChartCharset::HalfBlock => ratatui::symbols::Marker::HalfBlock,
        ChartCharset::Bar => ratatui::symbols::Marker::Bar,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_color_preserves_the_rgb_triple() {
        assert_eq!(
            to_color([0x12, 0x34, 0xab]),
            ratatui::style::Color::Rgb(0x12, 0x34, 0xab)
        );
        assert_eq!(
            to_color([0x00, 0x80, 0xff]),
            ratatui::style::Color::Rgb(0, 128, 255)
        );
    }

    #[test]
    fn border_for_maps_every_variant_to_its_set() {
        use ratatui::symbols::border::{DOUBLE, PLAIN, ROUNDED};

        // `Native` is the standard box-drawing frame ratatui draws by
        // default (`PLAIN` since 0.29; `EMPTY` is the blank-space set).
        assert_eq!(border_for(WidgetBorders::Native), PLAIN);
        assert_eq!(border_for(WidgetBorders::Rounded), ROUNDED);
        assert_eq!(border_for(WidgetBorders::Double), DOUBLE);
        assert_eq!(border_for(WidgetBorders::Plain), ASCII_BORDER);
        assert_eq!(border_for(WidgetBorders::Ascii), ASCII_BORDER);
    }

    #[test]
    fn ascii_border_uses_plus_dash_pipe() {
        assert_eq!(border_for(WidgetBorders::Plain), ASCII_BORDER);
        assert_eq!(ASCII_BORDER.top_left, "+");
        assert_eq!(ASCII_BORDER.top_right, "+");
        assert_eq!(ASCII_BORDER.bottom_left, "+");
        assert_eq!(ASCII_BORDER.bottom_right, "+");
        assert_eq!(ASCII_BORDER.vertical_left, "|");
        assert_eq!(ASCII_BORDER.vertical_right, "|");
        assert_eq!(ASCII_BORDER.horizontal_top, "-");
        assert_eq!(ASCII_BORDER.horizontal_bottom, "-");
    }

    #[test]
    fn marker_for_maps_every_variant_to_its_marker() {
        use ratatui::symbols::Marker;

        assert_eq!(marker_for(ChartCharset::Braille), Marker::Braille);
        assert_eq!(marker_for(ChartCharset::Dot), Marker::Dot);
        assert_eq!(marker_for(ChartCharset::Block), Marker::Block);
        assert_eq!(marker_for(ChartCharset::HalfBlock), Marker::HalfBlock);
        assert_eq!(marker_for(ChartCharset::Bar), Marker::Bar);
    }

    #[test]
    fn enum_names_round_trip_as_snake_case() {
        let json = serde_json::to_string(&ChartCharset::HalfBlock).unwrap();
        assert_eq!(json, r#""half_block""#);
        assert_eq!(
            serde_json::from_str::<ChartCharset>(&json).unwrap(),
            ChartCharset::HalfBlock
        );

        let json = serde_json::to_string(&WidgetBorders::Plain).unwrap();
        assert_eq!(json, r#""plain""#);
        assert_eq!(
            serde_json::from_str::<WidgetBorders>(&json).unwrap(),
            WidgetBorders::Plain
        );
    }

    #[test]
    fn every_variant_round_trips() {
        for charset in [
            ChartCharset::Braille,
            ChartCharset::Dot,
            ChartCharset::Block,
            ChartCharset::HalfBlock,
            ChartCharset::Bar,
        ] {
            let json = serde_json::to_string(&charset).unwrap();
            assert_eq!(
                serde_json::from_str::<ChartCharset>(&json).unwrap(),
                charset
            );
        }
        for borders in [
            WidgetBorders::Native,
            WidgetBorders::Rounded,
            WidgetBorders::Double,
            WidgetBorders::Plain,
            WidgetBorders::Ascii,
        ] {
            let json = serde_json::to_string(&borders).unwrap();
            assert_eq!(
                serde_json::from_str::<WidgetBorders>(&json).unwrap(),
                borders
            );
        }
    }
}
