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

    /// Summed disk **read** rate history (bytes/s, aggregate across disks).
    ///
    /// `(x, y)` pairs in the same shape as [`Self::net_rx_history`]; the
    /// default is an empty history for renderers/implementors that do not
    /// track it yet.
    fn disk_read_history(&self) -> &VecDeque<(f64, f64)> {
        static EMPTY: std::sync::LazyLock<VecDeque<(f64, f64)>> =
            std::sync::LazyLock::new(VecDeque::new);
        &EMPTY
    }

    /// Summed disk **write** rate history (bytes/s, aggregate across disks).
    ///
    /// `(x, y)` pairs in the same shape as [`Self::net_tx_history`]; the
    /// default is an empty history for renderers/implementors that do not
    /// track it yet.
    fn disk_write_history(&self) -> &VecDeque<(f64, f64)> {
        static EMPTY: std::sync::LazyLock<VecDeque<(f64, f64)>> =
            std::sync::LazyLock::new(VecDeque::new);
        &EMPTY
    }

    /// 1-minute load-average history.
    ///
    /// `(x, y)` pairs in the same shape as [`Self::mem_history`]; y is the
    /// 1-minute load average (`SystemSnapshot::load_avg.one`). The default
    /// is an empty history for renderers/implementors that do not track it
    /// yet.
    fn load_history(&self) -> &VecDeque<(f64, f64)> {
        static EMPTY: std::sync::LazyLock<VecDeque<(f64, f64)>> =
            std::sync::LazyLock::new(VecDeque::new);
        &EMPTY
    }

    // -- view/control state ---------------------------------------------------

    fn search_query(&self) -> &str;
    fn process_selected_pid(&self) -> Option<u32>;
    fn process_sort_label(&self) -> &str;

    /// Whether the active process sorting is descending.
    ///
    /// The processes widget draws a direction marker on the sorted column
    /// header (`▼` descending, `▲` ascending) from this flag, mirroring the
    /// `cycle_sort` key semantics of the kernel (each sort key press toggles
    /// the direction of the current column before advancing to the next
    /// column, which starts descending). The default `false` (ascending)
    /// reproduces the pre-direction behavior for implementors that do not
    /// track a direction; the kernel returns its live flag.
    fn process_sort_desc(&self) -> bool {
        false
    }

    fn layout_name(&self) -> &str;
    fn is_searching(&self) -> bool;
    fn fullscreen_label(&self) -> Option<&str>;
    fn sys_info(&self) -> SystemInfo;

    /// The process rows the processes widget draws: the shared per-tick
    /// sample filtered by the active search query and sorted by the user's
    /// chosen column. Selection is anchored by PID, so highlight and the
    /// kill action always agree on the same row.
    fn process_view(&self) -> Vec<&ProcessInfo>;

    // -- process mapping helpers (UX9.1) -------------------------------------
    //
    // Two additive process-row helpers: the login name for a numeric uid and
    // the recent per-process CPU samples for a small braille spark. Both have
    // defaults so implementors that do not track the data keep compiling
    // unchanged and renderers degrade to the numeric/empty fallbacks.

    /// The login name for a numeric user id, when the kernel can resolve it.
    ///
    /// `ProcessInfo::user_id` carries the numeric uid as a string; renderers
    /// map it to a human name through this method and MUST fall back to the
    /// numeric uid when `None` is returned (uid→name is a display mapping,
    /// deliberately not part of the data model). The kernel resolves names
    /// from `/etc/passwd` on unix platforms; on platforms without that file
    /// the map is empty and every uid yields `None`. The default `None`
    /// reproduces the numeric fallback for implementors that do not resolve
    /// names.
    fn uid_to_name(&self, _uid: u32) -> Option<String> {
        None
    }

    /// Recent CPU-usage samples (percent of one logical core) for one
    /// process, oldest → newest.
    ///
    /// The kernel feeds one sample per tick per process from the visible
    /// (sorted, capped) process list and bounds the storage: at most
    /// `XTOP_MAX_PROCESSES` pids plus a margin are tracked, ~30 samples per
    /// pid, and the oldest pid is evicted first when the pid cap is reached,
    /// so memory stays flat. Returns an owned, empty `Vec` for unknown or
    /// untracked pids (a freshly seen pid needs ~1 tick before its first
    /// sample lands) — renderers draw nothing for an empty series. The
    /// default is an empty series for implementors that do not track it.
    fn process_cpu_history(&self, _pid: u32) -> Vec<f64> {
        Vec::new()
    }

    // -- layout-driven display options (DR-UX1) ------------------------------
    //
    // Display discrimination is configured per widget instance in the layout
    // file (an `options` JSON object on the widget node, passthrough through
    // the `xtop-layout` model). The kernel feeds those options back to
    // renderers through the two methods below instead of changing renderer
    // signatures, so pack/plugin registration stays name-keyed and stable.
    // Both default to today's behavior: renderers that do not ask never see
    // a difference when no options are present.

    /// Number of logical processors the host reports.
    ///
    /// Used by renderers to normalize a per-process (or per-core) CPU usage
    /// — a fraction of one logical core — into a percentage of the whole
    /// machine's CPU (`CpuBasis::Total` display). The default `1` reproduces
    /// the pre-DR-UX1 behavior for renderers that ignore the method; the
    /// kernel implements it from the host's actual processor count
    /// (`available_parallelism`).
    fn logical_core_count(&self) -> usize {
        1
    }

    /// The `options` object of the widget currently being rendered.
    ///
    /// Per DR-UX1 the kernel sets this around each widget render call from
    /// the layout node that named the widget; `None` when the node carries no
    /// `options` object (or the widget is rendered outside a layout node,
    /// e.g. fullscreen without a matching node). Renderers MUST treat `None`
    /// as "default behavior" and may refine their output only from the keys
    /// they document; unknown keys are ignored. The value is never `null`.
    ///
    /// Read-only: renderers must never mutate state or cache a returned
    /// reference past the render call — it is only valid while the widget is
    /// being rendered.
    fn widget_options(&self) -> Option<&serde_json::Value> {
        None
    }
}
