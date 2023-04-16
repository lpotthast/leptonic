use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn PageOverview(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal_ls(cx, "count", 0u64);

    let increase_counter_by_one = move |_| set_count.update(|count| *count += 1);

    view! { cx,
        <Button on_click=increase_counter_by_one>"Click Me: " {count}</Button>

        <h2>"Sliders"</h2>

        <h2>"Separators"</h2>
        <Separator />
    }
}
