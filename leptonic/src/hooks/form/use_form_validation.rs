// Upstream: react-aria/src/form/useFormValidation.ts @ 99e6102368
//! Connects a field's validation state to the browser's constraint validation: custom validity,
//! the `invalid`, `change` and form `reset` events, and focusing the first invalid field.

use leptos::prelude::*;
use leptos_element_capture::CapturedElement;
use send_wrapper::SendWrapper;
use wasm_bindgen::{JsCast, closure::Closure};

use super::use_form_validation_state::{
    FormValidationState, ValidationBehavior, ValidationResult, ValidityStateSnapshot,
};
use crate::{
    hooks::focus::use_focus_visible::{Modality, set_modality},
    utils::event_listeners::listen,
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## OMITTED FEATURES
// - Patching `form.reset()` to ignore React's automatic resets after server actions. Reason:
//   React-specific; Leptos doesn't reset forms on its own.
//
// ## BEHAVIORAL DIFFERENCES
// - The custom validity is synced (and the native validity read back) when the realtime
//   validation changes, before each commit, on `change` and when a constraint attribute
//   (`required`, `min`, ...) changes (react-aria: in a layout effect after every render).
//   Reason: a render isn't an event in Leptos.
//
// =============================================================================

/// Input of [`use_form_validation`].
#[derive(Clone, Copy)]
pub struct UseFormValidationInput {
    /// The `<input>`, `<textarea>` or `<select>` holding the value.
    pub element: CapturedElement,
    /// The state from [`use_form_validation_state`](super::use_form_validation_state::use_form_validation_state).
    pub state: FormValidationState,
    pub validation_behavior: ValidationBehavior,
    /// Focuses the field when it is the form's first invalid field (default: focuses `element`),
    /// for fields whose focusable element isn't the validated one.
    pub focus: Option<Callback<()>>,
}

/// Connects a field's validation state to the browser's constraint validation:
/// - in `Native` mode, sets the element's custom validity from the realtime validation and reads
///   the native validity back,
/// - commits the validation on `invalid` (form submission) and `change`, focusing the form's
///   first invalid field on submission,
/// - resets the validation when the form is reset.
pub fn use_form_validation(input: UseFormValidationInput) {
    let UseFormValidationInput {
        element,
        state,
        validation_behavior,
        focus,
    } = input;

    let is_native = validation_behavior == ValidationBehavior::Native;
    // Native mode: sets the custom validity from the realtime validation and reads the native
    // validity back.
    let sync = move |realtime: ValidationResult| {
        let Some(el) = element.get_untracked() else {
            return;
        };
        let Some(field) = Validatable::of(&el) else {
            return;
        };
        if field.disabled() {
            return;
        }
        let error_message = if realtime.is_invalid {
            let message = realtime.validation_errors.join(" ");
            if message.is_empty() {
                "Invalid value.".to_owned()
            } else {
                message
            }
        } else {
            String::new()
        };
        field.set_custom_validity(&error_message);

        // Prevent the browser's tooltip for the validation message.
        // https://bugzilla.mozilla.org/show_bug.cgi?id=605277
        if !el.has_attribute("title") {
            let _ = el.set_attribute("title", "");
        }

        if !realtime.is_invalid {
            state.update_validation(field.native_validity());
        }
    };

    if is_native {
        Effect::new(move |_| {
            // Re-run once the element is captured.
            if element.get().is_none() {
                return;
            }
            sync(state.realtime_validation.get());
        });
        // Before each commit as well: the native validity changes without the realtime
        // validation changing.
        state
            .native_validity_readers
            .register(Callback::new(move |()| {
                sync(state.realtime_validation.get_untracked());
            }));
        // And whenever a constraint attribute changes (e.g. a checkbox group's `required`, gone
        // once it has a value): react-aria re-reads the native validity after every render, and
        // a checkbox group shows its items' native validity in realtime.
        Effect::new(move |_| {
            let Some(el) = element.get() else { return };
            let observer = ConstraintObserver::observe(&el, move || {
                sync(state.realtime_validation.get_untracked());
            });
            let observer = SendWrapper::new(observer);
            on_cleanup(move || drop(observer));
        });
    }

    Effect::new(move |_| {
        let Some(el) = element.get() else { return };
        let form = Validatable::of(&el).and_then(|field| field.form());

        let on_invalid = {
            let el = el.clone();
            move |e: web_sys::Event| {
                // Only commit when not already showing an error: this keeps server errors the
                // user didn't fix.
                if !state.display_validation.get_untracked().is_invalid {
                    state.commit_validation();
                }
                // Focus the form's first invalid field, unless the event's default was prevented.
                let is_first_invalid = Validatable::of(&el)
                    .and_then(|field| field.form())
                    .and_then(|form| first_invalid_element(&form))
                    .is_some_and(|first| first == *el);
                if !e.default_prevented() && is_first_invalid {
                    match focus {
                        Some(focus) => focus.run(()),
                        None => {
                            if let Some(el) = el.dyn_ref::<web_sys::HtmlElement>() {
                                let _ = el.focus();
                            }
                        }
                    }
                    // Always show the focus ring.
                    set_modality(Modality::Keyboard);
                }
                // Prevent the browser's error UI.
                e.prevent_default();
            }
        };
        let listeners = (
            listen(&el, "invalid", false, on_invalid),
            listen(&el, "change", false, move |_| {
                // The new value's native validity (e.g. a checked required checkbox).
                if is_native {
                    sync(state.realtime_validation.get_untracked());
                }
                state.commit_validation();
            }),
            form.map(|form| {
                listen(&form, "reset", false, move |_| {
                    state.reset_validation();
                })
            }),
        );
        let listeners = SendWrapper::new(listeners);
        on_cleanup(move || drop(listeners));
    });
}

/// The attributes constraining an input's value.
const CONSTRAINT_ATTRIBUTES: [&str; 11] = [
    "required",
    "disabled",
    "min",
    "max",
    "minlength",
    "maxlength",
    "pattern",
    "step",
    "type",
    "multiple",
    "value",
];

/// Calls a function whenever one of the element's [`CONSTRAINT_ATTRIBUTES`] changes, until
/// dropped.
struct ConstraintObserver {
    observer: web_sys::MutationObserver,
    _on_mutation: Closure<dyn FnMut(js_sys::Array, web_sys::MutationObserver)>,
}

impl ConstraintObserver {
    fn observe(el: &web_sys::Element, on_change: impl Fn() + 'static) -> Option<Self> {
        let on_mutation = Closure::<dyn FnMut(js_sys::Array, web_sys::MutationObserver)>::new(
            move |_: js_sys::Array, _: web_sys::MutationObserver| on_change(),
        );
        let observer = web_sys::MutationObserver::new(on_mutation.as_ref().unchecked_ref()).ok()?;
        let init = web_sys::MutationObserverInit::new();
        init.set_attributes(true);
        init.set_attribute_filter(
            &CONSTRAINT_ATTRIBUTES
                .iter()
                .map(|name| wasm_bindgen::JsValue::from_str(name))
                .collect::<js_sys::Array>(),
        );
        observer.observe_with_options(el, &init).ok()?;
        Some(Self {
            observer,
            _on_mutation: on_mutation,
        })
    }
}

impl Drop for ConstraintObserver {
    fn drop(&mut self) {
        self.observer.disconnect();
    }
}

/// The elements taking part in constraint validation.
enum Validatable<'a> {
    Input(&'a web_sys::HtmlInputElement),
    TextArea(&'a web_sys::HtmlTextAreaElement),
    Select(&'a web_sys::HtmlSelectElement),
}

impl<'a> Validatable<'a> {
    fn of(el: &'a web_sys::Element) -> Option<Self> {
        el.dyn_ref()
            .map(Self::Input)
            .or_else(|| el.dyn_ref().map(Self::TextArea))
            .or_else(|| el.dyn_ref().map(Self::Select))
    }

    fn disabled(&self) -> bool {
        match self {
            Self::Input(el) => el.disabled(),
            Self::TextArea(el) => el.disabled(),
            Self::Select(el) => el.disabled(),
        }
    }

    fn set_custom_validity(&self, message: &str) {
        match self {
            Self::Input(el) => el.set_custom_validity(message),
            Self::TextArea(el) => el.set_custom_validity(message),
            Self::Select(el) => el.set_custom_validity(message),
        }
    }

    fn form(&self) -> Option<web_sys::HtmlFormElement> {
        match self {
            Self::Input(el) => el.form(),
            Self::TextArea(el) => el.form(),
            Self::Select(el) => el.form(),
        }
    }

    fn validity(&self) -> web_sys::ValidityState {
        match self {
            Self::Input(el) => el.validity(),
            Self::TextArea(el) => el.validity(),
            Self::Select(el) => el.validity(),
        }
    }

    fn validation_message(&self) -> String {
        match self {
            Self::Input(el) => el.validation_message(),
            Self::TextArea(el) => el.validation_message(),
            Self::Select(el) => el.validation_message(),
        }
        .unwrap_or_default()
    }

    fn native_validity(&self) -> ValidationResult {
        let validity = self.validity();
        let message = self.validation_message();
        ValidationResult {
            is_invalid: !validity.valid(),
            // A snapshot: the native `ValidityState` is live.
            validation_details: ValidityStateSnapshot {
                bad_input: validity.bad_input(),
                custom_error: validity.custom_error(),
                pattern_mismatch: validity.pattern_mismatch(),
                range_overflow: validity.range_overflow(),
                range_underflow: validity.range_underflow(),
                step_mismatch: validity.step_mismatch(),
                too_long: validity.too_long(),
                too_short: validity.too_short(),
                type_mismatch: validity.type_mismatch(),
                value_missing: validity.value_missing(),
                valid: validity.valid(),
            },
            validation_errors: if message.is_empty() {
                vec![]
            } else {
                vec![message]
            },
        }
    }
}

/// The form of an `<input>`, `<textarea>` or `<select>`.
pub(crate) fn get_parent_form(el: &web_sys::Element) -> Option<web_sys::HtmlFormElement> {
    Validatable::of(el).and_then(|field| field.form())
}

/// The form's first element that fails constraint validation.
fn first_invalid_element(form: &web_sys::HtmlFormElement) -> Option<web_sys::Element> {
    let elements = form.elements();
    (0..elements.length())
        .filter_map(|i| elements.item(i))
        .find(|el| Validatable::of(el).is_some_and(|field| !field.validity().valid()))
}
