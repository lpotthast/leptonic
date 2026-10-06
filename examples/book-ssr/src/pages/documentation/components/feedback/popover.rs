use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::popover_hover::PopoverHoverDemo;
use crate::{kit::*, routes};

#[component]
pub fn PagePopover() -> impl IntoView {
    view! {
        <DocPage title="Popover component">
            <p>
                "The themed "<Code inline=true>"Popover"</Code>" shows content in an overlay positioned next to a "
                "trigger element. It bundles the popover, trigger, dialog and dismiss button atoms into a single "
                "component with ARIA dialog semantics. See the "
                <Link href=routes::doc::Popover.materialize()>"Popover overview"</Link>" for concept guidance."
            </p>

            <p>
                "In its simplest form, the popover manages its open state itself. A "<Code inline=true>"Button"</Code>
                " in the trigger slot toggles it automatically. Wrap other trigger content in a "
                <Code inline=true>"Pressable"</Code>" to make it toggle the popover."
            </p>

            <Code language=Language::Rust>
                {indoc!(r#"
                    <Popover placement_y=PlacementY::Below>
                        <PopoverTrigger slot>
                            <Button on_press=|_| {}>"Open"</Button>
                        </PopoverTrigger>
                        "Popover content"
                    </Popover>
                "#)}
            </Code>

            <Section title="Props">
                <ApiTable kind=ApiKind::Props of="components::popover::Popover">
                    <ApiRow name="popover_trigger" ty="PopoverTrigger">
                        "The trigger slot. The popover is positioned relative to it. Takes "
                        <Code inline=true>"classes"</Code>", "<Code inline=true>"styles"</Code>" and children. Required."
                    </ApiRow>
                    <ApiRow name="state" ty="Option<OverlayTriggerState>" default="None">
                        "The open state as app state ("<Code inline=true>"state=rw_signal"</Code>"), see "
                        <a href="#controlled-state">"Controlled State"</a>". "<Keys keys="Escape"/>", an outside interaction "
                        "or a dismiss button set it to "<Code inline=true>"false"</Code>"."
                    </ApiRow>
                    <ApiRow name="placement_x" ty="Signal<PlacementX>" default="Center">
                        "Horizontal placement relative to the trigger. Logical placements ("
                        <Code inline=true>"Start"</Code>", "<Code inline=true>"End"</Code>
                        ", ...) follow the writing direction of the current locale."
                    </ApiRow>
                    <ApiRow name="placement_y" ty="Signal<PlacementY>" default="Above">
                        "Vertical placement relative to the trigger."
                    </ApiRow>
                    <ApiRow name="modality" ty="PopoverModality" default="NonModal">
                        "Whether users can still interact with the rest of the page. A "<Code inline=true>"Modal"</Code>
                        " popover keeps focus inside, makes the rest of the page inert and closes on a click outside; a "
                        <Code inline=true>"NonModal"</Code>" one closes when focus leaves it or the page scrolls."
                    </ApiRow>
                    <ApiRow name="is_keyboard_dismiss_disabled" ty="bool" default="false">
                        "Whether pressing "<Keys keys="Escape"/>" no longer closes the popover."
                    </ApiRow>
                    <ApiRow name="should_close_on_interact_outside"
                        ty="Option<Callback<web_sys::Element, bool>>"
                        default="None"
                    >
                        "Decides for the element interacted with whether the outside interaction closes the popover."
                    </ApiRow>
                    <ApiRow name="role" ty="DialogRole" default="Dialog">
                        <Code inline=true>"Dialog"</Code>" or "<Code inline=true>"AlertDialog"</Code>"."
                    </ApiRow>
                    <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">"Accessible label of the dialog."</ApiRow>
                    <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">
                        "Additional classes and styles of the popover panel."
                    </ApiRow>
                    <ApiRow name="children" ty="ChildrenFn">"The popover content."</ApiRow>
                </ApiTable>
            </Section>

            <Section title="Controlled State">
                <p>
                    "Bind the open state to app state with "<Code inline=true>"state=(show, set_show)"</Code>" (or an "
                    <Code inline=true>"RwSignal<bool>"</Code>") to open and close the popover yourself; dismissing it sets "
                    "the state back to "<Code inline=true>"false"</Code>". This "
                    "example opens the popover while the pointer hovers a "<Code inline=true>"Hoverable"</Code>
                    " trigger. Hover alone isn\u{2019}t reachable by keyboard; for hints on hover and focus, use a "
                    <Link href=routes::doc::Tooltip.materialize()>"Tooltip"</Link>"."
                </p>

                <Demo description="Popover shown while hovering its trigger" source=include_str!("demos/popover_hover.rs")>
                    <PopoverHoverDemo/>
                </Demo>
            </Section>

            <Section title="Styling">
                <p>"Override any of these CSS variables to adapt popovers to your design:"</p>
                <CssVariables prefix="--popover-" scss=theme_scss!("popover")/>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Popover.materialize()>"Popover overview"</Link></li>
                <li><Link href=routes::doc::popover::Atom.materialize()>"Popover atoms"</Link></li>
                <li><Link href=routes::doc::popover::Hook.materialize()>"use_popover"</Link></li>
                <li><Link href=routes::doc::modal::Component.materialize()>"Modal component"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
