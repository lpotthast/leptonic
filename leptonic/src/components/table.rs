use leptos::prelude::*;

#[component]
pub fn TableContainer(
    children: Children,
) -> impl IntoView {
    view! {
        <leptonic-table-container>
            {children()}
        </leptonic-table-container>
    }
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
    view! {
        <leptonic-table-header>{children()}</leptonic-table-header>
    }
}

#[component]
pub fn TableBody(children: Children) -> impl IntoView {
    view! {
        <leptonic-table-body>
            {children()}
        </leptonic-table-body>
    }
}

#[component]
pub fn TableFooter(children: Children) -> impl IntoView {
    view! {
        <leptonic-table-footer>
            {children()}
        </leptonic-table-footer>
    }
}

#[component]
pub fn TableRow(children: Children) -> impl IntoView {
    view! {
        <leptonic-table-row>
            {children()}
        </leptonic-table-row>
    }
}

#[component]
pub fn TableHeaderCell(
    #[prop(optional)] min_width: Option<bool>,
    children: Children,
) -> impl IntoView {
    view! {
        <leptonic-table-header-cell class:min-width=min_width.unwrap_or(false)>
            {children()}
        </leptonic-table-header-cell>
    }
}

#[component]
pub fn TableCell(children: Children) -> impl IntoView {
    view! {
        <leptonic-table-cell>
            {children()}
        </leptonic-table-cell>
    }
}
