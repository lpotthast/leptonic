use leptos::prelude::*;

use crate::{
    hooks::{PressEvent, UsePressInput, UsePressReturn, use_press},
    utils::{aria::AriaSort, classes::Classes, styles::Styles},
};

#[component]
pub fn TableContainer(
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    view! { <div class=classes.add("leptonic-table-container") style=styles>{children()}</div> }
}

#[component]
pub fn Table(
    /// Borders around every cell.
    #[prop(into, optional)]
    bordered: Signal<bool>,
    /// Highlights the hovered row.
    #[prop(into, optional)]
    hoverable: Signal<bool>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    view! {
        <table
            class=classes
                .add("leptonic-table")
                .add_reactive("leptonic-table-bordered", bordered)
                .add_reactive("leptonic-table-hoverable", hoverable)
            style=styles
        >
            {children()}
        </table>
    }
}

#[component]
pub fn TableHeader(
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    view! { <thead class=classes.add("leptonic-table-header") style=styles>{children()}</thead> }
}

#[component]
pub fn TableBody(
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    view! { <tbody class=classes.add("leptonic-table-body") style=styles>{children()}</tbody> }
}

#[component]
pub fn TableFooter(
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    view! { <tfoot class=classes.add("leptonic-table-footer") style=styles>{children()}</tfoot> }
}

#[component]
pub fn TableRow(
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    view! { <tr class=classes.add("leptonic-table-row") style=styles>{children()}</tr> }
}

#[component]
pub fn TableHeaderCell(
    /// Whether the column keeps its content's minimum width (no wrapping). Default: `true`.
    #[prop(into, default = Signal::stored(true))]
    min_width: Signal<bool>,
    /// Makes the header pressable (e.g. to sort by its column): it becomes focusable and reacts to
    /// Enter and Space too.
    #[prop(optional, into)]
    on_press: Option<Callback<PressEvent>>,
    /// The column's sort order, for assistive technology (`aria-sort`).
    #[prop(optional, into)]
    aria_sort: MaybeProp<AriaSort>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let UsePressReturn {
        props: press_props, ..
    } = use_press(UsePressInput {
        on_press: Some(Callback::new(move |e| {
            if let Some(on_press) = on_press {
                on_press.run(e);
            }
        })),
        ..UsePressInput::default()
    });

    let (press_attrs, press_styles) = press_props.into_parts();
    let styles = press_styles.merge(styles);
    let tabindex = on_press.is_some().then_some(0);

    view! {
        <th
            {..press_attrs}
            tabindex=tabindex
            aria-sort=move || aria_sort.get()
            class=classes.add("leptonic-table-header-cell").add_reactive("min-width", min_width)
            style=styles
        >
            {children()}
        </th>
    }
}

#[component]
pub fn TableCell(
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    view! { <td class=classes.add("leptonic-table-cell") style=styles>{children()}</td> }
}
