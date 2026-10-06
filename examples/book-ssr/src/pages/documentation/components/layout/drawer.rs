use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::{drawer_left::DrawerLeftDemo, drawer_right_overlay::DrawerRightOverlayDemo};
use crate::{kit::*, routes};

#[component]
pub fn PageDrawer() -> impl IntoView {
    view! {
        <DocPage title="Drawer">
            <p>
                "The "<Code inline=true>"Drawer"</Code>" component is a side panel, typically used as a side menu. "
                "It slides in and out of view whenever its "<Code inline=true>"shown"</Code>" signal changes. "
                "The required "<Code inline=true>"side"</Code>" prop decides to which side it moves when hiding."
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
                    <ApiRow name="children" ty="Children">"The drawer content."</ApiRow>
                </ApiTable>
            </Section>

            <Section title="Avoiding Layout Shifts">
                <p>
                    "A drawer in the normal document flow pushes the neighboring content aside when it appears. "
                    "To avoid this layout shift, position the drawer absolutely, so that it overlays your content when shown. "
                    "This works well for menus that only open on user action on small screens, where they often fill the "
                    "whole width of the viewport. When you view this documentation on a small device, its main and "
                    "documentation menus work this way."
                </p>

                <Demo
                    description="Absolutely positioned drawer overlaying the content on the right side"
                    source=include_str!("demos/drawer_right_overlay.rs")
                >
                    <DrawerRightOverlayDemo/>
                </Demo>
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
                <li><Link href=routes::doc::components::AppBar.materialize()>"App Bar"</Link></li>
                <li><Link href=routes::doc::components::Stack.materialize()>"Stack"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
