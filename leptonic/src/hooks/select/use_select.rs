use std::collections::HashSet;

use leptos::{
    attr,
    attr::Attr,
    ev,
    ev::{On, SharedEventCallback},
    prelude::*,
};
use uuid::Uuid;
use wasm_bindgen::JsCast;
use web_sys::KeyboardEvent;

use crate::{
    hooks::{
        IntoAttrs,
        form::{
            use_field::{UseFieldInput, ValidationState, use_field},
            use_form_validation_state::{
                UseFormValidationStateInput, UseFormValidationStateReturn, ValidateFn,
                ValidationBehavior,
            },
        },
        selection::{
            SelectionKey,
            use_selectable_collection::EscapeKeyBehavior,
            use_selection_state::{
                DisabledBehavior, FocusStrategy, Selection, SelectionBehavior, SelectionMode,
                UseSelectionStateInput, UseSelectionStateReturn, use_selection_state,
            },
            use_type_select::{UseTypeSelectInput, use_type_select},
        },
    },
    utils::{
        EventHandler,
        aria::{AriaDisabled, AriaExpanded, AriaHasPopup, AriaInvalid, AriaRequired, AriaRole},
        dom_ext::node_contains,
        element_capture::{CapturedElement, ElementCaptureAttr},
        focus::focus_safely,
    },
};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/select/src/useSelect.ts

// REACT-ARIA DEVIATIONS
//
// ## API DIFFERENCES
//
// - Hook-owned state: Selection state is created and owned internally via
//   `use_selection_state`. Callers get read-only `Signal<T>` and semantic
//   callbacks instead of passing in an externally created state object.
// - `UseSelectMenuConfig` replaces react-aria's `menuProps: AriaListBoxOptions`.
//   The consumer constructs `UseListBoxInput` from this config.
// - Validation uses `use_form_validation_state` instead of
//   `useFormValidationState`. DOM-level validation (`use_form_validation`) is
//   left to the consumer since select has no native input to attach to.
// - `get_option_id` callback requires `K: Display` (matching `use_combobox`).
//
// ## IMPLICIT BEHAVIORAL MATCHES
//
// - `allowDuplicateSelectionEvents`: React-aria passes `true` so re-selecting
//   the same key still fires `onSelectionChange`. Leptonic's
//   `use_selection_state.select` callback does not deduplicate — it always
//   rebuilds the selection and fires the callback, matching this behavior
//   implicitly without a separate flag.
//
// ## OMITTED FEATURES
//
// - `useMenuTrigger` composition: Leptonic does not have a standalone
//   `use_menu_trigger` hook. Trigger behavior (keyboard shortcuts, press
//   handling) is implemented inline, matching the same semantics.
// - `filterDOMProps`: Leptos handles DOM attribute forwarding via the spread
//   syntax and typed `Attr` tuples, so no DOM-prop filtering is needed.
// - `useField` from `@react-aria/label`: Replaced by `use_field`
//   which provides the same ID generation.

/// Input parameters for the `use_select` hook.
#[allow(clippy::struct_excessive_bools)]
#[derive(Clone)]
pub struct UseSelectInput<K>
where
    K: SelectionKey,
{
    // -- Selection --
    /// The selection mode (single or multiple).
    pub selection_mode: SelectionMode,

    /// Whether the select is disabled.
    pub is_disabled: Signal<bool>,

    /// Whether the select is required.
    pub is_required: bool,

    /// The controlled selected keys.
    pub selected_keys: Option<Signal<Selection<K>>>,

    /// The default selected keys (uncontrolled).
    pub default_selected_keys: Option<Selection<K>>,

    /// Callback when selection changes.
    pub on_selection_change: Option<Callback<Selection<K>>>,

    /// Keys that cannot be selected.
    pub disabled_keys: Signal<HashSet<K>>,

    /// All available items.
    pub items: Signal<Vec<K>>,

    /// A function to get the text value for a key (used for display and type-ahead).
    pub get_text_value: Option<Callback<K, String>>,

    // -- Open state --
    /// Whether the menu is open (controlled).
    pub is_open: Option<Signal<bool>>,

    /// The default open state (uncontrolled).
    pub default_open: bool,

    /// Callback when open state changes.
    pub on_open_change: Option<Callback<bool>>,

    /// Whether to allow opening the menu when the collection is empty.
    pub allows_empty_collection: bool,

    // -- Label / description --
    /// Visible label text (used to determine label ID linking).
    pub label: Option<&'static str>,

    /// An accessibility label for the select (when no visible label is present).
    pub aria_label: Option<&'static str>,

    /// The ID of an element that labels the select.
    pub aria_labelledby: Option<String>,

    /// Whether the consumer renders a description element.
    pub has_description: bool,

    // -- Validation --
    /// Controlled invalid state. Overrides all other validation sources.
    pub is_invalid: Option<Signal<bool>>,

    /// Custom client-side validation function.
    pub validate: Option<ValidateFn<Selection<K>>>,

    /// Validation behavior mode.
    pub validation_behavior: ValidationBehavior,

    // -- Form --
    /// The name attribute for form submission.
    pub name: Option<&'static str>,

    // -- Focus --
    /// Callback when the select trigger receives focus.
    pub on_focus: Option<Callback<()>>,

    /// Callback when the select trigger loses focus.
    pub on_blur: Option<Callback<()>>,

    /// Callback when focus state changes.
    pub on_focus_change: Option<Callback<bool>>,

    // -- Keyboard --
    /// User keydown handler (chained after internal handling).
    pub on_key_down: Option<Callback<KeyboardEvent>>,

    /// User keyup handler.
    pub on_key_up: Option<Callback<KeyboardEvent>>,

    /// Placeholder text when nothing is selected.
    pub placeholder: Option<&'static str>,
}

impl<K: SelectionKey> Default for UseSelectInput<K> {
    fn default() -> Self {
        Self {
            selection_mode: SelectionMode::Single,
            is_disabled: Signal::derive(|| false),
            is_required: false,
            selected_keys: None,
            default_selected_keys: None,
            on_selection_change: None,
            disabled_keys: Signal::derive(HashSet::new),
            items: Signal::derive(Vec::new),
            get_text_value: None,
            is_open: None,
            default_open: false,
            on_open_change: None,
            allows_empty_collection: false,
            label: None,
            aria_label: None,
            aria_labelledby: None,
            has_description: false,
            is_invalid: None,
            validate: None,
            validation_behavior: ValidationBehavior::Aria,
            name: None,
            on_focus: None,
            on_blur: None,
            on_focus_change: None,
            on_key_down: None,
            on_key_up: None,
            placeholder: None,
        }
    }
}

/// The return value of the `use_select` hook.
pub struct UseSelectReturn<K>
where
    K: SelectionKey,
{
    // -- Props for elements --
    /// Props for the trigger button element.
    pub trigger_props: UseSelectTriggerProps,

    /// Props for the value display element.
    pub value_props: UseSelectValueProps,

    /// Props for the label element (from `use_field`).
    pub label_props: UseSelectLabelProps,

    /// Props for the description element (from `use_field`).
    pub description_props: crate::hooks::form::use_field::UseFieldDescriptionProps,

    /// Props for the error message element (from `use_field`).
    pub error_message_props: crate::hooks::form::use_field::UseFieldErrorMessageProps,

    /// Props for a hidden input for form submission.
    pub hidden_select_props: UseSelectHiddenProps,

    // -- IDs --
    /// The ID of the trigger element.
    pub trigger_id: String,

    /// The ID of the listbox element.
    pub listbox_id: String,

    /// The ID of the label element.
    pub label_id: String,

    /// The ID of the value display element.
    pub value_id: String,

    // -- State (read-only) --
    /// Whether the menu is open.
    pub is_open: Signal<bool>,

    /// The currently selected keys.
    pub selected_keys: Signal<Selection<K>>,

    /// The display value for the selected item(s).
    pub display_value: Signal<Option<String>>,

    /// Whether the trigger is focused.
    pub is_focused: Signal<bool>,

    /// The current focus strategy for the listbox.
    pub focus_strategy: Signal<Option<FocusStrategy>>,

    /// The currently focused key in the listbox.
    pub focused_key: Signal<Option<K>>,

    // -- Validation (read-only) --
    /// Whether the select is invalid.
    pub is_invalid: Signal<bool>,

    /// Validation error messages.
    pub validation_errors: Signal<Vec<String>>,

    /// Full validation state for advanced use.
    pub validation: UseFormValidationStateReturn,

    // -- Selection state --
    /// The selection state. Pass to `use_option` for per-item rendering.
    pub selection_state: UseSelectionStateReturn<K>,

    // -- Callbacks --
    /// Open the menu.
    pub open: Callback<()>,

    /// Close the menu.
    pub close: Callback<()>,

    /// Toggle the menu.
    pub toggle: Callback<()>,

    /// Set the selected keys programmatically.
    pub set_selected_keys: Callback<Selection<K>>,

    /// Returns the stable DOM element ID for a listbox option with the given key.
    /// Use this as the `id` attribute on each rendered option element so that
    /// `aria-activedescendant` correctly references the focused option.
    pub get_option_id: Callback<K, String>,

    // -- Element references --
    /// The captured trigger element. Use this for overlay positioning
    /// (pass to `UseOverlayPositionInput::target`).
    pub trigger_element: CapturedElement,

    // -- Listbox configuration --
    /// Configuration for the dropdown listbox. Pass these values when constructing
    /// `UseListBoxInput` for the dropdown menu.
    pub menu_config: UseSelectMenuConfig<K>,
}

/// Configuration for the listbox dropdown, derived from `use_select` state.
///
/// The consumer uses these values to construct `UseListBoxInput` for the dropdown.
#[allow(clippy::struct_excessive_bools)]
#[derive(Clone)]
pub struct UseSelectMenuConfig<K>
where
    K: SelectionKey,
{
    /// The selection mode.
    pub selection_mode: SelectionMode,

    /// Controlled selection signal (from `use_select`'s internal state).
    pub selected_keys: Signal<Selection<K>>,

    /// Callback that routes selection changes through `use_select`.
    pub on_selection_change: Callback<Selection<K>>,

    /// Disabled keys.
    pub disabled_keys: Signal<HashSet<K>>,

    /// Whether to disallow empty selection.
    pub disallow_empty_selection: bool,

    /// Reactive auto-focus strategy for the listbox.
    pub auto_focus: Signal<Option<FocusStrategy>>,

    /// Whether keyboard navigation should wrap.
    pub should_focus_wrap: bool,

    /// Whether to select items on focus.
    pub select_on_focus: bool,

    /// Whether selection should occur on pointer-up rather than pointer-down.
    /// React-aria sets this to `true` for select menus.
    pub should_select_on_press_up: bool,

    /// Whether items should receive focus on mouse hover.
    /// React-aria sets this to `true` for select menus.
    pub should_focus_on_hover: bool,

    /// Whether options should use virtual focus (aria-activedescendant) instead of real DOM focus.
    /// `false` for standard select menus (real DOM focus), `true` for combobox patterns.
    pub should_use_virtual_focus: bool,

    /// Escape key behavior for the listbox. `None` for select menus (Escape closes the menu
    /// instead of clearing selection).
    pub escape_key_behavior: EscapeKeyBehavior,

    /// Callback to close the menu (invoked on Escape when listbox has focus).
    pub on_close: Option<Callback<()>>,

    /// The composed aria-labelledby value for the listbox.
    pub aria_labelledby: String,

    /// All available items.
    pub items: Signal<Vec<K>>,

    /// Text value callback for type-ahead.
    pub get_text_value: Option<Callback<K, String>>,

    /// Blur handler for the listbox — the consumer should spread this onto the listbox
    /// container so that `is_focused` resets when focus leaves the menu entirely.
    pub on_blur: EventHandler<web_sys::FocusEvent>,

    /// Setter for the focused key — the consumer can write the listbox's `focused_key`
    /// into this signal to keep `select.focused_key` in sync with the listbox.
    ///
    /// ```ignore
    /// Effect::new(move || {
    ///     select.menu_config.set_focused_key
    ///         .set(listbox.state.collection.selection_state.focused_key.get());
    /// });
    /// ```
    pub set_focused_key: WriteSignal<Option<K>>,
}

/// Props from `use_select` for the trigger element.
#[derive(Debug, Clone)]
pub struct UseSelectTriggerProps {
    pub id: String,
    pub role: AriaRole,
    pub tabindex: &'static str,
    pub aria_haspopup: AriaHasPopup,
    pub aria_expanded: Signal<Option<AriaExpanded>>,
    pub aria_controls: String,
    pub aria_label: Option<&'static str>,
    pub aria_labelledby: Option<String>,
    pub aria_required: Option<AriaRequired>,
    pub aria_disabled: Signal<Option<AriaDisabled>>,
    pub aria_describedby: Signal<Option<String>>,
    pub aria_invalid: Signal<Option<AriaInvalid>>,
    pub on_pointerdown: EventHandler<web_sys::PointerEvent>,
    pub on_pointerup: EventHandler<web_sys::PointerEvent>,
    pub on_keydown: EventHandler<KeyboardEvent>,
    pub on_keyup: EventHandler<KeyboardEvent>,
    pub on_focus: EventHandler<web_sys::FocusEvent>,
    pub on_blur: EventHandler<web_sys::FocusEvent>,
    pub element_capture: ElementCaptureAttr,
}

impl IntoAttrs for UseSelectTriggerProps {
    type Attrs = UseSelectTriggerAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Id, self.id),
            Attr(attr::Role, self.role),
            Attr(attr::Tabindex, self.tabindex),
            Attr(attr::AriaHaspopup, self.aria_haspopup),
            Attr(attr::AriaExpanded, self.aria_expanded),
            Attr(attr::AriaControls, self.aria_controls),
            Attr(attr::AriaLabel, self.aria_label),
            Attr(attr::AriaLabelledby, self.aria_labelledby),
            Attr(attr::AriaRequired, self.aria_required),
            Attr(attr::AriaDisabled, self.aria_disabled),
            Attr(attr::AriaDescribedby, self.aria_describedby),
            Attr(attr::AriaInvalid, self.aria_invalid),
            self.on_pointerdown.into_on(ev::pointerdown),
            self.on_pointerup.into_on(ev::pointerup),
            self.on_keydown.into_on(ev::keydown),
            self.on_keyup.into_on(ev::keyup),
            self.on_focus.into_on(ev::focusin),
            self.on_blur.into_on(ev::focusout),
            self.element_capture,
        )
    }
}

/// Attributes for the select trigger button.
pub type UseSelectTriggerAttrs = (
    Attr<attr::Id, String>,
    Attr<attr::Role, AriaRole>,
    Attr<attr::Tabindex, &'static str>,
    Attr<attr::AriaHaspopup, AriaHasPopup>,
    Attr<attr::AriaExpanded, Signal<Option<AriaExpanded>>>,
    Attr<attr::AriaControls, String>,
    Attr<attr::AriaLabel, Option<&'static str>>,
    Attr<attr::AriaLabelledby, Option<String>>,
    Attr<attr::AriaRequired, Option<AriaRequired>>,
    Attr<attr::AriaDisabled, Signal<Option<AriaDisabled>>>,
    Attr<attr::AriaDescribedby, Signal<Option<String>>>,
    Attr<attr::AriaInvalid, Signal<Option<AriaInvalid>>>,
    On<ev::pointerdown, SharedEventCallback<web_sys::PointerEvent>>,
    On<ev::pointerup, SharedEventCallback<web_sys::PointerEvent>>,
    On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
    On<ev::keyup, SharedEventCallback<KeyboardEvent>>,
    On<ev::focusin, SharedEventCallback<web_sys::FocusEvent>>,
    On<ev::focusout, SharedEventCallback<web_sys::FocusEvent>>,
    ElementCaptureAttr,
);

/// Props for the value display element.
#[derive(Debug, Clone)]
pub struct UseSelectValueProps {
    /// The id of the value element.
    pub id: String,
}

/// Props for the label element. Extends `UseFieldLabelProps` with an `on_click`
/// handler that focuses the trigger (since the label is a `<span>`, not `<label>`).
#[derive(Debug, Clone)]
pub struct UseSelectLabelProps {
    /// The id of the label element.
    pub id: String,
    /// The "for" attribute linking to the field.
    pub html_for: String,
    /// Click handler that focuses the trigger element.
    pub on_click: EventHandler<web_sys::MouseEvent>,
}

impl IntoAttrs for UseSelectLabelProps {
    type Attrs = UseSelectLabelAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Id, self.id),
            Attr(attr::For, self.html_for),
            self.on_click.into_on(ev::click),
        )
    }
}

/// Attributes for the select label element.
pub type UseSelectLabelAttrs = (
    Attr<attr::Id, String>,
    Attr<attr::For, String>,
    On<ev::click, SharedEventCallback<web_sys::MouseEvent>>,
);

/// Props for the hidden select element (for form submission).
#[derive(Debug, Clone)]
pub struct UseSelectHiddenProps {
    /// The name attribute.
    pub name: Option<&'static str>,

    /// Whether the field is required.
    pub required: bool,

    /// Whether the field is disabled.
    pub disabled: Signal<bool>,

    /// The selection mode.
    pub selection_mode: SelectionMode,

    /// The validation behavior.
    pub validation_behavior: ValidationBehavior,
}

/// Provides the behavior and accessibility implementation for a select component.
///
/// A select displays a collapsible list of options and allows a user to select
/// one or more of them. This hook composes `use_selection_state`,
/// `use_type_select`, `use_field`, and
/// `use_form_validation_state` to provide comprehensive behavior.
///
/// # Composition Pattern
///
/// `use_select` returns a `menu_config` that the consumer passes to
/// `use_listbox` for the dropdown. It also returns `selection_state` for
/// wiring up `use_option` on each item.
///
/// # Example
///
/// ```ignore
/// let items = Signal::derive(|| vec!["Apple", "Banana", "Cherry"]);
///
/// let select = use_select(UseSelectInput {
///     items,
///     aria_label: Some("Select a fruit"),
///     get_text_value: Some(Callback::new(|k: &str| k.to_string())),
///     ..Default::default()
/// });
///
/// let listbox = use_listbox(UseListBoxInput {
///     selection_mode: select.menu_config.selection_mode,
///     selected_keys: Some(select.menu_config.selected_keys),
///     on_selection_change: Some(select.menu_config.on_selection_change),
///     disabled_keys: select.menu_config.disabled_keys,
///     disallow_empty_selection: select.menu_config.disallow_empty_selection,
///     auto_focus: select.menu_config.auto_focus,
///     items: select.menu_config.items,
///     aria_labelledby: Some(select.menu_config.aria_labelledby),
///     get_text_value: select.menu_config.get_text_value,
///     ..Default::default()
/// });
///
/// // Wire the listbox's focused key back to the trigger for aria-activedescendant.
/// let set_focused_key = select.menu_config.set_focused_key;
/// Effect::new(move || {
///     set_focused_key.set(listbox.state.collection.selection_state.focused_key.get());
/// });
///
/// view! {
///     <button {..select.trigger_props.into_attrs()}>
///         {move || select.display_value.get().unwrap_or_else(|| "Choose...".into())}
///     </button>
///     <Show when=move || select.is_open.get()>
///         <ul {..listbox.listbox_props.into_attrs()}>
///             <For
///                 each=move || items.get()
///                 key=|item| item.to_string()
///                 children=move |item| {
///                     let id = select.get_option_id.run(item.clone());
///                     let option = use_option(UseOptionInput {
///                         key: item.to_string(),
///                         state: select.selection_state,
///                         ..Default::default()
///                     });
///                     view! { <li id=id {..option.option_props.into_attrs()}>{item}</li> }
///                 }
///             />
///         </ul>
///     </Show>
/// }
/// ```
#[allow(clippy::too_many_lines)]
pub fn use_select<K>(input: UseSelectInput<K>) -> UseSelectReturn<K>
where
    K: SelectionKey,
{
    let UseSelectInput {
        selection_mode,
        is_disabled,
        is_required,
        selected_keys,
        default_selected_keys,
        on_selection_change,
        disabled_keys,
        items,
        get_text_value,
        is_open,
        default_open,
        on_open_change,
        allows_empty_collection,
        label,
        aria_label,
        aria_labelledby,
        has_description,
        is_invalid,
        validate,
        validation_behavior,
        name,
        on_focus,
        on_blur,
        on_focus_change,
        on_key_down,
        on_key_up,
        placeholder: _placeholder,
    } = input;

    // =========================================================================
    // IDs
    // =========================================================================

    let base_id = Uuid::new_v4();
    let trigger_id = format!("select-trigger-{base_id}");
    let value_id = format!("select-value-{base_id}");
    let listbox_id = format!("select-listbox-{base_id}");

    // =========================================================================
    // Trigger element capture (for focus restoration)
    // =========================================================================

    let trigger_element = CapturedElement::new();

    // =========================================================================
    // Open state
    // =========================================================================

    let (internal_open, set_internal_open) = signal(default_open);
    let is_open = is_open.unwrap_or_else(|| internal_open.into());

    // Focus strategy — changes each time the menu opens.
    let (focus_strategy, set_focus_strategy) = signal::<Option<FocusStrategy>>(None);

    let open_menu = move |strategy: Option<FocusStrategy>| {
        if is_disabled.get_untracked() {
            return;
        }
        // Empty collection guard
        if items.with_untracked(Vec::is_empty) && !allows_empty_collection {
            return;
        }
        set_focus_strategy.set(strategy.or(Some(FocusStrategy::First)));
        set_internal_open.set(true);
        if let Some(cb) = on_open_change {
            cb.run(true);
        }
    };

    let close_menu = move || {
        set_internal_open.set(false);
        set_focus_strategy.set(None);
        if let Some(cb) = on_open_change {
            cb.run(false);
        }
    };

    // =========================================================================
    // Focus restoration: when menu closes, restore focus to trigger
    // =========================================================================

    {
        let trigger_el = trigger_element;
        let prev_open: StoredValue<bool> = StoredValue::new(false);
        Effect::new(move |_| {
            let open = is_open.get();
            let was_open = prev_open.get_value();
            prev_open.set_value(open);
            if was_open && !open {
                if let Some(el) = trigger_el.get_untracked() {
                    focus_safely(&el);
                }
            }
        });
    }

    // =========================================================================
    // Selection state (via use_selection_state)
    // =========================================================================

    // Internal callback that wraps user callback + auto-close (single mode).
    let on_internal_selection_change = Callback::new(move |selection: Selection<K>| {
        if let Some(cb) = on_selection_change {
            cb.run(selection);
        }
        // Close menu on selection in single mode.
        if selection_mode == SelectionMode::Single {
            close_menu();
        }
    });

    let selection_state = use_selection_state(UseSelectionStateInput {
        selection_mode,
        selection_behavior: SelectionBehavior::Replace,
        disabled: is_disabled,
        selected_keys,
        default_selected_keys,
        on_selection_change: Some(on_internal_selection_change),
        disabled_keys,
        disallow_empty_selection: selection_mode == SelectionMode::Single,
        disabled_behavior: DisabledBehavior::default(),
    });

    // =========================================================================
    // Focus tracking
    // =========================================================================

    let (is_focused, set_is_focused) = signal(false);

    // Focused key in the listbox. Exposed via `menu_config.set_focused_key` so the
    // consumer can wire the listbox's focused_key signal back to keep this in sync.
    let (focused_key, set_focused_key) = signal::<Option<K>>(None);

    // =========================================================================
    // Validation
    // =========================================================================

    let validation = crate::hooks::form::use_form_validation_state::use_form_validation_state(
        UseFormValidationStateInput {
            is_invalid,
            value: selection_state.selected_keys,
            validate,
            validation_behavior,
            name: name.map(ToString::to_string),
        },
    );

    // Commit validation on selection change.
    let commit_validation = validation.commit_validation;
    Effect::new(move |_| {
        // Track selection changes.
        let _ = selection_state.selected_keys.get();
        commit_validation.run(());
    });

    // =========================================================================
    // Field (label, description, error message)
    // =========================================================================

    let field = use_field(UseFieldInput {
        label: label.map(ToString::to_string),
        description: if has_description {
            Some(String::new()) // Presence triggers aria-describedby inclusion.
        } else {
            None
        },
        error_message: Some(String::new()), // Presence ensures error_message_id is generated.
        validation_state: ValidationState::Valid, // Static; reactive describedby computed below.
        is_required,
        is_disabled: is_disabled.get_untracked(),
        ..Default::default()
    });

    // Compute aria-describedby reactively so it includes the error message ID
    // only when validation is invalid.
    let description_id = field.description_props.id.clone();
    let error_message_id = field.error_message_props.id.clone();
    let is_invalid_for_describedby = validation.is_invalid;
    let aria_describedby = Signal::derive(move || {
        let mut parts = Vec::new();
        if has_description {
            parts.push(description_id.clone());
        }
        if is_invalid_for_describedby.get() {
            parts.push(error_message_id.clone());
        }
        if parts.is_empty() {
            None
        } else {
            Some(parts.join(" "))
        }
    });

    // =========================================================================
    // Type-to-select (single mode only)
    // =========================================================================

    // In select (not listbox), type-to-select updates selection directly.
    let type_select_on_focus = {
        let select_cb = selection_state.select;
        Callback::new(move |key: Option<K>| {
            if let Some(k) = key {
                select_cb.run(k);
            }
        })
    };

    // Derive focused_key from selected_keys for type-select starting position.
    let type_select_focused_key =
        Signal::derive(move || match selection_state.selected_keys.get() {
            Selection::Keys(keys) => keys.into_iter().next(),
            Selection::All => items.with_untracked(|v| v.first().cloned()),
        });

    let type_select = if selection_mode == SelectionMode::Single {
        Some(use_type_select(UseTypeSelectInput {
            disabled: is_disabled,
            all_keys: items,
            get_key_label: get_text_value
                .unwrap_or_else(|| Callback::new(|key: K| format!("{key}"))),
            focused_key: type_select_focused_key,
            on_focus: type_select_on_focus,
            timeout_ms: 500,
        }))
    } else {
        None
    };

    // =========================================================================
    // Compute display value
    // =========================================================================

    let display_value = Signal::derive(move || {
        let sel = selection_state.selected_keys.get();
        match sel {
            Selection::Keys(keys) => {
                let first = keys.into_iter().next()?;
                if let Some(get_text) = get_text_value {
                    Some(get_text.run(first))
                } else {
                    Some(format!("{first}"))
                }
            }
            Selection::All => Some("All".to_string()),
        }
    });

    // =========================================================================
    // Keyboard handler (trigger)
    // =========================================================================

    let navigate_selection = move |direction: i32| {
        let keys = items.get_untracked();
        if keys.is_empty() {
            return;
        }

        let disabled_set = disabled_keys.get_untracked();
        let current = match selection_state.selected_keys.get_untracked() {
            Selection::Keys(k) => k.into_iter().next(),
            Selection::All => None,
        };

        let current_idx = current
            .as_ref()
            .and_then(|k| keys.iter().position(|item| item == k));

        // React-aria falls back to the first item regardless of direction
        // when nothing is selected.
        let start_idx = if direction > 0 {
            current_idx.map_or(0, |i| i + 1)
        } else {
            current_idx.map_or(0, |i| i.wrapping_sub(1))
        };

        // Search in the given direction for a non-disabled key.
        let len = keys.len();
        let mut idx = start_idx;
        for _ in 0..len {
            if idx >= len {
                break;
            }
            let key = &keys[idx];
            if !disabled_set.contains(key) {
                selection_state.select.run(key.clone());
                return;
            }
            if direction > 0 {
                idx += 1;
            } else {
                if idx == 0 {
                    break;
                }
                idx -= 1;
            }
        }
    };

    let handle_keydown = move |e: KeyboardEvent| {
        if is_disabled.get_untracked() {
            return;
        }

        // 1. Type-to-select (single mode, when closed).
        // Delegate printable characters to type-ahead. If type-ahead consumed the key
        // (called preventDefault), skip standard handling. Otherwise, fall through so
        // e.g. Space can still open the menu when there's no active search.
        if selection_mode == SelectionMode::Single && !is_open.get_untracked() {
            if let Some(ref ts) = type_select {
                let key_str = e.key();
                if key_str.len() == 1 && !e.ctrl_key() && !e.alt_key() && !e.meta_key() {
                    ts.on_keydown.run(e.clone());
                    if e.default_prevented() {
                        return;
                    }
                }
            }
        }

        // 2. Standard keyboard handling.
        match e.key().as_str() {
            " " | "Enter" | "ArrowDown" => {
                e.prevent_default();
                if is_open.get_untracked() {
                    close_menu();
                } else {
                    open_menu(Some(FocusStrategy::First));
                }
            }
            "ArrowUp" => {
                e.prevent_default();
                if is_open.get_untracked() {
                    close_menu();
                } else {
                    open_menu(Some(FocusStrategy::Last));
                }
            }
            // ArrowLeft/Right: closed-state navigation (single mode only).
            "ArrowLeft" if !is_open.get_untracked() && selection_mode == SelectionMode::Single => {
                e.prevent_default();
                navigate_selection(-1);
            }
            "ArrowRight" if !is_open.get_untracked() && selection_mode == SelectionMode::Single => {
                e.prevent_default();
                navigate_selection(1);
            }
            "Escape" if is_open.get_untracked() => {
                e.prevent_default();
                close_menu();
            }
            _ => {}
        }

        // 3. Forward to user handler.
        if let Some(user_handler) = on_key_down {
            user_handler.run(e);
        }
    };

    // =========================================================================
    // Focus handlers (trigger)
    // =========================================================================

    let handle_focus = move |_e: web_sys::FocusEvent| {
        if is_focused.get_untracked() {
            return;
        }
        set_is_focused.set(true);
        if let Some(cb) = on_focus_change {
            cb.run(true);
        }
        if let Some(cb) = on_focus {
            cb.run(());
        }
    };

    let handle_blur = move |_e: web_sys::FocusEvent| {
        // Suppress blur while menu is open (focus moves between trigger and listbox).
        if is_open.get_untracked() {
            return;
        }
        set_is_focused.set(false);
        if let Some(cb) = on_focus_change {
            cb.run(false);
        }
        if let Some(cb) = on_blur {
            cb.run(());
        }
    };

    // =========================================================================
    // Click handler (trigger)
    // =========================================================================

    // Pointer-down: mouse/pen opens immediately (no auto-focus on item);
    // virtual/screen reader opens with FocusStrategy::First.
    // Touch is handled separately on pointer-up.
    let handle_pointerdown = move |e: web_sys::PointerEvent| {
        if is_disabled.get_untracked() {
            return;
        }
        if items.with_untracked(Vec::is_empty) && !allows_empty_collection {
            return;
        }
        let pointer_type = e.pointer_type();
        if pointer_type == "touch" {
            return; // handled on pointerup
        }
        e.prevent_default();
        if is_open.get_untracked() {
            close_menu();
        } else {
            // Virtual/screen reader: auto-focus first item.
            // Mouse/pen: no item auto-focus (menu itself gets focus).
            let strategy = if pointer_type.is_empty() {
                Some(FocusStrategy::First)
            } else {
                None
            };
            open_menu(strategy);
        }
    };

    // Pointer-up: touch toggles on release (matching native mobile behavior).
    let handle_pointerup = move |e: web_sys::PointerEvent| {
        if is_disabled.get_untracked() {
            return;
        }
        if items.with_untracked(Vec::is_empty) && !allows_empty_collection {
            return;
        }
        if e.pointer_type() != "touch" {
            return; // already handled on pointerdown
        }
        if is_open.get_untracked() {
            close_menu();
        } else {
            open_menu(None);
        }
    };

    // =========================================================================
    // Menu blur handler
    // =========================================================================
    // Resets is_focused when focus leaves the listbox entirely (not moving
    // within the listbox). The consumer should spread this onto the listbox
    // container or wire it into the listbox's blur handling.

    let handle_menu_blur = move |e: web_sys::FocusEvent| {
        // If focus moved within the listbox, ignore.
        let ct_node = e
            .current_target()
            .and_then(|ct| ct.dyn_into::<web_sys::Node>().ok());
        let related_node = e
            .related_target()
            .and_then(|rt| rt.dyn_into::<web_sys::Node>().ok());
        if node_contains(ct_node.as_ref(), related_node.as_ref()) == Some(true) {
            return;
        }
        set_is_focused.set(false);
        if let Some(cb) = on_focus_change {
            cb.run(false);
        }
        if let Some(cb) = on_blur {
            cb.run(());
        }
    };

    // =========================================================================
    // Exposed callbacks
    // =========================================================================

    let open = Callback::new(move |_| {
        open_menu(Some(FocusStrategy::First));
    });

    let close = Callback::new(move |_| {
        close_menu();
    });

    let toggle = Callback::new(move |_| {
        if is_open.get_untracked() {
            close_menu();
        } else {
            open_menu(Some(FocusStrategy::First));
        }
    });

    let set_selected_keys = Callback::new(move |selection: Selection<K>| {
        selection_state.replace_selection.run(selection);
    });

    // =========================================================================
    // get_option_id
    // =========================================================================

    let listbox_id_for_option = listbox_id.clone();
    let get_option_id =
        Callback::new(move |key: K| format!("{listbox_id_for_option}-option-{key}"));

    // =========================================================================
    // Compose ARIA attributes
    // =========================================================================

    let aria_expanded = Signal::derive(move || Some(AriaExpanded::from(is_open.get())));
    let aria_required = is_required.then_some(AriaRequired::True);
    let aria_disabled = Signal::derive(move || is_disabled.get().then_some(AriaDisabled::True));
    let aria_invalid_signal = {
        let is_invalid = validation.is_invalid;
        Signal::derive(move || is_invalid.get().then_some(AriaInvalid::True))
    };

    // Compose aria-labelledby: [valueId, external-labelledby or label-id or trigger-id].
    let composed_aria_labelledby = {
        let mut parts = vec![value_id.clone()];
        if let Some(ref lblby) = aria_labelledby {
            parts.push(lblby.clone());
        } else if label.is_some() {
            parts.push(field.label_props.id.clone());
        } else if aria_label.is_some() {
            parts.push(trigger_id.clone());
        }
        Some(parts.join(" "))
    };

    // Compose menu aria-labelledby.
    let menu_aria_labelledby = if let Some(ref lblby) = aria_labelledby {
        lblby.clone()
    } else if label.is_some() {
        field.label_props.id.clone()
    } else {
        trigger_id.clone()
    };

    // =========================================================================
    // Build return value
    // =========================================================================

    let label_id = field.label_props.id.clone();

    // Label click handler: focus the trigger (since label is <span>, not <label>).
    let trigger_id_for_label = trigger_id.clone();
    let handle_label_click = move |_: web_sys::MouseEvent| {
        if is_disabled.get_untracked() {
            return;
        }
        if let Some(doc) = leptos_use::use_document().as_ref() {
            if let Some(el) = doc.get_element_by_id(&trigger_id_for_label) {
                if let Some(html_el) = el.dyn_ref::<web_sys::HtmlElement>() {
                    let _ = html_el.focus();
                }
            }
        }
    };

    UseSelectReturn {
        trigger_props: UseSelectTriggerProps {
            id: trigger_id.clone(),
            role: AriaRole::Button,
            tabindex: "0",
            aria_haspopup: AriaHasPopup::Listbox,
            aria_expanded,
            aria_controls: listbox_id.clone(),
            aria_label,
            aria_labelledby: composed_aria_labelledby,
            aria_required,
            aria_disabled,
            aria_describedby,
            aria_invalid: aria_invalid_signal,
            on_pointerdown: EventHandler::new(handle_pointerdown),
            on_pointerup: EventHandler::new(handle_pointerup),
            on_keydown: EventHandler::new(handle_keydown),
            on_keyup: EventHandler::new(move |e: KeyboardEvent| {
                if let Some(cb) = on_key_up {
                    cb.run(e);
                }
            }),
            on_focus: EventHandler::new(handle_focus),
            on_blur: EventHandler::new(handle_blur),
            element_capture: trigger_element.attr(),
        },
        value_props: UseSelectValueProps {
            id: value_id.clone(),
        },
        label_props: UseSelectLabelProps {
            id: field.label_props.id,
            html_for: field.label_props.html_for,
            on_click: EventHandler::new(handle_label_click),
        },
        description_props: field.description_props,
        error_message_props: field.error_message_props,
        hidden_select_props: UseSelectHiddenProps {
            name,
            required: is_required,
            disabled: is_disabled,
            selection_mode,
            validation_behavior,
        },
        trigger_id,
        listbox_id,
        label_id,
        value_id,
        is_open,
        selected_keys: selection_state.selected_keys,
        display_value,
        is_focused: is_focused.into(),
        focus_strategy: focus_strategy.into(),
        focused_key: focused_key.into(),
        is_invalid: validation.is_invalid,
        validation_errors: validation.validation_errors,
        validation,
        selection_state,
        open,
        close,
        toggle,
        set_selected_keys,
        get_option_id,
        trigger_element,
        menu_config: UseSelectMenuConfig {
            selection_mode,
            selected_keys: selection_state.selected_keys,
            on_selection_change: on_internal_selection_change,
            disabled_keys,
            disallow_empty_selection: selection_mode == SelectionMode::Single,
            auto_focus: focus_strategy.into(),
            should_focus_wrap: true,
            select_on_focus: false,
            should_select_on_press_up: true,
            should_focus_on_hover: true,
            should_use_virtual_focus: false,
            escape_key_behavior: EscapeKeyBehavior::None,
            on_close: Some(Callback::new(move |()| close_menu())),
            aria_labelledby: menu_aria_labelledby,
            items,
            get_text_value,
            on_blur: EventHandler::new(handle_menu_blur),
            set_focused_key,
        },
    }
}
