use indoc::indoc;
use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn PagePopover() -> impl IntoView {
    view! {
        <H1>"Popover"</H1>

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

        <H2>"Styling"</H2>

        <P>"You may overwrite any of the following CSS variables to meet your styling needs."</P>

        <Code>
            {indoc!(r"
                --popover-content-background-color
            ")}
        </Code>
    }
}
