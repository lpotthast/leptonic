use leptos::*;

pub enum TypographyVariant {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
}

// TODO: Control text properties like wrapping through props...

#[component]
pub fn Typography(cx: Scope, variant: TypographyVariant, children: Children) -> impl IntoView {
    match variant {
        TypographyVariant::H1 => view! {cx, <h1>{children(cx)}</h1>}.into_view(cx),
        TypographyVariant::H2 => view! {cx, <h2>{children(cx)}</h2>}.into_view(cx),
        TypographyVariant::H3 => view! {cx, <h3>{children(cx)}</h3>}.into_view(cx),
        TypographyVariant::H4 => view! {cx, <h4>{children(cx)}</h4>}.into_view(cx),
        TypographyVariant::H5 => view! {cx, <h5>{children(cx)}</h5>}.into_view(cx),
        TypographyVariant::H6 => view! {cx, <h6>{children(cx)}</h6>}.into_view(cx),
    }
}
