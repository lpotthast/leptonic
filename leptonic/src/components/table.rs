use leptos::prelude::*;

use crate::{
    hooks::{PressEvent, UsePressInput, UsePressReturn, use_press},
    utils::{classes::Classes, styles::Styles},
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
    #[prop(optional)] bordered: Option<bool>,
    #[prop(optional)] hoverable: Option<bool>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    view! {
        <table
            class=classes.add("leptonic-table")
            class:leptonic-table-bordered=bordered.unwrap_or(false)
            class:leptonic-table-hoverable=hoverable.unwrap_or(false)
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
    #[prop(optional)] min_width: Option<bool>,
    #[prop(optional, into)] on_press: Option<Callback<PressEvent>>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let UsePressReturn {
        props: press_props, ..
    } = use_press(UsePressInput {
        disabled: false.into(),
        force_prevent_default: false,
        force_propagation: false,
        allow_text_selection_on_press: false,
        should_cancel_on_pointer_exit: false,
        prevent_focus_on_press: false,
        force_is_pressed: None,
        on_press: Callback::new(move |e| {
            if let Some(on_press) = on_press {
                on_press.run(e);
            }
        }),
        on_press_up: None,
        on_press_start: None,
        on_press_end: None,
        on_press_change: None,
        on_double_press: None,
        on_long_press_start: None,
        on_long_press: None,
        on_long_press_end: None,
        long_press_threshold: None,
        long_press_accessibility_description: None,
    });

    let (press_attrs, press_styles) = press_props.into_parts();
    let styles = press_styles.merge(styles);

    view! {
        <th
            {..press_attrs}
            class=classes.add("leptonic-table-header-cell").add(("min-width", min_width.unwrap_or(true)))
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
