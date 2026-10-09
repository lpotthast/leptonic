use leptonic::atoms::link::AnchorLink;
use leptos::prelude::*;

#[component]
pub fn AnchorLinkAtomDemo() -> impl IntoView {
    let last_pressed = RwSignal::new("none");

    view! {
        <p>
            <AnchorLink
                href="#anchor-link-atom-demo-returns"
                on_press=move |_| last_pressed.set("the text link")
                classes="demo-link-atom"
            >
                "Jump to the returns policy"
            </AnchorLink>
        </p>
        <p>"Orders ship within two working days, in recyclable packaging."</p>

        <h3 id="anchor-link-atom-demo-returns" class="demo-anchor-heading">
            "Returns"
            // A bare `#` doesn't say where it leads: name the link.
            <AnchorLink
                href="#anchor-link-atom-demo-returns"
                aria_label="Link to this section: Returns"
                on_press=move |_| last_pressed.set("the heading\u{2019}s #")
                classes="demo-link-atom"
            >
                "#"
            </AnchorLink>
        </h3>
        <p>"Return anything within 30 days."</p>

        <p class="demo-status">"Last link pressed: " {move || last_pressed.get()}</p>
    }
}
