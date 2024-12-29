use leptonic::{components::prelude::*, prelude::*};
use leptos::prelude::*;

use crate::app::AppRoutes;

#[component]
pub(crate) fn PageErr404() -> impl IntoView {
    view! {
        <Grid gap=Size::Zero attr:class="err-404">
            <Row>
                <Col sm=6 xs=12 attr:class="info" h_align=ColAlign::Center>
                    <h1 attr:id="error">"404"</h1>
                    <h2 attr:id="whoops">"Whoops, this page doesn't exist :-("</h2>

                    <Link href=AppRoutes::Welcome>
                        <Button attr:id="back-btn" on_press=move |_| {} variant=ButtonVariant::Filled>
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
