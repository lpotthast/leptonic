// Upstream: react-aria/src/textfield/useFormattedTextField.ts @ 99e6102368
use leptos::prelude::*;
use web_sys::{CompositionEvent, InputEvent};

use crate::utils::{CapturedElement, EventHandler};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Returns the input's `beforeinput` and composition handlers, to merge with the text field's
//   props (react-aria: calls `useTextField` and returns its merged result).
//
// =============================================================================

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
/// `validate` before the browser applies them, and composed text (IMEs, autocorrect) is reverted
/// (through `set_input_value`) when it ends invalid.
pub fn use_formatted_text_field(
    element: CapturedElement,
    validate: Callback<String, bool>,
    set_input_value: Callback<String>,
) -> FormattedTextFieldHandlers {
    use wasm_bindgen::JsCast;

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
            NextValue::Text(text) => validate.run(text),
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
        if validate.run(input.value()) {
            return;
        }
        if let Some((value, start, end)) = composition_start.get_value() {
            input.set_value(&value);
            let _ = input.set_selection_range_with_direction(
                start.unwrap_or(0),
                end.unwrap_or(0),
                "none",
            );
            set_input_value.run(value);
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
