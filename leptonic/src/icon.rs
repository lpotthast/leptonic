use leptos::*;

use crate::Margin;

/// The Icon component.
#[component]
pub fn Icon(
    /// The icon to render.
    #[prop(into)]
    icon: MaybeSignal<icondata::Icon>,
    /// The width of the icon (horizontal side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional)]
    width: MaybeProp<TextProp>,
    /// The height of the icon (vertical side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional)]
    height: MaybeProp<TextProp>,

    #[prop(into, optional)] id: Option<AttributeValue>,

    /// HTML class attribute.
    #[prop(into, optional)]
    class: Option<AttributeValue>,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: Option<AttributeValue>,
    #[prop(optional)] margin: Option<Margin>,
    #[prop(into, optional)] aria_label: Option<AttributeValue>,
) -> impl IntoView
where
{
    let svg = move || {
        let icon = icon.get();
        let mut svg = svg::svg();
        svg = match icon.style {
            Some(s) => svg.attr("style", s),
            None => svg,
        };
        if let Some(x) = icon.x {
            svg = svg.attr("x", x);
        }
        if let Some(y) = icon.y {
            svg = svg.attr("y", y);
        }
        // The style set by the user overrides the style set by the icon.
        // We ignore the width and height attributes of the icon, even if the user hasn't specified any.
        svg = svg.attr(
            "width",
            Attribute::String(match (width.get(), icon.width) {
                (Some(a), _) => Oco::from(a.get()),
                _ => Oco::from("1em"),
            }),
        );
        svg = svg.attr(
            "height",
            Attribute::String(match (height.get(), icon.height) {
                (Some(a), _) => Oco::from(a.get()),
                _ => Oco::from("1em"),
            }),
        );
        if let Some(view_box) = icon.view_box {
            svg = svg.attr("viewBox", view_box);
        }
        if let Some(stroke_linecap) = icon.stroke_linecap {
            svg = svg.attr("stroke-linecap", stroke_linecap);
        }
        if let Some(stroke_linejoin) = icon.stroke_linejoin {
            svg = svg.attr("stroke-linejoin", stroke_linejoin);
        }
        if let Some(stroke_width) = icon.stroke_width {
            svg = svg.attr("stroke-width", stroke_width);
        }
        if let Some(stroke) = icon.stroke {
            svg = svg.attr("stroke", stroke);
        }
        svg = svg.attr("fill", icon.fill.unwrap_or("currentColor"));
        svg = svg.attr("role", "graphics-symbol");
        svg = svg.inner_html(icon.data);
        svg
    };

    view! {
        <leptonic-icon id=id class=class aria_label=aria_label style=style style=("--margin", move || margin.map(|it| format!("{it}")))>
            { svg }
        </leptonic-icon>
    }
}
