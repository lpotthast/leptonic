//! Headless 2D color area atom for selecting two color channels simultaneously.

use leptos::prelude::*;

use crate::{
    hooks::{
        IntoAttrs, UseColorAreaInput, UseColorAreaStateInput, use_color_area, use_color_area_state,
    },
    utils::{
        classes::Classes,
        color::ColorValue,
        css::pct,
        styles::{
            Background, BackgroundBlendMode, BackgroundColor, Bottom, Height, Left, Opacity,
            PointerEvents, Position, Styles, TouchAction, Transform, UserSelect, Width,
        },
    },
};

/// A headless 2D color area component built on `use_color_area_state` + `use_color_area`.
///
/// Renders a container div with a gradient background and a draggable thumb.
/// Users select two color channels simultaneously by dragging within the area.
///
/// Two visually hidden `<input type="range">` elements inside the thumb provide
/// screen reader semantics and form submission support.
///
/// Apply your own sizing, border, and styling via attrs/classes.
#[component]
pub fn ColorArea<C: ColorValue>(
    /// The initial color value.
    #[prop(into)]
    default_value: C,
    /// Which channel maps to the X axis.
    x_channel: C::Channel,
    /// Which channel maps to the Y axis.
    y_channel: C::Channel,
    /// Whether the area is disabled.
    #[prop(into, optional)]
    disabled: Signal<bool>,
    /// Callback fired when the color changes.
    #[prop(optional)]
    on_change: Option<Callback<C>>,
    /// Callback fired when interaction ends.
    #[prop(optional)]
    on_change_end: Option<Callback<C>>,
    /// Accessibility label.
    #[prop(into, optional)]
    aria_label: Option<&'static str>,
    /// Whether to use RTL layout.
    #[prop(optional)]
    is_rtl: bool,
    /// HTML `name` attribute for the hidden X-axis range input.
    #[prop(optional)]
    x_name: Option<&'static str>,
    /// HTML `name` attribute for the hidden Y-axis range input.
    #[prop(optional)]
    y_name: Option<&'static str>,
    /// HTML `form` attribute for form association.
    #[prop(optional)]
    form: Option<&'static str>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    /// Optional custom thumb content.
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    let state = use_color_area_state(UseColorAreaStateInput {
        default_value,
        x_channel,
        y_channel,
        x_channel_step: None,
        y_channel_step: None,
        on_change,
        on_change_end,
    });

    let area = use_color_area(UseColorAreaInput {
        state: state.clone(),
        disabled,
        aria_label,
        is_rtl,
        x_name,
        y_name,
        form,
    });

    let bg = area.background;
    let thumb_color = area.thumb_color;
    let thumb_x = area.thumb_x_percent;
    let thumb_y = area.thumb_y_percent;

    let styles = styles
        .add(Background, move || bg.get())
        .add(BackgroundBlendMode, move || {
            area.background_blend_mode.get().unwrap_or("normal")
        })
        .add(Position, "relative")
        .add(TouchAction, "none")
        .add(UserSelect, "none")
        .add("forced-color-adjust", "none");

    let thumb_styles = Styles::new()
        .add(Position, "absolute")
        .add(Left, move || pct(thumb_x.get()))
        .add(Bottom, move || pct(thumb_y.get()))
        .add(Transform, "translate(-50%, 50%)")
        .add(BackgroundColor, move || thumb_color.get());

    let hidden_input_styles = Styles::from([
        (Opacity, "0.0001"),
        (Width, "100%"),
        (Height, "100%"),
        (PointerEvents, "none"),
        (Position, "absolute"),
    ]);

    view! {
        <div
            {..area.area_props.into_attrs()}
            class=classes
            style=styles
        >
            <div
                {..area.thumb_props.into_attrs()}
                style=thumb_styles
            >
                <input
                    {..area.x_input_props.into_attrs()}
                    style=hidden_input_styles.clone()
                />
                <input
                    {..area.y_input_props.into_attrs()}
                    style=hidden_input_styles
                />
                {children.map(|c| c())}
            </div>
        </div>
    }
}
