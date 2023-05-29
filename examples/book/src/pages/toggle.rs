use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn PageToggle(cx: Scope) -> impl IntoView {
    let (state, set_state) = create_signal(cx, false);

    view! { cx,
        <H2>"Toggle"</H2>

        <Toggle state=state on_toggle=move |s| set_state.set(s)/>
    }
}
