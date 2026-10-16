//! `use_overlay_position` alone, for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_overlay_position.rs`): an overlay (`.perf-overlay`, 200px
//! wide, rendered while open, with the hook's props and styles; its resolved placement in
//! `data-placement`) positioned at a plain button (`#perf-target`, 100px wide, 150px from the top)
//! in a scrolling container (`#perf-scroller`), mounted at first, closed, below the target. The
//! open state is the app's; scrolling the container closes it (`on_close`). Its controls
//! ([`Control`]) mount and unmount it, open and close it, move it above the target and back,
//! change its offset, widen and narrow the target, and update its position (`PositionUpdater`).

use leptonic::{
    CapturedElement,
    hooks::overlay::{
        OverlayPositionOptions, Placement, PlacementAxis, PositionUpdater, UseOverlayPositionInput,
        use_overlay_position,
    },
    leptos_styles::Styles,
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let open = RwSignal::new(false);
    let placement = RwSignal::new(Placement::Bottom);
    let offset = RwSignal::new(8.0);
    let target_width = RwSignal::new(100);
    let updater = StoredValue::new(None::<PositionUpdater>);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("open", move || open.set(true)),
        Control::action("close", move || open.set(false)),
        Control::action("place-above", move || placement.set(Placement::Top)),
        Control::action("place-below", move || placement.set(Placement::Bottom)),
        Control::action("offset-far", move || offset.set(24.0)),
        Control::action("offset-near", move || offset.set(8.0)),
        Control::action("widen-target", move || target_width.set(300)),
        Control::action("narrow-target", move || target_width.set(100)),
        Control::action("update", move || {
            if let Some(updater) = updater.get_value() {
                updater.update();
            }
        }),
    ];
    view! {
        <h1>"use_overlay_position performance"</h1>
        <PerfControls controls />
        <div id="perf-scroller" style="overflow: auto; height: 400px;">
            <div style="height: 800px; padding: 150px 300px;">
                <Show when=move || mounted.get()>
                    <Subject open placement offset target_width updater />
                </Show>
            </div>
        </div>
    }
}

/// Calls the hook, with a plain target and a plain overlay while open.
#[component]
fn Subject(
    open: RwSignal<bool>,
    placement: RwSignal<Placement>,
    offset: RwSignal<f64>,
    target_width: RwSignal<i32>,
    updater: StoredValue<Option<PositionUpdater>>,
) -> impl IntoView {
    let target = CapturedElement::new();
    let position = use_overlay_position(UseOverlayPositionInput {
        target,
        is_open: open.into(),
        position: OverlayPositionOptions {
            placement: placement.into(),
            offset: offset.into(),
            ..OverlayPositionOptions::default()
        },
        target_rect: Signal::stored(None),
        scroll: None,
        on_close: Some(Callback::new(move |()| open.set(false))),
    });
    updater.set_value(Some(position.updater));
    on_cleanup(move || updater.set_value(None));
    let resolved = position.placement;
    let (attrs, styles) = position.props.into_parts();
    let attrs = StoredValue::new(attrs);
    // The overlay's only `style` writer: the hook's styles and its width.
    let styles = StoredValue::new(styles.merge(Styles::new().add_unchecked("width", "200px")));
    view! {
        <button
            id="perf-target"
            style:width=move || format!("{}px", target_width.get())
            {..target.attr()}
        >
            "Target"
        </button>
        <Show when=move || open.get()>
            <div
                class="perf-overlay"
                {..attrs.get_value()}
                style=styles.get_value()
                data-placement=move || resolved.get().map(PlacementAxis::as_str)
            >
                "Overlay"
            </div>
        </Show>
    }
}
