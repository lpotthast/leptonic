use std::marker::PhantomData;

use indoc::indoc;
use leptonic::{
    components::prelude::*,
    hooks::{PlacementX, PlacementY, *},
    prelude::Size,
    utils::{
        aria::{AriaExpanded, AriaHasPopup},
        locale::WritingDirection,
    },
};
use leptos::{html, portal::Portal, prelude::*};

use crate::pages::documentation::{article::Article, doc_styles::*, toc::Toc};

#[component]
pub fn PageUseOverlay() -> impl IntoView {
    view! {
        <Article>
            <h1 id="overlay-hooks" class="anchor">
                "Overlay Hooks"
                <AnchorLink href="#overlay-hooks" description="Direct link to article header"/>
            </h1>

            <p>"Low-level overlay primitives for dismiss behavior, ARIA trigger attributes, and CSS positioning. "
               "Higher-level hooks like "<code>"use_popover"</code>" and "<code>"use_modal"</code>" compose these internally."</p>

            <h2 id="hook-composition" class="anchor">
                "Hook Composition"
                <AnchorLink href="#hook-composition" description="Direct link to hook composition"/>
            </h2>

            <p>"The overlay system is split into three single-responsibility hooks:"</p>

            <ul>
                <li><code>"use_overlay"</code>" — Dismiss behavior (Escape key, click outside, blur) and overlay stacking. "
                    "Only the topmost overlay closes when multiple are open."</li>
                <li><code>"use_overlay_trigger"</code>" — ARIA attributes on the trigger element: "
                    <code>"aria-haspopup"</code>", "<code>"aria-expanded"</code>", and "<code>"aria-controls"</code>"."</li>
                <li><code>"use_overlay_position"</code>" — CSS positioning of the overlay relative to a target element, "
                    "with automatic flipping and viewport containment."</li>
            </ul>

            <p>"You can use them individually or combine all three. "<code>"use_overlay"</code>" is the only required hook — "
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
                        <td style="padding: 0.5em;"><code>"use_overlay"</code>" + "<code>"use_overlay_trigger"</code>" + "<code>"use_overlay_position"</code></td>
                    </tr>
                    <tr style="border-bottom: 1px solid #ccc;">
                        <td style="padding: 0.5em;">"Popover with sensible defaults"</td>
                        <td style="padding: 0.5em;"><code>"use_popover"</code></td>
                    </tr>
                    <tr style="border-bottom: 1px solid #ccc;">
                        <td style="padding: 0.5em;">"Menu with keyboard navigation"</td>
                        <td style="padding: 0.5em;"><code>"use_menu"</code>" + "<code>"use_menu_trigger"</code></td>
                    </tr>
                    <tr style="border-bottom: 1px solid #ccc;">
                        <td style="padding: 0.5em;">"Modal dialog"</td>
                        <td style="padding: 0.5em;"><code>"use_modal"</code>" + "<code>"use_dialog"</code></td>
                    </tr>
                </tbody>
            </table>

            <h2 id="interactive-demo" class="anchor">
                "Interactive Demo"
                <AnchorLink href="#interactive-demo" description="Direct link to interactive demo"/>
            </h2>

            <p>"This demo uses only "<code>"use_overlay"</code>" to show a fixed-position panel with Escape key and click-outside dismiss:"</p>

            <BasicOverlayDemo />

            <Code>
                {indoc!(r#"
                    let (is_open, set_is_open) = signal(false);

                    let UseOverlayReturn { props: overlay_props, underlay_props: _, id } =
                        use_overlay(UseOverlayInput {
                            is_open: is_open.into(),
                            on_close: Callback::new(move |()| set_is_open.set(false)),
                            is_dismissable: true,
                            should_close_on_blur: false,
                            is_keyboard_dismiss_disabled: false,
                            should_close_on_interact_outside: None,
                        });

                    view! {
                        <button on:click=move |_| set_is_open.set(!is_open.get())>
                            "Toggle Overlay"
                        </button>

                        <Portal>
                            <Show when=move || is_open.get()>
                                <div {..overlay_props.into_attrs()}>
                                    "Press Escape or click outside to close"
                                </div>
                            </Show>
                        </Portal>
                    }
                "#)}
            </Code>

            <h2 id="positioning-demo" class="anchor">
                "Positioning Demo"
                <AnchorLink href="#positioning-demo" description="Direct link to positioning demo"/>
            </h2>

            <p>"This demo combines all three hooks: "<code>"use_overlay"</code>" for dismiss behavior, "
               <code>"use_overlay_trigger"</code>" for ARIA attributes, and "<code>"use_overlay_position"</code>" for placement. "
               "Use the radio buttons to change the placement:"</p>

            <PositioningDemo />

            <Code>
                {indoc!(r#"
                    let trigger_el: NodeRef<html::Div> = NodeRef::new();
                    let overlay_el: NodeRef<html::Div> = NodeRef::new();
                    let (is_open, set_is_open) = signal(false);

                    // 1. Dismiss behavior + overlay stacking
                    let UseOverlayReturn { props: overlay_props, underlay_props: _, id } =
                        use_overlay(UseOverlayInput {
                            is_open: is_open.into(),
                            on_close: Callback::new(move |()| set_is_open.set(false)),
                            is_dismissable: true,
                            should_close_on_blur: false,
                            is_keyboard_dismiss_disabled: false,
                            should_close_on_interact_outside: None,
                        });

                    // 2. ARIA attributes for the trigger
                    let UseOverlayTriggerReturn { props: trigger_props } =
                        use_overlay_trigger(UseOverlayTriggerInput {
                            show: is_open.into(),
                            overlay_id: id,
                            overlay_type: OverlayTriggerType::Dialog,
                        });

                    // 3. CSS positioning relative to the trigger
                    let UseOverlayPositionReturn { props: pos_props, .. } =
                        use_overlay_position(UseOverlayPositionInput {
                            overlay: overlay_el,
                            target: trigger_el,
                            placement_x: Signal::derive(|| PlacementX::Center),
                            placement_y: Signal::derive(|| PlacementY::Below),
                            writing_direction: Signal::derive(|| WritingDirection::Ltr),
                            offset: 0.0.into(),
                            cross_offset: 0.0.into(),
                            container_padding: 12.0.into(),
                            should_flip: true.into(),
                            max_height: None,
                            is_open: is_open.into(),
                            phantom_data: PhantomData,
                        });

                    view! {
                        <div {..trigger_props.into_attrs()} node_ref=trigger_el>
                            "Trigger"
                        </div>

                        <Portal>
                            <Show when=move || is_open.get()>
                                <div
                                    {..overlay_props.into_attrs()}
                                    {..pos_props.into_attrs()}
                                    node_ref=overlay_el
                                >
                                    "Positioned overlay content"
                                </div>
                            </Show>
                        </Portal>
                    }
                "#)}
            </Code>

            <h2 id="dismiss-behavior" class="anchor">
                "Dismiss Behavior"
                <AnchorLink href="#dismiss-behavior" description="Direct link to dismiss behavior"/>
            </h2>

            <p><code>"use_overlay"</code>" provides three independent dismiss mechanisms:"</p>

            <h3>"Escape Key"</h3>
            <p>"When the user presses Escape, the topmost overlay closes. The event is stopped and default-prevented "
               "so it does not propagate to outer overlays. Disable with "<code>"is_keyboard_dismiss_disabled: true"</code>"."</p>

            <h3>"Click Outside"</h3>
            <p>"Enabled via "<code>"is_dismissable: true"</code>". When the user clicks outside the overlay, "
               "only the topmost overlay closes. A filter callback "<code>"should_close_on_interact_outside"</code>" "
               "lets you prevent dismissal for specific elements (e.g., a trigger button that should toggle instead)."</p>

            <h3>"Blur"</h3>
            <p>"Enabled via "<code>"should_close_on_blur: true"</code>". When focus leaves the overlay, it closes. "
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
                            <TableCell><code>"is_open"</code></TableCell>
                            <TableCell><code>"Signal<bool>"</code></TableCell>
                            <TableCell>"-"</TableCell>
                            <TableCell>"Whether the overlay is currently open."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"on_close"</code></TableCell>
                            <TableCell><code>"Callback<()>"</code></TableCell>
                            <TableCell>"-"</TableCell>
                            <TableCell>"Called when the overlay should close."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"is_dismissable"</code></TableCell>
                            <TableCell><code>"bool"</code></TableCell>
                            <TableCell>"-"</TableCell>
                            <TableCell>"Whether to close when the user interacts outside the overlay."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"should_close_on_blur"</code></TableCell>
                            <TableCell><code>"bool"</code></TableCell>
                            <TableCell>"-"</TableCell>
                            <TableCell>"Whether to close when focus leaves the overlay."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"is_keyboard_dismiss_disabled"</code></TableCell>
                            <TableCell><code>"bool"</code></TableCell>
                            <TableCell>"-"</TableCell>
                            <TableCell>"Whether to disable Escape key dismissal."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"should_close_on_interact_outside"</code></TableCell>
                            <TableCell><code>"Option<Callback<Element, bool>>"</code></TableCell>
                            <TableCell><code>"None"</code></TableCell>
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
                            <TableCell><code>"props"</code></TableCell>
                            <TableCell><code>"UseOverlayProps"</code></TableCell>
                            <TableCell>"Props for the overlay element (id, keydown, focus handlers)."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"underlay_props"</code></TableCell>
                            <TableCell><code>"UseOverlayUnderlayProps"</code></TableCell>
                            <TableCell>"Props for an optional underlay element (pointerdown handler)."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"id"</code></TableCell>
                            <TableCell><code>"Oco<'static, str>"</code></TableCell>
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
                            <TableCell><code>"show"</code></TableCell>
                            <TableCell><code>"Signal<bool>"</code></TableCell>
                            <TableCell>"-"</TableCell>
                            <TableCell>"Whether the overlay is currently shown."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"overlay_id"</code></TableCell>
                            <TableCell><code>"Oco<'static, str>"</code></TableCell>
                            <TableCell>"-"</TableCell>
                            <TableCell>"The overlay ID from UseOverlayReturn."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"overlay_type"</code></TableCell>
                            <TableCell><code>"OverlayTriggerType"</code></TableCell>
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
                            <TableCell><code>"props"</code></TableCell>
                            <TableCell><code>"UseOverlayTriggerProps"</code></TableCell>
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
                            <TableCell><code>"overlay"</code></TableCell>
                            <TableCell><code>"Overlay"</code></TableCell>
                            <TableCell>"-"</TableCell>
                            <TableCell>"Element ref for the overlay."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"target"</code></TableCell>
                            <TableCell><code>"Target"</code></TableCell>
                            <TableCell>"-"</TableCell>
                            <TableCell>"Element ref for the positioning target."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"placement_x"</code></TableCell>
                            <TableCell><code>"Signal<PlacementX>"</code></TableCell>
                            <TableCell>"-"</TableCell>
                            <TableCell>"Horizontal placement (OuterLeft, Left, Center, Right, OuterRight)."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"placement_y"</code></TableCell>
                            <TableCell><code>"Signal<PlacementY>"</code></TableCell>
                            <TableCell>"-"</TableCell>
                            <TableCell>"Vertical placement (Above, Top, Center, Bottom, Below)."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"writing_direction"</code></TableCell>
                            <TableCell><code>"Signal<WritingDirection>"</code></TableCell>
                            <TableCell>"-"</TableCell>
                            <TableCell>"Writing direction for logical placement variants."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"offset"</code></TableCell>
                            <TableCell><code>"Signal<f64>"</code></TableCell>
                            <TableCell><code>"0.0"</code></TableCell>
                            <TableCell>"Offset along the main axis (away from target)."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"cross_offset"</code></TableCell>
                            <TableCell><code>"Signal<f64>"</code></TableCell>
                            <TableCell><code>"0.0"</code></TableCell>
                            <TableCell>"Offset along the cross axis."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"container_padding"</code></TableCell>
                            <TableCell><code>"Signal<f64>"</code></TableCell>
                            <TableCell><code>"12.0"</code></TableCell>
                            <TableCell>"Minimum padding from viewport edges."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"should_flip"</code></TableCell>
                            <TableCell><code>"Signal<bool>"</code></TableCell>
                            <TableCell><code>"true"</code></TableCell>
                            <TableCell>"Whether to flip placement when space is insufficient."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"max_height"</code></TableCell>
                            <TableCell><code>"Option<Signal<f64>>"</code></TableCell>
                            <TableCell><code>"None"</code></TableCell>
                            <TableCell>"Optional maximum height override."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"is_open"</code></TableCell>
                            <TableCell><code>"Signal<bool>"</code></TableCell>
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
                            <TableCell><code>"props"</code></TableCell>
                            <TableCell><code>"UseOverlayPositionProps"</code></TableCell>
                            <TableCell>"CSS style props (position, z-index, top, left, max-height)."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"resolved_placement_x"</code></TableCell>
                            <TableCell><code>"Memo<PhysicalPlacementX>"</code></TableCell>
                            <TableCell>"Resolved horizontal placement after flipping."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"resolved_placement_y"</code></TableCell>
                            <TableCell><code>"Memo<PlacementY>"</code></TableCell>
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
                    " — "<code>"use_overlay_trigger"</code>" sets "<code>"aria-haspopup"</code>", "
                    <code>"aria-expanded"</code>", and "<code>"aria-controls"</code>" on the trigger element."
                </li>
                <li><strong>"Unique overlay ID"</strong>
                    " — Auto-generated by "<code>"use_overlay"</code>" and passed to the trigger hook via "<code>"overlay_id"</code>"."
                </li>
                <li><strong>"Keyboard dismiss"</strong>
                    " — Escape closes the topmost overlay. Respects IME composition (no close while composing)."
                </li>
                <li><strong>"No built-in focus trapping"</strong>
                    " — Combine with the "<code>"FocusScope"</code>" atom when focus containment is needed."
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
                <li>"ARIA trigger attributes ("<code>"aria-haspopup"</code>", "<code>"aria-expanded"</code>", "<code>"aria-controls"</code>")"</li>
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
                <li><code>"use_popover"</code>" — Composition hook for positioned overlays with sensible defaults."</li>
                <li><code>"use_modal"</code>" / "<code>"use_modal_backdrop"</code>" — For modal dialog overlays with scroll prevention."</li>
                <li>"Tooltip hooks — For hover/focus tooltip overlays with warmup/cooldown."</li>
                <li><code>"use_interact_outside"</code>" — Internal outside-click detection used by "<code>"use_overlay"</code>"."</li>
                <li><code>"FocusScope"</code>" (atom) — Focus trapping and restoration."</li>
            </ul>

            <h2 id="deviations" class="anchor">
                "React Aria Deviations"
                <AnchorLink href="#deviations" description="Direct link to deviations"/>
            </h2>

            <h3>"use_overlay"</h3>
            <ul>
                <li><strong>"ID generation"</strong>": React Aria generates overlay IDs in "<code>"useOverlayTrigger"</code>
                    ". Leptonic generates them in "<code>"use_overlay"</code>" and passes them to the trigger hook."</li>
                <li><strong>"Element capture"</strong>": Uses "<code>"ElementCaptureAttr"</code>" (attribute spread pattern) instead of a React ref parameter."</li>
            </ul>

            <h3>"use_overlay_trigger"</h3>
            <ul>
                <li><strong>"No "<code>"onPress"</code></strong>": React Aria includes a press handler in trigger props. "
                    "Leptonic leaves press handling to the caller (e.g., via "<code>"use_button"</code>")."</li>
                <li><strong>"No "<code>"overlayProps"</code></strong>": React Aria returns overlay props with the generated ID from the trigger hook. "
                    "Leptonic returns the ID from "<code>"use_overlay"</code>" instead."</li>
            </ul>

            <h3>"use_overlay_position"</h3>
            <ul>
                <li><strong>"Separate "<code>"PlacementX"</code>"/"<code>"PlacementY"</code></strong>
                    ": React Aria uses a single "<code>"Placement"</code>" enum (e.g., "<code>"\"top\""</code>
                    ", "<code>"\"bottom start\""</code>"). Leptonic uses orthogonal X/Y enums for more flexible combinations."</li>
                <li><strong><code>"position: fixed"</code></strong>
                    ": React Aria uses "<code>"position: absolute"</code>" with containing-block detection. "
                    "Leptonic uses "<code>"position: fixed"</code>" since "<code>"Portal"</code>" appends to "<code>"<body>"</code>
                    ", avoiding ~200 lines of containing-block logic."</li>
                <li><strong>"Arrow positioning"</strong>": Not implemented ("<code>"arrowSize"</code>", "<code>"arrowBoundaryOffset"</code>")."</li>
                <li><strong>"Visual viewport handling"</strong>": iOS virtual keyboard adjustments not yet implemented."</li>
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
            ]
        }/>
    }
}

/// Basic overlay demo: use_overlay only, with Escape and click-outside dismiss.
#[component]
fn BasicOverlayDemo() -> impl IntoView {
    let (is_open, set_is_open) = signal(false);

    let UseOverlayReturn {
        props: overlay_props,
        underlay_props: _,
        id: _,
    } = use_overlay(UseOverlayInput {
        is_open: is_open.into(),
        on_close: Callback::new(move |()| set_is_open.set(false)),
        is_dismissable: true,
        should_close_on_blur: false,
        is_keyboard_dismiss_disabled: false,
        should_close_on_interact_outside: None,
    });

    let overlay_attrs = StoredValue::new(overlay_props.into_attrs());

    view! {
        <div style=demo_flex_center()>
            <button
                on:click=move |_| set_is_open.set(!is_open.get())
                style=demo_button_primary()
            >
                {move || if is_open.get() { "Close Overlay" } else { "Open Overlay" }}
            </button>
        </div>

        <Portal>
            <Show when=move || is_open.get()>
                <div
                    {..overlay_attrs.get_value()}
                    style=overlay_panel()
                >
                    <h4 style="margin: 0 0 0.5em 0; color: #333;">"Overlay"</h4>
                    <p style="margin: 0; color: #666;">
                        "Press Escape or click outside to close this overlay."
                    </p>
                </div>
            </Show>
        </Portal>
    }
}

/// Positioning demo: all three hooks combined with placement controls.
#[component]
fn PositioningDemo() -> impl IntoView {
    let (selected_placement_x, set_selected_placement_x) = signal(PlacementX::Right);
    let (selected_placement_y, set_selected_placement_y) = signal(PlacementY::Above);

    let trigger_el: NodeRef<html::Div> = NodeRef::new();
    let overlay_el: NodeRef<html::Div> = NodeRef::new();

    let (is_open, set_is_open) = signal(false);

    let UseOverlayReturn {
        props: overlay_props,
        underlay_props: _,
        id,
    } = use_overlay(UseOverlayInput {
        is_open: is_open.into(),
        on_close: Callback::new(move |()| set_is_open.set(false)),
        is_dismissable: true,
        should_close_on_blur: false,
        is_keyboard_dismiss_disabled: false,
        should_close_on_interact_outside: None,
    });
    let overlay_attrs = StoredValue::new(overlay_props.into_attrs());

    let UseOverlayTriggerReturn {
        props: trigger_props,
    } = use_overlay_trigger(UseOverlayTriggerInput {
        show: is_open.into(),
        overlay_id: id,
        overlay_type: OverlayTriggerType::Dialog,
    });
    let trigger_attrs = StoredValue::new(trigger_props.into_attrs());

    let UseOverlayPositionReturn {
        props: overlay_pos_props,
        resolved_placement_x: _,
        resolved_placement_y: _,
    } = use_overlay_position(UseOverlayPositionInput {
        overlay: overlay_el,
        target: trigger_el,
        placement_y: selected_placement_y.into(),
        placement_x: selected_placement_x.into(),
        writing_direction: WritingDirection::Ltr.into(),
        offset: 0.0.into(),
        cross_offset: 0.0.into(),
        container_padding: 12.0.into(),
        should_flip: true.into(),
        max_height: None,
        is_open: is_open.into(),
        phantom_data: PhantomData,
    });
    let overlay_pos_attrs = StoredValue::new(overlay_pos_props.into_attrs());

    let UseButtonReturn {
        props: btn_props,
        is_hovered: _,
        is_pressed: _,
        is_focus_visible: _,
    } = use_button(UseButtonInput {
        disabled: false.into(),
        aria_haspopup: AriaHasPopup::default().into(),
        aria_expanded: AriaExpanded::default().into(),
        use_press_input: UsePressInput {
            disabled: false.into(),
            force_prevent_default: false,
            force_propagation: false,
            allow_text_selection_on_press: false,
            should_cancel_on_pointer_exit: false,
            prevent_focus_on_press: false,
            force_is_pressed: None,
            on_press: Callback::new(move |_e| {
                set_is_open.set(!is_open.get_untracked());
            }),
            on_press_up: None,
            on_press_start: None,
            on_press_end: None,
            on_press_change: None,
            on_double_press: None,
            on_long_press_start: None,
            on_long_press: None,
            on_long_press_end: None,
            long_press_threshold: None,
            long_press_accessibility_description: None,
        },
        use_hover_input: UseHoverInput {
            disabled: false.into(),
            on_hover_start: None,
            on_hover_end: None,
            on_hover_change: None,
        },
        use_focus_ring_input: UseFocusRingInput {
            disabled: false.into(),
            within: false,
            auto_focus: false,
            is_text_input: false,
            on_focus: None,
            on_blur: None,
            on_focus_change: None,
        },
    });
    let btn_attrs = StoredValue::new(btn_props.into_attrs());

    view! {
        <Grid gap=Size::Em(0.5) attr:style="margin-bottom: 1em;">
            <Row>
                <Col xs=6 attr:style="
                    border: 0.1em solid lightgrey;
                    border-radius: 0.25em;
                    padding: 0.5em;
                ">
                    <strong>"Horizontal"</strong>
                    <RadioGroup attr:style="display: flex; flex-direction: column; gap: 0.2em; width: 100%; margin-top: 0.5em;">
                        <FormControl attr:style="display: flex; flex-direction: row; align-items: center;">
                            <Radio
                                checked=Signal::derive(move || selected_placement_x.get() == PlacementX::OuterLeft)
                                set_checked=move |checked| { if checked { set_selected_placement_x.set(PlacementX::OuterLeft)} }
                            />
                            <Label attr:style="margin-left: 0.25em;">"OuterLeft"</Label>
                        </FormControl>
                        <FormControl attr:style="display: flex; flex-direction: row; align-items: center;">
                            <Radio
                                checked=Signal::derive(move || selected_placement_x.get() == PlacementX::Left)
                                set_checked=move |checked| { if checked { set_selected_placement_x.set(PlacementX::Left)} }
                            />
                            <Label attr:style="margin-left: 0.25em;">"Left"</Label>
                        </FormControl>
                        <FormControl attr:style="display: flex; flex-direction: row; align-items: center;">
                            <Radio
                                checked=Signal::derive(move || selected_placement_x.get() == PlacementX::Center)
                                set_checked=move |checked| { if checked { set_selected_placement_x.set(PlacementX::Center)} }
                            />
                            <Label attr:style="margin-left: 0.25em;">"Center"</Label>
                        </FormControl>
                        <FormControl attr:style="display: flex; flex-direction: row; align-items: center;">
                            <Radio
                                checked=Signal::derive(move || selected_placement_x.get() == PlacementX::Right)
                                set_checked=move |checked| { if checked { set_selected_placement_x.set(PlacementX::Right)} }
                            />
                            <Label attr:style="margin-left: 0.25em;">"Right"</Label>
                        </FormControl>
                        <FormControl attr:style="display: flex; flex-direction: row; align-items: center;">
                            <Radio
                                checked=Signal::derive(move || selected_placement_x.get() == PlacementX::OuterRight)
                                set_checked=move |checked| { if checked { set_selected_placement_x.set(PlacementX::OuterRight)} }
                            />
                            <Label attr:style="margin-left: 0.25em;">"OuterRight"</Label>
                        </FormControl>
                    </RadioGroup>
                </Col>
                <Col xs=6 attr:style="
                    border: 0.1em solid lightgrey;
                    border-radius: 0.25em;
                    padding: 0.5em;
                ">
                    <strong>"Vertical"</strong>
                    <RadioGroup attr:style="display: flex; flex-direction: column; gap: 0.2em; width: 100%; margin-top: 0.5em;">
                        <FormControl attr:style="display: flex; flex-direction: row; align-items: center;">
                            <Radio
                                checked=Signal::derive(move || selected_placement_y.get() == PlacementY::Above)
                                set_checked=move |checked| { if checked { set_selected_placement_y.set(PlacementY::Above)} }
                            />
                            <Label attr:style="margin-left: 0.25em;">"Above"</Label>
                        </FormControl>
                        <FormControl attr:style="display: flex; flex-direction: row; align-items: center;">
                            <Radio
                                checked=Signal::derive(move || selected_placement_y.get() == PlacementY::Top)
                                set_checked=move |checked| { if checked { set_selected_placement_y.set(PlacementY::Top)} }
                            />
                            <Label attr:style="margin-left: 0.25em;">"Top"</Label>
                        </FormControl>
                        <FormControl attr:style="display: flex; flex-direction: row; align-items: center;">
                            <Radio
                                checked=Signal::derive(move || selected_placement_y.get() == PlacementY::Center)
                                set_checked=move |checked| { if checked { set_selected_placement_y.set(PlacementY::Center)} }
                            />
                            <Label attr:style="margin-left: 0.25em;">"Center"</Label>
                        </FormControl>
                        <FormControl attr:style="display: flex; flex-direction: row; align-items: center;">
                            <Radio
                                checked=Signal::derive(move || selected_placement_y.get() == PlacementY::Bottom)
                                set_checked=move |checked| { if checked { set_selected_placement_y.set(PlacementY::Bottom)} }
                            />
                            <Label attr:style="margin-left: 0.25em;">"Bottom"</Label>
                        </FormControl>
                        <FormControl attr:style="display: flex; flex-direction: row; align-items: center;">
                            <Radio
                                checked=Signal::derive(move || selected_placement_y.get() == PlacementY::Below)
                                set_checked=move |checked| { if checked { set_selected_placement_y.set(PlacementY::Below)} }
                            />
                            <Label attr:style="margin-left: 0.25em;">"Below"</Label>
                        </FormControl>
                    </RadioGroup>
                </Col>
            </Row>
        </Grid>

        <div style="display: flex; width: 100%; height: 20em; justify-content: center; align-items: center;">
            <div
                {..trigger_attrs.get_value()}
                {..btn_attrs.get_value()}
                node_ref=trigger_el
                style="
                    display: inline-flex;
                    border: 0.1em solid green;
                    padding: 0.5em;
                    cursor: pointer;
                    width: 7em;
                    height: 7em;
                    justify-content: center;
                    align-items: center;
                "
            >
                "Press me"
            </div>
        </div>

        <Portal>
            <Show when=move || is_open.get()>
                <div
                    {..overlay_attrs.get_value()}
                    {..overlay_pos_attrs.get_value()}
                    node_ref=overlay_el
                    style="
                        background-color: #0009;
                        color: white;
                        padding: 1em;
                        border-radius: 0.25em;
                    "
                >
                    {move || format!("{:?} / {:?}", selected_placement_x.get(), selected_placement_y.get())}
                </div>
            </Show>
        </Portal>
    }
}
