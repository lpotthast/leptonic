use leptonic::{components::prelude::*, utils::css::CssDimension};
use leptos::prelude::*;

use crate::routes;

#[component]
pub(crate) fn PageErr404() -> impl IntoView {
    view! {
        <Grid gap=CssDimension::Zero attr:class="err-404">
            <Row>
                <Col sm=6 xs=12 attr:class="info" h_align=ColAlign::Center>
                    <h1 id="error">"404"</h1>
                    <h2 id="whoops">"Whoops, this page doesn't exist :-("</h2>

                    <Link href=routes::Root.materialize()>
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
