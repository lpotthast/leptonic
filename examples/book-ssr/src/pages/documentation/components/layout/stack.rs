use indoc::indoc;
use leptonic::{components::prelude::*, prelude::*};
use leptos::prelude::*;

use crate::pages::documentation::{article::Article, toc::Toc};

#[component]
pub fn PageStack() -> impl IntoView {
    view! {
        <Article>
            <h1 id="stack" class="anchor">
                "Stack"
                <AnchorLink href="#stack" description="Direct link to article header"/>
            </h1>

            <h2 id="vertical-stacks" class="anchor">
                "Vertical stacks"
                <AnchorLink href="#vertical-stacks" description="Direct link to section: Vertical stacks"/>
            </h2>

            <p>"Use a stack to create a container displaying its list of children one after the other while spacing them out by a predefined distance."</p>

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

            <h2 id="horizontal-stacks" class="anchor">
                "Horizontal stacks"
                <AnchorLink href="#horizontal-stacks" description="Direct link to section: Horizontal stacks"/>
            </h2>

            <p>
                "A stacks default orientation is "<Code inline=true>"StackOrientation::Vertical"</Code>". "
                "You can explicitly set the orientation to be "<Code inline=true>"StackOrientation::Horizontal"</Code>" "
                "to let the stack display its children horizontally."
            </p>

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
