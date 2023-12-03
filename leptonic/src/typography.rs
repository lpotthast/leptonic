use leptos::*;
use leptos_icons::{Icon, VsIcon};

use crate::prelude::{Button, ButtonVariant};

#[derive(Debug, Clone, Copy)]
pub enum TypographyVariant {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    Paragraph,
    Code { inline: bool },
}

#[component]
pub fn Typography(
    variant: TypographyVariant,
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<AttributeValue>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    children: ChildrenFn,
) -> impl IntoView {
    match variant {
        TypographyVariant::H1 => H1(H1Props {
            id,
            class,
            style,
            children,
        })
        .into_view(),
        TypographyVariant::H2 => H2(H2Props {
            id,
            class,
            style,
            children,
        })
        .into_view(),
        TypographyVariant::H3 => H3(H3Props {
            id,
            class,
            style,
            children,
        })
        .into_view(),
        TypographyVariant::H4 => H4(H4Props {
            id,
            class,
            style,
            children,
        })
        .into_view(),
        TypographyVariant::H5 => H5(H5Props {
            id,
            class,
            style,
            children,
        })
        .into_view(),
        TypographyVariant::H6 => H6(H6Props {
            id,
            class,
            style,
            children,
        })
        .into_view(),
        TypographyVariant::Paragraph => P(PProps {
            id,
            class,
            style,
            children,
        })
        .into_view(),
        TypographyVariant::Code { inline } => Code(CodeProps {
            id,
            class,
            style,
            inline: Some(inline),
            children,
        })
        .into_view(),
    }
}

#[component]
pub fn H1(
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<AttributeValue>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    children: ChildrenFn,
) -> impl IntoView {
    view! {
        <h1 id=id class=class style=style>
            {children()}
        </h1>
    }
}

#[component]
pub fn H2(
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<AttributeValue>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    children: ChildrenFn,
) -> impl IntoView {
    view! {
        <h2 id=id class=class style=style>
            {children()}
        </h2>
    }
}

#[component]
pub fn H3(
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<AttributeValue>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    children: ChildrenFn,
) -> impl IntoView {
    view! {
        <h3 id=id class=class style=style>
            {children()}
        </h3>
    }
}

#[component]
pub fn H4(
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<AttributeValue>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    children: ChildrenFn,
) -> impl IntoView {
    view! {
        <h4 id=id class=class style=style>
            {children()}
        </h4>
    }
}

#[component]
pub fn H5(
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<AttributeValue>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    children: ChildrenFn,
) -> impl IntoView {
    view! {
        <h5 id=id class=class style=style>
            {children()}
        </h5>
    }
}

#[component]
pub fn H6(
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<AttributeValue>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    children: ChildrenFn,
) -> impl IntoView {
    view! {
        <h6 id=id class=class style=style>
            {children()}
        </h6>
    }
}

#[component]
pub fn P(
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<AttributeValue>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    children: ChildrenFn,
) -> impl IntoView {
    view! {
        <p id=id class=class style=style>
            {children()}
        </p>
    }
}

#[component]
pub fn Code(
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<AttributeValue>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    #[prop(optional)] inline: Option<bool>,
    children: ChildrenFn,
) -> impl IntoView {
    let code_text = store_value(itertools::Itertools::intersperse(
        children().nodes.iter().map(|e| match e {
            View::Text(t) => t.content.to_owned(),
            _ => panic!("non-text children are not allowed: {:?}", e.to_owned()),
        }),
        "\n".into(),
    )
    .collect::<String>());

    code_text.with_value(|code_text| {
        tracing::info!("code_text: {:?}", code_text.len());
        tracing::info!("code_text: {:?}", code_text);
    });

    let show_copy_button = !inline.unwrap_or(false);
    view! {
        <leptonic-code inline=inline.map(|it| it.to_string())>
            <leptonic-code-text id=id class=class style=style inline=inline.map(|it| it.to_string()) >
                {children()}
            </leptonic-code-text>
            {show_copy_button.then_some(
                view!(
                    <Button
                        class="leptonic-code-copy-button"
                        variant=ButtonVariant::Flat
                        on_click=move |_| {
                            use wasm_bindgen_futures::spawn_local;
                            let _task = spawn_local(async move {
                                let window = web_sys::window().expect("window"); // { obj: val };
                                let nav = window.navigator().clipboard();
                                match nav {
                                    Some(a) => {
                                        let p = a.write_text(&code_text.with_value(|c| c.clone()));
                                        let _result = wasm_bindgen_futures::JsFuture::from(p)
                                            .await
                                            .expect("clipboard populated");
                                        // info!("clippyboy worked");
                                    }
                                    None => {
                                        // warn!("failed to copy clippyboy");
                                    }
                                };
                            });
                        }><Icon icon=Icon::from(VsIcon::VsCopy)/>
                    </Button>
                )
            )}
        </leptonic-code>
    }
}
