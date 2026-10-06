use leptonic::{components::prelude::*, hooks::*};
use leptos::prelude::*;

#[component]
pub fn ScrollWheelDemo() -> impl IntoView {
    let (value, set_value) = signal(50.0f64);
    let (disabled, set_disabled) = signal(false);
    let (last_delta, set_last_delta) = signal((0.0f64, 0.0f64));

    let scroll_wheel = use_scroll_wheel(UseScrollWheelInput {
        is_disabled: disabled.into(),
        on_scroll: Some(Callback::new(move |e: ScrollEvent| {
            set_last_delta.set((e.delta_x, e.delta_y));
            // Only react to mostly vertical scrolling.
            if e.delta_y.abs() > e.delta_x.abs() {
                set_value.update(|v| *v = (*v - e.delta_y * 0.1).clamp(0.0, 100.0));
            }
        })),
    });

    view! {
        <Checkbox state=(disabled, set_disabled) classes="demo-form-row">"Disable scroll wheel handling"</Checkbox>

        <div {..scroll_wheel.props.into_attrs()} tabindex="0" class="demo-scroll-wheel-target">
            <div class="demo-scroll-wheel-value">{move || format!("{:.0}", value.get())}</div>
            <p class="demo-muted-text">"Scroll here with a mouse wheel or trackpad"</p>
        </div>

        <p>
            "Last scroll delta: "
            <code>{move || format!("({:.1}, {:.1})", last_delta.get().0, last_delta.get().1)}</code>
        </p>
    }
}
