use leptonic::{components::prelude::*, utils::css::em};
use leptos::prelude::*;

#[component]
pub fn SkeletonDemo() -> impl IntoView {
    let (is_loading, set_loading) = signal(true);

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
                // Shaped like the content they stand in for: a name and a short text.
                <Skeleton width=em(10.0) height=em(1.5)/>
                <Skeleton height=em(3.0)/>
            </Show>
        </div>
        <div class="demo-controls">
            <Checkbox is_selected=is_loading set_selected=set_loading>"Loading"</Checkbox>
        </div>
    }
}
