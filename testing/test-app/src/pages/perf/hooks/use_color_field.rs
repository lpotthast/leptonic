//! `use_color_field` on a plain `<label>` and `<input class="perf-input">`, with its state
//! (`use_color_field_state`, an RGB color, `#808080` at first), for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_color_field.rs`): between two plain buttons
//! (`#perf-before`, `#perf-after`) to tab from and to, mounted at first. Its controls
//! ([`Control`]) mount and unmount it, change its inputs (disabled, read-only), and read its color
//! (`value`, as a number `0xRRGGBB`, `-1` without a color).

use leptonic::{
    IntoAttrs, RGB8,
    hooks::color::{
        UseColorFieldInput, UseColorFieldStateInput, use_color_field, use_color_field_state,
    },
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

const GRAY: RGB8 = RGB8 {
    r: 128,
    g: 128,
    b: 128,
};

/// `color` as a number `0xRRGGBB` (`-1`: none).
fn number(color: Option<RGB8>) -> f64 {
    color.map_or(-1.0, |color| {
        f64::from((u32::from(color.r) << 16) | (u32::from(color.g) << 8) | u32::from(color.b))
    })
}

/// The field: `use_color_field_state` and `use_color_field` on plain elements.
#[component]
fn Field(
    disabled: RwSignal<bool>,
    read_only: RwSignal<bool>,
    value: StoredValue<Option<RGB8>>,
) -> impl IntoView {
    let state = use_color_field_state(UseColorFieldStateInput {
        default_value: Some(GRAY),
        on_change: Some(Callback::new(move |color: Option<RGB8>| {
            value.set_value(color);
        })),
        ..UseColorFieldStateInput::default()
    });
    let field = use_color_field(UseColorFieldInput {
        state,
        id: None,
        has_label: Signal::stored(true),
        aria_label: MaybeProp::default(),
        aria_labelledby: Signal::default(),
        aria_describedby: Signal::default(),
        is_disabled: disabled.into(),
        is_read_only: read_only.into(),
        is_required: Signal::stored(false),
        placeholder: MaybeProp::default(),
        auto_focus: false,
        is_wheel_disabled: false,
        on_focus: None,
        on_blur: None,
        on_focus_change: None,
        on_key_down: None,
        on_key_up: None,
    });
    view! {
        <label {..field.label_props.into_attrs()}>"Color"</label>
        <input {..field.input_props.into_attrs()} class="perf-input" />
    }
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let disabled = RwSignal::new(false);
    let read_only = RwSignal::new(false);
    let value = StoredValue::new(Some(GRAY));
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("disable", move || disabled.set(true)),
        Control::action("enable", move || disabled.set(false)),
        Control::action("make-read-only", move || read_only.set(true)),
        Control::action("make-editable", move || read_only.set(false)),
        Control::read("value", move || number(value.get_value())),
    ];
    view! {
        <h1>"use_color_field performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <Field disabled read_only value />
        </Show>
        <button id="perf-after">"After"</button>
    }
}
