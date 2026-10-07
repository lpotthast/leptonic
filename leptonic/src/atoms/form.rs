// Upstream: react-aria-components/src/Form.tsx @ 99e6102368
use std::collections::HashMap;

use leptos::prelude::*;

use crate::{
    hooks::{FormValidationContext, ValidationBehavior},
    utils::{
        classes::Classes, default_class::with_default_class, scoped_context::scoped_view,
        styles::Styles,
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - DOM props (`action`, `onSubmit`, ...) are not taken: attach them to the component
//   (`on:submit=..`, `attr:action=..`), which passes them to the `<form>`. Reason: Leptos forwards
//   attributes and listeners on components to their root element.
//
// =============================================================================

/// What a [`Form`] provides to the field atoms inside it.
#[derive(Debug, Clone, Copy)]
pub struct FormContext {
    pub validation_behavior: ValidationBehavior,
}

/// A `<form>` whose fields share a validation behavior and show server-side validation errors.
///
/// With `ValidationBehavior::Native` (the default), fields use the browser's constraint
/// validation and show their errors when the form is submitted; with `Aria`, the form sets
/// `novalidate` and fields show their errors as the user edits.
///
/// Default class: `leptonic-Form`.
#[allow(clippy::implicit_hasher)]
#[component]
pub fn Form(
    #[prop(default = ValidationBehavior::Native)] validation_behavior: ValidationBehavior,
    /// Errors from the server, by field `name`. A field shows its errors until its value changes.
    #[prop(into, optional)]
    validation_errors: Option<Signal<HashMap<String, Vec<String>>>>,
    #[prop(into, optional)] id: Option<String>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let classes = with_default_class("leptonic-Form", classes);
    let errors = FormValidationContext {
        errors: validation_errors.unwrap_or_default(),
    };
    // Contexts for the children only, with the `<form>` as the root (getting attributes and
    // listeners set on the component).
    scoped_view(
        move || {
            provide_context(FormContext {
                validation_behavior,
            });
            provide_context(errors);
        },
        move || {
            view! {
                <form
                    id=id
                    novalidate=validation_behavior != ValidationBehavior::Native
                    class=classes
                    style=styles
                >
                    {children()}
                </form>
            }
        },
    )
}

/// A field atom's validation behavior: its own, else its [`Form`]'s, else `Native` (as
/// react-aria-components).
pub(crate) fn use_validation_behavior(own: Option<ValidationBehavior>) -> ValidationBehavior {
    own.or_else(|| use_context::<FormContext>().map(|form| form.validation_behavior))
        .unwrap_or(ValidationBehavior::Native)
}
