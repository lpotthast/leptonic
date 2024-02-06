use leptos::*;

#[component]
pub fn TableContainer(
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<AttributeValue>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    children: Children,
) -> impl IntoView {
    view! {
        <leptonic-table-container id=id class=class style=style>
            {children()}
        </leptonic-table-container>
    }
}

#[component]
pub fn Table(
    #[prop(optional)] bordered: Option<bool>,
    #[prop(optional)] hoverable: Option<bool>,
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<AttributeValue>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    children: Children,
) -> impl IntoView {
    view! {
        <leptonic-table
            id=id
            class=class
            class:leptonic-table-bordered=bordered.unwrap_or(false)
            class:leptonic-table-hoverable=hoverable.unwrap_or(false)
            style=style
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
