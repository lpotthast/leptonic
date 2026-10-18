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

use crate::utils::probe::probe_sources;

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
//   (`register_native_validity_reader`; react-aria re-reads it after every render). Only
//   `ValidationBehavior::Native` states have it (react-aria queues commits only for them).
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

// Manual: a derive would require `T: Clone`/`T: Debug`, and `validate` is a closure.
impl<T: Send + Sync + 'static> Clone for UseFormValidationStateInput<T> {
    fn clone(&self) -> Self {
        Self {
            is_invalid: self.is_invalid,
            value: self.value,
            validate: self.validate.clone(),
            builtin_validation: self.builtin_validation,
            validation_behavior: self.validation_behavior,
            names: self.names.clone(),
        }
    }
}

impl<T: Send + Sync + 'static> std::fmt::Debug for UseFormValidationStateInput<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UseFormValidationStateInput")
            .field("is_invalid", &self.is_invalid)
            .field("value", &self.value)
            .field("validate", &self.validate.as_ref().map(|_| "Fn"))
            .field("builtin_validation", &self.builtin_validation)
            .field("validation_behavior", &self.validation_behavior)
            .field("names", &self.names)
            .finish()
    }
}

/// A reader of an input's native validity ([`FormValidationState::register_native_validity_reader`]).
type NativeValidityReader = Arc<dyn Fn() + Send + Sync>;

/// The registered readers, with the id of the next one.
#[derive(Default)]
struct ReaderList {
    readers: Vec<(u64, NativeValidityReader)>,
    next_id: u64,
}

/// The readers of the native validity of the inputs validated with a state
/// ([`FormValidationState::register_native_validity_reader`]). A commit runs them first, so it
/// shows the inputs' current validity. Only `ValidationBehavior::Native` commits read them.
#[derive(Clone, Copy)]
struct NativeValidityReaders(StoredValue<ReaderList>);

impl std::fmt::Debug for NativeValidityReaders {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("NativeValidityReaders")
    }
}

impl NativeValidityReaders {
    fn new() -> Self {
        Self(StoredValue::new(ReaderList::default()))
    }

    /// Register `reader` until the current owner is cleaned up.
    fn register(self, reader: NativeValidityReader) {
        let registered = self.0.try_update_value(|list| {
            let id = list.next_id;
            list.next_id += 1;
            list.readers.push((id, reader));
            id
        });
        let Some(id) = registered else {
            return;
        };
        let list = self.0;
        on_cleanup(move || {
            list.try_update_value(|list| list.readers.retain(|(other, _)| *other != id));
        });
    }

    fn read_all(self) {
        // Collected first: readers update the validation, which must not find this borrowed.
        let readers: Vec<NativeValidityReader> = self.0.with_value(|list| {
            list.readers
                .iter()
                .map(|(_, reader)| Arc::clone(reader))
                .collect()
        });
        for reader in readers {
            reader();
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

    /// Readers of the native validity of the validated inputs, run before each commit (only
    /// with `ValidationBehavior::Native`, whose commits read them).
    native_validity_readers: Option<NativeValidityReaders>,
}

/// Whether a validation result is invalid (one closure type for all states: R13).
fn result_is_invalid(display_validation: Signal<ValidationResult>) -> Signal<bool> {
    Signal::derive(move || display_validation.with(|v| v.is_invalid))
}

/// The error messages of a validation result.
fn result_errors(display_validation: Signal<ValidationResult>) -> Signal<Vec<String>> {
    Signal::derive(move || display_validation.with(|v| v.validation_errors.clone()))
}

impl FormValidationState {
    /// A state whose operations are the given callbacks (a checkbox in a group: its own realtime
    /// validation and validity, the group's commit and reset). It shows `display_validation`, or
    /// with `None` the group's displayed validation.
    pub fn from_parts(
        realtime_validation: Signal<ValidationResult>,
        display_validation: Option<Signal<ValidationResult>>,
        update_validation: Callback<ValidationResult>,
        group: Self,
    ) -> Self {
        let (display_validation, is_invalid, validation_errors) = match display_validation {
            Some(display) => (display, result_is_invalid(display), result_errors(display)),
            None => (
                group.display_validation,
                group.is_invalid,
                group.validation_errors,
            ),
        };
        Self {
            realtime_validation,
            display_validation,
            is_invalid,
            validation_errors,
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

    /// Registers `reader` until the current owner is cleaned up: every commit runs it first, so
    /// that the committed validation is the input's current native validity, also when it
    /// changed without the value changing (a `min` attribute, the browser's own checks of a
    /// `type="email"`). `reader` reads the input's validity and passes it to
    /// [`update_validation`](Self::update_validation). Only `ValidationBehavior::Native` commits
    /// read the native validity (an `Aria` state shows updates at once): for an `Aria` state,
    /// this registers nothing.
    ///
    /// [`use_form_validation`](super::use_form_validation::use_form_validation) registers one
    /// for its input with `ValidationBehavior::Native`; register one of your own for a field that
    /// validates further constraints natively (`use_number_field` checks its range and step as a
    /// native number input would).
    ///
    /// ```ignore
    /// state.register_native_validity_reader(move || {
    ///     if let Some(input) = input_element.get_untracked() {
    ///         state.update_validation(read_validity(&input));
    ///     }
    /// });
    /// ```
    pub fn register_native_validity_reader(&self, reader: impl Fn() + Send + Sync + 'static) {
        if let Some(readers) = self.native_validity_readers {
            readers.register(Arc::new(reader));
        }
    }
}

/// Manages form validation state with multiple validation sources.
///
/// This is the state (stately) layer of form validation. It does not interact
/// with the DOM: pair it with
/// [`use_form_validation`](super::use_form_validation::use_form_validation)
/// to connect validation to native input elements.
///
/// # Validation Sources (in priority order)
///
/// 1. **Explicit**: `is_invalid` while `true`
/// 2. **Server**: errors from [`FormValidationContext`] under the field's `names`
/// 3. **Client**: custom `validate` function
/// 4. **Built-in**: the field's own `builtin_validation`
/// 5. **Committed**: native validity read via
///    [`update_validation`](FormValidationState::update_validation)
///
/// # Validation Behavior
///
/// - [`ValidationBehavior::Aria`]: all errors displayed in realtime
/// - [`ValidationBehavior::Native`]: client/native errors deferred until
///   [`commit_validation`](FormValidationState::commit_validation)
pub fn use_form_validation_state<T>(input: UseFormValidationStateInput<T>) -> FormValidationState
where
    T: Clone + PartialEq + Send + Sync + 'static,
{
    crate::metrics::subject!();
    let UseFormValidationStateInput {
        is_invalid,
        value,
        validate,
        builtin_validation,
        validation_behavior,
        names,
    } = input;
    // A source of errors that can never report one is left out: no memo, no effect
    // (`performance.md` R14). Most fields have none: `is_invalid` constantly `false`, no
    // `validate`, no name or no `FormValidationContext`, no builtin validation or a constant valid
    // one. `can_report` tells it from a probe of whether the source reports an error now.
    let can_report = |(reports_now, is_reactive): (bool, bool)| reports_now || is_reactive;

    // A valid result is no error.
    let builtin_validation = can_report(probe_sources(|| {
        builtin_validation.with(|result| result.as_ref().is_some_and(|r| r.is_invalid))
    }))
    .then(|| Memo::new(move |_| builtin_validation.get().filter(|result| result.is_invalid)));

    // ---- Explicit error ----
    // `is_invalid` marks the field invalid, taking precedence over the other sources.
    let controlled_error = can_report(probe_sources(|| is_invalid.get())).then(|| {
        Memo::new(move |_| {
            is_invalid.get().then(|| ValidationResult {
                is_invalid: true,
                validation_errors: vec![],
                validation_details: CUSTOM_VALIDITY_STATE,
            })
        })
    });

    // ---- Client validation ----
    let client_error = validate.map(|validate_fn| {
        Memo::new(move |_| {
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
        })
    });

    // ---- Server errors ----
    // Only for a named field inside a `FormValidationContext`.
    let server = use_context::<FormValidationContext>()
        .filter(|_| !names.is_empty())
        .map(|ctx| {
            let (server_error_cleared, set_server_error_cleared) = signal(false);
            // Every new set of server errors (a new response, even one equal to the last) shows
            // again (react-aria: every new errors object).
            Effect::new(move |previous: Option<()>| {
                ctx.errors.track();
                if previous.is_some() {
                    set_server_error_cleared.set(false);
                }
            });
            let server_error: Memo<Option<ValidationResult>> = Memo::new(move |_| {
                if server_error_cleared.get() {
                    return None;
                }
                let field_errors: Vec<String> = ctx.errors.with(|errors| {
                    names
                        .iter()
                        .filter_map(|name| errors.get(name))
                        .flatten()
                        .cloned()
                        .collect()
                });
                (!field_errors.is_empty()).then_some(ValidationResult {
                    is_invalid: true,
                    validation_errors: field_errors,
                    validation_details: CUSTOM_VALIDITY_STATE,
                })
            });
            (server_error, set_server_error_cleared)
        });
    let server_error = server.map(|(server_error, _)| server_error);
    let clear_server_error = move || {
        if let Some((_, set_server_error_cleared)) = server {
            set_server_error_cleared.set(true);
        }
    };
    let read = |source: Option<Memo<Option<ValidationResult>>>| source.and_then(|m| m.get());
    let read_untracked =
        |source: Option<Memo<Option<ValidationResult>>>| source.and_then(|m| m.get_untracked());

    // ---- Committed validation (for native mode deferred display) ----
    let current_validity = RwSignal::new(DEFAULT_VALIDATION_RESULT);
    // Only `Native` validation commits (react-aria queues commits for it only): an `Aria` state
    // has no commit state, effect or native validity readers (`performance.md` R14).
    let native = (validation_behavior == ValidationBehavior::Native).then(|| {
        let commit = StoredValue::new(NativeCommit::default());
        let trigger = Trigger::new();
        let readers = NativeValidityReaders::new();
        // Commit effect: when commit is queued, read latest validation and display it.
        Effect::new(move |_| {
            trigger.track();
            if !commit.with_value(|commit| commit.queued) {
                return;
            }
            // This effect can run before the input's property render effect, especially when
            // the field's state is composed from other states. Read native validity after the
            // complete render turn, as react-aria does in its post-render validation effect.
            leptos::task::spawn_local_scoped_with_cancellation(async move {
                leptos::task::tick().await;
                let queued = commit.try_update_value(|commit| std::mem::take(&mut commit.queued));
                if queued != Some(true) {
                    return;
                }
                readers.read_all();
                let error = read_untracked(client_error)
                    .or_else(|| read_untracked(builtin_validation))
                    .unwrap_or_else(|| commit.with_value(|commit| commit.next.clone()));
                let changed = commit.try_update_value(|commit| {
                    let changed = error != commit.last_error;
                    if changed {
                        commit.last_error.clone_from(&error);
                    }
                    changed
                });
                if changed == Some(true) {
                    current_validity.set(error);
                }
            });
        });
        (commit, trigger, readers)
    });

    // ---- Realtime validation ----
    // Priority: controlled > server > client > builtin > valid. Memos: every field reads them
    // several times (ARIA attributes, error message, native validity). Without any source: valid.
    let has_sources = controlled_error.is_some()
        || server_error.is_some()
        || client_error.is_some()
        || builtin_validation.is_some();
    let realtime_validation: Signal<ValidationResult> = if has_sources {
        Memo::new(move |_| {
            read(controlled_error)
                .or_else(|| read(server_error))
                .or_else(|| read(client_error))
                .or_else(|| read(builtin_validation))
                .unwrap_or(DEFAULT_VALIDATION_RESULT)
        })
        .into()
    } else {
        Signal::stored(DEFAULT_VALIDATION_RESULT)
    };

    // ---- Display validation ----
    // Aria: all errors shown in realtime.
    // Native: client/native errors deferred until commit.
    // Without a source shown at once: what was committed or reported (`update_validation`).
    let shown_at_once = match validation_behavior {
        ValidationBehavior::Native => controlled_error.is_some() || server_error.is_some(),
        ValidationBehavior::Aria => has_sources,
    };
    let display_validation: Signal<ValidationResult> = if shown_at_once {
        Memo::new(move |_| match validation_behavior {
            ValidationBehavior::Native => read(controlled_error)
                .or_else(|| read(server_error))
                .unwrap_or_else(|| current_validity.get()),
            ValidationBehavior::Aria => read(controlled_error)
                .or_else(|| read(server_error))
                .or_else(|| read(client_error))
                .or_else(|| read(builtin_validation))
                .unwrap_or_else(|| current_validity.get()),
        })
        .into()
    } else {
        current_validity.into()
    };

    FormValidationState {
        realtime_validation,
        display_validation,
        is_invalid: result_is_invalid(display_validation),
        validation_errors: result_errors(display_validation),
        update_validation: Callback::new(move |result: ValidationResult| {
            // In Aria mode, update displayed validation immediately.
            // In Native mode, queue for next commit.
            match native {
                None => {
                    if current_validity.with_untracked(|current| *current != result) {
                        current_validity.set(result);
                    }
                }
                Some((commit, _, _)) => {
                    commit.update_value(|commit| commit.next = result);
                }
            }
        }),
        reset_validation: Callback::new(move |()| {
            // Reset displayed validation to valid, and cancel a pending commit. Only commits
            // (`Native`) set the last error: an `Aria` reset leaves the validity reported with
            // `update_validation` (react-aria).
            if let Some((commit, _, _)) = native {
                let changed = commit.try_update_value(|commit| {
                    commit.queued = false;
                    let changed = commit.last_error != DEFAULT_VALIDATION_RESULT;
                    if changed {
                        commit.last_error = DEFAULT_VALIDATION_RESULT;
                    }
                    changed
                });
                if changed == Some(true) {
                    current_validity.set(DEFAULT_VALIDATION_RESULT);
                }
            }
            clear_server_error();
        }),
        commit_validation: Callback::new(move |()| {
            // Queue commit so display_validation shows latest errors.
            if let Some((commit, trigger, _)) = native {
                commit.update_value(|commit| commit.queued = true);
                trigger.notify();
            }
            // Clear server errors (user changed their value).
            clear_server_error();
        }),
        native_validity_readers: native.map(|(_, _, readers)| readers),
    }
}

/// The commit state of a `Native` validation (one `StoredValue`: `performance.md` R7).
struct NativeCommit {
    /// The validity reported with `update_validation`, shown on the next commit.
    next: ValidationResult,
    /// The result shown by the last commit (or reset).
    last_error: ValidationResult,
    /// Whether a commit waits for the end of the render turn.
    queued: bool,
}

impl Default for NativeCommit {
    fn default() -> Self {
        Self {
            next: DEFAULT_VALIDATION_RESULT,
            last_error: DEFAULT_VALIDATION_RESULT,
            queued: false,
        }
    }
}

/// Merges multiple validation results into one.
///
/// Combines all error messages (deduplicated) and ORs all validity details.
pub fn merge_validation<'a>(
    results: impl IntoIterator<Item = &'a ValidationResult>,
) -> ValidationResult {
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
    fn a_native_validity_reader_runs_on_each_commit_until_its_owner_is_cleaned_up() {
        with_owner(|| {
            let state = use_form_validation_state(input::<String>(
                Signal::stored(String::new()),
                ValidationBehavior::Native,
            ));
            let reads = StoredValue::new(0);
            let reader_owner = Owner::new();
            reader_owner.with(|| {
                state.register_native_validity_reader(move || {
                    reads.update_value(|r| *r += 1);
                });
            });
            state.commit_validation();
            flush_effects();
            assert_that!(reads.get_value()).is_equal_to(1);
            reader_owner.cleanup();
            state.commit_validation();
            flush_effects();
            assert_that!(reads.get_value()).is_equal_to(1);
        });
    }

    // react-stately queues commits for native validation only, and only commits set the last
    // error, which a reset compares with.
    #[test]
    fn an_aria_state_has_no_commit_and_its_reset_keeps_the_reported_validity() {
        with_owner(|| {
            let state =
                use_form_validation_state(input(Signal::stored(()), ValidationBehavior::Aria));
            let reads = StoredValue::new(0);
            state.register_native_validity_reader(move || reads.update_value(|r| *r += 1));
            state.update_validation(value_missing());
            state.commit_validation();
            flush_effects();
            assert_that!(reads.get_value()).is_equal_to(0);
            assert_that!(state.display_validation.get_untracked()).is_equal_to(value_missing());
            state.reset_validation();
            assert_that!(state.display_validation.get_untracked()).is_equal_to(value_missing());
            assert_that!(state.is_invalid.get_untracked()).is_true();
            assert_that!(state.validation_errors.get_untracked())
                .is_equal_to(vec!["Fill out this field".to_owned()]);
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
            state.register_native_validity_reader(move || {
                state.update_validation(if rendered.with_value(String::is_empty) {
                    invalid(&["Required"])
                } else {
                    DEFAULT_VALIDATION_RESULT
                });
            });
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

    /// Constant sources are folded at creation (`performance.md` R14): those reporting an error
    /// still show, the others leave the committed validation in charge.
    #[test]
    fn constant_sources_show_their_errors() {
        for behavior in [ValidationBehavior::Aria, ValidationBehavior::Native] {
            with_owner(|| {
                let marked = use_form_validation_state(UseFormValidationStateInput {
                    is_invalid: Signal::stored(true),
                    ..input(Signal::stored(()), behavior)
                });
                assert_that!(marked.display_validation.get_untracked()).is_equal_to(invalid(&[]));
                assert_that!(marked.realtime_validation.get_untracked()).is_equal_to(invalid(&[]));

                let builtin = use_form_validation_state(UseFormValidationStateInput {
                    builtin_validation: Signal::stored(Some(invalid(&["Out of range"]))),
                    ..input(Signal::stored(()), behavior)
                });
                assert_that!(builtin.realtime_validation.get_untracked())
                    .is_equal_to(invalid(&["Out of range"]));
                builtin.commit_validation();
                flush_effects();
                assert_that!(builtin.display_validation.get_untracked())
                    .is_equal_to(invalid(&["Out of range"]));

                let valid = use_form_validation_state(UseFormValidationStateInput {
                    builtin_validation: Signal::stored(Some(DEFAULT_VALIDATION_RESULT)),
                    ..input(Signal::stored(()), behavior)
                });
                assert_that!(valid.realtime_validation.get_untracked())
                    .is_equal_to(DEFAULT_VALIDATION_RESULT);
                valid.update_validation(value_missing());
                valid.commit_validation();
                flush_effects();
                assert_that!(valid.display_validation.get_untracked()).is_equal_to(value_missing());
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
