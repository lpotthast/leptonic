use indoc::indoc;
use leptonic::{atoms::link::AnchorLink, components::prelude::*, prelude::*};
use leptos::*;

use crate::pages::documentation::{article::Article, toc::Toc};

#[component]
pub fn PageGrid() -> impl IntoView {
    view! {
        <Article>
            <H1 id="grid" class="anchor">
                "Grid"
                <AnchorLink href="#grid" description="Direct link to article header"/>
            </H1>

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

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "Grid", link: "#grid" },
            ]
        }/>
    }
}
