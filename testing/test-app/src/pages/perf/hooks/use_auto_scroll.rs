//! The hook `use_auto_scroll` alone, for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_auto_scroll.rs`): a 100 × 100 px scrolling element
//! (`#perf-auto-scroll`, 1,000px of content), the drop target it scrolls during native drags
//! near its edges (only desktop Safari, which doesn't scroll during native drags itself),
//! mounted at first.
//!
//! Its controls ([`Control`]) mount and unmount it and call its handle: `move-to-the-middle`
//! (no edge near: no scrolling), `move-near-the-bottom` (5px from it: scrolls 15px per frame),
//! `stop`, and `scroll-for-frames`: near the bottom for [`FRAMES`] frames, then stopped (a
//! measurement must end with the page idle, which it isn't while scrolling).

use leptonic::{
    CapturedElement,
    hooks::dnd::{AutoScroll, use_auto_scroll},
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// The frames `scroll-for-frames` scrolls for.
pub const FRAMES: u32 = 5;

/// Runs `f` in the animation frame `frames` frames from now (the callback requested after those
/// of the current frame's code, so it runs after them).
fn after_frames(frames: u32, f: impl FnOnce() + 'static) {
    request_animation_frame(move || {
        if frames <= 1 {
            f();
        } else {
            after_frames(frames - 1, f);
        }
    });
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let handle = StoredValue::new(None::<AutoScroll>);
    let with_handle = move |f: fn(AutoScroll)| {
        if let Some(handle) = handle.get_value() {
            f(handle);
        }
    };
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("move-to-the-middle", move || {
            with_handle(|handle| handle.move_to(50.0, 50.0));
        }),
        Control::action("move-near-the-bottom", move || {
            with_handle(|handle| handle.move_to(50.0, 95.0));
        }),
        Control::action("stop", move || with_handle(|handle| handle.stop())),
        Control::action("scroll-for-frames", move || {
            with_handle(|handle| {
                handle.move_to(50.0, 95.0);
                after_frames(FRAMES, move || handle.stop());
            });
        }),
    ];
    view! {
        <h1>"use_auto_scroll performance"</h1>
        <PerfControls controls />
        <Show when=move || mounted.get()>
            <Scrolling handle />
        </Show>
    }
}

/// The scrolling element.
#[component]
fn Scrolling(handle: StoredValue<Option<AutoScroll>>) -> impl IntoView {
    let element = CapturedElement::new();
    handle.set_value(Some(use_auto_scroll(element)));
    on_cleanup(move || handle.set_value(None));
    view! {
        <div
            id="perf-auto-scroll"
            {..element.attr()}
            style="width: 100px; height: 100px; overflow: auto"
        >
            <div style="height: 1000px"></div>
        </div>
    }
}
