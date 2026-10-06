use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use crate::{kit::*, routes};

#[component]
pub fn PageUseOverlayFocusContain() -> impl IntoView {
    view! {
        <DocPage title="use_overlay_focus_contain">
            <p>
                "The "<Code inline=true>"use_overlay_focus_contain"</Code>" hook lets content of an overlay ask the overlay to "
                "keep focus inside, for example a dialog in a non-modal popover. See the "
                <Link href=routes::doc::OverlayBehavior.materialize()>"Overlay Behavior overview"</Link>
                " for the other overlay building blocks."
            </p>

            <ReactAriaSource path="overlays/Overlay.tsx"/>

            <Section title="Usage">
                <p>
                    "The hook takes no input and returns nothing. Called inside an overlay that provides an "
                    <Code inline=true>"OverlayFocusContain"</Code>" context, it switches the overlay\u{2019}s focus containment on "
                    "once the calling component is mounted. Outside such an overlay it does nothing."
                </p>
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::hooks::use_overlay_focus_contain;

                        /// A panel that keeps keyboard focus inside the overlay it is rendered in.
                        #[component]
                        fn ColorPanel(children: Children) -> impl IntoView {
                            use_overlay_focus_contain();
                            view! { <div role="dialog" aria-label="Colors">{children()}</div> }
                        }
                    "#)}
                </Code>
                <p>
                    "The "<Link href=routes::doc::dialog::Hook.materialize()>"use_dialog"</Link>" hook calls it, so a "
                    <Code inline=true>"Dialog"</Code>" inside a non-modal "<Link href=routes::doc::popover::Atom.materialize()>"Popover"</Link>
                    " keeps focus inside the popover, while the popover alone would let it leave."
                </p>
            </Section>

            <Section title="OverlayFocusContain">
                <p>
                    "An overlay whose focus scope doesn\u{2019}t always contain focus provides this context to its content, as "
                    "the "<Code inline=true>"Popover"</Code>" atom does. The "
                    <Link href=routes::doc::modal::Atom.materialize()>"ModalBackdrop"</Link>" atom provides a fresh one, so "
                    "that a dialog in a modal doesn\u{2019}t switch on the containment of an overlay around the modal. Overlays "
                    "you build yourself provide it like this:"
                </p>
                <Code language=Language::Rust>
                    {indoc!(r"
                        use leptonic::{atoms::prelude::FocusScope, hooks::OverlayFocusContain};
                        use leptos::context::Provider;

                        // Per opening: containment requested by one opening doesn't carry over to the next.
                        let overlay = OverlayFocusContain::new();

                        view! {
                            <FocusScope contain=overlay.contain() restore_focus=true>
                                <Provider value=overlay>{children()}</Provider>
                            </FocusScope>
                        }
                    ")}
                </Code>
                <DocTable headers=&["Method", "Purpose"]>
                    <TableRow>
                        <TableCell><Code inline=true>"OverlayFocusContain::new()"</Code></TableCell>
                        <TableCell>"A context without containment requested (also its "<Code inline=true>"Default"</Code>")."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"contain() -> Signal<bool>"</Code></TableCell>
                        <TableCell>"Whether content of the overlay asked for focus containment."</TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::OverlayBehavior.materialize()>"Overlay Behavior overview"</Link></li>
                <li><Link href=routes::doc::focus::FocusScope.materialize()>"FocusScope"</Link></li>
                <li><Link href=routes::doc::dialog::Hook.materialize()>"use_dialog"</Link></li>
                <li><Link href=routes::doc::popover::Atom.materialize()>"Popover Atoms"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
