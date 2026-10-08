use leptonic::{
    atoms::checkbox::{CheckboxButton, CheckboxField},
    hooks::*,
    utils::{color::RGB8, css::CssColor, style::BackgroundColorProperty, styles::Styles},
};
use leptos::prelude::*;

#[component]
pub fn ColorFieldDemo() -> impl IntoView {
    let disabled = RwSignal::new(false);

    let state = use_color_field_state(UseColorFieldStateInput {
        default_value: Some(RGB8 {
            r: 66,
            g: 135,
            b: 245,
        }),
        ..UseColorFieldStateInput::default()
    });
    let field = use_color_field(UseColorFieldInput {
        has_label: true.into(),
        is_disabled: disabled.into(),
        state,
        id: None,
        aria_label: MaybeProp::default(),
        aria_labelledby: None,
        aria_describedby: None,
        is_read_only: Signal::stored(false),
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

    // An empty field (no color) leaves the preview transparent.
    let color = state.color_value;
    let preview_styles = Styles::new().add_optional(move || {
        color
            .get()
            .map(|c| BackgroundColorProperty.declare(CssColor::from(c)))
    });

    view! {
        <div class="demo-field">
            <label class="demo-field-label" {..field.label_props.into_attrs()}>"Accent color"</label>
            <div class="demo-color-field-row">
                <input class="demo-color-field-input" {..field.input_props.into_attrs()}/>
                <span class="demo-color-field-preview" style=preview_styles></span>
            </div>
        </div>

        <p class="demo-status">
            {move || color.get().map_or_else(|| "No color".to_owned(), |c| format!("Color: {c}"))}
        </p>
        <div class="demo-controls">
            <CheckboxField is_selected=disabled set_selected=disabled>
                <CheckboxButton classes="demo-check">
                    <span class="demo-check-box" aria-hidden="true"></span>
                    "Disabled"
                </CheckboxButton>
            </CheckboxField>
        </div>
    }
}
