// Upstream: react-aria/src/selection/utils.ts @ 99e6102368
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

/// Returns `true` if the modifier for non-contiguous selection is pressed: toggling an item
/// without affecting the other selected items (Alt (Option) on Apple devices, Ctrl elsewhere;
/// react-aria's `isNonContiguousSelectionModifier`).
pub(super) fn is_non_contiguous_selection_modifier(m: Modifiers) -> bool {
    if is_apple_device() {
        m.alt_key
    } else {
        m.ctrl_key
    }
}
