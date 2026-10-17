//! `use_color_channel_field` on plain elements (a `<label>`, a group `<div>` with the
//! `<input class="perf-input">` between a decrement `.perf-decrement` and an increment button
//! `.perf-increment` of `use_button`), with its state (`use_color_channel_field_state`, the hue of
//! an HSV color, 180 at first; disabled through the state), for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_color_channel_field.rs`): between two plain buttons
//! (`#perf-before`, `#perf-after`) to tab from and to, mounted at first. Its controls
//! ([`Control`]) mount and unmount it, disable and enable it, and read its hue (`hue`, `-1`
//! without a color).

use leptonic::{
    HSV, HsvChannel,
    hooks::{
        button::use_button,
        color::{
            UseColorChannelFieldInput, UseColorChannelFieldStateInput, use_color_channel_field,
            use_color_channel_field_state,
        },
        form::ValidationBehavior,
    },
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// The field: `use_color_channel_field_state` and `use_color_channel_field` on plain elements.
#[component]
fn Field(disabled: RwSignal<bool>, hue: StoredValue<f64>) -> impl IntoView {
    let state = use_color_channel_field_state(UseColorChannelFieldStateInput {
        default_value: Some(HSV {
            hue: 180.0,
            saturation: 0.5,
            brightness: 1.0,
        }),
        value: None,
        channel: HsvChannel::Hue,
        is_disabled: disabled.into(),
        is_read_only: Signal::stored(false),
        is_invalid: Signal::stored(false),
        validate: None,
        validation_behavior: ValidationBehavior::default(),
        name: None,
        on_change: Some(Callback::new(move |color: Option<HSV>| {
            hue.set_value(color.map_or(-1.0, |color| color.hue));
        })),
    });
    let field = use_color_channel_field(UseColorChannelFieldInput {
        state,
        has_label: Signal::stored(true),
        id: None,
        aria_label: MaybeProp::default(),
        aria_labelledby: Signal::default(),
        aria_describedby: Signal::default(),
        is_required: Signal::stored(false),
        placeholder: MaybeProp::default(),
        auto_focus: false,
        is_wheel_disabled: false,
        increment_aria_label: MaybeProp::default(),
        decrement_aria_label: MaybeProp::default(),
        on_focus: None,
        on_blur: None,
        on_focus_change: None,
        on_key_down: None,
        on_key_up: None,
    });
    let (decrement_attrs, decrement_styles) = use_button(field.decrement_button).button.attrs();
    let (increment_attrs, increment_styles) = use_button(field.increment_button).button.attrs();
    view! {
        <label {..field.label.attrs()}>"Hue"</label>
        <div {..field.group.attrs()}>
            <button {..decrement_attrs} class="perf-decrement" style=decrement_styles>
                "-"
            </button>
            <input {..field.input.attrs()} class="perf-input" />
            <button {..increment_attrs} class="perf-increment" style=increment_styles>
                "+"
            </button>
        </div>
    }
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let disabled = RwSignal::new(false);
    let hue = StoredValue::new(180.0);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("disable", move || disabled.set(true)),
        Control::action("enable", move || disabled.set(false)),
        Control::read("hue", move || hue.get_value()),
    ];
    view! {
        <h1>"use_color_channel_field performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <Field disabled hue />
        </Show>
        <button id="perf-after">"After"</button>
    }
}
