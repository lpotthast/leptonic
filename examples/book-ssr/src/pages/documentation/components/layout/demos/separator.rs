use leptonic::{components::prelude::*, hooks::Orientation};
use leptos::prelude::*;

#[component]
pub fn SeparatorDemo() -> impl IntoView {
    view! {
        <p>"Content above the separator."</p>
        <Separator/>
        <p>"Content below the separator."</p>
        // A vertical separator stretches to the height of its flex row.
        <div class="demo-flex-center-row">
            <span>"Left"</span>
            <Separator orientation=Orientation::Vertical/>
            <span>"Right"</span>
        </div>
    }
}
