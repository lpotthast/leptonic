use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn PageToggle(cx: Scope) -> impl IntoView {
    let (state, set_state) = create_signal(cx, false);

    view! { cx,
        <Typography variant=TypographyVariant::H2>"Toggle"</Typography>

        <Toggle state=state on_toggle=set_state/>
    }
}
