use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn PageCollapsible() -> impl IntoView {
    view! {
        <H1>"Collapsibles"</H1>

        <Collapsibles default_on_open=OnOpen::CloseOthers>
            <Stack spacing=Size::Em(0.6)>
                <Collapsible
                    header=move || "Header1".into_view()
                    body=move || view! { "Body1" }.into_view() />
                <Collapsible
                    header=move || "Header2".into_view()
                    body=move || view! { "Body2" }.into_view() />
                <Collapsible
                    header=move || "Header3 - on_open::DoNothing".into_view()
                    body=move || view! { "Body3" }.into_view()
                    on_open=OnOpen::DoNothing />
            </Stack>
        </Collapsibles>
    }
}
