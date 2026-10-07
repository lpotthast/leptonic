use indoc::indoc;
use leptos::prelude::*;

use super::demos::animation::AnimationDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageAnimationHooks() -> impl IntoView {
    view! {
        <DocPage title="Animation">
            <p>
                "Building blocks that animate elements as they appear and disappear, so that a change on screen is easy to "
                "follow instead of happening in a single frame. The animations themselves stay in your CSS; these pieces "
                "decide when they run and how long an element stays rendered for them."
            </p>
            <p>
                "The animation hooks "<Code inline=true>"use_enter_animation"</Code>" and "
                <Code inline=true>"use_exit_animation"</Code>" are documented on this page, because they are used "
                "together: they tell you when an element\u{2019}s enter animation runs and keep the element rendered until "
                "its exit animation finished. They watch the element\u{2019}s animations and transitions with the Web "
                "Animations API. The "<Link href=format!("{}#animation", routes::doc::popover::Atom.materialize())>"Popover"</Link>", "
                <Link href=format!("{}#animation", routes::doc::modal::Atom.materialize())>"Modal"</Link>" and "
                <Link href=format!("{}#styling", routes::doc::tooltip::Atom.materialize())>"Tooltip"</Link>
                " atoms use them already: they set "<Code inline=true>"data-entering"</Code>" and "
                <Code inline=true>"data-exiting"</Code>", and stay rendered while they animate out."
            </p>
            <ReactAriaSource path="utils/animation.ts"/>

            <Section title="Relationships">
                <p>
                    "Content that stays mounted and only changes between shown and hidden, such as an expandable details "
                    "panel or a sidebar that slides away, needs no hook: transition it with CSS on the attribute that holds "
                    "its state ("<Code inline=true>"aria-expanded"</Code>", an atom\u{2019}s "<Code inline=true>"data-expanded"</Code>
                    ", or one of your own), and make the hidden content "<Code inline=true>"inert"</Code>" so that it "
                    "can\u{2019}t be focused or read. Use the animation hooks when the element is added to and removed from "
                    "the DOM, as overlays, popovers and modals are: CSS alone can\u{2019}t animate an element that is already "
                    "gone."
                </p>
                <p>
                    "The two hooks split the work: "<Code inline=true>"use_exit_animation"</Code>" decides how long the "
                    "element stays rendered, so you create it next to the "<Code inline=true>"is_open"</Code>" state, "
                    "outside the "<Code inline=true>"<Show>"</Code>". "<Code inline=true>"use_enter_animation"</Code>
                    " tracks one entry and then stays done, so you create it inside, in the component that renders the "
                    "element: every opening mounts that component again and gets a new hook. Both share one "
                    <Code inline=true>"CapturedElement"</Code>"."
                </p>
            </Section>

            <Section title="Quick Start">
                <p>
                    "A panel animated with "<Code inline=true>"[data-entering]"</Code>" and "<Code inline=true>"[data-exiting]"</Code>
                    " styles (open \u{201c}View styles\u{201d} for the CSS). It stays mounted until it faded out."
                </p>
                <Demo description="Panel fading in and out, kept mounted during its exit animation" source=include_str!("demos/animation.rs") source_open=true>
                    <AnimationDemo/>
                </Demo>
            </Section>

            <Section title="use_enter_animation">
                <p>
                    "Reports whether the element\u{2019}s enter animation is running: "<Code inline=true>"is_entering"</Code>
                    " is true from the moment the element is rendered and ready until its animations finished. Until "
                    <Code inline=true>"is_ready"</Code>" is true, the element is hidden with styles that don\u{2019}t affect "
                    "layout, so it doesn\u{2019}t flash, e.g. a popover before its position is calculated. Transitions that "
                    "started before it was ready are cancelled and run again with the entry."
                </p>
                <p>
                    "The hook tracks a single entry: once "<Code inline=true>"is_entering"</Code>" turned false, it stays "
                    "false. Create the hook in the component that renders the element, so that every opening creates it anew."
                </p>

                <Section title="Input" id="use-enter-animation-input">
                    <p>
                        "Pass a "<Code inline=true>"UseEnterAnimationInput"</Code>" with every field named; the Default column "
                        "gives the value for fields you don\u{2019}t need."
                    </p>
                    <ApiTable kind=ApiKind::Input of="UseEnterAnimationInput">
                        <ApiRow name="element" ty="CapturedElement">
                            "The animated element. Capture it with "<Code inline=true>"{..element.attr()}"</Code>" on that element. Required."
                        </ApiRow>
                        <ApiRow name="is_ready" ty="Signal<bool>" default="true">
                            "Whether the entry may start. The element is hidden until then."
                        </ApiRow>
                        <ApiRow name="on_enter" ty="Option<Callback<SendWrapper<web_sys::Element>>>" default="None">
                            "Called with the element when the entry starts, e.g. to start a Web Animation, which is awaited "
                            "like CSS ones."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-enter-animation-return">
                    <ApiTable kind=ApiKind::Return of="UseEnterAnimationReturn">
                        <ApiRow name="is_entering" ty="Signal<bool>">
                            "True while the element is ready and its enter animations run. Render it as a "
                            <Code inline=true>"data-entering"</Code>" attribute."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Example" id="use-enter-animation-example">
                    <Code language=Language::Rust>
                        {indoc!(r#"
                            use leptonic::{
                                hooks::{UseEnterAnimationInput, UseEnterAnimationReturn, use_enter_animation},
                                utils::CapturedElement,
                            };

                            let element = CapturedElement::new();
                            let UseEnterAnimationReturn { is_entering } =
                                use_enter_animation(UseEnterAnimationInput { element, is_ready: true.into(), on_enter: None });

                            view! {
                                <div {..element.attr()} class="panel" data-entering=move || is_entering.get().then_some("")>
                                    "Hello"
                                </div>
                            }
                        "#)}
                    </Code>
                    <Code language=Language::Css>
                        {indoc!(r"
                            .panel[data-entering] { animation: fade-in 200ms ease-out; }
                            @keyframes fade-in { from { opacity: 0; } }
                        ")}
                    </Code>
                </Section>
            </Section>

            <Section title="use_exit_animation">
                <p>
                    "When "<Code inline=true>"is_open"</Code>" turns false, the hook reports "<Code inline=true>"is_exiting"</Code>
                    " until all animations on the element finished. Keep the element rendered while it is exiting. If it "
                    "opens again during its exit animation, the exit is interrupted. An element that was never rendered "
                    "closes at once."
                </p>

                <Section title="Input" id="use-exit-animation-input">
                    <p>
                        "Pass a "<Code inline=true>"UseExitAnimationInput"</Code>" with every field named; the Default column "
                        "gives the value for fields you don\u{2019}t need."
                    </p>
                    <ApiTable kind=ApiKind::Input of="UseExitAnimationInput">
                        <ApiRow name="element" ty="CapturedElement">"The animated element. Required."</ApiRow>
                        <ApiRow name="is_open" ty="Signal<bool>">
                            "Whether the element is logically open. The exit starts when it turns false. Required."
                        </ApiRow>
                        <ApiRow name="on_exit" ty="Option<Callback<SendWrapper<web_sys::Element>>>" default="None">
                            "Called with the element when the exit starts, e.g. to start a Web Animation, which is awaited "
                            "like CSS ones."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-exit-animation-return">
                    <ApiTable kind=ApiKind::Return of="UseExitAnimationReturn">
                        <ApiRow name="is_exiting" ty="Signal<bool>">
                            "True from the moment "<Code inline=true>"is_open"</Code>" turns false until the exit animations "
                            "finished. Keep the element rendered while it is true, and render it as a "
                            <Code inline=true>"data-exiting"</Code>" attribute."
                        </ApiRow>
                        <ApiRow name="exit_state" ty="Signal<ExitState>">
                            <Code inline=true>"Open"</Code>", "<Code inline=true>"Exiting"</Code>" (keep the element in the DOM) or "
                            <Code inline=true>"Closed"</Code>" (it can be removed)."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Example" id="use-exit-animation-example">
                    <Code language=Language::Rust>
                        {indoc!(r#"
                            use leptonic::{
                                hooks::{UseExitAnimationInput, UseExitAnimationReturn, use_exit_animation},
                                utils::CapturedElement,
                            };

                            // `is_open: Signal<bool>` is your overlay's state.
                            let element = CapturedElement::new();
                            let UseExitAnimationReturn { is_exiting, .. } =
                                use_exit_animation(UseExitAnimationInput { element, is_open, on_exit: None });


                            view! {
                                <Show when=move || is_open.get() || is_exiting.get()>
                                    <div {..element.attr()} class="panel" data-exiting=move || is_exiting.get().then_some("")>
                                        "Goodbye"
                                    </div>
                                </Show>
                            }
                        "#)}
                    </Code>
                    <Code language=Language::Css>
                        {indoc!(r"
                            .panel[data-exiting] { animation: fade-out 200ms ease-in forwards; }
                            @keyframes fade-out { to { opacity: 0; } }
                        ")}
                    </Code>
                </Section>
            </Section>

            <Section title="Reduced Motion">
                <p>
                    "The hooks wait for whatever animations run, so honor "<Code inline=true>"prefers-reduced-motion"</Code>
                    " in your CSS: replace movement with a fade, or drop the animation. Without animations, the entry ends "
                    "and the element is removed at once."
                </p>
                <Code language=Language::Css>
                    {indoc!(r"
                        @media (prefers-reduced-motion: reduce) {
                            .panel[data-entering], .panel[data-exiting] { animation: none; }
                        }
                    ")}
                </Code>
            </Section>

            <Section title="Server-Side Rendering">
                <p>
                    "During server-side rendering, nothing is animated: "<Code inline=true>"is_entering"</Code>" is false, and the "
                    "exit state follows "<Code inline=true>"is_open"</Code>" directly."
                </p>
            </Section>
        </DocPage>
    }
}
