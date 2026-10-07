use indoc::indoc;
use leptos::prelude::*;

use super::demos::tooltip::TooltipDemo;
use crate::{kit::*, routes};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageAtomTooltip() -> impl IntoView {
    view! {
        <DocPage title="Tooltip Atoms">
            <p>
                "A "<Code inline=true>"TooltipTrigger"</Code>" shows its unstyled "<Code inline=true>"Tooltip"</Code>
                " while the focusable atom inside it is hovered or focused. See the "
                <Link href=routes::doc::Tooltip.materialize()>"Tooltip overview"</Link>" for when to use a tooltip."
            </p>

            <Section title="Hooks Used">
                <DocTable headers=&["Atom", "Built on"]>
                    <TableRow>
                        <TableCell><Code inline=true>"TooltipTrigger"</Code></TableCell>
                        <TableCell>
                            <Link href=format!("{}#use-tooltip-trigger-state", routes::doc::tooltip::Hook.materialize())>"use_tooltip_trigger_state"</Link>
                            " and "
                            <Link href=format!("{}#use-tooltip-trigger", routes::doc::tooltip::Hook.materialize())>"use_tooltip_trigger"</Link>
                            ". It hands the trigger\u{2019}s handlers and "<Code inline=true>"aria-describedby"</Code>
                            " to the focusable atom inside through a "<Code inline=true>"FocusableContext"</Code>" (see "
                            <Link href=routes::doc::focus::UseFocusable.materialize()>"use_focusable"</Link>")."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"Tooltip"</Code></TableCell>
                        <TableCell>
                            <Link href=format!("{}#use-tooltip", routes::doc::tooltip::Hook.materialize())>"use_tooltip"</Link>
                            " and "<Link href=routes::doc::overlay_behavior::UseOverlayPosition.materialize()>"use_overlay_position"</Link>"."
                        </TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Example">
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::atoms::prelude::{Button, Tooltip, TooltipTrigger};

                        view! {
                            <TooltipTrigger>
                                <Button>"Publish"</Button>
                                <Tooltip classes="my-tooltip">"Make the post visible to everyone"</Tooltip>
                            </TooltipTrigger>
                        }
                    "#)}
                </Code>
            </Section>

            <Section title="Demo">
                <p>
                    "Hover the button: its tooltip waits for the delay; hover the icon next, and its tooltip opens at once. "
                    "Focus either with "<Keys keys="Tab"/>" to open its tooltip right away; "<Keys keys="Escape"/>
                    " closes it. The tooltips fade in and out and point at their trigger with an "
                    <Code inline=true>"OverlayArrow"</Code>"."
                </p>

                <Demo description="A button and an info icon with tooltips, with a disabled toggle" source=include_str!("demos/tooltip.rs")>
                    <TooltipDemo/>
                </Demo>
            </Section>

            <Section title="TooltipTrigger">
                <p>"Owns the open state and wraps the trigger and its tooltip."</p>

                <Section title="Props" id="tooltiptrigger-props">
                    <ApiTable kind=ApiKind::Props of="TooltipTrigger">
                        <ApiRow name="delay" ty="Duration" default="1500 ms">
                            "How long hovering takes to open the first tooltip. Once one was open, the next ones open at once, see "
                            <Link href=format!("{}#warm-up-and-cooldown", routes::doc::tooltip::Hook.materialize())>"Warm-Up and Cooldown"</Link>"."
                        </ApiRow>
                        <ApiRow name="close_delay" ty="Duration" default="500 ms">"How long the tooltip stays open after the pointer leaves."</ApiRow>
                        <ApiRow name="trigger" ty="TooltipTriggerMode" default="Hover">
                            <Code inline=true>"Hover"</Code>": hovering and keyboard focus open the tooltip; "
                            <Code inline=true>"Focus"</Code>": only keyboard focus."
                        </ApiRow>
                        <ApiRow name="is_disabled" ty="Signal<bool>" default="false">"Whether the tooltip never opens."</ApiRow>
                        <ApiRow name="should_close_on_press" ty="Signal<bool>" default="true">"Whether pressing the trigger closes the tooltip."</ApiRow>
                        <ApiRow name="default_open" ty="bool" default="false">"Whether the tooltip starts open. Ignored with "<Code inline=true>"is_open"</Code>"."</ApiRow>
                        <ApiRow name="on_open_change" ty="Option<Callback<bool>>" default="None">"Called when the tooltip opens or closes."</ApiRow>
                        <ApiRow name="is_open" ty="Option<Signal<bool>>" default="None">
                            "Whether the tooltip is open (controlled): a value or any signal."
                        </ApiRow>
                        <ApiRow name="set_open" ty="Option<Out<bool>>" default="None">
                            "Receives the new state: an "<Code inline=true>"RwSignal"</Code>", "<Code inline=true>"WriteSignal"</Code>
                            ", closure, "<Code inline=true>"Callback"</Code>", \u{2026}"
                        </ApiRow>
                        <ApiRow name="children" ty="Children">
                            "The trigger, a focusable atom, and its "<Code inline=true>"Tooltip"</Code>". Required."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Custom Triggers">
                    <p>
                        "Every focusable atom inside a "<Code inline=true>"TooltipTrigger"</Code>" is its trigger: a "
                        <Link href=routes::doc::button::Atom.materialize()>"Button"</Link>", a "
                        <Link href=routes::doc::link::Atom.materialize()>"Link"</Link>", a "
                        <Link href=routes::doc::toggle_button::Atom.materialize()>"ToggleButton"</Link>", \u{2026} "
                        "For an element of your own, use one of two atoms:"
                    </p>
                    <ul>
                        <li>
                            <Link href=routes::doc::focus::Focusable.materialize()>"Focusable"</Link>
                            " makes an element that can\u{2019}t take focus on its own focusable, e.g. an icon. The element "
                            "needs a role and a name ("<Code inline=true>"role=\"img\""</Code>" and "
                            <Code inline=true>"aria-label"</Code>" for an icon), so that the tooltip describes something."
                        </li>
                        <li>
                            <Link href=format!("{}#pressable", routes::doc::interactions::PressResponder.materialize())>"Pressable"</Link>
                            " makes an element of your own with an interactive role pressable (e.g. a "
                            <Code inline=true>"<span role=\"button\">"</Code>"), and a tooltip trigger as well."
                        </li>
                    </ul>
                    <Code language=Language::Rust>
                        {indoc!(r#"
                            use leptonic::atoms::prelude::{Focusable, Tooltip, TooltipTrigger};
                            use leptos_icons::Icon;

                            view! {
                                <TooltipTrigger>
                                    <Focusable>
                                        <span role="img" aria-label="Shipping">
                                            <Icon icon=icondata::BsInfoCircle/>
                                        </span>
                                    </Focusable>
                                    <Tooltip>"Ships within two days"</Tooltip>
                                </TooltipTrigger>
                            }
                        "#)}
                    </Code>
                    <p>
                        "A disabled button can\u{2019}t take focus, so keyboard users never see its tooltip. Explain why an "
                        "action is unavailable in text next to it instead."
                    </p>
                </Section>
            </Section>

            <Section title="Tooltip">
                <p>
                    "The tooltip of the surrounding "<Code inline=true>"TooltipTrigger"</Code>": "
                    <Code inline=true>"role=\"tooltip\""</Code>", rendered in a portal while open and positioned next to the "
                    "trigger. Hovering it keeps it open; scrolling closes it. Put an "
                    <Link href=format!("{}#overlayarrow", routes::doc::popover::Atom.materialize())>"OverlayArrow"</Link>
                    " in it for an arrow pointing at the trigger."
                </p>

                <Section title="Props" id="tooltip-props">
                    <ApiTable kind=ApiKind::Props of="Tooltip">
                        <ApiRow name="placement" ty="Signal<Placement>" default="Top">
                            "Where the tooltip goes relative to the trigger, see "
                            <Link href=format!("{}#placements", routes::doc::overlay_behavior::UseOverlayPosition.materialize())>"Placements"</Link>"."
                        </ApiRow>
                        <ApiRow name="offset" ty="Signal<f64>" default="0.0">"Distance from the trigger, in pixels. Leave room for an arrow."</ApiRow>
                        <ApiRow name="cross_offset" ty="Signal<f64>" default="0.0">"Shift along the trigger\u{2019}s edge, in pixels."</ApiRow>
                        <ApiRow name="container_padding" ty="Signal<f64>" default="12.0">"Minimum distance from the viewport edges, in pixels."</ApiRow>
                        <ApiRow name="should_flip" ty="Signal<bool>" default="true">"Flip to the other side when there is no room."</ApiRow>
                        <ApiRow name="arrow_boundary_offset" ty="Signal<f64>" default="0.0">
                            "The minimum distance between an "<Code inline=true>"OverlayArrow"</Code>" and the tooltip\u{2019}s edges."
                        </ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the tooltip element."</ApiRow>
                        <ApiRow name="children" ty="ChildrenFn">"The tooltip text, and an optional "<Code inline=true>"OverlayArrow"</Code>". Required."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Data Attributes">
                <ApiTable kind=ApiKind::DataAttributes>
                    <ApiRow name="data-placement" ty="\"top\" | \"bottom\" | \"left\" | \"right\"">
                        "On the tooltip and its "<Code inline=true>"OverlayArrow"</Code>": the side of the trigger the "
                        "tooltip opened on, after flipping."
                    </ApiRow>
                    <ApiRow name="data-entering" ty="true">"On the tooltip while it appears, until its enter animations finished."</ApiRow>
                    <ApiRow name="data-exiting" ty="true">
                        "On the tooltip while it hides. It stays rendered until its exit animations finished."
                    </ApiRow>
                </ApiTable>
            </Section>

            <Section title="Styling">
                <p>
                    "The atoms bring no styles. "<Code inline=true>"Tooltip"</Code>" renders the class "
                    <Code inline=true>"leptonic-Tooltip"</Code>", an "<Code inline=true>"OverlayArrow"</Code>" inside it the "
                    "class "<Code inline=true>"leptonic-OverlayArrow"</Code>", each followed by the "
                    <Code inline=true>"classes"</Code>" you pass; target their state with the data attributes above. "
                    "Positioning and layering come from the atom. The tooltip also sets "
                    <Code inline=true>"--trigger-anchor-point"</Code>", the point closest to the trigger, e.g. as the "
                    <Code inline=true>"transform-origin"</Code>" of an animation."
                </p>
                <p>
                    "The arrow\u{2019}s shape is yours: render it inside "<Code inline=true>"OverlayArrow"</Code>
                    ", turn it with "<Code inline=true>"data-placement"</Code>", and give the tooltip an "
                    <Code inline=true>"offset"</Code>" that leaves room for it. Fade the tooltip in and out with "
                    <Code inline=true>"data-entering"</Code>" and "<Code inline=true>"data-exiting"</Code>
                    "; a fade doesn\u{2019}t move, so it needs no reduced-motion rule. The book\u{2019}s demos use these "
                    "rules, with the accent color so that the tooltip stands out:"
                </p>
                <Code language=Language::Css>
                    {indoc!(r#"
                        .my-tooltip { padding: 0.5em 1em; border-radius: 4px; background: var(--accent); color: var(--surface); }
                        .my-tooltip[data-entering] { animation: fade 150ms ease-out; }
                        .my-tooltip[data-exiting] { animation: fade 150ms ease-in reverse forwards; }
                        @keyframes fade { from { opacity: 0; } }

                        /* A triangle pointing down, at a trigger below the tooltip; turned for the other sides. */
                        .my-arrow { display: flex; width: 12px; height: 6px; }
                        .my-arrow-shape { border-left: 6px solid transparent; border-right: 6px solid transparent; border-top: 6px solid var(--accent); }
                        .my-arrow[data-placement="bottom"] .my-arrow-shape { transform: rotate(180deg); }
                        .my-arrow[data-placement="left"] .my-arrow-shape { transform: rotate(-90deg); }
                        .my-arrow[data-placement="right"] .my-arrow-shape { transform: rotate(90deg); }
                    "#)}
                </Code>
                <p>
                    "Leptonic also ships an optional atom theme that styles the default classes, for apps that don\u{2019}t "
                    "want to start from scratch: "<Code inline=true>"@use \"leptonic/leptonic-atoms\";"</Code>"."
                </p>
            </Section>

            <Section title="Composition">
                <ul>
                    <li>
                        "Triggers: any focusable atom, or "<Code inline=true>"Focusable"</Code>" and "
                        <Code inline=true>"Pressable"</Code>" around your own element (see "
                        <AnchorLink href="#custom-triggers">"Custom Triggers"</AnchorLink>")."
                    </li>
                    <li>
                        "Content: text, and an "<Code inline=true>"OverlayArrow"</Code>". A tooltip holds no interactive "
                        "content; for that, use a "<Link href=routes::doc::Popover.materialize()>"Popover"</Link>"."
                    </li>
                    <li>
                        "A tooltip inside a dialog or popover: "<Keys keys="Escape"/>" closes the tooltip only, not the "
                        "overlay around it."
                    </li>
                </ul>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Tooltip.materialize()>"Tooltip overview"</Link></li>
                <li><Link href=routes::doc::tooltip::Hook.materialize()>"Tooltip Hooks"</Link></li>
                <li><Link href=routes::doc::popover::Atom.materialize()>"Popover Atoms"</Link></li>
                <li><Link href=routes::doc::focus::Focusable.materialize()>"Focusable"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
