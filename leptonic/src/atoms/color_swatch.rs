//! Headless color swatch atom that displays a color with proper accessibility.

use leptos::prelude::*;

use crate::{
    hooks::{IntoAttrs, UseColorSwatchInput, use_color_swatch},
    utils::{
        classes::Classes,
        color::ColorValue,
        styles::{Style::BackgroundColor, Styles},
    },
};

/// A headless color swatch that displays a color with proper accessibility.
///
/// Renders a `<div>` with `role="img"`, `aria-roledescription="color swatch"`,
/// and the color as background. Apply your own sizing via styles/classes.
#[component]
pub fn ColorSwatch<C: ColorValue>(
    /// The color to display.
    #[prop(into)]
    color: Signal<C>,
    /// An optional color name for the aria-label.
    #[prop(into, optional)]
    color_name: Option<Signal<String>>,
    /// An optional aria-label override.
    #[prop(into, optional)]
    aria_label: Option<String>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    /// Optional children to render inside the swatch.
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    let swatch = use_color_swatch(UseColorSwatchInput {
        color,
        color_name,
        aria_label,
    });

    let bg = swatch.background_color;

    let styles = styles
        .add(BackgroundColor, move || bg.get())
        .add("forced-color-adjust", "none");

    view! {
        <div
            {..swatch.props.into_attrs()}
            class=classes
            style=styles
        >
            {children.map(|c| c())}
        </div>
    }
}
