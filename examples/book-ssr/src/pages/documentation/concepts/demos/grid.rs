use leptonic::{components::prelude::*, utils::css::em};
use leptos::prelude::*;

#[component]
pub fn GridConceptDemo() -> impl IntoView {
    view! {
        <Grid gap=em(1.0)>
            <Row>
                <Col xs=6 sm=4>"Column A"</Col>
                <Col xs=6 sm=8>"Column B"</Col>
            </Row>
        </Grid>
    }
}
