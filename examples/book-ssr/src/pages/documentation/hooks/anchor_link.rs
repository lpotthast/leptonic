use indoc::indoc;
use leptonic::atoms::link::AnchorLink;
use leptonic::components::prelude::*;
use leptonic::hooks::anchor_link::Href;
use leptonic::hooks::*;
use leptonic::ScrollBehavior;
use leptos::prelude::*;

use crate::pages::documentation::article::Article;
use crate::pages::documentation::toc::Toc;

#[component]
pub fn PageUseAnchorLink() -> impl IntoView {
    let (disabled, set_disabled) = signal(false);

    // We make links "use_press", so that optional PressResponder's higher up the component tree can react on link interactions
    // and so that a custom `on_press` handler can immediately work with the underlying link element.
    let UseAnchorLinkReturn { attrs, is_pressed } = use_anchor_link(UseAnchorLinkInput {
        href: Href::from_str(Oco::Borrowed("#my-anchor-element")).expect("valid href"),
        scroll_behavior: Some(ScrollBehavior::Smooth),
        disabled: disabled.into(),
        description: None,
        use_press_input: UsePressInput {
            // Links cannot be disabled (for now).
            disabled: false.into(),
            // We are using an <a> tag and want the custom scrolling behavior from `use_anchor_link`.
            // If we would not enforce prevention of default behavior, automatic browser scroll-jumps would occur.
            force_prevent_default: true,
            allow_propagation: false,
            on_press: Callback::new(move |_| {}),
            on_press_up: None,
            on_press_start: None,
            on_press_end: None,
        },
    });

    view! {
        <Article>
            <h1 id="use-anchor-link" class="anchor">
                "use_anchor_link"
                <AnchorLink href="#use-anchor-link" description="Direct link to article header"/>
            </h1>

            <p>"Make any element an anchor link."</p>

            <Code>
                {indoc!(r##"
                    <a
                        {..props.attrs}
                        on:keydown=props.on_key_down
                        on:click=props.on_click
                        on:pointerdown=props.on_pointer_down
                        class="leptonic-anchor-link"
                        target="_self"
                    >
                        "#"
                    </a>
                "##)}
            </Code>

            <a
                {..attrs}
                class="leptonic-anchor-link"
                target="_self"
            >
                "#"
            </a>

            <FormControl attr:style="flex-direction: row; align-items: center; gap: 0.5em;">
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
