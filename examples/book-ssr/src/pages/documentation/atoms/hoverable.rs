use indoc::indoc;
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
                <Link href=routes::doc::interactions::UseHover.materialize()>"use_hover"</Link>" and a "
                <Code inline=true>"data-hovered"</Code>" attribute to the child. See the "
                <Link href=routes::doc::Interactions.materialize()>"Interactions overview"</Link>" for the other interaction building blocks."
            </p>

            <Section title="Hooks Used">
                <p>
                    <Link href=routes::doc::interactions::UseHover.materialize()><Code inline=true>"use_hover"</Code></Link>
                    ". Use the hook directly when you need its "<Code inline=true>"is_hovered"</Code>
                    " signal in code or "<Code inline=true>"on_hover_change"</Code>"; the atom forwards start and end and "
                    "shows the state as "<Code inline=true>"data-hovered"</Code>"."
                </p>
            </Section>

            <Section title="Props">
                <ApiTable kind=ApiKind::Props of="Hoverable">
                    <ApiRow name="is_disabled" ty="Signal<bool>" default="false">
                        "Stop tracking hover while "<Code inline=true>"true"</Code>". If the child is hovered when this becomes "<Code inline=true>"true"</Code>", "
                        <Code inline=true>"on_hover_end"</Code>" is called."
                    </ApiRow>
                    <ApiRow name="on_hover_start" ty="Option<Callback<HoverStartEvent>>" default="None">
                        "Called when a mouse or pen starts hovering the child, with its "<Code inline=true>"pointer_type"</Code>"."
                    </ApiRow>
                    <ApiRow name="on_hover_end" ty="Option<Callback<HoverEndEvent>>" default="None">
                        "Called when the hover ends."
                    </ApiRow>
                    <ApiRow name="children" ty="ChildrenFn">"Required. The element to track. Must render a single element."</ApiRow>
                </ApiTable>
            </Section>

            <Section title="Example">
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::{atoms::prelude::*, hooks::HoverStartEvent};
                        use leptos::{logging::log, prelude::*};

                        // Style the hovered card with `.card[data-hovered]`.
                        view! {
                            <Hoverable on_hover_start=move |e: HoverStartEvent| log!("hovered with {:?}", e.pointer_type)>
                                <div class="card">"Hover me"</div>
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

                <Demo description="A box highlighted through data-hovered, with the pointer type and a Disabled checkbox" source=include_str!("demos/hoverable.rs")>
                    <HoverableDemo/>
                </Demo>
            </Section>

            <Section title="Data Attributes">
                <ApiTable kind=ApiKind::DataAttributes>
                    <ApiRow name="data-hovered">"Set on the child while a mouse or pen hovers it."</ApiRow>
                </ApiTable>
            </Section>

            <Section title="Styling">
                <p>
                    "Style the hovered child through "<Code inline=true>"[data-hovered]"</Code>". Unlike CSS "
                    <Code inline=true>":hover"</Code>", it doesn\u{2019}t stick after a tap on a touch screen."
                </p>

                <Code language=Language::Css>
                    {indoc!(r"
                        .card { border: 1px solid var(--border); transition: border-color 0.15s; }
                        .card[data-hovered] { border-color: var(--accent); }
                        @media (prefers-reduced-motion: reduce) { .card { transition: none; } }
                    ")}
                </Code>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Interactions.materialize()>"Interactions overview"</Link></li>
                <li><Link href=routes::doc::interactions::UseHover.materialize()>"use_hover"</Link></li>
                <li><Link href=routes::doc::interactions::PressResponder.materialize()>"PressResponder"</Link></li>
                <li><Link href=routes::doc::focus::FocusRing.materialize()>"FocusRing"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
