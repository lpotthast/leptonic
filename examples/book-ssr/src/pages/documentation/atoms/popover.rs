use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::popover::PopoverDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageAtomPopover() -> impl IntoView {
    view! {
        <DocPage title="Popover atom">
            <p>
                "A "<Code inline=true>"DialogTrigger"</Code>" opens an unstyled "<Code inline=true>"Popover"</Code>
                " when its "<Code inline=true>"Button"</Code>" is pressed. The popover is positioned next to the button and "
                "closes on Escape, outside interaction or when focus leaves it. See the "
                <Link href=routes::doc::Popover.materialize()>"Popover overview"</Link>" for concept guidance."
            </p>

            <Section title="Hooks Used">
                <p>
                    <Code inline=true>"DialogTrigger"</Code>" owns an "<Code inline=true>"OverlayTriggerState"</Code>
                    " and gives its button the press handler, "<Code inline=true>"aria-expanded"</Code>" and "
                    <Code inline=true>"aria-controls"</Code>" through a "
                    <Link href=routes::doc::interactions::PressResponder.materialize()>"PressResponder"</Link>". "
                    <Code inline=true>"Popover"</Code>" is built on "<Link href=routes::doc::popover::Hook.materialize()>"use_popover"</Link>
                    " (dismissal, placement, scroll prevention) and the "
                    <Link href=routes::doc::focus::FocusScope.materialize()>"FocusScope"</Link>" atom."
                </p>
            </Section>

            <Section title="DialogTrigger">
                <p>
                    "Opens and closes the overlay inside it when its pressable child (a "<Code inline=true>"Button"</Code>
                    ") is pressed. The overlay finds the state and the trigger element through context."
                </p>

                <ApiTable kind=ApiKind::Props of="DialogTrigger">
                    <ApiRow name="default_open" ty="bool" default="false">"Whether the overlay starts open."</ApiRow>
                    <ApiRow name="on_open_change" ty="Option<Callback<bool>>" default="None">
                        "Called when the overlay opens or closes."
                    </ApiRow>
                    <ApiRow name="state" ty="Option<OverlayTriggerState>" default="None">
                        "The open state as app state ("<Code inline=true>"state=rw_signal"</Code>", "
                        <Code inline=true>"state=(read, write)"</Code>") or a shared "<Code inline=true>"OverlayTriggerState"</Code>
                        ", replacing "<Code inline=true>"default_open"</Code>" and "<Code inline=true>"on_open_change"</Code>"."
                    </ApiRow>
                    <ApiRow name="children" ty="Children">"The trigger button and the overlay."</ApiRow>
                </ApiTable>
            </Section>

            <Section title="Popover">
                <p>
                    "The positioned overlay, rendered in a portal while open. Modal by default: focus stays inside, the "
                    "rest of the page doesn\u{2019}t scroll and is hidden from assistive technology, and screen reader users "
                    "get dismiss buttons. Put a "<Code inline=true>"Dialog"</Code>" in it, or let the popover be the dialog "
                    "itself (see "<a href="#popover-as-dialog">"Popover as Dialog"</a>")."
                </p>

                <ApiTable kind=ApiKind::Props of="atoms::popover::Popover">
                    <ApiRow name="state" ty="Option<OverlayTriggerState>" default="the DialogTrigger\u{2019}s">
                        "Whether the popover is open. Needed only without a surrounding "<Code inline=true>"DialogTrigger"</Code>"."
                    </ApiRow>
                    <ApiRow name="trigger" ty="Option<CapturedElement>" default="the DialogTrigger\u{2019}s">
                        "The element the popover is positioned at. Needed only without a surrounding "
                        <Code inline=true>"DialogTrigger"</Code>"."
                    </ApiRow>
                    <ApiRow name="placement_x" ty="Signal<PlacementX>" default="Center">
                        "Horizontal placement relative to the trigger. The logical placements "<Code inline=true>"Start"</Code>
                        " and "<Code inline=true>"End"</Code>" resolve against the writing direction of the enclosing "
                        <Code inline=true>"I18nProvider"</Code>" (left-to-right without one)."
                    </ApiRow>
                    <ApiRow name="placement_y" ty="Signal<PlacementY>" default="Below">"Vertical placement relative to the trigger."</ApiRow>
                    <ApiRow name="offset" ty="Signal<f64>" default="8.0">"Distance from the trigger, in pixels."</ApiRow>
                    <ApiRow name="cross_offset" ty="Signal<f64>" default="0.0">"Shift along the trigger\u{2019}s edge, in pixels."</ApiRow>
                    <ApiRow name="container_padding" ty="Signal<f64>" default="12.0">"Minimum distance from the viewport edges."</ApiRow>
                    <ApiRow name="should_flip" ty="Signal<bool>" default="true">"Flip to the other side when there is no room."</ApiRow>
                    <ApiRow name="modality" ty="PopoverModality" default="Modal">
                        "Whether the popover takes over the page while open. A "<Code inline=true>"NonModal"</Code>
                        " popover has no underlay and no focus trap, and closes when the page scrolls."
                    </ApiRow>
                    <ApiRow name="is_keyboard_dismiss_disabled" ty="bool" default="false">"Ignore Escape."</ApiRow>
                    <ApiRow name="should_close_on_interact_outside" ty="Option<Callback<web_sys::Element, bool>>" default="None">
                        "Decides per outside element whether interacting with it closes the popover."
                    </ApiRow>
                    <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">
                        "Names a modal popover without a "<Code inline=true>"Dialog"</Code>" inside, which is the dialog itself."
                    </ApiRow>
                    <ApiRow name="aria_labelledby" ty="Option<String>" default="None">
                        "Ids of the elements naming it then. Default: the "<Code inline=true>"DialogTrigger"</Code>"\u{2019}s trigger."
                    </ApiRow>
                    <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the popover element."</ApiRow>
                    <ApiRow name="children" ty="ChildrenFn">"The popover content."</ApiRow>
                </ApiTable>
            </Section>

            <Section title="Example">
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::atoms::{button::Button, dialog::*, popover::Popover};

                        view! {
                            <DialogTrigger>
                                <Button>"Settings"</Button>
                                <Popover placement_y=PlacementY::Below classes="my-popover">
                                    <Dialog>
                                        <DialogTitle>"Settings"</DialogTitle>
                                        "Content"
                                    </Dialog>
                                </Popover>
                            </DialogTrigger>
                        }
                    "#)}
                </Code>
                <p>
                    "Bind the open state to app state with "<Code inline=true>"<DialogTrigger state=is_open>"</Code>
                    " (an "<Code inline=true>"RwSignal<bool>"</Code>"), for example to open the popover from elsewhere."
                </p>
            </Section>

            <Section title="Popover as Dialog">
                <p>
                    "A modal popover without a "<Code inline=true>"Dialog"</Code>" inside is the dialog itself: it gets "
                    <Code inline=true>"role=\"dialog\""</Code>", is focused when it opens (unless focus already moved into it) "
                    "and is named by "<Code inline=true>"aria_label"</Code>", "<Code inline=true>"aria_labelledby"</Code>
                    " or, by default, its trigger. Use this for simple content; a "<Code inline=true>"Dialog"</Code>
                    " with a "<Code inline=true>"DialogTitle"</Code>" names itself by its heading."
                </p>
                <Code language=Language::Rust>
                    {indoc!(r#"
                        <DialogTrigger>
                            <Button>"Help"</Button>
                            // Named "Help" by its trigger.
                            <Popover>"Press Escape to close this popover."</Popover>
                        </DialogTrigger>
                    "#)}
                </Code>
                <p>
                    "A "<Code inline=true>"Dialog"</Code>" without a title, "<Code inline=true>"aria_label"</Code>" or "
                    <Code inline=true>"aria_labelledby"</Code>" is named by the "<Code inline=true>"DialogTrigger"</Code>
                    "\u{2019}s button in the same way."
                </p>
            </Section>

            <Section title="Demo">
                <p>"Press the button to toggle the popover."</p>

                <Demo description="Popover atom opened by a DialogTrigger" source=include_str!("demos/popover.rs")>
                    <PopoverDemo/>
                </Demo>
            </Section>

            <Section title="Data Attributes">
                <ApiTable kind=ApiKind::DataAttributes>
                    <ApiRow name="data-placement" ty="\"top\" | \"bottom\" | \"left\" | \"right\"">
                        "Set on the popover: the side of the trigger it opened on, after flipping."
                    </ApiRow>
                </ApiTable>
            </Section>

            <Section title="Styling">
                <p>
                    "The atoms add no classes. Use "<Code inline=true>"data-placement"</Code>
                    " to adapt to the side the popover opened on, for example to let an animation start at the trigger:"
                </p>
                <Code language=Language::Css>
                    {indoc!(r#"
                        .my-popover { padding: 1em; border-radius: 8px; background: white; }
                        .my-popover[data-placement="top"] { transform-origin: bottom center; }
                        .my-popover[data-placement="bottom"] { transform-origin: top center; }
                    "#)}
                </Code>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Popover.materialize()>"Popover overview"</Link></li>
                <li><Link href=routes::doc::popover::Hook.materialize()>"use_popover"</Link></li>
                <li><Link href=routes::doc::popover::Component.materialize()>"Popover component"</Link></li>
                <li><Link href=routes::doc::overlays::DismissButton.materialize()>"DismissButton atom"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
