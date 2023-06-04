use leptos::*;

use crate::{Margin, Size, FontWeight};

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
    match variant {
        TypographyVariant::H1 => H1(
            cx,
            H1Props {
                id,
                margin,
                font_size: None,
                children,
            },
        )
        .into_view(cx),
        TypographyVariant::H2 => H2(
            cx,
            H2Props {
                id,
                margin,
                font_size: None,
                font_weight: None,
                style: None,
                children,
            },
        )
        .into_view(cx),
        TypographyVariant::H3 => H3(
            cx,
            H3Props {
                id,
                margin,
                font_size: None,
                children,
            },
        )
        .into_view(cx),
        TypographyVariant::H4 => H4(
            cx,
            H4Props {
                id,
                margin,
                font_size: None,
                children,
            },
        )
        .into_view(cx),
        TypographyVariant::H5 => H5(
            cx,
            H5Props {
                id,
                margin,
                font_size: None,
                children,
            },
        )
        .into_view(cx),
        TypographyVariant::H6 => H6(
            cx,
            H6Props {
                id,
                margin,
                font_size: None,
                children,
            },
        )
        .into_view(cx),
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
pub fn H1(
    cx: Scope,
    /// Sets the `id` attribute, making it easier to target.
    #[prop(into, optional)]
    id: Option<String>,
    #[prop(optional)] margin: Option<Margin>,
    #[prop(optional)] font_size: Option<Size>,
    children: Children,
) -> impl IntoView {
    let margin = margin.map(|it| format!("--margin: {it}"));
    let font_size = font_size.map(|it| format!("font-size: {it}"));
    let style = match (margin, font_size) {
        (Some(a), Some(b)) => Some(format!("{a}; {b}")),
        (Some(a), None) => Some(a),
        (None, Some(b)) => Some(b),
        (None, None) => None,
    };
    view! { cx,
        <h1 id=id style=style>
            {children(cx)}
        </h1>
    }
}

#[component]
pub fn H2(
    cx: Scope,
    /// Sets the `id` attribute, making it easier to target.
    #[prop(into, optional)]
    id: Option<String>,
    #[prop(optional)] margin: Option<Margin>,
    #[prop(optional)] font_size: Option<Size>,
    #[prop(optional)] font_weight: Option<FontWeight>,
    #[prop(into, optional)] style: Option<String>,
    children: Children,
) -> impl IntoView {
    let mut final_style = String::new();
    if let Some(margin) = margin {
        final_style.push_str("--margin: ");
        final_style.push_str(&margin.to_string()); // TODO: perf
        final_style.push_str(";");
    }
    if let Some(font_size) = font_size {
        final_style.push_str("font-size: ");
        final_style.push_str(&font_size.to_string()); // TODO: perf
        final_style.push_str(";");
    }
    if let Some(font_weight) = font_weight {
        final_style.push_str("font-weight: ");
        final_style.push_str(&font_weight.to_string()); // TODO: perf
        final_style.push_str(";");
    }
    if let Some(style) = style {
        final_style.push_str(&style);
    }
    let final_style = match final_style.is_empty() {
        true => None,
        false => Some(final_style),
    };
    view! { cx,
        <h2 id=id style=final_style>
            {children(cx)}
        </h2>
    }
}

#[component]
pub fn H3(
    cx: Scope,
    /// Sets the `id` attribute, making it easier to target.
    #[prop(into, optional)]
    id: Option<String>,
    #[prop(optional)] margin: Option<Margin>,
    #[prop(optional)] font_size: Option<Size>,
    children: Children,
) -> impl IntoView {
    let margin = margin.map(|it| format!("--margin: {it}"));
    let font_size = font_size.map(|it| format!("font-size: {it}"));
    let style = match (margin, font_size) {
        (Some(a), Some(b)) => Some(format!("{a}; {b}")),
        (Some(a), None) => Some(a),
        (None, Some(b)) => Some(b),
        (None, None) => None,
    };
    view! { cx,
        <h3 id=id style=style>
            {children(cx)}
        </h3>
    }
}

#[component]
pub fn H4(
    cx: Scope,
    /// Sets the `id` attribute, making it easier to target.
    #[prop(into, optional)]
    id: Option<String>,
    #[prop(optional)] margin: Option<Margin>,
    #[prop(optional)] font_size: Option<Size>,
    children: Children,
) -> impl IntoView {
    let margin = margin.map(|it| format!("--margin: {it}"));
    let font_size = font_size.map(|it| format!("font-size: {it}"));
    let style = match (margin, font_size) {
        (Some(a), Some(b)) => Some(format!("{a}; {b}")),
        (Some(a), None) => Some(a),
        (None, Some(b)) => Some(b),
        (None, None) => None,
    };
    view! { cx,
        <h4 id=id style=style>
            {children(cx)}
        </h4>
    }
}

#[component]
pub fn H5(
    cx: Scope,
    /// Sets the `id` attribute, making it easier to target.
    #[prop(into, optional)]
    id: Option<String>,
    #[prop(optional)] margin: Option<Margin>,
    #[prop(optional)] font_size: Option<Size>,
    children: Children,
) -> impl IntoView {
    let margin = margin.map(|it| format!("--margin: {it}"));
    let font_size = font_size.map(|it| format!("font-size: {it}"));
    let style = match (margin, font_size) {
        (Some(a), Some(b)) => Some(format!("{a}; {b}")),
        (Some(a), None) => Some(a),
        (None, Some(b)) => Some(b),
        (None, None) => None,
    };
    view! { cx,
        <h5 id=id style=style>
            {children(cx)}
        </h5>
    }
}

#[component]
pub fn H6(
    cx: Scope,
    /// Sets the `id` attribute, making it easier to target.
    #[prop(into, optional)]
    id: Option<String>,
    #[prop(optional)] margin: Option<Margin>,
    #[prop(optional)] font_size: Option<Size>,
    children: Children,
) -> impl IntoView {
    let margin = margin.map(|it| format!("--margin: {it}"));
    let font_size = font_size.map(|it| format!("font-size: {it}"));
    let style = match (margin, font_size) {
        (Some(a), Some(b)) => Some(format!("{a}; {b}")),
        (Some(a), None) => Some(a),
        (None, Some(b)) => Some(b),
        (None, None) => None,
    };
    view! { cx,
        <h6 id=id style=style>
            {children(cx)}
        </h6>
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
