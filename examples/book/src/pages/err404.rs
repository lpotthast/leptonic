use leptos::*;
use leptonic::prelude::*;

#[component]
pub fn PageErr404(cx: Scope) -> impl IntoView {
    view! { cx,
        <h2>"404 Not Found"</h2>

        <Link href="">
            <Button on_click=move |_| {} variant=ButtonVariant::Filled>
                "Back"
            </Button>
        </Link>
    }
}