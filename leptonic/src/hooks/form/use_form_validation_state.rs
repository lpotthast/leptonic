// Upstream: react-stately/src/form/useFormValidationState.ts @ 99e6102368
// Upstream: react-aria-components/test/FieldError.test.js @ 99e6102368
//! Form validation state management hook.
//!
//! This module provides [`use_form_validation_state`], the state layer for form validation.
//! It manages multiple validation sources (controlled, server, client, native) and supports
//! two validation behaviors: [`ValidationBehavior::Aria`] (realtime) and
//! [`ValidationBehavior::Native`] (deferred until form submit).

use std::{collections::HashMap, sync::Arc};

use leptos::prelude::*;

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - The state is a `Copy` struct of read-only signals and methods (C3).
// - `validate` returns `Result<(), Vec<String>>` (react-aria: `ValidationError | true | null |
//   undefined`).
// - `is_invalid` is a `Signal<bool>` that marks the field invalid while `true` and leaves the
//   other validation sources in charge while `false` (C4). React-aria's `isInvalid` is a
//   controlled prop: `false` forces the field valid, overriding `validate` and native validity;
//   a controlled valid state is not a hook-owned-state shape.
// - `names` is a list of field names (react-aria: `name`, a name or a list); a field shows the
//   server errors of all of them (a date range picker: its start's and end's).
// - The commit runs in an `Effect` triggered by `commit_validation` (react-aria: a `useEffect`
//   after the next render), and re-reads the inputs' native validity first
//   (`NativeValidityReaders`; react-aria re-reads it after every render).
// - Server errors show again whenever the `FormValidationContext`'s signal changes (react-aria:
//   for every new errors object).
//
// ## DIFFERENT BEHAVIOR
// - `validate` also runs for empty values (`None` of an `Option` value): react-aria skips
//   `null`/`undefined` values, which a generic `T` can't tell apart. A validator of an optional
//   value decides itself whether an empty value is an error (`is_required` reports a missing
//   value).
//
// ## OMITTED FEATURES
// - `privateValidationStateProp`: a parent shares its state by passing it in the child hook's
//   input (e.g. `UseTextFieldInput.validation`).
//
// =============================================================================

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
    /// Maps field name to error messages. A field shows its errors until its value changes;
    /// every change of this signal (each server response, also one equal to the last) shows them
    /// again.
    pub errors: Signal<HashMap<String, Vec<String>>>,
}

/// Input parameters for [`use_form_validation_state`].
pub struct UseFormValidationStateInput<T: Send + Sync + 'static> {
    /// Marks the field invalid while `true` (taking precedence over the other sources); while
    /// `false`, the other validation sources decide.
    pub is_invalid: Signal<bool>,

    /// The current field value, used by the `validate` function.
    pub value: Signal<T>,

    /// Custom client-side validation function.
    ///
    /// Returns `Ok(())` for valid, `Err(messages)` for invalid.
    pub validate: Option<ValidateFn<T>>,

    /// The field's own validation (e.g. a date outside min and max), after `validate`'s
    /// (react-aria's `builtinValidation`).
    pub builtin_validation: Signal<Option<ValidationResult>>,

    /// Validation behavior mode.
    pub validation_behavior: ValidationBehavior,

    /// The names of the field's form values (usually its `name`; a range field: its start's and
    /// end's): it shows the server errors of a [`FormValidationContext`] under any of them.
    pub names: Vec<String>,
}

/// The readers of the native validity of the inputs validated with a state (registered by
/// [`use_form_validation`](super::use_form_validation::use_form_validation)). A commit runs them
/// first, so it shows the inputs' current validity.
#[derive(Debug, Clone, Copy)]
pub struct NativeValidityReaders {
    readers: StoredValue<Vec<(u64, Callback<()>)>>,
    next_id: StoredValue<u64>,
}

impl NativeValidityReaders {
    fn new() -> Self {
        Self {
            readers: StoredValue::new(Vec::new()),
            next_id: StoredValue::new(0),
        }
    }

    /// Register `reader` until the current owner is cleaned up.
    pub(crate) fn register(&self, reader: Callback<()>) {
        let id = self.next_id.get_value();
        self.next_id.set_value(id + 1);
        self.readers
            .update_value(|readers| readers.push((id, reader)));
        let readers = self.readers;
        on_cleanup(move || {
            readers.try_update_value(|readers| readers.retain(|(other, _)| *other != id));
        });
    }

    fn read_all(&self) {
        // Collected first: readers update the validation, which must not find this borrowed.
        let readers: Vec<Callback<()>> = self
            .readers
            .with_value(|readers| readers.iter().map(|(_, reader)| *reader).collect());
        for reader in readers {
            reader.run(());
        }
    }
}

/// The validation state of a field (react-stately's `FormValidationState`), from
/// [`use_form_validation_state`]: read-only signals and methods. `Copy`.
#[derive(Debug, Clone, Copy)]
pub struct FormValidationState {
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

    update_validation: Callback<ValidationResult>,
    reset_validation: Callback<()>,
    commit_validation: Callback<()>,

    /// Readers of the native validity of the validated inputs, run before each commit.
    pub(crate) native_validity_readers: NativeValidityReaders,
}

impl FormValidationState {
    /// A state showing `display_validation`, whose operations are the given callbacks (a
    /// checkbox in a group: its own realtime validation, the group's commit and reset).
    pub(crate) fn from_parts(
        realtime_validation: Signal<ValidationResult>,
        display_validation: Signal<ValidationResult>,
        update_validation: Callback<ValidationResult>,
        group: Self,
    ) -> Self {
        Self {
            realtime_validation,
            display_validation,
            is_invalid: Memo::new(move |_| display_validation.with(|v| v.is_invalid)).into(),
            validation_errors: Memo::new(move |_| {
                display_validation.with(|v| v.validation_errors.clone())
            })
            .into(),
            update_validation,
            reset_validation: group.reset_validation,
            commit_validation: group.commit_validation,
            native_validity_readers: group.native_validity_readers,
        }
    }

    /// Updates the committed validation result (e.g. from the input's native validity). With
    /// `Aria` validation it shows right away; with `Native` validation on the next
    /// [`commit_validation`](Self::commit_validation).
    pub fn update_validation(&self, result: ValidationResult) {
        self.update_validation.run(result);
    }

    /// Resets the displayed validation to valid (on form reset).
    pub fn reset_validation(&self) {
        self.reset_validation.run(());
    }

    /// Commits the realtime validation so that it is displayed (on change or submission), and
    /// clears the server errors (the user changed the value).
    pub fn commit_validation(&self) {
        self.commit_validation.run(());
    }
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
/// 1. **Explicit** — `is_invalid` while `true`
/// 2. **Server** — errors from [`FormValidationContext`] under the field's `names`
/// 3. **Client** — custom `validate` function
/// 4. **Committed** — native validity read via
///    [`update_validation`](FormValidationState::update_validation)
///
/// # Validation Behavior
///
/// - [`ValidationBehavior::Aria`] — all errors displayed in realtime
/// - [`ValidationBehavior::Native`] — client/native errors deferred until
///   [`commit_validation`](FormValidationState::commit_validation)
#[allow(clippy::too_many_lines)]
pub fn use_form_validation_state<T>(input: UseFormValidationStateInput<T>) -> FormValidationState
where
    T: Clone + PartialEq + Send + Sync + 'static,
{
    let UseFormValidationStateInput {
        is_invalid,
        value,
        validate,
        builtin_validation,
        validation_behavior,
        names,
    } = input;
    // A valid result is no error.
    let builtin_validation =
        Memo::new(move |_| builtin_validation.get().filter(|result| result.is_invalid));

    // Store validate function for closure capture.
    let validate = StoredValue::new(validate);
    let names = StoredValue::new(names);

    // ---- Explicit error ----
    // `is_invalid` marks the field invalid, taking precedence over the other sources.
    let controlled_error: Memo<Option<ValidationResult>> = Memo::new(move |_| {
        is_invalid.get().then(|| ValidationResult {
            is_invalid: true,
            validation_errors: vec![],
            validation_details: CUSTOM_VALIDITY_STATE,
        })
    });

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

    // Every new set of server errors (a new response, even one equal to the last) shows again
    // (react-aria: every new errors object).
    if let Some(ctx) = server_context {
        Effect::new(move |previous: Option<()>| {
            ctx.errors.track();
            if previous.is_some() {
                set_server_error_cleared.set(false);
            }
        });
    }

    let server_error: Memo<Option<ValidationResult>> = Memo::new(move |_| {
        if server_error_cleared.get() {
            return None;
        }
        let ctx = server_context?;
        let field_errors: Vec<String> = ctx.errors.with(|errors| {
            names.with_value(|names| {
                names
                    .iter()
                    .filter_map(|name| errors.get(name))
                    .flatten()
                    .cloned()
                    .collect()
            })
        });
        (!field_errors.is_empty()).then_some(ValidationResult {
            is_invalid: true,
            validation_errors: field_errors,
            validation_details: CUSTOM_VALIDITY_STATE,
        })
    });

    // ---- Committed validation (for native mode deferred display) ----
    let (current_validity, set_current_validity) = signal(DEFAULT_VALIDATION_RESULT);
    let next_validation = StoredValue::new(DEFAULT_VALIDATION_RESULT);
    let last_error = StoredValue::new(DEFAULT_VALIDATION_RESULT);
    let (commit_queued, set_commit_queued) = signal(false);
    let commit_trigger = Trigger::new();
    let native_validity_readers = NativeValidityReaders::new();

    // Commit effect: when commit is queued, read latest validation and display it.
    Effect::new(move |_| {
        commit_trigger.track();
        if !commit_queued.get_untracked() {
            return;
        }
        // This effect can run before the input's property render effect, especially when the
        // field's state is composed from other states. Read native validity after the complete
        // render turn, as react-aria does in its post-render validation effect.
        leptos::task::spawn_local_scoped_with_cancellation(async move {
            leptos::task::tick().await;
            if !commit_queued.get_untracked() {
                return;
            }
            set_commit_queued.set(false);
            native_validity_readers.read_all();
            let error = client_error
                .get_untracked()
                .or_else(|| builtin_validation.get_untracked())
                .unwrap_or_else(|| next_validation.get_value());
            if error != last_error.get_value() {
                last_error.set_value(error.clone());
                set_current_validity.set(error);
            }
        });
    });

    // ---- Realtime validation ----
    // Priority: controlled > server > client > default. Memos: every field reads them several
    // times (ARIA attributes, error message, native validity).
    let realtime_validation = Memo::new(move |_| {
        controlled_error
            .get()
            .or_else(|| server_error.get())
            .or_else(|| client_error.get())
            .or_else(|| builtin_validation.get())
            .unwrap_or(DEFAULT_VALIDATION_RESULT)
    });

    // ---- Display validation ----
    // Aria: all errors shown in realtime.
    // Native: client/native errors deferred until commit.
    let display_validation = Memo::new(move |_| match validation_behavior {
        ValidationBehavior::Native => controlled_error
            .get()
            .or_else(|| server_error.get())
            .unwrap_or_else(|| current_validity.get()),
        ValidationBehavior::Aria => controlled_error
            .get()
            .or_else(|| server_error.get())
            .or_else(|| client_error.get())
            .or_else(|| builtin_validation.get())
            .unwrap_or_else(|| current_validity.get()),
    });

    // ---- Convenience derived signals ----
    let result_is_invalid = Memo::new(move |_| display_validation.with(|v| v.is_invalid));
    let result_validation_errors =
        Memo::new(move |_| display_validation.with(|v| v.validation_errors.clone()));

    FormValidationState {
        realtime_validation: realtime_validation.into(),
        display_validation: display_validation.into(),
        is_invalid: result_is_invalid.into(),
        validation_errors: result_validation_errors.into(),
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
        native_validity_readers,
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

#[cfg(test)]
mod tests {
    // Upstream has no tests of `useFormValidationState` itself; these follow its implementation
    // (sources, their priority, the deferred commit of `validationBehavior="native"`).
    use assertr::prelude::*;

    use super::*;
    use crate::testing::{flush_effects, with_owner};

    fn input<T: Send + Sync + 'static>(
        value: Signal<T>,
        validation_behavior: ValidationBehavior,
    ) -> UseFormValidationStateInput<T> {
        UseFormValidationStateInput {
            is_invalid: Signal::default(),
            value,
            validate: None,
            builtin_validation: Signal::default(),
            validation_behavior,
            names: Vec::new(),
        }
    }

    fn required() -> ValidateFn<String> {
        Arc::new(|value: &String| {
            if value.is_empty() {
                Err(vec!["Required".to_owned()])
            } else {
                Ok(())
            }
        })
    }

    fn invalid(errors: &[&str]) -> ValidationResult {
        ValidationResult {
            is_invalid: true,
            validation_errors: errors.iter().map(|&e| e.to_owned()).collect(),
            validation_details: CUSTOM_VALIDITY_STATE,
        }
    }

    /// What a native input reports when it is `required` and empty.
    fn value_missing() -> ValidationResult {
        ValidationResult {
            is_invalid: true,
            validation_errors: vec!["Fill out this field".to_owned()],
            validation_details: ValidityStateSnapshot {
                value_missing: true,
                valid: false,
                ..VALID_VALIDITY_STATE
            },
        }
    }

    #[test]
    fn aria_shows_validate_errors_while_the_user_edits() {
        with_owner(|| {
            let value = RwSignal::new(String::new());
            let state = use_form_validation_state(UseFormValidationStateInput {
                validate: Some(required()),
                ..input(value.into(), ValidationBehavior::Aria)
            });
            flush_effects();
            assert_that!(state.display_validation.get_untracked())
                .is_equal_to(invalid(&["Required"]));
            assert_that!(state.realtime_validation.get_untracked())
                .is_equal_to(invalid(&["Required"]));
            assert_that!(state.is_invalid.get_untracked()).is_true();
            assert_that!(state.validation_errors.get_untracked())
                .is_equal_to(vec!["Required".to_owned()]);

            value.set("a".to_owned());
            assert_that!(state.display_validation.get_untracked())
                .is_equal_to(DEFAULT_VALIDATION_RESULT);
            assert_that!(state.realtime_validation.get_untracked())
                .is_equal_to(DEFAULT_VALIDATION_RESULT);
        });
    }

    #[test]
    fn native_shows_validate_errors_after_a_commit() {
        with_owner(|| {
            let value = RwSignal::new(String::new());
            let state = use_form_validation_state(UseFormValidationStateInput {
                validate: Some(required()),
                ..input(value.into(), ValidationBehavior::Native)
            });
            flush_effects();
            // Realtime (for the native input's custom validity), not displayed yet.
            assert_that!(state.realtime_validation.get_untracked())
                .is_equal_to(invalid(&["Required"]));
            assert_that!(state.is_invalid.get_untracked()).is_false();

            // Committed after the next render (an Effect).
            state.commit_validation();
            assert_that!(state.is_invalid.get_untracked()).is_false();
            flush_effects();
            assert_that!(state.display_validation.get_untracked())
                .is_equal_to(invalid(&["Required"]));

            // Fixed: displayed as invalid until the next commit.
            value.set("a".to_owned());
            assert_that!(state.realtime_validation.get_untracked())
                .is_equal_to(DEFAULT_VALIDATION_RESULT);
            assert_that!(state.is_invalid.get_untracked()).is_true();
            state.commit_validation();
            flush_effects();
            assert_that!(state.display_validation.get_untracked())
                .is_equal_to(DEFAULT_VALIDATION_RESULT);
        });
    }

    #[test]
    fn native_commit_reads_validity_after_the_render_turn() {
        with_owner(|| {
            let value = RwSignal::new(String::new());
            let rendered = StoredValue::new(String::new());
            let state = use_form_validation_state(input::<String>(
                value.into(),
                ValidationBehavior::Native,
            ));
            Effect::new(move |_| rendered.set_value(value.get()));
            state
                .native_validity_readers
                .register(Callback::new(move |()| {
                    state.update_validation(if rendered.with_value(String::is_empty) {
                        invalid(&["Required"])
                    } else {
                        DEFAULT_VALIDATION_RESULT
                    });
                }));
            flush_effects();
            state.commit_validation();
            flush_effects();
            assert_that!(state.is_invalid.get_untracked()).is_true();

            // Even when the commit effect is queued first, it must observe the property update
            // from the same event, not the input's previous native validity.
            state.commit_validation();
            value.set("present".to_owned());
            flush_effects();
            assert_that!(state.is_invalid.get_untracked()).is_false();
        });
    }

    #[test]
    fn is_invalid_marks_the_field_invalid_before_the_other_sources() {
        for behavior in [ValidationBehavior::Aria, ValidationBehavior::Native] {
            with_owner(|| {
                let value = RwSignal::new(String::new());
                let is_invalid = RwSignal::new(true);
                let state = use_form_validation_state(UseFormValidationStateInput {
                    is_invalid: is_invalid.into(),
                    validate: Some(required()),
                    ..input(value.into(), behavior)
                });
                flush_effects();
                // Without errors, shown at once (also with native validation).
                assert_that!(state.display_validation.get_untracked()).is_equal_to(invalid(&[]));
                assert_that!(state.realtime_validation.get_untracked()).is_equal_to(invalid(&[]));

                // `false` leaves the other sources in charge (see the deviations).
                is_invalid.set(false);
                assert_that!(state.realtime_validation.get_untracked())
                    .is_equal_to(invalid(&["Required"]));
                value.set("a".to_owned());
                assert_that!(state.display_validation.get_untracked())
                    .is_equal_to(DEFAULT_VALIDATION_RESULT);
            });
        }
    }

    #[test]
    fn an_empty_error_list_is_valid() {
        with_owner(|| {
            let state = use_form_validation_state(UseFormValidationStateInput {
                validate: Some(Arc::new(|_: &String| Err(Vec::new()))),
                ..input(Signal::stored(String::new()), ValidationBehavior::Aria)
            });
            assert_that!(state.realtime_validation.get_untracked())
                .is_equal_to(DEFAULT_VALIDATION_RESULT);
        });
    }

    #[test]
    fn updated_native_validity_shows_at_once_with_aria_and_on_commit_with_native() {
        with_owner(|| {
            let state =
                use_form_validation_state(input(Signal::stored(()), ValidationBehavior::Aria));
            flush_effects();
            state.update_validation(value_missing());
            assert_that!(state.display_validation.get_untracked()).is_equal_to(value_missing());
            // Not a realtime source: the native input reports it itself.
            assert_that!(state.realtime_validation.get_untracked())
                .is_equal_to(DEFAULT_VALIDATION_RESULT);
        });
        with_owner(|| {
            let state =
                use_form_validation_state(input(Signal::stored(()), ValidationBehavior::Native));
            flush_effects();
            state.update_validation(value_missing());
            flush_effects();
            assert_that!(state.is_invalid.get_untracked()).is_false();
            state.commit_validation();
            flush_effects();
            assert_that!(state.display_validation.get_untracked()).is_equal_to(value_missing());
        });
    }

    #[test]
    fn a_commit_shows_validate_then_builtin_then_native_validity() {
        with_owner(|| {
            let value = RwSignal::new(String::new());
            let builtin = RwSignal::new(Some(invalid(&["Out of range"])));
            let state = use_form_validation_state(UseFormValidationStateInput {
                validate: Some(required()),
                builtin_validation: builtin.into(),
                ..input(value.into(), ValidationBehavior::Native)
            });
            flush_effects();
            state.update_validation(value_missing());
            let commit = || {
                state.commit_validation();
                flush_effects();
                state.display_validation.get_untracked()
            };
            assert_that!(commit()).is_equal_to(invalid(&["Required"]));
            value.set("a".to_owned());
            assert_that!(commit()).is_equal_to(invalid(&["Out of range"]));
            builtin.set(None);
            assert_that!(commit()).is_equal_to(value_missing());
        });
    }

    #[test]
    fn a_valid_builtin_validation_is_no_source() {
        with_owner(|| {
            let builtin = RwSignal::new(Some(invalid(&["Out of range"])));
            let state = use_form_validation_state(UseFormValidationStateInput {
                builtin_validation: builtin.into(),
                ..input(Signal::stored(()), ValidationBehavior::Aria)
            });
            flush_effects();
            assert_that!(state.realtime_validation.get_untracked())
                .is_equal_to(invalid(&["Out of range"]));
            state.update_validation(value_missing());
            assert_that!(state.display_validation.get_untracked())
                .is_equal_to(invalid(&["Out of range"]));

            // Valid: the next source (the native validity) shows.
            builtin.set(Some(DEFAULT_VALIDATION_RESULT));
            assert_that!(state.display_validation.get_untracked()).is_equal_to(value_missing());
        });
    }

    #[test]
    fn a_reset_shows_valid_and_cancels_a_pending_commit() {
        with_owner(|| {
            let state = use_form_validation_state(UseFormValidationStateInput {
                validate: Some(required()),
                ..input(Signal::stored(String::new()), ValidationBehavior::Native)
            });
            flush_effects();
            state.commit_validation();
            flush_effects();
            assert_that!(state.is_invalid.get_untracked()).is_true();

            // Valid although the value isn't (until the next submit).
            state.reset_validation();
            assert_that!(state.display_validation.get_untracked())
                .is_equal_to(DEFAULT_VALIDATION_RESULT);

            // A commit right before the reset (e.g. from the reset's change): canceled.
            state.commit_validation();
            state.reset_validation();
            flush_effects();
            assert_that!(state.is_invalid.get_untracked()).is_false();

            // The next commit shows the error again.
            state.commit_validation();
            flush_effects();
            assert_that!(state.display_validation.get_untracked())
                .is_equal_to(invalid(&["Required"]));
        });
    }

    #[test]
    fn server_errors_show_until_the_user_changes_the_value() {
        with_owner(|| {
            let errors = RwSignal::new(HashMap::from([(
                "email".to_owned(),
                vec!["Already taken".to_owned()],
            )]));
            provide_context(FormValidationContext {
                errors: errors.into(),
            });
            let field = |name: Option<&str>| {
                use_form_validation_state(UseFormValidationStateInput {
                    validate: Some(required()),
                    names: name.map(str::to_owned).into_iter().collect(),
                    ..input(Signal::stored(String::new()), ValidationBehavior::Native)
                })
            };
            let state = field(Some("email"));
            let other = field(Some("name"));
            let unnamed = field(None);
            flush_effects();
            // Shown at once (also with native validation), before `validate`'s errors.
            assert_that!(state.display_validation.get_untracked())
                .is_equal_to(invalid(&["Already taken"]));
            assert_that!(state.realtime_validation.get_untracked())
                .is_equal_to(invalid(&["Already taken"]));
            assert_that!(other.is_invalid.get_untracked()).is_false();
            assert_that!(unnamed.is_invalid.get_untracked()).is_false();

            // The user changed the value: cleared.
            state.commit_validation();
            flush_effects();
            assert_that!(state.display_validation.get_untracked())
                .is_equal_to(invalid(&["Required"]));

            // A new server response (with the same errors): shown again.
            errors.set(errors.get_untracked());
            flush_effects();
            assert_that!(state.display_validation.get_untracked())
                .is_equal_to(invalid(&["Already taken"]));

            // A form reset clears them too.
            state.reset_validation();
            assert_that!(state.display_validation.get_untracked())
                .is_equal_to(DEFAULT_VALIDATION_RESULT);
        });
    }

    // react-aria's `name` list (a date range picker's start and end names): the errors under
    // every name, in the order of the names.
    #[test]
    fn server_errors_of_several_names_are_joined() {
        with_owner(|| {
            provide_context(FormValidationContext {
                errors: Signal::stored(HashMap::from([
                    ("end".to_owned(), vec!["End too late".to_owned()]),
                    ("start".to_owned(), vec!["Start too early".to_owned()]),
                ])),
            });
            let state = use_form_validation_state(UseFormValidationStateInput {
                names: vec!["start".to_owned(), "end".to_owned()],
                ..input(Signal::stored(String::new()), ValidationBehavior::Aria)
            });
            flush_effects();
            assert_that!(state.display_validation.get_untracked())
                .is_equal_to(invalid(&["Start too early", "End too late"]));
        });
    }

    #[test]
    fn merge_validation_combines_errors_and_details() {
        let merged = merge_validation(&[
            invalid(&["A", "B"]),
            value_missing(),
            invalid(&["B"]),
            DEFAULT_VALIDATION_RESULT,
        ]);
        assert_that!(merged).is_equal_to(ValidationResult {
            is_invalid: true,
            validation_errors: vec![
                "A".to_owned(),
                "B".to_owned(),
                "Fill out this field".to_owned(),
            ],
            validation_details: ValidityStateSnapshot {
                custom_error: true,
                value_missing: true,
                valid: false,
                ..VALID_VALIDITY_STATE
            },
        });
        assert_that!(merge_validation(&[
            DEFAULT_VALIDATION_RESULT,
            DEFAULT_VALIDATION_RESULT
        ]))
        .is_equal_to(DEFAULT_VALIDATION_RESULT);
    }
}
