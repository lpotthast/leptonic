// Upstream: react-aria-components/src/ColorWheel.tsx @ 99e6102368
// Upstream: react-aria-components/test/ColorWheel.test.js @ 99e6102368
//! Headless color wheel atoms: a ring of hues with a thumb.

use leptos::{context::Provider, prelude::*};
use leptos_classes::Classes;

use super::{
    color_picker::ColorPickerContext,
    color_thumb::{ColorThumbContext, ThumbParts},
};
use crate::{
    Out, Part, PropsWithStyles, ValueBinding,
    hooks::color::{
        UseColorWheelInput, UseColorWheelStateInput, UseColorWheelTrackProps, use_color_wheel,
        use_color_wheel_state,
    },
    utils::{
        color::{Color, ColorValue, HSL},
        data_attributes::data_flags,
        default_class::with_default_class,
        styles::Styles,
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Generic over the color type: the hue channel of HSV and HSL colors, the hue of the HSL form
//   of others (react-aria-components converts the color to HSL; see `use_color_wheel_state`).
// - The value is split into `value` (a value or any signal) and `set_value` (an `Out`), plus
//   `default_value` and `on_change` (C4).
//
// =============================================================================

/// What a [`ColorWheelTrack`] gets from its [`ColorWheel`].
#[derive(Clone)]
struct ColorWheelTrackContext {
    is_disabled: Signal<bool>,
    /// The track: one element at a time.
    track: Part<PropsWithStyles<UseColorWheelTrackProps>>,
}

/// A color wheel: a [`ColorWheelTrack`] (the ring of hues) and a
/// [`ColorThumb`](super::color_thumb::ColorThumb) on it.
///
/// ```ignore
/// <ColorWheel default_value=HSV::default() outer_radius=100.0 inner_radius=74.0>
///     <ColorWheelTrack />
///     <ColorThumb />
/// </ColorWheel>
/// ```
///
/// Data attributes: `data-disabled`.
///
/// Default class: `leptonic-ColorWheel`.
#[component]
pub fn ColorWheel<C: ColorValue>(
    /// The wheel's outer radius, in pixels (a value or any signal).
    #[prop(into)]
    outer_radius: Signal<f64>,
    /// The wheel's inner radius, in pixels (a value or any signal).
    #[prop(into)]
    inner_radius: Signal<f64>,
    /// The initial color. Default: `hsl(0, 100%, 50%)` (red, as react-aria-components).
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
    #[prop(into, optional)] is_disabled: Signal<bool>,
    /// Names the wheel. Without any label, the hue channel's name does.
    #[prop(into, optional)]
    aria_label: MaybeProp<String>,
    #[prop(into, optional)] aria_labelledby: MaybeProp<String>,
    /// Elements describing the wheel.
    #[prop(into, optional)]
    aria_describedby: MaybeProp<String>,
    /// Elements with details about the wheel.
    #[prop(into, optional)]
    aria_details: MaybeProp<String>,
    /// The element with the wheel's error message.
    #[prop(into, optional)]
    aria_errormessage: MaybeProp<String>,
    /// The name of the input, for form submission.
    #[prop(into, optional)]
    name: Option<String>,
    /// The id of a `<form>` the input belongs to.
    #[prop(into, optional)]
    form: Option<String>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    crate::metrics::subject!();
    let classes = with_default_class("leptonic-ColorWheel", classes);
    let (binding, on_change) = ValueBinding::from_state_props(value, set_value, on_change);
    let binding = binding.or_else(ColorPickerContext::binding::<C>);
    let state = use_color_wheel_state(UseColorWheelStateInput {
        value: binding,
        is_disabled,
        on_change,
        on_change_end,
        default_value: default_value.unwrap_or_else(|| C::from(Color::from(HSL::new()))),
    });
    let wheel = use_color_wheel(UseColorWheelInput {
        state,
        outer_radius,
        inner_radius,
        aria_label,
        aria_labelledby: crate::utils::id::id_ref_prop(aria_labelledby),
        aria_describedby: crate::utils::id::id_ref_prop(aria_describedby),
        aria_details: crate::utils::id::id_ref_prop(aria_details),
        aria_errormessage: crate::utils::id::id_ref_prop(aria_errormessage),
        name,
        form,
    });
    let track = ColorWheelTrackContext {
        is_disabled,
        track: wheel.track,
    };
    let color = state.display_color();
    let thumb = ColorThumbContext::new(
        Signal::derive(move || color.get().to_css_string()),
        state.is_dragging,
        is_disabled,
        ThumbParts::Wheel {
            thumb: wheel.thumb,
            input: wheel.input,
        },
    );
    let styles = Styles::new()
        .add_unchecked("position", "relative")
        .merge(styles);

    view! {
        <div class=classes style={styles} {..data_flags([("data-disabled", is_disabled)])}>
            <Provider value=track>
                <Provider value=thumb>{children()}</Provider>
            </Provider>
        </div>
    }
}

/// The ring of hues of the [`ColorWheel`] around it.
///
/// Data attributes: `data-disabled`.
///
/// Outside a [`ColorWheel`] it renders nothing (with a development warning).
///
/// Default class: `leptonic-ColorWheelTrack`.
#[component]
pub fn ColorWheelTrack(
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
) -> impl IntoView {
    crate::metrics::subject!();
    let classes = with_default_class("leptonic-ColorWheelTrack", classes);
    let Some(ColorWheelTrackContext { is_disabled, track }) = use_context() else {
        crate::utils::dev_warn!("A <ColorWheelTrack> must be inside a <ColorWheel>.");
        return None;
    };
    // One track per wheel (`Part`: a second one renders nothing).
    let (attrs, track_styles) = track.props()?.into_parts();
    Some(view! {
        <div {..attrs} class=classes style=track_styles.merge(styles) {..data_flags([("data-disabled", is_disabled)])} />
    })
}
