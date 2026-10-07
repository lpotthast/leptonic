// Upstream: react-aria/src/select/HiddenSelect.tsx @ 99e6102368
use leptos::{
    attr::{
        self, Attr,
        custom::{CustomAttr, custom_attribute},
    },
    ev,
    ev::{On, SharedEventCallback},
    prelude::*,
};
use wasm_bindgen::JsCast;

use super::{SelectMode, SelectState};
use crate::{
    hooks::{
        IntoAttrs,
        collections::Key,
        form::{
            use_form_reset::{UseFormResetInput, use_form_reset},
            use_form_validation::{UseFormValidationInput, use_form_validation},
            use_form_validation_state::ValidationBehavior,
        },
    },
    utils::{CapturedElement, ElementCaptureAttr, EventHandler, aria::AriaHidden},
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - A hook returning props and the `<option>`s / hidden input values to render, instead of a
//   component.
//
// =============================================================================

/// Collections up to this size are mirrored by a native `<select>` (supporting browser
/// autofill); larger ones by hidden inputs.
pub const NATIVE_SELECT_THRESHOLD: usize = 300;

/// Visually hides the container while keeping the `<select>` available to autofill.
/// (react-aria: visually hidden, fixed at the top left).
pub const HIDDEN_SELECT_CONTAINER_STYLE: &str = concat!(
    crate::utils::visually_hidden::visually_hidden_css!(),
    " position: fixed; top: 0; left: 0;"
);

/// Input of [`use_hidden_select`] (usually `use_select`'s `hidden_select`).
#[derive(Clone)]
pub struct UseHiddenSelectInput {
    pub state: SelectState,
    pub name: Option<String>,
    pub form: Option<String>,
    pub auto_complete: Option<String>,
    pub is_disabled: Signal<bool>,
    pub is_required: bool,
    pub validation_behavior: ValidationBehavior,
    /// The select's trigger, focused when the select is its form's first invalid field on
    /// submission.
    pub trigger: Option<CapturedElement>,
}

/// An `<option>` of the hidden `<select>`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HiddenSelectOption {
    /// The form value (the key).
    pub value: String,
    pub text: String,
    pub is_selected: bool,
}

/// Return value of [`use_hidden_select`].
#[derive(Debug)]
pub struct UseHiddenSelectReturn {
    pub container_props: UseHiddenSelectContainerProps,
    /// Render a native `<select>` (with `select_props` and `options`), else hidden inputs (with
    /// `input_props` per entry of `input_values`, only if the select has a name).
    pub use_native_select: Signal<bool>,
    pub select_props: UseHiddenSelectSelectProps,
    /// The `<option>`s, starting with an empty one (no value).
    pub options: Signal<Vec<HiddenSelectOption>>,
    pub input_props: UseHiddenSelectInputProps,
    /// One hidden input per selected key (one empty input without a selection).
    pub input_values: Signal<Vec<String>>,
}

#[derive(Debug)]
pub struct UseHiddenSelectContainerProps {
    pub aria_hidden: AriaHidden,
    pub style: &'static str,
}

pub type UseHiddenSelectContainerAttrs = (
    Attr<attr::AriaHidden, AriaHidden>,
    CustomAttr<&'static str, &'static str>,
    CustomAttr<&'static str, &'static str>,
);

impl IntoAttrs for UseHiddenSelectContainerProps {
    type Attrs = UseHiddenSelectContainerAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::AriaHidden, self.aria_hidden),
            custom_attribute("style", self.style),
            // Tells accessibility linters that the hidden, focusable `<select>` is intended.
            custom_attribute("data-a11y-ignore", "aria-hidden-focus"),
        )
    }
}

#[derive(Debug, Clone)]
pub struct UseHiddenSelectSelectProps {
    pub tabindex: i32,
    pub auto_complete: Option<String>,
    pub disabled: Signal<bool>,
    pub multiple: bool,
    pub required: bool,
    pub name: Option<String>,
    pub form: Option<String>,
    pub on_change: EventHandler<web_sys::Event>,
    pub element_capture: ElementCaptureAttr,
}

pub type UseHiddenSelectSelectAttrs = (
    Attr<attr::Tabindex, i32>,
    Attr<attr::Autocomplete, Option<String>>,
    Attr<attr::Disabled, Signal<bool>>,
    Attr<attr::Multiple, bool>,
    Attr<attr::Required, bool>,
    Attr<attr::Name, Option<String>>,
    Attr<attr::Form, Option<String>>,
    On<ev::change, SharedEventCallback<web_sys::Event>>,
    On<ev::input, SharedEventCallback<web_sys::Event>>,
    ElementCaptureAttr,
);

impl IntoAttrs for UseHiddenSelectSelectProps {
    type Attrs = UseHiddenSelectSelectAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Tabindex, self.tabindex),
            Attr(attr::Autocomplete, self.auto_complete),
            Attr(attr::Disabled, self.disabled),
            Attr(attr::Multiple, self.multiple),
            Attr(attr::Required, self.required),
            Attr(attr::Name, self.name),
            Attr(attr::Form, self.form),
            self.on_change.to_on(ev::change),
            self.on_change.into_on(ev::input),
            self.element_capture,
        )
    }
}

/// Props for each hidden input (large collections). With native validation, the first input
/// is a `type="text"` input hidden with `display: none`, so `required` works.
#[derive(Debug, Clone)]
pub struct UseHiddenSelectInputProps {
    pub r#type: &'static str,
    pub style: Option<&'static str>,
    pub auto_complete: Option<String>,
    pub name: Option<String>,
    pub form: Option<String>,
    pub disabled: Signal<bool>,
    pub required: bool,
}

/// Mirrors a select's value in a hidden native form element, so the select takes part in form
/// submission, reset, validation and browser autofill.
pub fn use_hidden_select(input: UseHiddenSelectInput) -> UseHiddenSelectReturn {
    let UseHiddenSelectInput {
        state,
        name,
        form,
        auto_complete,
        is_disabled,
        is_required,
        validation_behavior,
        trigger,
    } = input;
    let collection = state.list.collection;
    let select_element = CapturedElement::new();

    use_form_reset(UseFormResetInput {
        element: select_element,
        initial_value: state.default_value(),
        on_reset: Callback::new(move |value| state.set_value(value)),
    });
    use_form_validation(UseFormValidationInput {
        focus: Some(Callback::new(move |()| {
            if let Some(trigger) = trigger.and_then(|t| t.get_untracked())
                && let Some(trigger) = trigger.dyn_ref::<web_sys::HtmlElement>()
            {
                let _ = trigger.focus();
            }
        })),
        element: select_element,
        state: state.validation,
        validation_behavior,
    });

    // Autofill picks options of the native select.
    let on_change = EventHandler::new(move |e: web_sys::Event| {
        let Some(select) = e
            .target()
            .and_then(|t| t.dyn_into::<web_sys::HtmlSelectElement>().ok())
        else {
            return;
        };
        let values: Vec<String> = if select.multiple() {
            let options = select.selected_options();
            (0..options.length())
                .filter_map(|i| options.item(i))
                .filter_map(|o| o.dyn_into::<web_sys::HtmlOptionElement>().ok())
                .map(|o| o.value())
                .collect()
        } else {
            vec![select.value()]
        };
        let keys: Vec<Key> = collection.with_untracked(|c| {
            values
                .iter()
                .filter_map(|value| c.items().find(|n| n.key.to_string() == *value))
                .map(|n| n.key.clone())
                .collect()
        });
        state.set_value(keys);
    });

    let required = validation_behavior == ValidationBehavior::Native && is_required;

    UseHiddenSelectReturn {
        container_props: UseHiddenSelectContainerProps {
            aria_hidden: AriaHidden::True,
            style: HIDDEN_SELECT_CONTAINER_STYLE,
        },
        use_native_select: Signal::derive(move || {
            collection.with(|c| c.size() <= NATIVE_SELECT_THRESHOLD)
        }),
        select_props: UseHiddenSelectSelectProps {
            tabindex: -1,
            auto_complete: auto_complete.clone(),
            disabled: is_disabled,
            multiple: state.selection_mode == SelectMode::Multiple,
            required,
            name: name.clone(),
            form: form.clone(),
            on_change,
            element_capture: select_element.attr(),
        },
        options: Signal::derive(move || {
            let value = state.value();
            let mut options = vec![HiddenSelectOption {
                value: String::new(),
                text: String::new(),
                is_selected: value.is_empty(),
            }];
            collection.with(|c| {
                options.extend(c.items().map(|n| HiddenSelectOption {
                    value: n.key.to_string(),
                    text: n.text_value.to_string(),
                    is_selected: value.contains(&n.key),
                }));
            });
            options
        }),
        input_props: UseHiddenSelectInputProps {
            r#type: if validation_behavior == ValidationBehavior::Native {
                "text"
            } else {
                "hidden"
            },
            style: (validation_behavior == ValidationBehavior::Native).then_some("display: none"),
            auto_complete,
            name,
            form,
            disabled: is_disabled,
            required,
        },
        input_values: Signal::derive(move || {
            let values: Vec<String> = state.value().iter().map(ToString::to_string).collect();
            if values.is_empty() {
                vec![String::new()]
            } else {
                values
            }
        }),
    }
}
