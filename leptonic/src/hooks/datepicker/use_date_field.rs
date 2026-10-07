// Upstream: react-aria/src/datepicker/useDateField.ts @ 99e6102368
// Upstream: react-aria/src/datepicker/useDatePickerGroup.ts @ 99e6102368
// Upstream: react-aria/src/datepicker/useDisplayNames.ts @ 99e6102368
use leptos::{
    attr::{self, Attr},
    ev::{self, On, SharedEventCallback},
    prelude::*,
};
use web_sys::{FocusEvent, KeyboardEvent};

use super::{
    types::{DateSegmentType, DateValue, MaxGranularity, TimeValue},
    use_date_field_state::DateFieldState,
    use_time_field_state::TimeFieldState,
};
use crate::{
    hooks::{
        IntoAttrs, OverlayTriggerState, PressEvent, PropsWithStyles, UseKeyboardInput,
        UsePressAttrs, UsePressInput, UsePressProps,
        focus::{FocusManager, FocusManagerOptions, UseFocusWithinInput, use_focus_within},
        form::{
            LabelElementType, UseFieldInput, UseFieldReturn, UseFormResetInput,
            UseFormValidationInput, UseLabelProps, ValidationBehavior, use_field, use_form_reset,
            use_form_validation,
        },
        use_keyboard, use_press,
    },
    utils::{
        CapturedElement, ElementCaptureAttr, EventAccessors, EventHandler,
        aria::{AriaDisabled, AriaRole},
        i18n::use_direction,
        keyboard_shortcut::{KeyboardShortcuts, Shortcut, ShortcutOutcome},
        locale::WritingDirection,
        pointer_type::PointerType,
        slot_id::SlotProps,
        use_description::use_description,
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - The data segments need (labels, focus manager) is returned as `DateFieldData` (react-aria:
//   a `WeakMap` keyed by the state); the picker a field belongs to is an input field,
//   `DateFieldPicker` (react-aria: private props of `useDatePicker`'s field).
// - One input each (C8): `UseDateFieldInput`, `UseTimeFieldInput` (the state, the elements and
//   `DateFieldOptions`); `UseDatePickerGroupInput` with `GroupArrowKeys` (react-aria: a
//   `disableArrowNavigation` flag) and the picker's overlay state to open (react-aria: the
//   state's `setOpen`).
// - The hidden input is part of the return (`input_props`); the form reset and validation
//   capture it (`input_element`).
//
// ## OMITTED FEATURES
// - Localized field names and descriptions (`useDisplayNames`): English ("year", "Selected
//   Date: ..."); ICU4X has no display names for date fields yet.
//
// =============================================================================

/// The English names of the segments (react-aria's `useDisplayNames`).
pub(crate) fn display_name(kind: DateSegmentType) -> &'static str {
    match kind {
        DateSegmentType::Era => "era",
        DateSegmentType::Year => "year",
        DateSegmentType::Month => "month",
        DateSegmentType::Day => "day",
        DateSegmentType::Hour => "hour",
        DateSegmentType::Minute => "minute",
        DateSegmentType::Second => "second",
        DateSegmentType::DayPeriod => "AM/PM",
        DateSegmentType::TimeZoneName => "time zone",
        DateSegmentType::Literal => "",
    }
}

/// What a date field's segments need from it.
pub struct DateFieldData<V: DateValue> {
    pub state: DateFieldState<V>,
    pub aria_label: MaybeProp<String>,
    pub aria_labelledby: Signal<Option<String>>,
    pub aria_describedby: Signal<Option<String>>,
    pub(crate) focus_manager: StoredValue<FocusManager>,
}

// Derived, it would require a `Copy` value type.
#[allow(clippy::expl_impl_clone_on_copy)]
impl<V: DateValue> Clone for DateFieldData<V> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<V: DateValue> Copy for DateFieldData<V> {}

impl<V: DateValue> DateFieldData<V> {
    // `try_`: a segment removed with its field hands the focus over after the field's disposal.
    pub(crate) fn focus_next(&self) -> bool {
        self.focus_manager
            .try_with_value(|manager| manager.focus_next(FocusManagerOptions::default()))
            .flatten()
            .is_some()
    }

    pub(crate) fn focus_previous(&self) -> bool {
        self.focus_manager
            .try_with_value(|manager| manager.focus_previous(FocusManagerOptions::default()))
            .flatten()
            .is_some()
    }

    pub(crate) fn focus_first(&self) {
        self.focus_manager
            .try_with_value(|manager| manager.focus_first(FocusManagerOptions::default()));
    }
}

/// The focus manager of a field's segments.
pub(crate) fn segment_focus_manager(element: CapturedElement) -> FocusManager {
    FocusManager::new(move || element.get_untracked().map(|element| (*element).clone()))
}

/// Whether a group's left and right arrows move between its segments.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GroupArrowKeys {
    /// Move to the previous or next segment (by position in right-to-left locales).
    MoveBetweenSegments,
    /// Leave them to an enclosing group (a field inside a date picker: the picker's group moves
    /// across its fields).
    Ignore,
}

/// Input of [`use_date_picker_group`].
#[derive(Clone, Copy)]
pub struct UseDatePickerGroupInput {
    /// The group's element (containing the segments).
    pub element: CapturedElement,
    pub arrow_keys: GroupArrowKeys,
    /// The popover Alt+ArrowDown/Up opens, inside a date picker.
    pub overlay: Option<OverlayTriggerState>,
}

/// Props of a date field group (the segments' container), from `use_date_picker_group`.
#[derive(Debug)]
pub struct UseDatePickerGroupProps {
    pub press: UsePressProps,
    pub on_keydown: EventHandler<KeyboardEvent>,
    pub on_keyup: EventHandler<KeyboardEvent>,
}

impl IntoAttrs for UseDatePickerGroupProps {
    type Attrs = (
        UsePressAttrs,
        On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
        On<ev::keyup, SharedEventCallback<KeyboardEvent>>,
    );

    fn into_attrs(self) -> Self::Attrs {
        (
            self.press.into_attrs(),
            self.on_keydown.into_on(ev::keydown),
            self.on_keyup.into_on(ev::keyup),
        )
    }
}

/// Keyboard and pointer behavior of the element holding a field's segments (react-aria's
/// `useDatePickerGroup`): the left and right arrows move between segments (by position in
/// right-to-left locales), Alt+ArrowDown/Up opens a picker (`open`), and pressing the group
/// outside the segments focuses the last segment with a value.
pub fn use_date_picker_group(
    input: UseDatePickerGroupInput,
) -> PropsWithStyles<UseDatePickerGroupProps> {
    let UseDatePickerGroupInput {
        element,
        arrow_keys,
        overlay,
    } = input;
    let direction = use_direction();
    let manager = StoredValue::new(segment_focus_manager(element));

    let arrow = move |e: &KeyboardEvent, forward: bool| -> ShortcutOutcome {
        if arrow_keys == GroupArrowKeys::Ignore {
            return ShortcutOutcome::Ignored;
        }
        if direction.get_untracked() == WritingDirection::Rtl {
            let target = wasm_bindgen::JsCast::dyn_into::<web_sys::Element>(e.expect_target()).ok();
            let next = target.and_then(|target| {
                let group = element.get_untracked()?;
                find_next_segment(
                    &group,
                    target.get_bounding_client_rect().left(),
                    if forward { 1.0 } else { -1.0 },
                )
            });
            match next {
                Some(next) => {
                    crate::utils::focus::focus_element(&next, false);
                    ShortcutOutcome::Handled
                }
                None => ShortcutOutcome::Ignored,
            }
        } else {
            // Handled even at the ends (react-aria returns without `false`).
            manager.with_value(|manager| {
                if forward {
                    manager.focus_next(FocusManagerOptions::default());
                } else {
                    manager.focus_previous(FocusManagerOptions::default());
                }
            });
            ShortcutOutcome::Handled
        }
    };
    let open_with = move || match overlay {
        Some(overlay) => {
            overlay.set_open(true);
            ShortcutOutcome::Handled
        }
        None => ShortcutOutcome::Ignored,
    };
    let shortcuts = KeyboardShortcuts::new()
        .on(
            Shortcut::key("ArrowDown").alt(),
            move |_: &KeyboardEvent| open_with(),
        )
        .on(Shortcut::key("ArrowUp").alt(), move |_: &KeyboardEvent| {
            open_with()
        })
        .on(Shortcut::key("ArrowLeft"), move |e: &KeyboardEvent| {
            arrow(e, false)
        })
        .on(Shortcut::key("ArrowRight"), move |e: &KeyboardEvent| {
            arrow(e, true)
        });
    let keyboard = use_keyboard(UseKeyboardInput {
        shortcuts: Some(shortcuts),
        allow_repeats: true,
        ..UseKeyboardInput::default()
    });

    // Pressing the field (not a segment: they stop the pointer events) focuses the segment
    // before the pressed point, or the last one, skipping back over empty segments.
    let focus_last = move |target: Option<web_sys::Element>| {
        let Some(group) = element.get_untracked() else {
            return;
        };
        let segments = tabbable_segments(&group);
        let before = target.and_then(|target| {
            segments
                .iter()
                .rev()
                .find(|segment| {
                    target.compare_document_position(segment)
                        & web_sys::Node::DOCUMENT_POSITION_PRECEDING
                        != 0
                })
                .cloned()
        });
        let Some(mut index) = before
            .and_then(|before| segments.iter().position(|segment| *segment == before))
            .or_else(|| segments.len().checked_sub(1))
        else {
            return;
        };
        while index > 0
            && segments[index].has_attribute("data-placeholder")
            && segments[index - 1].has_attribute("data-placeholder")
        {
            index -= 1;
        }
        crate::utils::focus::focus_element(&segments[index], false);
    };
    let press_target = |e: &PressEvent| {
        wasm_bindgen::JsCast::dyn_into::<web_sys::Element>((*e.target).clone()).ok()
    };
    let press = use_press(UsePressInput {
        prevent_focus_on_press: Signal::stored(true),
        allow_text_selection_on_press: Signal::stored(true),
        on_press_start: Some(Callback::new(move |e: PressEvent| {
            if e.pointer_type == PointerType::Mouse {
                focus_last(press_target(&e));
            }
        })),
        on_press: Some(Callback::new(move |e: PressEvent| {
            if matches!(e.pointer_type, PointerType::Touch | PointerType::Pen) {
                focus_last(press_target(&e));
            }
        })),
        ..UsePressInput::default()
    });
    let (press_props, press_styles) = press.props.into_inner();
    PropsWithStyles::new(
        UseDatePickerGroupProps {
            press: press_props,
            on_keydown: keyboard.props.on_keydown,
            on_keyup: keyboard.props.on_keyup,
        },
        press_styles,
    )
}

/// The tabbable elements of a group (its segments, a picker's button), in document order.
fn tabbable_segments(group: &web_sys::Element) -> Vec<web_sys::Element> {
    // Segments, and a picker's button (react-aria: a tabbable tree walker over the group).
    let Ok(candidates) =
        group.query_selector_all("[tabindex], button, input, select, textarea, a[href]")
    else {
        return Vec::new();
    };
    (0..candidates.length())
        .filter_map(|index| candidates.item(index))
        .filter_map(|node| wasm_bindgen::JsCast::dyn_into::<web_sys::Element>(node).ok())
        .filter(crate::utils::focusability::is_tabbable)
        .collect()
}

/// The closest segment in a direction (-1: left, 1: right) from a horizontal position.
fn find_next_segment(
    group: &web_sys::Element,
    from_x: f64,
    direction: f64,
) -> Option<web_sys::Element> {
    tabbable_segments(group)
        .into_iter()
        .filter_map(|segment| {
            let distance = segment.get_bounding_client_rect().left() - from_x;
            (distance != 0.0 && distance.signum() == direction).then_some((distance.abs(), segment))
        })
        .min_by(|(a, _), (b, _)| a.total_cmp(b))
        .map(|(_, segment)| segment)
}

/// The date picker a field belongs to (react-aria: private props of `useDatePicker`'s field).
#[derive(Clone)]
pub struct DateFieldPicker {
    /// The picker's popover (Alt+ArrowDown opens it).
    pub overlay: OverlayTriggerState,
    /// The picker's focus manager, when it spans more fields (a range picker's).
    pub focus_manager: Option<FocusManager>,
}

/// The options of a date or time field (the parts of [`UseDateFieldInput`] and
/// [`UseTimeFieldInput`] besides the state and the elements).
#[derive(Clone, Default)]
pub struct DateFieldOptions {
    pub id: Option<String>,
    /// Whether a visible label labels the field.
    pub has_label: Signal<bool>,
    pub aria_label: MaybeProp<String>,
    pub aria_labelledby: Option<String>,
    pub aria_describedby: Option<String>,
    pub auto_focus: bool,
    /// The hidden input's form (when outside it).
    pub form: Option<String>,
    pub on_focus_change: Option<Callback<bool>>,
    pub on_key_down: Option<Callback<KeyboardEvent>>,
    pub on_key_up: Option<Callback<KeyboardEvent>>,
    /// The date picker the field belongs to: no group role then (the picker's group labels and
    /// describes it).
    pub picker: Option<DateFieldPicker>,
}

/// Input of [`use_date_field`].
pub struct UseDateFieldInput<V: DateValue> {
    pub state: DateFieldState<V>,
    /// The group of segments.
    pub element: CapturedElement,
    /// The hidden input (form reset and native validation).
    pub input_element: CapturedElement,
    pub options: DateFieldOptions,
}

/// Input of [`use_time_field`].
pub struct UseTimeFieldInput<T: TimeValue> {
    pub state: TimeFieldState<T>,
    /// The group of segments.
    pub element: CapturedElement,
    /// The hidden input (form reset and native validation).
    pub input_element: CapturedElement,
    pub options: DateFieldOptions,
}

/// Return value of [`use_date_field`].
pub struct UseDateFieldReturn<V: DateValue> {
    /// For the label (a `span`: it labels a group).
    pub label_props: UseDateFieldLabelProps,
    /// For the group of segments.
    pub field_props: PropsWithStyles<UseDateFieldProps>,
    /// For the hidden input carrying the value in forms.
    pub input_props: UseDateFieldInputProps,
    pub description_props: SlotProps,
    pub error_message_props: SlotProps,
    /// What the segments need (`use_date_segment`).
    pub data: DateFieldData<V>,
}

/// Props of the field's label: pressing it focuses the first segment.
#[derive(Debug, Clone)]
pub struct UseDateFieldLabelProps {
    pub label: UseLabelProps,
    pub on_click: EventHandler<web_sys::MouseEvent>,
}

impl IntoAttrs for UseDateFieldLabelProps {
    type Attrs = (
        <UseLabelProps as IntoAttrs>::Attrs,
        On<ev::click, SharedEventCallback<web_sys::MouseEvent>>,
    );

    fn into_attrs(self) -> Self::Attrs {
        (self.label.into_attrs(), self.on_click.into_on(ev::click))
    }
}

/// Props of the group of segments.
#[derive(Debug)]
pub struct UseDateFieldProps {
    pub id: Option<String>,
    pub role: AriaRole,
    pub aria_label: Signal<Option<String>>,
    pub aria_labelledby: Signal<Option<String>>,
    pub aria_describedby: Signal<Option<String>>,
    pub aria_disabled: Signal<Option<AriaDisabled>>,
    pub group: UseDatePickerGroupProps,
    pub on_focusin: EventHandler<FocusEvent>,
    pub on_focusout: EventHandler<FocusEvent>,
    pub element_capture: ElementCaptureAttr,
}

pub type UseDateFieldAttrs = (
    (
        Attr<attr::Id, Option<String>>,
        Attr<attr::Role, AriaRole>,
        Attr<attr::AriaLabel, Signal<Option<String>>>,
        Attr<attr::AriaLabelledby, Signal<Option<String>>>,
        Attr<attr::AriaDescribedby, Signal<Option<String>>>,
        Attr<attr::AriaDisabled, Signal<Option<AriaDisabled>>>,
    ),
    UsePressAttrs,
    (
        On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
        On<ev::keyup, SharedEventCallback<KeyboardEvent>>,
        On<ev::focusin, SharedEventCallback<FocusEvent>>,
        On<ev::focusout, SharedEventCallback<FocusEvent>>,
        ElementCaptureAttr,
    ),
);

impl IntoAttrs for UseDateFieldProps {
    type Attrs = UseDateFieldAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            (
                Attr(attr::Id, self.id),
                Attr(attr::Role, self.role),
                Attr(attr::AriaLabel, self.aria_label),
                Attr(attr::AriaLabelledby, self.aria_labelledby),
                Attr(attr::AriaDescribedby, self.aria_describedby),
                Attr(attr::AriaDisabled, self.aria_disabled),
            ),
            self.group.press.into_attrs(),
            (
                self.group.on_keydown.into_on(ev::keydown),
                self.group.on_keyup.into_on(ev::keyup),
                self.on_focusin.into_on(ev::focusin),
                self.on_focusout.into_on(ev::focusout),
                self.element_capture,
            ),
        )
    }
}

/// Props of the hidden input carrying the value (ISO 8601) in forms. With native validation a
/// hidden text input, so that `required` blocks submitting an empty field.
#[derive(Debug, Clone)]
pub struct UseDateFieldInputProps {
    pub input_type: &'static str,
    pub hidden: bool,
    pub name: Option<String>,
    pub form: Option<String>,
    pub value: Signal<String>,
    pub disabled: Signal<bool>,
    pub required: Signal<bool>,
    pub element_capture: ElementCaptureAttr,
}

impl IntoAttrs for UseDateFieldInputProps {
    type Attrs = (
        Attr<attr::Type, &'static str>,
        Attr<attr::Hidden, bool>,
        Attr<attr::Name, Option<String>>,
        Attr<attr::Form, Option<String>>,
        Attr<attr::Value, Signal<String>>,
        Attr<attr::Disabled, Signal<bool>>,
        Attr<attr::Required, Signal<bool>>,
        ElementCaptureAttr,
    );

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Type, self.input_type),
            Attr(attr::Hidden, self.hidden),
            Attr(attr::Name, self.name),
            Attr(attr::Form, self.form),
            Attr(attr::Value, self.value),
            Attr(attr::Disabled, self.disabled),
            Attr(attr::Required, self.required),
            self.element_capture,
        )
    }
}

/// Behavior and accessibility of a date field (react-aria's `useDateField`): a group of
/// segments labelled by the field, describing its value ("Selected Date: ..."), committing an
/// incomplete value and showing validation when left, with a hidden input for forms.
pub fn use_date_field<V: DateValue>(input: UseDateFieldInput<V>) -> UseDateFieldReturn<V> {
    let UseDateFieldInput {
        state,
        element,
        input_element,
        options,
    } = input;
    let DateFieldOptions {
        id,
        has_label,
        aria_label,
        aria_labelledby,
        aria_describedby,
        auto_focus,
        form,
        on_focus_change,
        on_key_down,
        on_key_up,
        picker,
    } = options;
    let is_in_picker = picker.is_some();
    let (overlay, focus_manager) = picker
        .map(|picker| (picker.overlay, picker.focus_manager))
        .unzip();
    let focus_manager = focus_manager.flatten();
    let validation = state.validation;

    let UseFieldReturn {
        label_props,
        field_props,
        description_props,
        error_message_props,
        ..
    } = use_field(UseFieldInput {
        id,
        has_label,
        label_element_type: LabelElementType::Span,
        aria_label,
        aria_labelledby: aria_labelledby.clone(),
        aria_describedby,
        ..UseFieldInput::default()
    });

    // Leaving the field commits an incomplete value, and shows the validation if it changed.
    let value_on_focus = StoredValue::new(None::<V>);
    let focus_within = use_focus_within(UseFocusWithinInput {
        on_focus_within: Some(Callback::new(move |_| {
            value_on_focus.set_value(state.value.get_untracked());
        })),
        on_blur_within: Some(Callback::new(move |_| {
            // After the field's disposal (a removed focused segment): nothing to commit ("Blur
            // After Disposal").
            if !state.is_alive() {
                return;
            }
            state.confirm_placeholder();
            if state.value.get_untracked() != value_on_focus.get_value() {
                validation.commit_validation();
            }
        })),
        on_focus_within_change: on_focus_change,
        ..UseFocusWithinInput::default()
    });

    // "Selected Date: May 20, 2024" ("Selected Time" for time fields).
    let description = Signal::derive(move || {
        state.value.get().map(|_| {
            let formatted = state.format_value();
            if state.max_granularity.get() == MaxGranularity::Hour {
                format!("Selected Time: {formatted}")
            } else {
                format!("Selected Date: {formatted}")
            }
        })
    });
    let description_id = use_description(description);
    let field_describedby = field_props.aria_describedby;
    let described_by = Signal::derive(move || {
        if is_in_picker {
            return field_describedby.get();
        }
        let ids: Vec<String> = [description_id.get(), field_describedby.get()]
            .into_iter()
            .flatten()
            .collect();
        (!ids.is_empty()).then(|| ids.join(" "))
    });

    let focus_manager =
        StoredValue::new(focus_manager.unwrap_or_else(|| segment_focus_manager(element)));
    let group = use_date_picker_group(UseDatePickerGroupInput {
        element,
        arrow_keys: if is_in_picker {
            GroupArrowKeys::Ignore
        } else {
            GroupArrowKeys::MoveBetweenSegments
        },
        overlay,
    });

    let label_id = label_props.id.clone();
    let segments_labelledby = Signal::derive(move || {
        let ids: Vec<String> = [
            has_label.get().then(|| label_id.clone()),
            aria_labelledby.clone(),
        ]
        .into_iter()
        .flatten()
        .collect();
        (!ids.is_empty()).then(|| ids.join(" "))
    });
    let data = DateFieldData {
        state,
        aria_label,
        aria_labelledby: segments_labelledby,
        aria_describedby: described_by,
        focus_manager,
    };

    if auto_focus {
        Effect::new(move |_| data.focus_first());
    }

    use_form_reset(UseFormResetInput {
        element: input_element,
        initial_value: state.default_value(),
        on_reset: Callback::new(move |value: Option<V>| state.set_value(value)),
    });
    use_form_validation(UseFormValidationInput {
        focus: Some(Callback::new(move |()| data.focus_first())),
        element: input_element,
        state: validation,
        validation_behavior: state.validation_behavior,
    });

    let (group_props, group_styles) = group.into_inner();
    let is_disabled = state.is_disabled;
    let label_aria = field_props.aria_label;
    let field_labelledby = field_props.aria_labelledby;
    let on_keydown = group_props
        .on_keydown
        .chain(EventHandler::new(move |e: KeyboardEvent| {
            if let Some(on_key_down) = on_key_down {
                on_key_down.run(e);
            }
        }));
    let on_keyup = group_props
        .on_keyup
        .chain(EventHandler::new(move |e: KeyboardEvent| {
            if let Some(on_key_up) = on_key_up {
                on_key_up.run(e);
            }
        }));
    let field = if is_in_picker {
        UseDateFieldProps {
            id: None,
            role: AriaRole::Presentation,
            aria_label: Signal::stored(None),
            aria_labelledby: Signal::stored(None),
            aria_describedby: Signal::stored(None),
            aria_disabled: Signal::stored(None),
            group: UseDatePickerGroupProps {
                on_keydown,
                on_keyup,
                ..group_props
            },
            on_focusin: focus_within.props.on_focusin,
            on_focusout: focus_within.props.on_focusout,
            element_capture: element.attr(),
        }
    } else {
        UseDateFieldProps {
            id: Some(field_props.id),
            role: AriaRole::Group,
            aria_label: Signal::derive(move || label_aria.get()),
            aria_labelledby: field_labelledby,
            aria_describedby: described_by,
            aria_disabled: Signal::derive(move || is_disabled.get().then_some(AriaDisabled::True)),
            group: UseDatePickerGroupProps {
                on_keydown,
                on_keyup,
                ..group_props
            },
            on_focusin: focus_within.props.on_focusin,
            on_focusout: focus_within.props.on_focusout,
            element_capture: element.attr(),
        }
    };
    let native = state.validation_behavior == ValidationBehavior::Native;

    UseDateFieldReturn {
        label_props: UseDateFieldLabelProps {
            label: label_props,
            on_click: EventHandler::new(move |_| data.focus_first()),
        },
        field_props: PropsWithStyles::new(
            field,
            group_styles.add_unchecked("unicode-bidi", "isolate"),
        ),
        input_props: UseDateFieldInputProps {
            input_type: if native { "text" } else { "hidden" },
            hidden: native,
            name: state.name(),
            form,
            value: Signal::derive(move || {
                state.value.with(|value| {
                    value
                        .as_ref()
                        .map(DateValue::to_iso_string)
                        .unwrap_or_default()
                })
            }),
            disabled: is_disabled,
            required: Signal::derive(move || native && state.is_required.get()),
            element_capture: input_element.attr(),
        },
        description_props,
        error_message_props,
        data,
    }
}

/// Behavior and accessibility of a time field (react-aria's `useTimeField`): a date field over the
/// time state's field whose hidden input submits the time (`"08:30:00"`), not a date and time.
pub fn use_time_field<T: TimeValue>(input: UseTimeFieldInput<T>) -> UseDateFieldReturn<T::Field> {
    let UseTimeFieldInput {
        state,
        element,
        input_element,
        options,
    } = input;
    let mut field = use_date_field(UseDateFieldInput {
        state: state.field,
        element,
        input_element,
        options,
    });
    let time_value = state.time_value;
    field.input_props.value = Signal::derive(move || {
        time_value
            .get()
            .map(|time| time.to_string())
            .unwrap_or_default()
    });
    field
}
