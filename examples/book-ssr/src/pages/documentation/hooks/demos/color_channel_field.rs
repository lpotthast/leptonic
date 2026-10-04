use leptonic::{
    hooks::*,
    utils::color::{ColorValue, HSV, HsvChannel},
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

    let field = use_color_channel_field(UseColorChannelFieldInput {
        state,
        is_disabled: Signal::derive(|| false),
        is_read_only: Signal::derive(|| false),
        aria_label: None,
    });

    let (decrement_attrs, decrement_styles) = use_button(field.decrement_button).props.into_parts();
    let (increment_attrs, increment_styles) = use_button(field.increment_button).props.into_parts();

    view! {
        <div>
            <div {..field.group_props.into_attrs()}>
                <label
                    {..field.label_props.into_attrs()}
                    style="display: block; font-weight: 500; margin-bottom: 0.25em;"
                >
                    "Hue (0\u{2013}360)"
                </label>
                <div style="display: flex; gap: 0.25em; align-items: center;">
                    <button
                        {..decrement_attrs}
                        style=decrement_styles
                        class="demo-stepper-btn"
                    >
                        "\u{2212}"
                    </button>
                    <input
                        prop:value=field.display_value
                        {..field.input_props.into_attrs()}
                        style="padding: 0.5em; border: 1px solid #ccc; border-radius: 4px; \
                               width: 80px; text-align: center;"
                    />
                    <button
                        {..increment_attrs}
                        style=increment_styles
                        class="demo-stepper-btn"
                    >
                        "+"
                    </button>
                    <span style=move || format!(
                        "display: inline-block; width: 32px; height: 32px; border-radius: 4px; \
                         border: 1px solid #ccc; background: {};",
                        color_value.get().to_css_string()
                    )></span>
                </div>
            </div>

            <p style="margin-top: 0.5em; font-size: 0.85em;">
                "Color: "
                <code>{ move || color_value.get().to_css_string() }</code>
            </p>
        </div>
    }
}
