use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::{overlay_basic::BasicOverlayDemo, overlay_positioning::PositioningDemo};
use crate::pages::documentation::{article::Article, demo_shell::DemoShell, toc::Toc};

#[component]
pub fn PageUseOverlay() -> impl IntoView {
    view! {
        <Article>
            <h1 id="overlay-hooks" class="anchor">
                "Overlay Hooks"
                <AnchorLink href="#overlay-hooks" description="Direct link to article header"/>
            </h1>

            <p>"Low-level overlay primitives for dismiss behavior, ARIA trigger attributes, and CSS positioning. "
               "Higher-level hooks like "<Code inline=true>"use_popover"</Code>" and "<Code inline=true>"use_modal"</Code>" compose these internally. "
               "See the "<Link href=crate::routes::doc::Overlays.materialize()>"Overlays overview"</Link>" for concept guidance."</p>

            <p>
                "Based on react-aria\u{2019}s "
                <LinkExt href="https://react-spectrum.adobe.com/react-aria/useOverlay.html" target=LinkTarget::_Blank>
                    "useOverlay"
                </LinkExt>
                "."
            </p>

            <h2 id="hook-composition" class="anchor">
                "Hook Composition"
                <AnchorLink href="#hook-composition" description="Direct link to hook composition"/>
            </h2>

            <p>"The overlay system is split into three single-responsibility hooks:"</p>

            <ul>
                <li><Code inline=true>"use_overlay"</Code>" — Dismiss behavior (Escape key, click outside, blur) and overlay stacking. "
                    "Only the topmost overlay closes when multiple are open."</li>
                <li><Code inline=true>"use_overlay_trigger"</Code>" — ARIA attributes on the trigger element: "
                    <Code inline=true>"aria-haspopup"</Code>", "<Code inline=true>"aria-expanded"</Code>", and "<Code inline=true>"aria-controls"</Code>"."</li>
                <li><Code inline=true>"use_overlay_position"</Code>" — CSS positioning of the overlay relative to a target element, "
                    "with automatic flipping and viewport containment."</li>
            </ul>

            <p>"You can use them individually or combine all three. "<Code inline=true>"use_overlay"</Code>" is the only required hook — "
               "trigger ARIA and positioning are optional depending on your use case."</p>

            <h2 id="when-to-use" class="anchor">
                "When to Use"
                <AnchorLink href="#when-to-use" description="Direct link to when to use"/>
            </h2>

            <p>"These hooks are low-level primitives. Choose the right level of abstraction:"</p>

            <table style="width: 100%; border-collapse: collapse;">
                <thead>
                    <tr style="border-bottom: 2px solid var(--brand-color);">
                        <th style="text-align: left; padding: 0.5em;">"Use Case"</th>
                        <th style="text-align: left; padding: 0.5em;">"Hook(s)"</th>
                    </tr>
                </thead>
                <tbody>
                    <tr style="border-bottom: 1px solid #ccc;">
                        <td style="padding: 0.5em;">"Direct overlay control with custom behavior"</td>
                        <td style="padding: 0.5em;"><Code inline=true>"use_overlay"</Code>" + "<Code inline=true>"use_overlay_trigger"</Code>" + "<Code inline=true>"use_overlay_position"</Code></td>
                    </tr>
                    <tr style="border-bottom: 1px solid #ccc;">
                        <td style="padding: 0.5em;">"Popover with sensible defaults"</td>
                        <td style="padding: 0.5em;"><Code inline=true>"use_popover"</Code></td>
                    </tr>
                    <tr style="border-bottom: 1px solid #ccc;">
                        <td style="padding: 0.5em;">"Menu with keyboard navigation"</td>
                        <td style="padding: 0.5em;"><Code inline=true>"use_menu"</Code>" + "<Code inline=true>"use_menu_trigger"</Code></td>
                    </tr>
                    <tr style="border-bottom: 1px solid #ccc;">
                        <td style="padding: 0.5em;">"Modal dialog"</td>
                        <td style="padding: 0.5em;"><Code inline=true>"use_modal"</Code>" + "<Code inline=true>"use_dialog"</Code></td>
                    </tr>
                </tbody>
            </table>

            <h2 id="interactive-demo" class="anchor">
                "Interactive Demo"
                <AnchorLink href="#interactive-demo" description="Direct link to interactive demo"/>
            </h2>

            <p>"This demo uses only "<Code inline=true>"use_overlay"</Code>" to show a fixed-position panel with Escape key and click-outside dismiss:"</p>

            <DemoShell source=include_str!("demos/overlay_basic.rs")>
                <BasicOverlayDemo />
            </DemoShell>

            <h2 id="positioning-demo" class="anchor">
                "Positioning Demo"
                <AnchorLink href="#positioning-demo" description="Direct link to positioning demo"/>
            </h2>

            <p>"This demo combines all three hooks: "<Code inline=true>"use_overlay"</Code>" for dismiss behavior, "
               <Code inline=true>"use_overlay_trigger"</Code>" for ARIA attributes, and "<Code inline=true>"use_overlay_position"</Code>" for placement. "
               "Use the radio buttons to change the placement:"</p>

            <DemoShell source=include_str!("demos/overlay_positioning.rs")>
                <PositioningDemo />
            </DemoShell>

            <h2 id="dismiss-behavior" class="anchor">
                "Dismiss Behavior"
                <AnchorLink href="#dismiss-behavior" description="Direct link to dismiss behavior"/>
            </h2>

            <p><Code inline=true>"use_overlay"</Code>" provides three independent dismiss mechanisms:"</p>

            <h3>"Escape Key"</h3>
            <p>"When the user presses Escape, the topmost overlay closes. The event is stopped and default-prevented "
               "so it does not propagate to outer overlays. Disable with "<Code inline=true>"is_keyboard_dismiss_disabled: true"</Code>"."</p>

            <h3>"Click Outside"</h3>
            <p>"Enabled via "<Code inline=true>"is_dismissable: true"</Code>". When the user clicks outside the overlay, "
               "only the topmost overlay closes. A filter callback "<Code inline=true>"should_close_on_interact_outside"</Code>" "
               "lets you prevent dismissal for specific elements (e.g., a trigger button that should toggle instead)."</p>

            <h3>"Blur"</h3>
            <p>"Enabled via "<Code inline=true>"should_close_on_blur: true"</Code>". When focus leaves the overlay, it closes. "
               "Unlike Escape and click-outside, blur bypasses the topmost check — any overlay that loses focus closes. "
               "Focus moving to a child focus scope (e.g., a menu inside a dialog) does not trigger blur dismissal."</p>

            <h3>"Overlay Stacking"</h3>
            <p>"A thread-local stack tracks all open overlays in order. Escape and click-outside only close the last "
               "(topmost) overlay in the stack. This ensures that opening a nested overlay does not accidentally close "
               "a parent overlay."</p>

            <h2 id="api" class="anchor">
                "API"
                <AnchorLink href="#api" description="Direct link to API"/>
            </h2>

            <h3 id="use-overlay-input" class="anchor">
                "UseOverlayInput"
                <AnchorLink href="#use-overlay-input" description="Direct link to UseOverlayInput"/>
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
                            <TableCell><Code inline=true>"is_open"</Code></TableCell>
                            <TableCell><Code inline=true>"Signal<bool>"</Code></TableCell>
                            <TableCell>"-"</TableCell>
                            <TableCell>"Whether the overlay is currently open."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"on_close"</Code></TableCell>
                            <TableCell><Code inline=true>"Callback<()>"</Code></TableCell>
                            <TableCell>"-"</TableCell>
                            <TableCell>"Called when the overlay should close."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"is_dismissable"</Code></TableCell>
                            <TableCell><Code inline=true>"bool"</Code></TableCell>
                            <TableCell>"-"</TableCell>
                            <TableCell>"Whether to close when the user interacts outside the overlay."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"should_close_on_blur"</Code></TableCell>
                            <TableCell><Code inline=true>"bool"</Code></TableCell>
                            <TableCell>"-"</TableCell>
                            <TableCell>"Whether to close when focus leaves the overlay."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"is_keyboard_dismiss_disabled"</Code></TableCell>
                            <TableCell><Code inline=true>"bool"</Code></TableCell>
                            <TableCell>"-"</TableCell>
                            <TableCell>"Whether to disable Escape key dismissal."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"should_close_on_interact_outside"</Code></TableCell>
                            <TableCell><Code inline=true>"Option<Callback<Element, bool>>"</Code></TableCell>
                            <TableCell><Code inline=true>"None"</Code></TableCell>
                            <TableCell>"Optional filter: return true to allow closing, false to prevent it."</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            <h3 id="use-overlay-return" class="anchor">
                "UseOverlayReturn"
                <AnchorLink href="#use-overlay-return" description="Direct link to UseOverlayReturn"/>
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
                            <TableCell><Code inline=true>"props"</Code></TableCell>
                            <TableCell><Code inline=true>"UseOverlayProps"</Code></TableCell>
                            <TableCell>"Props for the overlay element (id, keydown, focus handlers)."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"underlay_props"</Code></TableCell>
                            <TableCell><Code inline=true>"UseOverlayUnderlayProps"</Code></TableCell>
                            <TableCell>"Props for an optional underlay element (pointerdown handler)."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"id"</Code></TableCell>
                            <TableCell><Code inline=true>"Oco<'static, str>"</Code></TableCell>
                            <TableCell>"Unique overlay ID. Pass to use_overlay_trigger as overlay_id."</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            <h3 id="use-overlay-trigger-input" class="anchor">
                "UseOverlayTriggerInput"
                <AnchorLink href="#use-overlay-trigger-input" description="Direct link to UseOverlayTriggerInput"/>
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
                            <TableCell><Code inline=true>"show"</Code></TableCell>
                            <TableCell><Code inline=true>"Signal<bool>"</Code></TableCell>
                            <TableCell>"-"</TableCell>
                            <TableCell>"Whether the overlay is currently shown."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"overlay_id"</Code></TableCell>
                            <TableCell><Code inline=true>"Oco<'static, str>"</Code></TableCell>
                            <TableCell>"-"</TableCell>
                            <TableCell>"The overlay ID from UseOverlayReturn."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"overlay_type"</Code></TableCell>
                            <TableCell><Code inline=true>"OverlayTriggerType"</Code></TableCell>
                            <TableCell>"-"</TableCell>
                            <TableCell>"The type of overlay (Dialog, Menu, Listbox, Tree, Grid)."</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            <h3 id="use-overlay-trigger-return" class="anchor">
                "UseOverlayTriggerReturn"
                <AnchorLink href="#use-overlay-trigger-return" description="Direct link to UseOverlayTriggerReturn"/>
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
                            <TableCell><Code inline=true>"props"</Code></TableCell>
                            <TableCell><Code inline=true>"UseOverlayTriggerProps"</Code></TableCell>
                            <TableCell>"ARIA props for the trigger (aria-haspopup, aria-expanded, aria-controls)."</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            <h3 id="use-overlay-position-input" class="anchor">
                "UseOverlayPositionInput"
                <AnchorLink href="#use-overlay-position-input" description="Direct link to UseOverlayPositionInput"/>
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
                            <TableCell><Code inline=true>"target"</Code></TableCell>
                            <TableCell><Code inline=true>"CapturedElement"</Code></TableCell>
                            <TableCell>"-"</TableCell>
                            <TableCell>"Captured element for the positioning target. The overlay element is captured internally via the returned props."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"placement_x"</Code></TableCell>
                            <TableCell><Code inline=true>"Signal<PlacementX>"</Code></TableCell>
                            <TableCell>"-"</TableCell>
                            <TableCell>"Horizontal placement (OuterLeft, Left, Center, Right, OuterRight)."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"placement_y"</Code></TableCell>
                            <TableCell><Code inline=true>"Signal<PlacementY>"</Code></TableCell>
                            <TableCell>"-"</TableCell>
                            <TableCell>"Vertical placement (Above, Top, Center, Bottom, Below)."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"writing_direction"</Code></TableCell>
                            <TableCell><Code inline=true>"Signal<WritingDirection>"</Code></TableCell>
                            <TableCell>"-"</TableCell>
                            <TableCell>"Writing direction for logical placement variants."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"offset"</Code></TableCell>
                            <TableCell><Code inline=true>"Signal<f64>"</Code></TableCell>
                            <TableCell><Code inline=true>"0.0"</Code></TableCell>
                            <TableCell>"Offset along the main axis (away from target)."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"cross_offset"</Code></TableCell>
                            <TableCell><Code inline=true>"Signal<f64>"</Code></TableCell>
                            <TableCell><Code inline=true>"0.0"</Code></TableCell>
                            <TableCell>"Offset along the cross axis."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"container_padding"</Code></TableCell>
                            <TableCell><Code inline=true>"Signal<f64>"</Code></TableCell>
                            <TableCell><Code inline=true>"12.0"</Code></TableCell>
                            <TableCell>"Minimum padding from viewport edges."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"should_flip"</Code></TableCell>
                            <TableCell><Code inline=true>"Signal<bool>"</Code></TableCell>
                            <TableCell><Code inline=true>"true"</Code></TableCell>
                            <TableCell>"Whether to flip placement when space is insufficient."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"max_height"</Code></TableCell>
                            <TableCell><Code inline=true>"Option<Signal<f64>>"</Code></TableCell>
                            <TableCell><Code inline=true>"None"</Code></TableCell>
                            <TableCell>"Optional maximum height override."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"is_open"</Code></TableCell>
                            <TableCell><Code inline=true>"Signal<bool>"</Code></TableCell>
                            <TableCell>"-"</TableCell>
                            <TableCell>"Whether the overlay is open (skips computation when false)."</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            <h3 id="use-overlay-position-return" class="anchor">
                "UseOverlayPositionReturn"
                <AnchorLink href="#use-overlay-position-return" description="Direct link to UseOverlayPositionReturn"/>
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
                            <TableCell><Code inline=true>"props"</Code></TableCell>
                            <TableCell><Code inline=true>"UseOverlayPositionProps"</Code></TableCell>
                            <TableCell>"CSS style props (position, z-index, top, left, max-height)."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"resolved_placement_x"</Code></TableCell>
                            <TableCell><Code inline=true>"Memo<PhysicalPlacementX>"</Code></TableCell>
                            <TableCell>"Resolved horizontal placement after flipping."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"resolved_placement_y"</Code></TableCell>
                            <TableCell><Code inline=true>"Memo<PlacementY>"</Code></TableCell>
                            <TableCell>"Resolved vertical placement after flipping."</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            <h2 id="accessibility" class="anchor">
                "Accessibility"
                <AnchorLink href="#accessibility" description="Direct link to accessibility"/>
            </h2>

            <ul>
                <li><strong>"Overlay trigger ARIA"</strong>
                    " — "<Code inline=true>"use_overlay_trigger"</Code>" sets "<Code inline=true>"aria-haspopup"</Code>", "
                    <Code inline=true>"aria-expanded"</Code>", and "<Code inline=true>"aria-controls"</Code>" on the trigger element."
                </li>
                <li><strong>"Unique overlay ID"</strong>
                    " — Auto-generated by "<Code inline=true>"use_overlay"</Code>" and passed to the trigger hook via "<Code inline=true>"overlay_id"</Code>"."
                </li>
                <li><strong>"Keyboard dismiss"</strong>
                    " — Escape closes the topmost overlay. Respects IME composition (no close while composing)."
                </li>
                <li><strong>"No built-in focus trapping"</strong>
                    " — Combine with the "<Code inline=true>"FocusScope"</Code>" atom when focus containment is needed."
                </li>
            </ul>

            <h2 id="features" class="anchor">
                "Features"
                <AnchorLink href="#features" description="Direct link to features"/>
            </h2>

            <ul>
                <li>"Escape key dismissal (topmost overlay only)"</li>
                <li>"Click outside detection with overlay stacking"</li>
                <li>"Blur dismissal (optional, bypasses topmost check)"</li>
                <li>"Filter callback for selective outside-interaction dismissal"</li>
                <li>"ARIA trigger attributes ("<Code inline=true>"aria-haspopup"</Code>", "<Code inline=true>"aria-expanded"</Code>", "<Code inline=true>"aria-controls"</Code>")"</li>
                <li>"CSS positioning with automatic viewport flipping"</li>
                <li>"Writing-direction-aware logical placement variants"</li>
                <li>"Configurable offset, cross-offset, and container padding"</li>
                <li>"Optional underlay element for pointer event capture"</li>
            </ul>

            <h2 id="related-hooks" class="anchor">
                "Related Hooks"
                <AnchorLink href="#related-hooks" description="Direct link to related hooks"/>
            </h2>

            <ul>
                <li><Code inline=true>"use_popover"</Code>" — Composition hook for positioned overlays with sensible defaults."</li>
                <li><Code inline=true>"use_modal"</Code>" / "<Code inline=true>"use_modal_backdrop"</Code>" — For modal dialog overlays with scroll prevention."</li>
                <li>"Tooltip hooks — For hover/focus tooltip overlays with warmup/cooldown."</li>
                <li><Code inline=true>"use_interact_outside"</Code>" — Internal outside-click detection used by "<Code inline=true>"use_overlay"</Code>"."</li>
                <li><Code inline=true>"FocusScope"</Code>" (atom) — Focus trapping and restoration."</li>
            </ul>

            <h2 id="deviations" class="anchor">
                "React Aria Deviations"
                <AnchorLink href="#deviations" description="Direct link to deviations"/>
            </h2>

            <h3>"use_overlay"</h3>
            <ul>
                <li><strong>"ID generation"</strong>": React Aria generates overlay IDs in "<Code inline=true>"useOverlayTrigger"</Code>
                    ". Leptonic generates them in "<Code inline=true>"use_overlay"</Code>" and passes them to the trigger hook."</li>
                <li><strong>"Element capture"</strong>": Uses "<Code inline=true>"ElementCaptureAttr"</Code>" (attribute spread pattern) instead of a React ref parameter."</li>
            </ul>

            <h3>"use_overlay_trigger"</h3>
            <ul>
                <li><strong>"No "<Code inline=true>"onPress"</Code></strong>": React Aria includes a press handler in trigger props. "
                    "Leptonic leaves press handling to the caller (e.g., via "<Code inline=true>"use_button"</Code>")."</li>
                <li><strong>"No "<Code inline=true>"overlayProps"</Code></strong>": React Aria returns overlay props with the generated ID from the trigger hook. "
                    "Leptonic returns the ID from "<Code inline=true>"use_overlay"</Code>" instead."</li>
            </ul>

            <h3>"use_overlay_position"</h3>
            <ul>
                <li><strong>"Separate "<Code inline=true>"PlacementX"</Code>"/"<Code inline=true>"PlacementY"</Code></strong>
                    ": React Aria uses a single "<Code inline=true>"Placement"</Code>" enum (e.g., "<Code inline=true>"\"top\""</Code>
                    ", "<Code inline=true>"\"bottom start\""</Code>"). Leptonic uses orthogonal X/Y enums for more flexible combinations."</li>
                <li><strong><Code inline=true>"position: fixed"</Code></strong>
                    ": React Aria uses "<Code inline=true>"position: absolute"</Code>" with containing-block detection. "
                    "Leptonic uses "<Code inline=true>"position: fixed"</Code>" since "<Code inline=true>"Portal"</Code>" appends to "<Code inline=true>"<body>"</Code>
                    ", avoiding ~200 lines of containing-block logic."</li>
                <li><strong>"Arrow positioning"</strong>": Not implemented ("<Code inline=true>"arrowSize"</Code>", "<Code inline=true>"arrowBoundaryOffset"</Code>")."</li>
                <li><strong>"Visual viewport handling"</strong>": iOS virtual keyboard adjustments not yet implemented."</li>
            </ul>

            <h2 id="see-also" class="anchor">
                "See Also"
                <AnchorLink href="#see-also" description="Direct link to section: See Also"/>
            </h2>

            <ul>
                <li><Link href=crate::routes::doc::Overlays.materialize()>"Overlays overview"</Link></li>
                <li><Link href=crate::routes::doc::Popover.materialize()>"Popover concept"</Link></li>
                <li><Link href=crate::routes::doc::Modal.materialize()>"Modal concept"</Link></li>
                <li><Link href=crate::routes::doc::interactions::UseInteractOutside.materialize()>"use_interact_outside"</Link></li>
            </ul>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "Overlay Hooks", link: "#overlay-hooks" },
                Toc::Leaf { title: "Hook Composition", link: "#hook-composition" },
                Toc::Leaf { title: "When to Use", link: "#when-to-use" },
                Toc::Leaf { title: "Interactive Demo", link: "#interactive-demo" },
                Toc::Leaf { title: "Positioning Demo", link: "#positioning-demo" },
                Toc::Leaf { title: "Dismiss Behavior", link: "#dismiss-behavior" },
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
