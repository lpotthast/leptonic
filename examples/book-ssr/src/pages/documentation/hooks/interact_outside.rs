use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::interact_outside::InteractOutsideDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageUseInteractOutside() -> impl IntoView {
    view! {
        <DocPage title="use_interact_outside">
            <p>
                "The "<Code inline=true>"use_interact_outside"</Code>" hook detects clicks and touches outside an element, "
                "for example to close a popover. See the "<Link href=routes::doc::Interactions.materialize()>"Interactions overview"</Link>
                " for domain guidance."
            </p>

            <ReactAria hook="useInteractOutside"/>

            <Section title="Input">
                <p><Code inline=true>"UseInteractOutsideInput"</Code>" has no "<Code inline=true>"Default"</Code>", so you set all three fields."</p>

                <ApiTable kind=ApiKind::Input of="UseInteractOutsideInput">
                    <ApiRow name="is_disabled" ty="Signal<bool>">"Stop detecting outside interactions while "<Code inline=true>"true"</Code>"."</ApiRow>
                    <ApiRow name="on_interact_outside_start" ty="Option<Callback<PointerEvent>>">
                        "Called on "<Code inline=true>"pointerdown"</Code>" outside the element."
                    </ApiRow>
                    <ApiRow name="on_interact_outside" ty="Option<Callback<web_sys::MouseEvent>>">
                        "Called on "<Code inline=true>"click"</Code>" outside the element, if the interaction also started outside."
                    </ApiRow>
                </ApiTable>
            </Section>

            <Section title="Return">
                <ApiTable kind=ApiKind::Return of="UseInteractOutsideReturn">
                    <ApiRow name="props" ty="UseInteractOutsideProps">
                        "Captures the element that counts as \u{201c}inside\u{201d}. Spread it with "
                        <Code inline=true>"{..props.into_attrs()}"</Code>"."
                    </ApiRow>
                </ApiTable>
            </Section>

            <Section title="Example">
                <Code language=Language::Rust>
                    {indoc!(r#"
                        let (is_open, set_is_open) = signal(true);

                        let interact_outside = use_interact_outside(UseInteractOutsideInput {
                            is_disabled: Signal::stored(false),
                            on_interact_outside_start: None,
                            on_interact_outside: Some(Callback::new(move |_| set_is_open.set(false))),
                        });

                        view! {
                            <div {..interact_outside.props.into_attrs()}>"Click outside to close"</div>
                        }
                    "#)}
                </Code>
            </Section>

            <Section title="Demo">
                <Demo description="A panel that closes on outside clicks, with an outside click counter" source=include_str!("demos/interact_outside.rs")>
                    <InteractOutsideDemo/>
                </Demo>
            </Section>

            <Section title="Behavior">
                <p>
                    "Detection has two phases. On "<Code inline=true>"pointerdown"</Code>", the hook remembers whether the interaction "
                    "started outside and calls "<Code inline=true>"on_interact_outside_start"</Code>". On "<Code inline=true>"click"</Code>
                    ", it calls "<Code inline=true>"on_interact_outside"</Code>" only if the interaction started outside too. A drag "
                    "that starts inside the element and ends outside, like selecting text, doesn\u{2019}t count."
                </p>
                <p>
                    "Both listeners use the capture phase, so detection works even if elements on the page stop event propagation."
                </p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Interactions.materialize()>"Interactions overview"</Link></li>
                <li><Link href=routes::doc::Overlays.materialize()>"Overlays overview"</Link></li>
                <li><Link href=routes::doc::popover::Hook.materialize()>"use_popover"</Link></li>
                <li><Link href=routes::doc::modal::Hook.materialize()>"use_modal"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
