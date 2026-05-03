use leptonic::hooks::*;
use leptos::prelude::*;
use leptos_classes::Classes;

#[component]
pub fn OverlaysDomainDemo() -> impl IntoView {
    let (is_open, set_is_open) = signal(false);

    let UsePressReturn { props, .. } = use_press(UsePressInput {
        disabled: Signal::derive(|| false),
        force_prevent_default: false,
        force_propagation: false,
        allow_text_selection_on_press: false,
        should_cancel_on_pointer_exit: false,
        prevent_focus_on_press: false,
        force_is_pressed: None,
        on_press: Callback::new(move |_: PressEvent| {
            set_is_open.update(|v| *v = !*v);
        }),
        on_press_up: None,
        on_press_start: None,
        on_press_end: None,
        on_press_change: None,
        on_double_press: None,
        on_long_press_start: None,
        on_long_press: None,
        on_long_press_end: None,
        long_press_threshold: None,
        long_press_accessibility_description: None,
    });

    let UsePressReturn {
        props: close_props, ..
    } = use_press(UsePressInput {
        disabled: Signal::derive(|| false),
        force_prevent_default: false,
        force_propagation: false,
        allow_text_selection_on_press: false,
        should_cancel_on_pointer_exit: false,
        prevent_focus_on_press: false,
        force_is_pressed: None,
        on_press: Callback::new(move |_: PressEvent| {
            set_is_open.set(false);
        }),
        on_press_up: None,
        on_press_start: None,
        on_press_end: None,
        on_press_change: None,
        on_double_press: None,
        on_long_press_start: None,
        on_long_press: None,
        on_long_press_end: None,
        long_press_threshold: None,
        long_press_accessibility_description: None,
    });
    let (close_props, close_styles) = close_props.into_parts();
    let close_attrs = StoredValue::new(close_props);
    let close_styles = StoredValue::new(close_styles);

    let (press_props, press_styles) = props.into_parts();

    view! {
        <div
            tabindex=0
            {..press_props}
            style=press_styles
            class=Classes::from("demo-btn")
        >
            { move || if is_open.get() { "Close panel" } else { "Open panel" } }
        </div>
        <Show when=move || is_open.get()>
            <div class=Classes::from("demo-popover-panel")>
                <p style="margin: 0 0 0.5em 0;">"This is overlay content."</p>
                <p style="margin: 0 0 1em 0; color: #666; font-size: 0.875em;">
                    "The full overlay hooks add dismiss-on-click-outside, "
                    "ARIA attributes, and CSS positioning."
                </p>
                <div
                    tabindex=0
                    {..close_attrs.get_value()}
                    style=close_styles.get_value()
                    class=Classes::from("demo-btn")
                >
                    "Close"
                </div>
            </div>
        </Show>
    }
}
