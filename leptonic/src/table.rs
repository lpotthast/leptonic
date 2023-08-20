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
        <leptonic-table-container>
            {children(cx)}
        </leptonic-table-container>
    }
}

#[component]
pub fn Table(
    cx: Scope,
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<AttributeValue>,
    #[prop(into, optional)] style: Option<AttributeValue>,
    children: Children,
) -> impl IntoView {
    view! {cx,
        //<table class="crud-table crud-table-bordered crud-table-hoverable">
        <leptonic-table id=id class=class style=style>
            {children(cx)}
        </leptonic-table>
    }
}

#[component]
pub fn THeader(cx: Scope) -> impl IntoView {
    view! {cx,
        <leptonic-table-header>
        </leptonic-table-header>
    }
}

#[component]
pub fn TBody(cx: Scope) -> impl IntoView {
    view! {cx,
        <leptonic-table-header>
        </leptonic-table-header>
    }
}

#[component]
pub fn TFooter(cx: Scope) -> impl IntoView {
    view! {cx,
        <leptonic-table-header>
        </leptonic-table-header>
    }
}

#[component]
pub fn TRow(cx: Scope) -> impl IntoView {
    view! {cx,
        <leptonic-table-header>
        </leptonic-table-header>
    }
}

#[component]
pub fn TCell(cx: Scope) -> impl IntoView {
    view! {cx,
        <leptonic-table-header>
        </leptonic-table-header>
    }
}
