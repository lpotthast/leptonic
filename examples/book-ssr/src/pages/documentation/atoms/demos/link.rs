use leptonic::atoms::prelude::{CheckboxButton, CheckboxField, CurrentMatch, Link};
use leptos::prelude::*;

/// The pages of the Link concept. The link to the page you are on has `aria-current="page"`.
#[component]
pub fn LinkAtomDemo() -> impl IntoView {
    let disabled = RwSignal::new(false);

    view! {
        <nav aria-label="Link pages">
            <ul class="demo-link-nav">
                <li>
                    // `Exact`: the overview is current only on its own route, not on the pages below it.
                    <Link href="/doc/link" current_match=CurrentMatch::Exact is_disabled=disabled classes="demo-link-atom">
                        "Overview"
                    </Link>
                </li>
                <li><Link href="/doc/link/hook" is_disabled=disabled classes="demo-link-atom">"Hooks"</Link></li>
                <li><Link href="/doc/link/atom" is_disabled=disabled classes="demo-link-atom">"Atoms"</Link></li>
            </ul>
        </nav>

        <div class="demo-controls">
            <CheckboxField is_selected=disabled set_selected=disabled>
                <CheckboxButton classes="demo-check">
                    <span class="demo-check-box" aria-hidden="true"></span>
                    "Disabled"
                </CheckboxButton>
            </CheckboxField>
        </div>
    }
}
