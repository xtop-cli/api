//! Read-only widget state contract.
//!
//! Every renderer (built-in pack, community pack or plugin) draws against
//! [`WidgetState`] — never against kernel types. The kernel implements this
//! trait over its live state and hands it to the engine.

use std::collections::VecDeque;

use xtop_plugin_api::model::{ProcessInfo, SystemInfo, SystemSnapshot};
use xtop_plugin_api::AlertThresholds;

use crate::glyph::{ChartCharset, WidgetBorders};

/// The minimum a widget renderer needs to know about the running app, sampled
/// per tick. Read-only by design: widgets never mutate app state.
pub trait WidgetState {
    /// The current system sample (one per tick). `None` before the first tick.
    fn snapshot(&self) -> Option<&SystemSnapshot>;

    // -- theme ---------------------------------------------------------------

    fn theme_name(&self) -> &str;
    fn theme_fg(&self) -> &[u8; 3];
    fn theme_bg(&self) -> &[u8; 3];
    fn theme_palette(&self) -> &[[u8; 3]; 16];
    fn alerts(&self) -> AlertThresholds;

    // -- glyph style (resolved: global, honouring per-widget overrides) ------

    fn charset(&self, widget: &str) -> ChartCharset;
    fn borders(&self, widget: &str) -> WidgetBorders;

    // -- history (for chart widgets) ------------------------------------------

    fn cpu_history(&self) -> &[VecDeque<(f64, f64)>];
    fn mem_history(&self) -> &VecDeque<(f64, f64)>;
    fn net_rx_history(&self) -> &VecDeque<(f64, f64)>;
    fn net_tx_history(&self) -> &VecDeque<(f64, f64)>;

    // -- view/control state ---------------------------------------------------

    fn search_query(&self) -> &str;
    fn process_selected_pid(&self) -> Option<u32>;
    fn process_sort_label(&self) -> &str;
    fn layout_name(&self) -> &str;
    fn is_searching(&self) -> bool;
    fn fullscreen_label(&self) -> Option<&str>;
    fn sys_info(&self) -> SystemInfo;

    /// The process rows the processes widget draws: the shared per-tick
    /// sample filtered by the active search query and sorted by the user's
    /// chosen column. Selection is anchored by PID, so highlight and the
    /// kill action always agree on the same row.
    fn process_view(&self) -> Vec<&ProcessInfo>;
}
