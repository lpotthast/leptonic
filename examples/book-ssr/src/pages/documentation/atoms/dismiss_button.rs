use indoc::indoc;
use leptos::prelude::*;

use super::demos::dismiss_button::DismissButtonDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageAtomDismissButton() -> impl IntoView {
    view! {
        <DocPage title="DismissButton">
            <p>
                "The "<Code inline=true>"DismissButton"</Code>" atom is a visually hidden button that closes an overlay. Users "
                "of mobile screen readers such as VoiceOver on iOS have no Escape key; they find it at the start or end of the "
                "overlay. See the "<Link href=routes::doc::OverlayBehavior.materialize()>"Overlay Behavior overview"</Link>
                " for the other overlay building blocks."
            </p>

            <ReactAriaSource path="overlays/DismissButton.tsx"/>

            <p>
                "The "<Link href=routes::doc::popover::Atom.materialize()>"Popover"</Link>" atom already renders dismiss "
                "buttons: one at its end, and one at its start too when it is modal. Add them yourself to overlays you build from "
                "the hooks, such as "<Link href=routes::doc::overlay_behavior::UseOverlay.materialize()>"use_overlay"</Link>
                " or "<Link href=routes::doc::popover::Hook.materialize()>"use_popover"</Link>"."
            </p>

            <Section title="Hooks Used">
                <p>
                    "None: the atom renders a native "<Code inline=true>"<button>"</Code>" inside a "
                    <Link href=routes::doc::screen_readers::VisuallyHidden.materialize()>"VisuallyHidden"</Link>"."
                </p>
            </Section>

            <Section title="Props">
                <ApiTable kind=ApiKind::Props of="DismissButton">
                    <ApiRow name="on_dismiss" ty="Callback<()>">
                        "Called when the button is activated. Close the overlay here. Required."
                    </ApiRow>
                    <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">
                        "The button\u{2019}s name. Without it (and without "<Code inline=true>"aria_labelledby"</Code>
                        "), the button is named \u{201c}Dismiss\u{201d}."
                    </ApiRow>
                    <ApiRow name="aria_labelledby" ty="Option<String>" default="None">"The ids of the elements naming the button."</ApiRow>
                    <ApiRow name="id" ty="Option<String>" default="None">"The button\u{2019}s id. Generated when "<Code inline=true>"None"</Code>"."</ApiRow>
                </ApiTable>
            </Section>

            <Section title="Example">
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::atoms::prelude::DismissButton;

                        let close = Callback::new(move |()| set_is_open.set(false));

                        view! {
                            <div {..overlay_attrs} role="dialog" aria-label="Notifications">
                                <DismissButton on_dismiss=close/>
                                "…"
                                <DismissButton on_dismiss=close/>
                            </div>
                        }
                    "#)}
                </Code>
            </Section>

            <Section title="Demo">
                <p>
                    "The dismiss buttons are invisible. With a screen reader, move to the start or end of the open panel to find "
                    "them."
                </p>
                <Demo
                    description="A panel with dismiss buttons at its start and end"
                    source=include_str!("demos/dismiss_button.rs")
                >
                    <DismissButtonDemo/>
                </Demo>
            </Section>

            <Section title="Composition">
                <p>
                    "The button has "<Code inline=true>"tabindex=\"-1\""</Code>": the Tab key skips it, while screen reader "
                    "navigation still reaches it. It has no visible content and no data attributes, so there is nothing to style."
                </p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::OverlayBehavior.materialize()>"Overlay Behavior overview"</Link></li>
                <li><Link href=routes::doc::overlay_behavior::UseOverlay.materialize()>"use_overlay"</Link></li>
                <li><Link href=routes::doc::popover::Atom.materialize()>"Popover Atoms"</Link></li>
                <li><Link href=routes::doc::screen_readers::VisuallyHidden.materialize()>"VisuallyHidden"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
