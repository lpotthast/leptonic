use leptonic::prelude::*;
use leptos::*;

use crate::app::AppRoutes;

#[component]
pub fn PageErr404() -> impl IntoView {
    view! {
        <Grid spacing=Size::Zero class="err-404">
            <Row>
                <Col sm=6 xs=12 class="info" h_align=ColAlign::Center>
                    <H1 id="error">"404"</H1>
                    <H2 id="whoops">"Whoops, this page doesn't exist :-("</H2>

                    <Link href=AppRoutes::Welcome>
                        <Button id="back-btn" on_click=move |_| {} variant=ButtonVariant::Filled>
                            "Back"
                        </Button>
                    </Link>
                </Col>
                <Col sm=6 xs=12 h_align=ColAlign::Center>
                    <img id="ferris" src="/res/icon/ferris-panic_transparent.svg" alt="Ferris (Rust mascot, a crab) panicked" />
                </Col>
            </Row>
        </Grid>
    }
}
