//! Color helpers shared across the xtop ecosystem.

/// Parse a `#rrggbb` hex color into an `[r, g, b]` byte array.
///
/// Invalid or short inputs fall back to black per channel.
pub fn hex_to_rgb(hex: &str) -> [u8; 3] {
    let hex = hex.trim_start_matches('#');
    let r = u8::from_str_radix(hex.get(0..2).unwrap_or("00"), 16).unwrap_or(0);
    let g = u8::from_str_radix(hex.get(2..4).unwrap_or("00"), 16).unwrap_or(0);
    let b = u8::from_str_radix(hex.get(4..6).unwrap_or("00"), 16).unwrap_or(0);
    [r, g, b]
}
