use leptos::{html::Div, prelude::*};

use crate::utils::{
    classes::Classes,
    css::px,
    styles::{Height, MinHeight, MinWidth, Styles, Width},
};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub enum CollapseAxis {
    X,
    #[default]
    Y,
}

#[component]
pub fn Collapse(
    #[prop(into)] show: Signal<bool>,
    #[prop(optional)] axis: CollapseAxis,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let content: NodeRef<Div> = NodeRef::new();

    let axis_dimension = Signal::derive(move || {
        let show = show.get();
        let el_axis_dimension = content.get().map_or(0, |el| match axis {
            CollapseAxis::X => el.scroll_width(),
            CollapseAxis::Y => el.scroll_height(),
        });
        px(f64::from(if show { el_axis_dimension } else { 0 }))
    });

    let styles = styles
        .add_optional(MinWidth, move || (axis == CollapseAxis::X).then_some("0px"))
        .add_optional(Width, move || {
            (axis == CollapseAxis::X).then(|| axis_dimension.get())
        })
        .add_optional(MinHeight, move || {
            (axis == CollapseAxis::Y).then_some("0px")
        })
        .add_optional(Height, move || {
            (axis == CollapseAxis::Y).then(|| axis_dimension.get())
        });

    view! {
        <div
            class=classes.add("leptonic-collapse")
            class:width=move || { axis == CollapseAxis::X }
            class:height=move || { axis == CollapseAxis::Y }
            style=styles
        >
            <div class="content" class:show=move || show.get() node_ref=content>
                {children()}
            </div>
        </div>
    }
}
