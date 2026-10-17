//! `use_color_wheel` on plain elements (a ring `<div class="perf-track">`, radii 100 and 74, its
//! thumb `.perf-thumb` of 20 × 20 pixels with the hidden input, siblings in a positioned
//! `<div>`), with its state (`use_color_wheel_state`, an HSV color of hue 180 at first; disabled
//! through the state), for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_color_wheel.rs`): between two plain buttons
//! (`#perf-before`, `#perf-after`) to tab from and to, mounted at first. Its controls
//! ([`Control`]) mount and unmount it, disable and enable it, change its radii, set and unset its
//! label, and read its hue (`hue`) and how often it changed (`changes`, `change-ends`).

use leptonic::{
    HSV,
    hooks::color::{
        UseColorWheelInput, UseColorWheelStateInput, use_color_wheel, use_color_wheel_state,
    },
    leptos_styles::Styles,
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// The wheel: `use_color_wheel_state` and `use_color_wheel` on plain elements.
#[component]
fn Wheel(
    disabled: RwSignal<bool>,
    outer_radius: RwSignal<f64>,
    label: RwSignal<Option<String>>,
    hue: StoredValue<f64>,
    change_count: StoredValue<u32>,
    end_count: StoredValue<u32>,
) -> impl IntoView {
    let state = use_color_wheel_state(UseColorWheelStateInput {
        default_value: HSV {
            hue: 180.0,
            saturation: 1.0,
            brightness: 1.0,
        },
        value: None,
        is_disabled: disabled.into(),
        on_change: Some(Callback::new(move |color: HSV| {
            hue.set_value(color.hue);
            change_count.update_value(|count| *count += 1);
        })),
        on_change_end: Some(Callback::new(move |_: HSV| {
            end_count.update_value(|count| *count += 1);
        })),
    });
    let wheel = use_color_wheel(UseColorWheelInput {
        state,
        outer_radius: outer_radius.into(),
        inner_radius: Signal::derive(move || outer_radius.get() - 26.0),
        aria_label: label.into(),
        aria_labelledby: Signal::default(),
        aria_describedby: Signal::default(),
        aria_details: Signal::default(),
        aria_errormessage: Signal::default(),
        name: None,
        form: None,
    });
    let (track_attrs, track_styles) = wheel.track.attrs();
    let (thumb_attrs, thumb_styles) = wheel.thumb.attrs();
    let (input_attrs, input_styles) = wheel.input.attrs();
    view! {
        <div style="position: relative">
            <div {..track_attrs} class="perf-track" style=track_styles></div>
            <div
                {..thumb_attrs}
                class="perf-thumb"
                style=thumb_styles
                    .merge(Styles::new().add_unchecked("width", "20px").add_unchecked("height", "20px"))
            >
                <input {..input_attrs} style=input_styles />
            </div>
        </div>
    }
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let disabled = RwSignal::new(false);
    let outer_radius = RwSignal::new(100.0);
    let label = RwSignal::new(None::<String>);
    let hue = StoredValue::new(180.0);
    let change_count = StoredValue::new(0_u32);
    let end_count = StoredValue::new(0_u32);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("disable", move || disabled.set(true)),
        Control::action("enable", move || disabled.set(false)),
        Control::action("grow", move || outer_radius.set(120.0)),
        Control::action("shrink", move || outer_radius.set(100.0)),
        Control::action("set-label", move || label.set(Some("Tint".to_owned()))),
        Control::action("unset-label", move || label.set(None)),
        Control::read("hue", move || hue.get_value()),
        Control::read("changes", move || f64::from(change_count.get_value())),
        Control::read("change-ends", move || f64::from(end_count.get_value())),
    ];
    view! {
        <h1>"use_color_wheel performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <Wheel disabled outer_radius label hue change_count end_count />
        </Show>
        <button id="perf-after">"After"</button>
    }
}
