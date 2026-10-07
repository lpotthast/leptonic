use leptos::prelude::*;

use super::demos::{tooltip_basic::TooltipDemo, tooltip_positioning::PositioningDemo};
use crate::{kit::*, routes};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageUseTooltipHook() -> impl IntoView {
    view! {
        <DocPage title="Tooltip Hooks">
            <p>
                "Three hooks build a tooltip for elements you render yourself: its state with the delays shared by all "
                "tooltips, the trigger\u{2019}s behavior and the ARIA link between trigger and tooltip. See the "
                <Link href=routes::doc::Tooltip.materialize()>"Tooltip overview"</Link>" for when to use a tooltip."
            </p>

            <ReactAria hook="useTooltipTrigger"/>

            <Section title="Example">
                <ol>
                    <li>
                        <AnchorLink href="#use-tooltip-trigger-state">"use_tooltip_trigger_state"</AnchorLink>
                        " owns the open state and applies the delays."
                    </li>
                    <li>
                        <AnchorLink href="#use-tooltip-trigger">"use_tooltip_trigger"</AnchorLink>
                        " gives the trigger its handlers (hover, focus, "<Keys keys="Escape"/>", press) and "
                        <Code inline=true>"aria-describedby"</Code>", and the tooltip its id and role."
                    </li>
                    <li>
                        <AnchorLink href="#use-tooltip">"use_tooltip"</AnchorLink>
                        " keeps the tooltip open while the pointer is over it."
                    </li>
                </ol>
                <p>
                    "The hooks don\u{2019}t position anything: "<Link href=routes::doc::overlay_behavior::UseOverlayPosition.materialize()>"use_overlay_position"</Link>
                    " places the tooltip next to its trigger. Hover the button, or focus it with "<Keys keys="Tab"/>
                    "; "<Keys keys="Escape"/>" hides the tooltip. Uncheck \u{201c}Keep open while hovered\u{201d} and the "
                    "tooltip closes as soon as the pointer leaves the button, even towards the tooltip."
                </p>

                <Demo description="Tooltip above a button, with disabled and keep-open toggles" source=include_str!("demos/tooltip_basic.rs") source_open=true>
                    <TooltipDemo/>
                </Demo>
            </Section>

            <Section title="use_tooltip_trigger_state">
                <p>"Owns the open state. Pass the returned state to the other two hooks."</p>

                <Section title="Input" id="use-tooltip-trigger-state-input">
                    <ApiTable kind=ApiKind::Input of="UseTooltipTriggerStateInput">
                        <ApiRow name="delay" ty="Duration" default="1500 ms">
                            "How long hovering takes to open the first tooltip, see "
                            <AnchorLink href="#warm-up-and-cooldown">"Warm-Up and Cooldown"</AnchorLink>"."
                        </ApiRow>
                        <ApiRow name="close_delay" ty="Duration" default="500 ms">
                            "How long the tooltip stays open after the pointer leaves."
                        </ApiRow>
                        <ApiRow name="default_open" ty="bool" default="false">
                            "Whether the tooltip starts open. Ignored when "<Code inline=true>"value"</Code>" is bound."
                        </ApiRow>
                        <ApiRow name="value" ty="Option<ValueBinding<bool>>" default="None">
                            "The open state as app state ("<Code inline=true>"Some(rw_signal.into())"</Code>"), replacing "
                            <Code inline=true>"default_open"</Code>"."
                        </ApiRow>
                        <ApiRow name="on_open_change" ty="Option<Callback<bool>>" default="None">"Called when the open state changes."</ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-tooltip-trigger-state-return">
                    <ApiTable kind=ApiKind::Return of="TooltipTriggerState">
                        <ApiRow name="overlay" ty="OverlayTriggerState">
                            "The underlying overlay state; "<Code inline=true>"overlay.is_open"</Code>" tells whether the tooltip is open."
                        </ApiRow>
                        <ApiRow name="should_skip_animation" ty="Signal<bool>">
                            "Whether the current change should skip its animation: set when one tooltip replaces another "
                            "while warmed up."
                        </ApiRow>
                    </ApiTable>
                    <p>
                        <Code inline=true>"open(timing)"</Code>" and "<Code inline=true>"close(timing)"</Code>" take a "
                        <Code inline=true>"TooltipTiming"</Code>": "<Code inline=true>"Delayed"</Code>" waits for the delay "
                        "(as hovering does), "<Code inline=true>"Immediate"</Code>" doesn\u{2019}t (as focus and "
                        <Keys keys="Escape"/>" do). "<Code inline=true>"is_open()"</Code>" reads the open state."
                    </p>
                </Section>
            </Section>

            <Section title="use_tooltip_trigger">
                <p>
                    "Takes the input and the state: "<Code inline=true>"use_tooltip_trigger(input, state)"</Code>
                    ". Spread "<Code inline=true>"trigger_props.into_attrs()"</Code>" onto the trigger and put "
                    <Code inline=true>"tooltip_props.id"</Code>" and "<Code inline=true>"tooltip_props.role"</Code>
                    " on the tooltip element."
                </p>

                <Section title="Input" id="use-tooltip-trigger-input">
                    <ApiTable kind=ApiKind::Input of="UseTooltipTriggerInput">
                        <ApiRow name="is_disabled" ty="Signal<bool>" default="false">"Whether the tooltip never opens."</ApiRow>
                        <ApiRow name="trigger" ty="TooltipTriggerMode" default="Hover">
                            "What opens the tooltip, see "<AnchorLink href="#trigger-modes">"Trigger Modes"</AnchorLink>"."
                        </ApiRow>
                        <ApiRow name="should_close_on_press" ty="Signal<bool>" default="true">
                            "Whether pressing the trigger closes the tooltip."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-tooltip-trigger-return">
                    <ApiTable kind=ApiKind::Return of="UseTooltipTriggerReturn">
                        <ApiRow name="trigger_props" ty="UseTooltipTriggerProps">
                            "The trigger\u{2019}s id, "<Code inline=true>"aria-describedby"</Code>" (pointing to the tooltip "
                            "while it is open) and its pointer, focus and keyboard handlers."
                        </ApiRow>
                        <ApiRow name="tooltip_props" ty="UseTooltipTriggerTooltipProps">
                            "The "<Code inline=true>"id"</Code>" and the "<Code inline=true>"role"</Code>" ("
                            <Code inline=true>"tooltip"</Code>") of the tooltip element."
                        </ApiRow>
                        <ApiRow name="is_open" ty="Signal<bool>">"Whether the tooltip is open."</ApiRow>
                        <ApiRow name="trigger_id" ty="String">"The id of the trigger."</ApiRow>
                        <ApiRow name="tooltip_id" ty="String">"The id of the tooltip."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="use_tooltip">
                <p>
                    "Spread "<Code inline=true>"props.into_attrs()"</Code>" onto the tooltip element. Moving the pointer "
                    "from the trigger onto the tooltip then keeps it open, so a longer text can be read or selected."
                </p>

                <Section title="Input" id="use-tooltip-input">
                    <ApiTable kind=ApiKind::Input of="UseTooltipInput">
                        <ApiRow name="is_disabled" ty="Signal<bool>" default="false">"Whether hovering the tooltip is ignored."</ApiRow>
                        <ApiRow name="state" ty="Option<TooltipTriggerState>" default="None">
                            "The tooltip state: hovering the tooltip opens it right away, leaving it closes it after the close delay."
                        </ApiRow>
                        <ApiRow name="on_open" ty="Option<Callback<()>>" default="None">
                            "Called when the pointer enters the tooltip, without a "<Code inline=true>"state"</Code>"."
                        </ApiRow>
                        <ApiRow name="on_close" ty="Option<Callback<()>>" default="None">
                            "Called when the pointer leaves the tooltip, without a "<Code inline=true>"state"</Code>"."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-tooltip-return">
                    <ApiTable kind=ApiKind::Return of="UseTooltipReturn">
                        <ApiRow name="props" ty="UseTooltipProps">
                            "The "<Code inline=true>"pointerenter"</Code>" and "<Code inline=true>"pointerleave"</Code>
                            " handlers of the tooltip."
                        </ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Warm-Up and Cooldown">
                <p>"All tooltips share one global state, so they don\u{2019}t flash up while the pointer passes over the page:"</p>
                <ul>
                    <li>
                        "On hover, the first tooltip opens after "<Code inline=true>"delay"</Code>" (1500 ms by default). "
                        "Focusing a trigger with the keyboard opens its tooltip at once."
                    </li>
                    <li>"Once a tooltip was open, the next ones open at once."</li>
                    <li>
                        "After a tooltip starts closing, others still open at once for a cooldown of "
                        <Code inline=true>"close_delay"</Code>", but at least 500 ms. Then the next hover waits for the "
                        "delay again."
                    </li>
                    <li>"Only one tooltip is open at a time."</li>
                </ul>
            </Section>

            <Section title="Trigger Modes">
                <DocTable headers=&["TooltipTriggerMode", "Opens the tooltip on"]>
                    <TableRow>
                        <TableCell><Code inline=true>"Hover"</Code>" (default)"</TableCell>
                        <TableCell>"Hover and keyboard focus. Focusing the trigger by clicking it doesn\u{2019}t open the tooltip."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"Focus"</Code></TableCell>
                        <TableCell>"Keyboard focus only; hovering is ignored."</TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Positioning">
                <p>
                    <Link href=routes::doc::overlay_behavior::UseOverlayPosition.materialize()>"use_overlay_position"</Link>" places the tooltip on the side of "
                    "its trigger you choose and flips it to the opposite side when there is no room. Hover the buttons:"
                </p>

                <Demo description="Tooltips above, below, left and right of their buttons" source=include_str!("demos/tooltip_positioning.rs")>
                    <PositioningDemo/>
                </Demo>
            </Section>

            <Section title="Keyboard">
                <KeyboardTable>
                    <KeyRow keys="Tab">"Focusing the trigger opens its tooltip; moving focus away closes it."</KeyRow>
                    <KeyRow keys="Escape">
                        "Closes the tooltip, wherever focus is. Only the tooltip closes: an enclosing dialog or popover stays open."
                    </KeyRow>
                </KeyboardTable>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Tooltip.materialize()>"Tooltip overview"</Link></li>
                <li><Link href=routes::doc::tooltip::Atom.materialize()>"Tooltip Atoms"</Link></li>
                <li><Link href=routes::doc::popover::Hook.materialize()>"use_popover"</Link></li>
                <li><Link href=routes::doc::OverlayBehavior.materialize()>"Overlay Behavior"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
