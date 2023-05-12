use leptos::*;

use crate::Margin;

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

// TODO: Control text properties like wrapping through props...

#[component]
pub fn Typography(
    cx: Scope,
    variant: TypographyVariant,
    /// Sets the `id` attribute, making it easier to target.
    #[prop(into, optional)]
    id: Option<String>,
    #[prop(optional)] margin: Option<Margin>,
    children: Children,
) -> impl IntoView {
    let style = margin.map(|it| format!("--margin: {it}"));

    match variant {
        TypographyVariant::H1 => {
            view! {cx, <h1 id=id style=style>{children(cx)}</h1>}.into_view(cx)
        }
        TypographyVariant::H2 => {
            view! {cx, <h2 id=id style=style>{children(cx)}</h2>}.into_view(cx)
        }
        TypographyVariant::H3 => {
            view! {cx, <h3 id=id style=style>{children(cx)}</h3>}.into_view(cx)
        }
        TypographyVariant::H4 => {
            view! {cx, <h4 id=id style=style>{children(cx)}</h4>}.into_view(cx)
        }
        TypographyVariant::H5 => {
            view! {cx, <h5 id=id style=style>{children(cx)}</h5>}.into_view(cx)
        }
        TypographyVariant::H6 => {
            view! {cx, <h6 id=id style=style>{children(cx)}</h6>}.into_view(cx)
        }
        TypographyVariant::Paragraph => P(
            cx,
            PProps {
                id,
                margin,
                children,
            },
        )
        .into_view(cx),
        TypographyVariant::Code { inline } => Code(
            cx,
            CodeProps {
                id,
                margin,
                inline: Some(inline),
                children,
            },
        )
        .into_view(cx),
    }
}

#[component]
pub fn P(
    cx: Scope,
    /// Sets the `id` attribute, making it easier to target.
    #[prop(into, optional)]
    id: Option<String>,
    #[prop(optional)] margin: Option<Margin>,
    children: Children,
) -> impl IntoView {
    let style = margin.map(|it| format!("--margin: {it}"));
    view! { cx,
        <p id=id style=style>
            {children(cx)}
        </p>
    }
}

#[component]
pub fn Code(
    cx: Scope,
    /// Sets the `id` attribute, making it easier to target.
    #[prop(into, optional)]
    id: Option<String>,
    #[prop(optional)] margin: Option<Margin>,
    #[prop(optional)] inline: Option<bool>,
    children: Children,
) -> impl IntoView {
    let style = margin.map(|it| format!("--margin: {it}"));
    view! { cx,
        <leptonic-code id=id inline=inline.map(|it| it.to_string()) style=style>
            {children(cx)}
        </leptonic-code>
    }
}
