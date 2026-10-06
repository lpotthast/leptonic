use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::{
    tooltip_basic::TooltipDemo, tooltip_hover_to_keep_open::HoverToKeepOpenDemo,
    tooltip_positioning::PositioningDemo,
};
use crate::{kit::*, routes};

#[component]
pub fn PageUseTooltipHook() -> impl IntoView {
    view! {
        <DocPage title="Tooltip Hooks">
            <p>
                "These hooks build accessible tooltips: shown on hover and keyboard focus, with warmup and cooldown "
                "delays shared by all tooltips, hover-to-keep-open, and the ARIA link between trigger and tooltip. "
                "See the "<Link href=routes::doc::Tooltip.materialize()>"Tooltip overview"</Link>" for concept guidance."
            </p>

            <ReactAria hook="useTooltipTrigger"/>

            <Section title="Hook Composition">
                <p>"A tooltip is built from three hooks, each handling one concern:"</p>

                <ol>
                    <li>
                        <a href="#use-tooltip-trigger-state"><Code inline=true>"use_tooltip_trigger_state"</Code></a>
                        " owns the open state, applies the warmup and cooldown delays and makes sure only one tooltip is "
                        "open at a time."
                    </li>
                    <li>
                        <a href="#use-tooltip-trigger"><Code inline=true>"use_tooltip_trigger"</Code></a>
                        " provides the trigger\u{2019}s event handlers (hover, focus, Escape, press) and its "
                        <Code inline=true>"aria-describedby"</Code>", plus the id and role of the tooltip."
                    </li>
                    <li>
                        <a href="#use-tooltip"><Code inline=true>"use_tooltip"</Code></a>
                        " keeps the tooltip open while the pointer is over the tooltip itself."
                    </li>
                </ol>

                <p>
                    "The hooks don\u{2019}t position anything. Combine them with "<Code inline=true>"use_overlay_position"</Code>
                    " to place the tooltip next to its trigger."
                </p>
            </Section>

            <Section title="When to Use">
                <p>
                    "Tooltips hold short, supplementary, non-interactive text. WAI-ARIA doesn\u{2019}t allow interactive "
                    "content in a tooltip; use a popover for that."
                </p>

                <DocTable headers=&["Use case", "Hooks"]>
                    <TableRow><TableCell>"Brief helper text on hover or focus"</TableCell><TableCell>"Tooltip hooks"</TableCell></TableRow>
                    <TableRow><TableCell>"Rich or interactive overlay content"</TableCell><TableCell><Link href=routes::doc::popover::Hook.materialize()>"use_popover"</Link></TableCell></TableRow>
                    <TableRow><TableCell>"Menu with keyboard navigation"</TableCell><TableCell><Link href=routes::doc::menu::Hook.materialize()>"use_menu_trigger, use_menu"</Link></TableCell></TableRow>
                    <TableRow><TableCell>"Modal dialog"</TableCell><TableCell><Link href=routes::doc::modal::Hook.materialize()>"use_modal_backdrop, use_modal, use_dialog"</Link></TableCell></TableRow>
                </DocTable>
            </Section>

            <Section title="Example">
                <p>
                    "Hover or focus the button with the keyboard to show the tooltip, and press Escape to hide it. The tooltip "
                    "stays open while you move the pointer onto it."
                </p>

                <Demo description="Tooltip above a button, with a disabled toggle" source=include_str!("demos/tooltip_basic.rs") source_open=true>
                    <TooltipDemo/>
                </Demo>
            </Section>

            <Section title="Hover-to-Keep-Open">
                <p>
                    <Code inline=true>"use_tooltip"</Code>" adds pointer handlers to the tooltip element. When you move the "
                    "pointer from the trigger onto the tooltip, it stays open, which matters for longer text you want to "
                    "read or select. Without it, the tooltip closes as soon as the pointer leaves the trigger. Compare both:"
                </p>

                <Demo description="Two tooltips, one with use_tooltip disabled" source=include_str!("demos/tooltip_hover_to_keep_open.rs")>
                    <HoverToKeepOpenDemo/>
                </Demo>
            </Section>

            <Section title="Warmup and Cooldown">
                <p>"All tooltips share one global state:"</p>

                <ul>
                    <li>
                        "On hover, the first tooltip appears after "<Code inline=true>"delay"</Code>" (1500 ms by default). "
                        "Keyboard focus shows it immediately."
                    </li>
                    <li>"Once a tooltip has been shown, the next ones appear immediately."</li>
                    <li>
                        "After the last tooltip closes, a cooldown ("<Code inline=true>"close_delay"</Code>
                        ", 500 ms by default) returns to the initial state."
                    </li>
                    <li>"Only one tooltip is open at a time."</li>
                </ul>
            </Section>

            <Section title="Trigger Modes">
                <DocTable headers=&["TooltipTriggerMode", "Shows the tooltip on"]>
                    <TableRow>
                        <TableCell><Code inline=true>"Hover"</Code>" (default)"</TableCell>
                        <TableCell>"Pointer hover and keyboard focus. Focusing the trigger by clicking it does not show the tooltip."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"Focus"</Code></TableCell>
                        <TableCell>"Keyboard focus only. Hover is ignored."</TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Positioning">
                <p>
                    "With "<Code inline=true>"use_overlay_position"</Code>" the tooltip is placed next to its trigger and "
                    "flips to the opposite side when there is not enough space. Hover the buttons:"
                </p>

                <Demo description="Tooltips above, below, left and right of their buttons" source=include_str!("demos/tooltip_positioning.rs")>
                    <PositioningDemo/>
                </Demo>
            </Section>

            <Section title="use_tooltip_trigger_state">
                <p>"Owns the open state. Pass the returned state to the other two hooks."</p>

                <Section title="Input" id="use-tooltip-trigger-state-input">
                    <ApiTable kind=ApiKind::Input of="UseTooltipTriggerStateInput">
                        <ApiRow name="delay" ty="Duration" default="1.5 s">
                            "How long hovering takes to open the first tooltip. Once a tooltip was open, others open right away "
                            "until all were closed for a while."
                        </ApiRow>
                        <ApiRow name="close_delay" ty="Duration" default="500 ms">"How long the tooltip stays open after the pointer leaves."</ApiRow>
                        <ApiRow name="default_open" ty="bool" default="false">"Whether the tooltip starts open. Ignored when "<Code inline=true>"value"</Code>" is bound."</ApiRow>
                        <ApiRow name="value" ty="Option<ValueBinding<bool>>" default="None">
                            "The open state as app state ("<Code inline=true>"Some(rw_signal.into())"</Code>"), replacing "<Code inline=true>"default_open"</Code>"."
                        </ApiRow>
                        <ApiRow name="on_open_change" ty="Option<Callback<bool>>" default="None">"Called when the open state changes."</ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-tooltip-trigger-state-return">
                    <ApiTable kind=ApiKind::Return of="TooltipTriggerState">
                        <ApiRow name="overlay" ty="OverlayTriggerState">"Whether the tooltip is open ("<Code inline=true>"overlay.is_open"</Code>")."</ApiRow>
                        <ApiRow name="should_skip_animation" ty="Signal<bool>">
                            "Whether the current transition should skip its animation: set when one tooltip replaces another "
                            "during the warm-up period."
                        </ApiRow>
                    </ApiTable>
                    <p>
                        <Code inline=true>"open(timing)"</Code>" and "<Code inline=true>"close(timing)"</Code>" take a "<Code inline=true>"TooltipTiming"</Code>": "
                        <Code inline=true>"Delayed"</Code>" waits for the delay (as hovering does), "<Code inline=true>"Immediate"</Code>
                        " doesn\u{2019}t (as focus and Escape do). "<Code inline=true>"is_open()"</Code>" reads the open state."
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
                        <ApiRow name="is_disabled" ty="Signal<bool>" default="false">"Never show the tooltip."</ApiRow>
                        <ApiRow name="trigger" ty="TooltipTriggerMode" default="Hover">
                            "What shows the tooltip, see "<a href="#trigger-modes">"Trigger Modes"</a>"."
                        </ApiRow>
                        <ApiRow name="should_close_on_press" ty="bool" default="true">"Close the tooltip when the trigger is pressed."</ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-tooltip-trigger-return">
                    <ApiTable kind=ApiKind::Return of="UseTooltipTriggerReturn">
                        <ApiRow name="trigger_props" ty="UseTooltipTriggerProps">
                            "The trigger\u{2019}s id, "<Code inline=true>"aria-describedby"</Code>" (set while the tooltip is open) "
                            "and its pointer, focus and keyboard handlers."
                        </ApiRow>
                        <ApiRow name="tooltip_props" ty="UseTooltipTriggerTooltipProps">
                            "The "<Code inline=true>"id"</Code>" and "<Code inline=true>"role"</Code>" ("<Code inline=true>"tooltip"</Code>
                            ") for the tooltip element."
                        </ApiRow>
                        <ApiRow name="is_open" ty="Signal<bool>">"Whether the tooltip is open."</ApiRow>
                        <ApiRow name="trigger_id" ty="String">"The id of the trigger."</ApiRow>
                        <ApiRow name="tooltip_id" ty="String">"The id of the tooltip."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="use_tooltip">
                <p>"Spread "<Code inline=true>"props.into_attrs()"</Code>" onto the tooltip element."</p>

                <Section title="Input" id="use-tooltip-input">
                    <ApiTable kind=ApiKind::Input of="UseTooltipInput">
                        <ApiRow name="is_disabled" ty="Signal<bool>">"Ignore hovering the tooltip."</ApiRow>
                        <ApiRow name="state" ty="Option<TooltipTriggerState>">
                            "The tooltip state. Hovering the tooltip opens it immediately, leaving it closes it after the close delay."
                        </ApiRow>
                        <ApiRow name="on_open" ty="Option<Callback<()>>">"Called on hover start when "<Code inline=true>"state"</Code>" is "<Code inline=true>"None"</Code>"."</ApiRow>
                        <ApiRow name="on_close" ty="Option<Callback<()>>">"Called on hover end when "<Code inline=true>"state"</Code>" is "<Code inline=true>"None"</Code>"."</ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-tooltip-return">
                    <ApiTable kind=ApiKind::Return of="UseTooltipReturn">
                        <ApiRow name="props" ty="UseTooltipProps">"The "<Code inline=true>"pointerenter"</Code>" and "<Code inline=true>"pointerleave"</Code>" handlers of the tooltip."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Accessibility">
                <ul>
                    <li>"The tooltip element has "<Code inline=true>"role=\"tooltip\""</Code>" (from "<Code inline=true>"tooltip_props.role"</Code>")."</li>
                    <li>"While the tooltip is open, the trigger\u{2019}s "<Code inline=true>"aria-describedby"</Code>" points to it."</li>
                    <li>"Keyboard focus shows the tooltip immediately, focus by pointer does not."</li>
                    <li>"Pressing the trigger closes the tooltip (unless "<Code inline=true>"should_close_on_press"</Code>" is "<Code inline=true>"false"</Code>")."</li>
                    <li>"Tooltip content should be supplementary and must not contain interactive elements."</li>
                </ul>

                <KeyboardTable>
                    <KeyRow keys="Tab">"Focusing the trigger shows the tooltip, leaving it hides the tooltip."</KeyRow>
                    <KeyRow keys="Escape">
                        "Hides the tooltip, wherever focus is. Only the tooltip closes: an enclosing dialog or popover stays open."
                    </KeyRow>
                </KeyboardTable>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Tooltip.materialize()>"Tooltip overview"</Link></li>
                <li><Link href=routes::doc::tooltip::Atom.materialize()>"Tooltip atoms"</Link></li>
                <li><Link href=routes::doc::Popover.materialize()>"Popover"</Link></li>
                <li><Link href=routes::doc::focus::UseFocusVisible.materialize()>"use_focus_visible"</Link></li>
                <li><Link href=routes::doc::Overlays.materialize()>"Overlays overview"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
