use leptonic::{
    atoms::checkbox::Checkbox,
    hooks::*,
    utils::{
        color::{ColorValue, HSV, HsvChannel},
        css::CssColor,
        style::BackgroundColorProperty,
        styles::Styles,
    },
};
use leptos::prelude::*;

#[component]
pub fn ColorChannelFieldDemo() -> impl IntoView {
    let disabled = RwSignal::new(false);

    // The field edits the hue of a blue. Disabled lives on the state (as in a number field).
    let state = use_color_channel_field_state(UseColorChannelFieldStateInput {
        default_value: Some(HSV {
            hue: 210.0,
            saturation: 0.6,
            brightness: 0.8,
        }),
        is_disabled: disabled.into(),
        value: None,
        channel: HsvChannel::Hue,
        is_read_only: Signal::stored(false),
        is_invalid: Signal::stored(false),
        validate: None,
        validation_behavior: ValidationBehavior::default(),
        name: None,
        on_change: None,
    });
    let field = use_color_channel_field(UseColorChannelFieldInput {
        field: UseNumberFieldInput {
            has_label: true.into(),
            state: state.number,
            id: None,
            aria_label: MaybeProp::default(),
            aria_labelledby: None,
            aria_describedby: None,
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
        },
        state,
    });

    // The stepper buttons come as `UseButtonInput`s: render them with `use_button`.
    let (decrement_attrs, decrement_styles) = use_button(field.decrement_button).props.into_parts();
    let (increment_attrs, increment_styles) = use_button(field.increment_button).props.into_parts();

    let color = state.color_value;
    let preview_styles = Styles::new().add_optional(move || {
        color
            .get()
            .map(|c| BackgroundColorProperty.declare(CssColor::from(c.to_rgb8())))
    });

    view! {
        <div class="demo-field">
            <label class="demo-field-label" {..field.label_props.into_attrs()}>"Hue (0\u{2013}360)"</label>
            <div class="demo-color-field-row" {..field.group_props.into_attrs()}>
                <button class="demo-btn" {..decrement_attrs} style=decrement_styles>
                    <span aria-hidden="true">"\u{2212}"</span>
                </button>
                <input class="demo-color-channel-field-input" {..field.input_props.into_attrs()}/>
                <button class="demo-btn" {..increment_attrs} style=increment_styles>
                    <span aria-hidden="true">"+"</span>
                </button>
                <span class="demo-color-field-preview" style=preview_styles></span>
            </div>
        </div>

        <p class="demo-status">
            {move || color.get().map_or_else(|| "No color".to_owned(), |c| format!("Color: {}", c.to_css_string()))}
        </p>
        <div class="demo-controls">
            <Checkbox is_selected=disabled set_selected=disabled classes="demo-check">
                <span class="demo-check-box" aria-hidden="true"></span>
                "Disabled"
            </Checkbox>
        </div>
    }
}
