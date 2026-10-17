//! What the app's code did, counted: the wasm's heap allocations (the app's global allocator
//! counts them) and the reactive graph's work (`reactive_graph::metrics`: memo computations,
//! effect runs, signal notifications). The difference of two [`Counters`] is the work done in
//! between: the app's metrics ([`Metric`], [`leptonic_perf_metrics::Source::App`]), which the
//! browser tests read through `window.leptonicCounters` ([`register`]).

use std::{
    alloc::{GlobalAlloc, Layout, System},
    cell::Cell,
    sync::atomic::{AtomicI64, AtomicU64, Ordering},
};

use leptonic_perf_metrics::Metric;
use leptos::prelude::window;
use reactive_graph::metrics::{self, Metrics};
use serde_json::{Map, Value};
use wasm_bindgen::{JsValue, closure::Closure};

/// The wasm's allocator: the default one (`System`, dlmalloc), counting.
#[cfg(target_arch = "wasm32")]
#[global_allocator]
static ALLOCATOR: Counting = Counting;

static ALLOCATIONS: AtomicU64 = AtomicU64::new(0);
static REALLOCATIONS: AtomicU64 = AtomicU64::new(0);
static ALLOCATED_BYTES: AtomicU64 = AtomicU64::new(0);
/// The largest allocation since the measurement started (`leptonicCounters.start()`).
static LARGEST_ALLOCATION: AtomicU64 = AtomicU64::new(0);
static LIVE_BYTES: AtomicI64 = AtomicI64::new(0);

/// The system allocator, counting allocations, reallocations and bytes.
pub struct Counting;

fn bytes(size: usize) -> i64 {
    i64::try_from(size).unwrap_or(i64::MAX)
}

// SAFETY: every method forwards to `System` with the same arguments; counting touches no memory
// of the allocations.
unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // SAFETY: the caller's guarantees for `alloc` hold for `System.alloc`.
        let pointer = unsafe { System.alloc(layout) };
        if !pointer.is_null() {
            counted_allocation(layout.size());
        }
        pointer
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        // SAFETY: the caller's guarantees for `alloc_zeroed` hold for `System.alloc_zeroed`.
        let pointer = unsafe { System.alloc_zeroed(layout) };
        if !pointer.is_null() {
            counted_allocation(layout.size());
        }
        pointer
    }

    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        // SAFETY: the caller's guarantees for `dealloc` hold for `System.dealloc` (the memory came
        // from `System`).
        unsafe { System.dealloc(pointer, layout) };
        LIVE_BYTES.fetch_sub(bytes(layout.size()), Ordering::Relaxed);
    }

    unsafe fn realloc(&self, pointer: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        // SAFETY: the caller's guarantees for `realloc` hold for `System.realloc` (the memory came
        // from `System`).
        let new_pointer = unsafe { System.realloc(pointer, layout, new_size) };
        if !new_pointer.is_null() {
            REALLOCATIONS.fetch_add(1, Ordering::Relaxed);
            LARGEST_ALLOCATION.fetch_max(new_size as u64, Ordering::Relaxed);
            ALLOCATED_BYTES.fetch_add(
                new_size.saturating_sub(layout.size()) as u64,
                Ordering::Relaxed,
            );
            LIVE_BYTES.fetch_add(bytes(new_size) - bytes(layout.size()), Ordering::Relaxed);
        }
        new_pointer
    }
}

fn counted_allocation(size: usize) {
    ALLOCATIONS.fetch_add(1, Ordering::Relaxed);
    ALLOCATED_BYTES.fetch_add(size as u64, Ordering::Relaxed);
    LARGEST_ALLOCATION.fetch_max(size as u64, Ordering::Relaxed);
    LIVE_BYTES.fetch_add(bytes(size), Ordering::Relaxed);
}

/// The counters' values at one moment.
#[derive(Debug, Clone, Copy)]
pub struct Counters {
    /// Heap allocations (`alloc`, `alloc_zeroed`).
    allocations: u64,
    /// Reallocations (`realloc`: a growing `Vec` or `String`).
    reallocations: u64,
    /// Bytes requested by allocations, plus what reallocations grew by.
    allocated_bytes: u64,
    /// The largest allocation since the measurement started.
    largest_allocation: u64,
    /// Bytes allocated and not freed.
    live_bytes: i64,
    reactive: Metrics,
}

impl Counters {
    /// The counters' current values. Allocates nothing.
    pub fn now() -> Self {
        Self {
            allocations: ALLOCATIONS.load(Ordering::Relaxed),
            reallocations: REALLOCATIONS.load(Ordering::Relaxed),
            allocated_bytes: ALLOCATED_BYTES.load(Ordering::Relaxed),
            largest_allocation: LARGEST_ALLOCATION.load(Ordering::Relaxed),
            live_bytes: LIVE_BYTES.load(Ordering::Relaxed),
            reactive: metrics::snapshot(),
        }
    }

    /// A number that grows whenever the app's code works (allocates or runs reactive code), to
    /// tell when the page became idle. Allocates nothing.
    #[allow(clippy::cast_precision_loss)]
    pub fn activity(self) -> f64 {
        (self.allocations + self.reallocations + self.reactive_activity()) as f64
    }

    /// A number that grows whenever the reactive graph works (memo computations, effect runs,
    /// signal notifications).
    pub fn reactive_activity(self) -> u64 {
        let Metrics {
            memo_runs,
            effect_runs,
            render_effect_runs,
            immediate_effect_runs,
            signal_notifications,
            ..
        } = self.reactive;
        memo_runs + effect_runs + render_effect_runs + immediate_effect_runs + signal_notifications
    }

    /// The value of the app's `metric` from `start` to `self` (`None`: the metric isn't the
    /// app's, see [`Metric::source`]).
    fn since(self, start: Self, metric: Metric) -> Option<i64> {
        let reactive = self.reactive.since(start.reactive);
        let count = |value: u64| Some(i64::try_from(value).unwrap_or(i64::MAX));
        match metric {
            Metric::Allocations => count(self.allocations - start.allocations),
            Metric::Reallocations => count(self.reallocations - start.reallocations),
            Metric::AllocatedBytes => count(self.allocated_bytes - start.allocated_bytes),
            Metric::LargestAllocation => count(self.largest_allocation),
            Metric::RetainedBytes => Some(self.live_bytes - start.live_bytes),
            Metric::MemoRuns => count(reactive.memo_runs),
            Metric::MemoRunsUnchanged => count(reactive.memo_runs_unchanged),
            Metric::EffectRuns => count(reactive.effect_runs),
            Metric::RenderEffectRuns => count(reactive.render_effect_runs),
            Metric::ImmediateEffectRuns => count(reactive.immediate_effect_runs),
            Metric::SignalNotifications => count(reactive.signal_notifications),
            Metric::DomAttributes
            | Metric::DomText
            | Metric::DomAddedNodes
            | Metric::DomRemovedNodes
            | Metric::Layouts
            | Metric::StyleRecalcs
            | Metric::Frames => None,
        }
    }

    /// The app's metrics from `start` to `self`, as JSON: `{<metric name>: value}`.
    fn since_json(self, start: Self) -> String {
        let values: Map<String, Value> = Metric::ALL
            .into_iter()
            .filter_map(|metric| {
                Some((metric.name().to_owned(), self.since(start, metric)?.into()))
            })
            .collect();
        Value::Object(values).to_string()
    }
}

thread_local! {
    /// The counters when the current measurement started (`leptonicCounters.start()`).
    static START: Cell<Option<Counters>> = const { Cell::new(None) };
}

/// Registers `window.leptonicCounters` for the browser tests' measurements (`Page::measure` in
/// `leptonic/tests`), for as long as the page lives:
/// - `start()`: a measurement starts; remembers the counters (and resets the largest
///   allocation). Allocates nothing.
/// - `activity()`: a number that grows whenever the app works ([`Counters::activity`]), to tell
///   when the page is idle. Allocates nothing.
/// - `since()`: the app's metrics since `start()` as JSON, `{<metric name>: value}`
///   ([`Metric::name`]); `null` before a `start()`. Reads the counters before it allocates.
/// - `subjects()`: the hooks and atoms that ran on the page so far, as reported by leptonic
///   (`leptonic::metrics::ran`), by id: `hooks/use_press`, `atoms/button`.
pub fn register() {
    let counters = js_sys::Object::new();
    let start = Closure::<dyn Fn()>::new(|| {
        LARGEST_ALLOCATION.store(0, Ordering::Relaxed);
        START.set(Some(Counters::now()));
    });
    let activity = Closure::<dyn Fn() -> f64>::new(|| Counters::now().activity());
    let since = Closure::<dyn Fn() -> String>::new(|| {
        let end = Counters::now();
        START
            .get()
            .map_or_else(|| "null".to_owned(), |start| end.since_json(start))
    });
    let subjects = Closure::<dyn Fn() -> js_sys::Array>::new(|| {
        leptonic::metrics::ran()
            .into_iter()
            .map(|subject| JsValue::from_str(&subject.id()))
            .collect::<js_sys::Array>()
    });
    for (name, closure) in [
        ("start", start.as_ref()),
        ("activity", activity.as_ref()),
        ("since", since.as_ref()),
        ("subjects", subjects.as_ref()),
    ] {
        let _ = js_sys::Reflect::set(&counters, &JsValue::from_str(name), closure);
    }
    let _ = js_sys::Reflect::set(&window(), &JsValue::from_str("leptonicCounters"), &counters);
    // The page's lifetime.
    start.forget();
    activity.forget();
    since.forget();
    subjects.forget();
}
