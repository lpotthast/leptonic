use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::tooltip::TooltipConceptDemo;
use crate::{
    pages::documentation::{article::Article, demo_shell::DemoShell, toc::Toc},
    routes,
};

#[component]
pub fn PageTooltipOverview() -> impl IntoView {
    view! {
        <Article>
            <h1 id="tooltip" class="anchor">
                "Tooltip"
                <AnchorLink href="#tooltip" description="Direct link to article header"/>
            </h1>

            <p>
                "Tooltips are non-interactive floating labels that appear on hover or focus "
                "to describe an element. They are purely informational \u{2014} "
                "if the user needs to interact with the popup content, use a Popover instead."
            </p>

            <p>
                "Tooltips are currently available as hooks only. "
                "See "<Link href=routes::doc::Architecture.materialize()>"Hooks, Atoms & Components"</Link>
                " for a detailed explanation of each layer."
            </p>

            <h2 id="when-to-use" class="anchor">
                "When to Use"
                <AnchorLink href="#when-to-use" description="Direct link to section: When to Use"/>
            </h2>

            <TableContainer>
                <Table bordered=true hoverable=true>
                    <TableHeader>
                        <TableRow>
                            <TableHeaderCell>"If you want to\u{2026}"</TableHeaderCell>
                            <TableHeaderCell min_width=true>"Use"</TableHeaderCell>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        <TableRow>
                            <TableCell>"Show a short description on hover/focus"</TableCell>
                            <TableCell><b>"Tooltip"</b></TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>"Show interactive content in a popup"</TableCell>
                            <TableCell>"Popover"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>"Provide a permanent visible label"</TableCell>
                            <TableCell>"Label"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>"Use the browser's native tooltip"</TableCell>
                            <TableCell><Code inline=true>"title"</Code>" attribute"</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            <p>
                "Tooltips have a warmup delay (they don't appear instantly) and a cooldown period. "
                "Once one tooltip has been shown, subsequent tooltips appear faster \u{2014} "
                "matching the user's expectation that hovering reveals information."
            </p>

            <h2 id="dive-deeper" class="anchor">
                "Dive Deeper"
                <AnchorLink href="#dive-deeper" description="Direct link to section: Dive Deeper"/>
            </h2>

            <ul>
                <li><Link href=routes::doc::tooltip::Hook.materialize()>"Hook: use_tooltip"</Link></li>
            </ul>

            <h2 id="quick-start" class="anchor">
                "Quick Start"
                <AnchorLink href="#quick-start" description="Direct link to section: Quick Start"/>
            </h2>

            <p>
                "Tooltips are composed from hooks. Here is the minimal setup. "
                "See the "<Link href=routes::doc::tooltip::Hook.materialize()>"hook deep-dive"</Link>
                " for full details."
            </p>

            <Code language=Language::Rust>
                {indoc!(r#"
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
                        ..
                    });
                    let position = use_overlay_position(UseOverlayPositionInput {
                        target: target_element,
                        placement_x: Signal::derive(|| PlacementX::Center),
                        placement_y: Signal::derive(|| PlacementY::Above),
                        ..
                    });

                    view! {
                        <button {..target_element.attr()} {..trigger.trigger_props.into_attrs()}>
                            "Hover me"
                        </button>
                        <Portal>
                            <Show when=move || trigger.is_open.get()>
                                <div
                                    {..position.props.into_attrs()}
                                    {..tooltip.props.into_attrs()}
                                    id=trigger.tooltip_props.id.clone()
                                    role="tooltip"
                                >
                                    "This is a tooltip!"
                                </div>
                            </Show>
                        </Portal>
                    }
                "#)}
            </Code>

            <DemoShell description="Tooltip with hover trigger" source=include_str!("demos/tooltip.rs")>
                <TooltipConceptDemo />
            </DemoShell>

            <h2 id="accessibility" class="anchor">
                "Accessibility"
                <AnchorLink href="#accessibility" description="Direct link to section: Accessibility"/>
            </h2>

            <p>
                "Leptonic tooltips follow the WAI-ARIA Tooltip pattern."
            </p>

            <h3>"ARIA attributes"</h3>

            <ul>
                <li><Code inline=true>"role=\"tooltip\""</Code>" on the popup content"</li>
                <li><Code inline=true>"aria-describedby"</Code>" \u{2014} on the trigger, dynamically set only when the tooltip is visible"</li>
            </ul>

            <h3>"Keyboard interaction"</h3>

            <TableContainer>
                <Table bordered=true hoverable=true>
                    <TableHeader>
                        <TableRow>
                            <TableHeaderCell min_width=true>"Key"</TableHeaderCell>
                            <TableHeaderCell>"Action"</TableHeaderCell>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        <TableRow>
                            <TableCell><Code inline=true>"Focus (Tab)"</Code></TableCell>
                            <TableCell>"Shows the tooltip"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"Escape"</Code></TableCell>
                            <TableCell>"Hides the tooltip"</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "Tooltip", link: "#tooltip" },
                Toc::Leaf { title: "When to Use", link: "#when-to-use" },
                Toc::Leaf { title: "Dive Deeper", link: "#dive-deeper" },
                Toc::Leaf { title: "Quick Start", link: "#quick-start" },
                Toc::Leaf { title: "Accessibility", link: "#accessibility" },
            ]
        }/>
    }
}
