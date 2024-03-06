use indoc::indoc;
use leptonic::{components::prelude::*, prelude::*};
use leptos::*;

use crate::pages::documentation::article::Article;

#[component]
pub fn PageGrid() -> impl IntoView {
    view! {
        <Article>
            <H1>"Grid"</H1>

            <Code>
                {indoc!(r#"
                    <Grid spacing=Size::Em(0.6)>
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

            <Grid gap=Size::Em(0.6)>
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
        </Article>
    }
}
