//! Renderer registration types for widget packs.
//!
//! A pack registers one renderer per widget *name* (the names layouts use).
//! The engine resolves `(pack, name)` at render time; plugin widgets are just
//! another pack with higher precedence.

use std::fmt::Debug;
use std::sync::Arc;

use ratatui::prelude::Rect;
use ratatui::Frame;

use crate::state::WidgetState;

/// How a widget draws itself.
pub type WidgetRenderer = Arc<dyn Fn(&mut Frame, &dyn WidgetState, Rect) + Send + Sync>;

/// One renderer registration under a widget name.
pub struct WidgetRegistration {
    pub name: String,
    pub render: WidgetRenderer,
}

impl Debug for WidgetRegistration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WidgetRegistration")
            .field("name", &self.name)
            .finish()
    }
}
