// Upstream: react-aria/src/selection/utils.ts @ 99e6102368
use web_sys::KeyboardEvent;

use crate::utils::{
    modifiers::Modifiers,
    platform::device::{is_apple_device, is_mac},
};

/// Returns `true` if the "control" modifier is pressed: Cmd (Meta) on macOS, Ctrl elsewhere
/// (react-aria's `isCtrlKeyPressed`, `utils/keyboard.tsx`).
pub(super) fn is_ctrl_key_pressed(modifiers: Modifiers) -> bool {
    if is_mac() {
        modifiers.meta_key
    } else {
        modifiers.ctrl_key
    }
}

/// Returns `true` if the modifier for non-contiguous selection is pressed.
///
/// Non-contiguous selection means toggling an individual item without affecting
/// other selected items (e.g., Ctrl+Click on Windows, Alt+Click on macOS).
///
/// On Apple devices this is the Alt (Option) key.
/// On other platforms this is the Ctrl key.
pub(super) fn is_non_contiguous_selection_modifier_keyboard(e: &KeyboardEvent) -> bool {
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
pub(super) fn is_non_contiguous_selection_modifier(m: Modifiers) -> bool {
    if is_apple_device() {
        m.alt_key
    } else {
        m.ctrl_key
    }
}
