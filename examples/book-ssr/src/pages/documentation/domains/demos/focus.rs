use leptonic::hooks::*;
use leptos::prelude::*;
use leptos_classes::Classes;

#[component]
pub fn FocusDomainDemo() -> impl IntoView {
    let (is_focused, set_is_focused) = signal(false);

    let UseFocusReturn { props } = use_focus(UseFocusInput {
        disabled: Signal::derive(|| false),
        on_focus: None,
        on_blur: None,
        on_focus_change: Some(Callback::new(move |focused: bool| {
            set_is_focused.set(focused);
        })),
    });

    view! {
        <div
            tabindex=0
            {..props.into_attrs()}
            class=Classes::builder().with_toggle(is_focused.get(), "demo-container-active", "demo-container-inactive").build()
        >
            <strong class=Classes::builder().with_toggle(is_focused.get(), "demo-state-active", "demo-state-inactive").build()>
                { move || if is_focused.get() { "Focused" } else { "Not focused" } }
            </strong>
            " \u{2014} click here or press Tab"
        </div>
    }
}
