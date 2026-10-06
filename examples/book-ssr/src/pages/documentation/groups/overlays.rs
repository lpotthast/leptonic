use leptonic::components::prelude::*;
use leptos::prelude::*;

use crate::{kit::*, routes};

#[component]
pub fn PageOverlays() -> impl IntoView {
    view! {
        <DocPage title="Overlays">
            <p>
                "Overlays show content above the page: modals that block it until the user responds, popovers anchored to "
                "the element that opened them, tooltips describing an element, and drawers sliding in from the edge. They "
                "belong together because they appear on demand, float above the rest of the page and must close again."
            </p>
            <p>
                "The behavior they share (closing on Escape or an outside interaction, positioning next to a trigger, "
                "locking page scrolling) is documented in "
                <Link href=routes::doc::OverlayBehavior.materialize()>"Overlay Behavior"</Link>
                ". Reach for it when you build an overlay none of these concepts covers."
            </p>

            <Section title="Pages">
                <SectionMembers overview=routes::doc::Overlays.materialize()/>
            </Section>

            <Section title="Relationships">
                <ul>
                    <li>
                        <Link href=routes::doc::Modal.materialize()>"Modal"</Link>" and "
                        <Link href=routes::doc::Popover.materialize()>"Popover"</Link>" render their content in a "
                        <Link href=routes::doc::focus::FocusScope.materialize()>"FocusScope"</Link>" that returns the "
                        "focus when they close. A modal blocks the whole page and keeps the focus inside; a popover is "
                        "placed next to its trigger and keeps the focus inside only when it is modal or holds a dialog."
                    </li>
                    <li>
                        <Link href=routes::doc::Dialog.materialize()>"Dialog"</Link>" is the content of a modal or a "
                        "popover that asks something of the user, such as a confirmation or a form: it gives the content the "
                        <Code inline=true>"dialog"</Code>" role and names it after its title."
                    </li>
                    <li>
                        <Link href=routes::doc::Tooltip.materialize()>"Tooltip"</Link>" is positioned like a popover, but "
                        "it opens on hover or keyboard focus, never takes the focus and holds no interactive content."
                    </li>
                    <li>
                        <Link href=routes::doc::Drawer.materialize()>"Drawer"</Link>" is a styled panel that slides in from "
                        "the left or right when a signal changes. It has no overlay behavior of its own: it doesn\u{2019}t "
                        "contain focus, close on "<Keys keys="Escape"/>" or lock scrolling. For a panel that covers the "
                        "page, compose the "<Link href=routes::doc::modal::Atom.materialize()>"Modal Atoms"</Link>" instead."
                    </li>
                    <li>
                        "Other concepts open overlays too: a "<Link href=routes::doc::Menu.materialize()>"menu"</Link>
                        ", a "<Link href=routes::doc::Select.materialize()>"select"</Link>", a "
                        <Link href=routes::doc::Combobox.materialize()>"combobox"</Link>" and a "
                        <Link href=routes::doc::DatePicker.materialize()>"date picker"</Link>" show their content in a popover."
                    </li>
                </ul>
            </Section>

            <Section title="Decision Guide">
                <DocTable headers=&["If you want to\u{2026}", "Use"]>
                    <TableRow>
                        <TableCell>"Ask for a decision or input before the user continues"</TableCell>
                        <TableCell><Link href=routes::doc::Modal.materialize()>"Modal"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Show details, a small form or a preview next to the element that opened it"</TableCell>
                        <TableCell><Link href=routes::doc::Popover.materialize()>"Popover"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Give the content of a modal or popover a title that screen readers announce"</TableCell>
                        <TableCell><Link href=routes::doc::Dialog.materialize()>"Dialog"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Describe a button or an icon in a few words, on hover or focus"</TableCell>
                        <TableCell><Link href=routes::doc::Tooltip.materialize()>"Tooltip"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Slide a side panel in and out next to the content"</TableCell>
                        <TableCell><Link href=routes::doc::Drawer.materialize()>"Drawer"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Cover the page with a panel, such as a menu on small screens"</TableCell>
                        <TableCell><Link href=routes::doc::modal::Atom.materialize()>"Modal Atoms"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Offer a list of actions from a button"</TableCell>
                        <TableCell><Link href=routes::doc::Menu.materialize()>"Menu"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Tell the user something without interrupting them"</TableCell>
                        <TableCell>
                            <Link href=routes::doc::Toast.materialize()>"Toast"</Link>" or "
                            <Link href=routes::doc::Alert.materialize()>"Alert"</Link>
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Show or hide content in place, in the flow of the page"</TableCell>
                        <TableCell><Link href=routes::doc::Disclosure.materialize()>"Disclosure"</Link></TableCell>
                    </TableRow>
                </DocTable>
            </Section>
        </DocPage>
    }
}
