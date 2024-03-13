use indoc::indoc;
use leptonic::{atoms::link::AnchorLink, components::prelude::*, prelude::*};
use leptos::*;

use crate::pages::documentation::{article::Article, toc::Toc};

#[component]
pub fn PageCollapsible() -> impl IntoView {
    view! {
        <Article>
            <H1 id="collapsible" class="anchor">
                "Collapsible"
                <AnchorLink href="#collapsible" description="Direct link to article header"/>
            </H1>

            <Code>
                {indoc!(r#"
                    <Collapsibles default_on_open=OnOpen::CloseOthers>
                        <Stack spacing=Size::Em(0.6)>
                            <Collapsible>
                                <CollapsibleHeader slot>"Header1"</CollapsibleHeader>
                                <CollapsibleBody class="my-body" slot>"Body1"</CollapsibleBody>
                            </Collapsible>
                            <Collapsible>
                                <CollapsibleHeader slot>"Header2"</CollapsibleHeader>
                                <CollapsibleBody slot>"Body2"</CollapsibleBody>
                            </Collapsible>
                            <Collapsible on_open=OnOpen::DoNothing >
                                <CollapsibleHeader slot>"Header3 - on_open::DoNothing"</CollapsibleHeader>
                                <CollapsibleBody slot>"Body3"</CollapsibleBody>
                            </Collapsible>
                        </Stack>
                    </Collapsibles>
                "#)}
            </Code>

            <Collapsibles default_on_open=OnOpen::CloseOthers>
                <Stack spacing=Size::Em(0.6)>
                    <Collapsible>
                        <CollapsibleHeader slot>"Header1"</CollapsibleHeader>
                        <CollapsibleBody class="my-body" slot>"Body1"</CollapsibleBody>
                    </Collapsible>
                    <Collapsible>
                        <CollapsibleHeader slot>"Header2"</CollapsibleHeader>
                        <CollapsibleBody slot>"Body2"</CollapsibleBody>
                    </Collapsible>
                    <Collapsible on_open=OnOpen::DoNothing >
                        <CollapsibleHeader slot>"Header3 - on_open::DoNothing"</CollapsibleHeader>
                        <CollapsibleBody slot>"Body3"</CollapsibleBody>
                    </Collapsible>
                </Stack>
            </Collapsibles>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "Collapsible", link: "#collapsible" },
            ]
        }/>
    }
}
