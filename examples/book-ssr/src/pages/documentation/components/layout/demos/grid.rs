use leptonic::{components::prelude::*, utils::css::em};
use leptos::prelude::*;

#[component]
pub fn GridDemo() -> impl IntoView {
    view! {
        <Grid gap=em(0.6)>
            // Two columns per row on phones, four on desktops; on tablets, three and then a full-width one.
            <Row>
                <Col xs=6 sm=4 md=3><div class="demo-grid-item">"Item 1"</div></Col>
                <Col xs=6 sm=4 md=3><div class="demo-grid-item">"Item 2"</div></Col>
                <Col xs=6 sm=4 md=3><div class="demo-grid-item">"Item 3"</div></Col>
                <Col xs=6 sm=12 md=3><div class="demo-grid-item">"Item 4"</div></Col>
            </Row>
            // Stacked on phones; side by side, two thirds and one third, on desktops.
            <Row>
                <Col xs=12 sm=6 md=8><div class="demo-grid-item">"Item 5"</div></Col>
                <Col xs=12 sm=6 md=4><div class="demo-grid-item">"Item 6"</div></Col>
            </Row>
        </Grid>
    }
}
