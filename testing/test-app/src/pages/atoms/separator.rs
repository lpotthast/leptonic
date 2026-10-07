use leptonic::{atoms::prelude::Separator, utils::orientation::Orientation};
use leptos::prelude::*;

/// Separator atoms (react-aria-components' `Separator.test.js` setups): a plain one
/// (`.test-sep-plain`), one named by `aria_label` (`.test-sep-labelled`), one with an id named
/// by a heading (`#test-sep-by`), and one whose orientation `#test-sep-toggle` switches
/// (`.test-sep-switching`, starting vertical).
#[component]
pub fn PageAtomSeparator() -> impl IntoView {
    let orientation = RwSignal::new(Orientation::Vertical);
    view! {
        <div id="test-page-atom-separator">
            <Separator classes="test-sep-plain" />
            <Separator classes="test-sep-labelled" aria_label="label" />
            <h2 id="test-sep-heading">"Section"</h2>
            <Separator id="test-sep-by" aria_labelledby="test-sep-heading" />
            <Separator classes="test-sep-switching" orientation=orientation />
            <button
                id="test-sep-toggle"
                on:click=move |_| {
                    orientation
                        .update(|o| {
                            *o = match o {
                                Orientation::Horizontal => Orientation::Vertical,
                                Orientation::Vertical => Orientation::Horizontal,
                            };
                        });
                }
            >
                "Toggle orientation"
            </button>
        </div>
    }
}
