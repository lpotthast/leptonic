use leptonic::{
    components::prelude::*,
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
    let state = use_color_channel_field_state(&UseColorChannelFieldStateInput {
        default_value: HSV::new(),
        channel: HsvChannel::Hue,
        on_change: None,
    });

    let color_value = state.color_value;
    let disabled = RwSignal::new(false);

    let field = use_color_channel_field(UseColorChannelFieldInput {
        state,
        is_disabled: disabled.into(),
        is_read_only: Signal::stored(false),
        aria_label: None,
    });

    let (decrement_attrs, decrement_styles) = use_button(field.decrement_button).props.into_parts();
    let (increment_attrs, increment_styles) = use_button(field.increment_button).props.into_parts();

    let preview_styles = Styles::new().add_reactive(move || {
        BackgroundColorProperty.declare(CssColor::from(color_value.get().into_rgb8()))
    });

    view! {
        <div>
            <div {..field.group_props.into_attrs()}>
                <label {..field.label_props.into_attrs()} class="demo-color-channel-field-label">
                    "Hue (0\u{2013}360)"
                </label>
                <div class="demo-color-channel-field-row">
                    <button {..decrement_attrs} style=decrement_styles class="demo-stepper-btn">
                        "\u{2212}"
                    </button>
                    <input {..field.input_props.into_attrs()} class="demo-color-channel-field-input"/>
                    <button {..increment_attrs} style=increment_styles class="demo-stepper-btn">
                        "+"
                    </button>
                    <span class="demo-color-channel-field-preview" style=preview_styles></span>
                </div>
            </div>

            <Checkbox state=disabled>"Disabled"</Checkbox>

            <p class="demo-mt-half">
                "Color: "
                <code>{move || color_value.get().to_css_string()}</code>
            </p>
        </div>
    }
}
