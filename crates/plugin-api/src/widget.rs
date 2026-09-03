use std::fmt::Debug;

use crate::host::HostState;

type RenderFn = std::sync::Arc<
    dyn Fn(&mut ratatui::Frame, &dyn HostState, ratatui::prelude::Rect) + Send + Sync,
>;

/// A widget that a plugin registers for rendering in the TUI.
pub struct WidgetRegistration {
    pub name: String,
    pub render: RenderFn,
}

impl Debug for WidgetRegistration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WidgetRegistration")
            .field("name", &self.name)
            .finish()
    }
}
