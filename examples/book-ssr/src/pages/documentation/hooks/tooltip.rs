use indoc::indoc;
use leptonic::{components::prelude::*, hooks::*, utils::{locale::WritingDirection, CapturedElement}};
use leptos::{portal::Portal, prelude::*};

use crate::pages::documentation::{article::Article, doc_styles::*, toc::Toc};

#[allow(clippy::too_many_lines)]
#[component]
pub fn PageUseTooltipHook() -> impl IntoView {
    view! {
        <Article>
            <h1 id="tooltip" class="anchor">
                "Tooltip Hooks"
                <AnchorLink href="#tooltip" description="Direct link to article header"/>
            </h1>

            <p>"Hooks for creating accessible tooltips with hover/focus triggers, warmup/cooldown delays, hover-to-keep-open behavior, and proper ARIA associations. "
               "The tooltip system is built from three composable hooks."</p>

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

            <TooltipDemo />

            <Code>
                {indoc!(r#"
                    use leptonic::hooks::{
                        use_tooltip_trigger_state, UseTooltipTriggerStateInput,
                        use_tooltip_trigger, UseTooltipTriggerInput,
                        use_tooltip, UseTooltipInput,
                        use_overlay_position, UseOverlayPositionInput,
                        PlacementX, PlacementY,
                    };
                    use leptonic::utils::{locale::WritingDirection, CapturedElement};
                    use leptos::portal::Portal;

                    let target_element = CapturedElement::new();

                    let state = use_tooltip_trigger_state(UseTooltipTriggerStateInput {
                        delay: 300,
                        close_delay: 100,
                        ..Default::default()
                    });
                    let trigger = use_tooltip_trigger(UseTooltipTriggerInput::default(), state);
                    let tooltip = use_tooltip(UseTooltipInput {
                        disabled: Signal::derive(|| false),
                        state: Some(state),
                        on_open: None,
                        on_close: None,
                    });

                    // The overlay element is captured internally by use_overlay_position.
                    let position = use_overlay_position(UseOverlayPositionInput {
                        target: target_element,
                        placement_x: Signal::derive(|| PlacementX::Center),
                        placement_y: Signal::derive(|| PlacementY::Above),
                        writing_direction: Signal::derive(|| WritingDirection::Ltr),
                        offset: 4.0.into(),
                        cross_offset: 0.0.into(),
                        container_padding: 12.0.into(),
                        should_flip: true.into(),
                        max_height: None,
                        is_open: trigger.is_open,
                    });

                    let target_capture = target_element.attr();
                    let position_attrs = position.props.into_attrs();

                    view! {
                        <button
                            {..target_capture}
                            {..trigger.trigger_props.into_attrs()}
                        >
                            "Hover me"
                        </button>
                        <Portal>
                            <Show when=move || trigger.is_open.get()>
                                <div
                                    {..position_attrs.clone()}
                                    {..tooltip.props.into_attrs()}
                                    id=trigger.tooltip_props.id.clone()
                                    role=trigger.tooltip_props.role
                                >
                                    "Tooltip content"
                                </div>
                            </Show>
                        </Portal>
                    }
                "#)}
            </Code>

            <h2 id="hover-to-keep-open" class="anchor">
                "Hover-to-Keep-Open"
                <AnchorLink href="#hover-to-keep-open" description="Direct link to hover-to-keep-open"/>
            </h2>

            <p>"The "<code>"use_tooltip"</code>" hook adds pointer event handlers to the tooltip content element. "
               "When the user moves their cursor from the trigger to the tooltip, it stays open. "
               "Without this hook, the tooltip closes as soon as the cursor leaves the trigger."</p>

            <p>"Compare the two buttons below:"</p>

            <HoverToKeepOpenDemo />

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

            <PositioningDemo />

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
            ]
        }/>
    }
}

/// Simple tooltip demo using all three hooks
#[component]
fn TooltipDemo() -> impl IntoView {
    let target_element = CapturedElement::new();

    let state = use_tooltip_trigger_state(UseTooltipTriggerStateInput {
        delay: 300,
        close_delay: 100,
        ..Default::default()
    });
    let trigger = use_tooltip_trigger(UseTooltipTriggerInput::default(), state);
    let tooltip = use_tooltip(UseTooltipInput {
        disabled: Signal::derive(|| false),
        state: Some(state),
        on_open: None,
        on_close: None,
    });

    let position = use_overlay_position(UseOverlayPositionInput {
        target: target_element,
        placement_x: Signal::derive(|| PlacementX::Center),
        placement_y: Signal::derive(|| PlacementY::Above),
        writing_direction: Signal::derive(|| WritingDirection::Ltr),
        offset: 4.0.into(),
        cross_offset: 0.0.into(),
        container_padding: 12.0.into(),
        should_flip: true.into(),
        max_height: None,
        is_open: trigger.is_open,
    });

    let is_open = trigger.is_open;
    let tooltip_id = trigger.tooltip_props.id.clone();
    let target_capture = target_element.attr();
    let position_attrs = position.props.into_attrs();
    let tooltip_attrs = tooltip.props.into_attrs();

    view! {
        <div style="margin: 3em 0; display: flex; justify-content: center;">
            <button
                {..target_capture}
                style=demo_button_primary()
                {..trigger.trigger_props.into_attrs()}
            >
                "Hover me"
            </button>

            <Portal>
                {
                    let tooltip_id = tooltip_id.clone();
                    let position_attrs = position_attrs.clone();
                    let tooltip_attrs = tooltip_attrs.clone();
                    view! {
                        <Show when=move || is_open.get()>
                            <div
                                {..position_attrs.clone()}
                                {..tooltip_attrs.clone()}
                                id=tooltip_id.clone()
                                role="tooltip"
                                class="tooltip-demo"
                            >
                                "This is a tooltip!"
                            </div>
                        </Show>
                    }
                }
            </Portal>
        </div>
    }
}

/// Demo comparing tooltips with and without use_tooltip
#[component]
fn HoverToKeepOpenDemo() -> impl IntoView {
    view! {
        <div style="display: flex; gap: 3em; justify-content: center; margin: 2em 0;">
            <div style="text-align: center;">
                <WithoutUseTooltipDemo />
                <p style="margin-top: 0.5em; font-size: 0.85em; color: #888;">"Without use_tooltip"</p>
            </div>
            <div style="text-align: center;">
                <WithUseTooltipDemo />
                <p style="margin-top: 0.5em; font-size: 0.85em; color: #888;">"With use_tooltip"</p>
            </div>
        </div>
    }
}

/// Tooltip without use_tooltip — closes when moving cursor to tooltip content
#[component]
fn WithoutUseTooltipDemo() -> impl IntoView {
    let target_element = CapturedElement::new();

    let state = use_tooltip_trigger_state(UseTooltipTriggerStateInput {
        delay: 200,
        close_delay: 50,
        ..Default::default()
    });
    let trigger = use_tooltip_trigger(UseTooltipTriggerInput::default(), state);

    let position = use_overlay_position(UseOverlayPositionInput {
        target: target_element,
        placement_x: Signal::derive(|| PlacementX::Center),
        placement_y: Signal::derive(|| PlacementY::Above),
        writing_direction: Signal::derive(|| WritingDirection::Ltr),
        offset: 4.0.into(),
        cross_offset: 0.0.into(),
        container_padding: 12.0.into(),
        should_flip: true.into(),
        max_height: None,
        is_open: trigger.is_open,
    });

    let is_open = trigger.is_open;
    let tooltip_id = trigger.tooltip_props.id.clone();
    let target_capture = target_element.attr();
    let position_attrs = position.props.into_attrs();

    view! {
        <button
            {..target_capture}
            style=demo_button_dark()
            {..trigger.trigger_props.into_attrs()}
        >
            "Hover me"
        </button>

        <Portal>
            {
                let tooltip_id = tooltip_id.clone();
                let position_attrs = position_attrs.clone();
                view! {
                    <Show when=move || is_open.get()>
                        <div
                            {..position_attrs.clone()}
                            id=tooltip_id.clone()
                            role="tooltip"
                            class="tooltip-demo"
                        >
                            "Move cursor here — closes!"
                        </div>
                    </Show>
                }
            }
        </Portal>
    }
}

/// Tooltip with use_tooltip — stays open when moving cursor to tooltip content
#[component]
fn WithUseTooltipDemo() -> impl IntoView {
    let target_element = CapturedElement::new();

    let state = use_tooltip_trigger_state(UseTooltipTriggerStateInput {
        delay: 200,
        close_delay: 50,
        ..Default::default()
    });
    let trigger = use_tooltip_trigger(UseTooltipTriggerInput::default(), state);
    let tooltip = use_tooltip(UseTooltipInput {
        disabled: Signal::derive(|| false),
        state: Some(state),
        on_open: None,
        on_close: None,
    });

    let position = use_overlay_position(UseOverlayPositionInput {
        target: target_element,
        placement_x: Signal::derive(|| PlacementX::Center),
        placement_y: Signal::derive(|| PlacementY::Above),
        writing_direction: Signal::derive(|| WritingDirection::Ltr),
        offset: 4.0.into(),
        cross_offset: 0.0.into(),
        container_padding: 12.0.into(),
        should_flip: true.into(),
        max_height: None,
        is_open: trigger.is_open,
    });

    let is_open = trigger.is_open;
    let tooltip_id = trigger.tooltip_props.id.clone();
    let target_capture = target_element.attr();
    let position_attrs = position.props.into_attrs();
    let tooltip_attrs = tooltip.props.into_attrs();

    view! {
        <button
            {..target_capture}
            style=demo_button_primary()
            {..trigger.trigger_props.into_attrs()}
        >
            "Hover me"
        </button>

        <Portal>
            {
                let tooltip_id = tooltip_id.clone();
                let position_attrs = position_attrs.clone();
                let tooltip_attrs = tooltip_attrs.clone();
                view! {
                    <Show when=move || is_open.get()>
                        <div
                            {..position_attrs.clone()}
                            {..tooltip_attrs.clone()}
                            id=tooltip_id.clone()
                            role="tooltip"
                            class="tooltip-demo"
                        >
                            "Move cursor here — stays open!"
                        </div>
                    </Show>
                }
            }
        </Portal>
    }
}

/// Demo showing different tooltip placements
#[component]
fn PositioningDemo() -> impl IntoView {
    view! {
        <div style="display: grid; grid-template-columns: repeat(2, 1fr); gap: 2em; max-width: 400px; margin: 2em auto;">
            <PositionedTooltip
                label="Above"
                placement_x=PlacementX::Center
                placement_y=PlacementY::Above
            />
            <PositionedTooltip
                label="Below"
                placement_x=PlacementX::Center
                placement_y=PlacementY::Below
            />
            <PositionedTooltip
                label="Left"
                placement_x=PlacementX::OuterLeft
                placement_y=PlacementY::Center
            />
            <PositionedTooltip
                label="Right"
                placement_x=PlacementX::OuterRight
                placement_y=PlacementY::Center
            />
        </div>
    }
}

/// Individual positioned tooltip button
#[component]
fn PositionedTooltip(
    label: &'static str,
    placement_x: PlacementX,
    placement_y: PlacementY,
) -> impl IntoView {
    let target_element = CapturedElement::new();

    let state = use_tooltip_trigger_state(UseTooltipTriggerStateInput {
        delay: 200,
        close_delay: 50,
        ..Default::default()
    });
    let trigger = use_tooltip_trigger(UseTooltipTriggerInput::default(), state);
    let tooltip = use_tooltip(UseTooltipInput {
        disabled: Signal::derive(|| false),
        state: Some(state),
        on_open: None,
        on_close: None,
    });

    let position = use_overlay_position(UseOverlayPositionInput {
        target: target_element,
        placement_x: Signal::derive(move || placement_x),
        placement_y: Signal::derive(move || placement_y),
        writing_direction: Signal::derive(|| WritingDirection::Ltr),
        offset: 4.0.into(),
        cross_offset: 0.0.into(),
        container_padding: 12.0.into(),
        should_flip: true.into(),
        max_height: None,
        is_open: trigger.is_open,
    });

    let is_open = trigger.is_open;
    let tooltip_id = trigger.tooltip_props.id.clone();
    let target_capture = target_element.attr();
    let position_attrs = position.props.into_attrs();
    let tooltip_attrs = tooltip.props.into_attrs();

    view! {
        <div style="display: flex; justify-content: center;">
            <button
                {..target_capture}
                style="padding: 0.5em 1em; border-radius: 6px; cursor: pointer; background: #555; color: white; border: none; font-size: 0.9em; min-width: 80px;"
                {..trigger.trigger_props.into_attrs()}
            >
                {label}
            </button>

            <Portal>
                {
                    let tooltip_id = tooltip_id.clone();
                    let position_attrs = position_attrs.clone();
                    let tooltip_attrs = tooltip_attrs.clone();
                    view! {
                        <Show when=move || is_open.get()>
                            <div
                                {..position_attrs.clone()}
                                {..tooltip_attrs.clone()}
                                id=tooltip_id.clone()
                                role="tooltip"
                                class="positioned-tooltip"
                            >
                                {format!("Tooltip: {label}")}
                            </div>
                        </Show>
                    }
                }
            </Portal>
        </div>
    }
}
