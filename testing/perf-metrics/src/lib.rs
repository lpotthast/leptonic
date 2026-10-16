//! The performance metrics of leptonic's browser tests: what a measurement
//! (`Page::measure` in `leptonic/tests`) counts while an action runs, defined once.
//!
//! The test-app counts the app's metrics ([`Source::App`]) and names them with [`Metric::name`];
//! the tests count the others, parse the app's by name and check every [`Metric::is_budgeted`]
//! metric against the budgets in `leptonic/tests/budgets/`. How a measurement works:
//! `documentation/testing.md`, "Performance measurements".

/// A metric of a measurement: how often something happened, or how much, while an action ran
/// (from the action until the page was idle again).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Metric {
    Allocations,
    Reallocations,
    AllocatedBytes,
    RetainedBytes,
    MemoRuns,
    MemoRunsUnchanged,
    EffectRuns,
    RenderEffectRuns,
    ImmediateEffectRuns,
    SignalNotifications,
    DomAttributes,
    DomText,
    DomAddedNodes,
    DomRemovedNodes,
    Layouts,
    StyleRecalcs,
    Frames,
}

/// Where a metric's values come from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Source {
    /// The app's wasm: the test-app's counting global allocator and `reactive_graph::metrics`.
    App,
    /// A `MutationObserver` on the document.
    Dom,
    /// Chrome (CDP's `Performance.getMetrics`).
    Browser,
    /// The measurement itself.
    Measurement,
}

/// What a metric's values are.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Unit {
    Count,
    Bytes,
    Frames,
}

impl Metric {
    /// Every metric, in the order reports list them.
    pub const ALL: [Self; 17] = [
        Self::Allocations,
        Self::Reallocations,
        Self::AllocatedBytes,
        Self::RetainedBytes,
        Self::MemoRuns,
        Self::MemoRunsUnchanged,
        Self::EffectRuns,
        Self::RenderEffectRuns,
        Self::ImmediateEffectRuns,
        Self::SignalNotifications,
        Self::DomAttributes,
        Self::DomText,
        Self::DomAddedNodes,
        Self::DomRemovedNodes,
        Self::Layouts,
        Self::StyleRecalcs,
        Self::Frames,
    ];

    /// The metric's name in reports, budgets and the test-app's counters.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Allocations => "allocations",
            Self::Reallocations => "reallocations",
            Self::AllocatedBytes => "allocated_bytes",
            Self::RetainedBytes => "retained_bytes",
            Self::MemoRuns => "memo_runs",
            Self::MemoRunsUnchanged => "memo_runs_unchanged",
            Self::EffectRuns => "effect_runs",
            Self::RenderEffectRuns => "render_effect_runs",
            Self::ImmediateEffectRuns => "immediate_effect_runs",
            Self::SignalNotifications => "signal_notifications",
            Self::DomAttributes => "dom_attributes",
            Self::DomText => "dom_text",
            Self::DomAddedNodes => "dom_added_nodes",
            Self::DomRemovedNodes => "dom_removed_nodes",
            Self::Layouts => "layouts",
            Self::StyleRecalcs => "style_recalcs",
            Self::Frames => "frames",
        }
    }

    /// The metric named `name`.
    #[must_use]
    pub fn from_name(name: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|metric| metric.name() == name)
    }

    /// What the metric counts, and what a high value hints at.
    #[must_use]
    pub const fn description(self) -> &'static str {
        match self {
            Self::Allocations => {
                "Heap allocations of the wasm (`alloc`, `alloc_zeroed`). Every `Box`, `Vec`, \
                 `String`, `Rc`/`Arc`, closure and signal allocates; allocations per action are \
                 the first thing to bring down."
            }
            Self::Reallocations => {
                "Reallocations (`realloc`): a `Vec` or `String` grew past its capacity. Many hint \
                 at collections built element by element without `with_capacity`."
            }
            Self::AllocatedBytes => {
                "Bytes requested by allocations, plus what reallocations grew by: the memory \
                 traffic of the action. Not budgeted: the size of an allocation can differ by a \
                 few bytes between runs while the counts stay equal (seen when closing overlays, \
                 reopening a popover, changing collections: 16-48 bytes), probably text whose \
                 length depends on what the page did before (generated ids crossing a digit). \
                 `allocations` is budgeted."
            }
            Self::RetainedBytes => {
                "Bytes allocated and not freed when the measurement ended (negative: more freed \
                 than allocated): memory the action kept. Not budgeted: some memory is freed at \
                 times nothing in the page decides (after the browser's garbage collection, \
                 which frees what the wasm's JS objects held). Leaks show in leak checks."
            }
            Self::MemoRuns => {
                "Memo computations (`Memo`, the first and every recomputation). More than the \
                 values that depend on what changed hints at memos reading too much."
            }
            Self::MemoRunsUnchanged => {
                "Memo computations whose value equaled the previous one, so that the memo's \
                 subscribers weren't notified: work a finer-grained source would have saved."
            }
            Self::EffectRuns => {
                "Runs of `Effect`s (their first run included, and watches' dependency functions)."
            }
            Self::RenderEffectRuns => {
                "Runs of `RenderEffect`s (their first run included): the renderer's reactive \
                 attributes, properties, classes, styles, text and children. One per DOM value \
                 that may have changed; more hint at view closures reading more than they show."
            }
            Self::ImmediateEffectRuns => "Runs of `ImmediateEffect`s (their first run included).",
            Self::SignalNotifications => {
                "Notifications of signals and triggers (writes, `notify`): each marks the \
                 signal's subscribers dirty. Writes of unchanged values belong behind \
                 `set_if_changed`-style checks."
            }
            Self::DomAttributes => {
                "Attribute mutation records: every `setAttribute`/`removeAttribute`, also of an \
                 unchanged value. Writes beyond the attributes whose value changed are wasted."
            }
            Self::DomText => "Text node mutation records (`characterData`).",
            Self::DomAddedNodes => {
                "Nodes added by child list mutations (a subtree inserted at once counts once)."
            }
            Self::DomRemovedNodes => {
                "Nodes removed by child list mutations (a subtree removed at once counts once)."
            }
            Self::Layouts => {
                "Layouts of the page (Chrome's `LayoutCount`). More than the frames the action \
                 took hints at forced layouts: geometry read after a DOM write."
            }
            Self::StyleRecalcs => "Style recalculations of the page (Chrome's `RecalcStyleCount`).",
            Self::Frames => {
                "Frames until the page was idle again (a frame without DOM mutations, \
                 allocations and reactive work). Not budgeted: on a busy machine a late frame \
                 takes in work that otherwise spans two."
            }
        }
    }

    /// Where the metric's values come from.
    #[must_use]
    pub const fn source(self) -> Source {
        match self {
            Self::Allocations
            | Self::Reallocations
            | Self::AllocatedBytes
            | Self::RetainedBytes
            | Self::MemoRuns
            | Self::MemoRunsUnchanged
            | Self::EffectRuns
            | Self::RenderEffectRuns
            | Self::ImmediateEffectRuns
            | Self::SignalNotifications => Source::App,
            Self::DomAttributes | Self::DomText | Self::DomAddedNodes | Self::DomRemovedNodes => {
                Source::Dom
            }
            Self::Layouts | Self::StyleRecalcs => Source::Browser,
            Self::Frames => Source::Measurement,
        }
    }

    /// What the metric's values are.
    #[must_use]
    pub const fn unit(self) -> Unit {
        match self {
            Self::AllocatedBytes | Self::RetainedBytes => Unit::Bytes,
            Self::Frames => Unit::Frames,
            _ => Unit::Count,
        }
    }

    /// Whether the metric has budgets: its values repeat exactly from run to run, also on a busy
    /// machine (see the descriptions of those that don't).
    #[must_use]
    pub const fn is_budgeted(self) -> bool {
        !matches!(
            self,
            Self::AllocatedBytes | Self::RetainedBytes | Self::Frames
        )
    }
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::*;

    #[test]
    fn every_metric_is_found_by_its_name() {
        for metric in Metric::ALL {
            assert_that!(Metric::from_name(metric.name())).is_equal_to(Some(metric));
        }
        assert_that!(Metric::from_name("unknown")).is_none();
    }
}
