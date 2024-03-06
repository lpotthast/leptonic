use indoc::indoc;
use leptonic::{components::prelude::*, prelude::*};
use leptos::*;

use crate::pages::documentation::article::Article;

#[component]
pub fn PageCollapsible() -> impl IntoView {
    view! {
        <Article>
            <H1>"Collapsibles"</H1>

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
    }
}
