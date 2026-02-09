use leptos::prelude::*;

use crate::hooks::{use_press, PressEvent, UsePressInput, UsePressReturn};

#[component]
pub fn TableContainer(children: Children) -> impl IntoView {
    view! { <div class="leptonic-table-container">{children()}</div> }
}

#[component]
pub fn Table(
    #[prop(optional)] bordered: Option<bool>,
    #[prop(optional)] hoverable: Option<bool>,
    children: Children,
) -> impl IntoView {
    view! {
        <table
            class="leptonic-table"
            class:leptonic-table-bordered=bordered.unwrap_or(false)
            class:leptonic-table-hoverable=hoverable.unwrap_or(false)
        >
            {children()}
        </table>
    }
}

#[component]
pub fn TableHeader(children: Children) -> impl IntoView {
    view! { <thead class="leptonic-table-header">{children()}</thead> }
}

#[component]
pub fn TableBody(children: Children) -> impl IntoView {
    view! { <tbody class="leptonic-table-body">{children()}</tbody> }
}

#[component]
pub fn TableFooter(children: Children) -> impl IntoView {
    view! { <tfoot class="leptonic-table-footer">{children()}</tfoot> }
}

#[component]
pub fn TableRow(children: Children) -> impl IntoView {
    view! { <tr class="leptonic-table-row">{children()}</tr> }
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
        on_long_press_start: None,
        on_long_press: None,
        on_long_press_end: None,
        long_press_threshold: None,
        long_press_accessibility_description: None,
    });

    view! {
        <th
            class="leptonic-table-header-cell"
            class:min-width=min_width.unwrap_or(false)
            {..props.into_attrs()}
        >
            {children()}
        </th>
    }
}

#[component]
pub fn TableCell(children: Children) -> impl IntoView {
    view! { <td class="leptonic-table-cell">{children()}</td> }
}
