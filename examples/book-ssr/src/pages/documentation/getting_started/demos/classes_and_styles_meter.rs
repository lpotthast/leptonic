use leptonic::utils::{
    classes::Classes,
    css::{CssColor, NonNegativeLengthPercentage, Size, css_custom_property, pct, rgb},
    style::WidthProperty,
    styles::Styles,
};
use leptos::prelude::*;

// A typed custom property: only `CssColor` values can be assigned to it.
// `.demo-meter-fill` reads it via `var(--demo-meter-fill)`.
css_custom_property!(FILL_COLOR: CssColor = "--demo-meter-fill");

#[component]
pub fn ClassesAndStylesMeterDemo() -> impl IntoView {
    let (progress, set_progress) = signal(40_u8);
    let (warm, set_warm) = signal(false);

    // Static look lives in CSS; the `complete` class follows the progress reactively.
    let fill_classes =
        Classes::from("demo-meter-fill").add_reactive("complete", move || progress.get() == 100);

    // Only the genuinely dynamic values are inline styles, each a typed declaration.
    // `progress` is always within 0..=100, so the panicking `pct` and `new` are safe here.
    let fill_styles = Styles::builder()
        .with_reactive(move || {
            WidthProperty.declare(Size::from(NonNegativeLengthPercentage::new(pct(
                progress.get()
            ))))
        })
        .with_reactive(move || {
            FILL_COLOR.declare(if warm.get() {
                rgb(230, 105, 86)
            } else {
                rgb(74, 144, 217)
            })
        })
        .build();

    view! {
        <div class="demo-meter">
            <div class=fill_classes style=fill_styles></div>
        </div>

        <div class="demo-flex-center-row">
            <button
                class="demo-btn"
                on:click=move |_| set_progress.update(|p| *p = p.saturating_sub(10))
            >
                "-10%"
            </button>
            <button
                class="demo-btn"
                on:click=move |_| set_progress.update(|p| *p = (*p + 10).min(100))
            >
                "+10%"
            </button>
            <span>{move || format!("{}%", progress.get())}</span>
        </div>

        <label class="demo-checkbox-label">
            <input
                type="checkbox"
                prop:checked=warm
                on:change=move |e| set_warm.set(event_target_checked(&e))
            />
            "Warm accent color"
        </label>
    }
}
