//! Form validation state management hook.
//!
//! This module provides [`use_form_validation_state`], the state layer for form validation.
//! It manages multiple validation sources (controlled, server, client, native) and supports
//! two validation behaviors: [`ValidationBehavior::Aria`] (realtime) and
//! [`ValidationBehavior::Native`] (deferred until form submit).

use std::{collections::HashMap, sync::Arc};

use leptos::prelude::*;

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-stately/form/src/useFormValidationState.ts

// REACT-ARIA DEVIATIONS
//
// OMITTED FEATURES
// - `privateValidationStateProp`: React-specific prop-passing mechanism for parent
//   components to share validation state with children. Not needed because Leptos
//   signals are Copy and can be passed directly.
// - `builtinValidation` as a separate input prop: Simplified for the initial
//   implementation. Native validation is read via `use_form_validation`'s
//   `update_validation` callback instead. Can be added later for complex field hooks
//   (e.g., NumberField with hidden native inputs).
//
// LEPTOS-SPECIFIC ADAPTATIONS
// - Hook-owned state: All validation signals are created and owned internally.
//   Callers get read-only `Signal<T>` and semantic callbacks.
// - Validate function signature: `Fn(&T) -> Result<(), Vec<String>>` instead of
//   React-aria's `(value: T) => ValidationError | true | null | undefined`.
// - Commit mechanism: Uses Leptos `Effect` + `Trigger` instead of React's useEffect
//   render-cycle scheduling.
// - FormValidationContext: Uses Leptos `provide_context`/`use_context` instead of
//   React's `createContext`.
// - `is_invalid` uses `Option<Signal<bool>>` to distinguish "not controlled" (`None`)
//   from "controlled as valid" (`Some(false)`) -- matches React-aria's `undefined` vs
//   `false` semantics.

/// Snapshot of the browser's native `ValidityState`.
///
/// The native `ValidityState` is a live object (each property is a getter).
/// This struct captures a frozen snapshot at a point in time.
#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ValidityStateSnapshot {
    pub bad_input: bool,
    pub custom_error: bool,
    pub pattern_mismatch: bool,
    pub range_overflow: bool,
    pub range_underflow: bool,
    pub step_mismatch: bool,
    pub too_long: bool,
    pub too_short: bool,
    pub type_mismatch: bool,
    pub value_missing: bool,
    pub valid: bool,
}

impl Default for ValidityStateSnapshot {
    fn default() -> Self {
        VALID_VALIDITY_STATE
    }
}

/// A `ValidityState` snapshot where all fields are valid.
pub const VALID_VALIDITY_STATE: ValidityStateSnapshot = ValidityStateSnapshot {
    bad_input: false,
    custom_error: false,
    pattern_mismatch: false,
    range_overflow: false,
    range_underflow: false,
    step_mismatch: false,
    too_long: false,
    too_short: false,
    type_mismatch: false,
    value_missing: false,
    valid: true,
};

/// A `ValidityState` snapshot indicating a custom error.
pub const CUSTOM_VALIDITY_STATE: ValidityStateSnapshot = ValidityStateSnapshot {
    bad_input: false,
    custom_error: true,
    pattern_mismatch: false,
    range_overflow: false,
    range_underflow: false,
    step_mismatch: false,
    too_long: false,
    too_short: false,
    type_mismatch: false,
    value_missing: false,
    valid: false,
};

/// The result of validation from a single source.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationResult {
    /// Whether the field is invalid.
    pub is_invalid: bool,
    /// Human-readable error messages.
    pub validation_errors: Vec<String>,
    /// Detailed validity state (mirrors native `ValidityState`).
    pub validation_details: ValidityStateSnapshot,
}

/// The default (valid) validation result.
pub const DEFAULT_VALIDATION_RESULT: ValidationResult = ValidationResult {
    is_invalid: false,
    validation_details: VALID_VALIDITY_STATE,
    validation_errors: Vec::new(),
};

/// Validation behavior mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ValidationBehavior {
    /// All validation errors are displayed in realtime as the user edits.
    #[default]
    Aria,
    /// Validation errors are deferred until form submission, using native
    /// HTML constraint validation (`setCustomValidity`).
    Native,
}

/// A custom validation function.
///
/// Returns `Ok(())` for valid, `Err(messages)` for invalid.
pub type ValidateFn<T> = Arc<dyn Fn(&T) -> Result<(), Vec<String>> + Send + Sync>;

/// Leptos context for server-side form validation errors.
///
/// Provide this in a parent component to pass server validation errors
/// to child form fields. Fields match by their `name` prop.
///
/// # Example
///
/// ```ignore
/// let (errors, set_errors) = signal(HashMap::new());
/// provide_context(FormValidationContext { errors: errors.into() });
///
/// // After a server action returns validation errors:
/// set_errors.set(HashMap::from([
///     ("email".into(), vec!["Email already in use".into()]),
/// ]));
/// ```
#[derive(Debug, Clone, Copy)]
pub struct FormValidationContext {
    /// Maps field name to error messages.
    pub errors: Signal<HashMap<String, Vec<String>>>,
}

/// Input parameters for [`use_form_validation_state`].
pub struct UseFormValidationStateInput<T: Send + Sync + 'static> {
    /// Whether the field is explicitly marked as invalid (controlled error).
    ///
    /// - `None` — not controlled; validation comes from other sources.
    /// - `Some(signal)` — controlled; the signal value determines valid/invalid
    ///   and overrides all other validation sources.
    pub is_invalid: Option<Signal<bool>>,

    /// The current field value, used by the `validate` function.
    pub value: Signal<T>,

    /// Custom client-side validation function.
    ///
    /// Returns `Ok(())` for valid, `Err(messages)` for invalid.
    pub validate: Option<ValidateFn<T>>,

    /// Validation behavior mode.
    pub validation_behavior: ValidationBehavior,

    /// The field's `name` attribute, used to match server errors
    /// from [`FormValidationContext`].
    pub name: Option<String>,
}

/// Return value of [`use_form_validation_state`].
///
/// All fields are `Copy` (signals and callbacks are indices into the reactive system).
#[derive(Clone, Copy)]
pub struct UseFormValidationStateReturn {
    /// Realtime validation result (updated as the user edits).
    ///
    /// Used by [`use_form_validation`](super::use_form_validation::use_form_validation)
    /// to set `setCustomValidity` on the native input.
    pub realtime_validation: Signal<ValidationResult>,

    /// Currently displayed validation result.
    ///
    /// Use this for ARIA attributes and user-visible error messages.
    pub display_validation: Signal<ValidationResult>,

    /// Convenience: whether the displayed validation is invalid.
    pub is_invalid: Signal<bool>,

    /// Convenience: the displayed validation error messages.
    pub validation_errors: Signal<Vec<String>>,

    /// Updates the committed validation result (e.g., from native input validity).
    ///
    /// In `Aria` mode, updates immediately. In `Native` mode, queued until
    /// [`commit_validation`](Self::commit_validation).
    pub update_validation: Callback<ValidationResult>,

    /// Resets displayed validation to valid (on form reset).
    pub reset_validation: Callback<()>,

    /// Commits realtime validation so it is displayed to the user (on change/submit).
    pub commit_validation: Callback<()>,
}

/// Manages form validation state with multiple validation sources.
///
/// This is the state (stately) layer of form validation. It does not interact
/// with the DOM — pair with
/// [`use_form_validation`](super::use_form_validation::use_form_validation)
/// to connect validation to native input elements.
///
/// # Validation Sources (in priority order)
///
/// 1. **Controlled** — explicit `is_invalid` prop
/// 2. **Server** — errors from [`FormValidationContext`] matched by field `name`
/// 3. **Client** — custom `validate` function
/// 4. **Committed** — native validity read via
///    [`update_validation`](UseFormValidationStateReturn::update_validation)
///
/// # Validation Behavior
///
/// - [`ValidationBehavior::Aria`] — all errors displayed in realtime
/// - [`ValidationBehavior::Native`] — client/native errors deferred until
///   [`commit_validation`](UseFormValidationStateReturn::commit_validation)
#[allow(clippy::too_many_lines)]
pub fn use_form_validation_state<T>(
    input: UseFormValidationStateInput<T>,
) -> UseFormValidationStateReturn
where
    T: Clone + PartialEq + Send + Sync + 'static,
{
    let UseFormValidationStateInput {
        is_invalid,
        value,
        validate,
        validation_behavior,
        name,
    } = input;

    // Store validate function for closure capture.
    let validate = StoredValue::new(validate);
    let name = StoredValue::new(name);

    // ---- Controlled error ----
    // When is_invalid is explicitly provided, it overrides all other sources.
    // Even `Some(false)` overrides — it means "I control validation and say it's valid."
    let controlled_error: Signal<Option<ValidationResult>> = match is_invalid {
        Some(is_invalid_signal) => Signal::derive(move || {
            Some(ValidationResult {
                is_invalid: is_invalid_signal.get(),
                validation_errors: vec![],
                validation_details: CUSTOM_VALIDITY_STATE,
            })
        }),
        None => Signal::derive(|| None),
    };

    // ---- Client validation ----
    let client_error: Memo<Option<ValidationResult>> = Memo::new(move |_| {
        let validate_fn = validate.get_value();
        let validate_fn = validate_fn.as_ref()?;
        let val = value.get();
        match validate_fn(&val) {
            Ok(()) => None,
            Err(errors) if errors.is_empty() => None,
            Err(errors) => Some(ValidationResult {
                is_invalid: true,
                validation_errors: errors,
                validation_details: CUSTOM_VALIDITY_STATE,
            }),
        }
    });

    // ---- Server errors ----
    let server_context = use_context::<FormValidationContext>();
    let (server_error_cleared, set_server_error_cleared) = signal(false);

    // When server errors change (new form submission response), reset the
    // "cleared" flag so new errors become visible.
    if let Some(ctx) = server_context {
        let prev_errors = StoredValue::new(HashMap::<String, Vec<String>>::new());
        Effect::new(move |_| {
            let current = ctx.errors.get();
            if current != prev_errors.get_value() {
                prev_errors.set_value(current);
                set_server_error_cleared.set(false);
            }
        });
    }

    let server_error: Memo<Option<ValidationResult>> = Memo::new(move |_| {
        if server_error_cleared.get() {
            return None;
        }
        let ctx = server_context?;
        let name_val = name.get_value();
        let name_str = name_val.as_deref()?;
        let errors_map = ctx.errors.get();
        let field_errors = errors_map.get(name_str)?;
        if field_errors.is_empty() {
            return None;
        }
        Some(ValidationResult {
            is_invalid: true,
            validation_errors: field_errors.clone(),
            validation_details: CUSTOM_VALIDITY_STATE,
        })
    });

    // ---- Committed validation (for native mode deferred display) ----
    let (current_validity, set_current_validity) = signal(DEFAULT_VALIDATION_RESULT);
    let next_validation = StoredValue::new(DEFAULT_VALIDATION_RESULT);
    let last_error = StoredValue::new(DEFAULT_VALIDATION_RESULT);
    let (commit_queued, set_commit_queued) = signal(false);
    let commit_trigger = Trigger::new();

    // Commit effect: when commit is queued, read latest validation and display it.
    Effect::new(move |_| {
        commit_trigger.track();
        if !commit_queued.get_untracked() {
            return;
        }
        set_commit_queued.set(false);
        let error = client_error
            .get_untracked()
            .unwrap_or_else(|| next_validation.get_value());
        if error != last_error.get_value() {
            last_error.set_value(error.clone());
            set_current_validity.set(error);
        }
    });

    // ---- Realtime validation ----
    // Priority: controlled > server > client > default.
    let realtime_validation = Signal::derive(move || {
        controlled_error
            .get()
            .or_else(|| server_error.get())
            .or_else(|| client_error.get())
            .unwrap_or(DEFAULT_VALIDATION_RESULT)
    });

    // ---- Display validation ----
    // Aria: all errors shown in realtime.
    // Native: client/native errors deferred until commit.
    let display_validation = Signal::derive(move || match validation_behavior {
        ValidationBehavior::Native => controlled_error
            .get()
            .or_else(|| server_error.get())
            .unwrap_or_else(|| current_validity.get()),
        ValidationBehavior::Aria => controlled_error
            .get()
            .or_else(|| server_error.get())
            .or_else(|| client_error.get())
            .unwrap_or_else(|| current_validity.get()),
    });

    // ---- Convenience derived signals ----
    let result_is_invalid = Signal::derive(move || display_validation.get().is_invalid);
    let result_validation_errors =
        Signal::derive(move || display_validation.get().validation_errors);

    UseFormValidationStateReturn {
        realtime_validation,
        display_validation,
        is_invalid: result_is_invalid,
        validation_errors: result_validation_errors,
        update_validation: Callback::new(move |result: ValidationResult| {
            // In Aria mode, update displayed validation immediately.
            // In Native mode, queue for next commit.
            if validation_behavior == ValidationBehavior::Aria {
                if current_validity.get_untracked() != result {
                    set_current_validity.set(result);
                }
            } else {
                next_validation.set_value(result);
            }
        }),
        reset_validation: Callback::new(move |()| {
            // Reset displayed validation to valid.
            let error = DEFAULT_VALIDATION_RESULT;
            if error != last_error.get_value() {
                last_error.set_value(error.clone());
                set_current_validity.set(error);
            }
            // Cancel pending commit.
            if validation_behavior == ValidationBehavior::Native {
                set_commit_queued.set(false);
            }
            set_server_error_cleared.set(true);
        }),
        commit_validation: Callback::new(move |()| {
            // Queue commit so display_validation shows latest errors.
            if validation_behavior == ValidationBehavior::Native {
                set_commit_queued.set(true);
                commit_trigger.notify();
            }
            // Clear server errors (user changed their value).
            set_server_error_cleared.set(true);
        }),
    }
}

/// Merges multiple validation results into one.
///
/// Combines all error messages (deduplicated) and ORs all validity details.
pub fn merge_validation(results: &[ValidationResult]) -> ValidationResult {
    let mut errors = Vec::new();
    let mut seen = std::collections::HashSet::new();
    let mut is_invalid = false;
    let mut details = VALID_VALIDITY_STATE;

    for v in results {
        for e in &v.validation_errors {
            if seen.insert(e.clone()) {
                errors.push(e.clone());
            }
        }
        is_invalid |= v.is_invalid;
        details.bad_input |= v.validation_details.bad_input;
        details.custom_error |= v.validation_details.custom_error;
        details.pattern_mismatch |= v.validation_details.pattern_mismatch;
        details.range_overflow |= v.validation_details.range_overflow;
        details.range_underflow |= v.validation_details.range_underflow;
        details.step_mismatch |= v.validation_details.step_mismatch;
        details.too_long |= v.validation_details.too_long;
        details.too_short |= v.validation_details.too_short;
        details.type_mismatch |= v.validation_details.type_mismatch;
        details.value_missing |= v.validation_details.value_missing;
    }
    details.valid = !is_invalid;

    ValidationResult {
        is_invalid,
        validation_errors: errors,
        validation_details: details,
    }
}
