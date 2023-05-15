use leptos::*;
use web_sys::HtmlInputElement;

use crate::{Margin, OptionalMaybeSignal};

pub enum InputType {
    Text,
    Password,
    Number,
}

#[component]
pub fn Input<S>(
    cx: Scope,
    #[prop(optional, default=InputType::Text)] ty: InputType,
    #[prop(optional, into)] label: OptionalMaybeSignal<String>,
    #[prop(into)] get: Signal<String>,
    set: S,
    #[prop(optional, into)] prepend: OptionalMaybeSignal<View>,
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
                placeholder=move || match &label.0 {
                    Some(label) => std::borrow::Cow::Owned(label.get()),
                    None => std::borrow::Cow::Borrowed(""),
                }
                type=ty_str
                prop:value=move || get.get()
                on:change=move |e| set(event_target::<HtmlInputElement>(&e).value())
            />
            {match prepend.0 {
                Some(view) => view! { cx,
                    <div>
                        { view.get() }
                    </div>
                }.into_view(cx),
                None => ().into_view(cx),
            }}

            //<LeptosIcon icon=icon />
        </leptonic-input-field>
    }
}
