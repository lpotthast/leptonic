use indoc::indoc;
use leptonic::atoms::link::AnchorLink;
use leptonic::components::prelude::*;
use leptonic::{hooks::*, Size};
use leptos::*;

use crate::pages::documentation::article::Article;
use crate::pages::documentation::toc::Toc;

#[component]
pub fn PageUsePreventScroll() -> impl IntoView {
    let (disabled, set_disabled) = create_signal(false);

    let el = create_node_ref();
    let prevent_scroll = use_prevent_scroll(
    Option::<NodeRef<leptos::html::Pre>>::Some(el),
        UsePreventScrollInput::builder()
            .disabled(disabled)
            .build(),
    );

    view! {
        <Article>
            <H1 id="use-prevent-scroll" class="anchor">
                "use_prevent_scroll"
                <AnchorLink href="#use-prevent-scroll" description="Direct link to section: use_prevent_scroll"/>
            </H1>

            <P>"Prevent scrolling."</P>

            <Code>
                {indoc!(r"
                    ...
                ")}
            </Code>

            <FormControl style="flex-direction: row; align-items: center; gap: 0.5em;">
                <Checkbox checked=disabled set_checked=set_disabled />
                <Label>"Disabled"</Label>
            </FormControl>

            <pre
                {..prevent_scroll.props.attrs}
                ref=el
                style="
                    width: 100%;
                    height: 15em;
                    overflow: auto;
                    padding: var(--typography-code-padding);
                    border: none;
                    border-radius: var(--typography-code-border-radius);
                    background-color: var(--typography-code-background-color);
                    color: var(--typography-code-color);
                "
            >
                "Lorem ipsum dolor sit amet, consetetur sadipscing elitr, "<br/>
                "sed diam nonumy eirmod tempor invidunt ut labore et dolore "<br/>
                "magna aliquyam erat, sed diam voluptua. At vero eos et accusam "<br/>
                "et justo duo dolores et ea rebum. Stet clita kasd gubergren, "<br/>
                "no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem "<br/>
                "ipsum dolor sit amet, consetetur sadipscing elitr, sed diam "<br/>
                "nonumy eirmod tempor invidunt ut labore et dolore magna "<br/>
                "aliquyam erat, sed diam voluptua. At vero eos et accusam et "<br/>
                "justo duo dolores et ea rebum. Stet clita kasd gubergren, "<br/>
                "no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem "<br/>
                "ipsum dolor sit amet, consetetur sadipscing elitr, sed diam "<br/>
                "nonumy eirmod tempor invidunt ut labore et dolore magna "<br/>
                "aliquyam erat, sed diam voluptua. At vero eos et accusam et "<br/>
                "justo duo dolores et ea rebum. Stet clita kasd gubergren, "<br/>
                "no sea takimata sanctus est Lorem ipsum dolor sit amet."<br/>
            </pre>

            <Skeleton animated=false height=Size::Em(50.0)/>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "use_prevent_scroll", link: "#use-prevent-scroll" },
            ]
        }/>
    }
}
