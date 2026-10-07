//! The thumb of the color atoms (`ColorArea`, `ColorSlider`'s track, `ColorWheel`).
// Upstream: react-aria-components/src/ColorThumb.tsx @ 99e6102368

use leptos::prelude::*;

use crate::{
    hooks::{
        IntoAttrs, PropsWithStyles, UseColorAreaInputAttrs, UseColorAreaInputProps,
        UseColorAreaThumbAttrs, UseColorAreaThumbProps, UseColorWheelInputAttrs,
        UseColorWheelInputProps, UseColorWheelThumbAttrs, UseColorWheelThumbProps,
        UseFocusRingInput, UseFocusRingReturn, UseHoverInput, UseSliderThumbAttrs,
        UseSliderThumbInputAttrs, UseSliderThumbInputProps, UseSliderThumbProps, use_focus_ring,
        use_hover,
    },
    utils::{
        classes::Classes, data_attributes::flag, default_class::with_default_class, styles::Styles,
    },
};

/// The props a [`ColorThumb`] takes from the color atom around it (the hooks' props).
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

/// The thumb's attributes, cloned for every render of the thumb (it may mount again, e.g.
/// inside a `<Show>`), as the slider atoms do.
// One per color atom, in a `StoredValue`: the size of the variants doesn't matter.
#[allow(clippy::large_enum_variant)]
#[derive(Clone)]
enum ThumbAttrs {
    Area {
        thumb: (UseColorAreaThumbAttrs, Styles),
        x_input: (UseColorAreaInputAttrs, Styles),
        y_input: (UseColorAreaInputAttrs, Styles),
    },
    Slider {
        thumb: (UseSliderThumbAttrs, Styles),
        input: (UseSliderThumbInputAttrs, Styles),
    },
    Wheel {
        thumb: (UseColorWheelThumbAttrs, Styles),
        input: (UseColorWheelInputAttrs, Styles),
    },
}

impl From<ThumbParts> for ThumbAttrs {
    fn from(parts: ThumbParts) -> Self {
        match parts {
            ThumbParts::Area(area) => Self::Area {
                thumb: area.thumb.into_parts(),
                x_input: area.x_input.into_parts(),
                y_input: area.y_input.into_parts(),
            },
            ThumbParts::Slider(slider) => Self::Slider {
                thumb: slider.thumb.into_parts(),
                input: (slider.input.into_attrs(), slider.input_styles),
            },
            ThumbParts::Wheel(wheel) => Self::Wheel {
                thumb: wheel.thumb.into_parts(),
                input: wheel.input.into_parts(),
            },
        }
    }
}

/// What a [`ColorThumb`] gets from the color atom around it (react-aria-components'
/// `InternalColorThumbContext`).
#[derive(Clone, Copy)]
pub struct ColorThumbContext {
    /// The current color, as CSS.
    pub color: Signal<String>,
    pub is_dragging: Signal<bool>,
    pub is_disabled: Signal<bool>,
    attrs: StoredValue<ThumbAttrs>,
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
            attrs: StoredValue::new(parts.into()),
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
/// Outside a color atom.
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
        attrs,
    } = expect_context::<ColorThumbContext>();

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

    match attrs.get_value() {
        ThumbAttrs::Area {
            thumb: (thumb_attrs, area_thumb_styles),
            x_input: (x_attrs, x_styles),
            y_input: (y_attrs, y_styles),
        } => view! {
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
        .into_any(),
        ThumbAttrs::Slider {
            thumb: (thumb_attrs, slider_thumb_styles),
            input: (input_attrs, input_styles),
        } => view! {
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
                <input {..input_attrs} style=input_styles />
                {children()}
            </div>
        }
        .into_any(),
        ThumbAttrs::Wheel {
            thumb: (thumb_attrs, wheel_thumb_styles),
            input: (input_attrs, input_styles),
        } => view! {
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
        .into_any(),
    }
}
