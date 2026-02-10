use std::marker::PhantomData;

use crate::pages::documentation::article::Article;
use crate::pages::documentation::toc::Toc;
use indoc::indoc;

use leptonic::components::prelude::*;
use leptonic::hooks::*;
use leptonic::utils::locale::WritingDirection;
use leptos::html;
use leptos::portal::Portal;
use leptos::prelude::*;

#[component]
pub fn PageUseTooltipHook() -> impl IntoView {
    view! {
        <Article>
            <h1 id="tooltip" class="anchor">
                "use_tooltip_trigger"
                <AnchorLink href="#tooltip" description="Direct link to article header"/>
            </h1>

            <p>"Hooks for creating accessible tooltips with hover/focus triggers, delays, and proper ARIA associations."</p>

            <h2 id="demo" class="anchor">
                "Interactive Demo"
                <AnchorLink href="#demo" description="Direct link to demo"/>
            </h2>

            <p>"Hover over or focus the button to see the tooltip. Press Escape to close it."</p>

            <TooltipDemo />

            <h2 id="use_tooltip_trigger" class="anchor">
                "use_tooltip_trigger"
                <AnchorLink href="#use_tooltip_trigger" description="Direct link to use_tooltip_trigger"/>
            </h2>

            <p>"The "<code>"use_tooltip_trigger"</code>" hook manages the tooltip trigger element and provides props for both the trigger and tooltip."</p>

            <Code>
                {indoc!(r#"
                    use leptonic::hooks::{use_tooltip_trigger, UseTooltipTriggerInput};
                    use leptos::portal::Portal;

                    let tooltip = use_tooltip_trigger(UseTooltipTriggerInput {
                        delay: 300,
                        close_delay: 100,
                        ..Default::default()
                    });

                    view! {
                        <button {..tooltip.trigger_props}>
                            "Hover me"
                        </button>
                        <Portal>
                            <Show when=move || tooltip.is_open.get()>
                                <div
                                    id=tooltip.tooltip_props.id.clone()
                                    role=tooltip.tooltip_props.role
                                >
                                    "Tooltip content"
                                </div>
                            </Show>
                        </Portal>
                    }
                "#)}
            </Code>

            <p>"The hook returns:"</p>
            <ul>
                <li><code>"trigger_props"</code>" - Props to spread onto the trigger element (handles events and ARIA)"</li>
                <li><code>"tooltip_props"</code>" - Props for the tooltip element (id and role)"</li>
                <li><code>"is_open"</code>" - Signal indicating if the tooltip is visible"</li>
                <li><code>"open"</code>" / "<code>"close"</code>" - Callbacks to manually control visibility"</li>
            </ul>

            <h2 id="positioning" class="anchor">
                "Positioning with use_overlay_position"
                <AnchorLink href="#positioning" description="Direct link to positioning"/>
            </h2>

            <p>"Combine "<code>"use_tooltip_trigger"</code>" with "<code>"use_overlay_position"</code>" for automatic positioning with collision detection."</p>

            <Code>
                {indoc!(r#"
                    use leptonic::hooks::{
                        use_tooltip_trigger, UseTooltipTriggerInput,
                        use_overlay_position, UseOverlayPositionInput,
                        PlacementX, PlacementY,
                    };
                    use leptonic::utils::locale::WritingDirection;
                    use leptos::portal::Portal;
                    use std::marker::PhantomData;

                    let trigger_ref = NodeRef::<html::Button>::new();
                    let tooltip_ref = NodeRef::<html::Div>::new();

                    let tooltip = use_tooltip_trigger(UseTooltipTriggerInput::default());

                    let position = use_overlay_position(UseOverlayPositionInput {
                        overlay: tooltip_ref,
                        target: trigger_ref,
                        placement_x: Signal::derive(|| PlacementX::Center),
                        placement_y: Signal::derive(|| PlacementY::Above),
                        writing_direction: Signal::derive(|| WritingDirection::Ltr),
                        phantom_data: PhantomData,
                    });

                    view! {
                        <button node_ref=trigger_ref {..tooltip.trigger_props}>
                            "Hover me"
                        </button>
                        <Portal>
                            <Show when=move || tooltip.is_open.get()>
                                <div
                                    {..position.props.to_attrs()}
                                    node_ref=tooltip_ref
                                    id=tooltip.tooltip_props.id.clone()
                                    role=tooltip.tooltip_props.role
                                >
                                    "Positioned tooltip"
                                </div>
                            </Show>
                        </Portal>
                    }
                "#)}
            </Code>

            <h3>"Positioning Demo"</h3>

            <p>"Hover over the buttons to see tooltips positioned in different directions:"</p>

            <PositioningDemo />

            <h2 id="trigger-modes" class="anchor">
                "Trigger Modes"
                <AnchorLink href="#trigger-modes" description="Direct link to trigger modes"/>
            </h2>

            <ul>
                <li><code>"TooltipTrigger::Hover"</code>" - Show on hover and focus (default, most accessible)"</li>
                <li><code>"TooltipTrigger::Focus"</code>" - Show on focus only"</li>
            </ul>

            <h2 id="delays" class="anchor">
                "Open & Close Delays"
                <AnchorLink href="#delays" description="Direct link to delays"/>
            </h2>

            <p>"Delays help prevent tooltip flashing when users move their cursor across the page:"</p>
            <ul>
                <li><code>"delay"</code>" - Time before showing tooltip (default: 300ms)"</li>
                <li><code>"close_delay"</code>" - Time before hiding tooltip (default: 0ms)"</li>
            </ul>

            <h2 id="accessibility" class="anchor">
                "Accessibility"
                <AnchorLink href="#accessibility" description="Direct link to accessibility"/>
            </h2>

            <ul>
                <li>"Tooltips are shown on focus for keyboard users"</li>
                <li>"Pressing Escape closes the tooltip"</li>
                <li>"ARIA attributes ("<code>"aria-describedby"</code>") are automatically managed"</li>
                <li>"Tooltip content should be supplementary, not essential"</li>
                <li>"Essential content should use different patterns (e.g., visible labels)"</li>
            </ul>

            <h2 id="features" class="anchor">
                "Features"
                <AnchorLink href="#features" description="Direct link to features"/>
            </h2>

            <ul>
                <li>"Hover and focus triggers"</li>
                <li>"Configurable open/close delays"</li>
                <li>"Escape key to close"</li>
                <li>"Automatic ARIA associations"</li>
                <li>"Disabled state support"</li>
                <li>"Automatic positioning with collision detection (via "<code>"use_overlay_position"</code>")"</li>
            </ul>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "use_tooltip_trigger", link: "#tooltip" },
                Toc::Leaf { title: "Demo", link: "#demo" },
                Toc::Leaf { title: "use_tooltip_trigger", link: "#use_tooltip_trigger" },
                Toc::Leaf { title: "Positioning", link: "#positioning" },
                Toc::Leaf { title: "Trigger Modes", link: "#trigger-modes" },
                Toc::Leaf { title: "Delays", link: "#delays" },
                Toc::Leaf { title: "Accessibility", link: "#accessibility" },
                Toc::Leaf { title: "Features", link: "#features" },
            ]
        }/>
    }
}

/// Simple tooltip demo using use_tooltip_trigger
#[component]
fn TooltipDemo() -> impl IntoView {
    let trigger_ref = NodeRef::<html::Button>::new();
    let tooltip_ref = NodeRef::<html::Div>::new();

    let tooltip = use_tooltip_trigger(UseTooltipTriggerInput {
        delay: 300,
        close_delay: 100,
        ..Default::default()
    });

    let position = use_overlay_position(UseOverlayPositionInput {
        overlay: tooltip_ref,
        target: trigger_ref,
        placement_x: Signal::derive(|| PlacementX::Center),
        placement_y: Signal::derive(|| PlacementY::Above),
        writing_direction: Signal::derive(|| WritingDirection::Ltr),
        phantom_data: PhantomData,
    });

    let is_open = tooltip.is_open;
    let tooltip_id = tooltip.tooltip_props.id.clone();
    let position_attrs = position.props.to_attrs();

    view! {
        <div style="margin: 3em 0; display: flex; justify-content: center;">
            <button
                node_ref=trigger_ref
                style="padding: 0.75em 1.5em; border-radius: 8px; cursor: pointer; background: var(--brand-color); color: white; border: none; font-size: 1em;"
                {..tooltip.trigger_props}
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
                                node_ref=tooltip_ref
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
    let trigger_ref = NodeRef::<html::Button>::new();
    let tooltip_ref = NodeRef::<html::Div>::new();

    let tooltip = use_tooltip_trigger(UseTooltipTriggerInput {
        delay: 200,
        close_delay: 50,
        ..Default::default()
    });

    let position = use_overlay_position(UseOverlayPositionInput {
        overlay: tooltip_ref,
        target: trigger_ref,
        placement_x: Signal::derive(move || placement_x),
        placement_y: Signal::derive(move || placement_y),
        writing_direction: Signal::derive(|| WritingDirection::Ltr),
        phantom_data: PhantomData,
    });

    let is_open = tooltip.is_open;
    let tooltip_id = tooltip.tooltip_props.id.clone();
    let position_attrs = position.props.to_attrs();

    view! {
        <div style="display: flex; justify-content: center;">
            <button
                node_ref=trigger_ref
                style="padding: 0.5em 1em; border-radius: 6px; cursor: pointer; background: #555; color: white; border: none; font-size: 0.9em; min-width: 80px;"
                {..tooltip.trigger_props}
            >
                {label}
            </button>

            <Portal>
                {
                    let tooltip_id = tooltip_id.clone();
                    let position_attrs = position_attrs.clone();
                    view! {
                        <Show when=move || is_open.get()>
                            <div
                                {..position_attrs.clone()}
                                node_ref=tooltip_ref
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
