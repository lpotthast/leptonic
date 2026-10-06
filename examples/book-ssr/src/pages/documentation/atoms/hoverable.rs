use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::hoverable::HoverableDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageAtomHoverable() -> impl IntoView {
    view! {
        <DocPage title="Hoverable">
            <p>
                "The "<Code inline=true>"Hoverable"</Code>" atom reports when a mouse or pen starts and stops hovering its child. "
                "It renders no element of its own: it adds the pointer listeners of "
                <Link href=routes::doc::interactions::UseHover.materialize()>"use_hover"</Link>" to the child. See the "
                <Link href=routes::doc::Interactions.materialize()>"Interactions overview"</Link>" for domain guidance."
            </p>

            <Section title="Hooks Used">
                <p>
                    <Link href=routes::doc::interactions::UseHover.materialize()><Code inline=true>"use_hover"</Code></Link>
                    ". Use the hook directly when you also need its "<Code inline=true>"is_hovered"</Code>
                    " signal or "<Code inline=true>"on_hover_change"</Code>"; the atom only forwards start and end."
                </p>
            </Section>

            <Section title="Props">
                <ApiTable kind=ApiKind::Props of="Hoverable">
                    <ApiRow name="is_disabled" ty="Signal<bool>" default="false">
                        "Stop reporting hover while true. If the child is hovered when this becomes true, "
                        <Code inline=true>"on_hover_end"</Code>" is called."
                    </ApiRow>
                    <ApiRow name="on_hover_start" ty="Option<Callback<HoverStartEvent>>" default="None">
                        "Called when a mouse or pen starts hovering the child, with its "<Code inline=true>"pointer_type"</Code>"."
                    </ApiRow>
                    <ApiRow name="on_hover_end" ty="Option<Callback<HoverEndEvent>>" default="None">
                        "Called when the hover ends."
                    </ApiRow>
                    <ApiRow name="children" ty="ChildrenFn">"The element to track. Must render a single element."</ApiRow>
                </ApiTable>
            </Section>

            <Section title="Example">
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::{atoms::prelude::*, hooks::HoverStartEvent};

                        let (hovered, set_hovered) = signal(false);

                        view! {
                            <Hoverable
                                on_hover_start=move |_: HoverStartEvent| set_hovered.set(true)
                                on_hover_end=move |_| set_hovered.set(false)
                            >
                                <div class:highlighted=hovered>"Hover me"</div>
                            </Hoverable>
                        }
                    "#)}
                </Code>
            </Section>

            <Section title="Demo">
                <p>
                    "Move the mouse over the box. On a touch screen, tapping doesn\u{2019}t count as hovering: browsers emulate "
                    "mouse events after a tap, which "<Code inline=true>"use_hover"</Code>" ignores."
                </p>

                <Demo description="A box highlighted while hovered, with the pointer type and a disabled switch" source=include_str!("demos/hoverable.rs")>
                    <HoverableDemo/>
                </Demo>
            </Section>

            <Section title="Styling">
                <p>
                    "The atom adds no attributes for styling. Keep the hover state in a signal from the callbacks, as the demo does, "
                    "or use CSS "<Code inline=true>":hover"</Code>" when you only need a visual effect (it also matches emulated "
                    "hover after a tap)."
                </p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::interactions::UseHover.materialize()>"use_hover"</Link></li>
                <li><Link href=routes::doc::interactions::PressResponder.materialize()>"PressResponder"</Link></li>
                <li><Link href=routes::doc::Interactions.materialize()>"Interactions overview"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
