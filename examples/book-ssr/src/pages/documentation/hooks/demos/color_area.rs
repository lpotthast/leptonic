use leptonic::{
    components::prelude::*,
    hooks::*,
    utils::{
        color::{ColorValue, HSV, HsvChannel},
        css::{CssColor, CssDimension, LengthPercentageAuto, try_pct},
        style::{BackgroundColorProperty, BottomProperty, LeftProperty},
        styles::Styles,
    },
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
    let disabled = RwSignal::new(false);

    let area = use_color_area(UseColorAreaInput {
        state,
        is_disabled: disabled.into(),
        aria_label: Some("Color area"),
        x_name: None,
        y_name: None,
        form: None,
    });

    // The gradient and its blend mode are free-form values that leptos-css does not model, so they
    // go through the explicit unchecked escape hatch.
    let background = area.background;
    let blend_mode = area.background_blend_mode;
    let area_styles = Styles::new()
        .add_optional_unchecked("background", move || Some(background.get()))
        .add_optional_unchecked("background-blend-mode", move || blend_mode.get());

    // Position and color of the thumb are live values with typed declarations.
    // `try_pct` guards against a non-finite percentage, for which `pct` would panic.
    let thumb_x = area.thumb_x_percent;
    let thumb_y = area.thumb_y_percent;
    let thumb_styles = Styles::builder()
        .with_reactive(move || {
            LeftProperty.declare(LengthPercentageAuto::from(
                try_pct(thumb_x.get()).unwrap_or(CssDimension::Zero),
            ))
        })
        .with_reactive(move || {
            BottomProperty.declare(LengthPercentageAuto::from(
                try_pct(thumb_y.get()).unwrap_or(CssDimension::Zero),
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
        <div {..area.area_props.into_attrs()} class="demo-color-area" style=area_styles>
            <div {..area.thumb_props.into_attrs()} class="demo-color-area-thumb" style=thumb_styles>
                <input {..area.x_input_props.into_attrs()} class="demo-visually-hidden-input" />
                <input {..area.y_input_props.into_attrs()} class="demo-visually-hidden-input" />
            </div>
        </div>

        <p class="demo-mt-half">
            "Current color: " <span class="demo-color-preview" style=preview_styles></span> " "
            <code>{move || display_color.get().to_css_string()}</code>
        </p>

        <Checkbox state=disabled>"Disabled"</Checkbox>
    }
}
