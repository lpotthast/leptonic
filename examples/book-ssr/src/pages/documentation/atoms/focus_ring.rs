use indoc::indoc;
use leptos::prelude::*;

use super::demos::focus_ring::FocusRingDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageAtomFocusRing() -> impl IntoView {
    view! {
        <DocPage title="FocusRing">
            <p>
                "The "<Code inline=true>"FocusRing"</Code>" atom marks its child with "<Code inline=true>"data-focused"</Code>
                " while it has focus and "<Code inline=true>"data-focus-visible"</Code>" while it has keyboard focus, so you "
                "can style focus rings in CSS. It renders no element of its own. See the "
                <Link href=routes::doc::Focus.materialize()>"Focus overview"</Link>" for the other focus building blocks."
            </p>

            <ReactAria hook="FocusRing"/>

            <Section title="Hooks Used">
                <p>
                    <Link href=routes::doc::focus::UseFocusRing.materialize()><Code inline=true>"use_focus_ring"</Code></Link>
                    ". The atom adds the hook\u{2019}s listeners and both data attributes to its child."
                </p>
            </Section>

            <Section title="Props">
                <ApiTable kind=ApiKind::Props of="FocusRing">
                    <ApiRow name="is_disabled" ty="Signal<bool>" default="false">
                        "Ignore focus events while true. Disabling it while the child is focused removes both data attributes."
                    </ApiRow>
                    <ApiRow name="within" ty="bool" default="false">
                        "Track focus anywhere inside the child instead of on the child itself."
                    </ApiRow>
                    <ApiRow name="is_text_input" ty="bool" default="false">
                        "Set this when the child behaves like a text input without being one (e.g. a date segment): typing "
                        "in it doesn\u{2019}t make focus visible, only "<Keys keys="Tab"/>" and "<Keys keys="Escape"/>
                        " do. Real text inputs follow this rule anyway, see "
                        <Link href=format!("{}#text-input-mode", routes::doc::focus::UseFocusRing.materialize())>"use_focus_ring"</Link>"."
                    </ApiRow>
                    <ApiRow name="auto_focus" ty="bool" default="false">
                        "Set this when the child is focused on mount: the ring then starts out visible. It does not focus the child."
                    </ApiRow>
                    <ApiRow name="on_focus, on_blur" ty="Option<Callback<FocusEvent>>" default="None">
                        "Called when the child receives or loses focus."
                    </ApiRow>
                    <ApiRow name="on_focus_change" ty="Option<Callback<bool>>" default="None">
                        "Called with the new focus state."
                    </ApiRow>
                    <ApiRow name="children" ty="Children">"The element to track. Must render a single element. Required."</ApiRow>
                </ApiTable>
            </Section>

            <Section title="Example">
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::atoms::prelude::FocusRing;

                        view! {
                            <FocusRing>
                                <button class="my-button">"Click or Tab to me"</button>
                            </FocusRing>
                        }
                    "#)}
                </Code>
            </Section>

            <Section title="Demo">
                <p>
                    "Click the button: no ring appears. Then tab away and back: now the ring shows, and the button reads the "
                    <AnchorLink href="#focusringcontext">"FocusRingContext"</AnchorLink>" to show a keyboard hint."
                </p>

                <Demo description="Button showing a focus ring and a keyboard hint only for keyboard focus, with a disabled toggle" source=include_str!("demos/focus_ring.rs")>
                    <FocusRingDemo/>
                </Demo>
            </Section>

            <Section title="Data Attributes">
                <ApiTable kind=ApiKind::DataAttributes>
                    <ApiRow name="data-focused" ty="true">
                        "Present while the child (or, with "<Code inline=true>"within"</Code>", something inside it) has focus, "
                        "whether by keyboard, pointer or script. Use it for styles that don\u{2019}t depend on the input device, "
                        "e.g. a highlighted border."
                    </ApiRow>
                    <ApiRow name="data-focus-visible" ty="true">
                        "Present on the child while it (or, with "<Code inline=true>"within"</Code>
                        ", one of its descendants) has keyboard focus."
                    </ApiRow>
                </ApiTable>
            </Section>

            <Section title="Styling">
                <p>"Hide the browser\u{2019}s default outline and draw your own ring for keyboard focus:"</p>

                <Code language=Language::Css>
                    {indoc!(r"
                        .my-button:focus { outline: none; }
                        .my-button[data-focused] { border-color: var(--accent); }
                        .my-button[data-focus-visible] { outline: 2px solid var(--focus); outline-offset: 2px; }
                    ")}
                </Code>
            </Section>

            <Section title="FocusRingContext">
                <p>
                    <Code inline=true>"FocusRing"</Code>" provides a "<Code inline=true>"FocusRingContext"</Code>
                    " with the "<Code inline=true>"is_focused"</Code>" and "<Code inline=true>"is_focus_visible"</Code>
                    " signals of "<Code inline=true>"use_focus_ring"</Code>". A Leptos component you render as its child can read "
                    "them with "<Code inline=true>"expect_context::<FocusRingContext>()"</Code>", like the demo\u{2019}s "
                    <Code inline=true>"SaveButton"</Code>"."
                </p>

                <ApiTable kind=ApiKind::Fields of="FocusRingContext">
                    <ApiRow name="is_focused" ty="Signal<bool>">"Whether the child (or something inside it, with "<Code inline=true>"within"</Code>") has focus."</ApiRow>
                    <ApiRow name="is_focus_visible" ty="Signal<bool>">"Whether that focus should be visible."</ApiRow>
                </ApiTable>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Focus.materialize()>"Focus overview"</Link></li>
                <li><Link href=routes::doc::focus::UseFocusRing.materialize()>"use_focus_ring"</Link></li>
                <li><Link href=routes::doc::focus::UseFocusVisible.materialize()>"use_focus_visible"</Link></li>
                <li><Link href=routes::doc::focus::Focusable.materialize()>"Focusable"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
