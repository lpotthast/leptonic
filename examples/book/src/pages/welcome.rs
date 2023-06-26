use leptonic::prelude::*;
use leptos::*;

use crate::app::AppRoutes;

#[component]
pub fn PageWelcome(cx: Scope) -> impl IntoView {
    view! { cx,
        <Box id="welcome-page">
            <img src="/res/leptonic.svg" id="big-logo"/>

            <H1 id="slogan">
                "LEPTONIC"
            </H1>

            <H2 id="sub-slogan">
                "Elevate Web Development with Unleashed Performance, Ergonomic Design, and Reactive Delight!"
            </H2>

            <Link href=AppRoutes::Doc>
                <Button size=ButtonSize::Big on_click=move |_| {} style="font-size: 1.5em; margin: 2em;">
                    "Read the docs"
                </Button>
            </Link>
        </Box>
    }
}
