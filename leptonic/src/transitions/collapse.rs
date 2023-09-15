use leptos::{html::Div, *};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum CollapseAxis {
    X,
    Y,
}

impl Default for CollapseAxis {
    fn default() -> Self {
        Self::Y
    }
}

#[component]
pub fn Collapse(
    #[prop(into)] show: Signal<bool>,
    #[prop(optional)] axis: CollapseAxis,
    children: Children,
) -> impl IntoView {
    let content: NodeRef<Div> = create_node_ref();

    let style = Signal::derive(move || {
        let show = show.get();
        let el_axis_dimension = content
            .get()
            .map(|el| match axis {
                CollapseAxis::X => el.scroll_width(),
                CollapseAxis::Y => el.scroll_height(),
            })
            .unwrap_or(0);
        match axis {
            CollapseAxis::X => format!(
                "min-width: 0px; width: {}px",
                if show { el_axis_dimension } else { 0 }
            ),
            CollapseAxis::Y => format!(
                "min-height: 0px; height: {}px",
                if show { el_axis_dimension } else { 0 }
            ),
        }
    });

    view! {
        <div class="leptonic-collapse"
            class:width=move || {axis == CollapseAxis::X}
            class:height=move || {axis == CollapseAxis::Y}
            style=move || style.get()
        >
            <div class="content" class:show=move || show.get() node_ref=content>
                { children() }
            </div>
        </div>
    }
}
