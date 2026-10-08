// Upstream: react-aria/src/color/useColorWheel.ts @ 99e6102368
use leptos::{
    attr,
    attr::Attr,
    ev,
    ev::{On, SharedEventCallback},
    prelude::*,
    tachys::html::property::{Property, prop},
};
use send_wrapper::SendWrapper;
use wasm_bindgen::JsCast;
use web_sys::{Event, KeyboardEvent, PointerEvent};

use super::use_color_wheel_state::ColorWheelState;
use crate::{
    hooks::{
        IntoAttrs, MoveEndEvent, MoveEvent, MoveStartEvent, PropsWithStyles, UseFormResetInput,
        UseKeyboardInput, UseMoveInput, use_form_reset, use_keyboard, use_move,
    },
    utils::{
        CapturedElement, ElementCaptureAttr, EventAccessors, EventHandler,
        color::{ColorValue, HSL, HslChannel},
        css::{ForcedColorAdjust, TouchAction, computed_size},
        event_listeners::{Listener, listen_to},
        focus::focus_element,
        i18n::use_locale,
        id::use_id,
        keyboard_shortcut::{KeyboardShortcuts, Shortcut},
        number_formatter::NumberFormatter,
        pointer_type::PointerType,
        style::{ForcedColorAdjustProperty, HeightProperty, TouchActionProperty, WidthProperty},
        styles::Styles,
        visually_hidden::visually_hidden_full_size_styles,
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - The track and input elements are captured by the returned props instead of refs passed in.
// - Any color type: the hue is its hue channel, or its HSL form's (see `use_color_wheel_state`).
//
// ## OMITTED FEATURES
// - The mouse and touch fallbacks for browsers without `PointerEvent` (CLAUDE.md).
//
// =============================================================================

/// Input of [`use_color_wheel`].
#[derive(Debug, Clone)]
pub struct UseColorWheelInput<C: ColorValue> {
    pub state: ColorWheelState<C>,
    /// The wheel's outer radius, in pixels.
    pub outer_radius: f64,
    /// The wheel's inner radius, in pixels (the track is the ring between the radii).
    pub inner_radius: f64,
    /// Names the wheel. Without any label, the hue channel's name does.
    pub aria_label: MaybeProp<String>,
    pub aria_labelledby: Option<String>,
    pub aria_describedby: Option<String>,
    pub aria_details: Option<String>,
    /// The name of the input, for form submission.
    pub name: Option<String>,
    /// The id of a `<form>` the input belongs to.
    pub form: Option<String>,
}

/// Return value of [`use_color_wheel`].
#[derive(Debug)]
pub struct UseColorWheelReturn {
    /// For the track (the ring of hues).
    pub track_props: PropsWithStyles<UseColorWheelTrackProps>,
    /// For the thumb on the track.
    pub thumb_props: PropsWithStyles<UseColorWheelThumbProps>,
    /// For the visually hidden range input (inside the thumb).
    pub input_props: PropsWithStyles<UseColorWheelInputProps>,
}

/// Props of the wheel's track.
#[derive(Debug)]
pub struct UseColorWheelTrackProps {
    pub on_pointerdown: EventHandler<PointerEvent>,
    pub element_capture: ElementCaptureAttr,
}

pub type UseColorWheelTrackAttrs = (
    On<ev::pointerdown, SharedEventCallback<PointerEvent>>,
    ElementCaptureAttr,
);

impl IntoAttrs for UseColorWheelTrackProps {
    type Attrs = UseColorWheelTrackAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            self.on_pointerdown.into_on(ev::pointerdown),
            self.element_capture,
        )
    }
}

/// Props of the wheel's thumb.
#[derive(Debug)]
pub struct UseColorWheelThumbProps {
    pub on_pointerdown: EventHandler<PointerEvent>,
    pub on_keydown: EventHandler<KeyboardEvent>,
    pub on_keyup: EventHandler<KeyboardEvent>,
    pub element_capture: ElementCaptureAttr,
}

pub type UseColorWheelThumbAttrs = (
    On<ev::pointerdown, SharedEventCallback<PointerEvent>>,
    On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
    On<ev::keyup, SharedEventCallback<KeyboardEvent>>,
    ElementCaptureAttr,
);

impl IntoAttrs for UseColorWheelThumbProps {
    type Attrs = UseColorWheelThumbAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            self.on_pointerdown.into_on(ev::pointerdown),
            self.on_keydown.into_on(ev::keydown),
            self.on_keyup.into_on(ev::keyup),
            self.element_capture,
        )
    }
}

/// Props of the wheel's visually hidden range input.
#[derive(Debug)]
pub struct UseColorWheelInputProps {
    pub id: String,
    pub min: f64,
    pub max: f64,
    pub step: f64,
    pub value: Signal<f64>,
    pub is_disabled: Signal<bool>,
    pub name: Option<String>,
    pub form: Option<String>,
    pub aria_label: Signal<Option<String>>,
    pub aria_labelledby: Signal<Option<String>>,
    pub aria_describedby: Option<String>,
    pub aria_details: Option<String>,
    pub aria_valuetext: Signal<String>,
    /// Sets the channel from the input's value (e.g. by assistive technology).
    pub on_input: EventHandler<Event>,
    pub element_capture: ElementCaptureAttr,
}

pub type UseColorWheelInputAttrs = (
    (
        Attr<attr::Id, String>,
        Attr<attr::Type, &'static str>,
        Attr<attr::Min, f64>,
        Attr<attr::Max, f64>,
        Attr<attr::Step, f64>,
        Attr<attr::Value, Signal<f64>>,
        Property<&'static str, Signal<f64>>,
        Attr<attr::Disabled, Signal<bool>>,
        Attr<attr::Name, Option<String>>,
        Attr<attr::Form, Option<String>>,
    ),
    (
        Attr<attr::AriaLabel, Signal<Option<String>>>,
        Attr<attr::AriaLabelledby, Signal<Option<String>>>,
        Attr<attr::AriaDescribedby, Option<String>>,
        Attr<attr::AriaDetails, Option<String>>,
        Attr<attr::AriaValuetext, Signal<String>>,
        On<ev::input, SharedEventCallback<Event>>,
        ElementCaptureAttr,
    ),
);

impl IntoAttrs for UseColorWheelInputProps {
    type Attrs = UseColorWheelInputAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            (
                Attr(attr::Id, self.id),
                Attr(attr::Type, "range"),
                Attr(attr::Min, self.min),
                Attr(attr::Max, self.max),
                Attr(attr::Step, self.step),
                Attr(attr::Value, self.value),
                // The attribute is the initial value only: once changed (e.g. by assistive
                // technology), the input follows its property.
                prop("value", self.value),
                Attr(attr::Disabled, self.is_disabled),
                Attr(attr::Name, self.name),
                Attr(attr::Form, self.form),
            ),
            (
                Attr(attr::AriaLabel, self.aria_label),
                Attr(attr::AriaLabelledby, self.aria_labelledby),
                Attr(attr::AriaDescribedby, self.aria_describedby),
                Attr(attr::AriaDetails, self.aria_details),
                Attr(attr::AriaValuetext, self.aria_valuetext),
                self.on_input.into_on(ev::input),
                self.element_capture,
            ),
        )
    }
}

/// The SVG path of a circle around (`cx`, `cy`).
fn circle_path(cx: f64, cy: f64, r: f64) -> String {
    format!(
        "M {cx}, {cy} m {}, 0 a {r}, {r}, 0, 1, 0, {}, 0 a {r}, {r}, 0, 1, 0 {}, 0",
        -r,
        r * 2.0,
        -r * 2.0
    )
}

/// The pointer interaction under way: the pointer and whether it started on the track.
#[derive(Debug, Clone, Copy)]
struct Press {
    pointer_id: i32,
    on_track: bool,
}

/// Behavior and accessibility of a color wheel: a ring of hues with a thumb, dragged or pressed
/// on the ring, arrow keys (with Shift: page steps), PageUp/PageDown, a visually hidden range
/// input for assistive technology and forms.
#[allow(clippy::too_many_lines)]
pub fn use_color_wheel<C: ColorValue>(input: UseColorWheelInput<C>) -> UseColorWheelReturn {
    let UseColorWheelInput {
        state,
        outer_radius,
        inner_radius,
        aria_label,
        aria_labelledby,
        aria_describedby,
        aria_details,
        name,
        form,
    } = input;
    let is_disabled = state.is_disabled;
    let thumb_radius = f64::midpoint(inner_radius, outer_radius);
    let track_element = CapturedElement::new();
    let thumb_element = CapturedElement::new();
    let input_element = CapturedElement::new();
    let focus_input = move || {
        if let Some(input) = input_element.get_untracked() {
            focus_element(&input, true);
        }
    };

    use_form_reset(UseFormResetInput {
        element: input_element,
        initial_value: state.default_value(),
        on_reset: Callback::new(move |color: C| state.set_value(color)),
    });

    let keyboard = use_keyboard(UseKeyboardInput {
        is_disabled,
        shortcuts: Some(
            KeyboardShortcuts::new()
                .on(Shortcut::key("PageUp"), move |_| {
                    state.set_dragging(true);
                    state.increment(state.page_step);
                    state.set_dragging(false);
                })
                .on(Shortcut::key("PageDown"), move |_| {
                    state.set_dragging(true);
                    state.decrement(state.page_step);
                    state.set_dragging(false);
                }),
        ),
        allow_repeats: true,
        ..UseKeyboardInput::default()
    })
    .props;

    let press = StoredValue::new(None::<Press>);
    let current_position = StoredValue::new(None::<(f64, f64)>);
    let on_move_start = Callback::new(move |_: MoveStartEvent| {
        current_position.set_value(None);
        state.set_dragging(true);
    });
    let on_move = Callback::new(move |e: MoveEvent| {
        let (mut x, mut y) = current_position
            .get_value()
            .unwrap_or_else(|| untrack(|| state.thumb_position(thumb_radius)));
        x += e.delta_x;
        y += e.delta_y;
        current_position.set_value(Some((x, y)));
        if e.pointer_type == PointerType::Keyboard {
            let step = if e.modifiers.shift_key {
                state.page_step
            } else {
                state.step
            };
            if e.delta_x > 0.0 || e.delta_y < 0.0 {
                state.increment(step);
            } else if e.delta_x < 0.0 || e.delta_y > 0.0 {
                state.decrement(step);
            }
        } else {
            state.set_hue_from_point(x, y, thumb_radius);
        }
    });
    let on_move_end = Callback::new(move |_: MoveEndEvent| {
        press.update_value(|press| {
            if let Some(press) = press {
                press.on_track = false;
            }
        });
        state.set_dragging(false);
        focus_input();
    });
    let thumb_move = use_move(UseMoveInput {
        is_disabled,
        on_move_start: Some(on_move_start),
        on_move: Some(on_move),
        on_move_end: Some(on_move_end),
    })
    .props;
    // The track forwards its moves only while a press started on it.
    let on_track = move || press.get_value().is_some_and(|press| press.on_track);
    let track_move = use_move(UseMoveInput {
        is_disabled,
        on_move_start: Some(Callback::new(move |e| {
            if on_track() {
                on_move_start.run(e);
            }
        })),
        on_move: Some(Callback::new(move |e| {
            if on_track() {
                on_move.run(e);
            }
        })),
        on_move_end: Some(Callback::new(move |e| {
            if on_track() {
                on_move_end.run(e);
            }
        })),
    })
    .props;

    let pointer_up_listener = StoredValue::new(None::<SendWrapper<Listener>>);
    let release = move |pointer_id: i32| {
        let Some(current) = press.get_value() else {
            return;
        };
        if current.pointer_id != pointer_id {
            return;
        }
        press.set_value(None);
        pointer_up_listener.set_value(None);
        state.set_dragging(false);
        focus_input();
    };
    let listen_for_release = move || {
        if let Some(window) = leptos_use::use_window().as_ref() {
            let listener = listen_to(window, ev::pointerup, false, move |e: PointerEvent| {
                release(e.pointer_id());
            });
            pointer_up_listener.set_value(Some(SendWrapper::new(listener)));
        }
    };
    let ignored = |e: &PointerEvent| {
        PointerType::from(e.pointer_type()) == PointerType::Mouse
            && (e.button() != 0 || e.alt_key() || e.ctrl_key() || e.meta_key())
    };
    let on_thumb_down = EventHandler::new(move |e: PointerEvent| {
        if is_disabled.get_untracked() || ignored(&e) || state.is_dragging.get_untracked() {
            return;
        }
        press.set_value(Some(Press {
            pointer_id: e.pointer_id(),
            on_track: false,
        }));
        focus_input();
        state.set_dragging(true);
        listen_for_release();
    });
    let on_track_down = EventHandler::new(move |e: PointerEvent| {
        if is_disabled.get_untracked() || ignored(&e) {
            return;
        }
        let Some(track) = e
            .expect_current_target()
            .dyn_into::<web_sys::Element>()
            .ok()
        else {
            return;
        };
        let rect = track.get_bounding_client_rect();
        let x = e.client_x() - rect.x() - rect.width() / 2.0;
        let y = e.client_y() - rect.y() - rect.height() / 2.0;
        let radius = x.hypot(y);
        if inner_radius < radius
            && radius < outer_radius
            && !state.is_dragging.get_untracked()
            && press.get_value().is_none()
        {
            press.set_value(Some(Press {
                pointer_id: e.pointer_id(),
                on_track: true,
            }));
            state.set_hue_from_point(x, y, radius);
            focus_input();
            state.set_dragging(true);
            listen_for_release();
        }
    });

    // Without any label, the channel names the wheel (react-aria).
    let has_labelledby = aria_labelledby.is_some();
    let locale = use_locale();
    let input_label = Signal::derive(move || {
        aria_label.get().or_else(|| {
            (!has_labelledby).then(|| HSL::channel_name(HslChannel::Hue, &locale.get()))
        })
    });
    let input_id = use_id("color-wheel");
    // With other labels next to its `aria-label`, the input names itself too (`use_label`).
    let input_labelledby = {
        let input_id = input_id.clone();
        Signal::derive(move || {
            let ids = aria_labelledby.clone()?;
            Some(if input_label.with(Option::is_some) {
                format!("{input_id} {ids}")
            } else {
                ids
            })
        })
    };
    let value = state.value;
    let hue = state.hue;
    let range = HSL::channel_range(HslChannel::Hue);

    let size = computed_size(crate::utils::css::computed_px(outer_radius * 2.0));
    let hue_stops = (0..=12)
        .map(|i| format!("hsl({}, 100%, 50%)", i * 30))
        .collect::<Vec<_>>()
        .join(", ");
    let track_styles = Styles::new()
        .add_unchecked("position", "relative")
        .add(TouchActionProperty.declare(TouchAction::None))
        .add(WidthProperty.declare(size.clone()))
        .add(HeightProperty.declare(size))
        // Gradients and paths: no checked grammar in `leptos-css` yet.
        .add_unchecked(
            "background",
            format!("conic-gradient(from 90deg, {hue_stops})"),
        )
        .add_unchecked(
            "clip-path",
            format!(
                "path(evenodd, \"{} {}\")",
                circle_path(outer_radius, outer_radius, outer_radius),
                circle_path(outer_radius, outer_radius, inner_radius)
            ),
        )
        .add(ForcedColorAdjustProperty.declare(ForcedColorAdjust::None));
    let thumb_styles = Styles::new()
        .add_unchecked("position", "absolute")
        .add_optional_unchecked("left", move || {
            Some(format!(
                "{:.3}px",
                outer_radius + state.thumb_position(thumb_radius).0
            ))
        })
        .add_optional_unchecked("top", move || {
            Some(format!(
                "{:.3}px",
                outer_radius + state.thumb_position(thumb_radius).1
            ))
        })
        .add_unchecked("transform", "translate(-50%, -50%)")
        .add(TouchActionProperty.declare(TouchAction::None))
        .add(ForcedColorAdjustProperty.declare(ForcedColorAdjust::None));
    let input_styles = visually_hidden_full_size_styles();

    UseColorWheelReturn {
        track_props: PropsWithStyles::new(
            UseColorWheelTrackProps {
                on_pointerdown: on_track_down.chain(track_move.on_pointerdown),
                element_capture: track_element.attr().chain(track_move.element_capture),
            },
            track_styles,
        ),
        thumb_props: PropsWithStyles::new(
            UseColorWheelThumbProps {
                on_pointerdown: on_thumb_down.chain(thumb_move.on_pointerdown),
                on_keydown: keyboard.on_keydown.chain(thumb_move.on_keydown),
                on_keyup: keyboard.on_keyup,
                element_capture: thumb_element.attr().chain(thumb_move.element_capture),
            },
            thumb_styles,
        ),
        input_props: PropsWithStyles::new(
            UseColorWheelInputProps {
                id: input_id,
                min: range.min_value,
                max: range.max_value,
                step: range.step,
                value: state.hue,
                is_disabled,
                name,
                form,
                aria_label: input_label,
                aria_labelledby: input_labelledby,
                aria_describedby,
                aria_details,
                aria_valuetext: Signal::derive(move || {
                    // The hue formatted as react-aria's (the color's HSL hue), and its name.
                    let degrees = NumberFormatter::new(
                        &locale.get(),
                        HSL::channel_format_options(HslChannel::Hue),
                    )
                    .format(hue.get());
                    format!("{degrees}, {}", value.get().hue_name(&locale.get()))
                }),
                on_input: EventHandler::new(move |e: Event| {
                    if let Some(target) = e
                        .expect_target()
                        .dyn_into::<web_sys::HtmlInputElement>()
                        .ok()
                        && let Ok(hue) = target.value().parse::<f64>()
                    {
                        state.set_hue(hue);
                    }
                }),
                element_capture: input_element.attr(),
            },
            input_styles,
        ),
    }
}
