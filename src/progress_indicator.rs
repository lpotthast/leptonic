use leptos::*;
use leptos_use::{use_element_size, UseElementSizeReturn};

#[component]
pub fn ProgressBar(
    cx: Scope,
    #[prop(default = 100.0)] max: f64,
    #[prop(into)] progress: Signal<f64>,
) -> impl IntoView {
    let el = create_node_ref(cx);

    let UseElementSizeReturn { width, height: _ } = use_element_size(cx, el);

    // Calculates the percentage done in range [0, 1].
    let percentage_done = Signal::derive(cx, move || {
        let progress = progress.get().abs();
        let percentage = match progress == 0.0 {
            true => 0.0,
            false => progress / max,
        };
        f64::max(f64::min(percentage, 1.0), 0.0)
    });

    let fill_width_px = Signal::derive(cx, move || width.get() * percentage_done.get());

    let fill_style = Signal::derive(cx, move || format!("width: {}px", fill_width_px.get()));

    view! { cx,
        <leptonic-progress-bar node_ref=el>
            <leptonic-progress-bar-background>
                <leptonic-progress-bar-fill style=move || fill_style.get() />
                <leptonic-progress-info>
                    { move || format!("{:.2} %", (percentage_done.get() * 100.0)) }
                </leptonic-progress-info>
            </leptonic-progress-bar-background>
        </leptonic-progress-bar>
    }
}
