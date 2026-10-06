use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::popover_controlled::PopoverControlledDemo;
use crate::{
    kit::*, pages::documentation::concepts::demos::popover::PopoverConceptDemo, routes,
};

#[component]
pub fn PagePopover() -> impl IntoView {
    view! {
        <DocPage title="Popover Component">
            <p>
                "The themed "<Code inline=true>"Popover"</Code>" opens a dialog next to the button in its trigger slot. "
                "See the "<Link href=routes::doc::Popover.materialize()>"Popover overview"</Link>" for when to use a popover."
            </p>

            <Demo description="Popover component opened by a button" source=include_str!("../../concepts/demos/popover.rs")>
                <PopoverConceptDemo/>
            </Demo>

            <Section title="Props">
                <ApiTable kind=ApiKind::Props of="components::popover::Popover">
                    <ApiRow name="popover_trigger" ty="PopoverTrigger">
                        "The trigger slot: put a "<Code inline=true>"Button"</Code>" (or a "<Code inline=true>"Pressable"</Code>
                        ") in it, which opens and closes the popover. The popover is positioned at the slot. Takes "
                        <Code inline=true>"classes"</Code>", "<Code inline=true>"styles"</Code>" and children. Required."
                    </ApiRow>
                    <ApiRow name="is_open" ty="Option<Signal<bool>>" default="None">
                        "Whether the popover is open (controlled): a value or any signal. Default: owned by the popover."
                    </ApiRow>
                    <ApiRow name="set_open" ty="Option<Out<bool>>" default="None">
                        "Receives the open state: an "<Code inline=true>"RwSignal"</Code>", "<Code inline=true>"WriteSignal"</Code>
                        ", closure, "<Code inline=true>"Callback"</Code>", \u{2026}"
                    </ApiRow>
                    <ApiRow name="placement" ty="Signal<Placement>" default="Top">
                        "Where the popover goes relative to the trigger, see "
                        <Link href=format!("{}#placements", routes::doc::overlay_behavior::UseOverlayPosition.materialize())>"Placements"</Link>"."
                    </ApiRow>
                    <ApiRow name="modality" ty="PopoverModality" default="NonModal">
                        "Whether the popover takes over the page while open, see "
                        <AnchorLink href="#modality">"Modality"</AnchorLink>"."
                    </ApiRow>
                    <ApiRow name="is_keyboard_dismiss_disabled" ty="Signal<bool>" default="false">
                        "Whether "<Keys keys="Escape"/>" no longer closes the popover."
                    </ApiRow>
                    <ApiRow name="should_close_on_interact_outside" ty="Option<InteractOutsideFilter>" default="None">
                        "Decides per element outside whether pressing it (in a modal popover) or moving focus to it closes "
                        "the popover. "<Code inline=true>"None"</Code>" closes for every element."
                    </ApiRow>
                    <ApiRow name="role" ty="DialogRole" default="Dialog">
                        <Code inline=true>"Dialog"</Code>", or "<Code inline=true>"AlertDialog"</Code>" for content that "
                        "needs an immediate response."
                    </ApiRow>
                    <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">
                        "Names the popover\u{2019}s dialog. Default: the trigger names it."
                    </ApiRow>
                    <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">
                        "Additional classes and styles of the popover panel."
                    </ApiRow>
                    <ApiRow name="children" ty="ChildrenFn">"The popover content. Required."</ApiRow>
                </ApiTable>
            </Section>

            <Section title="Controlled State">
                <p>
                    "Bind the open state with "<Code inline=true>"is_open"</Code>" and "<Code inline=true>"set_open"</Code>
                    " (e.g. both an "<Code inline=true>"RwSignal<bool>"</Code>") to close the popover from your own code, "
                    "here from its \u{201c}Done\u{201d} button. Dismissing the popover calls "<Code inline=true>"set_open"</Code>
                    " with "<Code inline=true>"false"</Code>"."
                </p>

                <Demo description="Popover with a switch and a button that closes it" source=include_str!("demos/popover_controlled.rs")>
                    <PopoverControlledDemo/>
                </Demo>
            </Section>

            <Section title="Modality">
                <p>
                    "The component is non-modal: presses outside reach the page, and moving focus out of the popover or "
                    "scrolling the page closes it. A modal popover closes on a press outside and makes the rest of the page "
                    "inert until it closes. See "
                    <Link href=format!("{}#modal-and-non-modal", routes::doc::popover::Atom.materialize())>"Modal and Non-Modal"</Link>
                    " for the details."
                </p>
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::{components::prelude::*, hooks::PopoverModality};

                        view! {
                            <Popover modality=PopoverModality::Modal>
                                <PopoverTrigger slot>
                                    <Button>"Filters"</Button>
                                </PopoverTrigger>
                                "Filter options"
                            </Popover>
                        }
                    "#)}
                </Code>
            </Section>

            <Section title="Accessibility">
                <p>
                    "The component has the accessibility of the "<Link href=routes::doc::popover::Atom.materialize()>"Popover Atoms"</Link>
                    ": a dialog named by its trigger (or "<Code inline=true>"aria_label"</Code>"), focus moved in, kept "
                    "inside and returned to the trigger, and a hidden dismiss button at its end for screen reader users. To name the dialog by a heading instead, put a "
                    <Link href=format!("{}#dialogtitle", routes::doc::dialog::Atom.materialize())>"DialogTitle"</Link>
                    " atom into its content."
                </p>
                <p>
                    "Open the popover from the button in its trigger slot. Opening it on hover alone (through "
                    <Code inline=true>"is_open"</Code>") excludes keyboard and touch users; for a hint on hover and focus, "
                    "use a "<Link href=routes::doc::Tooltip.materialize()>"Tooltip"</Link>"."
                </p>
            </Section>

            <Section title="Styling">
                <p>"Override any of these CSS variables to adapt popovers to your design:"</p>
                <CssVariables prefix="--popover-" scss=theme_scss!("popover")/>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Popover.materialize()>"Popover overview"</Link></li>
                <li><Link href=routes::doc::popover::Hook.materialize()>"use_popover"</Link></li>
                <li><Link href=routes::doc::popover::Atom.materialize()>"Popover Atoms"</Link></li>
                <li><Link href=routes::doc::modal::Component.materialize()>"Modal Components"</Link></li>
                <li><Link href=routes::doc::Tooltip.materialize()>"Tooltip"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
