use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn PageInput(cx: Scope) -> impl IntoView {
    view! { cx,
        <Typography variant=TypographyVariant::H2>"Inputs"</Typography>

        <Input />
        <Input ty=InputType::Text/>
        <Input ty=InputType::Password/>
        <Input ty=InputType::Number/>
    }
}
