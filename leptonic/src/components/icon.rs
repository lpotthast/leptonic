use crate::Margin;
use leptos::prelude::*;
use leptos::svg;
use leptos::text_prop::TextProp;

/// The Icon component.
#[component]
pub fn Icon(
    /// The icon to render.
    #[prop(into)]
    icon: Signal<icondata::Icon>,

    /// The width of the icon (horizontal side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional)]
    width: MaybeProp<TextProp>,

    /// The height of the icon (vertical side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional)]
    height: MaybeProp<TextProp>,

    #[prop(optional)] margin: Option<Margin>,

    #[prop(into, optional)] aria_label: Option<Oco<'static, str>>,
) -> impl IntoView
where
{
    let svg = move || {
        let icon = icon.get();
        // TODO (new): Does into_any() do what we want? We deed it as svg() itself is now typed and alters the type whenever attributes are added.
        // TODO (new): We only add an arbitrary attribute here (aria-hidden) to get an `AnyViewWithAttrs`, on which we can conditionally add further attributes ...
        let mut svg = svg::svg()
            .inner_html(icon.data)
            .into_any()
            .attr("aria-hidden", "false");
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
            match (width.get(), icon.width) {
                (Some(a), _) => a.get(),
                _ => Oco::from("1em"),
            },
        );
        svg = svg.attr(
            "height",
            match (height.get(), icon.height) {
                (Some(a), _) => a.get(),
                _ => Oco::from("1em"),
            },
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
        svg
    };

    view! {
        <span
            class="leptonic-icon"
            aria_label=aria_label
            style=margin.map(|it| { ("--margin", format!("{it}")) })
        >
            {svg}
        </span>
    }
}
