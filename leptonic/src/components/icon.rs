use leptos::{prelude::*, svg};
use leptos_styles::css::css_custom_property;

use crate::{
    Height, Margin, Width,
    utils::{classes::Classes, styles::Styles},
};

css_custom_property!(MARGIN: leptos_styles::css::Margin = "--margin");
css_custom_property!(ICON_WIDTH: Width = "--icon-width");
css_custom_property!(ICON_HEIGHT: Height = "--icon-height");

/// The Icon component.
#[component]
pub fn Icon(
    /// The icon to render.
    #[prop(into)]
    icon: Signal<icondata::Icon>,

    /// The width of the icon. Default: the theme's (`1rem`).
    #[prop(optional)]
    width: Option<Width>,

    /// The height of the icon. Default: the theme's (`1rem`).
    #[prop(optional)]
    height: Option<Height>,

    #[prop(optional)] margin: Option<Margin>,

    /// Labels the icon as an image. Without it, the icon is decorative (hidden from assistive
    /// technology), e.g. next to text or inside a labelled button.
    #[prop(into, optional)]
    aria_label: Option<Oco<'static, str>>,

    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
) -> impl IntoView
where
{
    let svg = move || {
        let icon = icon.get();
        // TODO (new): Does into_any() do what we want? We deed it as svg() itself is now typed and alters the type whenever attributes are added.
        // TODO (new): We only add an arbitrary attribute here (aria-hidden) to get an `AnyViewWithAttrs`, on which we can conditionally add further attributes ...
        // SAFETY: `icon.data` is a `&'static str` from the `icondata` crate,
        // containing compile-time SVG path data. Not user-supplied input.
        let mut svg = svg::svg()
            .inner_html(icon.data)
            .into_any()
            // The `<span>` around carries the label, if any.
            .attr("aria-hidden", "true");
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
        // The `<span>` around sets the size (the theme fills it), not the icon's own attributes.
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
        svg
    };

    let is_labelled = aria_label.is_some();
    let styles = styles
        .add_optional(margin.map(|margin| MARGIN.declare(margin)))
        .add_optional(width.map(|width| ICON_WIDTH.declare(width)))
        .add_optional(height.map(|height| ICON_HEIGHT.declare(height)));
    view! {
        <span
            class=classes.add("leptonic-icon")
            role=is_labelled.then_some("img")
            aria-label=aria_label
            aria-hidden=(!is_labelled).then_some("true")
            style=styles
        >
            {svg}
        </span>
    }
}
