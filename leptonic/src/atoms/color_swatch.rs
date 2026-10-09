//! Headless color swatch atom.
// Upstream: react-aria-components/src/ColorSwatch.tsx @ 99e6102368

use leptos::prelude::*;
use leptos_classes::Classes;

use super::{color_picker::ColorPickerContext, color_swatch_picker::ColorSwatchPickerItemContext};
use crate::{
    hooks::color::{UseColorSwatchInput, use_color_swatch},
    utils::{
        color::{Color, ColorProp},
        default_class::with_default_class,
        styles::Styles,
    },
};

/// A swatch showing a color: an image named after the color, with the color as its background.
/// Size it with styles or classes.
///
/// Default class: `leptonic-ColorSwatch`.
#[component]
pub fn ColorSwatch(
    /// The color to show: any color value or signal of one. Default: the color of the
    /// `ColorSwatchPickerItem` around it, else the `ColorPicker`'s.
    #[prop(into, optional)]
    color: Option<ColorProp>,
    /// Replaces the color's name (e.g. "Ocean" instead of "dark vibrant cyan blue").
    #[prop(into, optional)]
    color_name: MaybeProp<String>,
    /// Added to the color's name.
    #[prop(into, optional)]
    aria_label: MaybeProp<String>,
    #[prop(into, optional)] aria_labelledby: Option<String>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let classes = with_default_class("leptonic-ColorSwatch", classes);
    let color = color
        .map(|ColorProp(color)| color)
        .or_else(|| {
            use_context::<ColorSwatchPickerItemContext>()
                .map(|ColorSwatchPickerItemContext(color)| Signal::stored(color))
        })
        .or_else(|| use_context::<ColorPickerContext>().map(|ColorPickerContext(state)| state.color))
        .unwrap_or_else(|| {
            crate::utils::dev_warn!(
                "ColorSwatch: no `color` given (and not inside a ColorSwatchPickerItem or ColorPicker)"
            );
            Signal::stored(Color::default())
        });
    let (attrs, swatch_styles) = use_color_swatch(UseColorSwatchInput {
        color_name,
        aria_label,
        aria_labelledby,
        color,
        id: None,
    })
    .color_swatch_props
    .into_parts();
    view! {
        <div {..attrs} class=classes style=swatch_styles.merge(styles)>
            {children.map(|children| children())}
        </div>
    }
}
