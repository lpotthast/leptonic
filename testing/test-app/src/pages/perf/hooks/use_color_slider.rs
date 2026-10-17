//! `use_color_slider` on plain elements (a group/track `<div class="perf-track">` of 200 × 20
//! pixels, its thumb `.perf-thumb` of 10 × 20 pixels with the hidden input, an `<output>`
//! `.perf-output`), with its state (`use_color_slider_state`, the red channel of an RGB color,
//! `#8080FF` at first; disabled through the state), for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_color_slider.rs`): between two plain buttons
//! (`#perf-before`, `#perf-after`) to tab from and to, mounted at first. Its controls
//! ([`Control`]) mount and unmount it, disable and enable it, set and unset its label, and read
//! its value (`value`, as a number `0xRRGGBB`) and how often it changed (`changes`,
//! `change-ends`).

use leptonic::{
    Orientation, RGB8, RgbChannel,
    hooks::color::{
        UseColorSliderInput, UseColorSliderReturn, UseColorSliderStateInput, use_color_slider,
        use_color_slider_state,
    },
    leptos_styles::Styles,
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

const START: RGB8 = RGB8 {
    r: 128,
    g: 128,
    b: 255,
};

/// `color` as a number `0xRRGGBB`.
fn number(color: RGB8) -> f64 {
    f64::from((u32::from(color.r) << 16) | (u32::from(color.g) << 8) | u32::from(color.b))
}

/// The slider: `use_color_slider_state` and `use_color_slider` on plain elements.
#[component]
fn Slider(
    disabled: RwSignal<bool>,
    label: RwSignal<Option<String>>,
    value: StoredValue<RGB8>,
    change_count: StoredValue<u32>,
    end_count: StoredValue<u32>,
) -> impl IntoView {
    let state = use_color_slider_state(UseColorSliderStateInput {
        default_value: START,
        value: None,
        channel: RgbChannel::Red,
        is_disabled: disabled.into(),
        orientation: Signal::stored(Orientation::Horizontal),
        on_change: Some(Callback::new(move |color: RGB8| {
            value.set_value(color);
            change_count.update_value(|count| *count += 1);
        })),
        on_change_end: Some(Callback::new(move |_: RGB8| {
            end_count.update_value(|count| *count += 1);
        })),
    });
    let UseColorSliderReturn {
        slider,
        thumb,
        track_styles,
        input_styles,
    } = use_color_slider(UseColorSliderInput {
        state,
        has_label: Signal::stored(false),
        aria_label: label.into(),
        aria_labelledby: Signal::default(),
        aria_describedby: Signal::default(),
        aria_details: Signal::default(),
        name: None,
        form: None,
    });
    let (track_attrs, slider_track_styles) = slider
        .track
        .props()
        .expect("the first track takes the props")
        .into_parts();
    let (thumb_attrs, thumb_styles) = thumb.thumb.attrs();
    let formatted = state.formatted_value();
    view! {
        <output {..slider.output.attrs()} class="perf-output">
            {formatted}
        </output>
        <div
            {..slider.group.attrs()}
            {..track_attrs}
            class="perf-track"
            style=slider_track_styles
                .merge(track_styles)
                .merge(Styles::new().add_unchecked("width", "200px").add_unchecked("height", "20px"))
        >
            <div
                {..thumb_attrs}
                class="perf-thumb"
                style=thumb_styles
                    .merge(Styles::new().add_unchecked("width", "10px").add_unchecked("height", "20px"))
            >
                <input {..thumb.input.attrs()} style=input_styles />
            </div>
        </div>
    }
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let disabled = RwSignal::new(false);
    let label = RwSignal::new(None::<String>);
    let value = StoredValue::new(START);
    let change_count = StoredValue::new(0_u32);
    let end_count = StoredValue::new(0_u32);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("disable", move || disabled.set(true)),
        Control::action("enable", move || disabled.set(false)),
        Control::action("set-label", move || label.set(Some("Redness".to_owned()))),
        Control::action("unset-label", move || label.set(None)),
        Control::read("value", move || number(value.get_value())),
        Control::read("changes", move || f64::from(change_count.get_value())),
        Control::read("change-ends", move || f64::from(end_count.get_value())),
    ];
    view! {
        <h1>"use_color_slider performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <Slider disabled label value change_count end_count />
        </Show>
        <button id="perf-after">"After"</button>
    }
}
