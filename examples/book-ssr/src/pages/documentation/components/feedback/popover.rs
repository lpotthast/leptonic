use indoc::indoc;
use leptonic::{components::prelude::*, prelude::*};
use leptos::prelude::*;

use crate::pages::documentation::{article::Article, toc::Toc};

#[component]
pub fn PagePopover() -> impl IntoView {
    view! {
        <Article>
            <h1 id="popover" class="anchor">
                "Popover"
                <AnchorLink href="#popover" description="Direct link to article header"/>
            </h1>

            <p>
                "Floating information which can be shown or hidden using a signal."
            </p>

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

            <h2 id="styling" class="anchor">
                "Styling"
                <AnchorLink href="#styling" description="Direct link to section: Styling"/>
            </h2>

            <p>"You may overwrite any of the following CSS variables to meet your styling needs."</p>

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
