use std::fmt::Debug;

use crate::context::PluginContext;
use crate::provider::SystemDataProvider;
use crate::widget::PluginWidget;
use crate::{PluginError, PluginManifest};

/// The core trait that every plugin must implement.
///
/// All methods have default empty implementations so plugins only
/// override what they need.
pub trait Plugin: Debug + Send {
    /// Static metadata about this plugin.
    fn manifest(&self) -> PluginManifest;

    /// Called once when the plugin is loaded and enabled.
    fn on_enable(&mut self, _ctx: &mut PluginContext) -> Result<(), PluginError> {
        Ok(())
    }

    /// Called once when the plugin is disabled or xtop shuts down.
    fn on_disable(&mut self, _ctx: &mut PluginContext) -> Result<(), PluginError> {
        Ok(())
    }

    /// Called on every tick (every ~1s by default).
    fn on_tick(&mut self, _ctx: &mut PluginContext) -> Result<(), PluginError> {
        Ok(())
    }

    /// Called when a key is pressed.
    /// Return `Ok(true)` if the plugin consumed the key event.
    fn on_key(&mut self, _ctx: &mut PluginContext, _key: &str) -> Result<bool, PluginError> {
        Ok(false)
    }

    /// Optionally provide additional system data.
    /// The returned provider is merged into the main data stream via the
    /// kernel's composite provider.
    fn data_provider(&self) -> Option<Box<dyn SystemDataProvider>> {
        None
    }

    /// Optionally register a custom widget for TUI rendering.
    ///
    /// The widget renders against the plugin [`HostState`](crate::HostState)
    /// view; see [`PluginWidget`] (distinct from `xtop-widget-api`'s
    /// registration type, which draws over `WidgetState`).
    fn widget(&self) -> Option<PluginWidget> {
        None
    }

    /// Execute a named command with string parameters.
    /// Used by external agents (AI, CLI, IPC) to interact with the plugin.
    ///
    /// Returns a JSON-like string response.
    fn execute(
        &mut self,
        _ctx: &mut PluginContext,
        _action: &str,
        _params: &str,
    ) -> Result<String, PluginError> {
        Err(PluginError::UnknownAction(_action.to_string()))
    }
}
