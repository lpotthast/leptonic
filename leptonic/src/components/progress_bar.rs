use leptos::{html, prelude::*};
use leptos_use::{UseElementSizeReturn, use_element_size};

use crate::utils::{
    classes::Classes,
    css::{pct, px},
    styles::{Styles, Width},
};

#[component]
pub fn ProgressBar(
    #[prop(into, default = Signal::from(100.0))] max: Signal<f64>,
    #[prop(into)] progress: Signal<Option<f64>>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
) -> impl IntoView {
    let el: NodeRef<html::Div> = NodeRef::new();

    let UseElementSizeReturn { width, height: _ } = use_element_size(el);

    // Calculates the percentage done in range [0, 1].
    let percentage_done = Signal::derive(move || {
        let max = max.get();
        progress
            .get()
            .map(|it| f64::max(it, 0.0))
            .map(|pos_progress| {
                let percentage = if max == 0.0 { 0.0 } else { pos_progress / max };
                percentage.clamp(0.0, 1.0)
            })
    });

    let fill_width_px = Signal::derive(move || {
        let width = width.get();
        let percentage_done = percentage_done.get();

        percentage_done.map(|percentage_done| percentage_done * width)
    });

    let fill_styles = Styles::new().add(Width, move || match fill_width_px.get() {
        Some(px_val) => px(px_val),
        None => pct(20.0),
    });

    view! {
        <div
            class=classes.add("leptonic-progress-bar")
            style=styles
            node_ref=el
            data-indeterminate=move || progress.get().is_none()
        >
            <div class="leptonic-progress-bar-background">
                <div class="leptonic-progress-bar-fill" style=fill_styles>
                    <div class="leptonic-progress-bar-fill-overlay" />
                </div>

                <Show when=move || percentage_done.get().is_some() fallback=|| ()>
                    <div class="leptonic-progress-info">
                        {move || match percentage_done.get() {
                            Some(percentage_done) => format!("{:.2} %", (percentage_done * 100.0)),
                            None => String::new(),
                        }}
                    </div>
                </Show>
            </div>
        </div>
    }
}
