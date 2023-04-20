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
    Code,
}

// TODO: Control text properties like wrapping through props...

#[component]
pub fn Typography(
    cx: Scope,
    variant: TypographyVariant,
    #[prop(optional)] margin: Option<Margin>,
    children: Children,
) -> impl IntoView {
    let style = margin.map(|it| format!("--margin: {it}"));

    match variant {
        TypographyVariant::H1 => view! {cx, <h1 style=style>{children(cx)}</h1>}.into_view(cx),
        TypographyVariant::H2 => view! {cx, <h2 style=style>{children(cx)}</h2>}.into_view(cx),
        TypographyVariant::H3 => view! {cx, <h3 style=style>{children(cx)}</h3>}.into_view(cx),
        TypographyVariant::H4 => view! {cx, <h4 style=style>{children(cx)}</h4>}.into_view(cx),
        TypographyVariant::H5 => view! {cx, <h5 style=style>{children(cx)}</h5>}.into_view(cx),
        TypographyVariant::H6 => view! {cx, <h6 style=style>{children(cx)}</h6>}.into_view(cx),
        TypographyVariant::Paragraph => view! {cx, <p style=style>{children(cx)}</p>}.into_view(cx),
        TypographyVariant::Code => {
            view! {cx, <leptonic-code style=style>{children(cx)}</leptonic-code>}.into_view(cx)
        }
    }
}
