// Upstream: react-aria-components/src/ColorThumb.tsx @ 99e6102368
//! The thumb of the color atoms (`ColorArea`, `ColorSlider`'s track, `ColorWheel`).

use leptos::prelude::*;
use leptos_classes::Classes;

use crate::{
    Part, PropsWithStyles,
    hooks::{
        color::{
            UseColorAreaInputProps, UseColorAreaThumbProps, UseColorWheelInputProps,
            UseColorWheelThumbProps,
        },
        focus::{FocusRingTarget, UseFocusRingInput, UseFocusRingReturn, use_focus_ring},
        interactions::{UseHoverInput, use_hover},
        slider::{UseSliderThumbInputProps, UseSliderThumbProps},
    },
    utils::{data_attributes::data_flags, default_class::with_default_class, styles::Styles},
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Render props become `data-*` attributes plus plain children; there is no `color` render
//   prop: the thumb's background is the current color, as upstream's default style.
//
// =============================================================================

/// The parts a [`ColorThumb`] renders, from the color atom around it (its hook's parts).
#[derive(Clone)]
pub(crate) enum ThumbParts {
    /// A color area's thumb, with the inputs of both channels.
    Area {
        thumb: Part<PropsWithStyles<UseColorAreaThumbProps>>,
        x_input: Part<PropsWithStyles<UseColorAreaInputProps>>,
        y_input: Part<PropsWithStyles<UseColorAreaInputProps>>,
    },
    /// A color slider's thumb, with its input.
    Slider {
        thumb: Part<PropsWithStyles<UseSliderThumbProps>>,
        input: Part<PropsWithStyles<UseSliderThumbInputProps>>,
    },
    /// A color wheel's thumb, with its input.
    Wheel {
        thumb: Part<PropsWithStyles<UseColorWheelThumbProps>>,
        input: Part<PropsWithStyles<UseColorWheelInputProps>>,
    },
}

/// What a [`ColorThumb`] gets from the color atom around it (react-aria-components'
/// `InternalColorThumbContext`).
#[derive(Clone)]
pub struct ColorThumbContext {
    /// The current color, as CSS.
    pub color: Signal<String>,
    pub is_dragging: Signal<bool>,
    pub is_disabled: Signal<bool>,
    thumb: ThumbParts,
}

impl ColorThumbContext {
    pub(crate) fn new(
        color: Signal<String>,
        is_dragging: Signal<bool>,
        is_disabled: Signal<bool>,
        thumb: ThumbParts,
    ) -> Self {
        Self {
            color,
            is_dragging,
            is_disabled,
            thumb,
        }
    }
}

/// The thumb of the color atom around it ([`ColorArea`](super::color_area::ColorArea), a
/// `ColorSlider`'s `ColorSliderTrack`, a `ColorWheel`), with its visually hidden range inputs. Its background is
/// the current color.
///
/// Data attributes: `data-dragging`, `data-focused`, `data-focus-visible`, `data-hovered`,
/// `data-disabled`.
///
/// Outside a color atom it renders nothing (with a development warning).
///
/// Default class: `leptonic-ColorThumb`.
#[component]
pub fn ColorThumb(
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    /// Content of the thumb.
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    crate::metrics::subject!();
    let classes = with_default_class("leptonic-ColorThumb", classes);
    let Some(ColorThumbContext {
        color,
        is_dragging,
        is_disabled,
        thumb,
    }) = use_context::<ColorThumbContext>()
    else {
        crate::utils::dev_warn!(
            "A <ColorThumb> must be inside a <ColorArea>, <ColorSlider> or <ColorWheel>."
        );
        return None;
    };

    // Focus is on the hidden inputs inside the thumb.
    let UseFocusRingReturn {
        target: focus_ring,
        is_focused,
        ..
    } = use_focus_ring(UseFocusRingInput {
        target: FocusRingTarget::Within,
        ..UseFocusRingInput::default()
    });
    // Hovered also while disabled (react-aria-components' `useHover(props)`: a thumb has no
    // `isDisabled` prop).
    let hover = use_hover(UseHoverInput::default());
    let thumb_styles = |thumb_styles: Styles| {
        thumb_styles
            // A computed color: no checked grammar in `leptos-css` yet.
            .add_optional_unchecked("background-color", move || Some(color.get()))
            .merge(styles)
    };
    let children = move || children.map(|children| children());

    // The thumb's parts render at one element at a time (a second thumb gets none).
    Some(match thumb {
        ThumbParts::Area {
            thumb,
            x_input,
            y_input,
        } => {
            let (thumb_attrs, area_thumb_styles) = thumb.attrs();
            let (x_attrs, x_styles) = x_input.attrs();
            let (y_attrs, y_styles) = y_input.attrs();
            view! {
            <div
                {..thumb_attrs}
                {..focus_ring.attrs()}
                {..hover.target.attrs()}
                class=classes
                style=thumb_styles(area_thumb_styles)
                {..data_flags([
                    ("data-dragging", is_dragging),
                    ("data-focused", is_focused),
                    ("data-hovered", hover.is_hovered),
                ])}
                {..data_flags([("data-disabled", is_disabled)])}
            >
                <input {..x_attrs} style=x_styles />
                <input {..y_attrs} style=y_styles />
                {children()}
            </div>
            }
            .into_any()
        }
        ThumbParts::Slider { thumb, input } => {
            let (thumb_attrs, slider_thumb_styles) = thumb.attrs();
            let (input_attrs, input_styles) = input.attrs();
            view! {
            <div
                {..thumb_attrs}
                {..focus_ring.attrs()}
                {..hover.target.attrs()}
                class=classes
                style=thumb_styles(slider_thumb_styles)
                {..data_flags([
                    ("data-dragging", is_dragging),
                    ("data-focused", is_focused),
                    ("data-hovered", hover.is_hovered),
                ])}
                {..data_flags([("data-disabled", is_disabled)])}
            >
                <input {..input_attrs} style=input_styles />
                {children()}
            </div>
            }
            .into_any()
        }
        ThumbParts::Wheel { thumb, input } => {
            let (thumb_attrs, wheel_thumb_styles) = thumb.attrs();
            let (input_attrs, input_styles) = input.attrs();
            view! {
            <div
                {..thumb_attrs}
                {..focus_ring.attrs()}
                {..hover.target.attrs()}
                class=classes
                style=thumb_styles(wheel_thumb_styles)
                {..data_flags([
                    ("data-dragging", is_dragging),
                    ("data-focused", is_focused),
                    ("data-hovered", hover.is_hovered),
                ])}
                {..data_flags([("data-disabled", is_disabled)])}
            >
                <input {..input_attrs} style=input_styles />
                {children()}
            </div>
            }
            .into_any()
        }
    })
}
