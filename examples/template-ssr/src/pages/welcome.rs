use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn Welcome() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {

        <div style="display: flex; flex-direction: column; align-items: center; padding: 1em; max-width: 10em; min-width: 100%">
            <H2>"Leptonic template SSR"</H2>

            <span style="margin-top: 3em;">"Count: " {move || count.get()}</span>
            <Button on_click=move|_| set_count.update(|c| *c += 1)>
                "Increase"
            </Button>
        </div>
    }
}
