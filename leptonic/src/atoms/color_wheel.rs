//! Headless color wheel atoms: a ring of hues with a thumb.
// Upstream: react-aria-components/src/ColorWheel.tsx @ 99e6102368

use leptos::{context::Provider, prelude::*};

use super::color_picker::ColorPickerContext;
use super::color_thumb::{ColorThumbContext, ThumbParts, WheelThumbParts};
use crate::{
    Out,
    hooks::{
        UseColorWheelInput, UseColorWheelStateInput, UseColorWheelTrackAttrs, use_color_wheel,
        use_color_wheel_state,
    },
    utils::{
        ValueBinding,
        classes::Classes,
        color::{ColorChannel, ColorValue},
        data_attributes::flag,
        styles::Styles,
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Generic over the color type; `channel` names its hue channel (react-aria-components converts
//   the color to HSL).
// - The value is split into `value` (a value or any signal) and `set_value` (an `Out`), plus
//   `default_value` and `on_change` (C4).
//
// =============================================================================

/// What a [`ColorWheelTrack`] gets from its [`ColorWheel`].
#[derive(Clone, Copy)]
struct ColorWheelTrackContext {
    is_disabled: Signal<bool>,
    track: StoredValue<Option<(UseColorWheelTrackAttrs, Styles)>>,
}

/// A color wheel: a [`ColorWheelTrack`] (the ring of hues) and a
/// [`ColorThumb`](super::color_thumb::ColorThumb) on it.
///
/// ```ignore
/// <ColorWheel default_value=HSV::default() channel=HsvChannel::Hue outer_radius=100.0 inner_radius=74.0>
///     <ColorWheelTrack />
///     <ColorThumb />
/// </ColorWheel>
/// ```
///
/// Data attributes: `data-disabled`.
#[component]
#[allow(clippy::too_many_arguments)]
pub fn ColorWheel<Ch: ColorChannel<Color: Default>>(
    /// The color type's hue channel.
    channel: Ch,
    /// The wheel's outer radius, in pixels.
    outer_radius: f64,
    /// The wheel's inner radius, in pixels.
    inner_radius: f64,
    /// The initial color. Default: the color type's default.
    #[prop(optional)]
    default_value: Option<Ch::Color>,
    /// The color (controlled): a value or any signal. Default: the `ColorPicker`'s around it.
    #[prop(into, optional)]
    value: Option<Signal<Ch::Color>>,
    /// Receives the new color: an `RwSignal`, `WriteSignal`, closure, `Callback`, ...
    #[prop(into, optional)]
    set_value: Option<Out<Ch::Color>>,
    /// Called with the color whenever it changes, also while dragging.
    #[prop(into, optional)]
    on_change: Option<Callback<Ch::Color>>,
    /// Called with the color when the user stops dragging.
    #[prop(into, optional)]
    on_change_end: Option<Callback<Ch::Color>>,
    #[prop(into, optional)] is_disabled: Signal<bool>,
    /// Names the wheel. Without any label, the hue channel's name does.
    #[prop(into, optional)]
    aria_label: MaybeProp<String>,
    #[prop(into, optional)] aria_labelledby: Option<String>,
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
    let (binding, on_change) = ValueBinding::from_state_props(value, set_value, on_change);
    let binding = binding.or_else(ColorPickerContext::binding::<Ch::Color>);
    let state = use_color_wheel_state(UseColorWheelStateInput {
        value: binding,
        is_disabled,
        on_change,
        on_change_end,
        ..UseColorWheelStateInput::new(default_value.unwrap_or_default(), channel)
    });
    let wheel = use_color_wheel(UseColorWheelInput {
        aria_label,
        aria_labelledby,
        name,
        form,
        ..UseColorWheelInput::new(state, outer_radius, inner_radius)
    });
    let (track_attrs, track_styles) = wheel.track_props.into_parts();
    let track = ColorWheelTrackContext {
        is_disabled,
        track: StoredValue::new(Some((track_attrs, track_styles))),
    };
    let color = state.display_color();
    let thumb = ColorThumbContext::new(
        Signal::derive(move || color.get().to_css_string()),
        state.is_dragging,
        is_disabled,
        ThumbParts::Wheel(Box::new(WheelThumbParts {
            thumb: wheel.thumb_props,
            input: wheel.input_props,
        })),
    );
    let styles = Styles::new()
        .add_unchecked("position", "relative")
        .merge(styles);

    view! {
        <div class=classes style=styles data-disabled=flag(is_disabled)>
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
/// # Panics
///
/// Outside a [`ColorWheel`], or as its second track.
#[component]
pub fn ColorWheelTrack(
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
) -> impl IntoView {
    let ColorWheelTrackContext { is_disabled, track } = expect_context::<ColorWheelTrackContext>();
    let (attrs, track_styles) = track
        .try_update_value(Option::take)
        .flatten()
        .expect("a `ColorWheel` has one `ColorWheelTrack`");
    view! {
        <div {..attrs} class=classes style=track_styles.merge(styles) data-disabled=flag(is_disabled) />
    }
}
