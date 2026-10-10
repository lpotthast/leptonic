// Upstream: react-aria/src/datepicker/useDateField.ts @ 740c6c5c4a
// Upstream: react-aria/src/datepicker/useDatePickerGroup.ts @ 740c6c5c4a
// Upstream: react-aria/src/datepicker/useDisplayNames.ts @ 740c6c5c4a
// Upstream: react-aria-components/test/DateField.test.js @ 740c6c5c4a
// Upstream: react-aria-components/test/TimeField.test.js @ 740c6c5c4a
// Upstream: @adobe/react-spectrum/test/datepicker/DateField.test.js @ 740c6c5c4a
// Upstream: @adobe/react-spectrum/test/datepicker/TimeField.test.js @ 740c6c5c4a
use leptos::{
    attr::{self, Attr},
    ev::{self},
    prelude::*,
};
use send_wrapper::SendWrapper;
use web_sys::{FocusEvent, KeyboardEvent, MouseEvent, PointerEvent};

use super::{
    types::{DateSegmentType, DateValue, MaxGranularity, TimeValue},
    use_date_field_state::DateFieldState,
    use_time_field_state::TimeFieldState,
};
use crate::{
    CapturedElement, ElementCaptureAttr, EventHandler, IdRefs, IntoAttrs, OnEvent, PropsWithStyles,
    hooks::{
        focus::{FocusManager, FocusManagerOptions, UseFocusWithinInput, use_focus_within},
        form::{
            LabelElementType, UseFieldInput, UseFieldReturn, UseFormResetInput,
            UseFormValidationInput, UseLabelProps, ValidationBehavior, use_field, use_form_reset,
            use_form_validation,
        },
        interactions::{
            PressEvent, UseKeyboardInput, UsePressAttrs, UsePressInput, UsePressProps,
            use_keyboard, use_press,
        },
        overlay::OverlayTriggerState,
    },
    utils::{
        aria::{AriaDisabled, AriaRole},
        dom_ext::EventAccessors,
        i18n::{WritingDirection, use_direction},
        intl_strings::{DatePickerStrings, use_localized_strings},
        key::KeyboardKey,
        keyboard_shortcut::{KeyboardShortcuts, Shortcut, ShortcutOutcome},
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
// ## DIFFERENT BEHAVIOR
// - Segment names come from the date picker's messages ("year", "Jahr"), react-aria's fallback
//   when the browser has no `Intl.DisplayNames` (ICU4X has no display names for date fields).
//
// =============================================================================

/// The names of the segments (react-aria's `useDisplayNames`).
pub(crate) fn display_name(strings: &DatePickerStrings, kind: DateSegmentType) -> String {
    match kind {
        DateSegmentType::Era => strings.era(),
        DateSegmentType::Year => strings.year(),
        DateSegmentType::Month => strings.month(),
        DateSegmentType::Day => strings.day(),
        DateSegmentType::Hour => strings.hour(),
        DateSegmentType::Minute => strings.minute(),
        DateSegmentType::Second => strings.second(),
        DateSegmentType::DayPeriod => strings.day_period(),
        DateSegmentType::TimeZoneName => strings.time_zone_name(),
        DateSegmentType::Literal => String::new(),
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
    type Attrs = (UsePressAttrs, OnEvent<ev::keydown>, OnEvent<ev::keyup>);

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
            Shortcut::new(KeyboardKey::ArrowDown).alt(),
            move |_: &KeyboardEvent| open_with(),
        )
        .on(
            Shortcut::new(KeyboardKey::ArrowUp).alt(),
            move |_: &KeyboardEvent| open_with(),
        )
        .on(
            Shortcut::new(KeyboardKey::ArrowLeft),
            move |e: &KeyboardEvent| arrow(e, false),
        )
        .on(
            Shortcut::new(KeyboardKey::ArrowRight),
            move |e: &KeyboardEvent| arrow(e, true),
        );
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
    // PressEvent's target is the group that owns the press hook. Retain the DOM event's
    // original target so a range separator resolves to the segments immediately before it
    // (react-aria reads this from window.event while the press callback runs).
    // SSR leaves this empty and can dispose its owner on a different executor thread.
    let press_target = StoredValue::new(None::<SendWrapper<web_sys::Element>>);
    let press = use_press(UsePressInput {
        prevent_focus_on_press: Signal::stored(true),
        allow_text_selection_on_press: Signal::stored(true),
        on_press_start: Some(Callback::new(move |e: PressEvent| {
            if e.pointer_type == PointerType::Mouse {
                focus_last(press_target.get_value().map(SendWrapper::take));
            }
        })),
        on_press: Some(Callback::new(move |e: PressEvent| {
            if matches!(e.pointer_type, PointerType::Touch | PointerType::Pen) {
                focus_last(press_target.get_value().map(SendWrapper::take));
            }
        })),
        ..UsePressInput::default()
    });
    let (mut press_props, press_styles) = press.props.into_inner();
    press_props.on_pointerdown = EventHandler::new(move |e: PointerEvent| {
        press_target.set_value(
            wasm_bindgen::JsCast::dyn_into::<web_sys::Element>(e.expect_target())
                .ok()
                .map(SendWrapper::new),
        );
    })
    .chain(press_props.on_pointerdown);
    press_props.on_click = EventHandler::new(move |e: MouseEvent| {
        press_target.set_value(
            wasm_bindgen::JsCast::dyn_into::<web_sys::Element>(e.expect_target())
                .ok()
                .map(SendWrapper::new),
        );
    })
    .chain(press_props.on_click);
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
    /// What labels the picker, and so the field's segments (`UseDatePickerReturn::labelledby`).
    pub labelledby: Signal<Option<String>>,
    /// What describes the picker, and so the field's first segment
    /// (`UseDatePickerReturn::field_describedby`).
    pub describedby: Signal<Option<String>>,
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
    type Attrs = (<UseLabelProps as IntoAttrs>::Attrs, OnEvent<ev::click>);

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
        Attr<attr::AriaDisabled, Signal<Option<AriaDisabled>>>,
    ),
    UsePressAttrs,
    (
        OnEvent<ev::keydown>,
        OnEvent<ev::keyup>,
        OnEvent<ev::focusin>,
        OnEvent<ev::focusout>,
        ElementCaptureAttr,
    ),
);

impl IntoAttrs for UseDateFieldProps {
    type Attrs = UseDateFieldAttrs;

    fn into_attrs(self) -> Self::Attrs {
        // Press props carry their own optional long-press description. Emit one merged
        // attribute so an empty press description cannot erase the field's descriptions.
        let mut press = self.group.press;
        press.aria_describedby = IdRefs::derive([self.aria_describedby, press.aria_describedby]);
        (
            (
                Attr(attr::Id, self.id),
                Attr(attr::Role, self.role),
                Attr(attr::AriaLabel, self.aria_label),
                Attr(attr::AriaLabelledby, self.aria_labelledby),
                Attr(attr::AriaDisabled, self.aria_disabled),
            ),
            press.into_attrs(),
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
    let (overlay, focus_manager, picker_labels) = match picker {
        Some(picker) => (
            Some(picker.overlay),
            picker.focus_manager,
            Some((picker.labelledby, picker.describedby)),
        ),
        None => (None, None, None),
    };
    // Inside a picker, the picker's label labels the field (react-aria: `useDatePicker` gives
    // its field `aria-labelledby`).
    let has_label = match picker_labels {
        Some((labelledby, _)) => Signal::derive(move || labelledby.with(Option::is_some)),
        None => has_label,
    };
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
    let strings = use_localized_strings::<DatePickerStrings>();
    let description = Signal::derive(move || {
        state.value.get().map(|_| {
            let formatted = state.format_value();
            if state.max_granularity.get() == MaxGranularity::Hour {
                strings.read().selected_time_description(&formatted)
            } else {
                strings.read().selected_date_description(&formatted)
            }
        })
    });
    let description_id = use_description(description);
    let field_describedby = field_props.aria_describedby;
    let described_by = Signal::derive(move || {
        if let Some((_, picker_describedby)) = picker_labels {
            return picker_describedby.get();
        }
        [description_id.get(), field_describedby.get()]
            .into_iter()
            .flatten()
            .collect::<IdRefs>()
            .into_value()
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
        if let Some((picker_labelledby, _)) = picker_labels {
            return picker_labelledby.get();
        }
        [
            has_label.get().then(|| label_id.clone()),
            aria_labelledby.clone(),
        ]
        .into_iter()
        .flatten()
        .collect::<IdRefs>()
        .into_value()
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

#[cfg(all(test, feature = "ssr"))]
mod tests {
    use assertr::prelude::*;

    use super::*;
    use crate::testing::with_owner;

    #[test]
    fn date_picker_group_owner_can_be_cleaned_up_on_another_thread() {
        with_owner(|| {
            let owner = Owner::new();
            owner.with(|| {
                let _ = use_date_picker_group(UseDatePickerGroupInput {
                    element: CapturedElement::new(),
                    arrow_keys: GroupArrowKeys::MoveBetweenSegments,
                    overlay: None,
                });
            });

            // A streamed SSR response can finish on a different executor worker than the
            // one that rendered the field. Its empty pointer state must be safe to dispose.
            assert_that!(std::thread::spawn(move || owner.cleanup()).join()).is_ok();
        });
    }
}
