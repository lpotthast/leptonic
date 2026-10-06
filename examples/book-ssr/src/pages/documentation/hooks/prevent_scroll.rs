use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::prevent_scroll::PreventScrollDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageUsePreventScroll() -> impl IntoView {
    view! {
        <DocPage title="use_prevent_scroll">
            <p>
                "The "<Code inline=true>"use_prevent_scroll"</Code>" hook stops the page from scrolling, for example while a "
                "modal is open. See the "<Link href=routes::doc::OverlayBehavior.materialize()>"Overlay Behavior overview"</Link>
                " to compare it with the other overlay building blocks."
            </p>

            <ReactAriaSource path="overlays/usePreventScroll.ts"/>

            <Section title="Input">
                <ApiTable kind=ApiKind::Input of="UsePreventScrollInput">
                    <ApiRow name="is_disabled" ty="Signal<bool>" default="false">
                        "Scrolling is prevented while this is "<Code inline=true>"false"</Code>". Derive it from your open state, "
                        "e.g. "<Code inline=true>"Signal::derive(move || !is_open.get())"</Code>"."
                    </ApiRow>
                </ApiTable>
                <p>
                    "The hook returns nothing. With "<Code inline=true>"UsePreventScrollInput::default()"</Code>
                    " it prevents scrolling for as long as the calling component is mounted."
                </p>
            </Section>

            <Section title="Example">
                <Code language=Language::Rust>
                    {indoc!(r"
                        use leptonic::hooks::*;

                        let is_modal_open = RwSignal::new(false);

                        use_prevent_scroll(UsePreventScrollInput {
                            is_disabled: Signal::derive(move || !is_modal_open.get()),
                        });
                    ")}
                </Code>
            </Section>

            <Section title="Demo">
                <p>"Turn scroll prevention on and try to scroll this page."</p>

                <Demo description="A checkbox toggling page scroll prevention" source=include_str!("demos/prevent_scroll.rs")>
                    <PreventScrollDemo/>
                </Demo>
            </Section>

            <Section title="How It Works">
                <p>
                    "In most browsers, the hook sets "<Code inline=true>"overflow: hidden"</Code>" on the document root. To keep the "
                    "layout from shifting when the scrollbar disappears, it sets "<Code inline=true>"scrollbar-gutter: stable"</Code>
                    " where supported and adds the scrollbar width as "<Code inline=true>"padding-right"</Code>" elsewhere. The previous "
                    "inline styles are restored afterwards."
                </p>

                <p>
                    "On iOS Safari, "<Code inline=true>"overflow: hidden"</Code>" isn\u{2019}t enough. There, the hook intercepts "
                    <Code inline=true>"touchmove"</Code>" outside scrollable areas (while still allowing pinch zoom and adjusting a text "
                    "selection), applies "<Code inline=true>"overscroll-behavior: contain"</Code>" so nested scroll areas don\u{2019}t "
                    "scroll the page, and focuses inputs with "<Code inline=true>"preventScroll"</Code>
                    ", scrolling them into view itself while accounting for the on-screen keyboard."
                </p>

                <Section title="Reference Counting">
                    <p>
                        "Several overlays can prevent scrolling at the same time, like a popover opened from a modal. Scrolling comes "
                        "back only after the last of them is disabled or unmounted."
                    </p>
                </Section>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::OverlayBehavior.materialize()>"Overlay Behavior overview"</Link></li>
                <li><Link href=format!("{}#use-modal-backdrop", routes::doc::modal::Hook.materialize())>"use_modal_backdrop"</Link></li>
                <li><Link href=routes::doc::popover::Hook.materialize()>"use_popover"</Link></li>
                <li><Link href=routes::doc::overlay_behavior::UseCloseOnScroll.materialize()>"use_close_on_scroll"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
