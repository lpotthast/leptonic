use leptos::*;

// TODO: Implement

// struct TableCtx {}

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
        //<table class="crud-table crud-table-bordered crud-table-hoverable">
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
pub fn Thead( children: Children) -> impl IntoView {
    view! {
        <thead>{children()}</thead>
    }
}

#[component]
pub fn Tbody( children: Children) -> impl IntoView {
    view! {
        <tbody>{children()}</tbody>
    }
}

#[component]
pub fn Tfoot( children: Children) -> impl IntoView {
    view! {
        <tfoot>{children()}</tfoot>
    }
}

#[component]
pub fn Tr( children: Children) -> impl IntoView {
    view! {
        <tr>{children()}</tr>
    }
}

#[component]
pub fn Th( #[prop(optional)] min_width: Option<bool>, children: Children) -> impl IntoView {
    view! {
        <th class:min-width=min_width.unwrap_or(false)>{children()}</th>
    }
}
#[component]
pub fn Td( children: Children) -> impl IntoView {
    view! {
        <td>{children()}</td>
    }
}
