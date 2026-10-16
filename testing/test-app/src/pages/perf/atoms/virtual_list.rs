//! A long virtualized log (`VirtualList`, `#perf-vl-log`), as the dev tool UI's: [`ROWS`] lines
//! (`data-line` holds a line's number), every seventh wrapping over several rows (estimated 20px
//! each, measured), in a 200px high view.
//!
//! Scenarios:
//! - `mount`: mounting the list with its lines ([`MOUNTS`] times, unmounted in between).
//! - `append-following-end`: appending [`BATCH`] lines [`ROUNDS`] times while the list follows its
//!   end (after [`WARMUP`] unmeasured appends).
//! - `append-at-start`: the same with the view at the start (not following).
//! - `resize-grown`: changing the list's width ([`ROUNDS`] times, between 300px and 285px, as a
//!   scrollbar appearing does, after [`WARMUP`] unmeasured changes) after it grew to
//!   [`GROWN_ROWS`] lines while following its end, [`GROWN_BATCH`] per frame: every line was laid
//!   out (as in a log that grew while shown), and a new width lays them out again.

use leptonic::{
    atoms::virtualizer::{VirtualList, VirtualListOptions},
    hooks::{collections::Key, virtualizer::ItemSize},
    leptos_styles::Styles,
};
use leptos::{prelude::*, wasm_bindgen::JsCast, web_sys};

use crate::pages::perf::{Measurements, PerfScenarios, Scenario, next_frame, settled};

/// The lines of the log before appending.
pub const ROWS: usize = 20_000;
/// Lines per append.
pub const BATCH: usize = 50;
/// Measured appends per scenario.
pub const ROUNDS: usize = 30;
/// Appends before measuring: the first lay out what the view shows, and the browser optimizes the
/// wasm's hot code only after a while (with 5, runs varied by up to 40%).
pub const WARMUP: usize = 20;
/// Measured mounts.
pub const MOUNTS: usize = 5;
/// The lines of the log grown for `resize-grown`.
pub const GROWN_ROWS: usize = 2_000;
/// Lines appended per frame while growing it: fewer than the view shows, so that each is laid out.
pub const GROWN_BATCH: usize = 8;

fn line_text(i: usize) -> String {
    if i.is_multiple_of(7) {
        format!("Line {i} {}", "long text that wraps ".repeat(12))
    } else {
        format!("Line {i}")
    }
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let lines = RwSignal::new(Vec::<usize>::new());
    let mounted = RwSignal::new(false);
    let follow = RwSignal::new(true);
    // The list's width, in px.
    let width = RwSignal::new(300);

    // Mounts the list with `ROWS` lines (following its end if `following`), laid out and measured.
    let mount = move |following: bool| async move {
        mounted.set(false);
        settled().await;
        follow.set(following);
        lines.set((0..ROWS).collect());
        mounted.set(true);
        next_frame().await;
        next_frame().await;
    };
    let append = move || {
        lines.update(|lines| {
            let start = lines.len();
            lines.extend(start..start + BATCH);
        });
    };
    let appends = move |scenario: &'static str, following: bool| async move {
        mount(following).await;
        if !following && let Some(log) = log_element() {
            log.set_scroll_top(0.0);
            next_frame().await;
        }
        for _ in 0..WARMUP {
            append();
            next_frame().await;
        }
        let mut measurements = parameters(Measurements::new(scenario))
            .with_parameter("batch", BATCH)
            .with_parameter("rounds", ROUNDS)
            .with_parameter("warmup", WARMUP)
            .with_parameter("following_end", following);
        for _ in 0..ROUNDS {
            measurements.measure("append", append).await;
        }
        measurements
    };
    let scenarios = vec![
        Scenario::new("mount", move || async move {
            let mut measurements =
                parameters(Measurements::new("mount")).with_parameter("mounts", MOUNTS);
            for _ in 0..MOUNTS {
                mounted.set(false);
                lines.set((0..ROWS).collect());
                next_frame().await;
                measurements.measure("mount", || mounted.set(true)).await;
            }
            measurements
        }),
        Scenario::new("append-following-end", move || {
            appends("append-following-end", true)
        }),
        Scenario::new("append-at-start", move || appends("append-at-start", false)),
        Scenario::new("resize-grown", move || async move {
            mounted.set(false);
            settled().await;
            width.set(300);
            follow.set(true);
            lines.set(Vec::new());
            mounted.set(true);
            next_frame().await;
            while lines.with_untracked(Vec::len) < GROWN_ROWS {
                lines.update(|lines| {
                    let start = lines.len();
                    lines.extend(start..start + GROWN_BATCH);
                });
                next_frame().await;
            }
            let toggle =
                move || width.update(|width| *width = if *width == 300 { 285 } else { 300 });
            for _ in 0..WARMUP {
                toggle();
                next_frame().await;
            }
            let mut measurements = Measurements::new("resize-grown")
                .with_parameter("rows", GROWN_ROWS)
                .with_parameter("rounds", ROUNDS)
                .with_parameter("warmup", WARMUP);
            for _ in 0..ROUNDS {
                measurements.measure("resize", toggle).await;
            }
            measurements
        }),
    ];

    view! {
        <h1>"VirtualList performance"</h1>
        <PerfScenarios scenarios=scenarios />
        <Show when=move || mounted.get()>
            <div style:width=move || format!("{}px", width.get())>
                <VirtualList
                    items=lines
                    key=|i: &usize| Key::from(*i)
                    layout_options=VirtualListOptions {
                        row_size: ItemSize::Estimated(20.0),
                        end_threshold: 10.0,
                        ..VirtualListOptions::default()
                    }
                    should_observe_item_size=true
                    is_anchored_to_end=follow
                    set_anchored_to_end=follow
                    styles=Styles::new()
                        .add_unchecked("width", "100%")
                        .add_unchecked("height", "200px")
                    attr:id="perf-vl-log"
                    attr:role="log"
                    attr:aria-label="Log"
                    let:i
                >
                    <div
                        class="line"
                        data-line=i
                        style="font-family: monospace; white-space: pre-wrap; overflow-wrap: anywhere"
                    >
                        {line_text(i)}
                    </div>
                </VirtualList>
            </div>
        </Show>
    }
}

/// The fixture's parameters, recorded with every scenario.
fn parameters(measurements: Measurements) -> Measurements {
    measurements.with_parameter("rows", ROWS)
}

/// The list's scroll view.
fn log_element() -> Option<web_sys::HtmlElement> {
    document()
        .get_element_by_id("perf-vl-log")?
        .dyn_into::<web_sys::HtmlElement>()
        .ok()
}
