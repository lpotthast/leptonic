use leptonic::prelude::*;
use leptos::*;

use crate::app::AppRoutes;

#[component]
pub fn PageErr404(cx: Scope) -> impl IntoView {
    view! { cx,
        <H1>"404 Not Found"</H1>

        <div class="err-404">
            <div class="info">
                <H1 id="error">"404"</H1>
                <H2 id="whoops">"Whoops, this page doesn’t exist :-("</H2>

                <Link href=AppRoutes::Welcome>
                    <Button on_click=move |_| {} variant=ButtonVariant::Filled>
                        "Back"
                    </Button>
                </Link>
            </div>
            <img id="ferris" src="/res/icon/ferris-panic.svg" alt="Panicked ferris (crab, logo of Rust)" />
        </div>
    }
}
