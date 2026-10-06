use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use crate::{kit::*, routes};

#[component]
pub fn PageAtomDismissButton() -> impl IntoView {
    view! {
        <DocPage title="DismissButton">
            <p>
                "A visually hidden button that lets screen reader users dismiss an overlay. Users of mobile screen readers "
                "such as VoiceOver on iOS have no Escape key, so place one at the start and one at the end of popovers, "
                "modals and trays. See the "<Link href=routes::doc::Overlays.materialize()>"Overlays overview"</Link>
                " for the other overlay building blocks."
            </p>

            <ReactAriaSource path="overlays/DismissButton.tsx"/>

            <Section title="Props">
                <ApiTable kind=ApiKind::Props of="DismissButton">
                    <ApiRow name="on_dismiss" ty="Option<Callback<()>>" default="None">
                        "Called when the button is activated. Close the overlay here."
                    </ApiRow>
                    <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">
                        "The accessible name of the button. Without it (and without "<Code inline=true>"aria_labelledby"</Code>
                        "), the button is labeled \u{201c}Dismiss\u{201d}."
                    </ApiRow>
                    <ApiRow name="aria_labelledby" ty="Option<String>" default="None">"Elements naming the button."</ApiRow>
                </ApiTable>
            </Section>

            <Section title="Example">
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::atoms::prelude::DismissButton;

                        view! {
                            <div class="my-overlay">
                                <DismissButton on_dismiss=move |()| set_is_open.set(false)/>
                                // ... overlay content ...
                                <DismissButton on_dismiss=move |()| set_is_open.set(false)/>
                            </div>
                        }
                    "#)}
                </Code>
            </Section>

            <Section title="Accessibility">
                <p>
                    "The atom renders an empty "<Code inline=true>"<button>"</Code>" with an "<Code inline=true>"aria-label"</Code>
                    ", visually hidden with inline styles. It has "<Code inline=true>"tabindex=\"-1\""</Code>
                    ", so the Tab key skips it while screen reader navigation still reaches it."
                </p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Overlays.materialize()>"Overlays overview"</Link></li>
                <li><Link href=routes::doc::overlays::UseOverlay.materialize()>"Overlay hooks"</Link></li>
                <li><Link href=routes::doc::Popover.materialize()>"Popover overview"</Link></li>
                <li><Link href=routes::doc::Modal.materialize()>"Modal overview"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
