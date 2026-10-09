use leptonic::{
    IntoAttrs,
    hooks::interactions::{ScrollEvent, UseScrollWheelInput, use_scroll_wheel},
};
use leptos::prelude::*;

/// `use_scroll_wheel`: a target logging every scroll as `delta_x,delta_y` (pixels) to
/// `#test-scroll-wheel-log`, inside a wrapper logging the wheel events reaching it.
#[component]
pub fn PageHookScrollWheel() -> impl IntoView {
    let log = RwSignal::new(Vec::<String>::new());
    let scroll_wheel = use_scroll_wheel(UseScrollWheelInput {
        on_scroll: Some(Callback::new(move |e: ScrollEvent| {
            log.update(|l| l.push(format!("{},{}", e.delta_x, e.delta_y)));
        })),
        ..UseScrollWheelInput::default()
    });
    view! {
        <div id="test-page-hook-scroll-wheel">
            <h1>"use_scroll_wheel"</h1>
            <div on:wheel=move |_| log.update(|l| l.push("wrapper".to_owned()))>
                <div id="test-scroll-wheel-target" tabindex="0" {..scroll_wheel.props.into_attrs()}>
                    "Scroll here"
                </div>
            </div>
            <div>"Log: " <span id="test-scroll-wheel-log">{move || log.get().join(" ")}</span></div>
        </div>
    }
}
