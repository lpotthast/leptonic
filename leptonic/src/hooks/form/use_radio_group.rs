use std::sync::Arc;

use leptos::{
    attr,
    attr::Attr,
    ev,
    ev::{On, SharedEventCallback},
    prelude::*,
};
use uuid::Uuid;
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, HtmlInputElement, KeyboardEvent};

use super::{
    use_checkbox_group::Orientation,
    use_form_validation_state::{
        use_form_validation_state, UseFormValidationStateInput, ValidateFn, ValidationBehavior,
        ValidityStateSnapshot,
    },
};
use crate::{
    hooks::IntoAttrs,
    utils::{
        aria::{AriaDisabled, AriaInvalid, AriaLive, AriaOrientation, AriaRequired, AriaRole}, focusable_tree_walker::{get_focusable_tree_walker, FocusableTreeWalkerOptions}, i18n::{try_use_locale, I18nContext},
        locale::WritingDirection,
        EventAccessors,
        EventHandler,
        EventTargetExt,
    },
};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/radio/src/useRadioGroup.ts

// No intentional deviations from the react-aria implementation.

#[derive(Clone, Copy)]
enum Direction {
    Next,
    Prev,
}

/// Input parameters for the `use_radio_group` hook.
#[derive(Clone)]
pub struct UseRadioGroupInput<T>
where
    T: Clone + PartialEq + Send + Sync + 'static,
{
    /// The current selected value (controlled).
    pub value: Signal<Option<T>>,

    /// Callback when the selection changes.
    pub on_change: Option<Callback<T>>,

    /// Whether the group is disabled.
    pub is_disabled: Signal<bool>,

    /// Whether the group is read-only.
    pub is_read_only: Signal<bool>,

    /// Whether the group is required.
    pub is_required: bool,

    /// Whether the group is explicitly marked as invalid (controlled validation).
    ///
    /// - `None` — not controlled; validation comes from `validate`, server errors,
    ///   or native constraint validation.
    /// - `Some(signal)` — controlled; the signal value determines valid/invalid
    ///   and overrides all other validation sources.
    pub is_invalid: Option<Signal<bool>>,

    /// Custom client-side validation function.
    ///
    /// Returns `Ok(())` for valid, `Err(messages)` for invalid.
    pub validate: Option<ValidateFn<Option<T>>>,

    /// Validation behavior mode.
    pub validation_behavior: ValidationBehavior,

    /// The label for the group.
    pub label: Option<String>,

    /// A description for the group.
    pub description: Option<String>,

    /// The name attribute for form submission.
    pub name: Option<&'static str>,

    /// The orientation of the group.
    pub orientation: Orientation,
}

impl<T: Clone + PartialEq + Send + Sync + 'static> Default for UseRadioGroupInput<T> {
    fn default() -> Self {
        Self {
            value: Signal::derive(|| None),
            on_change: None,
            is_disabled: Signal::derive(|| false),
            is_read_only: Signal::derive(|| false),
            is_required: false,
            is_invalid: None,
            validate: None,
            validation_behavior: ValidationBehavior::default(),
            label: None,
            description: None,
            name: None,
            orientation: Orientation::Vertical,
        }
    }
}

/// The return value of the `use_radio_group` hook.
pub struct UseRadioGroupReturn<T>
where
    T: Clone + PartialEq + Send + Sync + 'static,
{
    /// Props for the group container element.
    pub group_props: UseRadioGroupProps,

    /// Props for the label element.
    pub label_props: UseRadioGroupLabelProps,

    /// Props for the error message element.
    pub error_props: UseRadioGroupErrorProps,

    /// The group state for use by individual radio buttons.
    pub state: UseRadioGroupState<T>,

    /// Whether the displayed validation is invalid.
    pub is_invalid: Signal<bool>,

    /// The displayed validation error messages.
    pub validation_errors: Signal<Vec<String>>,

    /// Detailed validity state (mirrors native `ValidityState`).
    pub validation_details: Signal<ValidityStateSnapshot>,
}

/// Props for the radio group container.
#[derive(Debug)]
pub struct UseRadioGroupProps {
    /// The role attribute.
    pub role: AriaRole,

    /// The aria-labelledby attribute.
    pub aria_labelledby: Option<String>,

    /// The aria-describedby attribute.
    pub aria_describedby: Signal<Option<String>>,

    /// The aria-invalid attribute.
    pub aria_invalid: Signal<Option<AriaInvalid>>,

    /// The aria-required attribute.
    pub aria_required: Option<AriaRequired>,

    /// The aria-disabled attribute.
    pub aria_disabled: Signal<Option<AriaDisabled>>,

    /// The aria-orientation attribute.
    pub aria_orientation: AriaOrientation,

    /// Keyboard handler implementing arrow-key cycling between radios.
    pub on_keydown: EventHandler<KeyboardEvent>,
}

impl IntoAttrs for UseRadioGroupProps {
    type Attrs = UseRadioGroupAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Role, self.role),
            Attr(attr::AriaLabelledby, self.aria_labelledby),
            Attr(attr::AriaDescribedby, self.aria_describedby),
            Attr(attr::AriaInvalid, self.aria_invalid),
            Attr(attr::AriaRequired, self.aria_required),
            Attr(attr::AriaDisabled, self.aria_disabled),
            Attr(attr::AriaOrientation, self.aria_orientation),
            self.on_keydown.into_on(ev::keydown),
        )
    }
}

pub type UseRadioGroupAttrs = (
    Attr<attr::Role, AriaRole>,
    Attr<attr::AriaLabelledby, Option<String>>,
    Attr<attr::AriaDescribedby, Signal<Option<String>>>,
    Attr<attr::AriaInvalid, Signal<Option<AriaInvalid>>>,
    Attr<attr::AriaRequired, Option<AriaRequired>>,
    Attr<attr::AriaDisabled, Signal<Option<AriaDisabled>>>,
    Attr<attr::AriaOrientation, AriaOrientation>,
    On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
);

/// Props for the group label.
#[derive(Debug)]
pub struct UseRadioGroupLabelProps {
    /// The id of the label element.
    pub id: String,
}

impl IntoAttrs for UseRadioGroupLabelProps {
    type Attrs = UseRadioGroupLabelAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (Attr(attr::Id, self.id),)
    }
}

pub type UseRadioGroupLabelAttrs = (Attr<attr::Id, String>,);

/// Props for the error message element.
#[derive(Debug)]
pub struct UseRadioGroupErrorProps {
    /// The id of the error message element.
    pub id: String,

    /// The role attribute.
    pub role: AriaRole,

    /// The aria-live attribute.
    pub aria_live: AriaLive,
}

impl IntoAttrs for UseRadioGroupErrorProps {
    type Attrs = UseRadioGroupErrorAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Id, self.id),
            Attr(attr::Role, self.role),
            Attr(attr::AriaLive, self.aria_live),
        )
    }
}

/// Attributes for the radio group error message element.
pub type UseRadioGroupErrorAttrs = (
    Attr<attr::Id, String>,
    Attr<attr::Role, AriaRole>,
    Attr<attr::AriaLive, AriaLive>,
);

/// State for a radio group.
#[derive(Clone, Copy)]
pub struct UseRadioGroupState<T>
where
    T: Clone + PartialEq + Send + Sync + 'static,
{
    /// The current selected value.
    pub selected_value: Signal<Option<T>>,

    /// Whether the group is disabled.
    pub is_disabled: Signal<bool>,

    /// Whether the group is read-only.
    pub is_read_only: Signal<bool>,

    /// Set the selected value.
    pub set_selected_value: Callback<T>,

    /// The name for the radio group (for form submission).
    pub name: &'static str,
}

/// Provides the behavior and accessibility implementation for a radio group.
///
/// Radio groups allow users to select a single option from a set.
///
/// # Example
///
/// ```ignore
/// let (selected, set_selected) = signal(None::<String>);
///
/// let group = use_radio_group(UseRadioGroupInput {
///     value: selected.into(),
///     on_change: Some(Callback::new(move |value| {
///         set_selected.set(Some(value));
///     })),
///     label: Some("Choose an option".to_string()),
///     ..Default::default()
/// });
///
/// view! {
///     <fieldset
///         role=group.group_props.role
///         aria-labelledby=group.group_props.aria_labelledby
///     >
///         <legend id=group.label_props.id>"Choose an option"</legend>
///         // Individual radio buttons here using group.state
///     </fieldset>
/// }
/// ```
#[allow(clippy::needless_pass_by_value)]
pub fn use_radio_group<T>(input: UseRadioGroupInput<T>) -> UseRadioGroupReturn<T>
where
    T: Clone + PartialEq + Send + Sync + 'static,
{
    let UseRadioGroupInput {
        value,
        on_change,
        is_disabled,
        is_read_only,
        is_required,
        is_invalid,
        validate,
        validation_behavior,
        label,
        description,
        name,
        orientation,
    } = input;

    // ---- Form validation state ----
    let validation = use_form_validation_state(UseFormValidationStateInput {
        is_invalid,
        value,
        validate,
        validation_behavior,
        name: name.map(ToString::to_string),
    });

    // ---- IDs ----
    let base_id = Uuid::new_v4();
    let label_id = format!("radio-group-label-{base_id}");
    let description_id = format!("radio-group-description-{base_id}");
    let error_id = format!("radio-group-error-{base_id}");
    let group_name: &'static str =
        name.unwrap_or_else(|| Box::leak(format!("radio-group-{base_id}").into_boxed_str()));

    // ---- Reactive ARIA attributes ----
    let has_description = description.is_some();
    let has_label = label.is_some();

    let description_id_for_signal = description_id.clone();
    let error_id_for_signal = error_id.clone();
    let aria_describedby = Signal::derive(move || {
        let mut parts = Vec::new();
        if has_description {
            parts.push(description_id_for_signal.clone());
        }
        if validation.is_invalid.get() {
            parts.push(error_id_for_signal.clone());
        }
        if parts.is_empty() {
            None
        } else {
            Some(parts.join(" "))
        }
    });

    // Build aria-labelledby
    let aria_labelledby = if has_label {
        Some(label_id.clone())
    } else {
        None
    };

    let aria_invalid =
        Signal::derive(move || validation.is_invalid.get().then_some(AriaInvalid::True));

    // ---- Validation details convenience signal ----
    let validation_details =
        Signal::derive(move || validation.display_validation.get().validation_details);

    // Set selected value callback
    let set_selected_value = Callback::new(move |v: T| {
        if is_disabled.get_untracked() || is_read_only.get_untracked() {
            return;
        }

        if let Some(on_change) = on_change {
            on_change.run(v);
        }
    });

    // Arrow-key cycling between radios. Walks the group container for radio inputs, focuses the
    // next/previous one, and dispatches a click so the radio's own change handler invokes
    // set_selected_value.
    let i18n = try_use_locale();
    let handle_keydown = move |e: KeyboardEvent| {
        if is_disabled.get_untracked() || is_read_only.get_untracked() {
            return;
        }

        let key = e.key();
        let direction = i18n
            .as_ref()
            .map_or(WritingDirection::Ltr, I18nContext::direction);
        let next_dir = match key.as_str() {
            "ArrowRight" => match (direction, orientation) {
                (WritingDirection::Rtl, Orientation::Horizontal) => Direction::Prev,
                _ => Direction::Next,
            },
            "ArrowLeft" => match (direction, orientation) {
                (WritingDirection::Rtl, Orientation::Horizontal) => Direction::Next,
                _ => Direction::Prev,
            },
            "ArrowDown" => Direction::Next,
            "ArrowUp" => Direction::Prev,
            _ => return,
        };

        let Some(container) = e.expect_current_target().to_element() else {
            return;
        };
        let from = e.expect_target().to_element();

        let accept: Arc<dyn Fn(&web_sys::Element) -> bool + Send + Sync> =
            Arc::new(|el: &web_sys::Element| {
                el.dyn_ref::<HtmlInputElement>()
                    .is_some_and(|input| input.type_() == "radio")
            });
        let Some(mut walker) = get_focusable_tree_walker(
            &container,
            FocusableTreeWalkerOptions {
                tabbable: false,
                from: from.clone(),
                from_radio_group: None,
                accept: Some(accept),
            },
        ) else {
            return;
        };

        let next = match next_dir {
            Direction::Next => walker.next_node().or_else(|| {
                walker.set_current_node(container.as_ref());
                walker.first_child()
            }),
            Direction::Prev => walker.previous_node().or_else(|| {
                walker.set_current_node(container.as_ref());
                walker.last_child()
            }),
        };

        let Some(next_node) = next else { return };
        let Some(next_input) = next_node.dyn_ref::<HtmlInputElement>() else {
            return;
        };

        e.prevent_default();
        if let Some(html) = next_input.dyn_ref::<HtmlElement>() {
            let _ = html.focus();
        }
        next_input.click();
    };

    UseRadioGroupReturn {
        group_props: UseRadioGroupProps {
            role: AriaRole::Radiogroup,
            aria_labelledby,
            aria_describedby,
            aria_invalid,
            aria_required: is_required.then_some(AriaRequired::True),
            aria_disabled: Signal::derive(move || is_disabled.get().then_some(AriaDisabled::True)),
            aria_orientation: AriaOrientation::from(orientation),
            on_keydown: EventHandler::new(handle_keydown),
        },
        label_props: UseRadioGroupLabelProps { id: label_id },
        error_props: UseRadioGroupErrorProps {
            id: error_id,
            role: AriaRole::Alert,
            aria_live: AriaLive::Polite,
        },
        state: UseRadioGroupState {
            selected_value: value,
            is_disabled,
            is_read_only,
            set_selected_value,
            name: group_name,
        },
        is_invalid: validation.is_invalid,
        validation_errors: validation.validation_errors,
        validation_details,
    }
}
