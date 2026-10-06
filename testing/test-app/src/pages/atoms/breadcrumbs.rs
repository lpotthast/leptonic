use leptonic::{
    atoms::prelude::{Breadcrumb, Breadcrumbs, Link},
    hooks::collections::Key,
};
use leptos::prelude::*;

/// Breadcrumbs atoms (react-aria-components' `Breadcrumbs.test.js` setups):
/// - A trail of `#test-bc-count` items ("Item 1", ... linking to other routes;
///   `#test-bc-add`/`#test-bc-remove` change it), the last one marked current;
///   `#test-bc-disable` disables all.
/// - An "Actions" trail of in-page links ("Action 1", "Action 2"), reporting the pressed item's
///   id in `#test-bc-action`.
#[component]
pub fn PageAtomBreadcrumbs() -> impl IntoView {
    let count = RwSignal::new(3usize);
    let action = RwSignal::new(String::new());
    let disabled = RwSignal::new(false);

    view! {
        <div id="test-page-atom-breadcrumbs">
            <nav>
                <Breadcrumbs is_disabled=disabled>
                    <For each=move || 1..=count.get() key=|n| *n let:n>
                        <Breadcrumb is_current=move || n == count.get()>
                            <Link href=format!("/atoms/toolbar?item={n}")>{format!("Item {n}")}</Link>
                        </Breadcrumb>
                    </For>
                </Breadcrumbs>
            </nav>
            <button id="test-bc-add" on:click=move |_| count.update(|c| *c += 1)>"Add"</button>
            <button id="test-bc-remove" on:click=move |_| count.update(|c| *c -= 1)>"Remove"</button>
            <button id="test-bc-disable" on:click=move |_| disabled.update(|d| *d = !*d)>"Disable"</button>
            <div>"Count: " <span id="test-bc-count">{count}</span></div>

            <nav>
                <Breadcrumbs aria_label="Actions" on_action=move |key: Key| action.set(key.to_string())>
                    <Breadcrumb id="action-1">
                        <Link href="#action-1">"Action 1"</Link>
                    </Breadcrumb>
                    <Breadcrumb id="action-2" is_current=true>
                        <Link href="#action-2">"Action 2"</Link>
                    </Breadcrumb>
                </Breadcrumbs>
            </nav>
            <div>"Action: " <span id="test-bc-action">{action}</span></div>
        </div>
    }
}
