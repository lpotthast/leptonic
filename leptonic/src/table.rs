use leptos::*;

// TODO: Implement

// struct TableCtx {}

#[component]
pub fn TableContainer(
    cx: Scope,
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<AttributeValue>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    children: Children,
) -> impl IntoView {
    view! {cx,
        <leptonic-table-container id=id class=class style=style>
            {children(cx)}
        </leptonic-table-container>
    }
}

#[component]
pub fn Table(
    cx: Scope,
    #[prop(optional)] bordered: Option<bool>,
    #[prop(optional)] hoverable: Option<bool>,
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<AttributeValue>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    children: Children,
) -> impl IntoView {
    view! {cx,
        //<table class="crud-table crud-table-bordered crud-table-hoverable">
        <leptonic-table
            id=id
            class=class
            class:leptonic-table-bordered=bordered.unwrap_or(false)
            class:leptonic-table-hoverable=hoverable.unwrap_or(false)
            style=style
        >
            {children(cx)}
        </leptonic-table>
    }
}

#[component]
pub fn Thead(cx: Scope, children: Children) -> impl IntoView {
    view! {cx,
        <thead>{children(cx)}</thead>
    }
}

#[component]
pub fn Tbody(cx: Scope, children: Children) -> impl IntoView {
    view! {cx,
        <tbody>{children(cx)}</tbody>
    }
}

#[component]
pub fn Tfoot(cx: Scope, children: Children) -> impl IntoView {
    view! {cx,
        <tfoot>{children(cx)}</tfoot>
    }
}

#[component]
pub fn Tr(cx: Scope, children: Children) -> impl IntoView {
    view! {cx,
        <tr>{children(cx)}</tr>
    }
}

#[component]
pub fn Th(cx: Scope, #[prop(optional)] min_width: Option<bool>, children: Children) -> impl IntoView {
    view! {cx,
        <th class:min-width=min_width.unwrap_or(false)>{children(cx)}</th>
    }
}
#[component]
pub fn Td(cx: Scope, children: Children) -> impl IntoView {
    view! {cx,
        <td>{children(cx)}</td>
    }
}
