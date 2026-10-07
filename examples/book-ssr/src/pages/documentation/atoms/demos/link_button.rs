use leptonic::atoms::prelude::{Checkbox, Link};
use leptos::prelude::*;

#[component]
pub fn LinkStyledAsButtonDemo() -> impl IntoView {
    let disabled = RwSignal::new(false);

    view! {
        // It navigates, so it is a link, even though the classes make it look like a button.
        <Link href="/doc/installation" is_disabled=disabled classes=["demo-btn", "demo-link-button"]>
            "Install leptonic"
        </Link>

        <div class="demo-controls">
            <Checkbox is_selected=disabled set_selected=disabled classes="demo-check">
                <span class="demo-check-box" aria-hidden="true"></span>
                "Disabled"
            </Checkbox>
        </div>
    }
}
