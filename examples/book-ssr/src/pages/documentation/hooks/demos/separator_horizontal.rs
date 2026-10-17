use leptonic::hooks::separator::{UseSeparatorInput, use_separator};
use leptos::prelude::*;

#[component]
pub fn SeparatorHorizontalDemo() -> impl IntoView {
    // The default: a horizontal separator on an <hr>.
    let separator = use_separator(UseSeparatorInput::default());

    view! {
        <p>"Content above the separator"</p>
        <hr {..separator.separator.attrs()} class="demo-separator-line"/>
        <p>"Content below the separator"</p>
    }
}
