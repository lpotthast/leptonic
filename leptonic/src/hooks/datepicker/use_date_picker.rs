// Upstream: react-aria/src/datepicker/useDatePicker.ts @ 740c6c5c4a
// Upstream: react-aria/src/datepicker/useDateRangePicker.ts @ 740c6c5c4a
// Upstream: react-aria/test/datepicker/useDatePicker.test.tsx @ 740c6c5c4a
// Upstream: react-aria-components/test/DatePicker.test.js @ 740c6c5c4a
// Upstream: react-aria-components/test/DateRangePicker.test.js @ 740c6c5c4a
// Upstream: @adobe/react-spectrum/test/datepicker/DatePicker.test.js @ 740c6c5c4a
// Upstream: @adobe/react-spectrum/test/datepicker/DateRangePicker.test.js @ 740c6c5c4a
// Upstream: @adobe/react-spectrum/test/datepicker/DatePickerBase.test.js @ 740c6c5c4a
use leptos::prelude::*;
use web_sys::KeyboardEvent;

use super::{
    types::DateValue,
    use_date_field::{
        GroupArrowKeys, UseDateFieldLabelProps, UseDateFieldProps, UseDatePickerGroupInput,
        UseDatePickerGroupProps, segment_focus_manager, use_date_picker_group,
    },
    use_date_picker_state::DatePickerState,
    use_date_range_picker_state::DateRangePickerState,
};
use crate::{
    CapturedElement, EventHandler, Id, IdRefs, Part, PropsWithStyles,
    hooks::{
        button::UseButtonInput,
        focus::{
            FocusManager, FocusManagerOptions, FocusWithinEvent, UseFocusWithinInput,
            use_focus_within,
        },
        form::{LabelElementType, UseFieldInput, UseFieldReturn, use_field},
        overlay::OverlayTriggerState,
    },
    utils::{
        aria::{AriaExpanded, AriaHasPopup, AriaRole, aria_true},
        id::use_id,
        intl_strings::{DatePickerStrings, SelectedRangeDescriptionArgs, use_localized_strings},
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
// =============================================================================

/// The options of a date or date range picker (the parts of [`UseDatePickerInput`] and
/// [`UseDateRangePickerInput`] besides the state and the group).
#[derive(Clone, Default)]
pub struct DatePickerOptions {
    pub id: Option<Id>,
    /// Whether a visible label labels the picker.
    pub has_label: Signal<bool>,
    pub aria_label: MaybeProp<String>,
    pub aria_labelledby: Signal<Option<Id>>,
    pub aria_describedby: Signal<Option<Id>>,
    pub is_disabled: Signal<bool>,
    pub is_read_only: Signal<bool>,
    pub on_focus_change: Option<Callback<bool>>,
    pub on_key_down: Option<Callback<KeyboardEvent>>,
    pub on_key_up: Option<Callback<KeyboardEvent>>,
    /// The id of the popover's dialog (moving the focus into it doesn't leave the picker).
    pub dialog_id: Signal<Option<Id>>,
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

/// The return value of [`use_date_picker`].
#[derive(Debug)]
pub struct UseDatePickerReturn {
    /// The label element.
    pub label: Part<UseDateFieldLabelProps>,
    /// The group element around the field and the button: `let (attrs, styles) =
    /// group.attrs();`.
    pub group: Part<PropsWithStyles<UseDateFieldProps>>,
    /// What describes the field's segments (the picker's description and value).
    pub field_describedby: Signal<Option<Id>>,
    /// For the button opening the popover.
    pub button: UseButtonInput,
    /// What names the dialog: the button and the picker's label.
    pub dialog_labelledby: Signal<Option<Id>>,
    pub description: Part<SlotProps>,
    pub error_message: Part<SlotProps>,
    /// What labels the picker (for its fields).
    pub labelledby: Signal<Option<Id>>,
    /// The segments of the picker's fields, without the button (the fields share it).
    pub focus_manager: FocusManager,
}

/// Behavior and accessibility of a date picker (react-aria's `useDatePicker`): a group of a date
/// field and a button opening a dialog with a calendar, labelled and described as one ("Selected
/// Date: ..."); Alt+ArrowDown opens it.
pub fn use_date_picker<V: DateValue>(input: UseDatePickerInput<V>) -> UseDatePickerReturn {
    crate::metrics::subject!();
    let UseDatePickerInput {
        state,
        group,
        options,
    } = input;
    let strings = use_localized_strings::<DatePickerStrings>();
    let description = Signal::derive(move || {
        let date = state.format_value();
        (!date.is_empty()).then(|| strings.read().selected_date_description(&date))
    });
    let has_label = options.has_label;
    let aria_labelledby = options.aria_labelledby;
    let mut result = picker_aria(options, state.overlay, description, group);
    let labelledby = result.labelledby;
    // A single field receives the picker's aria-label directly. Only visible or external
    // labels need ID references; the group's self-reference is for the button and dialog.
    result.labelledby = Signal::derive(move || {
        if has_label.get() || aria_labelledby.with(Option::is_some) {
            labelledby.get()
        } else {
            None
        }
    });
    result
}

/// Behavior and accessibility of a date range picker (react-aria's `useDateRangePicker`): a group
/// of a start and an end date field and a button opening a dialog with a range calendar,
/// described as one ("Selected Range: ... to ..."). Label the fields "Start Date" and "End Date",
/// labelled by the picker (`labelledby`), with its `focus_manager`.
pub fn use_date_range_picker<V: DateValue>(
    input: UseDateRangePickerInput<V>,
) -> UseDatePickerReturn {
    crate::metrics::subject!();
    let UseDateRangePickerInput {
        state,
        group,
        options,
    } = input;
    let strings = use_localized_strings::<DatePickerStrings>();
    let description = Signal::derive(move || {
        state.format_value().map(|(start, end)| {
            strings
                .read()
                .selected_range_description(SelectedRangeDescriptionArgs {
                    start_date: &start,
                    end_date: &end,
                })
        })
    });
    picker_aria(options, state.overlay, description, group)
}

/// What date and date range pickers share: the group, label, button and dialog.
fn picker_aria(
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
        label: label_props,
        field: field_props,
        description: description_props,
        error_message: error_message_props,
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
    let label_props = label_props.into_props();
    let field_props = field_props.into_props();
    let field_id = field_props.id.clone();
    let field_labelledby = field_props.aria_labelledby;
    let labelledby =
        Signal::derive(move || field_labelledby.get().or_else(|| Some(field_id.clone())));

    let description_id = use_description(description);
    let field_describedby_ids = field_props.aria_describedby;
    let described_by = IdRefs::derive([description_id, field_describedby_ids]);

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
        on_focus_within_change: None,
    });
    let focus_within_props = focus_within.target.into_props();

    let (group_keys, group_styles) = use_date_picker_group(UseDatePickerGroupInput {
        element: group,
        arrow_keys: GroupArrowKeys::MoveBetweenSegments,
        overlay: Some(overlay),
    })
    .into_props()
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
        .with_default_accept(move |element| excluded_button != element.id().as_str());
    let manager = StoredValue::new(focus_manager);
    let button_labelledby = {
        let button_id = button_id.clone();
        Signal::derive(move || {
            crate::utils::id::join_ids([Some(button_id.clone()), labelledby.get()])
        })
    };
    UseDatePickerReturn {
        label: Part::stored(UseDateFieldLabelProps {
            label: label_props,
            on_click: EventHandler::new(move |_| {
                manager.with_value(|manager| manager.focus_first(FocusManagerOptions::default()));
            }),
        }),
        group: Part::stored(PropsWithStyles::new(
            UseDateFieldProps {
                id: Some(field_props.id),
                role: AriaRole::Group,
                aria_label: Signal::derive(move || field_props.aria_label.get()),
                aria_labelledby: labelledby,
                aria_describedby: described_by,
                aria_disabled: aria_true(is_disabled),
                group: UseDatePickerGroupProps {
                    on_keydown,
                    on_keyup,
                    ..group_keys
                },
                on_focusin: focus_within_props.on_focusin,
                on_focusout: focus_within_props.on_focusout,
                element_capture: group.attr(),
            },
            group_styles,
        )),
        field_describedby: described_by,
        button: UseButtonInput {
            id: Some(button_id),
            aria_haspopup: Signal::stored(Some(AriaHasPopup::Dialog)),
            aria_label: {
                let strings = use_localized_strings::<DatePickerStrings>();
                Signal::derive(move || Some(strings.read().calendar())).into()
            },
            aria_labelledby: button_labelledby,
            aria_describedby: described_by,
            aria_controls: Signal::derive(
                move || {
                    if is_open.get() { dialog_id.get() } else { None }
                },
            ),
            aria_expanded: Signal::derive(move || {
                Some(if is_open.get() {
                    AriaExpanded::True
                } else {
                    AriaExpanded::False
                })
            }),
            is_disabled: Signal::derive(move || is_disabled.get() || is_read_only.get()),
            on_press: Some(Callback::new(move |_| overlay.set_open(true))),
            element_type: crate::hooks::button::ButtonElementType::Button,
            button_type: crate::hooks::button::ButtonType::Button,
            is_pending: Signal::stored(false),
            allow_focus_when_disabled: Signal::stored(false),
            exclude_from_tab_order: Signal::stored(false),
            auto_focus: false,
            prevent_focus_on_press: Signal::stored(false),
            href: Signal::stored(None),
            target: Signal::stored(crate::hooks::link::LinkTarget::Same),
            rel: Vec::new(),
            form: crate::hooks::button::ButtonFormAttributes::default(),
            aria_pressed: Signal::stored(None),
            aria_checked: Signal::stored(None),
            role: None,
            aria_current: Signal::stored(None),
            aria_disabled: Signal::stored(None),
            on_press_start: None,
            on_press_end: None,
            on_press_up: None,
            on_press_change: None,
            long_press: None,
            on_hover_start: None,
            on_hover_end: None,
            on_hover_change: None,
            on_focus: None,
            on_blur: None,
            on_focus_change: None,
            on_key_down: None,
            on_key_up: None,
            shortcuts: None,
            on_context_menu: None,
        },
        dialog_labelledby: button_labelledby,
        description: description_props,
        error_message: error_message_props,
        labelledby,
        focus_manager,
    }
}
