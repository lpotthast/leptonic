use leptos::prelude::*;

use crate::hooks::{use_press, PressEvent, UsePressInput, UsePressReturn};

#[component]
pub fn TableContainer(children: Children) -> impl IntoView {
    view! { <leptonic-table-container>{children()}</leptonic-table-container> }
}

#[component]
pub fn Table(
    #[prop(optional)] bordered: Option<bool>,
    #[prop(optional)] hoverable: Option<bool>,
    children: Children,
) -> impl IntoView {
    view! {
        <leptonic-table
            class:leptonic-table-bordered=bordered.unwrap_or(false)
            class:leptonic-table-hoverable=hoverable.unwrap_or(false)
        >
            {children()}
        </leptonic-table>
    }
}

#[component]
pub fn TableHeader(children: Children) -> impl IntoView {
    view! { <leptonic-table-header>{children()}</leptonic-table-header> }
}

#[component]
pub fn TableBody(children: Children) -> impl IntoView {
    view! { <leptonic-table-body>{children()}</leptonic-table-body> }
}

#[component]
pub fn TableFooter(children: Children) -> impl IntoView {
    view! { <leptonic-table-footer>{children()}</leptonic-table-footer> }
}

#[component]
pub fn TableRow(children: Children) -> impl IntoView {
    view! { <leptonic-table-row>{children()}</leptonic-table-row> }
}

#[component]
pub fn TableHeaderCell(
    #[prop(optional)] min_width: Option<bool>,
    #[prop(optional, into)] on_press: Option<Callback<PressEvent>>,
    children: Children,
) -> impl IntoView {
    let UsePressReturn { props, .. } = use_press(UsePressInput {
        disabled: false.into(),
        force_prevent_default: false,
        allow_propagation: false,
        allow_text_selection_on_press: false,
        should_cancel_on_pointer_exit: false,
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
    });

    view! {
        <leptonic-table-header-cell
            class:min-width=min_width.unwrap_or(false)
            {..props.into_attrs()}
        >
            {children()}
        </leptonic-table-header-cell>
    }
}

#[component]
pub fn TableCell(children: Children) -> impl IntoView {
    view! { <leptonic-table-cell>{children()}</leptonic-table-cell> }
}
