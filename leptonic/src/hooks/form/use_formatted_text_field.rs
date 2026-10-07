// Upstream: react-aria/src/textfield/useFormattedTextField.ts @ 99e6102368
use leptos::prelude::*;
use web_sys::{CompositionEvent, InputEvent};

use super::use_number_field_state::NumberFieldState;
use crate::{
    hooks::ColorFieldState,
    utils::{CapturedElement, EventHandler, NumberValue, color::ColorValue},
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Returns the input's `beforeinput` and composition handlers, to merge with the text field's
//   props (react-aria: calls `useTextField` and returns its merged result).
// - The state is any [`FormattedTextState`] (react-aria: an object with `validate` and
//   `setInputValue`).
//
// =============================================================================

/// The text state of a field whose text must stay valid while typing (a number or hex color
/// field), for [`use_formatted_text_field`].
pub trait FormattedTextState: Copy + Send + Sync + 'static {
    /// Whether `text` may be typed: valid, or the beginning of a valid text.
    fn is_valid_text(&self, text: &str) -> bool;

    /// Sets the text, without committing it.
    fn set_text(&self, text: String);
}

impl<T: NumberValue> FormattedTextState for NumberFieldState<T> {
    fn is_valid_text(&self, text: &str) -> bool {
        self.validate(text.to_owned())
    }

    fn set_text(&self, text: String) {
        self.set_input_value(text);
    }
}

impl<C: ColorValue> FormattedTextState for ColorFieldState<C> {
    fn is_valid_text(&self, text: &str) -> bool {
        self.validate(text)
    }

    fn set_text(&self, text: String) {
        self.set_input_value(text);
    }
}

/// Input of [`use_formatted_text_field`].
#[derive(Debug, Clone, Copy)]
pub struct UseFormattedTextFieldInput<S: FormattedTextState> {
    /// The field's `<input>`.
    pub element: CapturedElement,
    /// The field's text state.
    pub state: S,
}

/// Handlers keeping a text field's text valid while typing, from [`use_formatted_text_field`].
#[derive(Debug, Clone)]
pub struct FormattedTextFieldHandlers {
    /// Rejects edits that would make the text invalid, before the browser applies them.
    pub on_beforeinput: EventHandler<InputEvent>,
    /// Remembers the text before a composition.
    pub on_compositionstart: EventHandler<CompositionEvent>,
    /// Restores the text from before a composition whose result is invalid.
    pub on_compositionend: EventHandler<CompositionEvent>,
}

/// Keeps the text of a field (number, hex color, ...) valid while typing: edits are checked with
/// the state's [`is_valid_text`](FormattedTextState::is_valid_text) before the browser applies
/// them, and composed text (IMEs, autocorrect) is reverted when it ends invalid.
pub fn use_formatted_text_field<S: FormattedTextState>(
    input: UseFormattedTextFieldInput<S>,
) -> FormattedTextFieldHandlers {
    use wasm_bindgen::JsCast;

    let UseFormattedTextFieldInput { element, state } = input;

    let input = move || {
        element
            .get_untracked()
            .and_then(|el| el.dyn_ref::<web_sys::HtmlInputElement>().cloned())
    };

    let on_beforeinput = EventHandler::new(move |e: InputEvent| {
        let Some(input) = input() else {
            return;
        };
        let allowed = match next_input_value(&input, &e.input_type(), e.data()) {
            NextValue::Allowed => true,
            NextValue::Text(text) => state.is_valid_text(&text),
            NextValue::Unknown => false,
        };
        if !allowed {
            e.prevent_default();
        }
    });

    // Composed text can't be rejected while composing: restore the text from before the
    // composition if the result is invalid.
    let composition_start = StoredValue::new(None::<(String, Option<u32>, Option<u32>)>);
    let on_compositionstart = EventHandler::new(move |_: CompositionEvent| {
        if let Some(input) = input() {
            composition_start.set_value(Some((
                input.value(),
                input.selection_start().ok().flatten(),
                input.selection_end().ok().flatten(),
            )));
        }
    });
    let on_compositionend = EventHandler::new(move |_: CompositionEvent| {
        let Some(input) = input() else {
            return;
        };
        if state.is_valid_text(&input.value()) {
            return;
        }
        if let Some((value, start, end)) = composition_start.get_value() {
            input.set_value(&value);
            let _ = input.set_selection_range_with_direction(
                start.unwrap_or(0),
                end.unwrap_or(0),
                "none",
            );
            state.set_text(value);
        }
    });

    FormattedTextFieldHandlers {
        on_beforeinput,
        on_compositionstart,
        on_compositionend,
    }
}

/// What a `beforeinput` would make of the input's text.
enum NextValue {
    /// Always allowed (undo/redo, line breaks submitting the form).
    Allowed,
    Text(String),
    /// Not computable: rejected.
    Unknown,
}

/// The input's text after the edit `input_type` (with `data`) would apply (react-aria's
/// `useFormattedTextField`). Selections are UTF-16 offsets.
fn next_input_value(
    input: &web_sys::HtmlInputElement,
    input_type: &str,
    data: Option<String>,
) -> NextValue {
    let value: Vec<u16> = input.value().encode_utf16().collect();
    let clamp = |offset: Option<u32>| {
        usize::try_from(offset.unwrap_or(0))
            .unwrap_or(usize::MAX)
            .min(value.len())
    };
    let start = clamp(input.selection_start().ok().flatten());
    let end = clamp(input.selection_end().ok().flatten()).max(start);
    let text = |parts: &[&[u16]]| String::from_utf16_lossy(&parts.concat());
    NextValue::Text(match input_type {
        "historyUndo" | "historyRedo" | "insertLineBreak" => return NextValue::Allowed,
        "deleteContentForward" if start == end => {
            text(&[&value[..start], &value[(end + 1).min(value.len())..]])
        }
        "deleteContentBackward" if start == end => {
            text(&[&value[..start.saturating_sub(1)], &value[start..]])
        }
        // Deleting the selection.
        "deleteContent"
        | "deleteByCut"
        | "deleteByDrag"
        | "deleteContentForward"
        | "deleteContentBackward" => text(&[&value[..start], &value[end..]]),
        "deleteSoftLineBackward" | "deleteHardLineBackward" => text(&[&value[start..]]),
        _ => match data {
            Some(data) => {
                let data: Vec<u16> = data.encode_utf16().collect();
                text(&[&value[..start], &data, &value[end..]])
            }
            None => return NextValue::Unknown,
        },
    })
}
