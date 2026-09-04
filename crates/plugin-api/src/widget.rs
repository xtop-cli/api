use std::fmt::Debug;

use crate::host::HostState;

type RenderFn = std::sync::Arc<
    dyn Fn(&mut ratatui::Frame, &dyn HostState, ratatui::prelude::Rect) + Send + Sync,
>;

/// A widget that a plugin registers for rendering in the TUI.
///
/// This is the plugin view over [`HostState`]: its render closure draws
/// against the plugin's host view. `xtop-widget-api` owns the canonical
/// widget-pack registration type, the one drawn over `WidgetState`; the two
/// are distinct contracts and must not share a name, so this type is named
/// for what it is — a plugin widget.
pub struct PluginWidget {
    pub name: String,
    pub render: RenderFn,
}

impl Debug for PluginWidget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PluginWidget")
            .field("name", &self.name)
            .finish()
    }
}
