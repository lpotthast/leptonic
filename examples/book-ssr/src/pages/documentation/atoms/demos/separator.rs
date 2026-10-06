use leptonic::{atoms::prelude as atoms, hooks::Orientation};
use leptos::prelude::*;

#[component]
pub fn SeparatorAtomDemo() -> impl IntoView {
    view! {
        <p>"Content above"</p>
        // An <hr>, styled through its class.
        <atoms::Separator classes="demo-separator-line"/>
        <div class="demo-flex-center-row">
            <span>"Bold"</span>
            <span>"Italic"</span>
            // A <div role="separator" aria-orientation="vertical">.
            <atoms::Separator orientation=Orientation::Vertical classes="demo-separator-line"/>
            <span>"Undo"</span>
        </div>
    }
}
