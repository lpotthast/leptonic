use indoc::indoc;
use leptonic::atoms::link::AnchorLink;
use leptonic::components::prelude::*;
use leptonic::hooks::anchor_link::Href;
use leptonic::hooks::*;
use leptonic::ScrollBehavior;
use leptos::*;

use crate::pages::documentation::article::Article;
use crate::pages::documentation::toc::Toc;

#[component]
pub fn PageUseAnchorLink() -> impl IntoView {
    let (disabled, set_disabled) = create_signal(false);

    // We make links "use_press", so that optional PressResponder's higher up the component tree can react on link interactions
    // and so that a custom `on_press` handler can immediately work with the underlying link element.
    let UseAnchorLinkReturn { props } = use_anchor_link(
        UseAnchorLinkInput::builder()
            .href(Href::from_str(Oco::Borrowed("#my-anchor-element")).expect("valid href"))
            .scroll_behavior(Some(ScrollBehavior::Smooth))
            .disabled(disabled)
            .description(None)
            .use_press_input(
                UsePressInput::builder()
                    // Links cannot be disabled (for now).
                    .disabled(false)
                    // We are using an <a> tag and want the custom scrolling behavior from `use_anchor_link`.
                    // If we would not enforce prevention of default behavior, automatic browser scroll-jumps would occur.
                    .force_prevent_default(true)
                    .on_press(move |_| {})
                    .build(),
            )
            .build(),
    );

    view! {
        <Article>
            <H1 id="use-anchor-link" class="anchor">
                "use_anchor_link"
                <AnchorLink href="#use-anchor-link" description="Direct link to article header"/>
            </H1>

            <P>"Make any element an anchor link."</P>

            <Code>
                {indoc!(r##"
                    <a
                        use:attrs=props.attrs
                        use:handlers=props.handlers
                        class="leptonic-anchor-link"
                        target="_self"
                    >
                        "#"
                    </a>
                "##)}
            </Code>

            <a
                {..props.attrs}
                {..props.handlers}
                class="leptonic-anchor-link"
                target="_self"
            >
                "#"
            </a>

            <FormControl style="flex-direction: row; align-items: center; gap: 0.5em;">
                <Checkbox checked=disabled set_checked=set_disabled />
                <Label>"Disabled"</Label>
            </FormControl>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "use_anchor_link", link: "#use-anchor-link" },
            ]
        }/>
    }
}
