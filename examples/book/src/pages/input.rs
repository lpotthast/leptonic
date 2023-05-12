use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn PageInput(cx: Scope) -> impl IntoView {
    let (text, set_text) = create_signal(cx, "text".to_owned());
    let (password, set_password) = create_signal(cx, "password".to_owned());
    let (number, set_number) = create_signal(cx, "number".to_owned());

    view! { cx,
        <Typography variant=TypographyVariant::H2>"Inputs"</Typography>

        <Input get=text set=set_text/>
        <Input get=text set=set_text ty=InputType::Text/>
        <Input get=password set=set_password ty=InputType::Password/>
        <Input get=number set=set_number ty=InputType::Number/>
    }
}
