use leptonic::utils::css::{pct, px};
use leptonic::{
    hooks::*,
    utils::color::{ColorValue, HSV, HsvChannel},
};
use leptos::prelude::*;
use leptos_styles::{
    Background, Border, BorderRadius, BoxShadow, Cursor, Height, Left, Position, Styles, Top,
    TouchAction, Transform, Width,
};

#[component]
pub fn ColorSliderDemo() -> impl IntoView {
    let state = use_color_slider_state(&UseColorSliderStateInput {
        default_value: HSV::new(),
        channel: HsvChannel::Hue,
        disabled: false.into(),
        orientation: Signal::default(),
        on_change: None,
        on_change_end: None,
    });

    let display_color = state.display_color;
    let thumb_label = state.thumb_value_label;

    let color_slider = use_color_slider(UseColorSliderInput {
        state,
        disabled: false.into(),
        orientation: Signal::default(),
        aria_label: Some("Hue"),
        is_rtl: false,
        name: None,
    });

    let percentage = color_slider.thumb.percentage;
    let (track_props, track_styles) = color_slider.slider.track_props.into_parts();
    let background = color_slider.background;
    let track_styles = track_styles
        .add("position", "relative")
        .add("width", "100%")
        .add("height", "16px")
        .add("border-radius", "8px")
        .add("cursor", "pointer")
        .add("forced-color-adjust", "none");

    view! {
        <div>
            <div
                {..track_props}
                style=track_styles
                style:background=move || background.get()
            >
                <div
                    {..color_slider.thumb.thumb_props.into_attrs()}
                    style={Styles::builder()
                        .with(Position, "absolute")
                        .with(Top, pct(50.0))
                        .with(Transform, "translate(-50%, -50%)")
                        .with(Width, px(20))
                        .with(Height, px(20))
                        .with(BorderRadius, pct(50))
                        .with(Border, "2px solid white")
                        .with(BoxShadow, "0 2px 4px rgba(0,0,0,0.3)")
                        .with(Cursor, "grab")
                        .with(TouchAction, "none")
                        .with(Left, move || pct(percentage.get()))
                        .with(Background, move || display_color.get().to_css_string())
                        .build()}
                >
                    <input
                        {..color_slider.thumb.input_props.into_attrs()}
                        style="opacity: 0.0001; width: 100%; height: 100%; pointer-events: none; position: absolute; top: 0; left: 0;"
                    />
                </div>
            </div>

            <p style="margin-top: 0.5em;">
                "Hue: "
                <span style=move || format!(
                    "display: inline-block; width: 1em; height: 1em; vertical-align: middle; \
                     border-radius: 2px; background: {};",
                    display_color.get().to_css_string()
                )></span>
                " "
                <code>{ move || thumb_label.get() }</code>
            </p>
        </div>
    }
}
