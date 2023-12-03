use leptos::*;
use leptos_icons::{Icon, VsIcon};
use leptos_use::use_window;

use crate::prelude::{Button, ButtonVariant};

#[component]
pub fn H1(
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<AttributeValue>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    children: Children,
) -> impl IntoView {
    view! {
        <h1 id=id class=class style=style>
            { children() }
        </h1>
    }
}

#[component]
pub fn H2(
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<AttributeValue>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    children: Children,
) -> impl IntoView {
    view! {
        <h2 id=id class=class style=style>
            { children() }
        </h2>
    }
}

#[component]
pub fn H3(
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<AttributeValue>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    children: Children,
) -> impl IntoView {
    view! {
        <h3 id=id class=class style=style>
            { children() }
        </h3>
    }
}

#[component]
pub fn H4(
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<AttributeValue>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    children: Children,
) -> impl IntoView {
    view! {
        <h4 id=id class=class style=style>
            { children() }
        </h4>
    }
}

#[component]
pub fn H5(
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<AttributeValue>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    children: Children,
) -> impl IntoView {
    view! {
        <h5 id=id class=class style=style>
            { children() }
        </h5>
    }
}

#[component]
pub fn H6(
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<AttributeValue>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    children: Children,
) -> impl IntoView {
    view! {
        <h6 id=id class=class style=style>
            { children() }
        </h6>
    }
}

#[component]
pub fn P(
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<AttributeValue>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    children: Children,
) -> impl IntoView {
    view! {
        <p id=id class=class style=style>
            { children() }
        </p>
    }
}

#[component]
pub fn Code(
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<AttributeValue>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    #[prop(optional)] inline: Option<bool>,
    #[prop(optional)] show_copy_button: Option<bool>,
    #[prop(optional)] on_copy: Option<Callback<Result<(), ()>>>,
    children: Children,
) -> impl IntoView {
    let code_text = store_value(
        itertools::Itertools::intersperse(
            children().nodes.iter().map(|e| match e {
                View::Text(t) => t.content.to_owned(),
                _ => panic!("non-text children are not allowed: {:?}", e.to_owned()),
            }),
            "\n".into(),
        )
        .collect::<String>(),
    );

    let show_copy_button = show_copy_button.unwrap_or_else(|| !inline.unwrap_or(false));
    let on_success = move || {
        if let Some(on_copy) = on_copy {
            on_copy.call(Ok(()))
        }
    };
    let on_err = move || {
        if let Some(on_copy) = on_copy {
            on_copy.call(Err(()))
        } else {
            tracing::warn!("copy to clipboard failed")
        }
    };
    let copy_btn = show_copy_button.then(|| {
        view!(
            <Button
                class="leptonic-code-copy-button"
                variant=ButtonVariant::Flat
                on_click=move |_| copy_to_clipboard(
                    code_text.with_value(|c| c.clone()),
                    on_success,
                    on_err
                )>
                <Icon icon=Icon::from(VsIcon::VsCopy)/>
            </Button>
        )
    });

    view! {
        <leptonic-code inline=inline.map(|it| it.to_string())>
            <leptonic-code-text id=id class=class style=style inline=inline.map(|it| it.to_string()) >
                { code_text.get_value() }
            </leptonic-code-text>
            { copy_btn }
        </leptonic-code>
    }
}

fn copy_to_clipboard<T: AsRef<str>, S: Fn() + 'static, E: Fn() + 'static>(
    text: T,
    on_success: S,
    on_err: E,
) {
    match use_window().navigator() {
        Some(navigator) => {
            match navigator.clipboard() {
                Some(clipboard) => {
                    let promise = clipboard.write_text(text.as_ref());
                    let future = wasm_bindgen_futures::JsFuture::from(promise);
                    wasm_bindgen_futures::spawn_local(async move {
                        match future.await {
                            Ok(_result) => {
                                on_success();
                            }
                            Err(_err) => {
                                on_err();
                            }
                        }
                    });
                }
                None => {
                    on_err();
                }
            };
        }
        None => {
            on_err();
        }
    }
}
