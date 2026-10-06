use leptonic::{
    components::prelude::*,
    hooks::*,
    utils::{
        color::{HSV, HsvChannel},
        css::{CssColor, CssDimension, LengthPercentageAuto, try_pct},
        style::{BackgroundColorProperty, LeftProperty},
        styles::Styles,
    },
};
use leptos::prelude::*;

#[component]
pub fn ColorSliderDemo() -> impl IntoView {
    let disabled = RwSignal::new(false);
    let state = use_color_slider_state(&UseColorSliderStateInput {
        default_value: HSV::new(),
        channel: HsvChannel::Hue,
        is_disabled: disabled.into(),
        orientation: Orientation::Horizontal.into(),
        on_change: None,
        on_change_end: None,
    });

    let display_color = state.display_color;
    let thumb_label = state.thumb_value_label;

    let color_slider = use_color_slider(UseColorSliderInput {
        state,
        is_disabled: disabled.into(),
        orientation: Orientation::Horizontal.into(),
        aria_label: Some("Hue"),
        name: None,
    });

    let percentage = color_slider.thumb.percentage;
    let (track_props, track_styles) = color_slider.slider.track_props.into_parts();
    let background = color_slider.background;

    // The gradient is a free-form `background` value, which leptos-css does not model,
    // so it goes through the explicit unchecked escape hatch.
    let track_styles =
        track_styles.add_optional_unchecked("background", move || Some(background.get()));

    // Position and color of the thumb are live values with typed declarations.
    // `try_pct` guards against a non-finite percentage, for which `pct` would panic.
    let thumb_styles = Styles::builder()
        .with_reactive(move || {
            LeftProperty.declare(LengthPercentageAuto::from(
                try_pct(percentage.get()).unwrap_or(CssDimension::Zero),
            ))
        })
        .with_reactive(move || {
            BackgroundColorProperty.declare(CssColor::from(display_color.get().into_rgb8()))
        })
        .build();

    let preview_styles = Styles::new().add_reactive(move || {
        BackgroundColorProperty.declare(CssColor::from(display_color.get().into_rgb8()))
    });

    view! {
        <div>
            <div {..track_props} class="demo-color-slider-track" style=track_styles>
                <div
                    {..color_slider.thumb.thumb_props.into_attrs()}
                    class="demo-color-slider-thumb"
                    style=thumb_styles
                >
                    <input
                        {..color_slider.thumb.input_props.into_attrs()}
                        class="demo-visually-hidden-input"
                    />
                </div>
            </div>

            <p class="demo-mt-half">
                "Hue: " <span class="demo-color-preview" style=preview_styles></span> " "
                <code>{move || thumb_label.get()}</code>
            </p>

            <Checkbox state=disabled>"Disabled"</Checkbox>
        </div>
    }
}
