use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::animation::AnimationDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageAnimationHooks() -> impl IntoView {
    view! {
        <DocPage title="Animation Hooks">
            <p>
                "Overlays, popovers and other elements that appear and disappear often animate. The animation hooks tell you "
                "when an element\u{2019}s CSS enter animation is running, and keep an element alive until its exit animation "
                "finished, so that you can remove it from the DOM only afterwards. They watch the element\u{2019}s animations "
                "and transitions with the Web Animations API, so the animations themselves stay in your CSS."
            </p>

            <Section title="Demo">
                <p>
                    "A panel animated with "<Code inline=true>"[data-entering]"</Code>" and "<Code inline=true>"[data-exiting]"</Code>
                    " styles. It stays mounted until it faded out."
                </p>
                <Demo description="Panel fading in and out, kept mounted during its exit animation" source=include_str!("demos/animation.rs") source_open=true>
                    <AnimationDemo/>
                </Demo>
            </Section>

            <Section title="use_enter_animation">
                <p>
                    "Reports whether the element\u{2019}s enter animation is running. Transitions that were already running on the "
                    "element are cancelled when tracking starts. Tracking starts once the element is rendered and "
                    <Code inline=true>"is_ready"</Code>" is true; use it to wait for preconditions such as a calculated "
                    "popover position."
                </p>

                <Section title="Input" id="use-enter-animation-input">
                    <ApiTable kind=ApiKind::Input of="UseEnterAnimationInput">
                        <ApiRow name="element" ty="CapturedElement">
                            "The animated element. Capture it with "<Code inline=true>"{..element.attr()}"</Code>" on that element."
                        </ApiRow>
                        <ApiRow name="is_ready" ty="Signal<bool>">"Whether the enter animation may start being tracked."</ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-enter-animation-return">
                    <ApiTable kind=ApiKind::Return of="UseEnterAnimationReturn">
                        <ApiRow name="is_entering" ty="Signal<bool>">
                            "True while the enter animations run; false before "<Code inline=true>"is_ready"</Code>
                            " and once they finished. Typically rendered as a "<Code inline=true>"data-entering"</Code>" attribute."
                        </ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="use_exit_animation">
                <p>
                    "When "<Code inline=true>"is_open"</Code>" turns false, the hook reports "<Code inline=true>"is_exiting"</Code>
                    " until all animations on the element finished. Keep the element rendered while it is exiting. If the "
                    "element opens again during its exit animation, the exit is interrupted."
                </p>

                <Code language=Language::Rust>
                    {indoc!(r#"
                        let element = CapturedElement::new();
                        let UseExitAnimationReturn { is_exiting, .. } =
                            use_exit_animation(UseExitAnimationInput { element, is_open });

                        view! {
                            <Show when=move || is_open.get() || is_exiting.get()>
                                <div {..element.attr()} data-exiting=move || is_exiting.get().then_some("")>"..."</div>
                            </Show>
                        }
                    "#)}
                </Code>

                <Section title="Input" id="use-exit-animation-input">
                    <ApiTable kind=ApiKind::Input of="UseExitAnimationInput">
                        <ApiRow name="element" ty="CapturedElement">"The animated element."</ApiRow>
                        <ApiRow name="is_open" ty="Signal<bool>">"Whether the element is logically open."</ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-exit-animation-return">
                    <ApiTable kind=ApiKind::Return of="UseExitAnimationReturn">
                        <ApiRow name="is_exiting" ty="Signal<bool>">
                            "True while the exit animations run. Typically rendered as a "<Code inline=true>"data-exiting"</Code>" attribute."
                        </ApiRow>
                        <ApiRow name="exit_state" ty="Signal<ExitState>">
                            <Code inline=true>"Open"</Code>", "<Code inline=true>"Exiting"</Code>" (keep the element in the DOM) or "
                            <Code inline=true>"Closed"</Code>" (it can be removed)."
                        </ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Server-side rendering">
                <p>
                    "During server-side rendering, nothing is animated: "<Code inline=true>"is_entering"</Code>" is false, and the "
                    "exit state follows "<Code inline=true>"is_open"</Code>" directly."
                </p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Popover.materialize()>"Popover"</Link></li>
                <li><Link href=routes::doc::Modal.materialize()>"Modal"</Link></li>
                <li><Link href=routes::doc::Overlays.materialize()>"Overlays"</Link></li>
                <li><Link href=routes::doc::components::Transitions.materialize()>"Transitions"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
