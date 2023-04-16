use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn PageCollapsible(cx: Scope) -> impl IntoView {
    view! { cx,
        <h2>"Collapsibles"</h2>

        <Collapsibles default_on_open=OnOpen::CloseOthers>
            <Collapsible
                header="Header1"
                body=view! {cx, "Body1"} />
            <Collapsible
                header="Header2"
                body=view! {cx, "Body2"} />
            <Collapsible
                header="Header3 - on_open::DoNothing"
                body=view! {cx, "Body3"}
                on_open=OnOpen::DoNothing />
        </Collapsibles>
    }
}
