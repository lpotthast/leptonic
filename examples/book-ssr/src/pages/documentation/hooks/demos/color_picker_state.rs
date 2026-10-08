use leptonic::{
    atoms::button::Button,
    hooks::*,
    utils::{
        ValueBinding,
        color::{Color, ColorValue, HSV, HsvChannel, RGB8},
        css::CssColor,
        i18n::use_locale,
        style::BackgroundColorProperty,
    },
};
use leptos::prelude::*;

const BLUE: HSV = HSV {
    hue: 210.0,
    saturation: 0.6,
    brightness: 0.8,
};

#[component]
pub fn ColorPickerStateDemo() -> impl IntoView {
    let locale = use_locale();

    // The one color all parts share.
    let picker = use_color_picker_state(UseColorPickerStateInput {
        default_value: Color::from(BLUE),
        ..UseColorPickerStateInput::default()
    });
    // The shared color as HSV, for the parts that edit HSV channels.
    let hsv = ValueBinding::new(
        Signal::derive(move || picker.color.get().to::<HSV>()),
        Callback::new(move |color: HSV| picker.set_color(Color::from(color))),
    );

    // Two interactive parts edit the picker's color ...
    let area_state = use_color_area_state(UseColorAreaStateInput {
        default_value: BLUE,
        value: Some(hsv),
        x_channel: Some(HsvChannel::Saturation),
        y_channel: Some(HsvChannel::Brightness),
        x_channel_step: None,
        y_channel_step: None,
        on_change: None,
        on_change_end: None,
    });
    let area = use_color_area(UseColorAreaInput {
        is_disabled: false.into(),
        aria_label: "Saturation and brightness".into(),
        state: area_state,
        aria_labelledby: None,
        aria_describedby: None,
        aria_details: None,
        x_name: None,
        y_name: None,
        form: None,
    });
    let hue_state = use_color_slider_state(UseColorSliderStateInput {
        default_value: BLUE,
        value: Some(hsv),
        channel: HsvChannel::Hue,
        is_disabled: false.into(),
        orientation: Signal::stored(Orientation::Horizontal),
        on_change: None,
        on_change_end: None,
    });
    let hue = use_color_slider(UseColorSliderInput {
        has_label: false.into(),
        state: hue_state,
        aria_label: "Hue".into(),
        aria_labelledby: None,
        aria_describedby: None,
        name: None,
        form: None,
    });

    // ... and a display-only part reads it.
    let swatch = use_color_swatch(UseColorSwatchInput {
        color: picker.color,
        color_name: MaybeProp::default(),
        aria_label: MaybeProp::default(),
        aria_labelledby: None,
        id: None,
    });

    // The thumbs hold the focused (hidden) inputs: `within` reports their focus as `data-focus-visible`.
    let area_focus = use_focus_ring(UseFocusRingInput {
        within: true,
        ..UseFocusRingInput::default()
    });
    let hue_focus = use_focus_ring(UseFocusRingInput {
        within: true,
        ..UseFocusRingInput::default()
    });

    let color = picker.color;
    let thumb_fill = move || BackgroundColorProperty.declare(CssColor::from(color.get().to::<RGB8>()));
    let (area_attrs, area_styles) = area.color_area_props.into_parts();
    let (area_thumb_attrs, area_thumb_styles) = area.thumb_props.into_parts();
    let (x_attrs, x_styles) = area.x_input_props.into_parts();
    let (y_attrs, y_styles) = area.y_input_props.into_parts();
    let (hue_track_attrs, hue_track_styles) = hue.slider.track_props.into_parts();
    let (hue_thumb_attrs, hue_thumb_styles) = hue.thumb.thumb_props.into_parts();
    let hue_display = hue_state.display_color();
    let hue_thumb_styles = hue_thumb_styles
        .add_reactive(move || BackgroundColorProperty.declare(CssColor::from(hue_display.get().to_rgb8())));
    let (swatch_attrs, swatch_styles) = swatch.color_swatch_props.into_parts();

    view! {
        <div class="demo-color-picker-hooks">
            <div class="demo-color-picker-hooks-parts">
                <div {..area_attrs} class="demo-color-area" style=area_styles>
                    <div
                        {..area_thumb_attrs}
                        {..area_focus.props.into_attrs()}
                        class="demo-color-area-thumb"
                        style=area_thumb_styles.add_reactive(thumb_fill)
                    >
                        <input {..x_attrs} style=x_styles/>
                        <input {..y_attrs} style=y_styles/>
                    </div>
                </div>
                <div
                    class="demo-color-slider-track"
                    {..hue.slider.group_props.into_attrs()}
                    {..hue_track_attrs}
                    style=hue_track_styles.merge(hue.track_styles)
                >
                    <div class="demo-color-slider-thumb" {..hue_thumb_attrs} {..hue_focus.props.into_attrs()} style=hue_thumb_styles>
                        <input {..hue.thumb.input_props.into_attrs()} style=hue.input_styles/>
                    </div>
                </div>
            </div>
            <div class="demo-color-picker-hooks-result">
                <div {..swatch_attrs} class="demo-color-swatch-hook" style=swatch_styles></div>
                <p class="demo-status">
                    {move || format!("{}, {}", color.get().to::<RGB8>(), color.get().color_name(&locale.get()))}
                </p>
                // Setting the picker's color from code moves every part.
                <Button on_press=move |_| picker.set_color(Color::from(BLUE)) classes="demo-btn">"Reset"</Button>
            </div>
        </div>
    }
}
