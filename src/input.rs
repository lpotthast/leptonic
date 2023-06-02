use std::borrow::Cow;

use leptos::*;
use web_sys::HtmlInputElement;

use crate::{Margin, OptionalMaybeSignal};

// TODO: Consider merging this component with DateTimeInput if that does not impose a performance regression when using standard text inputs.
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
    #[prop(into)] get: MaybeSignal<String>,
    set: S,
    #[prop(optional, into)] prepend: OptionalMaybeSignal<View>,
    #[prop(into, optional)] id: Option<Cow<'static, str>>,
    #[prop(into, optional)] class: Option<Cow<'static, str>>,
    #[prop(into, optional)] disabled: OptionalMaybeSignal<bool>,
    #[prop(optional)] margin: Option<Margin>,
) -> impl IntoView
where
    S: Fn(String) + Clone + 'static,
{
    let id = id.map(|it| it.to_owned().to_string());
    let class = class
        .map(|it| Cow::Owned(format!("leptonic-input {it}")))
        .unwrap_or(Cow::Borrowed("leptonic-input"));
    let style = margin.map(|it| format!("--margin: {it}"));

    let ty_str = match ty {
        InputType::Text => "text",
        InputType::Password => "password",
        InputType::Number => "number",
    };

    let set_clone = set.clone();

    view! { cx,
        <leptonic-input-field style=style>
            <input
                id=id
                class=class
                placeholder=move || match &label.0 {
                    Some(label) => std::borrow::Cow::Owned(label.get()),
                    None => std::borrow::Cow::Borrowed(""),
                }
                type=ty_str
                prop:disabled=move || disabled.0.as_ref().map(|it| it.get()).unwrap_or(false)
                prop:value=move || get.get()
                on:change=move |e| set(event_target::<HtmlInputElement>(&e).value())
                on:keyup=move |e| set_clone(event_target::<HtmlInputElement>(&e).value())
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
