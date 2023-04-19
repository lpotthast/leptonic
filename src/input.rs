use leptos::*;

use crate::Margin;

pub enum InputType {
    Text,
    Password,
    Number,
}

#[component]
pub fn Input(
    cx: Scope,
    #[prop(optional, default=InputType::Text)] ty: InputType,
    #[prop(optional)] margin: Option<Margin>,
) -> impl IntoView {
    let style = margin.map(|it| format!("--margin: {it}"));

    let ty_str = match ty {
        InputType::Text => "text",
        InputType::Password => "password",
        InputType::Number => "number",
    };

    view! { cx,
        <leptonic-input-field style=style>
            <input class="leptonic-input" type=ty_str/>

            //<LeptosIcon icon=icon />
        </leptonic-input-field>
    }
}
