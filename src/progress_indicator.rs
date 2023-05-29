use leptos::*;

#[component]
pub fn ProgressBar(
    cx: Scope,
    #[prop(default = 100)]
    max: u32,
    #[prop(into)]
    progress: Signal<u32>,
) -> impl IntoView {
    view! { cx,
        <progress
            class="leptonic-progress-bar"
            max=max
            value=move || progress.get()
        />
    }
}