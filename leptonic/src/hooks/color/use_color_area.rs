// Upstream: react-aria/src/color/useColorArea.ts @ 99e6102368
// Upstream: react-aria/src/color/useColorAreaGradient.ts @ 99e6102368
// Upstream: react-aria-components/test/ColorArea.test.js @ 99e6102368
// Upstream: @adobe/react-spectrum/test/color/ColorArea.test.tsx @ 99e6102368
use leptos::{
    ev,
    prelude::*,
    tachys::html::property::{Property, prop},
};
use send_wrapper::SendWrapper;
use wasm_bindgen::JsCast;
use web_sys::{Event, FocusEvent, KeyboardEvent, PointerEvent};

use super::use_color_area_state::ColorAreaState;
use crate::{
    CapturedElement, ElementCaptureAttr, EventHandler, Id, IntoAttrs, OnEvent, PropsWithStyles,
    hooks::{
        focus::{
            FocusWithinEvent, UseFocusInput, UseFocusWithinInput, use_focus, use_focus_within,
        },
        form::{UseFormResetInput, use_form_reset},
        interactions::{
            MoveEndEvent, MoveEvent, MoveStartEvent, UseKeyboardInput, UseMoveInput, use_keyboard,
            use_move,
        },
    },
    utils::{
        aria::{AriaHidden, AriaOrientation, AriaRole},
        color::{BlendMode, ColorValue},
        dom_ext::EventAccessors,
        event_listeners::{Listener, listen_to},
        focus::focus_element,
        i18n::{WritingDirection, use_direction, use_locale},
        id::use_id,
        intl_strings::{
            ColorInputLabelArgs, ColorNameAndValueArgs, ColorStrings, use_localized_strings,
        },
        key::KeyboardKey,
        keyboard_shortcut::{KeyboardShortcuts, Shortcut},
        part::{Part, Remount},
        platform::{
            device::{is_android, is_ios},
            use_platform_check,
        },
        point::Point,
        pointer_type::PointerType,
        signal_attr::{FixedAttr, SignalAttr, fixed_attr, signal_attr},
        styles::{
            Styles,
            css::{ForcedColorAdjust, LengthPercentageAuto, TouchAction, computed_pct},
            property::{ForcedColorAdjustProperty, LeftProperty, TopProperty, TouchActionProperty},
        },
        visually_hidden::visually_hidden_full_size_styles,
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - The elements (area, inputs) are captured by the returned props instead of refs passed in.
// - The gradient styles (`useColorAreaGradient`) are part of the returned props' styles; the
//   gradient comes from the color type (`ColorValue::area_gradient`).
//
// ## OMITTED FEATURES
// - The mouse and touch fallbacks for browsers without `PointerEvent` (CLAUDE.md).
//
// =============================================================================

/// Input of [`use_color_area`].
#[derive(Debug, Clone)]
pub struct UseColorAreaInput<C: ColorValue> {
    pub state: ColorAreaState<C>,
    pub is_disabled: Signal<bool>,
    /// Names the area (its inputs get "{label}, Color picker").
    pub aria_label: MaybeProp<String>,
    pub aria_labelledby: Signal<Option<Id>>,
    pub aria_describedby: Signal<Option<Id>>,
    pub aria_details: Signal<Option<Id>>,
    /// The name of the x channel's input, for form submission.
    pub x_name: Option<String>,
    /// The name of the y channel's input, for form submission.
    pub y_name: Option<String>,
    /// The id of a `<form>` the inputs belong to.
    pub form: Option<String>,
}

/// The return value of [`use_color_area`].
#[derive(Debug)]
pub struct UseColorAreaReturn {
    /// The color area element: `let (attrs, styles) = area.attrs();`.
    pub area: Part<PropsWithStyles<UseColorAreaProps>>,
    /// The thumb element.
    pub thumb: Part<PropsWithStyles<UseColorAreaThumbProps>>,
    /// The (visually hidden) range input of the horizontal channel.
    pub x_input: Part<PropsWithStyles<UseColorAreaInputProps>>,
    /// The (visually hidden) range input of the vertical channel.
    pub y_input: Part<PropsWithStyles<UseColorAreaInputProps>>,
}

/// Props of the color area's element.
#[derive(Debug)]
pub struct UseColorAreaProps {
    pub id: Id,
    pub aria_label: Signal<Option<String>>,
    pub aria_labelledby: Signal<Option<Id>>,
    pub on_pointerdown: EventHandler<PointerEvent>,
    pub element_capture: ElementCaptureAttr,
}

impl Remount for UseColorAreaProps {
    fn remount(&self) -> Self {
        Self {
            id: self.id.clone(),
            aria_label: self.aria_label,
            aria_labelledby: self.aria_labelledby,
            on_pointerdown: self.on_pointerdown.clone(),
            element_capture: self.element_capture.clone(),
        }
    }
}

pub type UseColorAreaAttrs = (
    FixedAttr,
    FixedAttr,
    SignalAttr<Option<String>>,
    SignalAttr<Option<Id>>,
    OnEvent<ev::pointerdown>,
    ElementCaptureAttr,
);

impl IntoAttrs for UseColorAreaProps {
    type Attrs = UseColorAreaAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            fixed_attr("id", self.id),
            fixed_attr("role", AriaRole::Group),
            signal_attr("aria-label", self.aria_label),
            signal_attr("aria-labelledby", self.aria_labelledby),
            self.on_pointerdown.into_on(ev::pointerdown),
            self.element_capture,
        )
    }
}

/// Props of the thumb (presentational: the inputs carry the semantics).
#[derive(Debug)]
pub struct UseColorAreaThumbProps {
    pub on_pointerdown: EventHandler<PointerEvent>,
    pub on_keydown: EventHandler<KeyboardEvent>,
    pub on_keyup: EventHandler<KeyboardEvent>,
    pub on_focusin: EventHandler<FocusEvent>,
    pub on_focusout: EventHandler<FocusEvent>,
}

impl Remount for UseColorAreaThumbProps {
    fn remount(&self) -> Self {
        Self {
            on_pointerdown: self.on_pointerdown.clone(),
            on_keydown: self.on_keydown.clone(),
            on_keyup: self.on_keyup.clone(),
            on_focusin: self.on_focusin.clone(),
            on_focusout: self.on_focusout.clone(),
        }
    }
}

pub type UseColorAreaThumbAttrs = (
    FixedAttr,
    OnEvent<ev::pointerdown>,
    OnEvent<ev::keydown>,
    OnEvent<ev::keyup>,
    OnEvent<ev::focusin>,
    OnEvent<ev::focusout>,
);

impl IntoAttrs for UseColorAreaThumbProps {
    type Attrs = UseColorAreaThumbAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            fixed_attr("role", AriaRole::Presentation),
            self.on_pointerdown.into_on(ev::pointerdown),
            self.on_keydown.into_on(ev::keydown),
            self.on_keyup.into_on(ev::keyup),
            self.on_focusin.into_on(ev::focusin),
            self.on_focusout.into_on(ev::focusout),
        )
    }
}

/// Props of a channel's visually hidden range input.
#[derive(Debug)]
pub struct UseColorAreaInputProps {
    pub id: Id,
    /// "2D slider" (localized).
    pub aria_roledescription: Signal<String>,
    pub min: f64,
    pub max: f64,
    pub step: f64,
    pub value: Signal<f64>,
    pub is_disabled: Signal<bool>,
    pub name: Option<String>,
    pub form: Option<String>,
    pub tabindex: Signal<Option<i32>>,
    pub aria_label: Signal<Option<String>>,
    pub aria_labelledby: Signal<Option<Id>>,
    pub aria_describedby: Signal<Option<Id>>,
    pub aria_details: Signal<Option<Id>>,
    pub aria_orientation: AriaOrientation,
    pub aria_valuetext: Signal<String>,
    pub aria_hidden: Signal<Option<AriaHidden>>,
    /// Sets the channel from the input's value (e.g. by assistive technology).
    pub on_input: EventHandler<Event>,
    pub on_focus: EventHandler<FocusEvent>,
    pub on_blur: EventHandler<FocusEvent>,
    pub element_capture: ElementCaptureAttr,
}

impl Remount for UseColorAreaInputProps {
    fn remount(&self) -> Self {
        Self {
            id: self.id.clone(),
            aria_roledescription: self.aria_roledescription,
            min: self.min,
            max: self.max,
            step: self.step,
            value: self.value,
            is_disabled: self.is_disabled,
            name: self.name.clone(),
            form: self.form.clone(),
            tabindex: self.tabindex,
            aria_label: self.aria_label,
            aria_labelledby: self.aria_labelledby,
            aria_describedby: self.aria_describedby,
            aria_details: self.aria_details,
            aria_orientation: self.aria_orientation,
            aria_valuetext: self.aria_valuetext,
            aria_hidden: self.aria_hidden,
            on_input: self.on_input.clone(),
            on_focus: self.on_focus.clone(),
            on_blur: self.on_blur.clone(),
            element_capture: self.element_capture.clone(),
        }
    }
}

pub type UseColorAreaInputAttrs = (
    (
        FixedAttr,
        FixedAttr,
        FixedAttr,
        FixedAttr,
        FixedAttr,
        SignalAttr<f64>,
        Property<&'static str, Signal<f64>>,
        SignalAttr<bool>,
        FixedAttr,
        FixedAttr,
        SignalAttr<Option<i32>>,
    ),
    (
        SignalAttr<String>,
        SignalAttr<Option<String>>,
        SignalAttr<Option<Id>>,
        SignalAttr<Option<Id>>,
        SignalAttr<Option<Id>>,
        FixedAttr,
        SignalAttr<String>,
        SignalAttr<Option<AriaHidden>>,
    ),
    (
        OnEvent<ev::input>,
        OnEvent<ev::focus>,
        OnEvent<ev::blur>,
        ElementCaptureAttr,
    ),
);

impl IntoAttrs for UseColorAreaInputProps {
    type Attrs = UseColorAreaInputAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            (
                fixed_attr("id", self.id),
                fixed_attr("type", "range"),
                fixed_attr("min", self.min),
                fixed_attr("max", self.max),
                fixed_attr("step", self.step),
                signal_attr("value", self.value),
                // The attribute is the initial value only: once changed (e.g. by assistive
                // technology), the input follows its property.
                prop("value", self.value),
                signal_attr("disabled", self.is_disabled),
                fixed_attr("name", self.name),
                fixed_attr("form", self.form),
                signal_attr("tabindex", self.tabindex),
            ),
            (
                signal_attr("aria-roledescription", self.aria_roledescription),
                signal_attr("aria-label", self.aria_label),
                signal_attr("aria-labelledby", self.aria_labelledby),
                signal_attr("aria-describedby", self.aria_describedby),
                signal_attr("aria-details", self.aria_details),
                fixed_attr("aria-orientation", self.aria_orientation),
                signal_attr("aria-valuetext", self.aria_valuetext),
                signal_attr("aria-hidden", self.aria_hidden),
            ),
            (
                self.on_input.into_on(ev::input),
                self.on_focus.into_on(ev::focus),
                self.on_blur.into_on(ev::blur),
                self.element_capture,
            ),
        )
    }
}

/// One of the area's two inputs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Axis {
    X,
    Y,
}

/// The pointer interaction under way: the pointer and whether it started on the area (else
/// on the thumb).
#[derive(Debug, Clone, Copy)]
struct Press {
    pointer_id: i32,
    on_area: bool,
}

/// Behavior and accessibility of a 2D color area: dragging the thumb or pressing the area,
/// arrow keys (with Shift: page steps), PageUp/PageDown and Home/End, two visually hidden range
/// inputs for assistive technology and forms.
pub fn use_color_area<C: ColorValue>(input: UseColorAreaInput<C>) -> UseColorAreaReturn {
    crate::metrics::subject!();
    let UseColorAreaInput {
        state,
        is_disabled,
        aria_label,
        aria_labelledby,
        aria_describedby,
        aria_details,
        x_name,
        y_name,
        form,
    } = input;

    let direction = use_direction();
    let is_rtl = move || direction.get_untracked() == WritingDirection::Rtl;
    let area_element = CapturedElement::new();
    let x_input = CapturedElement::new();
    let y_input = CapturedElement::new();

    let focused_input = RwSignal::new(None::<Axis>);
    let changed_via_keyboard = RwSignal::new(false);
    let changed_via_input = RwSignal::new(false);
    let focus_input = move |axis: Axis| {
        let element = match axis {
            Axis::X => x_input,
            Axis::Y => y_input,
        };
        if let Some(element) = element.get_untracked() {
            focus_element(&element, true);
        }
    };

    use_form_reset(UseFormResetInput {
        element: x_input,
        initial_value: state.default_value(),
        on_reset: Callback::new(move |color: C| state.set_value(color)),
    });

    // -- Keyboard: PageUp/PageDown on y, Home/End on x (arrows come through `use_move`) --
    let keyboard_update = move |step: &dyn Fn(), axis: Axis| {
        state.set_dragging(true);
        changed_via_keyboard.set(true);
        step();
        state.set_dragging(false);
        focus_input(axis);
        focused_input.set(Some(axis));
    };
    let keyboard = use_keyboard(UseKeyboardInput {
        is_disabled,
        shortcuts: Some(
            KeyboardShortcuts::new()
                .on(Shortcut::new(KeyboardKey::PageUp), move |_| {
                    keyboard_update(&|| state.increment_y(state.y_channel_page_step), Axis::Y);
                })
                .on(Shortcut::new(KeyboardKey::PageDown), move |_| {
                    keyboard_update(&|| state.decrement_y(state.y_channel_page_step), Axis::Y);
                })
                .on(Shortcut::new(KeyboardKey::Home), move |_| {
                    keyboard_update(
                        &|| {
                            if is_rtl() {
                                state.increment_x(state.x_channel_page_step);
                            } else {
                                state.decrement_x(state.x_channel_page_step);
                            }
                        },
                        Axis::X,
                    );
                })
                .on(Shortcut::new(KeyboardKey::End), move |_| {
                    keyboard_update(
                        &|| {
                            if is_rtl() {
                                state.decrement_x(state.x_channel_page_step);
                            } else {
                                state.increment_x(state.x_channel_page_step);
                            }
                        },
                        Axis::X,
                    );
                }),
        ),
        allow_repeats: true,
        on_key_down: None,
        on_key_up: None,
        allow_composing: false,
    })
    .target
    .into_props();

    // -- Moving: dragging (pointer deltas over the area's size) and arrow keys (steps) --
    let press = StoredValue::new(None::<Press>);
    let current_position = StoredValue::new(None::<Point>);
    let on_move_start = Callback::new(move |_: MoveStartEvent| {
        current_position.set_value(None);
        state.set_dragging(true);
    });
    let on_move = Callback::new(move |e: MoveEvent| {
        let (dx, dy) = (e.delta_x, e.delta_y);
        if e.pointer_type == PointerType::Keyboard {
            let shift = e.modifiers.shift_key;
            let x_step = if shift && state.x_channel_page_step > state.x_channel_step {
                state.x_channel_page_step
            } else {
                state.x_channel_step
            };
            let y_step = if shift && state.y_channel_page_step > state.y_channel_step {
                state.y_channel_page_step
            } else {
                state.y_channel_step
            };
            let towards_end = if is_rtl() { dx < 0.0 } else { dx > 0.0 };
            if dx != 0.0 && towards_end {
                state.increment_x(x_step);
            } else if dx != 0.0 {
                state.decrement_x(x_step);
            } else if dy > 0.0 {
                state.decrement_y(y_step);
            } else if dy < 0.0 {
                state.increment_y(y_step);
            }
            let changed = dx != 0.0 || dy != 0.0;
            changed_via_keyboard.set(changed);
            // The input of the axis that moved more.
            focused_input.set(Some(if changed && dy.abs() > dx.abs() {
                Axis::Y
            } else {
                Axis::X
            }));
        } else {
            let (width, height) = area_element.get_untracked().map_or((0.0, 0.0), |area| {
                let rect = area.get_bounding_client_rect();
                (rect.width(), rect.height())
            });
            let mut position = current_position
                .get_value()
                .unwrap_or_else(|| untrack(|| state.thumb_position()));
            if width > 0.0 {
                position.x += if is_rtl() { -dx } else { dx } / width;
            }
            if height > 0.0 {
                position.y += dy / height;
            }
            current_position.set_value(Some(position));
            state.set_color_from_point(position);
        }
    });
    let on_move_end = Callback::new(move |_: MoveEndEvent| {
        press.update_value(|press| {
            if let Some(press) = press {
                press.on_area = false;
            }
        });
        state.set_dragging(false);
        focus_input(focused_input.get_untracked().unwrap_or(Axis::X));
    });
    let thumb_move = use_move(UseMoveInput {
        is_disabled,
        on_move_start: Some(on_move_start),
        on_move: Some(on_move),
        on_move_end: Some(on_move_end),
    })
    .target
    .into_props();
    // The area forwards its moves only while a press started on it.
    let on_area = move || press.get_value().is_some_and(|press| press.on_area);
    let area_move = use_move(UseMoveInput {
        is_disabled,
        on_move_start: Some(Callback::new(move |e| {
            if on_area() {
                on_move_start.run(e);
            }
        })),
        on_move: Some(Callback::new(move |e| {
            if on_area() {
                on_move.run(e);
            }
        })),
        on_move_end: Some(Callback::new(move |e| {
            if on_area() {
                on_move_end.run(e);
            }
        })),
    })
    .target
    .into_props();

    let focus_within = use_focus_within(UseFocusWithinInput {
        on_blur_within: Some(Callback::new(move |_: FocusWithinEvent| {
            changed_via_keyboard.set(false);
            changed_via_input.set(false);
        })),
        ..UseFocusWithinInput::default()
    })
    .target
    .into_props();

    // -- Presses: on the thumb (start dragging it) or the area (jump there, then drag) --
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
        changed_via_keyboard.set(false);
        state.set_dragging(false);
        focus_input(Axis::X);
    };
    // On the window, not the owner document, as react-aria listens.
    let listen_for_release = move || {
        if let Some(window) = leptos_use::use_window().as_ref() {
            let listener = listen_to(window, ev::pointerup, false, move |e: PointerEvent| {
                release(e.pointer_id());
            });
            pointer_up_listener.set_value(Some(SendWrapper::new(listener)));
        }
    };
    let ignored = |e: &PointerEvent| {
        PointerType::of(e) == PointerType::Mouse
            && (e.button() != 0 || e.alt_key() || e.ctrl_key() || e.meta_key())
    };
    let on_thumb_down = EventHandler::new(move |e: PointerEvent| {
        if is_disabled.get_untracked() || ignored(&e) || state.is_dragging.get_untracked() {
            return;
        }
        press.set_value(Some(Press {
            pointer_id: e.pointer_id(),
            on_area: false,
        }));
        changed_via_keyboard.set(false);
        focus_input(Axis::X);
        state.set_dragging(true);
        listen_for_release();
    });
    let on_area_down = EventHandler::new(move |e: PointerEvent| {
        if is_disabled.get_untracked() || ignored(&e) {
            return;
        }
        let Some(area) = e
            .expect_current_target()
            .dyn_into::<web_sys::Element>()
            .ok()
        else {
            return;
        };
        let rect = area.get_bounding_client_rect();
        let mut x = (e.client_x() - rect.x()) / rect.width();
        let y = (e.client_y() - rect.y()) / rect.height();
        if is_rtl() {
            x = 1.0 - x;
        }
        let inside = (0.0..=1.0).contains(&x) && (0.0..=1.0).contains(&y);
        if inside && !state.is_dragging.get_untracked() && press.get_value().is_none() {
            press.set_value(Some(Press {
                pointer_id: e.pointer_id(),
                on_area: true,
            }));
            changed_via_keyboard.set(false);
            state.set_color_from_point(Point::new(x, y));
            focus_input(Axis::X);
            state.set_dragging(true);
            listen_for_release();
        }
    });

    // -- Inputs --
    let on_input = EventHandler::new(move |e: Event| {
        changed_via_input.set(true);
        let Some(target) = e
            .expect_target()
            .dyn_into::<web_sys::HtmlInputElement>()
            .ok()
        else {
            return;
        };
        let Ok(value) = target.value().parse::<f64>() else {
            return;
        };
        let target: &web_sys::Node = &target;
        if x_input.get_untracked().is_some_and(|x| **x == *target) {
            state.set_x_value(value);
        } else if y_input.get_untracked().is_some_and(|y| **y == *target) {
            state.set_y_value(value);
        }
    });
    let input_focus = |axis: Axis| {
        use_focus(UseFocusInput {
            on_focus: Some(Callback::new(move |_| focused_input.set(Some(axis)))),
            ..UseFocusInput::default()
        })
        .target
        .into_props()
    };
    let x_focus = input_focus(Axis::X);
    let y_focus = input_focus(Axis::Y);

    let (x_channel, y_channel, z_channel) = (state.x_channel, state.y_channel, state.z_channel);
    let locale = use_locale();
    let strings = use_localized_strings::<ColorStrings>();
    let display_color = state.display_color();
    let value_text = move |channel: C::Channel| {
        Signal::derive(move || {
            let color = display_color.get();
            let locale = locale.get();
            let strings = strings.read();
            let name_and_value = |c: C::Channel| {
                strings.color_name_and_value(ColorNameAndValueArgs {
                    name: &C::channel_name(c, &locale),
                    value: &color.format_channel_value(c, &locale),
                })
            };
            let text = if changed_via_input.get() || changed_via_keyboard.get() {
                name_and_value(channel)
            } else {
                let other = if channel == y_channel {
                    x_channel
                } else {
                    y_channel
                };
                [channel, other, z_channel].map(name_and_value).join(", ")
            };
            format!("{text}, {}", color.color_name(&locale))
        })
    };

    let is_mobile = use_platform_check(|| is_ios() || is_android());
    // Memos (the labels and the role description): they allocate and have several readers.
    let input_label: Signal<Option<String>> = Memo::new(move |_| {
        let strings = strings.read();
        let color_picker = strings.color_picker();
        Some(match aria_label.get() {
            Some(label) => strings.color_input_label(ColorInputLabelArgs {
                label: &label,
                channel_label: &color_picker,
            }),
            None => color_picker,
        })
    })
    .into();
    // The inputs are named by their label too, next to other labels (`labels`).
    let labelled_by = |own_id: &Id| {
        let own_id = own_id.clone();
        Signal::derive(move || {
            crate::utils::labels::labelledby_ids(
                &own_id,
                input_label.with(|label| label.as_deref().is_some_and(|label| !label.is_empty())),
                aria_labelledby.get().as_ref(),
            )
        })
    };
    let area_id = use_id("color-area");
    let area_label: Signal<Option<String>> = Memo::new(move |_| {
        let color_picker = strings.read().color_picker();
        match aria_label.get() {
            Some(label) => Some(format!("{label}, {color_picker}")),
            // On touch devices, the area itself is announced (react-aria's default label).
            None => {
                (is_mobile.get() && aria_labelledby.with(Option::is_none)).then_some(color_picker)
            }
        }
    })
    .into();
    let roledescription: Signal<String> =
        Memo::new(move |_| strings.read().two_dimensional_slider()).into();

    let x_range = C::channel_range(x_channel);
    let y_range = C::channel_range(y_channel);
    let x_id = use_id("color-area-x");
    let y_id = use_id("color-area-y");
    // So that only one "2D slider" is listed by screen readers, the unfocused input is hidden
    // until the value changes by keyboard (react-aria).
    let x_input_props = UseColorAreaInputProps {
        aria_roledescription: roledescription,
        aria_labelledby: labelled_by(&x_id),
        id: x_id,
        min: x_range.min_value,
        max: x_range.max_value,
        step: state.x_channel_step,
        value: state.x_value,
        is_disabled,
        name: x_name,
        form: form.clone(),
        tabindex: Signal::derive(move || {
            let focused = focused_input.get();
            (!(is_mobile.get() || focused.is_none() || focused == Some(Axis::X))).then_some(-1)
        }),
        aria_label: input_label,
        aria_describedby,
        aria_details,
        aria_orientation: AriaOrientation::Horizontal,
        aria_valuetext: value_text(x_channel),
        aria_hidden: Signal::derive(move || {
            let focused = focused_input.get();
            let shown = is_mobile.get()
                || focused.is_none()
                || focused == Some(Axis::X)
                || changed_via_keyboard.get();
            (!shown).then_some(AriaHidden::True)
        }),
        on_input: on_input.clone(),
        on_focus: x_focus.on_focus,
        on_blur: x_focus.on_blur,
        element_capture: x_input.attr(),
    };
    let y_input_props = UseColorAreaInputProps {
        aria_roledescription: roledescription,
        aria_labelledby: labelled_by(&y_id),
        id: y_id,
        min: y_range.min_value,
        max: y_range.max_value,
        step: state.y_channel_step,
        value: state.y_value,
        is_disabled,
        name: y_name,
        form,
        tabindex: Signal::derive(move || {
            (!(is_mobile.get() || focused_input.get() == Some(Axis::Y))).then_some(-1)
        }),
        aria_label: input_label,
        aria_describedby,
        aria_details,
        aria_orientation: AriaOrientation::Vertical,
        aria_valuetext: value_text(y_channel),
        aria_hidden: Signal::derive(move || {
            let shown = is_mobile.get()
                || focused_input.get() == Some(Axis::Y)
                || changed_via_keyboard.get();
            (!shown).then_some(AriaHidden::True)
        }),
        on_input,
        on_focus: y_focus.on_focus,
        on_blur: y_focus.on_blur,
        element_capture: y_input.attr(),
    };

    // -- Styles (react-aria's `useColorAreaGradient`) --
    let gradient = Memo::new(move |_| {
        state
            .value
            .get()
            .area_gradient(x_channel, y_channel, direction.get())
    });
    let area_styles = Styles::new()
        .add_unchecked("position", "relative")
        .add(TouchActionProperty.declare(TouchAction::None))
        .add(ForcedColorAdjustProperty.declare(ForcedColorAdjust::None))
        // Computed gradients: no checked grammar in `leptos-css` yet.
        .add_optional_unchecked("background", move || Some(gradient.get().background))
        .add_optional_unchecked("background-blend-mode", move || {
            gradient.get().blend_mode.map(BlendMode::as_str)
        });
    let thumb_position = move || {
        let Point { x, y } = state.thumb_position();
        Point::new(if is_rtl() { 1.0 - x } else { x }, y)
    };
    let thumb_styles = Styles::new()
        .add_unchecked("position", "absolute")
        .add_reactive(move || {
            LeftProperty.declare(LengthPercentageAuto::from(computed_pct(
                thumb_position().x * 100.0,
            )))
        })
        .add_reactive(move || {
            TopProperty.declare(LengthPercentageAuto::from(computed_pct(
                thumb_position().y * 100.0,
            )))
        })
        .add_unchecked("transform", "translate(-50%, -50%)")
        .add(TouchActionProperty.declare(TouchAction::None))
        .add(ForcedColorAdjustProperty.declare(ForcedColorAdjust::None));

    // With other labels next to its `aria-label`, the area names itself too.
    let area_labelledby = {
        let area_id = area_id.clone();
        Signal::derive(move || {
            crate::utils::labels::labelledby_ids(
                &area_id,
                area_label.with(|label| label.as_deref().is_some_and(|label| !label.is_empty())),
                aria_labelledby.get().as_ref(),
            )
        })
    };
    UseColorAreaReturn {
        area: Part::stored(PropsWithStyles::new(
            UseColorAreaProps {
                id: area_id,
                aria_label: area_label,
                aria_labelledby: area_labelledby,
                on_pointerdown: on_area_down.chain(area_move.on_pointerdown),
                element_capture: area_element.attr(),
            },
            area_styles,
        )),
        thumb: Part::stored(PropsWithStyles::new(
            UseColorAreaThumbProps {
                on_pointerdown: on_thumb_down.chain(thumb_move.on_pointerdown),
                on_keydown: keyboard.on_keydown.chain(thumb_move.on_keydown),
                on_keyup: keyboard.on_keyup,
                on_focusin: focus_within.on_focusin,
                on_focusout: focus_within.on_focusout,
            },
            thumb_styles,
        )),
        x_input: Part::stored(PropsWithStyles::new(
            x_input_props,
            visually_hidden_full_size_styles(),
        )),
        y_input: Part::stored(PropsWithStyles::new(
            y_input_props,
            visually_hidden_full_size_styles(),
        )),
    }
}
