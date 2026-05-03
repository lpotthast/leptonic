use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::{
    popover_basic::BasicPopoverDemo, popover_non_modal::NonModalPopoverDemo,
    popover_placement::PlacementPopoverDemo,
};
use crate::pages::documentation::{article::Article, demo_shell::DemoShell, toc::Toc};

#[component]
pub fn PageUsePopoverHook() -> impl IntoView {
    view! {
        <Article>
            <h1 id="popover" class="anchor">
                "use_popover"
                <AnchorLink href="#popover" description="Direct link to article header"/>
            </h1>

            <p>"A composition hook that provides behavior and accessibility implementation for a popover component. "
               "A popover is an overlay element positioned relative to a trigger. "
               "See the "<Link href=crate::routes::doc::Popover.materialize()>"Popover overview"</Link>" for concept guidance."</p>

            <p>
                "Based on react-aria\u{2019}s "
                <LinkExt href="https://react-spectrum.adobe.com/react-aria/usePopover.html" target=LinkTarget::_Blank>
                    "usePopover"
                </LinkExt>
                "."
            </p>

            <h2 id="hook-composition" class="anchor">
                "Hook Composition"
                <AnchorLink href="#hook-composition" description="Direct link to hook composition"/>
            </h2>

            <p><code>"use_popover"</code>" is a composition hook that delegates to several lower-level primitives:"</p>

            <ul>
                <li><code>"use_overlay"</code>" - Dismiss handling (Escape key, click outside, blur) and overlay stacking (only topmost closes)"</li>
                <li><code>"use_overlay_position"</code>" - Positions the popover relative to the trigger element"</li>
                <li><code>"use_prevent_scroll"</code>" - Prevents page scrolling when the popover is open (unless non-modal)"</li>
            </ul>

            <h2 id="when-to-use" class="anchor">
                "When to Use"
                <AnchorLink href="#when-to-use" description="Direct link to when to use"/>
            </h2>

            <p><code>"use_popover"</code>" is a positioning and dismiss hook only. Choose the right hook based on your needs:"</p>

            <table style="width: 100%; border-collapse: collapse;">
                <thead>
                    <tr style="border-bottom: 2px solid var(--brand-color);">
                        <th style="text-align: left; padding: 0.5em;">"Use Case"</th>
                        <th style="text-align: left; padding: 0.5em;">"Hook(s)"</th>
                    </tr>
                </thead>
                <tbody>
                    <tr style="border-bottom: 1px solid #ccc;">
                        <td style="padding: 0.5em;">"Simple info popover, tooltip, profile card"</td>
                        <td style="padding: 0.5em;"><code>"use_popover"</code></td>
                    </tr>
                    <tr style="border-bottom: 1px solid #ccc;">
                        <td style="padding: 0.5em;">"Menu with keyboard navigation"</td>
                        <td style="padding: 0.5em;"><code>"use_menu"</code>" + "<code>"use_menu_trigger"</code>" + "<code>"use_menu_item"</code></td>
                    </tr>
                    <tr style="border-bottom: 1px solid #ccc;">
                        <td style="padding: 0.5em;">"Dropdown with selection"</td>
                        <td style="padding: 0.5em;"><code>"use_select"</code>" or "<code>"use_menu"</code>" hooks"</td>
                    </tr>
                    <tr style="border-bottom: 1px solid #ccc;">
                        <td style="padding: 0.5em;">"Autocomplete/combobox"</td>
                        <td style="padding: 0.5em;"><code>"use_combobox"</code></td>
                    </tr>
                    <tr style="border-bottom: 1px solid #ccc;">
                        <td style="padding: 0.5em;">"Modal dialog"</td>
                        <td style="padding: 0.5em;"><code>"use_modal"</code>" + "<code>"use_modal_backdrop"</code></td>
                    </tr>
                </tbody>
            </table>

            <p style="margin-top: 1em;">
                <strong>"Key distinction:"</strong>
                " For menus with arrow key navigation and ARIA menu roles, use the "
                <code>"use_menu"</code>" family of hooks instead of "<code>"use_popover"</code>"."
            </p>

            <h2 id="usage" class="anchor">
                "Usage"
                <AnchorLink href="#usage" description="Direct link to usage"/>
            </h2>

            <Code language=Language::Rust>
                {indoc!(r#"
                    use leptonic::hooks::{
                        use_popover, UsePopoverInput, UsePopoverReturn,
                        PlacementX, PlacementY,
                    };
                    use leptonic::utils::locale::WritingDirection;
                    use leptos::portal::Portal;

                    let (is_open, set_is_open) = signal(false);

                    let UsePopoverReturn {
                        props,
                        trigger_props,
                        underlay_props,
                        id,
                        resolved_placement_x,
                        resolved_placement_y,
                    } = use_popover(UsePopoverInput {
                        is_open: is_open.into(),
                        on_close: Callback::new(move |_| set_is_open.set(false)),
                        placement_x: Signal::derive(|| PlacementX::Center),
                        placement_y: Signal::derive(|| PlacementY::Below),
                        writing_direction: Signal::derive(|| WritingDirection::Ltr),
                        offset: 0.0.into(),
                        cross_offset: 0.0.into(),
                        container_padding: 12.0.into(),
                        should_flip: true.into(),
                        is_non_modal: false,
                        is_keyboard_dismiss_disabled: false,
                        should_close_on_interact_outside: None,
                    });

                    let trigger_attrs = StoredValue::new(trigger_props.into_attrs());
                    let popover_props = StoredValue::new(props.into_attrs());
                    let underlay_props = StoredValue::new(underlay_props.into_attrs());

                    view! {
                        <button
                            {..trigger_attrs.get_value()}
                            on:click=move |_| set_is_open.set(!is_open.get())
                        >
                            "Toggle Popover"
                        </button>

                        <Portal>
                            <Show when=move || is_open.get()>
                                // Underlay captures pointer events to close (modal only)
                                <div
                                    {..underlay_props.get_value()}
                                    style="position: fixed; inset: 0; z-index: 999;"
                                />
                                // Popover content — positioned automatically
                                <div
                                    {..popover_props.get_value()}
                                    style="z-index: 1000;"
                                >
                                    "Popover content"
                                </div>
                            </Show>
                        </Portal>
                    }
                "#)}
            </Code>

            <h2 id="interactive-demo" class="anchor">
                "Interactive Demo"
                <AnchorLink href="#interactive-demo" description="Direct link to interactive demo"/>
            </h2>

            <p>"Click the button to show a popover positioned below it:"</p>

            <DemoShell source=include_str!("demos/popover_basic.rs")>
                <BasicPopoverDemo />
            </DemoShell>

            <h2 id="positioning" class="anchor">
                "Positioning"
                <AnchorLink href="#positioning" description="Direct link to positioning"/>
            </h2>

            <p>"Popover placement is controlled by two orthogonal axes:"</p>

            <ul>
                <li><strong>"PlacementX"</strong>" (horizontal): "
                    <code>"OuterLeft"</code>", "
                    <code>"OuterStart"</code>", "
                    <code>"Start"</code>", "
                    <code>"Left"</code>", "
                    <code>"Center"</code>", "
                    <code>"Right"</code>", "
                    <code>"End"</code>", "
                    <code>"OuterEnd"</code>", "
                    <code>"OuterRight"</code>
                </li>
                <li><strong>"PlacementY"</strong>" (vertical): "
                    <code>"Above"</code>", "
                    <code>"Top"</code>", "
                    <code>"Center"</code>", "
                    <code>"Bottom"</code>", "
                    <code>"Below"</code>
                </li>
            </ul>

            <p>"The logical variants "<code>"Start"</code>", "<code>"End"</code>", "
               <code>"OuterStart"</code>", and "<code>"OuterEnd"</code>
               " resolve to physical directions based on the "<code>"writing_direction"</code>
               " signal. In LTR, "<code>"Start"</code>" maps to "<code>"Left"</code>
               "; in RTL it maps to "<code>"Right"</code>"."</p>

            <p>"Fine-tuning options:"</p>

            <ul>
                <li><strong><code>"offset"</code></strong>
                    " — Pushes the popover away from the trigger along the main axis. Default: "<code>"0.0"</code>"."
                </li>
                <li><strong><code>"cross_offset"</code></strong>
                    " — Shifts the popover along the cross axis. Default: "<code>"0.0"</code>"."
                </li>
                <li><strong><code>"container_padding"</code></strong>
                    " — Minimum distance from the viewport edge. Default: "<code>"12.0"</code>" px."
                </li>
                <li><strong><code>"should_flip"</code></strong>
                    " — When enabled (default), the popover flips to the opposite side if there is insufficient space."
                </li>
            </ul>

            <p>"After positioning, "<code>"resolved_placement_x"</code>" and "
               <code>"resolved_placement_y"</code>
               " reflect the actual placement (which may differ from the requested placement if flipping occurred). "
               "These are useful for positioning arrows or applying directional styles."</p>

            <h2 id="placement-demo" class="anchor">
                "Placement Demo"
                <AnchorLink href="#placement-demo" description="Direct link to placement demo"/>
            </h2>

            <p>"Use the radio buttons to change placement. Logical variants ("
               <code>"Start"</code>", "<code>"End"</code>", "<code>"OuterStart"</code>", "<code>"OuterEnd"</code>
               ") are also available for RTL support but not shown here:"</p>

            <DemoShell source=include_str!("demos/popover_placement.rs")>
                <PlacementPopoverDemo />
            </DemoShell>

            <h2 id="dismiss-behavior" class="anchor">
                "Dismiss Behavior"
                <AnchorLink href="#dismiss-behavior" description="Direct link to dismiss behavior"/>
            </h2>

            <p>"Dismiss handling is delegated to "<code>"use_overlay"</code>", which participates in a global overlay stack. "
               "Only the topmost overlay reacts to dismiss triggers:"</p>

            <ul>
                <li><strong>"Escape key"</strong>
                    " — Closes the topmost overlay. Respects IME composition (no close while composing). "
                    "Disable with "<code>"is_keyboard_dismiss_disabled: true"</code>"."
                </li>
                <li><strong>"Click outside"</strong>
                    " — Uses two-phase detection (pointerdown + click, both in the capture phase) to reliably detect outside interaction. "
                    "Only the topmost dismissable overlay closes."
                </li>
                <li><strong>"Blur"</strong>
                    " — The popover closes when focus moves outside it. This is always enabled for popovers."
                </li>
                <li><strong><code>"should_close_on_interact_outside"</code></strong>
                    " — An optional filter callback. Return "<code>"false"</code>" to prevent dismissal for specific outside elements."
                </li>
            </ul>

            <h2 id="modal-vs-non-modal" class="anchor">
                "Modal vs Non-Modal"
                <AnchorLink href="#modal-vs-non-modal" description="Direct link to modal vs non-modal"/>
            </h2>

            <p>"The "<code>"is_non_modal"</code>" flag controls two key behaviors:"</p>

            <table style="width: 100%; border-collapse: collapse;">
                <thead>
                    <tr style="border-bottom: 2px solid var(--brand-color);">
                        <th style="text-align: left; padding: 0.5em;">"Behavior"</th>
                        <th style="text-align: left; padding: 0.5em;">"Modal (default)"</th>
                        <th style="text-align: left; padding: 0.5em;">"Non-Modal"</th>
                    </tr>
                </thead>
                <tbody>
                    <tr style="border-bottom: 1px solid #ccc;">
                        <td style="padding: 0.5em;">"Scroll prevention"</td>
                        <td style="padding: 0.5em;">"Active (page scroll blocked)"</td>
                        <td style="padding: 0.5em;">"Disabled (page scrolls freely)"</td>
                    </tr>
                    <tr style="border-bottom: 1px solid #ccc;">
                        <td style="padding: 0.5em;">"Outside click dismissal"</td>
                        <td style="padding: 0.5em;">"Enabled (via overlay stack)"</td>
                        <td style="padding: 0.5em;">"Disabled"</td>
                    </tr>
                    <tr style="border-bottom: 1px solid #ccc;">
                        <td style="padding: 0.5em;">"Blur dismissal"</td>
                        <td style="padding: 0.5em;">"Always enabled"</td>
                        <td style="padding: 0.5em;">"Always enabled"</td>
                    </tr>
                    <tr style="border-bottom: 1px solid #ccc;">
                        <td style="padding: 0.5em;">"Underlay element"</td>
                        <td style="padding: 0.5em;">"Recommended"</td>
                        <td style="padding: 0.5em;">"Not needed"</td>
                    </tr>
                </tbody>
            </table>

            <h2 id="non-modal-demo" class="anchor">
                "Non-Modal Demo"
                <AnchorLink href="#non-modal-demo" description="Direct link to non-modal demo"/>
            </h2>

            <p>"A non-modal popover allows interaction with elements outside while open, "
               "and does not prevent page scrolling:"</p>

            <DemoShell source=include_str!("demos/popover_non_modal.rs")>
                <NonModalPopoverDemo />
            </DemoShell>

            <h2 id="underlay" class="anchor">
                "Underlay Element"
                <AnchorLink href="#underlay" description="Direct link to underlay element"/>
            </h2>

            <p>"The "<code>"underlay_props"</code>" return value provides props for an optional underlay element — a "
               <code>"position: fixed; inset: 0"</code>" div rendered behind the popover content."</p>

            <p>"The underlay's "<code>"on_pointerdown"</code>" handler works around a Firefox bug where text selection "
               "behind the popover could interfere with interactions. For modal popovers, include the underlay element. "
               "For non-modal popovers, skip it."</p>

            <Code language=Language::Rust>
                {indoc!(r#"
                    // Modal popover: include underlay
                    <Show when=move || is_open.get()>
                        <div
                            {..underlay_props.get_value()}
                            style="position: fixed; inset: 0; z-index: 999;"
                        />
                        <div {..popover_props.get_value()}>
                            "Popover content"
                        </div>
                    </Show>

                    // Non-modal popover: no underlay
                    <Show when=move || is_open.get()>
                        <div {..popover_props.get_value()}>
                            "Popover content"
                        </div>
                    </Show>
                "#)}
            </Code>

            <h2 id="api" class="anchor">
                "API"
                <AnchorLink href="#api" description="Direct link to API"/>
            </h2>

            <h3 id="input" class="anchor">
                "UsePopoverInput"
                <AnchorLink href="#input" description="Direct link to input"/>
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
                            <TableCell>"Whether the popover is currently open."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"on_close"</code></TableCell>
                            <TableCell><code>"Callback<()>"</code></TableCell>
                            <TableCell>"-"</TableCell>
                            <TableCell>"Called when the popover should close."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"placement_x"</code></TableCell>
                            <TableCell><code>"Signal<PlacementX>"</code></TableCell>
                            <TableCell>"-"</TableCell>
                            <TableCell>"Horizontal placement."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"placement_y"</code></TableCell>
                            <TableCell><code>"Signal<PlacementY>"</code></TableCell>
                            <TableCell>"-"</TableCell>
                            <TableCell>"Vertical placement."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"writing_direction"</code></TableCell>
                            <TableCell><code>"Signal<WritingDirection>"</code></TableCell>
                            <TableCell>"-"</TableCell>
                            <TableCell>"Writing direction for resolving logical placements (Start/End)."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"offset"</code></TableCell>
                            <TableCell><code>"Signal<f64>"</code></TableCell>
                            <TableCell><code>"0.0"</code></TableCell>
                            <TableCell>"Additional offset along the main axis."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"cross_offset"</code></TableCell>
                            <TableCell><code>"Signal<f64>"</code></TableCell>
                            <TableCell><code>"0.0"</code></TableCell>
                            <TableCell>"Additional offset along the cross axis."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"container_padding"</code></TableCell>
                            <TableCell><code>"Signal<f64>"</code></TableCell>
                            <TableCell><code>"12.0"</code></TableCell>
                            <TableCell>"Minimum padding from viewport edge."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"should_flip"</code></TableCell>
                            <TableCell><code>"Signal<bool>"</code></TableCell>
                            <TableCell><code>"true"</code></TableCell>
                            <TableCell>"Flip to opposite side when insufficient space."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"is_non_modal"</code></TableCell>
                            <TableCell><code>"bool"</code></TableCell>
                            <TableCell>"-"</TableCell>
                            <TableCell>"Non-modal popovers allow outside interaction."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"is_keyboard_dismiss_disabled"</code></TableCell>
                            <TableCell><code>"bool"</code></TableCell>
                            <TableCell>"-"</TableCell>
                            <TableCell>"Disable Escape key dismissal."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"should_close_on_interact_outside"</code></TableCell>
                            <TableCell><code>"Option<Callback<Element, bool>>"</code></TableCell>
                            <TableCell><code>"None"</code></TableCell>
                            <TableCell>"Optional filter for which outside interactions should close."</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            <h3 id="return" class="anchor">
                "UsePopoverReturn"
                <AnchorLink href="#return" description="Direct link to return"/>
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
                            <TableCell><code>"UsePopoverProps"</code></TableCell>
                            <TableCell>"Props for the popover element (overlay + positioning attrs)."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"trigger_props"</code></TableCell>
                            <TableCell><code>"UsePopoverTriggerProps"</code></TableCell>
                            <TableCell>"Props for the trigger element (element capture for positioning)."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"underlay_props"</code></TableCell>
                            <TableCell><code>"UsePopoverUnderlayProps"</code></TableCell>
                            <TableCell>"Props for an optional underlay element (pointerdown handler)."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"id"</code></TableCell>
                            <TableCell><code>"Oco<'static, str>"</code></TableCell>
                            <TableCell>"Unique overlay ID for ARIA."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"resolved_placement_x"</code></TableCell>
                            <TableCell><code>"Memo<PhysicalPlacementX>"</code></TableCell>
                            <TableCell>"Actual horizontal placement after flipping."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"resolved_placement_y"</code></TableCell>
                            <TableCell><code>"Memo<PlacementY>"</code></TableCell>
                            <TableCell>"Actual vertical placement after flipping."</TableCell>
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
                    " — Pair with "<code>"use_overlay_trigger"</code>" to get "<code>"aria-expanded"</code>", "
                    <code>"aria-controls"</code>", and "<code>"aria-haspopup"</code>" on the trigger element."
                </li>
                <li><strong>"Unique overlay ID"</strong>
                    " — The "<code>"id"</code>" returned by "<code>"use_popover"</code>" is automatically set on the popover element via "
                    <code>"props"</code>". Pass it to "<code>"use_overlay_trigger"</code>" as "<code>"overlay_id"</code>"."
                </li>
                <li><strong>"Keyboard dismiss"</strong>
                    " — Escape closes the popover, which is expected accessible behavior."
                </li>
                <li><strong>"Focus management"</strong>
                    " — For modal popovers, wrap content in a "<code>"FocusScope"</code>" atom with "
                    <code>"contain=true"</code>" to trap focus within the popover."
                </li>
                <li><strong>"Not aria-modal"</strong>
                    " — Unlike "<code>"use_modal"</code>", popovers do not set "<code>"aria-modal"</code>". "
                    "If you need a modal dialog, use the modal hooks instead."
                </li>
            </ul>

            <h2 id="features" class="anchor">
                "Features"
                <AnchorLink href="#features" description="Direct link to features"/>
            </h2>

            <ul>
                <li>"Automatic positioning relative to trigger element"</li>
                <li>"Escape key dismissal (topmost overlay only)"</li>
                <li>"Click outside detection (via overlay stack)"</li>
                <li>"Blur dismissal"</li>
                <li>"Optional underlay element"</li>
                <li>"Scroll prevention (automatic for modal popovers)"</li>
                <li>"Non-modal mode for allowing outside interaction"</li>
                <li>"Optional "<code>"should_close_on_interact_outside"</code>" filter callback"</li>
            </ul>

            <h2 id="related-hooks" class="anchor">
                "Related Hooks"
                <AnchorLink href="#related-hooks" description="Direct link to related hooks"/>
            </h2>

            <ul>
                <li><code>"use_overlay"</code>" / "<code>"use_overlay_trigger"</code>" / "<code>"use_overlay_position"</code>" — Lower-level primitives."</li>
                <li><code>"use_modal"</code>" / "<code>"use_modal_backdrop"</code>" — For modal dialogs."</li>
                <li><code>"use_menu"</code>" / "<code>"use_menu_trigger"</code>" — For menus with keyboard navigation."</li>
                <li><code>"FocusScope"</code>" (atom) — Focus trapping for modal popovers."</li>
            </ul>

            <h2 id="deviations" class="anchor">
                "React Aria Deviations"
                <AnchorLink href="#deviations" description="Direct link to deviations"/>
            </h2>

            <h3>"Omitted Features"</h3>
            <ul>
                <li><code>"arrowRef"</code>"/"<code>"arrowProps"</code>" - Arrow element positioning not built-in"</li>
                <li><code>"groupRef"</code>" - Submenu-style popover groups not implemented"</li>
                <li><code>"ariaHideOutside"</code>"/"<code>"keepVisible"</code>" - Hiding outside elements from assistive technology not implemented"</li>
            </ul>

            <h3>"API Differences"</h3>
            <ul>
                <li><code>"underlayProps"</code>" renamed to "<code>"underlay_props"</code>" following Rust naming conventions"</li>
            </ul>

            <h2 id="see-also" class="anchor">
                "See Also"
                <AnchorLink href="#see-also" description="Direct link to section: See Also"/>
            </h2>

            <ul>
                <li><Link href=crate::routes::doc::Popover.materialize()>"Popover overview"</Link></li>
                <li><Link href=crate::routes::doc::popover::Atom.materialize()>"Popover atom"</Link></li>
                <li><Link href=crate::routes::doc::popover::Component.materialize()>"Popover component"</Link></li>
                <li><Link href=crate::routes::doc::Overlays.materialize()>"Overlays overview"</Link></li>
                <li><Link href=crate::routes::doc::overlays::UseOverlay.materialize()>"use_overlay"</Link></li>
            </ul>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "use_popover", link: "#popover" },
                Toc::Leaf { title: "Hook Composition", link: "#hook-composition" },
                Toc::Leaf { title: "When to Use", link: "#when-to-use" },
                Toc::Leaf { title: "Usage", link: "#usage" },
                Toc::Leaf { title: "Interactive Demo", link: "#interactive-demo" },
                Toc::Leaf { title: "Positioning", link: "#positioning" },
                Toc::Leaf { title: "Placement Demo", link: "#placement-demo" },
                Toc::Leaf { title: "Dismiss Behavior", link: "#dismiss-behavior" },
                Toc::Leaf { title: "Modal vs Non-Modal", link: "#modal-vs-non-modal" },
                Toc::Leaf { title: "Non-Modal Demo", link: "#non-modal-demo" },
                Toc::Leaf { title: "Underlay Element", link: "#underlay" },
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
