//! Headless 2D color area atom for selecting two color channels simultaneously.
// Upstream: react-aria-components/src/ColorArea.tsx @ 99e6102368

use leptos::{context::Provider, prelude::*};
use leptos_classes::Classes;

use super::color_picker::ColorPickerContext;
use crate::{
    Out, ValueBinding,
    atoms::color_thumb::{AreaThumbParts, ColorThumbContext, ThumbParts},
    hooks::color::{
        UseColorAreaInput, UseColorAreaStateInput, use_color_area, use_color_area_state,
    },
    utils::{
        color::{Color, ColorValue, RGB8},
        data_attributes::flag,
        default_class::with_default_class,
        styles::Styles,
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - The thumb's background is the current color (react-aria-components: a render prop).
// - The value is split into `value` (a value or any signal) and `set_value` (an `Out`), plus
//   `default_value` and `on_change` (C4).
// - Render props become `data-*` attributes.
//
// ## ADDITIONS
// - `x_channel_step`/`y_channel_step` override the channels' steps (`use_color_area_state`).
//
// =============================================================================

/// A 2D color area: drag its [`ColorThumb`](super::color_thumb::ColorThumb) or press the area to change two channels of a color.
///
/// ```ignore
/// <ColorArea default_value=HSV::default() aria_label="Color">
///     <ColorThumb />
/// </ColorArea>
/// ```
///
/// Data attributes: `data-disabled`.
///
/// Default class: `leptonic-ColorArea`.
#[component]
#[allow(clippy::too_many_arguments)]
pub fn ColorArea<C: ColorValue>(
    /// The initial color. Default: white (`#ffffff`, as react-aria-components).
    #[prop(optional)]
    default_value: Option<C>,
    /// The color (controlled): a value or any signal. Default: the `ColorPicker`'s around it.
    #[prop(into, optional)]
    value: Option<Signal<C>>,
    /// Receives the new color: an `RwSignal`, `WriteSignal`, closure, `Callback`, ...
    #[prop(into, optional)]
    set_value: Option<Out<C>>,
    /// Called with the color whenever it changes, also while dragging.
    #[prop(into, optional)]
    on_change: Option<Callback<C>>,
    /// Called with the color when the user stops dragging.
    #[prop(into, optional)]
    on_change_end: Option<Callback<C>>,
    /// The channel on the horizontal axis. Default: the color space's first axis.
    #[prop(optional)]
    x_channel: Option<C::Channel>,
    /// The channel on the vertical axis. Default: the color space's second axis.
    #[prop(optional)]
    y_channel: Option<C::Channel>,
    /// The horizontal step. Default: the x channel's step.
    #[prop(optional)]
    x_channel_step: Option<f64>,
    /// The vertical step. Default: the y channel's step.
    #[prop(optional)]
    y_channel_step: Option<f64>,
    #[prop(into, optional)] is_disabled: Signal<bool>,
    /// Names the area.
    #[prop(into, optional)]
    aria_label: MaybeProp<String>,
    #[prop(into, optional)] aria_labelledby: Option<String>,
    #[prop(into, optional)] aria_describedby: Option<String>,
    /// Elements with details about the area.
    #[prop(into, optional)]
    aria_details: Option<String>,
    /// The name of the x channel's input, for form submission.
    #[prop(into, optional)]
    x_name: Option<String>,
    /// The name of the y channel's input, for form submission.
    #[prop(into, optional)]
    y_name: Option<String>,
    /// The id of a `<form>` the inputs belong to.
    #[prop(into, optional)]
    form: Option<String>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    /// The `ColorThumb`, and anything else to draw on the area.
    children: Children,
) -> impl IntoView {
    let classes = with_default_class("leptonic-ColorArea", classes);
    let (binding, on_change) = ValueBinding::from_state_props(value, set_value, on_change);
    let binding = binding.or_else(ColorPickerContext::binding::<C>);
    let state = use_color_area_state(UseColorAreaStateInput {
        value: binding,
        x_channel,
        y_channel,
        on_change,
        on_change_end,
        default_value: default_value.unwrap_or_else(|| {
            C::from(Color::from(RGB8 {
                r: 255,
                g: 255,
                b: 255,
            }))
        }),
        x_channel_step,
        y_channel_step,
    });
    let area = use_color_area(UseColorAreaInput {
        state,
        is_disabled,
        aria_label,
        aria_labelledby,
        aria_describedby,
        aria_details,
        x_name,
        y_name,
        form,
    });
    let color = state.display_color();
    let context = ColorThumbContext::new(
        Signal::derive(move || color.get().to_css_string()),
        state.is_dragging,
        is_disabled,
        ThumbParts::Area(Box::new(AreaThumbParts {
            thumb: area.thumb_props,
            x_input: area.x_input_props,
            y_input: area.y_input_props,
        })),
    );
    let (area_attrs, area_styles) = area.color_area_props.into_parts();

    view! {
        <div
            {..area_attrs}
            class=classes
            style=area_styles.merge(styles)
            data-disabled=flag(is_disabled)
        >
            <Provider value=context>{children()}</Provider>
        </div>
    }
}
