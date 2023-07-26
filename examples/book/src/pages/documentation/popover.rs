use indoc::indoc;
use leptonic::prelude::*;
use leptos::*;
use leptos_use::use_element_hover;

#[component]
pub fn PagePopover(cx: Scope) -> impl IntoView {
    let el = create_node_ref(cx);
    let is_hovered = use_element_hover(cx, el);

    view! { cx,
        <H1>"Popover"</H1>

        <P>
            "Floating information which can be shown or hidden using a signal."
        </P>

        <Code>
            {indoc!(r#"
                let el = create_node_ref(cx);
                let is_hovered = use_element_hover(cx, el);

                view! {cx,
                    <Skeleton node_ref=el animated=false width=Size::Em(10.0)>
                        <Popover show=move || is_hovered.get()>
                            "1"
                        </Popover>
                        "Hover me!"
                    </Skeleton>
                }
            "#)}
        </Code>

        <div node_ref=el style="margin-top: 3em; margin-bottom: 1em;">
            <Skeleton animated=false width=Size::Em(10.0)>
                <Popover show=move || is_hovered.get()>
                    "1"
                </Popover>
                "Hover me!"
            </Skeleton>
        </div>

        <H2>"Styling"</H2>

        <P>"You may overwrite any of the following CSS variables to meet your styling needs."</P>

        <Code>
            {indoc!(r#"
                --popover-content-background-color
            "#)}
        </Code>
    }
}
