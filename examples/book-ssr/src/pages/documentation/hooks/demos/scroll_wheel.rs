use leptonic::{components::prelude::*, hooks::*};
use leptos::prelude::*;

#[component]
pub fn ScrollWheelDemo() -> impl IntoView {
    let (value, set_value) = signal(50.0f64);
    let (disabled, set_disabled) = signal(false);
    let (last_delta, set_last_delta) = signal((0.0f64, 0.0f64));

    let scroll_wheel = use_scroll_wheel(UseScrollWheelInput {
        disabled: disabled.into(),
        on_scroll: Some(Callback::new(move |e: ScrollEvent| {
            set_last_delta.set((e.delta_x, e.delta_y));
            if e.delta_y.abs() > e.delta_x.abs() {
                set_value.update(|v| {
                    *v = (*v - e.delta_y * 0.1).clamp(0.0, 100.0);
                });
            }
        })),
    });

    view! {
        <FormControl classes="demo-form-row">
            <Checkbox checked=disabled set_checked=set_disabled />
            <Label>"Disable scroll wheel handling"</Label>
        </FormControl>

        <div
            {..scroll_wheel.props.into_attrs()}
            tabindex="0"
            style="text-align: center; cursor: ns-resize; padding: 1em 0;"
        >
            <div style="font-size: 3em; font-weight: bold;">
                { move || format!("{:.0}", value.get().round()) }
            </div>
            <p style="margin: 1em 0 0 0; font-size: 0.9em; opacity: 0.8;">
                "Use mouse wheel or trackpad"
            </p>
        </div>

        <p>"Last scroll delta: "
            <code>{ move || format!("({:.1}, {:.1})", last_delta.get().0, last_delta.get().1) }</code>
        </p>
    }
}
