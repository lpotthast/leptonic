use leptonic::{
    IntoAttrs,
    hooks::focus::{UseFocusInput, UseFocusReturn, use_focus},
};
use leptos::prelude::*;

#[component]
pub fn FocusQuickStartDemo() -> impl IntoView {
    let (is_focused, set_is_focused) = signal(false);

    let UseFocusReturn { props } = use_focus(UseFocusInput {
        on_focus_change: Some(Callback::new(move |focused| set_is_focused.set(focused))),
        ..Default::default()
    });

    view! {
        <label class="demo-focus-label">
            "Email "
            <input type="email" class="demo-focus-item" {..props.into_attrs()}/>
        </label>

        <p class="demo-status">
            {move || if is_focused.get() { "The field has focus." } else { "The field doesn\u{2019}t have focus." }}
        </p>
    }
}
