use indoc::indoc;
use leptos::prelude::*;

use super::demos::{popover::PopoverDemo, popover_animated::PopoverAnimatedDemo};
use crate::{kit::*, routes};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageAtomPopover() -> impl IntoView {
    view! {
        <DocPage title="Popover Atoms">
            <p>
                "A "<Code inline=true>"DialogTrigger"</Code>" opens an unstyled "<Code inline=true>"Popover"</Code>
                " next to its button; an "<Code inline=true>"OverlayArrow"</Code>" inside the popover points at the "
                "button. See the "<Link href=routes::doc::Popover.materialize()>"Popover overview"</Link>
                " for when to use a popover."
            </p>

            <Section title="Hooks Used">
                <DocTable headers=&["Atom", "Built on"]>
                    <TableRow>
                        <TableCell><Code inline=true>"DialogTrigger"</Code></TableCell>
                        <TableCell>
                            <Link href=routes::doc::overlay_behavior::UseOverlayTriggerState.materialize()>"use_overlay_trigger_state"</Link>
                            " for the open state; a "
                            <Link href=routes::doc::interactions::PressResponder.materialize()>"PressResponder"</Link>
                            " gives its button the press handler, "<Code inline=true>"aria-expanded"</Code>" and "
                            <Code inline=true>"aria-controls"</Code>"."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"Popover"</Code></TableCell>
                        <TableCell>
                            <Link href=routes::doc::popover::Hook.materialize()>"use_popover"</Link>
                            " (dismissal, placement, scroll prevention, hiding the page), the "
                            <Link href=routes::doc::focus::FocusScope.materialize()>"FocusScope"</Link>" and "
                            <Link href=routes::doc::overlay_behavior::DismissButton.materialize()>"DismissButton"</Link>
                            " atoms, and "<Link href=routes::doc::Animation.materialize()>"use_enter_animation and use_exit_animation"</Link>
                            "."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"OverlayArrow"</Code></TableCell>
                        <TableCell>"The arrow position "<Code inline=true>"use_popover"</Code>" computes."</TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Example">
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::atoms::prelude::{Button, Dialog, DialogTitle, DialogTrigger, Popover};

                        view! {
                            <DialogTrigger>
                                <Button>"Settings"</Button>
                                <Popover classes="my-popover">
                                    <Dialog>
                                        <DialogTitle>"Settings"</DialogTitle>
                                        <p>"Choose how the list is sorted."</p>
                                    </Dialog>
                                </Popover>
                            </DialogTrigger>
                        }
                    "#)}
                </Code>
                <p>
                    "To open the popover from elsewhere, bind its state: "
                    <Code inline=true>"<DialogTrigger is_open=open set_open=open>"</Code>" with an "
                    <Code inline=true>"RwSignal<bool>"</Code>"."
                </p>
            </Section>

            <Section title="Demo">
                <p>
                    "Press the button to toggle the popover. It opens above the button and flips below it when there is no "
                    "room; the arrow follows. "<Keys keys="Escape"/>" or a press outside closes it."
                </p>

                <Demo description="Popover atom with an arrow, opened by a DialogTrigger" source=include_str!("demos/popover.rs")>
                    <PopoverDemo/>
                </Demo>
            </Section>

            <Section title="DialogTrigger">
                <p>
                    "Opens and closes the popover inside it when its pressable child (a "<Code inline=true>"Button"</Code>
                    ", or a "<Code inline=true>"Pressable"</Code>") is pressed. The popover finds the state and the trigger "
                    "element through context. Its props are documented with the "
                    <Link href=format!("{}#dialogtrigger", routes::doc::dialog::Atom.materialize())>"Dialog Atoms"</Link>"."
                </p>
            </Section>

            <Section title="Popover">
                <p>
                    "The positioned overlay, rendered in a portal while open. Put a "<Code inline=true>"Dialog"</Code>
                    " in it, or let the popover be the dialog itself (see "
                    <AnchorLink href="#popover-as-dialog">"Popover as Dialog"</AnchorLink>")."
                </p>

                <Section title="Props" id="popover-props">
                    <ApiTable kind=ApiKind::Props of="atoms::popover::Popover">
                        <ApiRow name="is_open" ty="Option<Signal<bool>>" default="None">
                            "Whether the popover is open (controlled): a value or any signal. Default: the surrounding "
                            <Code inline=true>"DialogTrigger"</Code>"\u{2019}s state."
                        </ApiRow>
                        <ApiRow name="set_open" ty="Option<Out<bool>>" default="None">
                            "Receives the open state (closing): an "<Code inline=true>"RwSignal"</Code>", "
                            <Code inline=true>"WriteSignal"</Code>", closure, "<Code inline=true>"Callback"</Code>", \u{2026}"
                        </ApiRow>
                        <ApiRow name="default_open" ty="Option<bool>" default="None">
                            "Whether the popover starts open, with its own state instead of a surrounding "
                            <Code inline=true>"DialogTrigger"</Code>"\u{2019}s. Ignored with "<Code inline=true>"is_open"</Code>"."
                        </ApiRow>
                        <ApiRow name="on_open_change" ty="Option<Callback<bool>>" default="None">
                            "Called when the popover opens or closes. With a surrounding "<Code inline=true>"DialogTrigger"</Code>
                            "\u{2019}s state, pass it to the trigger instead."
                        </ApiRow>
                        <ApiRow name="trigger" ty="Option<CapturedElement>" default="the DialogTrigger\u{2019}s">
                            "The element the popover is positioned at. Required without a surrounding "
                            <Code inline=true>"DialogTrigger"</Code>"."
                        </ApiRow>
                        <ApiRow name="placement" ty="Option<Signal<Placement>>" default="Bottom">
                            "Where the popover goes relative to the trigger, see "<Link href=format!("{}#placements", routes::doc::overlay_behavior::UseOverlayPosition.materialize())>"Placements"</Link>
                            ". In a "<Code inline=true>"MenuTrigger"</Code>" the default is "<Code inline=true>"BottomStart"</Code>
                            ", in a submenu "<Code inline=true>"EndTop"</Code>"."
                        </ApiRow>
                        <ApiRow name="offset" ty="Option<Signal<f64>>" default="8.0">
                            "Distance from the trigger, in pixels. "<Code inline=true>"0.0"</Code>" for a context menu, which "
                            "opens at the pointer."
                        </ApiRow>
                        <ApiRow name="cross_offset" ty="Signal<f64>" default="0.0">"Shift along the trigger\u{2019}s edge, in pixels."</ApiRow>
                        <ApiRow name="container_padding" ty="Signal<f64>" default="12.0">"Minimum distance from the viewport edges, in pixels."</ApiRow>
                        <ApiRow name="should_flip" ty="Signal<bool>" default="true">"Flip to the other side when there is no room."</ApiRow>
                        <ApiRow name="max_height" ty="Signal<Option<f64>>" default="None">
                            "A maximum height; the room available limits it further."
                        </ApiRow>
                        <ApiRow name="arrow_boundary_offset" ty="Signal<f64>" default="0.0">
                            "The minimum distance between an "<Code inline=true>"OverlayArrow"</Code>" and the popover\u{2019}s edges."
                        </ApiRow>
                        <ApiRow name="modality" ty="Option<PopoverModality>" default="Modal">
                            "Whether the popover takes over the page while open, see "
                            <AnchorLink href="#modal-and-non-modal">"Modal and Non-Modal"</AnchorLink>". "
                            <Code inline=true>"NonModal"</Code>" for a submenu."
                        </ApiRow>
                        <ApiRow name="is_keyboard_dismiss_disabled" ty="Signal<bool>" default="false">
                            "Whether "<Keys keys="Escape"/>" no longer closes the popover."
                        </ApiRow>
                        <ApiRow name="should_close_on_interact_outside" ty="Option<InteractOutsideFilter>" default="None">
                            "Decides per element outside whether pressing it (in a modal popover) or moving focus to it closes "
                            "the popover. "<Code inline=true>"None"</Code>" closes for every element."
                        </ApiRow>
                        <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">
                            "Names the popover when it is the dialog itself."
                        </ApiRow>
                        <ApiRow name="aria_labelledby" ty="Option<String>" default="None">
                            "The ids of the elements naming the popover when it is the dialog itself. Default: the "
                            <Code inline=true>"DialogTrigger"</Code>"\u{2019}s trigger."
                        </ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the popover element."</ApiRow>
                        <ApiRow name="children" ty="ChildrenFn">"The popover content. Required."</ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Modal and Non-Modal">
                    <p><Code inline=true>"modality"</Code>" decides how much the popover takes over the page:"</p>
                    <DocTable headers=&["Behavior", "Modal (default)", "NonModal"]>
                        <TableRow>
                            <TableCell>"A press outside"</TableCell>
                            <TableCell>"Lands on an underlay and closes the popover."</TableCell>
                            <TableCell>
                                "Reaches the page. It closes the popover only when it moves focus out of it, e.g. onto a button."
                            </TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>"Focus"</TableCell>
                            <TableCell>"Stays inside."</TableCell>
                            <TableCell>
                                "Stays inside once a "<Code inline=true>"Dialog"</Code>" is in the popover; moving it out "
                                "closes the popover."
                            </TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>"The rest of the page"</TableCell>
                            <TableCell>"Inert: hidden from assistive technology and not scrollable."</TableCell>
                            <TableCell>"Usable. Scrolling it closes the popover, which would drift away from its trigger."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"DismissButton"</Code>"s"</TableCell>
                            <TableCell>"At the start and the end of the popover."</TableCell>
                            <TableCell>"At the end."</TableCell>
                        </TableRow>
                    </DocTable>
                    <p><Keys keys="Escape"/>" closes both."</p>
                </Section>

                <Section title="Popover as Dialog">
                    <p>
                        "A modal popover without a "<Code inline=true>"Dialog"</Code>" inside is the dialog itself: it gets "
                        <Code inline=true>"role=\"dialog\""</Code>", is focused when it opens (unless focus already moved into "
                        "it) and is named by "<Code inline=true>"aria_label"</Code>", "<Code inline=true>"aria_labelledby"</Code>
                        " or, by default, its trigger. Use this for short content; a "<Code inline=true>"Dialog"</Code>
                        " with a "<Code inline=true>"DialogTitle"</Code>" is named by its heading."
                    </p>
                    <Code language=Language::Rust>
                        {indoc!(r#"
                            use leptonic::atoms::prelude::{Button, DialogTrigger, Popover};

                            view! {
                                <DialogTrigger>
                                    <Button>"Help"</Button>
                                    // Named "Help" by its trigger.
                                    <Popover>"Press Escape to close this popover."</Popover>
                                </DialogTrigger>
                            }
                        "#)}
                    </Code>
                </Section>
            </Section>

            <Section title="OverlayArrow">
                <p>
                    "An arrow pointing from the popover at its trigger, also usable in a "
                    <Link href=routes::doc::tooltip::Atom.materialize()>"Tooltip"</Link>". It sits at the popover\u{2019}s "
                    "edge facing the trigger, as close to the trigger\u{2019}s center as the popover allows, and is hidden from "
                    "assistive technology. Its children draw it; "<Code inline=true>"data-placement"</Code>
                    " tells which way to point."
                </p>

                <Section title="Props" id="overlayarrow-props">
                    <ApiTable kind=ApiKind::Props of="OverlayArrow">
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the arrow element."</ApiRow>
                        <ApiRow name="children" ty="Option<Children>" default="None">
                            "The arrow\u{2019}s shape, e.g. an SVG triangle. Give it a fixed size: the popover measures its "
                            "width. Without children, draw the element itself (e.g. with CSS borders)."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::{atoms::prelude::{OverlayArrow, Popover}, hooks::Placement};

                        view! {
                            <Popover placement=Placement::Top classes="my-popover">
                                <p>"Content"</p>
                                <OverlayArrow classes="my-arrow">
                                    <svg width="12" height="6" viewBox="0 0 12 6"><path d="M0 0 L6 6 L12 0"/></svg>
                                </OverlayArrow>
                            </Popover>
                        }
                    "#)}
                </Code>
            </Section>

            <Section title="Data Attributes">
                <ApiTable kind=ApiKind::DataAttributes>
                    <ApiRow name="data-placement" ty="\"top\" | \"bottom\" | \"left\" | \"right\"">
                        "On the popover and the "<Code inline=true>"OverlayArrow"</Code>": the side of the trigger the "
                        "popover opened on, after flipping."
                    </ApiRow>
                    <ApiRow name="data-entering" ty="true">
                        "On the popover while it opens, until the animations its "<Code inline=true>"data-entering"</Code>
                        " styles started have finished."
                    </ApiRow>
                    <ApiRow name="data-exiting" ty="true">
                        "On the popover while it closes. It stays rendered until the animations its "
                        <Code inline=true>"data-exiting"</Code>" styles started have finished."
                    </ApiRow>
                    <ApiRow name="data-trigger" ty="\"DialogTrigger\" | \"MenuTrigger\" | \"SubmenuTrigger\" | \"Select\" | \"ComboBox\"">
                        "On the popover when a trigger opened it: which one, e.g. to style the popovers of menus apart "
                        "from dialogs. Not set on a popover with its own "<Code inline=true>"trigger"</Code>" and state."
                    </ApiRow>
                </ApiTable>
            </Section>

            <Section title="Styling">
                <p>
                    "The atoms bring no styles. "<Code inline=true>"Popover"</Code>" renders the class "
                    <Code inline=true>"leptonic-Popover"</Code>", "<Code inline=true>"OverlayArrow"</Code>" the class "
                    <Code inline=true>"leptonic-OverlayArrow"</Code>", each followed by the "<Code inline=true>"classes"</Code>
                    " you pass; target their state with the data attributes above. Positioning and layering come from the "
                    "atom. The popover also sets two CSS variables: "<Code inline=true>"--trigger-width"</Code>
                    " (the trigger\u{2019}s width, e.g. "<Code inline=true>"min-width: var(--trigger-width)"</Code>
                    " for a popover as wide as its trigger) and "<Code inline=true>"--trigger-anchor-point"</Code>
                    " (the point closest to the trigger, e.g. as "<Code inline=true>"transform-origin"</Code>")."
                </p>
                <p>
                    "The arrow\u{2019}s shape is yours: render it inside "<Code inline=true>"OverlayArrow"</Code>
                    " (an SVG, or an element drawn with borders as in the demo), give the arrow a fixed size, and turn the "
                    "shape with "<Code inline=true>"data-placement"</Code>". The book\u{2019}s demos use these rules:"
                </p>
                <Code language=Language::Css>
                    {indoc!(r#"
                        .my-popover { max-width: 300px; padding: 1em; border: 1px solid var(--border); border-radius: 8px; background: var(--surface); }
                        .my-arrow { display: flex; width: 16px; height: 8px; }
                        /* A triangle pointing down, at a trigger below the popover; turned for the other sides. */
                        .my-arrow-shape { border-left: 8px solid transparent; border-right: 8px solid transparent; border-top: 8px solid var(--surface); }
                        .my-arrow[data-placement="bottom"] .my-arrow-shape { transform: rotate(180deg); }
                        .my-arrow[data-placement="left"] .my-arrow-shape { transform: rotate(-90deg); }
                        .my-arrow[data-placement="right"] .my-arrow-shape { transform: rotate(90deg); }
                    "#)}
                </Code>
                <p>
                    "Leptonic also ships an optional atom theme that styles the default classes, for apps that don\u{2019}t "
                    "want to start from scratch: "<Code inline=true>"@use \"leptonic/leptonic-atoms\";"</Code>"."
                </p>

                <Section title="Animation">
                    <p>
                        "Animate the popover in and out with CSS: "<Code inline=true>"data-entering"</Code>" is set while it "
                        "opens, "<Code inline=true>"data-exiting"</Code>" while it closes. A closing popover stays rendered "
                        "until its exit animation finished, so the animation plays to its end. Keep animations short, and "
                        "replace movement with a fade for users who prefer reduced motion:"
                    </p>
                    <Code language=Language::Css>
                        {indoc!(r"
                            .my-popover { transform-origin: var(--trigger-anchor-point); }
                            .my-popover[data-entering] { animation: pop-in 150ms ease-out; }
                            .my-popover[data-exiting] { animation: pop-out 150ms ease-in forwards; }

                            @keyframes pop-in { from { opacity: 0; transform: scale(0.9); } }
                            @keyframes pop-out { to { opacity: 0; transform: scale(0.9); } }

                            @media (prefers-reduced-motion: reduce) {
                                @keyframes pop-in { from { opacity: 0; } }
                                @keyframes pop-out { to { opacity: 0; } }
                            }
                        ")}
                    </Code>
                    <Demo description="A popover that grows out of its button and shrinks back when it closes" source=include_str!("demos/popover_animated.rs")>
                        <PopoverAnimatedDemo/>
                    </Demo>
                </Section>
            </Section>

            <Section title="Composition">
                <ul>
                    <li>
                        "Content: a "<Link href=routes::doc::dialog::Atom.materialize()>"Dialog"</Link>
                        " with a title for anything interactive, or the popover itself as the dialog for short content."
                    </li>
                    <li>
                        <Link href=routes::doc::menu::Atom.materialize()>"MenuTrigger"</Link>" opens a menu in a "
                        <Code inline=true>"Popover"</Code>" and sets its default placement; "
                        <Code inline=true>"SubmenuTrigger"</Code>" opens a non-modal one next to its item."
                    </li>
                    <li>
                        "The "<Link href=routes::doc::select::Atom.materialize()>"Select"</Link>" and "
                        <Link href=routes::doc::combobox::Atom.materialize()>"ComboBox"</Link>
                        " atoms render their listbox in popovers of their own ("<Code inline=true>"SelectPopover"</Code>", "
                        <Code inline=true>"ComboBoxPopover"</Code>") that share the popover\u{2019}s rendering and data attributes."
                    </li>
                </ul>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Popover.materialize()>"Popover overview"</Link></li>
                <li><Link href=routes::doc::popover::Hook.materialize()>"use_popover"</Link></li>
                <li><Link href=routes::doc::dialog::Atom.materialize()>"Dialog Atoms"</Link></li>
                <li><Link href=routes::doc::tooltip::Atom.materialize()>"Tooltip Atoms"</Link></li>
                <li><Link href=routes::doc::overlay_behavior::DismissButton.materialize()>"DismissButton"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
