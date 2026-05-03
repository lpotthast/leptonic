use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn ChipDismissibleDemo() -> impl IntoView {
    let (dismissed, set_dismissed) = signal(false);

    view! {
        <Show
            when=move || !dismissed.get()
            fallback=move || view! { <Button on_press=move |_| set_dismissed.set(false)>"Reveal chip"</Button>}
        >
            <Chip color=ChipColor::Secondary dismissible=move |_| set_dismissed.set(true)>
                "Dismissible"
            </Chip>
        </Show>
    }
}
