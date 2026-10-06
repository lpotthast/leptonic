use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::hover::HoverDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageUseHover() -> impl IntoView {
    view! {
        <DocPage title="use_hover">
            <p>
                "The "<Code inline=true>"use_hover"</Code>" hook tracks whether a mouse or pen is over an element. "
                "See the "<Link href=routes::doc::Interactions.materialize()>"Interactions overview"</Link>" for domain guidance."
            </p>

            <ReactAria hook="useHover"/>

            <Section title="Input">
                <p>
                    <Code inline=true>"UseHoverInput"</Code>" implements "<Code inline=true>"Default"</Code>" (enabled, no callbacks)."
                </p>

                <ApiTable kind=ApiKind::Input of="UseHoverInput">
                    <ApiRow name="is_disabled" ty="Signal<bool>" default="false">
                        "Turns hover tracking off. If the element is hovered when this becomes "<Code inline=true>"true"</Code>
                        ", "<Code inline=true>"on_hover_end"</Code>" fires and "<Code inline=true>"is_hovered"</Code>" resets."
                    </ApiRow>
                    <ApiRow name="on_hover_start" ty="Option<Callback<HoverStartEvent>>" default="None">
                        "Called when a pointer starts hovering the element."
                    </ApiRow>
                    <ApiRow name="on_hover_end" ty="Option<Callback<HoverEndEvent>>" default="None">
                        "Called when a pointer stops hovering the element, or when "<Code inline=true>"is_disabled"</Code>
                        " becomes "<Code inline=true>"true"</Code>" while it is hovered."
                    </ApiRow>
                    <ApiRow name="on_hover_change" ty="Option<Callback<bool>>" default="None">
                        "Called whenever the hover state changes."
                    </ApiRow>
                </ApiTable>

                <p>
                    <Code inline=true>"HoverStartEvent"</Code>" and "<Code inline=true>"HoverEndEvent"</Code>" carry the "
                    <Code inline=true>"pointer_type"</Code>" and the "<Code inline=true>"current_target"</Code>" element."
                </p>
            </Section>

            <Section title="Return">
                <ApiTable kind=ApiKind::Return of="UseHoverReturn">
                    <ApiRow name="props" ty="UseHoverProps">
                        "Pointer event handlers. Spread them onto the element with "<Code inline=true>"{..props.into_attrs()}"</Code>"."
                    </ApiRow>
                    <ApiRow name="is_hovered" ty="Signal<bool>">"Whether the element is currently hovered."</ApiRow>
                </ApiTable>
            </Section>

            <Section title="Example">
                <Code language=Language::Rust>
                    {indoc!(r#"
                        let UseHoverReturn { props, is_hovered } = use_hover(UseHoverInput::default());

                        view! {
                            <div {..props.into_attrs()} class:hovered=move || is_hovered.get()>"Hover me"</div>
                        }
                    "#)}
                </Code>
            </Section>

            <Section title="Demo">
                <Demo description="Hover state and hover start/end events with a disabled toggle" source=include_str!("demos/hover.rs")>
                    <HoverDemo/>
                </Demo>
            </Section>

            <Section title="Behavior">
                <ul>
                    <li>
                        "Only mouse and pen pointers hover. Touch input never starts a hover, and the emulated mouse events "
                        "browsers fire after a touch are ignored for 500 ms, so elements don\u{2019}t get stuck in the hovered state."
                    </li>
                    <li>
                        "If the hovered element is removed from the DOM, the browser fires no "<Code inline=true>"pointerleave"</Code>
                        ". The hook notices the pointer moving over another element and ends the hover."
                    </li>
                </ul>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Interactions.materialize()>"Interactions overview"</Link></li>
                <li><Link href=routes::doc::interactions::UsePress.materialize()>"use_press"</Link></li>
                <li><Link href=routes::doc::interactions::UseKeyboard.materialize()>"use_keyboard"</Link></li>
                <li><Link href=routes::doc::button::Hook.materialize()>"use_button"</Link>" (composes use_hover)"</li>
            </SeeAlso>
        </DocPage>
    }
}
