// Upstream: react-aria/src/combobox/useComboBox.ts @ 99e6102368
// Upstream: react-aria/test/combobox/useComboBox.test.js @ 99e6102368
// Upstream: react-aria-components/test/ComboBox.test.js @ 99e6102368
// Upstream: react-aria-components/test/ComboBox.browser.test.tsx @ 99e6102368
// Upstream: @adobe/react-spectrum/test/combobox/ComboBox.test.js @ 99e6102368
use std::sync::Arc;

use leptos::{
    attr::{self, Attr},
    ev,
    prelude::*,
};
use wasm_bindgen::JsCast;
use web_sys::{FocusEvent, TouchEvent};

use super::{ComboBoxState, MenuTriggerAction};
use crate::{
    CapturedElement, ElementCaptureAttr, EventHandler, IntoAttrs, OnEvent, Propagation,
    hooks::{
        button::use_button::UseButtonInput,
        collections::{
            AutoFocus, CollectionOptions, FocusStrategy, Key, KeyboardDelegate, LinkBehavior,
            ListLayout, UseListKeyboardDelegateInput, UseSelectableCollectionInput,
            use_list_keyboard_delegate, use_selectable_collection,
        },
        form::{
            InputType, TextFieldElement,
            use_form_reset::{UseFormResetInput, use_form_reset},
            use_text_field::UseTextFieldInput,
            use_text_field_state::{UseTextFieldStateInput, use_text_field_state},
        },
        interactions::{use_keyboard::KeyboardEventWrapper, use_press::PressEvent},
        listbox::{UseListBoxInput, option_id},
        menu::use_menu_trigger::{MenuTriggerType, UseMenuTriggerInput, use_menu_trigger},
        overlay::use_overlay_trigger::OverlayTriggerType,
        select::SelectMode,
    },
    utils::{
        aria::{AriaAutocomplete, AriaExpanded, AriaRole},
        focus::focus_element,
        id::use_id,
        intl_strings::{ComboBoxStrings, FocusAnnouncementArgs, use_localized_strings},
        key::KeyboardKey,
        keyboard_shortcut::{KeyboardShortcuts, Shortcut, ShortcutOutcome},
        labels::labels,
        orientation::Orientation,
        pointer_type::PointerType,
        shadow_dom::{get_active_element, get_event_target},
        virtual_focus::dispatch_virtual_focus,
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - The input, button and listbox are configured, not rendered: `input` is the
//   `UseTextFieldInput` for `use_text_field` (plus `input_props`), `button` the
//   `UseButtonInput` for `use_button`, `listbox` the `UseListBoxInput` for `use_listbox`.
// - `has_label` says whether a visible label is rendered.
// - `name`, `is_read_only` and `validation_behavior` are read from the state (C8).
// - `form_value` and the values of the hidden inputs (`form_values`) come from the hook
//   (react-aria-components renders them in `ComboBox`). Whether the input or the hidden inputs
//   carry the name is decided when the hook is created (`allows_custom_value` then).
// - Element references are `CapturedElement`s: the caller attaches `popover` and
//   `button_element` (react-aria: `popoverRef`, `buttonRef`).
//
// ## DIFFERENT BEHAVIOR
// - The group size in the focus announcement counts the section's options (react-aria counts its
//   child nodes, the header included).
//
// ## OMITTED FEATURES
// - The value description of multiple selection (`useValueId`).
// - Item actions and links in the popover (`onAction`, `href` items).
//
// =============================================================================

/// What a combo box submits with its form (react-aria-components: `formValue`).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ComboBoxFormValue {
    /// The selected keys, in hidden inputs named like the field.
    #[default]
    Key,
    /// The input's text, under the field's name. Always used with `allows_custom_value`.
    Text,
}

/// Input of [`use_combobox`].
#[derive(Clone, Debug)]
pub struct UseComboBoxInput {
    /// The state; the field's `name`, `is_read_only` and validation come from it.
    pub state: ComboBoxState,
    /// The input's id. Generated when `None`.
    pub id: Option<String>,
    pub is_disabled: Signal<bool>,
    pub is_required: Signal<bool>,
    /// Whether a visible label is rendered.
    pub has_label: Signal<bool>,
    pub aria_label: MaybeProp<String>,
    pub aria_labelledby: Option<String>,
    pub aria_describedby: Option<String>,
    pub placeholder: MaybeProp<String>,
    /// What the form submits (`Text` when the state allows custom values).
    pub form_value: ComboBoxFormValue,
    /// The id of the form the combo box belongs to, if it is outside of it.
    pub form: Option<String>,
    /// Arrow keys wrap around at the ends of the list.
    pub should_focus_wrap: bool,
    /// Replaces the list keyboard delegate.
    pub keyboard_delegate: Option<Signal<Arc<dyn KeyboardDelegate>>>,
    /// The popover element (focus moving into it doesn't blur the combo box).
    pub popover: CapturedElement,
    /// The trigger button element: attach it to the button rendered with `button` (focus moving
    /// to it doesn't blur the combo box).
    pub button_element: CapturedElement,
    pub on_focus: Option<Callback<FocusEvent>>,
    pub on_blur: Option<Callback<FocusEvent>>,
}

/// Return value of [`use_combobox`].
pub struct UseComboBoxReturn {
    /// The text input's configuration, for `use_text_field`.
    pub input: UseTextFieldInput,
    /// Spread onto the input in addition to the text field's props.
    pub input_props: UseComboBoxInputProps,
    /// The trigger button's configuration, for `use_button`.
    pub button: UseButtonInput,
    /// The popover's listbox, for `use_listbox`.
    pub listbox: UseListBoxInput,
    /// With [`ComboBoxFormValue::Key`] and a `name`: render one `<input type="hidden">` per
    /// entry, named like the field (with `form`): the selected keys, or one empty value without
    /// a selection. Empty otherwise (the input submits its text).
    pub form_values: Signal<Vec<String>>,
}

impl std::fmt::Debug for UseComboBoxReturn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UseComboBoxReturn")
            .field("input_props", &self.input_props)
            .finish_non_exhaustive()
    }
}

/// Combo box props for the input, in addition to the text field's.
#[derive(Debug, Clone)]
pub struct UseComboBoxInputProps {
    pub role: AriaRole,
    pub aria_expanded: Signal<Option<AriaExpanded>>,
    /// Tapping the input's center on touch devices toggles the popover.
    pub on_touchend: EventHandler<TouchEvent>,
    /// Captures the input (for form reset).
    pub element_capture: ElementCaptureAttr,
}

pub type UseComboBoxInputAttrs = (
    Attr<attr::Role, AriaRole>,
    Attr<attr::AriaExpanded, Signal<Option<AriaExpanded>>>,
    OnEvent<ev::touchend>,
    ElementCaptureAttr,
);

impl IntoAttrs for UseComboBoxInputProps {
    type Attrs = UseComboBoxInputAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Role, self.role),
            Attr(attr::AriaExpanded, self.aria_expanded),
            self.on_touchend.into_on(ev::touchend),
            self.element_capture,
        )
    }
}

/// A combo box: a text input with a popover listbox of suggestions. Typing filters the options
/// (with the state's filter), arrow keys move a virtual focus through them while DOM focus stays
/// in the input, Enter selects, Escape reverts.
#[allow(clippy::too_many_lines)]
pub fn use_combobox(input: UseComboBoxInput) -> UseComboBoxReturn {
    let UseComboBoxInput {
        state,
        id,
        is_disabled,
        is_required,
        has_label,
        aria_label,
        aria_labelledby,
        aria_describedby,
        placeholder,
        form_value,
        form,
        should_focus_wrap,
        keyboard_delegate,
        popover,
        button_element,
        on_focus,
        on_blur,
    } = input;

    let is_read_only = state.is_read_only_signal();
    // Where the form value goes (the input's name or hidden inputs) is decided once.
    let form_value = if untrack(|| state.allows_custom_value()) {
        ComboBoxFormValue::Text
    } else {
        form_value
    };
    let name = state.name();
    let input_id = id.unwrap_or_else(|| use_id("combobox-input"));
    let label_id = use_id("combobox-label");
    let listbox_element = CapturedElement::new();
    // The input (for form reset, focusing it, and hiding the rest of the page).
    let input_element = CapturedElement::new();
    let focus_input = move || {
        if let Some(input) = input_element.get_untracked() {
            focus_element(&input, false);
        }
    };

    let menu_trigger = use_menu_trigger(UseMenuTriggerInput {
        menu_type: OverlayTriggerType::Listbox,
        is_disabled: Signal::derive(move || is_disabled.get() || is_read_only.get()),
        trigger: MenuTriggerType::Press,
        state,
    });
    let listbox_id = menu_trigger.menu_props.id.clone();

    let delegate = keyboard_delegate.unwrap_or_else(|| {
        use_list_keyboard_delegate(UseListKeyboardDelegateInput {
            state: state.list,
            element: listbox_element,
            orientation: Orientation::Vertical.into(),
            layout: ListLayout::Stack,
            layout_delegate: None,
        })
    });
    // Arrow keys in the input move the (virtual) focus through the options.
    let collection = use_selectable_collection(UseSelectableCollectionInput {
        selection: state.list.selection,
        item_elements: state.list.item_elements,
        delegate,
        element: CapturedElement::new(),
        options: CollectionOptions {
            disallow_type_ahead: true,
            disallow_empty_selection: true,
            should_focus_wrap,
            ..CollectionOptions::default()
        },
    })
    .props;
    let collection_keydown = collection.on_keydown;

    // Shortcuts ignore auto-repeated key presses (`use_keyboard`); the arrow keys below don't.
    // A read-only combo box handles no keys (react-aria passes only the caller's `onKeyDown`).
    let shortcuts = KeyboardShortcuts::new()
        .on(Shortcut::new(KeyboardKey::Enter), move |_| {
            if is_read_only.get_untracked() {
                return ShortcutOutcome::Ignored;
            }
            let was_open = untrack(|| state.is_open());
            state.commit();
            // Enter submits forms only while the popover is closed.
            ShortcutOutcome::Custom {
                prevent_default: was_open,
                continue_propagation: false,
            }
        })
        .on(Shortcut::new(KeyboardKey::Tab), move |_| {
            if is_read_only.get_untracked() {
                return ShortcutOutcome::Ignored;
            }
            if untrack(|| state.is_open()) {
                state.commit();
            }
            ShortcutOutcome::Custom {
                prevent_default: false,
                continue_propagation: true,
            }
        })
        .on(Shortcut::new(KeyboardKey::Escape), move |_| {
            if is_read_only.get_untracked() {
                return ShortcutOutcome::Ignored;
            }
            let continue_propagation = !untrack(|| state.list.selection.is_empty())
                || untrack(|| state.input_value()).is_empty()
                || untrack(|| state.allows_custom_value());
            state.revert();
            ShortcutOutcome::Custom {
                prevent_default: false,
                continue_propagation,
            }
        });
    // The arrow keys also act on auto-repeated key presses (react-aria: `allowRepeats`), but not
    // while an input method editor composes text.
    let arrow_shortcuts = KeyboardShortcuts::new()
        .on(Shortcut::new(KeyboardKey::ArrowDown), move |_| {
            state.open(Some(FocusStrategy::First), MenuTriggerAction::Manual);
            ShortcutOutcome::Custom {
                prevent_default: false,
                continue_propagation: false,
            }
        })
        .on(Shortcut::new(KeyboardKey::ArrowUp), move |_| {
            state.open(Some(FocusStrategy::Last), MenuTriggerAction::Manual);
            ShortcutOutcome::Custom {
                prevent_default: false,
                continue_propagation: false,
            }
        })
        .on(Shortcut::new(KeyboardKey::ArrowLeft), move |_| {
            state.list.selection.set_focused_key(None, None);
            ShortcutOutcome::Custom {
                prevent_default: false,
                continue_propagation: false,
            }
        })
        .on(Shortcut::new(KeyboardKey::ArrowRight), move |_| {
            state.list.selection.set_focused_key(None, None);
            ShortcutOutcome::Custom {
                prevent_default: false,
                continue_propagation: false,
            }
        });

    // Focus moving between the input, the button and the popover keeps the combo box focused.
    let button_id = menu_trigger.button.id.clone().unwrap_or_default();
    let on_input_blur = Callback::new(move |e: FocusEvent| {
        let related = e
            .related_target()
            .and_then(|t| t.dyn_into::<web_sys::Node>().ok());
        let within = |element: Option<web_sys::Element>| {
            element.is_some_and(|el| related.as_ref().is_some_and(|r| el.contains(Some(r))))
        };
        if within(button_element.get_untracked().map(|el| (*el).clone()))
            || within(popover.get_untracked().map(|el| (*el).clone()))
        {
            return;
        }
        if let Some(on_blur) = on_blur {
            on_blur.try_run(e);
        }
        state.set_focused(false);
    });
    let on_input_focus = Callback::new(move |e: FocusEvent| {
        if untrack(|| state.is_focused()) {
            return;
        }
        if let Some(on_focus) = on_focus {
            on_focus.run(e);
        }
        state.set_focused(true);
    });

    // The focused option is the input's active descendant.
    let focused_option_id = listbox_id.clone();
    let aria_activedescendant = Signal::derive(move || {
        if !state.is_open() {
            return None;
        }
        state
            .list
            .selection
            .focused_key()
            .filter(|key| state.list.collection.with(|c| c.contains_key(key)))
            .map(|key| option_id(&focused_option_id, &key))
    });
    let controls_id = listbox_id.clone();

    // Form reset restores the default value.
    use_form_reset(UseFormResetInput {
        element: input_element,
        initial_value: state.default_value(),
        on_reset: Callback::new(move |value| state.set_value(value)),
    });

    // Re-show the focus ring when no item is virtually focused any more (react-aria): a virtual
    // focus event on the focused input lets its focus ring show again.
    Effect::new(move |previous: Option<bool>| {
        let has_focused_item = state
            .list
            .selection
            .focused_key()
            .is_some_and(|key| state.list.collection.with(|c| c.contains_key(&key)));
        if previous.is_some_and(|had| had != has_focused_item)
            && !has_focused_item
            && let Some(input) = input_element.get_untracked()
            && input
                .owner_document()
                .as_ref()
                .and_then(get_active_element)
                .as_ref()
                == Some(&*input)
        {
            dispatch_virtual_focus(&input, None);
        }
        has_focused_item
    });

    announce_changes(&state);

    let text_field_state = use_text_field_state(UseTextFieldStateInput {
        default_value: String::new(),
        value: Some(crate::ValueBinding::new(
            state.input_value_signal(),
            Callback::new(move |value| state.set_input_value(value)),
        )),
        on_change: None,
    });
    let text_field = UseTextFieldInput {
        id: Some(input_id.clone()),
        is_disabled,
        is_read_only,
        // In multiple mode, the selection satisfies `required` once it has a value.
        is_required: match state.selection_mode {
            SelectMode::Single => is_required,
            SelectMode::Multiple => {
                Signal::derive(move || is_required.get() && state.list.selection.is_empty())
            }
        },
        is_invalid: state.validation.is_invalid,
        // The input carries the name only when it submits its text; else hidden inputs do.
        name: (form_value == ComboBoxFormValue::Text)
            .then(|| name.clone())
            .flatten(),
        placeholder,
        auto_complete: Some("off".to_owned()),
        auto_correct: Some(false),
        spell_check: Some(false),
        label_id: Some(label_id.clone()),
        has_label,
        aria_label,
        aria_labelledby: aria_labelledby.clone(),
        aria_describedby,
        aria_activedescendant,
        aria_autocomplete: Some(AriaAutocomplete::List),
        aria_controls: Signal::derive(move || state.is_open().then(|| controls_id.clone())),
        on_focus: Some(on_input_focus),
        on_blur: Some(on_input_blur),
        on_key_down: Some(Callback::new(move |e: KeyboardEventWrapper| {
            if is_read_only.get_untracked() {
                e.continue_propagation();
                return;
            }
            if untrack(|| state.is_open()) {
                collection_keydown.call(e.event().clone());
            }
            if !e.event().is_composing()
                && arrow_shortcuts
                    .handle(e.event())
                    .is_some_and(|outcome| !outcome.continue_propagation())
            {
                return;
            }
            e.continue_propagation();
        })),
        shortcuts: Some(shortcuts),
        state: text_field_state,
        element: TextFieldElement::Input,
        input_type: Signal::stored(InputType::Text),
        // The combo box validates its value and text together (react-aria:
        // `privateValidationStateProp`): the input shows the state's validation, and in native
        // mode reports it to the form.
        validate: None,
        validation_behavior: state.validation_behavior(),
        validation: Some(state.validation),
        form: form.clone(),
        pattern: None,
        min_length: None,
        max_length: None,
        auto_capitalize: None,
        input_mode: Signal::stored(None),
        enter_key_hint: None,
        auto_focus: false,
        exclude_from_tab_order: false,
        aria_errormessage: None,
        aria_haspopup: None,
        on_focus_change: None,
        on_key_up: None,
    };

    // Pressing the button focuses the input and toggles the popover.
    // The button and the listbox are named by the field's labels: the visible label only while
    // there is one.
    let field_labelledby = {
        let label_id = label_id.clone();
        Signal::derive(move || {
            aria_labelledby
                .clone()
                .or_else(|| has_label.get().then(|| label_id.clone()))
        })
    };
    let strings = use_localized_strings::<ComboBoxStrings>();
    // Named by their own label and the field's (react-aria: `useLabels`).
    let button_labels = Memo::new(move |_| {
        labels(
            &button_id,
            Some(strings.read().button_label()),
            field_labelledby.read().as_deref(),
        )
    });
    let listbox_labels = {
        let listbox_id = listbox_id.clone();
        Memo::new(move |_| {
            labels(
                &listbox_id,
                Some(strings.read().listbox_label()),
                field_labelledby.read().as_deref(),
            )
        })
    };
    let button = UseButtonInput {
        aria_label: Signal::derive(move || button_labels.read().aria_label.clone()).into(),
        aria_labelledby: Signal::derive(move || button_labels.read().aria_labelledby.clone()),
        exclude_from_tab_order: Signal::stored(true),
        prevent_focus_on_press: true.into(),
        is_disabled: Signal::derive(move || is_disabled.get() || is_read_only.get()),
        on_press: Some(Callback::new(move |e: PressEvent| {
            if e.pointer_type == PointerType::Touch {
                focus_input();
                state.toggle(None, MenuTriggerAction::Manual);
            }
        })),
        on_press_start: Some(Callback::new(move |e: PressEvent| {
            if e.pointer_type != PointerType::Touch {
                focus_input();
                let strategy =
                    matches!(e.pointer_type, PointerType::Keyboard | PointerType::Virtual)
                        .then_some(FocusStrategy::First);
                state.toggle(strategy, MenuTriggerAction::Manual);
            }
        })),
        ..menu_trigger.button
    };

    // Tapping the center of the input (what assistive technology does) toggles the popover.
    let last_touch = StoredValue::new(0.0_f64);
    let on_touchend = EventHandler::new(move |e: TouchEvent| {
        if is_disabled.get_untracked() || is_read_only.get_untracked() {
            return;
        }
        if e.time_stamp() - last_touch.get_value() < 500.0 {
            e.prevent_default();
            focus_input();
            return;
        }
        let Some(target) = get_event_target(&e).and_then(|t| t.dyn_into::<web_sys::Element>().ok())
        else {
            return;
        };
        let rect = target.get_bounding_client_rect();
        let Some(touch) = e.changed_touches().get(0) else {
            return;
        };
        let center_x = (rect.left() + 0.5 * rect.width()).ceil();
        let center_y = (rect.top() + 0.5 * rect.height()).ceil();
        if f64::from(touch.client_x()) == center_x && f64::from(touch.client_y()) == center_y {
            e.prevent_default();
            focus_input();
            state.toggle(None, MenuTriggerAction::Manual);
            last_touch.set_value(e.time_stamp());
        }
    });

    // While open, everything but the input and the popover is hidden from assistive technology.
    #[cfg(not(feature = "ssr"))]
    {
        use leptos::prelude::LocalStorage;

        use crate::utils::aria_hide_outside::{AriaHideOutsideOptions, aria_hide_outside};

        let undo: StoredValue<Option<Box<dyn FnOnce()>>, LocalStorage> =
            StoredValue::new_local(None);
        let restore = move || {
            undo.update_value(|undo| {
                if let Some(undo) = undo.take() {
                    undo();
                }
            });
        };
        Effect::new(move |_| {
            restore();
            if !state.is_open() {
                return;
            }
            let targets: Vec<web_sys::Element> = [
                input_element.get().map(|el| (*el).clone()),
                popover.get().map(|el| (*el).clone()),
            ]
            .into_iter()
            .flatten()
            .collect();
            if !targets.is_empty() {
                undo.set_value(Some(aria_hide_outside(
                    &targets,
                    AriaHideOutsideOptions::default(),
                )));
            }
        });
        on_cleanup(restore);
    }

    let listbox = UseListBoxInput {
        id: Some(listbox_id),
        aria_label: Signal::derive(move || listbox_labels.read().aria_label.clone()).into(),
        aria_labelledby: Signal::derive(move || listbox_labels.read().aria_labelledby.clone()),
        options: CollectionOptions {
            auto_focus: Signal::derive(move || {
                Some(match state.focus_strategy() {
                    Some(FocusStrategy::First) => AutoFocus::First,
                    Some(FocusStrategy::Last) => AutoFocus::Last,
                    None => AutoFocus::Selected,
                })
            }),
            should_use_virtual_focus: true,
            link_behavior: Signal::stored(LinkBehavior::Selection),
            ..CollectionOptions::default()
        },
        should_select_on_press_up: true,
        should_focus_on_hover: true,
        state: state.list,
        element: listbox_element,
        orientation: Orientation::Vertical.into(),
        layout: ListLayout::Stack,
        layout_delegate: None,
        is_virtualized: false,
        keyboard_delegate: None,
        on_action: None,
        on_focus: None,
        on_blur: None,
        on_focus_change: None,
    };

    let form_values = Signal::derive(move || {
        if form_value != ComboBoxFormValue::Key || name.is_none() {
            return Vec::new();
        }
        let values: Vec<String> = state.value().iter().map(Key::to_string).collect();
        if values.is_empty() {
            vec![String::new()]
        } else {
            values
        }
    });

    UseComboBoxReturn {
        form_values,
        input: text_field,
        input_props: UseComboBoxInputProps {
            role: AriaRole::Combobox,
            aria_expanded: Signal::derive(move || Some(AriaExpanded::from(state.is_open()))),
            on_touchend,
            element_capture: input_element.attr(),
        },
        button,
        listbox,
    }
}

/// Screen reader announcements (react-aria `useComboBox`): the number of options when the
/// popover opens without a focused option or the number changes; on Apple devices, whose
/// VoiceOver doesn't announce `aria-activedescendant` changes reliably, also the focused option
/// (with the section it enters) and the selection.
fn announce_changes(state: &ComboBoxState) {
    use crate::{
        hooks::collections::NodeKind,
        utils::{live_announcer::announce_assertive, platform::device::is_apple_device},
    };

    let state = *state;
    let strings = use_localized_strings::<ComboBoxStrings>();

    let option_text = |node: &crate::hooks::collections::Node| {
        node.aria_label
            .as_deref()
            .unwrap_or(&node.text_value)
            .to_owned()
    };
    // The focused option, and the section it is in.
    let last_section = StoredValue::new(None::<Key>);
    let last_item = StoredValue::new(None::<Key>);
    Effect::new(move |_| {
        let is_open = state.is_open();
        let item_key = state.list.selection.focused_key();
        let collection = state.list.collection.get();
        untrack(|| {
            let focused = item_key
                .as_ref()
                .filter(|_| is_open)
                .and_then(|key| collection.get(key));
            let section_key = focused.and_then(|node| node.parent_key.clone());
            if is_apple_device()
                && let Some(focused) = focused
                && item_key != last_item.get_value()
            {
                let section = section_key
                    .as_ref()
                    .and_then(|key| collection.get(key))
                    .filter(|node| node.kind == NodeKind::Section);
                let mut group = None;
                if let Some(section) = section
                    && section_key != last_section.get_value()
                {
                    let title = section.aria_label.as_deref().map_or_else(
                        || {
                            collection
                                .children(&section.key)
                                .find(|node| node.kind == NodeKind::Header)
                                .map(|header| header.text_value.to_string())
                                .unwrap_or_default()
                        },
                        ToOwned::to_owned,
                    );
                    let count = collection
                        .children(&section.key)
                        .filter(|node| node.is_item())
                        .count();
                    group = Some((title, count));
                }
                let (title, count) = group.clone().unwrap_or_default();
                announce_assertive(strings.read().focus_announcement(FocusAnnouncementArgs {
                    is_group_change: group.is_some(),
                    group_title: &title,
                    group_count: count,
                    option_text: &option_text(focused),
                    is_selected: state.list.selection.is_selected(&focused.key),
                }));
            }
            last_section.set_value(section_key);
            last_item.set_value(item_key);
        });
    });

    // The number of options.
    let last_size = StoredValue::new(None::<usize>);
    let last_open = StoredValue::new(false);
    Effect::new(move |_| {
        let is_open = state.is_open();
        let count = state.list.collection.with(|c| c.size());
        let has_focused_key = state.list.selection.focused_key().is_some();
        untrack(|| {
            let did_open_without_focused_item =
                is_open != last_open.get_value() && (!has_focused_key || is_apple_device());
            if is_open && (did_open_without_focused_item || last_size.get_value() != Some(count)) {
                announce_assertive(strings.read().count_announcement(count));
            }
            last_size.set_value(Some(count));
            last_open.set_value(is_open);
        });
    });

    // The selection (other screen readers announce it themselves).
    let last_selected = StoredValue::new(untrack(|| state.selected_key()));
    Effect::new(move |_| {
        let selected_key = state.selected_key();
        let is_focused = state.is_focused();
        untrack(|| {
            if is_apple_device()
                && is_focused
                && selected_key.is_some()
                && selected_key != last_selected.get_value()
                && let Some(item) = state.selected_items().first()
            {
                announce_assertive(strings.read().selected_announcement(&option_text(item)));
            }
            last_selected.set_value(selected_key);
        });
    });
}
