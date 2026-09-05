//! `xtop-widget-api` — widget renderer contract for xtop.
//!
//! Widgets are the visual parts drawn inside layout areas. The kernel ships a
//! *base pack* of them, but any pack (community, plugin, custom) can provide
//! renderers for the same widget names or new ones.
//!
//! This crate defines:
//!
//! - [`state::WidgetState`] — the read-only view a renderer receives.
//! - [`glyph`] — shared glyph choices (chart charsets, border styles) and
//!   the canonical mappings to ratatui (`to_color`, `border_for`,
//!   `marker_for`) every pack imports.
//! - [`WidgetRegistration`] — how packs register renderers by widget name.
//!
//! It never depends on the kernel, so every repo in the ecosystem can consume
//! it standalone (the same rule as `xtop-plugin-api`).

pub mod glyph;

mod options;
mod renderer;
mod state;

pub use glyph::{ChartCharset, WidgetBorders};
pub use options::CpuBasis;
pub use renderer::{WidgetRegistration, WidgetRenderer};
pub use state::WidgetState;
