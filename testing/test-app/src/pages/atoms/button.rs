use leptonic::atoms::button::Button;
use leptos::prelude::*;

#[component]
pub fn PageAtomButton() -> impl IntoView {
    let (basic_count, set_basic_count) = signal(0u32);
    let (disabled_count, set_disabled_count) = signal(0u32);

    view! {
        <div id="test-page-atom-button">
            <h1>"Button Atom Test Page"</h1>

            <section>
                <h2>"Basic Button"</h2>
                <Button
                    on_press=move |_| set_basic_count.update(|c| *c += 1)
                    attr:id="test-button-basic"
                >
                    "Press me"
                </Button>
                <div>"Press count: " <span id="test-button-basic-count">{basic_count}</span></div>
            </section>

            <section>
                <h2>"Disabled Button"</h2>
                <Button
                    on_press=move |_| set_disabled_count.update(|c| *c += 1)
                    disabled=Signal::from(true)
                    attr:id="test-button-disabled"
                >
                    "Disabled"
                </Button>
                <div>
                    "Press count: " <span id="test-button-disabled-count">{disabled_count}</span>
                </div>
            </section>
        </div>
    }
}
