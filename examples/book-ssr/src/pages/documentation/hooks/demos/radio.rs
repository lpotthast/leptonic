use leptonic::{atoms::checkbox::Checkbox, hooks::*};
use leptos::prelude::*;

const SIZES: [(&str, &str); 3] = [("s", "Small"), ("m", "Medium"), ("l", "Large")];

#[component]
pub fn RadioDemo() -> impl IntoView {
    let disabled = RwSignal::new(false);
    let read_only = RwSignal::new(false);

    let state = use_radio_group_state(UseRadioGroupStateInput {
        default_value: Some(Key::from("m")),
        name: Some("size".to_owned()),
        is_disabled: disabled.into(),
        is_read_only: read_only.into(),
        ..UseRadioGroupStateInput::default()
    });
    let group = use_radio_group(UseRadioGroupInput {
        has_label: true.into(),
        state,
        id: None,
        aria_label: MaybeProp::default(),
        aria_labelledby: None,
        aria_describedby: None,
        aria_errormessage: None,
        orientation: Orientation::Vertical,
        form: None,
        on_focus: None,
        on_blur: None,
        on_focus_change: None,
    });
    let data = group.data;

    // The label of the selected size.
    let selected_size = move || {
        state.selected_value.get().and_then(|selected| {
            SIZES
                .into_iter()
                .find(|(value, _)| Key::from(*value) == selected)
                .map(|(_, label)| label)
        })
    };

    view! {
        <div {..group.props.into_attrs()} class="demo-choice-group">
            <span {..group.label_props.into_attrs()} class="demo-choice-group-label">"Size"</span>
            {SIZES
                .into_iter()
                .map(|(value, label)| view! { <SizeRadio group=data value label/> })
                .collect_view()}
        </div>

        <p class="demo-status">
            {move || selected_size().map_or_else(|| "No size selected.".to_owned(), |size| format!("Size: {size}."))}
        </p>

        <div class="demo-controls">
            <Checkbox is_selected=disabled set_selected=disabled classes="demo-check">
                <span class="demo-check-box" aria-hidden="true"></span>
                "Disabled"
            </Checkbox>
            <Checkbox is_selected=read_only set_selected=read_only classes="demo-check">
                <span class="demo-check-box" aria-hidden="true"></span>
                "Read-only"
            </Checkbox>
        </div>
    }
}

#[component]
fn SizeRadio(group: RadioGroupData, value: &'static str, label: &'static str) -> impl IntoView {
    let radio = use_radio(UseRadioInput {
        group,
        value: value.into(),
        is_disabled: Signal::stored(false),
        id: None,
        aria_label: MaybeProp::default(),
        aria_labelledby: None,
        aria_describedby: None,
        auto_focus: false,
        on_focus: None,
        on_blur: None,
        on_focus_change: None,
        on_press_start: None,
        on_press_end: None,
        on_press_up: None,
        on_press: None,
        on_press_change: None,
    });
    let (label_attrs, label_styles) = radio.label_props.into_parts();
    let (input_attrs, input_styles) = radio.input_props.into_parts();
    let is_focus_visible = radio.is_focus_visible;

    // The hook tracks keyboard focus; `data-focus-visible` lets the stylesheet draw a focus ring around the input.
    view! {
        <label
            {..label_attrs}
            style=label_styles
            class="demo-checkbox-label demo-no-margin"
            data-focus-visible=move || is_focus_visible.get().then_some("")
        >
            <input {..input_attrs} style=input_styles/>
            {label}
        </label>
    }
}
