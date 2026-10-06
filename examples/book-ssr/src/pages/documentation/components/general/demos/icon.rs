use leptonic::{components::prelude::*, prelude::*};
use leptos::prelude::*;

#[component]
pub fn IconDemo() -> impl IntoView {
    view! {
        <div class="demo-control-row">
            // Decorative: the text next to it says the same, so screen readers skip the icon.
            <span class="demo-icon-with-text">
                <Icon icon=icondata::BsFolderFill classes="demo-icon-large"/>
                "Projects"
            </span>
            // Meaningful on its own: `aria_label` makes it an image with that name.
            <Icon icon=icondata::BsCloudCheck aria_label="Synced" classes="demo-icon-large"/>
        </div>
    }
}
