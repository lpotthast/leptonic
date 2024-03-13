use indoc::indoc;
use leptonic::atoms::link::AnchorLink;
use leptonic::components::prelude::*;
use leptonic::prelude::*;
use leptos::*;

use crate::pages::documentation::article::Article;
use crate::pages::documentation::toc::Toc;

#[component]
pub fn PagePopover() -> impl IntoView {
    view! {
        <Article>
            <H1 id="popover" class="anchor">
                "Popover"
                <AnchorLink href="#popover" description="Direct link to article header"/>
            </H1>

            <P>
                "Floating information which can be shown or hidden using a signal."
            </P>

            <Code>
                {indoc!(r#"
                    view! {
                        <Popover>
                            <PopoverContent slot>
                                "1"
                            </PopoverContent>

                            <Skeleton animated=false width=Size::Em(10.0)>
                                "Hover me!"
                            </Skeleton>
                        </Popover>
                    }
                "#)}
            </Code>

            <div style="margin-top: 3em; margin-bottom: 1em;">
                <Popover>
                    <PopoverContent slot>
                        "1"
                    </PopoverContent>

                    <Skeleton animated=false width=Size::Em(10.0)>
                        "Hover me!"
                    </Skeleton>
                </Popover>
            </div>

            <H2 id="styling" class="anchor">
                "Styling"
                <AnchorLink href="#styling" description="Direct link to section: Styling"/>
            </H2>

            <P>"You may overwrite any of the following CSS variables to meet your styling needs."</P>

            <Code>
                {indoc!(r"
                    --popover-content-background-color
                ")}
            </Code>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "Popover", link: "#popover" },
                Toc::Leaf { title: "Styling", link: "#styling" },
            ]
        }/>
    }
}
