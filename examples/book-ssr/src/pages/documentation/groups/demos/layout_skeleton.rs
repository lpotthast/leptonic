use leptonic::atoms::checkbox::Checkbox;
use leptos::prelude::*;

#[component]
pub fn LayoutSkeletonDemo() -> impl IntoView {
    let is_loading = RwSignal::new(true);

    view! {
        // `aria-busy` tells screen readers that this part of the page is still loading.
        <div class="demo-skeleton-profile" aria-busy=move || is_loading.get().to_string()>
            <Show
                when=move || is_loading.get()
                fallback=|| view! {
                    <p class="demo-skeleton-name">"Ada Lovelace"</p>
                    <p>"Wrote the first published program, for the Analytical Engine of Charles Babbage."</p>
                }
            >
                // Shaped like the content they stand in for: a name and a short text. Without a role, screen
                // readers pass over them.
                <div class="demo-skeleton demo-skeleton-line"></div>
                <div class="demo-skeleton demo-skeleton-block"></div>
            </Show>
        </div>
        <div class="demo-controls">
            <Checkbox is_selected=is_loading set_selected=is_loading classes="demo-check">
                <span class="demo-check-box" aria-hidden="true"></span>
                "Loading"
            </Checkbox>
        </div>
    }
}
