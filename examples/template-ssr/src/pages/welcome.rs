use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn Welcome() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
        <Box style="display: flex; flex-direction: column; align-items: center; padding: 1em; min-height: 100%; min-width: 100%">
            <H2>"Leptonic template SSR"</H2>

            <span style="margin-top: 3em;">"Count: " {move || count.get()}</span>
            <Button on_click=move|_| set_count.update(|c| *c += 1)>
                "Increase"
            </Button>
        </Box>
    }
}
