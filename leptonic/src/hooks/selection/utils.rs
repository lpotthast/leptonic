use web_sys::{Element, KeyboardEvent, MouseEvent, PointerEvent};

use crate::utils::{modifiers::Modifiers, platform::device::is_apple_device};

// This is based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/selection/src/utils.ts

/// Returns `true` if the Ctrl key (or Meta key on Apple) is pressed.
///
/// On Apple devices, Ctrl+key is used for system shortcuts, so we use Meta (Cmd)
/// as the "control" modifier. On other platforms, we use Ctrl.
pub fn is_ctrl_key_pressed(e: &KeyboardEvent) -> bool {
    if is_apple_device() {
        e.meta_key()
    } else {
        e.ctrl_key()
    }
}

/// Returns `true` if the modifier for non-contiguous selection is pressed.
///
/// Non-contiguous selection means toggling an individual item without affecting
/// other selected items (e.g., Ctrl+Click on Windows, Alt+Click on macOS).
///
/// On Apple devices this is the Alt (Option) key.
/// On other platforms this is the Ctrl key.
pub fn is_non_contiguous_selection_modifier_keyboard(e: &KeyboardEvent) -> bool {
    if is_apple_device() {
        e.alt_key()
    } else {
        e.ctrl_key()
    }
}

/// Returns `true` if the modifier for non-contiguous selection is pressed (mouse event variant).
pub fn is_non_contiguous_selection_modifier_mouse(e: &MouseEvent) -> bool {
    if is_apple_device() {
        e.alt_key()
    } else {
        e.ctrl_key()
    }
}

/// Returns `true` if the modifier for non-contiguous selection is active in the given modifiers.
///
/// This variant works with the `Modifiers` struct from `PressEvent`, allowing
/// `use_selectable_item` to check modifiers from `use_press` events.
pub fn is_non_contiguous_selection_modifier(m: &Modifiers) -> bool {
    if is_apple_device() {
        m.alt_key
    } else {
        m.ctrl_key
    }
}

/// Returns `true` if the modifier for non-contiguous selection is pressed (pointer event variant).
pub fn is_non_contiguous_selection_modifier_pointer(e: &PointerEvent) -> bool {
    if is_apple_device() {
        e.alt_key()
    } else {
        e.ctrl_key()
    }
}

/// Finds the DOM element for a collection item by its `data-key` attribute.
///
/// Items rendered via `use_selectable_item` set `data-key` on their root element.
/// This function queries the collection container to find the matching element.
///
/// When `collection_id` is provided, the query is scoped to elements with a matching
/// `data-collection` attribute, preventing false matches in nested collections.
pub fn get_item_element(
    container: &Element,
    key: &str,
    collection_id: Option<&str>,
) -> Option<Element> {
    let selector = match collection_id {
        Some(cid) => format!(
            "[data-collection=\"{}\"][data-key=\"{}\"]",
            css_escape_attr_value(cid),
            css_escape_attr_value(key)
        ),
        None => format!("[data-key=\"{}\"]", css_escape_attr_value(key)),
    };
    container.query_selector(&selector).ok().flatten()
}

/// Escapes special characters in a CSS attribute value selector.
///
/// This handles characters that would break `[data-key="..."]` selectors.
fn css_escape_attr_value(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for ch in value.chars() {
        match ch {
            '"' | '\\' => {
                escaped.push('\\');
                escaped.push(ch);
            }
            _ => escaped.push(ch),
        }
    }
    escaped
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_css_escape_attr_value() {
        assert_eq!(css_escape_attr_value("simple"), "simple");
        assert_eq!(css_escape_attr_value(r#"has"quote"#), r#"has\"quote"#);
        assert_eq!(css_escape_attr_value(r"has\backslash"), r"has\\backslash");
        assert_eq!(css_escape_attr_value("item-1"), "item-1");
    }
}
