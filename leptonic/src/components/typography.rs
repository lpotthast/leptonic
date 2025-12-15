use leptos::prelude::*;

use crate::{
    components::prelude::{Button, ButtonVariant, Icon},
    Out,
};

#[derive(Clone)]
#[slot]
pub struct Li {
    children: ChildrenFn,
}

#[component]
pub fn Ul(#[prop(default=vec![])] li: Vec<Li>) -> impl IntoView {
    view! {
        <ul>
            <For
                each=move || { li.clone().into_iter().enumerate() }
                key=move |(index, _e)| *index
                children=move |(_index, e)| view!{
                    <li>
                        { (e.children)() }
                    </li>
                }
            />
        </ul>
    }
}

// TODO (new): This once received children (in leptos 0.6) and extracted text from them. This does not seem to be possible anymore?
#[component]
pub fn Code(
    #[prop(optional)] inline: Option<bool>,
    #[prop(optional)] show_copy_button: Option<bool>,
    #[prop(into, optional)] on_copy: Option<Out<Result<(), ()>>>,
    //#[prop(into)] code: String,
    children: TypedChildren<impl Into<Oco<'static, str>>>,
) -> impl IntoView {
    let code = children.into_inner()().into_inner().into();

    let code_text = StoredValue::new(code);

    let show_copy_button = show_copy_button.unwrap_or_else(|| !inline.unwrap_or(false));
    let on_success = Callback::new(move |()| {
        if let Some(on_copy) = on_copy {
            on_copy.set(Ok(()));
        }
    });
    let on_err = Callback::new(move |()| {
        if let Some(on_copy) = on_copy {
            on_copy.set(Err(()));
        } else {
            tracing::warn!("copy to clipboard failed");
        }
    });

    let copy_btn = show_copy_button.then(|| {
        view!(
            <Button
                attr:class="leptonic-code-copy-button"
                variant=ButtonVariant::Flat
                on_press=move |_| {
                    let text = code_text.get_value();
                    copy_to_clipboard(
                        text.as_str(),
                        on_success,
                        on_err
                    );
                }>
                <Icon icon=icondata::VsCopy/>
            </Button>
        )
    });

    view! {
        <leptonic-code inline=inline.map(|it| it.to_string())>
            <leptonic-code-text inline=inline.map(|it| it.to_string()) >
                { code_text.get_value() }
            </leptonic-code-text>
            { copy_btn }
        </leptonic-code>
    }
}

#[cfg(feature = "clipboard")]
fn copy_to_clipboard(text: &str, on_success: Callback<(), ()>, on_err: Callback<(), ()>) {
    match leptos_use::use_window().navigator() {
        Some(navigator) => {
            let promise = navigator.clipboard().write_text(text);
            let future = wasm_bindgen_futures::JsFuture::from(promise);
            wasm_bindgen_futures::spawn_local(async move {
                match future.await {
                    Ok(_result) => {
                        on_success.run(());
                    }
                    Err(_err) => {
                        on_err.run(());
                    }
                }
            });
        }
        None => {
            on_err.run(());
        }
    }
}

#[cfg(not(feature = "clipboard"))]
fn copy_to_clipboard(_text: &str, _on_success: Callback<()>, _on_err: Callback<()>) {
    tracing::warn!("Clipboard related functionality requires leptonic's 'Clipboard' feature as well as '--cfg=web_sys_unstable_apis'.");
}
