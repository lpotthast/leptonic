use indoc::indoc;
use leptonic::atoms::link::AnchorLink;
use leptonic::components::prelude::*;
use leptonic::hooks::*;
use leptos::*;
use ringbuf::{HeapRb, Rb};

use crate::pages::documentation::article::Article;
use crate::pages::documentation::toc::Toc;

#[component]
pub fn PageUseInteractOutside() -> impl IntoView {
    let (events, set_events) = create_signal(HeapRb::<Oco<'static, str>>::new(50));
    let (disabled, set_disabled) = create_signal(false);

    let string = create_memo(move |_| {
        events.with(|events| {
            let mut result = String::new();
            for e in events.iter().rev() {
                result.push_str(e.as_str());
                result.push_str("\n");
            }
            result
        })
    });

    let el = create_node_ref();
    let interact_outside = use_interact_outside(
        el,
        UseInteractOutsideInput::builder()
            .disabled(disabled)
            .on_interact_outside_start(move |e| {
                set_events.update(|events| {
                    events.push_overwrite(Oco::Owned(format!("{e:?}")));
                });
            })
            .on_interact_outside(move |e| {
                set_events.update(|events| {
                    events.push_overwrite(Oco::Owned(format!("{e:?}")));
                });
            })
            .build(),
    );

    view! {
        <Article>
            <H1 id="use-interact-outside" class="anchor">
                "use_interact_outside"
                <AnchorLink href="#use-interact-outside" description="Direct link to section: use_interact_outside"/>
            </H1>

            <P>"Track interactions outside a certain element tree."</P>

            <Code>
                {indoc!(r"
                    ...
                ")}
            </Code>

            <div
                {..interact_outside.props.attrs}
                {..interact_outside.props.handlers}
                ref=el
                style="display: inline-flex;
                border: 0.1em solid green;
                padding: 0.5em 1em;"
            >
                "Interact outside of me"
            </div>

            <FormControl style="flex-direction: row; align-items: center; gap: 0.5em;">
                <Checkbox checked=disabled set_checked=set_disabled />
                <Label>"Disabled"</Label>
            </FormControl>

            <P>"Last " { move || events.with(|events| events.len()) } " events: "</P>

            <pre style="
                width: 100%;
                height: 15em;
                overflow: auto;
                padding: var(--typography-code-padding);
                border: none;
                border-radius: var(--typography-code-border-radius);
                background-color: var(--typography-code-background-color);
                color: var(--typography-code-color);
            ">
                { move || string.get() }
            </pre>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "use_interact_outside", link: "#use-interact-outside" },
            ]
        }/>
    }
}
