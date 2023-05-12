use leptos::*;
use web_sys::HtmlInputElement;

use crate::Margin;

pub enum InputType {
    Text,
    Password,
    Number,
}

#[component]
pub fn Input<S>(
    cx: Scope,
    #[prop(optional, default=InputType::Text)] ty: InputType,
    #[prop(optional, into)] label: String,
    #[prop(into)] get: Signal<String>,
    set: S,
    #[prop(optional)] margin: Option<Margin>,
) -> impl IntoView
where
    S: Fn(String) + 'static,
{
    let style = margin.map(|it| format!("--margin: {it}"));

    let ty_str = match ty {
        InputType::Text => "text",
        InputType::Password => "password",
        InputType::Number => "number",
    };

    view! { cx,
        <leptonic-input-field style=style>
            <input
                class="leptonic-input"
                placeholder=label
                type=ty_str
                value=move || get.get()
                on:change=move |e| set(event_target::<HtmlInputElement>(&e).value())
            />

            //<LeptosIcon icon=icon />
        </leptonic-input-field>
    }
}
