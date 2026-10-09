use leptonic::{Orientation, atoms};
use leptos::prelude::*;

#[component]
pub fn SeparatorAtomDemo() -> impl IntoView {
    view! {
        <p>"Content above"</p>
        // An <hr>, styled through its class.
        <atoms::separator::Separator classes="demo-separator-line"/>
        <div class="demo-flex-center-row">
            <span>"Bold"</span>
            <span>"Italic"</span>
            // A <div role="separator" aria-orientation="vertical">.
            <atoms::separator::Separator orientation=Orientation::Vertical classes="demo-separator-line"/>
            <span>"Undo"</span>
        </div>
    }
}
