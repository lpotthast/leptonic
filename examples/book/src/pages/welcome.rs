use leptonic::prelude::*;
use leptos::*;

use crate::app::AppRoutes;

#[component]
pub fn PageWelcome(cx: Scope) -> impl IntoView {
    view! { cx,
        <Box id="welcome-page">
            <Grid spacing=Size::Em(0.6)>
                <Row>
                    <Col md=5 sm=5 xs=12>
                        <H1 id="leptonic">"LEPTONIC"</H1>
                    </Col>
                    <Col md=6 sm=6 xs=12>
                        <img src="/res/leptonic.svg" style="height: 10em; width: 10em;"/>
                    </Col>
                </Row>
            </Grid>

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
