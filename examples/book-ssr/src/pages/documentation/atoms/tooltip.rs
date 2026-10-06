use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::tooltip::TooltipDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageAtomTooltip() -> impl IntoView {
    view! {
        <DocPage title="Tooltip atoms">
            <p>
                "A "<Code inline=true>"TooltipTrigger"</Code>" shows its unstyled "<Code inline=true>"Tooltip"</Code>
                " while the focusable atom inside it (a "<Code inline=true>"Button"</Code>", a "<Code inline=true>"Link"</Code>
                ", ...) is hovered or focused. See the "<Link href=routes::doc::Tooltip.materialize()>"Tooltip overview"</Link>
                " for concept guidance."
            </p>

            <Section title="Hooks Used">
                <p>
                    <Code inline=true>"TooltipTrigger"</Code>" combines "<Code inline=true>"use_tooltip_trigger_state"</Code>
                    " and "<Code inline=true>"use_tooltip_trigger"</Code>" (see the "
                    <Link href=routes::doc::tooltip::Hook.materialize()>"tooltip hooks"</Link>"). It hands the trigger\u{2019}s "
                    "handlers and "<Code inline=true>"aria-describedby"</Code>" to the focusable atom inside it through a "
                    <Code inline=true>"FocusableContext"</Code>" (see "
                    <Link href=routes::doc::focus::UseFocusable.materialize()>"use_focusable"</Link>"). "
                    <Code inline=true>"Tooltip"</Code>" calls "<Code inline=true>"use_tooltip"</Code>" and "
                    <Link href=format!("{}#use-overlay-position", routes::doc::overlays::UseOverlay.materialize())>"use_overlay_position"</Link>"."
                </p>
            </Section>

            <Section title="TooltipTrigger">
                <p>"Owns the open state and wraps the trigger and its tooltip."</p>

                <ApiTable kind=ApiKind::Props of="TooltipTrigger">
                    <ApiRow name="delay" ty="Duration" default="1500 ms">
                        "How long hovering takes to open the first tooltip. Once one was shown, the next ones open at once."
                    </ApiRow>
                    <ApiRow name="close_delay" ty="Duration" default="500 ms">"How long the tooltip stays after the pointer leaves."</ApiRow>
                    <ApiRow name="trigger" ty="TooltipTriggerMode" default="Hover">
                        <Code inline=true>"Hover"</Code>": hovering and focusing open the tooltip; "<Code inline=true>"Focus"</Code>
                        ": only focusing."
                    </ApiRow>
                    <ApiRow name="is_disabled" ty="Signal<bool>" default="false">"Whether the tooltip never opens."</ApiRow>
                    <ApiRow name="should_close_on_press" ty="bool" default="true">"Whether pressing the trigger closes the tooltip."</ApiRow>
                    <ApiRow name="default_open" ty="bool" default="false">"Whether the tooltip starts open."</ApiRow>
                    <ApiRow name="on_open_change" ty="Option<Callback<bool>>" default="None">"Called when the tooltip opens or closes."</ApiRow>
                    <ApiRow name="state" ty="Option<ValueBinding<bool>>" default="None">
                        "The open state as app state (e.g. an "<Code inline=true>"RwSignal<bool>"</Code>"), replacing "
                        <Code inline=true>"default_open"</Code>"."
                    </ApiRow>
                    <ApiRow name="children" ty="Children">"A focusable atom and its "<Code inline=true>"Tooltip"</Code>"."</ApiRow>
                </ApiTable>
            </Section>

            <Section title="Tooltip">
                <p>
                    "The tooltip of the surrounding "<Code inline=true>"TooltipTrigger"</Code>": "
                    <Code inline=true>"role=\"tooltip\""</Code>", rendered in a portal while open and positioned next to the "
                    "trigger. Hovering it keeps it open; scrolling closes it."
                </p>

                <ApiTable kind=ApiKind::Props of="Tooltip">
                    <ApiRow name="placement_x" ty="Signal<PlacementX>" default="Center">"Horizontal placement relative to the trigger."</ApiRow>
                    <ApiRow name="placement_y" ty="Signal<PlacementY>" default="Above">"Vertical placement relative to the trigger."</ApiRow>
                    <ApiRow name="offset, cross_offset" ty="Signal<f64>" default="0.0">
                        "Distance from the trigger and shift along its edge, in pixels."
                    </ApiRow>
                    <ApiRow name="container_padding" ty="Signal<f64>" default="12.0">"Minimum distance from the viewport edges."</ApiRow>
                    <ApiRow name="should_flip" ty="Signal<bool>" default="true">"Flip to the other side when there is no room."</ApiRow>
                    <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the tooltip element."</ApiRow>
                    <ApiRow name="children" ty="ChildrenFn">"The tooltip text."</ApiRow>
                </ApiTable>
            </Section>

            <Section title="Example">
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::atoms::prelude::{Button, Tooltip, TooltipTrigger};

                        view! {
                            <TooltipTrigger>
                                <Button>"Save"</Button>
                                <Tooltip classes="my-tooltip">"Save the document (Ctrl+S)"</Tooltip>
                            </TooltipTrigger>
                        }
                    "#)}
                </Code>
            </Section>

            <Section title="Demo">
                <p>
                    "Hover a button: the first tooltip waits for the delay, the second opens right away. Tab to a button to "
                    "open its tooltip at once; "<Keys keys="Escape"/>" closes it."
                </p>

                <Demo description="Two buttons with tooltips above and below, with a disabled toggle" source=include_str!("demos/tooltip.rs")>
                    <TooltipDemo/>
                </Demo>
            </Section>

            <Section title="Data Attributes">
                <ApiTable kind=ApiKind::DataAttributes>
                    <ApiRow name="data-placement" ty="\"top\" | \"bottom\" | \"left\" | \"right\"">
                        "Set on the tooltip: the side of the trigger it opened on, after flipping."
                    </ApiRow>
                </ApiTable>
            </Section>

            <Section title="Styling">
                <p>
                    "The atoms add no classes. Style the tooltip through "<Code inline=true>"classes"</Code>
                    ", and use "<Code inline=true>"data-placement"</Code>" to point an arrow at the trigger:"
                </p>
                <Code language=Language::Css>
                    {indoc!(r#"
                        .my-tooltip { padding: 4px 8px; border-radius: 4px; background: #222; color: white; }
                        .my-tooltip[data-placement="top"] { margin-bottom: 4px; }
                    "#)}
                </Code>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Tooltip.materialize()>"Tooltip overview"</Link></li>
                <li><Link href=routes::doc::tooltip::Hook.materialize()>"Tooltip hooks"</Link></li>
                <li><Link href=routes::doc::popover::Atom.materialize()>"Popover atoms"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
