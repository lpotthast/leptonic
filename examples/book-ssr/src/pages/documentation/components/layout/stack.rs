use indoc::indoc;
use leptonic::{atoms::link::AnchorLink, components::prelude::*, prelude::*};
use leptos::*;

use crate::pages::documentation::{article::Article, toc::Toc};

#[component]
pub fn PageStack() -> impl IntoView {
    view! {
        <Article>
            <H1 id="stack" class="anchor">
                "Stack"
                <AnchorLink href="#stack" description="Direct link to article header"/>
            </H1>

            <H2 id="vertical-stacks" class="anchor">
                "Vertical stacks"
                <AnchorLink href="#vertical-stacks" description="Direct link to section: Vertical stacks"/>
            </H2>

            <P>"Use a stack to create a container displaying its list of children one after the other while spacing them out by a predefined distance."</P>

            <Code>
                {indoc!(r#"
                    <Stack spacing=Size::Em(0.6)>
                        <Skeleton animated=false>"Item 1"</Skeleton>
                        <Skeleton animated=false>"Item 2"</Skeleton>
                        <Skeleton animated=false>"Item 3"</Skeleton>
                    </Stack>
                "#)}
            </Code>

            <Stack spacing=Size::Em(0.6)>
                <Skeleton animated=false>"Item 1"</Skeleton>
                <Skeleton animated=false>"Item 2"</Skeleton>
                <Skeleton animated=false>"Item 3"</Skeleton>
            </Stack>

            <H2 id="horizontal-stacks" class="anchor">
                "Horizontal stacks"
                <AnchorLink href="#horizontal-stacks" description="Direct link to section: Horizontal stacks"/>
            </H2>

            <P>
                "A stacks default orientation is "<Code inline=true>"StackOrientation::Vertical"</Code>". "
                "You can explicitly set the orientation to be "<Code inline=true>"StackOrientation::Horizontal"</Code>" "
                "to let the stack display its children horizontally."
            </P>

            <Code>
                {indoc!(r#"
                    <Stack orientation=StackOrientation::Horizontal spacing=Size::Em(0.6)>
                        <Skeleton animated=false>"Item 1"</Skeleton>
                        <Skeleton animated=false>"Item 2"</Skeleton>
                        <Skeleton animated=false>"Item 3"</Skeleton>
                    </Stack>
                "#)}
            </Code>

            <Stack orientation=StackOrientation::Horizontal spacing=Size::Em(0.6)>
                <Skeleton animated=false>"Item 1"</Skeleton>
                <Skeleton animated=false>"Item 2"</Skeleton>
                <Skeleton animated=false>"Item 3"</Skeleton>
            </Stack>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "Stack", link: "#stack" },
                Toc::Leaf { title: "Vertical stacks", link: "#vertical-stacks" },
                Toc::Leaf { title: "Horizontal stacks", link: "#horizontal-stacks" },
            ]
        }/>
    }
}
