use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn PageGrid(cx: Scope) -> impl IntoView {
    view! { cx,
        <H2>"Grid"</H2>

        <Grid spacing=6>
            <Row>
                <Col md=3 sm=4 xs=6>
                    <Skeleton>"Item 1"</Skeleton>
                </Col>
                <Col md=3 sm=4 xs=6>
                    <Skeleton>"Item 2"</Skeleton>
                </Col>
                <Col md=3 sm=4 xs=6>
                    <Skeleton>"Item 3"</Skeleton>
                </Col>
                <Col md=3 sm=12 xs=6>
                    <Skeleton>"Item 4"</Skeleton>
                </Col>
            </Row>
            <Row>
                <Col md=8 sm=6 xs=12>
                    <Skeleton>"Item 5"</Skeleton>
                </Col>
                <Col md=4 sm=6 xs=12>
                    <Skeleton>"Item 6"</Skeleton>
                </Col>
            </Row>
        </Grid>
    }
}
