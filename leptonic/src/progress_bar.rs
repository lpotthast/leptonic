use leptos::*;
use leptos_use::{use_element_size, UseElementSizeReturn};

#[component]
pub fn ProgressBar(
    #[prop(into, default = MaybeSignal::Static(100.0))] max: MaybeSignal<f64>,
    #[prop(into)] progress: MaybeSignal<Option<f64>>,
) -> impl IntoView {
    let el = create_node_ref();

    let UseElementSizeReturn { width, height: _ } = use_element_size(el);

    // Calculates the percentage done in range [0, 1].
    let percentage_done = Signal::derive(move || {
        let max = max.get();
        progress
            .get()
            .map(|it| f64::max(it, 0.0))
            .map(|pos_progress| {
                let percentage = match max == 0.0 {
                    true => 0.0,
                    false => pos_progress / max,
                };
                f64::max(f64::min(percentage, 1.0), 0.0)
            })
    });

    let fill_width_px = Signal::derive(move || {
        let width = width.get();
        let percentage_done = percentage_done.get();

        percentage_done.map(|percentage_done| percentage_done * width)
    });

    let fill_style = Signal::derive(move || match fill_width_px.get() {
        Some(px) => format!("width: {}px", px),
        None => "width: 20%".to_owned(),
    });

    view! {
        <leptonic-progress-bar node_ref=el data-indeterminate=move || progress.get().is_none()>
            <leptonic-progress-bar-background>
                <leptonic-progress-bar-fill style=move || fill_style.get()>
                    <leptonic-progress-bar-fill-overlay />
                </leptonic-progress-bar-fill>

                <Show when=move || percentage_done.get().is_some() fallback=|| ()>
                    <leptonic-progress-info>
                        { move || match percentage_done.get() {
                            Some(percentage_done) => format!("{:.2} %", (percentage_done * 100.0)),
                            None => "".to_owned()
                        } }
                    </leptonic-progress-info>
                </Show>
            </leptonic-progress-bar-background>
        </leptonic-progress-bar>
    }
}
