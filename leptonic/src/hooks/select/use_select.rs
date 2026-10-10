// Upstream: react-aria/src/select/useSelect.ts @ 99e6102368
// Upstream: react-aria-components/test/Select.test.js @ 99e6102368
// Upstream: react-aria-components/test/Select.ssr.test.js @ 99e6102368
// Upstream: @adobe/react-spectrum/test/picker/Picker.test.js @ 99e6102368
use std::sync::Arc;

use leptos::{
    attr::{self, Attr},
    ev,
    prelude::*,
};
use leptos_use::use_document;
use web_sys::{FocusEvent, KeyboardEvent, MouseEvent};

use super::{SelectMode, SelectState, UseHiddenSelectInput};
use crate::{
    CapturedElement, EventHandler, IdRefs, IntoAttrs, OnEvent, Propagation, SlotProps,
    hooks::{
        button::use_button::UseButtonInput,
        collections::{
            AutoFocus, CollectionOptions, FocusStrategy, KeyboardDelegate, LinkBehavior,
            ListLayout, NavigationOptions, UseListKeyboardDelegateInput, UseTypeSelectInput,
            UseTypeSelectProps, use_list_keyboard_delegate, use_type_select,
        },
        focus::{
            use_focus_visible::{Modality, set_modality},
            use_focus_within::FocusWithinEvent,
        },
        form::{
            use_field::{UseFieldInput, UseFieldReturn, use_field},
            use_label::LabelElementType,
        },
        interactions::use_keyboard::KeyboardEventWrapper,
        listbox::UseListBoxInput,
        menu::use_menu_trigger::{MenuTriggerType, UseMenuTriggerInput, use_menu_trigger},
        overlay::use_overlay_trigger::OverlayTriggerType,
    },
    utils::{
        focus::focus_element,
        key::KeyboardKey,
        keyboard_shortcut::{KeyboardShortcuts, Shortcut},
        orientation::Orientation,
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - The trigger is configured, not rendered: `trigger` is the `UseButtonInput` for
//   `use_button`, and `trigger_props` (capture-phase type-ahead) is spread next to the button's
//   props. Likewise `listbox` is the `UseListBoxInput` for the popover's `use_listbox`, and
//   `hidden_select` the input of `use_hidden_select`.
// - `has_label` says whether a visible label is rendered (react-aria: the `label` content).
// - `name` and `validation_behavior` are read from the state (C8).
//
// =============================================================================

/// Input of [`use_select`].
#[derive(Clone, Debug)]
pub struct UseSelectInput {
    pub state: SelectState,
    /// The trigger element's id. Generated when `None`.
    pub id: Option<String>,
    pub is_disabled: Signal<bool>,
    pub is_required: Signal<bool>,
    /// Whether a visible label is rendered (with `label_props`).
    pub has_label: Signal<bool>,
    pub aria_label: MaybeProp<String>,
    pub aria_labelledby: Option<String>,
    pub aria_describedby: Option<String>,
    /// Replaces the list keyboard delegate (type-ahead and arrow keys on the trigger).
    pub keyboard_delegate: Option<Signal<Arc<dyn KeyboardDelegate>>>,
    /// Called when focus moves to the select (trigger or popover) from outside.
    pub on_focus: Option<Callback<FocusEvent>>,
    /// Called when focus leaves the select.
    pub on_blur: Option<Callback<FocusEvent>>,
    pub on_focus_change: Option<Callback<bool>>,
    /// The id of the form the select belongs to, if it is outside of it.
    pub form: Option<String>,
}

/// Return value of [`use_select`].
pub struct UseSelectReturn {
    pub label_props: UseSelectLabelProps,
    /// The trigger button's configuration, for `use_button`.
    pub trigger: UseButtonInput,
    /// Spread onto the trigger in addition to the button's props.
    pub trigger_props: UseSelectTriggerProps,
    /// For the element showing the selected value inside the trigger.
    pub value_props: UseSelectValueProps,
    pub description_props: SlotProps,
    pub error_message_props: SlotProps,
    /// The popover's listbox, for `use_listbox`.
    pub listbox: UseListBoxInput,
    /// The native form element mirroring the value, for `use_hidden_select`.
    pub hidden_select: UseHiddenSelectInput,
    pub is_invalid: Signal<bool>,
    pub validation_errors: Signal<Vec<String>>,
}

/// Props for the label element.
#[derive(Debug, Clone)]
pub struct UseSelectLabelProps {
    pub id: String,
    /// Focuses the trigger.
    pub on_click: EventHandler<MouseEvent>,
}

pub type UseSelectLabelAttrs = (Attr<attr::Id, String>, OnEvent<ev::click>);

impl IntoAttrs for UseSelectLabelProps {
    type Attrs = UseSelectLabelAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (Attr(attr::Id, self.id), self.on_click.into_on(ev::click))
    }
}

/// Trigger props in addition to the button's.
#[derive(Debug, Clone)]
pub struct UseSelectTriggerProps {
    /// Type-ahead: a space continues a search instead of opening the popover.
    pub on_keydown_capture: EventHandler<KeyboardEvent>,
}

pub type UseSelectTriggerAttrs = (OnEvent<ev::Capture<ev::keydown>>,);

impl IntoAttrs for UseSelectTriggerProps {
    type Attrs = UseSelectTriggerAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (self.on_keydown_capture.into_on(ev::capture(ev::keydown)),)
    }
}

/// Props for the value element.
#[derive(Debug)]
pub struct UseSelectValueProps {
    pub id: String,
}

pub type UseSelectValueAttrs = (Attr<attr::Id, String>,);

impl IntoAttrs for UseSelectValueProps {
    type Attrs = UseSelectValueAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (Attr(attr::Id, self.id),)
    }
}

/// A select: a button showing the selected option that opens a listbox popover. The trigger
/// supports type-ahead and (in single selection mode) changing the value with ArrowLeft and
/// ArrowRight without opening the popover.
#[allow(clippy::too_many_lines)]
pub fn use_select(input: UseSelectInput) -> UseSelectReturn {
    let UseSelectInput {
        state,
        id,
        is_disabled,
        is_required,
        has_label,
        aria_label,
        aria_labelledby,
        aria_describedby,
        keyboard_delegate,
        on_focus,
        on_blur,
        on_focus_change,
        form,
    } = input;

    let listbox_element = CapturedElement::new();
    let delegate = keyboard_delegate.unwrap_or_else(|| {
        use_list_keyboard_delegate(UseListKeyboardDelegateInput {
            state: state.list,
            element: listbox_element,
            orientation: Orientation::Vertical.into(),
            layout: ListLayout::Stack,
            layout_delegate: None,
        })
    });

    let menu_trigger = use_menu_trigger(UseMenuTriggerInput {
        menu_type: OverlayTriggerType::Listbox,
        is_disabled,
        trigger: MenuTriggerType::Press,
        state,
    });

    // -- Changing the value from the closed trigger --
    let single = state.selection_mode == SelectMode::Single;
    let step = move |forward: bool| {
        if !single {
            return false;
        }
        let delegate = delegate.get_untracked();
        let key = match untrack(|| state.selected_key()) {
            Some(current) if forward => delegate.key_below(&current, NavigationOptions::default()),
            Some(current) => delegate.key_above(&current, NavigationOptions::default()),
            None => delegate.first_key(None, false),
        };
        if let Some(key) = key {
            state.set_value(vec![key]);
        }
        true
    };
    // Also while an arrow key is held (the button's own shortcuts ignore key repeats).
    let arrows = KeyboardShortcuts::new()
        .on(Shortcut::new(KeyboardKey::ArrowLeft), move |_| step(false))
        .on(Shortcut::new(KeyboardKey::ArrowRight), move |_| step(true));

    let UseTypeSelectProps {
        on_keydown_capture,
        on_keydown: type_select_keydown,
    } = if single {
        use_type_select(UseTypeSelectInput {
            delegate,
            selection: state.list.selection,
            on_type_select: Some(Callback::new(move |key| state.set_value(vec![key]))),
        })
        .props
    } else {
        UseTypeSelectProps {
            on_keydown_capture: EventHandler::empty(),
            on_keydown: EventHandler::empty(),
        }
    };

    // -- Labelling --
    let has_aria_label = Signal::derive(move || aria_label.with(Option::is_some));
    let UseFieldReturn {
        label_props,
        field_props,
        description_props,
        error_message_props,
        ..
    } = use_field(UseFieldInput {
        id,
        label_id: None,
        has_label,
        label_element_type: LabelElementType::Span,
        aria_label,
        aria_labelledby,
        aria_describedby,
    });
    let trigger_id = field_props.id.clone();
    let value_id = crate::utils::id::use_id("select-value");
    let join =
        |ids: Vec<Option<String>>| ids.into_iter().flatten().collect::<IdRefs>().into_value();
    // Labelled by aria-label (the trigger itself), unless other labels exist. Both follow the
    // field's labels (e.g. whether its label is rendered).
    let field_labelledby = field_props.aria_labelledby;
    let self_label = {
        let trigger_id = trigger_id.clone();
        move || {
            (has_aria_label.get() && field_labelledby.with(Option::is_none))
                .then(|| trigger_id.clone())
        }
    };
    let trigger_labelledby = {
        let (value_id, self_label) = (value_id.clone(), self_label.clone());
        Signal::derive(move || {
            join(vec![
                Some(value_id.clone()),
                field_labelledby.get(),
                self_label(),
            ])
        })
    };
    let listbox_labelledby =
        Signal::derive(move || join(vec![field_labelledby.get(), self_label()]));

    // -- Focus --
    let trigger_on_focus = Callback::new(move |e: FocusEvent| {
        if untrack(|| state.is_focused()) {
            return;
        }
        if let Some(on_focus) = on_focus {
            on_focus.run(e);
        }
        if let Some(on_focus_change) = on_focus_change {
            on_focus_change.run(true);
        }
        state.set_focused(true);
    });
    // Focus moving to the popover keeps the select focused.
    let trigger_on_blur = Callback::new(move |e: FocusEvent| {
        if untrack(|| state.is_open()) {
            return;
        }
        if let Some(on_blur) = on_blur {
            on_blur.try_run(e);
        }
        if let Some(on_focus_change) = on_focus_change {
            on_focus_change.run(false);
        }
        state.set_focused(false);
    });
    let listbox_on_blur = Callback::new(move |e: FocusWithinEvent| {
        if let Some(on_blur) = on_blur {
            on_blur.try_run(e.event.clone());
        }
        if let Some(on_focus_change) = on_focus_change {
            on_focus_change.run(false);
        }
        state.set_focused(false);
    });

    let trigger = UseButtonInput {
        id: Some(trigger_id.clone()),
        aria_label,
        aria_labelledby: trigger_labelledby,
        aria_describedby: field_props.aria_describedby,
        on_key_down: Some(Callback::new(move |e: KeyboardEventWrapper| {
            // A handled arrow key stops (react-aria: a shortcut handled by `useKeyboard`); every
            // other key bubbles on.
            let handled = arrows.handle(e.event()).is_some_and(|outcome| {
                if outcome.prevent_default() {
                    e.event().prevent_default();
                }
                !outcome.continue_propagation()
            });
            type_select_keydown.call(e.event().clone());
            if !handled {
                e.continue_propagation();
            }
        })),
        on_focus: Some(trigger_on_focus),
        on_blur: Some(trigger_on_blur),
        ..menu_trigger.button
    };

    let label_trigger_id = trigger_id.clone();
    let label_on_click = EventHandler::new(move |_: MouseEvent| {
        if is_disabled.get_untracked() {
            return;
        }
        if let Some(trigger) = use_document()
            .as_ref()
            .and_then(|d| d.get_element_by_id(&label_trigger_id))
        {
            focus_element(&trigger, false);
            set_modality(Modality::Keyboard);
        }
    });

    let listbox = UseListBoxInput {
        id: Some(menu_trigger.menu_props.id.clone()),
        aria_labelledby: listbox_labelledby,
        options: CollectionOptions {
            auto_focus: Signal::derive(move || {
                Some(match state.focus_strategy() {
                    Some(FocusStrategy::First) => AutoFocus::First,
                    Some(FocusStrategy::Last) => AutoFocus::Last,
                    None => AutoFocus::Selected,
                })
            }),
            disallow_empty_selection: true,
            link_behavior: Signal::stored(LinkBehavior::Selection),
            ..CollectionOptions::default()
        },
        should_select_on_press_up: true,
        should_focus_on_hover: true,
        on_blur: Some(listbox_on_blur),
        state: state.list,
        element: listbox_element,
        aria_label: MaybeProp::default(),
        orientation: Orientation::Vertical.into(),
        layout: ListLayout::Stack,
        layout_delegate: None,
        is_virtualized: false,
        keyboard_delegate: None,
        on_action: None,
        on_focus: None,
        on_focus_change: None,
    };

    UseSelectReturn {
        label_props: UseSelectLabelProps {
            id: label_props.id,
            on_click: label_on_click,
        },
        trigger,
        trigger_props: UseSelectTriggerProps { on_keydown_capture },
        value_props: UseSelectValueProps { id: value_id },
        description_props,
        error_message_props,
        listbox,
        hidden_select: UseHiddenSelectInput {
            state,
            form,
            auto_complete: None,
            is_disabled,
            is_required,
            trigger: None,
            label: aria_label,
        },
        is_invalid: state.validation.is_invalid,
        validation_errors: state.validation.validation_errors,
    }
}
