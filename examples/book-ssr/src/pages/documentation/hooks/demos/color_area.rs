use leptonic::{
    hooks::*,
    utils::color::{ColorValue, HSV, HsvChannel},
};
use leptos::prelude::*;

#[component]
pub fn ColorAreaDemo() -> impl IntoView {
    let state = use_color_area_state(UseColorAreaStateInput {
        default_value: HSV::new(),
        x_channel: HsvChannel::Saturation,
        y_channel: HsvChannel::Brightness,
        x_channel_step: None,
        y_channel_step: None,
        on_change: None,
        on_change_end: None,
    });

    let display_color = state.display_color;

    let area = use_color_area(UseColorAreaInput {
        state,
        disabled: false.into(),
        aria_label: Some("Color area"),
        is_rtl: false,
        x_name: None,
        y_name: None,
        form: None,
    });

    view! {
        <div>
            <div
                {..area.area_props.into_attrs()}
                style=move || format!(
                    "position: relative; width: 200px; height: 200px; border-radius: 4px; touch-action: none; background: {};",
                    area.background.get()
                )
            >
                <div
                    {..area.thumb_props.into_attrs()}
                    style=move || format!(
                        "position: absolute; width: 16px; height: 16px; border-radius: 50%; \
                         border: 2px solid white; box-shadow: 0 0 2px rgba(0,0,0,0.5); \
                         transform: translate(-50%, 50%); touch-action: none; \
                         left: {}%; bottom: {}%; background: {};",
                        area.thumb_x_percent.get(),
                        area.thumb_y_percent.get(),
                        area.thumb_color.get(),
                    )
                >
                    <input
                        {..area.x_input_props.into_attrs()}
                        style="opacity: 0.0001; width: 100%; height: 100%; pointer-events: none; position: absolute;"
                    />
                    <input
                        {..area.y_input_props.into_attrs()}
                        style="opacity: 0.0001; width: 100%; height: 100%; pointer-events: none; position: absolute;"
                    />
                </div>
            </div>

            <p style="margin-top: 0.5em;">
                "Current color: "
                <span
                    style=move || format!(
                        "display: inline-block; width: 1em; height: 1em; vertical-align: middle; border-radius: 2px; background: {};",
                        display_color.get().to_css_string()
                    )
                ></span>
                " "
                <code>{ move || display_color.get().to_css_string() }</code>
            </p>
        </div>
    }
}
