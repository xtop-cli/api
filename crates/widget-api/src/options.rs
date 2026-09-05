//! Typed helpers for interpreting per-widget layout options (DR-UX1).
//!
//! Widget nodes in a layout may carry an `options` JSON object (passthrough:
//! the `xtop-layout` model keeps it verbatim, renderers own the
//! interpretation). The kernel exposes the active widget's options through
//! [`crate::WidgetState::widget_options`]; this module turns option *values*
//! into the typed, documented choices renderers switch on.

use serde::{Deserialize, Serialize};

/// How a CPU percentage is expressed by a widget.
///
/// Laid out per DR-UX1/UX3: the `processes` (and later `cpu`) widget decides
/// per instance, from the layout `options`, whether its CPU column/gauges
/// show the classic per-core fraction or the machine-share normalized value.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CpuBasis {
    /// Fraction of one logical core: 0..100 per core (top/htop convention,
    /// the current xtop behavior, and the default when no option is given).
    Core,
    /// Fraction of the whole machine's CPU: a per-process value divided by
    /// [`crate::WidgetState::logical_core_count`], shown with decimals when
    /// small (machine-share normalized display).
    Total,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn serde_roundtrip_core() {
        let value = CpuBasis::Core;
        let json = serde_json::to_string(&value).unwrap();
        assert_eq!(json, "\"core\"");
        let back: CpuBasis = serde_json::from_str(&json).unwrap();
        assert_eq!(value, back);
    }

    #[test]
    fn serde_roundtrip_total() {
        let value = CpuBasis::Total;
        let json = serde_json::to_string(&value).unwrap();
        assert_eq!(json, "\"total\"");
        let back: CpuBasis = serde_json::from_str(&json).unwrap();
        assert_eq!(value, back);
    }

    #[test]
    fn serde_from_option_value() {
        // The layout `options` arrive as a JSON value; parsing an option key
        // yields the enum or an error the renderer reports/ignores.
        let core: CpuBasis = serde_json::from_value(json!("core")).unwrap();
        assert_eq!(core, CpuBasis::Core);
        let total: CpuBasis = serde_json::from_value(json!("total")).unwrap();
        assert_eq!(total, CpuBasis::Total);
        assert!(serde_json::from_value::<CpuBasis>(json!("per-widget")).is_err());
    }
}
