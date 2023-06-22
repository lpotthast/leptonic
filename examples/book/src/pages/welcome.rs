use leptonic::prelude::*;
use leptos::*;

use crate::app::AppRoutes;

#[component]
pub fn PageWelcome(cx: Scope) -> impl IntoView {
    view! { cx,
        <Box style="max-width: 1200px; padding: 1em; margin: auto;">
            <Box style="">

                <H1>"LEPTONIC"</H1>

                <img src="/res/leptonic.svg" style="height: 10em;"/>

            </Box>

            <P>"Similar to Leptos, this crate comes with a prelude module."</P>
            <P>
                "Just "
                <Code inline=true>"use leptonic::prelude::*;"</Code>
                " and you are ready to go."
            </P>

            <Link href=AppRoutes::Doc>  
                <Button on_click=move |_| {}>
                    "Read the docs"
                </Button>
            </Link>
        </Box>
    }
}
