//! `use_color_area` on plain elements (an area `<div class="perf-area">` of 200 × 200 pixels, its
//! thumb `.perf-thumb` of 10 × 10 pixels with the two hidden inputs), with its state
//! (`use_color_area_state`, red on x and green on y of an RGB color, `#8080FF` at first), for its
//! performance test (`leptonic/tests/perf_tests/hooks/use_color_area.rs`): the thumb's dragging
//! state shown (`data-dragging`), between two plain buttons (`#perf-before`, `#perf-after`) to
//! tab from and to, mounted at first. Its controls ([`Control`]) mount and unmount it, disable
//! and enable it, set and unset its label, and read its value (`value`, as a number `0xRRGGBB`)
//! and how often it changed (`changes`, `change-ends`).

use leptonic::{
    RGB8, flag,
    hooks::color::{
        UseColorAreaInput, UseColorAreaStateInput, use_color_area, use_color_area_state,
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

/// The area: `use_color_area_state` and `use_color_area` on plain elements.
#[component]
fn Area(
    disabled: RwSignal<bool>,
    label: RwSignal<Option<String>>,
    value: StoredValue<RGB8>,
    change_count: StoredValue<u32>,
    end_count: StoredValue<u32>,
) -> impl IntoView {
    let state = use_color_area_state(UseColorAreaStateInput {
        default_value: START,
        value: None,
        x_channel: None,
        y_channel: None,
        x_channel_step: None,
        y_channel_step: None,
        on_change: Some(Callback::new(move |color: RGB8| {
            value.set_value(color);
            change_count.update_value(|count| *count += 1);
        })),
        on_change_end: Some(Callback::new(move |_: RGB8| {
            end_count.update_value(|count| *count += 1);
        })),
    });
    let area = use_color_area(UseColorAreaInput {
        state,
        is_disabled: disabled.into(),
        aria_label: label.into(),
        aria_labelledby: Signal::default(),
        aria_describedby: Signal::default(),
        aria_details: Signal::default(),
        x_name: None,
        y_name: None,
        form: None,
    });
    let (area_attrs, area_styles) = area.color_area_props.into_parts();
    let (thumb_attrs, thumb_styles) = area.thumb_props.into_parts();
    let (x_attrs, x_styles) = area.x_input_props.into_parts();
    let (y_attrs, y_styles) = area.y_input_props.into_parts();
    view! {
        <div
            {..area_attrs}
            class="perf-area"
            style=area_styles
                .merge(Styles::new().add_unchecked("width", "200px").add_unchecked("height", "200px"))
        >
            <div
                {..thumb_attrs}
                class="perf-thumb"
                style=thumb_styles
                    .merge(Styles::new().add_unchecked("width", "10px").add_unchecked("height", "10px"))
                data-dragging=flag(state.is_dragging)
            >
                <input {..x_attrs} style=x_styles />
                <input {..y_attrs} style=y_styles />
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
        Control::action("set-label", move || label.set(Some("Color".to_owned()))),
        Control::action("unset-label", move || label.set(None)),
        Control::read("value", move || number(value.get_value())),
        Control::read("changes", move || f64::from(change_count.get_value())),
        Control::read("change-ends", move || f64::from(end_count.get_value())),
    ];
    view! {
        <h1>"use_color_area performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <Area disabled label value change_count end_count />
        </Show>
        <button id="perf-after">"After"</button>
    }
}
