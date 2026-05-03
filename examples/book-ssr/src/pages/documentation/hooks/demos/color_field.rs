use leptonic::{hooks::*, utils::color::RGB8};
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

    let field = use_color_field(UseColorFieldInput {
        state,
        disabled: false.into(),
        read_only: false.into(),
        aria_label: Some("Hex color"),
        is_wheel_disabled: false,
    });

    view! {
        <div>
            <label style="display: block; font-weight: 500; margin-bottom: 0.25em;">
                "Hex Color"
            </label>
            <div style="display: flex; align-items: center; gap: 0.5em;">
                <span style="font-family: monospace; color: #666;">"#"</span>
                <input
                    prop:value={input_value}
                    {..field.input_props.into_attrs()}
                    style="padding: 0.5em; border: 1px solid #ccc; border-radius: 4px; \
                           width: 120px; font-family: monospace; text-transform: uppercase;"
                />
                <span style=move || format!(
                    "display: inline-block; width: 32px; height: 32px; border-radius: 4px; \
                     border: 1px solid #ccc; background: {};",
                    color_value.get().map_or_else(
                        || "transparent".to_string(),
                        |c| format!("rgb({}, {}, {})", c.r, c.g, c.b),
                    )
                )></span>
            </div>

            <p style="margin-top: 0.5em; font-size: 0.85em;">
                "Parsed: "
                <code>{ move || color_value.get().map_or_else(
                    || "(none)".to_string(),
                    |c| format!("rgb({}, {}, {})", c.r, c.g, c.b),
                )}</code>
            </p>
        </div>
    }
}
