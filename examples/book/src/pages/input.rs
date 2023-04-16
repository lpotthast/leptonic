use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn PageInput(cx: Scope) -> impl IntoView {
    view! { cx,
        <h2>"Inputs"</h2>

        <Input />
        <Input ty=InputType::Text/>
        <Input ty=InputType::Password/>
        <Input ty=InputType::Number/>
    }
}
