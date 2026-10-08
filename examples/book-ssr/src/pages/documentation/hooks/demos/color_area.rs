use leptonic::{
    atoms::checkbox::{CheckboxButton, CheckboxField},
    hooks::*,
    utils::{
        color::{ColorValue, HSV, HsvChannel},
        css::CssColor,
        i18n::use_locale,
        style::BackgroundColorProperty,
    },
};
use leptos::prelude::*;

#[component]
pub fn ColorAreaDemo() -> impl IntoView {
    let locale = use_locale();
    let disabled = RwSignal::new(false);

    let state = use_color_area_state(UseColorAreaStateInput {
        x_channel: Some(HsvChannel::Saturation),
        y_channel: Some(HsvChannel::Brightness),
        default_value: HSV {
            hue: 210.0,
            saturation: 0.6,
            brightness: 0.8,
        },
        value: None,
        x_channel_step: None,
        y_channel_step: None,
        on_change: None,
        on_change_end: None,
    });
    let area = use_color_area(UseColorAreaInput {
        is_disabled: disabled.into(),
        aria_label: "Saturation and brightness".into(),
        state,
        aria_labelledby: None,
        aria_describedby: None,
        aria_details: None,
        x_name: None,
        y_name: None,
        form: None,
    });

    // The focus is on a hidden input inside the thumb: `within` reports it on the thumb as
    // `data-focus-visible`.
    let focus_ring = use_focus_ring(UseFocusRingInput {
        within: true,
        ..Default::default()
    });

    // The props carry the gradient, the thumb's position and the inputs' hiding as styles.
    let (area_attrs, area_styles) = area.color_area_props.into_parts();
    let (thumb_attrs, thumb_styles) = area.thumb_props.into_parts();
    let (x_attrs, x_styles) = area.x_input_props.into_parts();
    let (y_attrs, y_styles) = area.y_input_props.into_parts();
    let color = state.value;
    let thumb_styles = thumb_styles.add_reactive(move || {
        BackgroundColorProperty.declare(CssColor::from(color.get().to_rgb8()))
    });

    view! {
        <div {..area_attrs} class="demo-color-area" style=area_styles>
            <div {..thumb_attrs} {..focus_ring.props.into_attrs()} class="demo-color-area-thumb" style=thumb_styles>
                <input {..x_attrs} style=x_styles/>
                <input {..y_attrs} style=y_styles/>
            </div>
        </div>

        <p class="demo-status">{move || format!("Color: {}, {}", color.get().to_rgb8(), color.get().color_name(&locale.get()))}</p>
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
