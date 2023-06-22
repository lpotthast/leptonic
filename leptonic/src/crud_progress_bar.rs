use leptos::*;

// TODO: Move to leptonic?
#[component]
pub fn CrudProgressBar(cx: Scope, percent: Signal<f64>, show_percentage: MaybeSignal<bool>) -> impl IntoView {
    let style = Signal::derive(cx, move ||
        format!("background: linear-gradient(90deg, rgba(57,46,242,1) 0%, rgba(136,186,254,1) {0}%, rgba(255,255,255,1) {0}%);", percent.get() * 100.0)
    );

    let optional_percentage_span = move || {
        let encoded = format!("{:.0} %", percent.get() * 100.0);
        show_percentage.get().then(|| view! {cx, <span>{ encoded }</span> });
    };

    view! {cx,
        <div class="crud-progress-bar" style=move || style.get()>
            { optional_percentage_span }
        </div>
    }
}
