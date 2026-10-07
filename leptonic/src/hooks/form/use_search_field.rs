// Upstream: react-aria/src/searchfield/useSearchField.ts @ 99e6102368
// Upstream: react-stately/src/searchfield/useSearchFieldState.ts @ 99e6102368
use leptos::prelude::*;
use wasm_bindgen::JsCast;

use super::use_text_field::{UseTextFieldInput, UseTextFieldReturn, use_text_field};
use crate::{
    hooks::button::use_button::UseButtonInput,
    utils::{
        EventAccessors,
        keyboard_shortcut::{KeyboardShortcuts, Shortcut, ShortcutOutcome},
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - The input is the text field's (`UseSearchFieldInput::text_field`, `type="search"` by default),
//   and its state a `TextFieldState` (react-stately's `useSearchFieldState` has the same shape:
//   a value and its setter). Reason: one state type for all text fields; app state binds through
//   `UseTextFieldStateInput::value`.
// - Returns the clear button's `UseButtonInput` (compose with `use_button`) instead of button
//   props. Reason: inputs compose, DOM props don't (project convention).
// - Enter and Escape are keyboard shortcuts merged with the text field's own (`shortcuts`), the
//   search field's winning for those keys (react-aria: both keydown handlers run, the search
//   field's first).
//
// ## OMITTED FEATURES
// - Localized clear button label: "Clear search" is English until leptonic has a localized
//   string formatter.
//
// =============================================================================

/// Input of [`use_search_field`].
#[derive(Clone)]
pub struct UseSearchFieldInput {
    /// The text field (value state, labelling, validation, ...), its `input_type`
    /// `InputType::Search`.
    pub text_field: UseTextFieldInput,
    /// Called with the value when Enter is pressed. Without it, Enter submits the form.
    pub on_submit: Option<Callback<String>>,
    /// Called when Escape or the clear button empties the field.
    pub on_clear: Option<Callback<()>>,
}

/// Return value of [`use_search_field`].
pub struct UseSearchFieldReturn {
    /// The text field: label, input, description and error message props and validation.
    pub text_field: UseTextFieldReturn,
    /// The clear button's configuration, for [`use_button`](crate::hooks::use_button): out of the
    /// tab order, keeps focus in the input, empties the field.
    pub clear_button: UseButtonInput,
}

/// A search field: a text field where Enter submits the search and Escape (or a clear button)
/// empties it.
///
/// ```ignore
/// let state = use_text_field_state(UseTextFieldStateInput::default());
/// // `text_field`: a `UseTextFieldInput` of `state` (every field named), `input_type` `Search`.
/// let search = use_search_field(UseSearchFieldInput {
///     text_field,
///     on_submit: Some(Callback::new(|query: String| run(query))),
///     on_clear: None,
/// });
/// let clear = use_button(search.clear_button);
/// view! {
///     <input {..search.text_field.input_props.into_attrs()} />
///     <button {..clear.props.into_attrs()}>"✕"</button>
/// }
/// ```
pub fn use_search_field(input: UseSearchFieldInput) -> UseSearchFieldReturn {
    let UseSearchFieldInput {
        text_field,
        on_submit,
        on_clear,
    } = input;
    let state = text_field.state;
    let is_disabled = text_field.is_disabled;
    let is_read_only = text_field.is_read_only;
    let inactive = move || is_disabled.get_untracked() || is_read_only.get_untracked();

    let search_shortcuts = KeyboardShortcuts::new()
        .on(Shortcut::key("Enter"), move |_| {
            if inactive() {
                return ShortcutOutcome::Ignored;
            }
            match on_submit {
                Some(on_submit) => {
                    on_submit.run(state.value.get_untracked());
                    ShortcutOutcome::Handled
                }
                // Enter submits the form.
                None => ShortcutOutcome::Ignored,
            }
        })
        .on(Shortcut::key("Escape"), move |e| {
            if inactive() {
                return ShortcutOutcome::Ignored;
            }
            // The input's own value too, in case it was set on the element directly.
            let input_value = e
                .expect_target()
                .dyn_into::<web_sys::HtmlInputElement>()
                .ok()
                .map(|input| input.value())
                .unwrap_or_default();
            if state.value.with_untracked(String::is_empty) && input_value.is_empty() {
                return ShortcutOutcome::Ignored;
            }
            state.set_value(String::new());
            if let Some(on_clear) = on_clear {
                on_clear.run(());
            }
            ShortcutOutcome::Handled
        });
    let shortcuts = match text_field.shortcuts.clone() {
        Some(own) => own.with(search_shortcuts),
        None => search_shortcuts,
    };

    let text_field = use_text_field(UseTextFieldInput {
        shortcuts: Some(shortcuts),
        ..text_field
    });

    let element = text_field.element;
    let clear_button = UseButtonInput {
        aria_label: "Clear search".into(),
        is_disabled: Signal::derive(move || is_disabled.get() || is_read_only.get()),
        exclude_from_tab_order: Signal::stored(true),
        prevent_focus_on_press: true,
        // On press start, so touching the button doesn't blur the input and close the virtual
        // keyboard.
        on_press_start: Some(Callback::new(move |_| {
            if let Some(input) = element.get_untracked()
                && let Some(input) = input.dyn_ref::<web_sys::HtmlElement>()
            {
                let _ = input.focus();
            }
        })),
        on_press: Some(Callback::new(move |_| {
            state.set_value(String::new());
            if let Some(on_clear) = on_clear {
                on_clear.run(());
            }
        })),
        ..UseButtonInput::default()
    };

    UseSearchFieldReturn {
        text_field,
        clear_button,
    }
}
