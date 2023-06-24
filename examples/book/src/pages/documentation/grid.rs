use indoc::indoc;
use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn PageGrid(cx: Scope) -> impl IntoView {
    view! { cx,
        <H1>"Grid"</H1>

        <Code>
            {indoc!(r#"
                <Grid spacing=6>
                    <Row>
                        <Col md=3 sm=4 xs=6>
                            <Skeleton animated=false>"Item 1"</Skeleton>
                        </Col>
                        <Col md=3 sm=4 xs=6>
                            <Skeleton animated=false>"Item 2"</Skeleton>
                        </Col>
                        <Col md=3 sm=4 xs=6>
                            <Skeleton animated=false>"Item 3"</Skeleton>
                        </Col>
                        <Col md=3 sm=12 xs=6>
                            <Skeleton animated=false>"Item 4"</Skeleton>
                        </Col>
                    </Row>
                    <Row>
                        <Col md=8 sm=6 xs=12>
                            <Skeleton animated=false>"Item 5"</Skeleton>
                        </Col>
                        <Col md=4 sm=6 xs=12>
                            <Skeleton animated=false>"Item 6"</Skeleton>
                        </Col>
                    </Row>
                </Grid>
            "#)}
        </Code>

        <Grid spacing=6>
            <Row>
                <Col md=3 sm=4 xs=6>
                    <Skeleton animated=false>"Item 1"</Skeleton>
                </Col>
                <Col md=3 sm=4 xs=6>
                    <Skeleton animated=false>"Item 2"</Skeleton>
                </Col>
                <Col md=3 sm=4 xs=6>
                    <Skeleton animated=false>"Item 3"</Skeleton>
                </Col>
                <Col md=3 sm=12 xs=6>
                    <Skeleton animated=false>"Item 4"</Skeleton>
                </Col>
            </Row>
            <Row>
                <Col md=8 sm=6 xs=12>
                    <Skeleton animated=false>"Item 5"</Skeleton>
                </Col>
                <Col md=4 sm=6 xs=12>
                    <Skeleton animated=false>"Item 6"</Skeleton>
                </Col>
            </Row>
        </Grid>
    }
}
