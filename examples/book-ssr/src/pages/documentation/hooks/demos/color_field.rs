use leptonic::{
    components::prelude::*,
    hooks::*,
    utils::{color::RGB8, css::CssColor, style::BackgroundColorProperty, styles::Styles},
};
use leptos::prelude::*;

#[component]
pub fn ColorFieldDemo() -> impl IntoView {
    let state = use_color_field_state(UseColorFieldStateInput {
        default_value: Some(RGB8 {
            r: 66,
            g: 135,
            b: 245,
        }),
        on_change: None,
    });

    let color_value = state.color_value;
    let input_value = state.input_value;
    let disabled = RwSignal::new(false);

    let field = use_color_field(UseColorFieldInput {
        state,
        is_disabled: disabled.into(),
        is_read_only: false.into(),
        aria_label: Some("Hex color"),
        is_wheel_disabled: false,
    });

    // No color (an empty field) leaves the preview transparent.
    let preview_styles = Styles::new().add_optional(move || {
        color_value
            .get()
            .map(|c| BackgroundColorProperty.declare(CssColor::from(c)))
    });

    view! {
        <div>
            <div class="demo-color-field-row">
                // The hook only sets `aria-disabled`; disable the native input as well.
                <input
                    {..field.input_props.into_attrs()}
                    prop:value=input_value
                    disabled=disabled
                    class="demo-color-field-input"
                />
                <span class="demo-color-field-preview" style=preview_styles></span>
            </div>

            <Checkbox state=disabled>"Disabled"</Checkbox>

            <p class="demo-mt-half">
                "Parsed: "
                <code>
                    {move || {
                        color_value
                            .get()
                            .map_or_else(|| "(none)".to_owned(), |c| format!("rgb({}, {}, {})", c.r, c.g, c.b))
                    }}
                </code>
            </p>
        </div>
    }
}
