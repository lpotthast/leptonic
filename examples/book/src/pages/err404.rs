use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn PageErr404(cx: Scope) -> impl IntoView {
    view! { cx,
        <H2>"404 Not Found"</H2>

        <Link href="">
            <Button on_click=move |_| {} variant=ButtonVariant::Filled>
                "Back"
            </Button>
        </Link>
    }
}
