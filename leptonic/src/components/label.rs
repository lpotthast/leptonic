use leptos::*;

use crate::components::form_control::FormControlContext;

#[component]
pub fn Label(children: Children) -> impl IntoView {
    let fc_ctx = use_context::<FormControlContext>();

    view! {
        <leptonic-label on:click=move |_| {
            if let Some(fc_ctx) = &fc_ctx {
                fc_ctx.input.with_untracked(move |input| match input {
                    Some(input) => { input.on_label_click(); },
                    None => {},
                });
            }
        }>
            { children() }
        </leptonic-label>
    }
}
