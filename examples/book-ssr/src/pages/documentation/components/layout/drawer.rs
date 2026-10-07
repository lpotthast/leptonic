use leptos::prelude::*;

use super::demos::{drawer_left::DrawerLeftDemo, drawer_trigger::DrawerTriggerDemo};
use crate::{kit::*, routes};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageDrawer() -> impl IntoView {
    view! {
        <DocPage title="Drawer Component">
            <p>
                "A drawer is a panel that slides in from the left or right edge of the screen and covers the page, such as "
                "a navigation menu on small screens or a set of filters. The "<Code inline=true>"Drawer"</Code>
                " component is a modal dialog: while it is open, the page behind it can\u{2019}t be used or scrolled, "
                "and "<Keys keys="Escape"/>" or a press outside closes it again."
            </p>

            <Demo description="Folder menu sliding in from the left, with a status line" source=include_str!("demos/drawer_left.rs")>
                <DrawerLeftDemo/>
            </Demo>

            <Section title="Props">
                <ApiTable kind=ApiKind::Props of="Drawer">
                    <ApiRow name="side" ty="DrawerSide" default="Left">
                        <Code inline=true>"Left"</Code>" or "<Code inline=true>"Right"</Code>
                        ": the edge of the screen the drawer slides in from."
                    </ApiRow>
                    <ApiRow name="width" ty="Option<Width>" default="None">
                        "The panel\u{2019}s width, e.g. "<Code inline=true>"rem(20.0)"</Code>", set as "
                        <Code inline=true>"--drawer-width"</Code>". Default: the theme\u{2019}s 17em, at most 85% of the "
                        "viewport\u{2019}s width."
                    </ApiRow>
                    <ApiRow name="is_open" ty="Option<Signal<bool>>" default="None">
                        "Whether the drawer is open (controlled): a value or any signal. Without it and "
                        <Code inline=true>"default_open"</Code>", the drawer takes the state of a surrounding "
                        <Code inline=true>"DialogTrigger"</Code>"."
                    </ApiRow>
                    <ApiRow name="set_open" ty="Option<Out<bool>>" default="None">
                        "Receives the open state when "<Keys keys="Escape"/>" or a press outside closes the drawer: an "
                        <Code inline=true>"RwSignal"</Code>", "<Code inline=true>"WriteSignal"</Code>", closure, "
                        <Code inline=true>"Callback"</Code>", \u{2026}"
                    </ApiRow>
                    <ApiRow name="default_open" ty="Option<bool>" default="None">
                        "Whether the drawer starts open, with its own state. Ignored with "<Code inline=true>"is_open"</Code>"."
                    </ApiRow>
                    <ApiRow name="on_open_change" ty="Option<Callback<bool>>" default="None">
                        "Called when the drawer opens or closes. Inside a "<Code inline=true>"DialogTrigger"</Code>
                        ", pass it to the trigger instead."
                    </ApiRow>
                    <ApiRow name="is_dismissable" ty="Signal<bool>" default="true">
                        "Whether a press outside the drawer closes it."
                    </ApiRow>
                    <ApiRow name="is_keyboard_dismiss_disabled" ty="Signal<bool>" default="false">
                        "Whether "<Keys keys="Escape"/>" no longer closes the drawer."
                    </ApiRow>
                    <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">
                        "Names a drawer without a "<Code inline=true>"DialogTitle"</Code>" inside (which names it)."
                    </ApiRow>
                    <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">
                        "Additional classes and styles of the dialog inside the panel ("
                        <Code inline=true>".leptonic-drawer-dialog"</Code>"), which holds the content and fills the panel."
                    </ApiRow>
                    <ApiRow name="children" ty="ChildrenFn">"The drawer content. Required."</ApiRow>
                </ApiTable>
            </Section>

            <Section title="Opening and Closing">
                <p>
                    "Pass your own state as "<Code inline=true>"is_open"</Code>" and "<Code inline=true>"set_open"</Code>
                    ", as the demo above does with one "<Code inline=true>"RwSignal"</Code>": a button sets it to open the "
                    "drawer, and your own controls inside, such as the close button or a chosen folder, reset it. "
                    <Keys keys="Escape"/>" and a press outside call "<Code inline=true>"set_open"</Code>" with "
                    <Code inline=true>"false"</Code>". Set "<Code inline=true>"is_dismissable=false"</Code>
                    " to ignore presses outside, and "<Code inline=true>"is_keyboard_dismiss_disabled=true"</Code>
                    " to ignore "<Keys keys="Escape"/>"."
                </p>
                <p>
                    "The drawer is rendered into the document body while it is open, so it isn\u{2019}t clipped by the "
                    "element you place it in. When it closes, it stays rendered until its slide-out animation ended."
                </p>

                <Section title="From a Dialog Trigger">
                    <p>
                        "Put the drawer and its button into a "
                        <Link href=format!("{}#dialogtrigger", routes::doc::dialog::Atom.materialize())>"DialogTrigger"</Link>
                        " and leave out "<Code inline=true>"is_open"</Code>": the trigger owns the open state and opens the "
                        "drawer when the button is pressed. The button gets "<Code inline=true>"aria-expanded"</Code>" and "
                        <Code inline=true>"aria-controls"</Code>", and a drawer without a title or "
                        <Code inline=true>"aria_label"</Code>" is named by the button. Give the trigger "
                        <Code inline=true>"is_open"</Code>" and "<Code inline=true>"set_open"</Code>" when your own controls "
                        "inside should close the drawer."
                    </p>
                    <Demo description="Filters in a drawer on the right, opened by a DialogTrigger" source=include_str!("demos/drawer_trigger.rs")>
                        <DrawerTriggerDemo/>
                    </Demo>
                </Section>
            </Section>

            <Section title="Accessibility">
                <p>
                    "The drawer is a modal "<Link href=routes::doc::Dialog.materialize()>"dialog"</Link>" ("
                    <Code inline=true>"role=\"dialog\""</Code>", "<Code inline=true>"aria-modal"</Code>"), built on the "
                    <Link href=routes::doc::modal::Atom.materialize()>"Modal Atoms"</Link>":"
                </p>
                <ul>
                    <li>
                        "Name it with a "<Code inline=true>"DialogTitle"</Code>" inside or with "
                        <Code inline=true>"aria_label"</Code>", so that screen readers announce what opened."
                    </li>
                    <li>
                        "When it opens, focus moves into it and stays there: "<Keys keys="Tab"/>" and "
                        <Keys keys="Shift + Tab"/>" cycle through its elements. When it closes, focus returns to the element "
                        "that had it before, usually the button that opened it."
                    </li>
                    <li>
                        "The page behind it is inert, so screen readers and the keyboard can\u{2019}t reach it, and it "
                        "doesn\u{2019}t scroll."
                    </li>
                    <li>
                        "When the user prefers reduced motion, the drawer appears and disappears without sliding."
                    </li>
                    <li>
                        "Offer a visible way to close it, such as a close button: a press outside works, but isn\u{2019}t "
                        "obvious, and there may be little room outside on a small screen."
                    </li>
                </ul>

                <KeyboardTable>
                    <KeyRow keys="Escape">"Close the drawer (unless "<Code inline=true>"is_keyboard_dismiss_disabled"</Code>")."</KeyRow>
                    <KeyRow keys="Tab / Shift + Tab">"Move focus to the next or previous element inside the drawer."</KeyRow>
                </KeyboardTable>
            </Section>

            <Section title="Data Attributes">
                <ApiTable kind=ApiKind::DataAttributes>
                    <ApiRow name="data-entering" ty="true">
                        "Set on the backdrop and on the panel while the drawer opens, until their animations finished."
                    </ApiRow>
                    <ApiRow name="data-exiting" ty="true">
                        "Set on the backdrop and on the panel while the drawer closes. They stay rendered until their "
                        "animations finished."
                    </ApiRow>
                </ApiTable>
            </Section>

            <Section title="Styling">
                <p>
                    "The drawer renders three elements: the backdrop covering the page ("
                    <Code inline=true>".leptonic-modal-backdrop.leptonic-drawer-backdrop"</Code>"), the panel at the edge ("
                    <Code inline=true>".leptonic-drawer"</Code>" with "<Code inline=true>".leptonic-drawer-left"</Code>" or "
                    <Code inline=true>".leptonic-drawer-right"</Code>") and the dialog filling it ("
                    <Code inline=true>".leptonic-drawer-dialog"</Code>", which takes "<Code inline=true>"classes"</Code>
                    " and "<Code inline=true>"styles"</Code>"). The theme gives the panel a width of 17em, at most 85% of the "
                    "viewport width, fades the backdrop and slides the panel with keyframe animations while "
                    <Code inline=true>"data-entering"</Code>" or "<Code inline=true>"data-exiting"</Code>" is set."
                </p>
                <p>
                    "The theme leaves the content unpadded, so that full-width rows such as menu entries reach the edges. "
                    "Space it through "<Code inline=true>"classes"</Code>":"
                </p>
                <Code language=Language::Css>
                    {".filters { gap: 8px; padding: 16px; }"}
                </Code>
                <p>"Override any of these CSS variables to adapt drawers to your design:"</p>
                <CssVariables prefix="--drawer-" scss=theme_scss!("drawer")/>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Overlays.materialize()>"Overlays"</Link></li>
                <li><Link href=routes::doc::Modal.materialize()>"Modal overview"</Link></li>
                <li><Link href=routes::doc::modal::Atom.materialize()>"Modal Atoms"</Link></li>
                <li><Link href=routes::doc::dialog::Atom.materialize()>"Dialog Atoms"</Link></li>
                <li><Link href=routes::doc::AppBar.materialize()>"App Bar"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
