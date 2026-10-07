//! The thumb of the color atoms (`ColorArea`, `ColorSlider`'s track, `ColorWheel`).
// Upstream: react-aria-components/src/ColorThumb.tsx @ 99e6102368

use leptos::prelude::*;

use crate::{
    hooks::{
        IntoAttrs, PropsWithStyles, UseColorAreaInputProps, UseColorAreaThumbProps,
        UseColorWheelInputProps, UseColorWheelThumbProps, UseFocusRingInput, UseFocusRingReturn,
        UseHoverInput, UseSliderThumbInputProps, UseSliderThumbProps, use_focus_ring, use_hover,
    },
    utils::{
        classes::Classes, data_attributes::flag, default_class::with_default_class, styles::Styles,
    },
};

/// The props a [`ColorThumb`] takes from the color atom around it, once.
pub(crate) enum ThumbParts {
    /// A color area's thumb, with the inputs of both channels.
    Area(Box<AreaThumbParts>),
    /// A color slider's thumb, with its input.
    Slider(Box<SliderThumbParts>),
    /// A color wheel's thumb, with its input.
    Wheel(Box<WheelThumbParts>),
}

/// A color area's thumb and inputs.
pub(crate) struct AreaThumbParts {
    pub thumb: PropsWithStyles<UseColorAreaThumbProps>,
    pub x_input: PropsWithStyles<UseColorAreaInputProps>,
    pub y_input: PropsWithStyles<UseColorAreaInputProps>,
}

/// A color slider's thumb and input.
pub(crate) struct SliderThumbParts {
    pub thumb: PropsWithStyles<UseSliderThumbProps>,
    pub input: UseSliderThumbInputProps,
    pub input_styles: Styles,
}

/// A color wheel's thumb and input.
pub(crate) struct WheelThumbParts {
    pub thumb: PropsWithStyles<UseColorWheelThumbProps>,
    pub input: PropsWithStyles<UseColorWheelInputProps>,
}

/// What a [`ColorThumb`] gets from the color atom around it (react-aria-components'
/// `InternalColorThumbContext`).
#[derive(Clone, Copy)]
pub struct ColorThumbContext {
    /// The current color, as CSS.
    pub color: Signal<String>,
    pub is_dragging: Signal<bool>,
    pub is_disabled: Signal<bool>,
    parts: StoredValue<Option<ThumbParts>>,
}

impl ColorThumbContext {
    pub(crate) fn new(
        color: Signal<String>,
        is_dragging: Signal<bool>,
        is_disabled: Signal<bool>,
        parts: ThumbParts,
    ) -> Self {
        Self {
            color,
            is_dragging,
            is_disabled,
            parts: StoredValue::new(Some(parts)),
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
/// # Panics
///
/// Outside a color atom, or as its second thumb.
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
    let classes = with_default_class("leptonic-ColorThumb", classes);
    let ColorThumbContext {
        color,
        is_dragging,
        is_disabled,
        parts,
    } = expect_context::<ColorThumbContext>();
    let parts = parts
        .try_update_value(Option::take)
        .flatten()
        .expect("a color atom has one `ColorThumb`");

    // Focus is on the hidden inputs inside the thumb.
    let UseFocusRingReturn {
        props: focus_ring,
        is_focused,
        is_focus_visible,
    } = use_focus_ring(UseFocusRingInput {
        within: true,
        ..UseFocusRingInput::default()
    });
    let hover = use_hover(UseHoverInput {
        is_disabled,
        ..UseHoverInput::default()
    });
    let thumb_styles = |thumb_styles: Styles| {
        thumb_styles
            // A computed color: no checked grammar in `leptos-css` yet.
            .add_optional_unchecked("background-color", move || Some(color.get()))
            .merge(styles)
    };
    let children = move || children.map(|children| children());

    match parts {
        ThumbParts::Area(area) => {
            let AreaThumbParts {
                thumb,
                x_input,
                y_input,
            } = *area;
            let (thumb_attrs, area_thumb_styles) = thumb.into_parts();
            let (x_attrs, x_styles) = x_input.into_parts();
            let (y_attrs, y_styles) = y_input.into_parts();
            view! {
                <div
                    {..thumb_attrs}
                    {..focus_ring.into_attrs()}
                    {..hover.props.into_attrs()}
                    class=classes
                    style=thumb_styles(area_thumb_styles)
                    data-dragging=flag(is_dragging)
                    data-focused=flag(is_focused)
                    data-focus-visible=flag(is_focus_visible)
                    data-hovered=flag(hover.is_hovered)
                    data-disabled=flag(is_disabled)
                >
                    <input {..x_attrs} style=x_styles />
                    <input {..y_attrs} style=y_styles />
                    {children()}
                </div>
            }
            .into_any()
        }
        ThumbParts::Slider(slider) => {
            let SliderThumbParts {
                thumb,
                input,
                input_styles,
            } = *slider;
            let (thumb_attrs, slider_thumb_styles) = thumb.into_parts();
            view! {
                <div
                    {..thumb_attrs}
                    {..focus_ring.into_attrs()}
                    {..hover.props.into_attrs()}
                    class=classes
                    style=thumb_styles(slider_thumb_styles)
                    data-dragging=flag(is_dragging)
                    data-focused=flag(is_focused)
                    data-focus-visible=flag(is_focus_visible)
                    data-hovered=flag(hover.is_hovered)
                    data-disabled=flag(is_disabled)
                >
                    <input {..input.into_attrs()} style=input_styles />
                    {children()}
                </div>
            }
            .into_any()
        }
        ThumbParts::Wheel(wheel) => {
            let WheelThumbParts { thumb, input } = *wheel;
            let (thumb_attrs, wheel_thumb_styles) = thumb.into_parts();
            let (input_attrs, input_styles) = input.into_parts();
            view! {
                <div
                    {..thumb_attrs}
                    {..focus_ring.into_attrs()}
                    {..hover.props.into_attrs()}
                    class=classes
                    style=thumb_styles(wheel_thumb_styles)
                    data-dragging=flag(is_dragging)
                    data-focused=flag(is_focused)
                    data-focus-visible=flag(is_focus_visible)
                    data-hovered=flag(hover.is_hovered)
                    data-disabled=flag(is_disabled)
                >
                    <input {..input_attrs} style=input_styles />
                    {children()}
                </div>
            }
            .into_any()
        }
    }
}
