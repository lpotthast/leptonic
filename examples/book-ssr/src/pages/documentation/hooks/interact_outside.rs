use indoc::indoc;
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
                " to compare it with the other interaction building blocks."
            </p>

            <ReactAriaSource path="interactions/useInteractOutside.ts"/>

            <Section title="Input">
                <p>
                    <Code inline=true>"UseInteractOutsideInput"</Code>" implements "<Code inline=true>"Default"</Code>
                    " (enabled, no callbacks), so you name only the fields you need."
                </p>

                <ApiTable kind=ApiKind::Input of="UseInteractOutsideInput">
                    <ApiRow name="is_disabled" ty="Signal<bool>" default="false">"Stop detecting outside interactions while "<Code inline=true>"true"</Code>"."</ApiRow>
                    <ApiRow name="element" ty="Option<CapturedElement>" default="None">
                        "The element interactions are outside of, when it is captured elsewhere (e.g. a group of popovers). With "
                        <Code inline=true>"None"</Code>", it is the element the returned props are spread on; with an element, "
                        "the props capture nothing."
                    </ApiRow>
                    <ApiRow name="on_interact_outside_start" ty="Option<Callback<InteractOutsideEvent>>" default="None">
                        "Called on "<Code inline=true>"pointerdown"</Code>" outside the element, with the "
                        <Code inline=true>"PointerEvent"</Code>" (which is a "<Code inline=true>"MouseEvent"</Code>")."
                    </ApiRow>
                    <ApiRow name="on_interact_outside" ty="Option<Callback<InteractOutsideEvent>>" default="None">
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
                        use leptonic::hooks::interactions::{UseInteractOutsideInput, UseInteractOutsideReturn, use_interact_outside};
                        use leptos::prelude::*;

                        let is_open = RwSignal::new(true);

                        let UseInteractOutsideReturn { props } = use_interact_outside(UseInteractOutsideInput {
                            on_interact_outside: Some(Callback::new(move |_| is_open.set(false))),
                            ..Default::default()
                        });

                        view! {
                            <div {..props.into_attrs()}>"Click outside to close"</div>
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
                <li><Link href=routes::doc::OverlayBehavior.materialize()>"Overlay Behavior overview"</Link></li>
                <li><Link href=routes::doc::popover::Hook.materialize()>"use_popover"</Link></li>
                <li><Link href=routes::doc::modal::Hook.materialize()>"Modal Hooks"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
