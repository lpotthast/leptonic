use leptos::prelude::*;

/// The value of a boolean `data-*` attribute: `"true"` while `signal` is `true`, absent otherwise
/// (as react-aria-components renders its state attributes). Style with `[data-selected]`.
///
/// ```ignore
/// view! { <div data-selected=flag(is_selected)></div> }
/// ```
pub fn flag(signal: Signal<bool>) -> impl Fn() -> Option<&'static str> + Clone + Send + Sync {
    move || signal.get().then_some("true")
}

pub trait ToStaticStrRepr {
    fn to_static_str_repr(&self) -> &'static str;
}

impl ToStaticStrRepr for bool {
    fn to_static_str_repr(&self) -> &'static str {
        match self {
            true => "true",
            false => "false",
        }
    }
}
