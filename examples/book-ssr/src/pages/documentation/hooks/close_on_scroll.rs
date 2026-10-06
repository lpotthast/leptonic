use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::close_on_scroll::CloseOnScrollDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageUseCloseOnScroll() -> impl IntoView {
    view! {
        <DocPage title="use_close_on_scroll">
            <p>
                "The "<Code inline=true>"use_close_on_scroll"</Code>" hook closes an overlay when the page or another scrollable "
                "ancestor of its trigger scrolls, so the overlay doesn\u{2019}t stay behind when its trigger scrolls away. See the "
                <Link href=routes::doc::OverlayBehavior.materialize()>"Overlay Behavior overview"</Link>
                " for the other overlay building blocks."
            </p>

            <ReactAriaSource path="overlays/useCloseOnScroll.ts"/>

            <Section title="Input">
                <p><Code inline=true>"UseCloseOnScrollInput"</Code>" has no "<Code inline=true>"Default"</Code>"; all fields are required."</p>
                <ApiTable kind=ApiKind::Input of="UseCloseOnScrollInput">
                    <ApiRow name="is_open" ty="Signal<bool>">"Whether the overlay is open. The hook listens only while it is. Required."</ApiRow>
                    <ApiRow name="trigger_element" ty="CapturedElement">"The trigger whose scrollable ancestors are watched. Required."</ApiRow>
                    <ApiRow name="on_close" ty="Callback<()>">"Called when the page or an ancestor of the trigger scrolls. Required."</ApiRow>
                </ApiTable>
                <p>"The hook returns nothing."</p>
            </Section>

            <Section title="Example">
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::{hooks::*, utils::CapturedElement};

                        let trigger = CapturedElement::new();
                        let (is_open, set_is_open) = signal(false);

                        use_close_on_scroll(UseCloseOnScrollInput {
                            is_open: is_open.into(),
                            trigger_element: trigger,
                            on_close: Callback::new(move |()| set_is_open.set(false)),
                        });

                        view! { <span {..trigger.attr()}>"…"</span> }
                    "#)}
                </Code>
            </Section>

            <Section title="Demo">
                <p>"Show the details, then scroll the list (or the page) that holds the button."</p>
                <Demo
                    description="Details that close when the list holding their button scrolls"
                    source=include_str!("demos/close_on_scroll.rs")
                >
                    <CloseOnScrollDemo/>
                </Demo>
            </Section>

            <Section title="Behavior">
                <p>
                    "Scroll events don\u{2019}t bubble, so the hook listens in the capture phase on the window and on every shadow "
                    "root around the trigger. Scrolling elsewhere on the page (a region that doesn\u{2019}t contain the trigger) "
                    "and scrolling inside inputs and text areas, whose cursor can scroll them, is ignored."
                </p>
                <p>
                    <Link href=routes::doc::overlay_behavior::UseOverlayPosition.materialize()>"use_overlay_position"</Link>
                    " calls this hook for its "<Code inline=true>"on_close"</Code>", so a positioned overlay doesn\u{2019}t need it "
                    "separately. Non-modal popovers close this way; modal overlays prevent scrolling with "
                    <Link href=routes::doc::overlay_behavior::UsePreventScroll.materialize()>"use_prevent_scroll"</Link>" instead."
                </p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::OverlayBehavior.materialize()>"Overlay Behavior overview"</Link></li>
                <li><Link href=routes::doc::overlay_behavior::UseOverlayPosition.materialize()>"use_overlay_position"</Link></li>
                <li><Link href=routes::doc::overlay_behavior::UsePreventScroll.materialize()>"use_prevent_scroll"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
