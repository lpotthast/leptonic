use leptos::prelude::*;

#[cfg(feature = "atoms")]
use crate::hooks::collections::SelectionMode;

/// The value of a boolean `data-*` attribute: `"true"` while `signal` is `true`, absent otherwise
/// (as react-aria-components renders its state attributes). Style with `[data-selected]`.
///
/// ```ignore
/// view! { <div data-selected=flag(is_selected)></div> }
/// ```
pub fn flag(signal: Signal<bool>) -> impl Fn() -> Option<&'static str> + Clone + Send + Sync {
    move || signal.get().then_some("true")
}

/// The value of a `data-selection-mode` attribute: `single` or `multiple`, absent without
/// selection (as react-aria-components renders it on collection items).
#[cfg(feature = "atoms")]
pub fn selection_mode(
    mode: impl Fn() -> SelectionMode + Clone + Send + Sync + 'static,
) -> impl Fn() -> Option<&'static str> + Clone + Send + Sync {
    move || match mode() {
        SelectionMode::None => None,
        SelectionMode::Single => Some("single"),
        SelectionMode::Multiple => Some("multiple"),
    }
}
