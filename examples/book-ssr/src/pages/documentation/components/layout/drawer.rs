use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::{drawer_left::DrawerLeftDemo, drawer_right_overlay::DrawerRightOverlayDemo};
use crate::{kit::*, routes};

#[component]
pub fn PageDrawer() -> impl IntoView {
    view! {
        <DocPage title="Drawer Component">
            <p>
                "A drawer is a panel at the edge of the page, such as a side navigation, that slides in and out of view. "
                "The "<Code inline=true>"Drawer"</Code>" component renders one: it slides whenever its "
                <Code inline=true>"shown"</Code>" signal changes, towards the side given by "<Code inline=true>"side"</Code>
                ". It is only the panel: see "<AnchorLink href="#accessibility">"Accessibility"</AnchorLink>" for what it "
                "leaves to you."
            </p>

            <Demo description="Drawer on the left side, toggled with a switch" source=include_str!("demos/drawer_left.rs")>
                <DrawerLeftDemo/>
            </Demo>

            <Section title="Props">
                <ApiTable kind=ApiKind::Props of="Drawer">
                    <ApiRow name="side" ty="DrawerSide">
                        <Code inline=true>"Left"</Code>" or "<Code inline=true>"Right"</Code>
                        ": the side the drawer slides out to. Required."
                    </ApiRow>
                    <ApiRow name="shown" ty="Signal<bool>" default="true">"Whether the drawer is visible."</ApiRow>
                    <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">
                        "Additional classes and styles."
                    </ApiRow>
                    <ApiRow name="children" ty="Children">"The drawer content. Required."</ApiRow>
                </ApiTable>
            </Section>

            <Section title="Avoiding Layout Shifts">
                <p>
                    "A drawer in the normal document flow pushes the neighboring content aside when it appears. "
                    "To avoid this layout shift, position the drawer absolutely, so that it overlays your content when shown. "
                    "This works well for menus that only open on user action on small screens, where they often fill the "
                    "whole width of the viewport."
                </p>

                <Demo
                    description="Absolutely positioned drawer overlaying the content on the right side"
                    source=include_str!("demos/drawer_right_overlay.rs")
                >
                    <DrawerRightOverlayDemo/>
                </Demo>
            </Section>

            <Section title="Accessibility">
                <p>
                    "The drawer is a styled "<Code inline=true>"<div>"</Code>" without overlay behavior. It has no role or "
                    "name, doesn\u{2019}t move focus into itself or keep it there, doesn\u{2019}t close on "<Keys keys="Escape"/>
                    " and doesn\u{2019}t lock page scrolling. Its slide animation also runs when the user prefers reduced "
                    "motion. While hidden, it isn\u{2019}t displayed, so its content can\u{2019}t be focused."
                </p>
                <ul>
                    <li>
                        "For a side navigation that stays next to the content, put the drawer\u{2019}s links in a "
                        <Code inline=true>"<nav>"</Code>" with an "<Code inline=true>"aria-label"</Code>", and give the "
                        "button that toggles it "<Code inline=true>"aria-expanded"</Code>"."
                    </li>
                    <li>
                        "For a panel that covers the page, such as a menu on small screens, compose the "
                        <Link href=routes::doc::modal::Atom.materialize()>"Modal Atoms"</Link>" instead: "
                        <Code inline=true>"ModalBackdrop"</Code>", "<Code inline=true>"ModalContent"</Code>" and a "
                        <Link href=routes::doc::dialog::Atom.materialize()>"Dialog"</Link>" keep focus inside, close on "
                        <Keys keys="Escape"/>" and return focus to the button. Style the panel to slide in with their "
                        <Code inline=true>"data-entering"</Code>" and "<Code inline=true>"data-exiting"</Code>" attributes."
                    </li>
                </ul>
            </Section>

            <Section title="Styling">
                <p>
                    "While animating, the drawer carries the class "<Code inline=true>"showing"</Code>" or "
                    <Code inline=true>"hiding"</Code>", afterwards "<Code inline=true>"shown"</Code>" or "
                    <Code inline=true>"hidden"</Code>". Its "<Code inline=true>"data-side"</Code>" attribute is "
                    <Code inline=true>"left"</Code>" or "<Code inline=true>"right"</Code>
                    ". Override any of these CSS variables to adapt the drawer to your design:"
                </p>
                <CssVariables prefix="--drawer-" scss=theme_scss!("drawer")/>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Overlays.materialize()>"Overlays"</Link></li>
                <li><Link href=routes::doc::modal::Atom.materialize()>"Modal Atoms"</Link></li>
                <li><Link href=routes::doc::AppBar.materialize()>"App Bar"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
