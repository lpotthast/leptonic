use leptos::{html::Div, prelude::*};

use crate::utils::{
    classes::Classes,
    css::{CssDimension, Size, computed_size, px},
    style::{HeightProperty, MinHeightProperty, MinWidthProperty, WidthProperty},
    styles::Styles,
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

    let zero: Size = computed_size(CssDimension::Zero);
    let styles = match axis {
        CollapseAxis::X => styles
            .add(MinWidthProperty.declare(zero))
            .add_reactive(move || WidthProperty.declare(computed_size(axis_dimension.get()))),
        CollapseAxis::Y => styles
            .add(MinHeightProperty.declare(zero))
            .add_reactive(move || HeightProperty.declare(computed_size(axis_dimension.get()))),
    };
    let classes = classes.add("leptonic-collapse").add(match axis {
        CollapseAxis::X => "width",
        CollapseAxis::Y => "height",
    });

    view! {
        <div
            class=classes
            style=styles
        >
            <div class="content" class:show=move || show.get() node_ref=content>
                {children()}
            </div>
        </div>
    }
}
