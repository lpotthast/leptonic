use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn ChipDismissibleDemo() -> impl IntoView {
    let (is_filtering, set_filtering) = signal(true);

    view! {
        // The chip doesn't remove itself: stop rendering it when it is dismissed.
        <Show
            when=move || is_filtering.get()
            fallback=move || view! { <Button on_press=move |_| set_filtering.set(true)>"Show open issues only"</Button> }
        >
            <Chip color=ChipColor::Secondary on_dismiss=move |()| set_filtering.set(false) dismiss_label="Remove filter">
                "Status: open"
            </Chip>
        </Show>
        <p class="demo-status">
            {move || if is_filtering.get() { "Showing open issues." } else { "Showing all issues." }}
        </p>
    }
}
