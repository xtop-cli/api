//! `xtop-effect-api` — shared effect contract for xtop.
//!
//! Effects are stateful, full-buffer visual transitions that the kernel
//! applies to every rendered frame. This crate defines the two pieces of
//! that contract:
//!
//! - [`Effect`] — an effect implementation.
//! - [`EffectManifest`] — static metadata about an effect.
//!
//! # Host contract
//!
//! The kernel (`xtop`) is the host and drives effects like this:
//!
//! - [`Effect::on_frame`] is called on **every rendered frame**, after
//!   widgets and layout have been drawn into the ratatui buffer and **before**
//!   the terminal flush.
//! - `elapsed` is the time since the effect started, not the frame delta.
//! - Effects are **stateful across frames**: they keep whatever progress
//!   state they need on `self` between calls.
//! - The host does not pace frames: it ticks at ~1s cadence plus ad-hoc
//!   repaints. Visual transitions must therefore advance by wall-clock time
//!   (`elapsed`), never by frame count alone.
//!
//! Effects never depend on the kernel; they only use this crate and
//! ratatui's buffer type.

use std::time::Duration;

use ratatui::buffer::Buffer;

/// Static metadata about an effect.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EffectManifest {
    pub id: &'static str,
    pub name: &'static str,
    pub description: &'static str,
}

/// A stateful, full-buffer visual effect.
///
/// The host calls [`on_frame`](Effect::on_frame) once per rendered frame;
/// implementations keep all progress on `self` and must derive their visuals
/// from `elapsed` (see the [crate docs](crate) for the host contract).
pub trait Effect: Send + Sync {
    /// Static metadata about this effect.
    fn manifest(&self) -> EffectManifest;

    /// Transform the next rendered frame.
    ///
    /// `buffer` holds the frame after widgets/layout were drawn and before
    /// the terminal flush; the effect may rewrite any of its cells.
    /// `elapsed` is the time since the effect started.
    fn on_frame(&mut self, buffer: &mut Buffer, elapsed: Duration);
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use ratatui::buffer::Buffer;
    use ratatui::layout::Rect;

    use super::{Effect, EffectManifest};

    const TEST_MANIFEST: EffectManifest = EffectManifest {
        id: "test-fade",
        name: "Test fade",
        description: "fake effect for tests",
    };

    /// Fake effect that records how often and for how long it has run.
    #[derive(Debug, Default)]
    struct FrameCounter {
        frames: u64,
        total_elapsed: Duration,
    }

    impl Effect for FrameCounter {
        fn manifest(&self) -> EffectManifest {
            TEST_MANIFEST
        }

        fn on_frame(&mut self, _buffer: &mut Buffer, elapsed: Duration) {
            self.frames += 1;
            self.total_elapsed += elapsed;
        }
    }

    #[test]
    fn on_frame_advances_state_per_frame() {
        let mut effect = FrameCounter::default();
        let mut buffer = Buffer::empty(Rect::new(0, 0, 5, 3));

        effect.on_frame(&mut buffer, Duration::from_millis(16));
        effect.on_frame(&mut buffer, Duration::from_millis(34));
        effect.on_frame(&mut buffer, Duration::from_millis(50));

        assert_eq!(effect.frames, 3);
        assert_eq!(effect.total_elapsed, Duration::from_millis(100));
    }

    #[test]
    fn manifests_are_plain_data_and_compare_equal() {
        let a = FrameCounter::default().manifest();
        let b = FrameCounter::default().manifest();
        assert_eq!(a, b);

        let other = EffectManifest {
            id: "test-wipe",
            name: "Test wipe",
            description: "different effect",
        };
        assert_ne!(a, other);
        assert_eq!(a.id, "test-fade");
    }
}
