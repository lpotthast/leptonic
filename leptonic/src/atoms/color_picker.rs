//! Headless color picker atom: one color shared by color atoms.
// Upstream: react-aria-components/src/ColorPicker.tsx @ 99e6102368

use leptos::{context::Provider, prelude::*};

use crate::{
    Out, ValueBinding,
    hooks::color::{ColorPickerState, UseColorPickerStateInput, use_color_picker_state},
    utils::color::{Color, ColorValue},
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - The value is split into `value` (a value or any signal) and `set_value` (an `Out`), plus
//   `default_value` and `on_change` (C4).
// - The color atoms read the picker from the context (`ColorPickerContext`) when they aren't
//   given a `value` (react-aria-components: one props context per component).
//
// =============================================================================

/// The picker's new color: `color`, keeping the picker's alpha when `color` has none (an opaque
/// slider doesn't make the color opaque).
fn merged<C: ColorValue>(state: ColorPickerState, color: C) -> Color {
    let color: Color = color.into();
    if C::HAS_ALPHA {
        color
    } else {
        color.with_alpha(state.color.get_untracked().alpha)
    }
}

/// The [`ColorPicker`] around a color atom.
#[derive(Debug, Clone, Copy)]
pub struct ColorPickerContext(pub ColorPickerState);

impl ColorPickerContext {
    /// The picker's color as `C`, for an atom without its own `value`.
    pub(crate) fn binding<C: ColorValue>() -> Option<ValueBinding<C>> {
        let Self(state) = use_context::<Self>()?;
        Some(ValueBinding::new(
            Signal::derive(move || state.color.get().to::<C>()),
            Callback::new(move |color: C| state.set_color(merged(state, color))),
        ))
    }

    /// The picker's color as `C`, for an atom whose value may be empty: emptying it leaves the
    /// picker's color as is (react-aria).
    pub(crate) fn optional_binding<C: ColorValue>() -> Option<ValueBinding<Option<C>>> {
        let Self(state) = use_context::<Self>()?;
        Some(ValueBinding::new(
            Signal::derive(move || Some(state.color.get().to::<C>())),
            Callback::new(move |color: Option<C>| {
                if let Some(color) = color {
                    state.set_color(merged(state, color));
                }
            }),
        ))
    }
}

/// Shares one color between the color atoms inside (`ColorArea`, `ColorSlider`, `ColorWheel`,
/// `ColorField`, `ColorChannelField`, `ColorSwatch`, `ColorSwatchPicker`): those without their
/// own `value` show and change it, each in its own color space. Renders no element.
///
/// ```ignore
/// <ColorPicker default_value=Color::from(HSV::new())>
///     <ColorArea<HSV> x_channel=HsvChannel::Saturation y_channel=HsvChannel::Brightness>
///         <ColorThumb />
///     </ColorArea<HSV>>
///     <ColorSwatch<RGB8> />
/// </ColorPicker>
/// ```
#[component]
pub fn ColorPicker(
    /// The initial color. Default: black.
    #[prop(optional)]
    default_value: Option<Color>,
    /// The color (controlled): a value or any signal.
    #[prop(into, optional)]
    value: Option<Signal<Color>>,
    /// Receives the color: an `RwSignal`, `WriteSignal`, closure, `Callback`, ...
    #[prop(into, optional)]
    set_value: Option<Out<Color>>,
    /// Called with every new color.
    #[prop(into, optional)]
    on_change: Option<Callback<Color>>,
    children: Children,
) -> impl IntoView {
    let (value, on_change) = ValueBinding::from_state_props(value, set_value, on_change);
    let state = use_color_picker_state(UseColorPickerStateInput {
        default_value: default_value.unwrap_or_default(),
        value,
        on_change,
    });
    view! { <Provider value=ColorPickerContext(state)>{children()}</Provider> }
}
