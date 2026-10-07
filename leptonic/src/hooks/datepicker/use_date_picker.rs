// Upstream: react-aria/src/datepicker/useDatePicker.ts @ 99e6102368
// Upstream: react-aria/src/datepicker/useDateRangePicker.ts @ 99e6102368
use leptos::prelude::*;
use web_sys::KeyboardEvent;

use super::{
    types::DateValue,
    use_date_field::{
        GroupArrowKeys, UseDateFieldLabelProps, UseDateFieldProps, UseDatePickerGroupInput,
        segment_focus_manager, use_date_picker_group,
    },
    use_date_picker_state::DatePickerState,
    use_date_range_picker_state::DateRangePickerState,
};
use crate::{
    hooks::{
        OverlayTriggerState, PropsWithStyles, UseButtonInput,
        focus::{
            FocusManager, FocusManagerOptions, FocusWithinEvent, UseFocusWithinInput,
            use_focus_within,
        },
        form::{LabelElementType, UseFieldInput, UseFieldReturn, use_field},
    },
    utils::{
        CapturedElement, EventHandler,
        aria::{AriaDisabled, AriaExpanded, AriaHasPopup, AriaRole},
        id::use_id,
        slot_id::SlotProps,
        use_description::use_description,
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Returns inputs for the parts' hooks (the button's `UseButtonInput`, the field's
//   description) and ids, rather than merged props objects; the calendar takes the state.
// - One input each (C8): `UseDatePickerInput`, `UseDateRangePickerInput` (the state, the group's
//   element and `DatePickerOptions`).
//
// ## OMITTED FEATURES
// - Localized strings: the button is named "Calendar", the description "Selected Date: ...".
//
// =============================================================================

/// The options of a date or date range picker (the parts of [`UseDatePickerInput`] and
/// [`UseDateRangePickerInput`] besides the state and the group).
#[derive(Clone, Default)]
pub struct DatePickerOptions {
    pub id: Option<String>,
    /// Whether a visible label labels the picker.
    pub has_label: Signal<bool>,
    pub aria_label: MaybeProp<String>,
    pub aria_labelledby: Option<String>,
    pub aria_describedby: Option<String>,
    pub is_disabled: Signal<bool>,
    pub is_read_only: Signal<bool>,
    pub on_focus_change: Option<Callback<bool>>,
    pub on_key_down: Option<Callback<KeyboardEvent>>,
    pub on_key_up: Option<Callback<KeyboardEvent>>,
    /// The id of the popover's dialog (moving the focus into it doesn't leave the picker).
    pub dialog_id: Signal<Option<String>>,
}

/// Input of [`use_date_picker`].
pub struct UseDatePickerInput<V: DateValue> {
    pub state: DatePickerState<V>,
    /// The group of the field and the button.
    pub group: CapturedElement,
    pub options: DatePickerOptions,
}

/// Input of [`use_date_range_picker`].
pub struct UseDateRangePickerInput<V: DateValue> {
    pub state: DateRangePickerState<V>,
    /// The group of the fields and the button.
    pub group: CapturedElement,
    pub options: DatePickerOptions,
}

/// Return value of [`use_date_picker`].
pub struct UseDatePickerReturn {
    pub label_props: UseDateFieldLabelProps,
    /// For the group of the field and the button.
    pub group_props: PropsWithStyles<UseDateFieldProps>,
    /// What describes the field's segments (the picker's description and value).
    pub field_describedby: Signal<Option<String>>,
    /// For the button opening the popover.
    pub button: UseButtonInput,
    /// What names the dialog: the button and the picker's label.
    pub dialog_labelledby: Signal<Option<String>>,
    pub description_props: SlotProps,
    pub error_message_props: SlotProps,
    /// What labels the picker (for its fields).
    pub labelledby: Signal<Option<String>>,
    /// The segments of the picker's fields, without the button (the fields share it).
    pub focus_manager: FocusManager,
}

/// Behavior and accessibility of a date picker (react-aria's `useDatePicker`): a group of a date
/// field and a button opening a dialog with a calendar, labelled and described as one ("Selected
/// Date: ..."); Alt+ArrowDown opens it.
pub fn use_date_picker<V: DateValue>(input: UseDatePickerInput<V>) -> UseDatePickerReturn {
    let UseDatePickerInput {
        state,
        group,
        options,
    } = input;
    let description = Signal::derive(move || {
        let date = state.format_value();
        (!date.is_empty()).then(|| format!("Selected Date: {date}"))
    });
    picker_aria(options, state.overlay, description, group)
}

/// Behavior and accessibility of a date range picker (react-aria's `useDateRangePicker`): a group
/// of a start and an end date field and a button opening a dialog with a range calendar,
/// described as one ("Selected Range: ... to ..."). Label the fields "Start Date" and "End Date",
/// labelled by the picker (`labelledby`), with its `focus_manager`.
pub fn use_date_range_picker<V: DateValue>(
    input: UseDateRangePickerInput<V>,
) -> UseDatePickerReturn {
    let UseDateRangePickerInput {
        state,
        group,
        options,
    } = input;
    let description = Signal::derive(move || {
        state
            .format_value()
            .map(|(start, end)| format!("Selected Range: {start} to {end}"))
    });
    picker_aria(options, state.overlay, description, group)
}

/// What date and date range pickers share: the group, label, button and dialog.
pub(crate) fn picker_aria(
    input: DatePickerOptions,
    overlay: OverlayTriggerState,
    description: Signal<Option<String>>,
    group: CapturedElement,
) -> UseDatePickerReturn {
    let DatePickerOptions {
        id,
        has_label,
        aria_label,
        aria_labelledby,
        aria_describedby,
        is_disabled,
        is_read_only,
        on_focus_change,
        on_key_down,
        on_key_up,
        dialog_id,
    } = input;
    let button_id = use_id("date-picker-button");

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
        aria_labelledby,
        aria_describedby,
        ..UseFieldInput::default()
    });
    let field_id = field_props.id.clone();
    let field_labelledby = field_props.aria_labelledby;
    let labelledby =
        Signal::derive(move || field_labelledby.get().or_else(|| Some(field_id.clone())));

    let description_id = use_description(description);
    let field_describedby_ids = field_props.aria_describedby;
    let described_by = Signal::derive(move || {
        let ids: Vec<String> = [description_id.get(), field_describedby_ids.get()]
            .into_iter()
            .flatten()
            .collect();
        (!ids.is_empty()).then(|| ids.join(" "))
    });

    // Focus moving into the popover doesn't leave the picker.
    let is_focused = StoredValue::new(false);
    let focus_within = use_focus_within(UseFocusWithinInput {
        is_disabled: overlay.is_open,
        on_focus_within: Some(Callback::new(move |_: FocusWithinEvent| {
            if !is_focused.get_value() {
                is_focused.set_value(true);
                if let Some(on_focus_change) = on_focus_change {
                    on_focus_change.run(true);
                }
            }
        })),
        on_blur_within: Some(Callback::new(move |e: FocusWithinEvent| {
            // After the picker's disposal: nothing to report ("Blur After Disposal").
            if is_focused.try_get_value().is_none() {
                return;
            }
            let into_dialog = dialog_id
                .get_untracked()
                .and_then(|id| {
                    leptos_use::use_document()
                        .as_ref()
                        .and_then(|document| document.get_element_by_id(&id))
                })
                .zip(e.event.related_target())
                .is_some_and(|(dialog, related)| {
                    wasm_bindgen::JsCast::dyn_ref::<web_sys::Node>(&related).is_some_and(
                        |related| crate::utils::shadow_dom::node_contains(&dialog, related),
                    )
                });
            if !into_dialog {
                is_focused.set_value(false);
                if let Some(on_focus_change) = on_focus_change {
                    on_focus_change.run(false);
                }
            }
        })),
        ..UseFocusWithinInput::default()
    });

    let (group_keys, group_styles) = use_date_picker_group(UseDatePickerGroupInput {
        element: group,
        arrow_keys: GroupArrowKeys::MoveBetweenSegments,
        overlay: Some(overlay),
    })
    .into_inner();
    // The user's handlers only while closed.
    let is_open = overlay.is_open;
    let on_keydown = group_keys
        .on_keydown
        .chain(EventHandler::new(move |e: KeyboardEvent| {
            if !is_open.get_untracked()
                && let Some(on_key_down) = on_key_down
            {
                on_key_down.run(e);
            }
        }));
    let on_keyup = group_keys
        .on_keyup
        .chain(EventHandler::new(move |e: KeyboardEvent| {
            if !is_open.get_untracked()
                && let Some(on_key_up) = on_key_up
            {
                on_key_up.run(e);
            }
        }));

    let excluded_button = button_id.clone();
    let focus_manager = segment_focus_manager(group)
        .with_default_accept(move |element| element.id() != excluded_button);
    let manager = StoredValue::new(focus_manager.clone());
    let button_labelledby = {
        let button_id = button_id.clone();
        Signal::derive(move || {
            Some(format!(
                "{button_id} {}",
                labelledby.get().unwrap_or_default()
            ))
        })
    };
    UseDatePickerReturn {
        label_props: UseDateFieldLabelProps {
            label: label_props,
            on_click: EventHandler::new(move |_| {
                manager.with_value(|manager| manager.focus_first(FocusManagerOptions::default()));
            }),
        },
        group_props: PropsWithStyles::new(
            UseDateFieldProps {
                id: Some(field_props.id),
                role: AriaRole::Group,
                aria_label: Signal::derive(move || field_props.aria_label.get()),
                aria_labelledby: labelledby,
                aria_describedby: described_by,
                aria_disabled: Signal::derive(move || {
                    is_disabled.get().then_some(AriaDisabled::True)
                }),
                group: super::use_date_field::UseDatePickerGroupProps {
                    on_keydown,
                    on_keyup,
                    ..group_keys
                },
                on_focusin: focus_within.props.on_focusin,
                on_focusout: focus_within.props.on_focusout,
                element_capture: group.attr(),
            },
            group_styles,
        ),
        field_describedby: described_by,
        button: UseButtonInput {
            id: Some(button_id),
            aria_haspopup: Signal::stored(Some(AriaHasPopup::Dialog)),
            aria_label: MaybeProp::from("Calendar".to_owned()),
            aria_labelledby: button_labelledby,
            aria_describedby: described_by,
            aria_expanded: Signal::derive(move || {
                Some(if is_open.get() {
                    AriaExpanded::True
                } else {
                    AriaExpanded::False
                })
            }),
            is_disabled: Signal::derive(move || is_disabled.get() || is_read_only.get()),
            on_press: Some(Callback::new(move |_| overlay.set_open(true))),
            ..UseButtonInput::default()
        },
        dialog_labelledby: button_labelledby,
        description_props,
        error_message_props,
        labelledby,
        focus_manager,
    }
}
