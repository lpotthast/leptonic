use leptonic::prelude::*;
use leptos::*;

use crate::app::AppRoutes;

#[component]
pub fn PageErr404(cx: Scope) -> impl IntoView {
    view! { cx,
        <H1>"404 Not Found"</H1>

        <Link href=AppRoutes::Welcome>
            <Button on_click=move |_| {} variant=ButtonVariant::Filled>
                "Back"
            </Button>
        </Link>
    }
}
