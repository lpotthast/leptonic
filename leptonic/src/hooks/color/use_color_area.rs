// Upstream: react-aria/src/color/useColorArea.ts @ 99e6102368
// Upstream: react-aria/src/color/useColorAreaGradient.ts @ 99e6102368
use leptos::{
    attr,
    attr::Attr,
    ev,
    prelude::*,
    tachys::html::property::{Property, prop},
};
use send_wrapper::SendWrapper;
use wasm_bindgen::JsCast;
use web_sys::{Event, FocusEvent, KeyboardEvent, PointerEvent};

use super::use_color_area_state::ColorAreaState;
use crate::{
    CapturedElement, ElementCaptureAttr, EventHandler, IntoAttrs, OnEvent, PropsWithStyles,
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
        platform::{
            device::{is_android, is_ios},
            use_platform_check,
        },
        point::Point,
        pointer_type::PointerType,
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
    pub aria_labelledby: Option<String>,
    pub aria_describedby: Option<String>,
    pub aria_details: Option<String>,
    /// The name of the x channel's input, for form submission.
    pub x_name: Option<String>,
    /// The name of the y channel's input, for form submission.
    pub y_name: Option<String>,
    /// The id of a `<form>` the inputs belong to.
    pub form: Option<String>,
}

/// Return value of [`use_color_area`].
#[derive(Debug)]
pub struct UseColorAreaReturn {
    /// For the area (the gradient).
    pub color_area_props: PropsWithStyles<UseColorAreaProps>,
    /// For the thumb inside the area.
    pub thumb_props: PropsWithStyles<UseColorAreaThumbProps>,
    /// For the visually hidden range input of the x channel (inside the thumb).
    pub x_input_props: PropsWithStyles<UseColorAreaInputProps>,
    /// For the visually hidden range input of the y channel (inside the thumb).
    pub y_input_props: PropsWithStyles<UseColorAreaInputProps>,
}

/// Props of the color area's element.
#[derive(Debug)]
pub struct UseColorAreaProps {
    pub id: String,
    pub aria_label: Signal<Option<String>>,
    pub aria_labelledby: Signal<Option<String>>,
    pub on_pointerdown: EventHandler<PointerEvent>,
    pub element_capture: ElementCaptureAttr,
}

pub type UseColorAreaAttrs = (
    Attr<attr::Id, String>,
    Attr<attr::Role, AriaRole>,
    Attr<attr::AriaLabel, Signal<Option<String>>>,
    Attr<attr::AriaLabelledby, Signal<Option<String>>>,
    OnEvent<ev::pointerdown>,
    ElementCaptureAttr,
);

impl IntoAttrs for UseColorAreaProps {
    type Attrs = UseColorAreaAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Id, self.id),
            Attr(attr::Role, AriaRole::Group),
            Attr(attr::AriaLabel, self.aria_label),
            Attr(attr::AriaLabelledby, self.aria_labelledby),
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
    pub element_capture: ElementCaptureAttr,
}

pub type UseColorAreaThumbAttrs = (
    Attr<attr::Role, AriaRole>,
    OnEvent<ev::pointerdown>,
    OnEvent<ev::keydown>,
    OnEvent<ev::keyup>,
    OnEvent<ev::focusin>,
    OnEvent<ev::focusout>,
    ElementCaptureAttr,
);

impl IntoAttrs for UseColorAreaThumbProps {
    type Attrs = UseColorAreaThumbAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Role, AriaRole::Presentation),
            self.on_pointerdown.into_on(ev::pointerdown),
            self.on_keydown.into_on(ev::keydown),
            self.on_keyup.into_on(ev::keyup),
            self.on_focusin.into_on(ev::focusin),
            self.on_focusout.into_on(ev::focusout),
            self.element_capture,
        )
    }
}

/// Props of a channel's visually hidden range input.
#[derive(Debug)]
pub struct UseColorAreaInputProps {
    pub id: String,
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
    pub aria_labelledby: Option<String>,
    pub aria_describedby: Option<String>,
    pub aria_details: Option<String>,
    pub aria_orientation: AriaOrientation,
    pub aria_valuetext: Signal<String>,
    pub aria_hidden: Signal<Option<AriaHidden>>,
    /// Sets the channel from the input's value (e.g. by assistive technology).
    pub on_input: EventHandler<Event>,
    pub on_focus: EventHandler<FocusEvent>,
    pub on_blur: EventHandler<FocusEvent>,
    pub element_capture: ElementCaptureAttr,
}

pub type UseColorAreaInputAttrs = (
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
        Attr<attr::Tabindex, Signal<Option<i32>>>,
    ),
    (
        Attr<attr::AriaRoledescription, Signal<String>>,
        Attr<attr::AriaLabel, Signal<Option<String>>>,
        Attr<attr::AriaLabelledby, Option<String>>,
        Attr<attr::AriaDescribedby, Option<String>>,
        Attr<attr::AriaDetails, Option<String>>,
        Attr<attr::AriaOrientation, AriaOrientation>,
        Attr<attr::AriaValuetext, Signal<String>>,
        Attr<attr::AriaHidden, Signal<Option<AriaHidden>>>,
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
                Attr(attr::Tabindex, self.tabindex),
            ),
            (
                Attr(attr::AriaRoledescription, self.aria_roledescription),
                Attr(attr::AriaLabel, self.aria_label),
                Attr(attr::AriaLabelledby, self.aria_labelledby),
                Attr(attr::AriaDescribedby, self.aria_describedby),
                Attr(attr::AriaDetails, self.aria_details),
                Attr(attr::AriaOrientation, self.aria_orientation),
                Attr(attr::AriaValuetext, self.aria_valuetext),
                Attr(attr::AriaHidden, self.aria_hidden),
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
#[allow(clippy::too_many_lines)]
pub fn use_color_area<C: ColorValue>(input: UseColorAreaInput<C>) -> UseColorAreaReturn {
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
    let thumb_element = CapturedElement::new();
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
        ..UseKeyboardInput::default()
    })
    .props;

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
    .props;
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
    .props;

    let focus_within = use_focus_within(UseFocusWithinInput {
        on_blur_within: Some(Callback::new(move |_: FocusWithinEvent| {
            changed_via_keyboard.set(false);
            changed_via_input.set(false);
        })),
        ..UseFocusWithinInput::default()
    })
    .props;

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
        .props
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
    let labelled_by = |own_id: &str| {
        aria_labelledby
            .as_ref()
            .map(|ids| format!("{own_id} {ids}"))
    };
    let input_label = Signal::derive(move || {
        let strings = strings.read();
        let color_picker = strings.color_picker();
        Some(match aria_label.get() {
            Some(label) => strings.color_input_label(ColorInputLabelArgs {
                label: &label,
                channel_label: &color_picker,
            }),
            None => color_picker,
        })
    });
    let area_id = use_id("color-area");
    let has_labelledby = aria_labelledby.is_some();
    let area_label = Signal::derive(move || {
        let color_picker = strings.read().color_picker();
        match aria_label.get() {
            Some(label) => Some(format!("{label}, {color_picker}")),
            // On touch devices, the area itself is announced (react-aria's default label).
            None => (is_mobile.get() && !has_labelledby).then_some(color_picker),
        }
    });
    let roledescription = Signal::derive(move || strings.read().two_dimensional_slider());

    let input_styles = || visually_hidden_full_size_styles();
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
        aria_describedby: aria_describedby.clone(),
        aria_details: aria_details.clone(),
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

    // With other labels next to its `aria-label`, the area names itself too (`use_label`'s rule).
    let area_labelledby = {
        let area_id = area_id.clone();
        let ids = aria_labelledby.clone();
        Signal::derive(move || {
            let ids = ids.clone()?;
            Some(if area_label.with(Option::is_some) {
                format!("{area_id} {ids}")
            } else {
                ids
            })
        })
    };
    UseColorAreaReturn {
        color_area_props: PropsWithStyles::new(
            UseColorAreaProps {
                id: area_id,
                aria_label: area_label,
                aria_labelledby: area_labelledby,
                on_pointerdown: on_area_down.chain(area_move.on_pointerdown),
                element_capture: area_element.attr(),
            },
            area_styles,
        ),
        thumb_props: PropsWithStyles::new(
            UseColorAreaThumbProps {
                on_pointerdown: on_thumb_down.chain(thumb_move.on_pointerdown),
                on_keydown: keyboard.on_keydown.chain(thumb_move.on_keydown),
                on_keyup: keyboard.on_keyup,
                on_focusin: focus_within.on_focusin,
                on_focusout: focus_within.on_focusout,
                element_capture: thumb_element.attr(),
            },
            thumb_styles,
        ),
        x_input_props: PropsWithStyles::new(x_input_props, input_styles()),
        y_input_props: PropsWithStyles::new(y_input_props, input_styles()),
    }
}
