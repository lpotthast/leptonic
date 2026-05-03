use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::{
    tooltip_basic::TooltipDemo, tooltip_hover_to_keep_open::HoverToKeepOpenDemo,
    tooltip_positioning::PositioningDemo,
};
use crate::pages::documentation::{article::Article, demo_shell::DemoShell, toc::Toc};

#[component]
pub fn PageUseTooltipHook() -> impl IntoView {
    view! {
        <Article>
            <h1 id="tooltip" class="anchor">
                "Tooltip Hooks"
                <AnchorLink href="#tooltip" description="Direct link to article header"/>
            </h1>

            <p>"Hooks for creating accessible tooltips with hover/focus triggers, warmup/cooldown delays, hover-to-keep-open behavior, and proper ARIA associations. "
               "The tooltip system is built from three composable hooks. "
               "See the "<Link href=crate::routes::doc::Tooltip.materialize()>"Tooltip overview"</Link>" for concept guidance."</p>

            <p>
                "Based on react-aria\u{2019}s "
                <LinkExt href="https://react-spectrum.adobe.com/react-aria/useTooltip.html" target=LinkTarget::_Blank>
                    "useTooltip"
                </LinkExt>
                "."
            </p>

            <h2 id="hook-composition" class="anchor">
                "Hook Composition"
                <AnchorLink href="#hook-composition" description="Direct link to hook composition"/>
            </h2>

            <p>"The tooltip system is built from composable layers, each handling a specific concern:"</p>

            <ol>
                <li>
                    <strong>"State Layer"</strong>
                    " - "
                    <code>"use_tooltip_trigger_state"</code>
                    ": Manages open/close state, global warmup/cooldown, and singleton enforcement (only one tooltip at a time)"
                </li>
                <li>
                    <strong>"Trigger Layer"</strong>
                    " - "
                    <code>"use_tooltip_trigger"</code>
                    ": ARIA attributes, event handlers (hover, focus, Escape key, press-to-close)"
                </li>
                <li>
                    <strong>"Tooltip Layer"</strong>
                    " - "
                    <code>"use_tooltip"</code>
                    ": Hover-to-keep-open behavior on the tooltip content element"
                </li>
            </ol>

            <p>"For positioned tooltips, combine with "<code>"use_overlay_position"</code>" for automatic placement with collision detection."</p>

            <h2 id="when-to-use" class="anchor">
                "When to Use"
                <AnchorLink href="#when-to-use" description="Direct link to when to use"/>
            </h2>

            <p>"Tooltips are for supplementary, non-interactive text. Choose the right hook based on your needs:"</p>

            <table style="width: 100%; border-collapse: collapse;">
                <thead>
                    <tr style="border-bottom: 2px solid var(--brand-color);">
                        <th style="text-align: left; padding: 0.5em;">"Use Case"</th>
                        <th style="text-align: left; padding: 0.5em;">"Hook(s)"</th>
                    </tr>
                </thead>
                <tbody>
                    <tr style="border-bottom: 1px solid #ccc;">
                        <td style="padding: 0.5em;">"Brief helper text on hover/focus"</td>
                        <td style="padding: 0.5em;">"Tooltip hooks"</td>
                    </tr>
                    <tr style="border-bottom: 1px solid #ccc;">
                        <td style="padding: 0.5em;">"Rich interactive overlay content"</td>
                        <td style="padding: 0.5em;"><code>"use_popover"</code></td>
                    </tr>
                    <tr style="border-bottom: 1px solid #ccc;">
                        <td style="padding: 0.5em;">"Menu with keyboard navigation"</td>
                        <td style="padding: 0.5em;"><code>"use_menu"</code>" + "<code>"use_menu_trigger"</code></td>
                    </tr>
                    <tr style="border-bottom: 1px solid #ccc;">
                        <td style="padding: 0.5em;">"Modal dialog"</td>
                        <td style="padding: 0.5em;"><code>"use_modal"</code>" + "<code>"use_modal_backdrop"</code></td>
                    </tr>
                </tbody>
            </table>

            <p style="margin-top: 1em;">
                <strong>"Key distinction:"</strong>
                " Tooltips should not contain interactive content (per WAI-ARIA). For interactive overlays, use a popover."
            </p>

            <h2 id="demo" class="anchor">
                "Interactive Demo"
                <AnchorLink href="#demo" description="Direct link to demo"/>
            </h2>

            <p>"Hover over or focus the button to see the tooltip. Move your mouse to the tooltip content — it stays open. Press Escape to close it."</p>

            <DemoShell source=include_str!("demos/tooltip_basic.rs")>
                <TooltipDemo />
            </DemoShell>

            <h2 id="hover-to-keep-open" class="anchor">
                "Hover-to-Keep-Open"
                <AnchorLink href="#hover-to-keep-open" description="Direct link to hover-to-keep-open"/>
            </h2>

            <p>"The "<code>"use_tooltip"</code>" hook adds pointer event handlers to the tooltip content element. "
               "When the user moves their cursor from the trigger to the tooltip, it stays open. "
               "Without this hook, the tooltip closes as soon as the cursor leaves the trigger."</p>

            <p>"Compare the two buttons below:"</p>

            <DemoShell source=include_str!("demos/tooltip_hover_to_keep_open.rs")>
                <HoverToKeepOpenDemo />
            </DemoShell>

            <h2 id="warmup-cooldown" class="anchor">
                "Warmup / Cooldown System"
                <AnchorLink href="#warmup-cooldown" description="Direct link to warmup/cooldown"/>
            </h2>

            <p>"The tooltip state hook implements a global warmup/cooldown system (matching react-aria):"</p>
            <ul>
                <li>"The first tooltip requires a delay before appearing (default: 1500ms)"</li>
                <li>"Once a tooltip has been shown, subsequent tooltips appear instantly (warm state)"</li>
                <li>"After all tooltips close, a cooldown period (default: 500ms) resets back to cold state"</li>
                <li>"Only one tooltip is visible at a time"</li>
            </ul>

            <h2 id="trigger-modes" class="anchor">
                "Trigger Modes"
                <AnchorLink href="#trigger-modes" description="Direct link to trigger modes"/>
            </h2>

            <ul>
                <li><code>"TooltipTrigger::Hover"</code>" - Show on hover and keyboard/virtual focus (default, most accessible). "
                    "Note: pointer focus (clicking the trigger) does not show the tooltip, only keyboard focus does."</li>
                <li><code>"TooltipTrigger::Focus"</code>" - Show on focus only (hover is ignored)"</li>
            </ul>

            <h2 id="positioning" class="anchor">
                "Positioning with use_overlay_position"
                <AnchorLink href="#positioning" description="Direct link to positioning"/>
            </h2>

            <p>"Combine the tooltip hooks with "<code>"use_overlay_position"</code>" for automatic positioning with collision detection. "
               "Hover over the buttons to see tooltips positioned in different directions:"</p>

            <DemoShell source=include_str!("demos/tooltip_positioning.rs")>
                <PositioningDemo />
            </DemoShell>

            <h2 id="api" class="anchor">
                "API"
                <AnchorLink href="#api" description="Direct link to API"/>
            </h2>

            <h3 id="state-input" class="anchor">
                "UseTooltipTriggerStateInput"
                <AnchorLink href="#state-input" description="Direct link to state input"/>
            </h3>

            <TableContainer>
                <Table bordered=true hoverable=true>
                    <TableHeader>
                        <TableRow>
                            <TableHeaderCell min_width=true>"Field"</TableHeaderCell>
                            <TableHeaderCell min_width=true>"Type"</TableHeaderCell>
                            <TableHeaderCell min_width=true>"Default"</TableHeaderCell>
                            <TableHeaderCell>"Description"</TableHeaderCell>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        <TableRow>
                            <TableCell><code>"delay"</code></TableCell>
                            <TableCell><code>"u32"</code></TableCell>
                            <TableCell><code>"1500"</code></TableCell>
                            <TableCell>"Delay before showing the tooltip when cold (in ms)."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"close_delay"</code></TableCell>
                            <TableCell><code>"u32"</code></TableCell>
                            <TableCell><code>"500"</code></TableCell>
                            <TableCell>"Delay before hiding the tooltip (in ms)."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"is_open"</code></TableCell>
                            <TableCell><code>"Option<Signal<bool>>"</code></TableCell>
                            <TableCell><code>"None"</code></TableCell>
                            <TableCell>"Controlled open state. When Some, external code controls the state."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"default_open"</code></TableCell>
                            <TableCell><code>"bool"</code></TableCell>
                            <TableCell><code>"false"</code></TableCell>
                            <TableCell>"Default open state for uncontrolled mode."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"on_open_change"</code></TableCell>
                            <TableCell><code>"Option<Callback<bool>>"</code></TableCell>
                            <TableCell><code>"None"</code></TableCell>
                            <TableCell>"Callback when the open state changes."</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            <h3 id="state-return" class="anchor">
                "UseTooltipTriggerStateReturn"
                <AnchorLink href="#state-return" description="Direct link to state return"/>
            </h3>

            <TableContainer>
                <Table bordered=true hoverable=true>
                    <TableHeader>
                        <TableRow>
                            <TableHeaderCell min_width=true>"Field"</TableHeaderCell>
                            <TableHeaderCell min_width=true>"Type"</TableHeaderCell>
                            <TableHeaderCell>"Description"</TableHeaderCell>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        <TableRow>
                            <TableCell><code>"is_open"</code></TableCell>
                            <TableCell><code>"Signal<bool>"</code></TableCell>
                            <TableCell>"Whether the tooltip is open."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"open"</code></TableCell>
                            <TableCell><code>"Callback<bool>"</code></TableCell>
                            <TableCell>"Open the tooltip. bool = true: skip delay, false: respect delay."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"close"</code></TableCell>
                            <TableCell><code>"Callback<bool>"</code></TableCell>
                            <TableCell>"Close the tooltip. bool = true: skip delay, false: respect delay."</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            <h3 id="trigger-input" class="anchor">
                "UseTooltipTriggerInput"
                <AnchorLink href="#trigger-input" description="Direct link to trigger input"/>
            </h3>

            <TableContainer>
                <Table bordered=true hoverable=true>
                    <TableHeader>
                        <TableRow>
                            <TableHeaderCell min_width=true>"Field"</TableHeaderCell>
                            <TableHeaderCell min_width=true>"Type"</TableHeaderCell>
                            <TableHeaderCell min_width=true>"Default"</TableHeaderCell>
                            <TableHeaderCell>"Description"</TableHeaderCell>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        <TableRow>
                            <TableCell><code>"is_disabled"</code></TableCell>
                            <TableCell><code>"Signal<bool>"</code></TableCell>
                            <TableCell><code>"false"</code></TableCell>
                            <TableCell>"Whether the tooltip is disabled."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"trigger"</code></TableCell>
                            <TableCell><code>"TooltipTrigger"</code></TableCell>
                            <TableCell><code>"Hover"</code></TableCell>
                            <TableCell>"The trigger behavior (Hover or Focus)."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"should_close_on_press"</code></TableCell>
                            <TableCell><code>"bool"</code></TableCell>
                            <TableCell><code>"true"</code></TableCell>
                            <TableCell>"Whether pressing the trigger closes the tooltip."</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            <h3 id="trigger-return" class="anchor">
                "UseTooltipTriggerReturn"
                <AnchorLink href="#trigger-return" description="Direct link to trigger return"/>
            </h3>

            <TableContainer>
                <Table bordered=true hoverable=true>
                    <TableHeader>
                        <TableRow>
                            <TableHeaderCell min_width=true>"Field"</TableHeaderCell>
                            <TableHeaderCell min_width=true>"Type"</TableHeaderCell>
                            <TableHeaderCell>"Description"</TableHeaderCell>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        <TableRow>
                            <TableCell><code>"trigger_props"</code></TableCell>
                            <TableCell><code>"UseTooltipTriggerProps"</code></TableCell>
                            <TableCell>"Props for the trigger element (events + ARIA)."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"tooltip_props"</code></TableCell>
                            <TableCell><code>"UseTooltipTriggerTooltipProps"</code></TableCell>
                            <TableCell>"Props for the tooltip element (id + role)."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"is_open"</code></TableCell>
                            <TableCell><code>"Signal<bool>"</code></TableCell>
                            <TableCell>"Whether the tooltip is open."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"trigger_id"</code></TableCell>
                            <TableCell><code>"String"</code></TableCell>
                            <TableCell>"The ID of the trigger element."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"tooltip_id"</code></TableCell>
                            <TableCell><code>"String"</code></TableCell>
                            <TableCell>"The ID of the tooltip element."</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            <h3 id="tooltip-input" class="anchor">
                "UseTooltipInput"
                <AnchorLink href="#tooltip-input" description="Direct link to tooltip input"/>
            </h3>

            <TableContainer>
                <Table bordered=true hoverable=true>
                    <TableHeader>
                        <TableRow>
                            <TableHeaderCell min_width=true>"Field"</TableHeaderCell>
                            <TableHeaderCell min_width=true>"Type"</TableHeaderCell>
                            <TableHeaderCell min_width=true>"Default"</TableHeaderCell>
                            <TableHeaderCell>"Description"</TableHeaderCell>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        <TableRow>
                            <TableCell><code>"disabled"</code></TableCell>
                            <TableCell><code>"Signal<bool>"</code></TableCell>
                            <TableCell>"-"</TableCell>
                            <TableCell>"Whether the tooltip is disabled."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"state"</code></TableCell>
                            <TableCell><code>"Option<UseTooltipTriggerStateReturn>"</code></TableCell>
                            <TableCell><code>"None"</code></TableCell>
                            <TableCell>"Tooltip trigger state. When provided, hovering the tooltip keeps it open via the warmup/cooldown system."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"on_open"</code></TableCell>
                            <TableCell><code>"Option<Callback<()>>"</code></TableCell>
                            <TableCell><code>"None"</code></TableCell>
                            <TableCell>"Called when the tooltip should open. Used when state is None."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"on_close"</code></TableCell>
                            <TableCell><code>"Option<Callback<()>>"</code></TableCell>
                            <TableCell><code>"None"</code></TableCell>
                            <TableCell>"Called when the tooltip should close. Used when state is None."</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            <h3 id="tooltip-return" class="anchor">
                "UseTooltipReturn"
                <AnchorLink href="#tooltip-return" description="Direct link to tooltip return"/>
            </h3>

            <TableContainer>
                <Table bordered=true hoverable=true>
                    <TableHeader>
                        <TableRow>
                            <TableHeaderCell min_width=true>"Field"</TableHeaderCell>
                            <TableHeaderCell min_width=true>"Type"</TableHeaderCell>
                            <TableHeaderCell>"Description"</TableHeaderCell>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        <TableRow>
                            <TableCell><code>"props"</code></TableCell>
                            <TableCell><code>"UseTooltipProps"</code></TableCell>
                            <TableCell>"Props for the tooltip element (pointerenter/pointerleave handlers)."</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            <h2 id="accessibility" class="anchor">
                "Accessibility"
                <AnchorLink href="#accessibility" description="Direct link to accessibility"/>
            </h2>

            <ul>
                <li>"Tooltip elements must have "<code>"role=\"tooltip\""</code>" (provided by "<code>"tooltip_props.role"</code>")"</li>
                <li>"Tooltips are shown on keyboard/virtual focus (not pointer focus)"</li>
                <li>"Pressing Escape closes the tooltip (global document handler in capture phase)"</li>
                <li>"ARIA attributes ("<code>"aria-describedby"</code>") are automatically managed by the trigger"</li>
                <li>"Tooltip content should be supplementary, not essential"</li>
                <li>"Tooltips should not contain interactive elements (WAI-ARIA)"</li>
                <li>"Clicking/pressing the trigger closes the tooltip"</li>
            </ul>

            <h2 id="features" class="anchor">
                "Features"
                <AnchorLink href="#features" description="Direct link to features"/>
            </h2>

            <ul>
                <li>"Hover and focus triggers"</li>
                <li>"Hover-to-keep-open on tooltip content (via "<code>"use_tooltip"</code>")"</li>
                <li>"Global warmup/cooldown system (instant subsequent tooltips)"</li>
                <li>"Singleton enforcement (only one tooltip at a time)"</li>
                <li>"Configurable open/close delays"</li>
                <li>"Escape key to close (global handler)"</li>
                <li>"Press to close"</li>
                <li>"Automatic ARIA associations"</li>
                <li>"Disabled state support"</li>
                <li>"Automatic positioning with collision detection (via "<code>"use_overlay_position"</code>")"</li>
            </ul>

            <h2 id="related-hooks" class="anchor">
                "Related Hooks"
                <AnchorLink href="#related-hooks" description="Direct link to related hooks"/>
            </h2>

            <ul>
                <li><code>"use_overlay_position"</code>" — Positioning primitive for tooltip placement."</li>
                <li><code>"use_popover"</code>" — For interactive overlay content (tooltips should be non-interactive per WAI-ARIA)."</li>
                <li><code>"use_focus_visible"</code>" — Provides "<code>"get_modality()"</code>" for keyboard vs pointer focus."</li>
            </ul>

            <h2 id="deviations" class="anchor">
                "React Aria Deviations"
                <AnchorLink href="#deviations" description="Direct link to deviations"/>
            </h2>

            <h3>"Omitted"</h3>
            <ul>
                <li>"No "<code>"mousedown"</code>" fallback handler — PointerEvent is always available"</li>
            </ul>

            <h3>"API Differences"</h3>
            <ul>
                <li><code>"Callback<bool>"</code>" for open/close (bool = immediate) instead of separate "<code>"() => void"</code>" callbacks"</li>
                <li><code>"use_tooltip"</code>" accepts "<code>"UseTooltipTriggerStateReturn"</code>" directly instead of separate "<code>"on_open"</code>"/"<code>"on_close"</code>" callbacks, enabling participation in the warmup/cooldown system"</li>
                <li>"Thread-local global state ("<code>"tooltip_registry"</code>") for warmup/cooldown instead of module-level JS variables"</li>
                <li><code>"get_modality()"</code>" from "<code>"use_focus_visible"</code>" instead of react-aria's "<code>"getInteractionModality()"</code></li>
            </ul>

            <h2 id="see-also" class="anchor">
                "See Also"
                <AnchorLink href="#see-also" description="Direct link to section: See Also"/>
            </h2>

            <ul>
                <li><Link href=crate::routes::doc::Tooltip.materialize()>"Tooltip overview"</Link></li>
                <li><Link href=crate::routes::doc::Popover.materialize()>"Popover concept"</Link></li>
                <li><Link href=crate::routes::doc::Overlays.materialize()>"Overlays overview"</Link></li>
            </ul>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "Tooltip Hooks", link: "#tooltip" },
                Toc::Leaf { title: "Hook Composition", link: "#hook-composition" },
                Toc::Leaf { title: "When to Use", link: "#when-to-use" },
                Toc::Leaf { title: "Interactive Demo", link: "#demo" },
                Toc::Leaf { title: "Hover-to-Keep-Open", link: "#hover-to-keep-open" },
                Toc::Leaf { title: "Warmup / Cooldown", link: "#warmup-cooldown" },
                Toc::Leaf { title: "Trigger Modes", link: "#trigger-modes" },
                Toc::Leaf { title: "Positioning", link: "#positioning" },
                Toc::Leaf { title: "API", link: "#api" },
                Toc::Leaf { title: "Accessibility", link: "#accessibility" },
                Toc::Leaf { title: "Features", link: "#features" },
                Toc::Leaf { title: "Related Hooks", link: "#related-hooks" },
                Toc::Leaf { title: "React Aria Deviations", link: "#deviations" },
                Toc::Leaf { title: "See Also", link: "#see-also" },
            ]
        }/>
    }
}
