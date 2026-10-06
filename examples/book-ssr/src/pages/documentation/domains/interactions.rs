use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::interactions::InteractionsDomainDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageInteractions() -> impl IntoView {
    view! {
        <DocPage title="Interactions">
            <p>
                "Low-level hooks for handling pointer, keyboard and scroll interactions. They are the behavioral foundation "
                "of all interactive components: higher-level concepts like Button, Slider and Menu compose them internally."
            </p>

            <p>
                "Each hook normalizes browser differences across mouse, touch, keyboard and screen reader input, so components "
                "built on top receive a consistent event model regardless of the input device."
            </p>

            <Section title="Pages">
                <SectionMembers overview=routes::doc::Interactions.materialize()/>
            </Section>

            <Section title="Relationships">
                <p>
                    <Code inline=true>"use_press"</Code>" and "<Code inline=true>"use_hover"</Code>" are the most commonly paired "
                    "hooks: nearly every interactive element needs both. The library provides merged props types for frequent "
                    "combinations:"
                </p>

                <DocTable headers=&["Merged type", "Combines"]>
                    <TableRow><TableCell><Code inline=true>"MergedPressHoverProps"</Code></TableCell><TableCell>"use_press + use_hover"</TableCell></TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"MergedPressHoverFocusRingProps"</Code></TableCell>
                        <TableCell>"use_press + use_hover + use_focus_ring"</TableCell>
                    </TableRow>
                    <TableRow><TableCell><Code inline=true>"MergedPressFocusRingProps"</Code></TableCell><TableCell>"use_press + use_focus_ring"</TableCell></TableRow>
                    <TableRow><TableCell><Code inline=true>"MergedHoverFocusRingProps"</Code></TableCell><TableCell>"use_hover + use_focus_ring"</TableCell></TableRow>
                    <TableRow><TableCell><Code inline=true>"MergedFocusablePressProps"</Code></TableCell><TableCell>"use_focusable + use_press"</TableCell></TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"MergedFocusablePressFocusRingProps"</Code></TableCell>
                        <TableCell>"use_focusable + use_press + use_focus_ring"</TableCell>
                    </TableRow>
                </DocTable>

                <p>"Cross-domain relationships:"</p>

                <ul>
                    <li>
                        <Link href=routes::doc::interactions::UseInteractOutside.materialize()>"use_interact_outside"</Link>
                        " is used by "<Link href=routes::doc::overlays::UseOverlay.materialize()>"use_overlay"</Link>
                        " to dismiss popovers and menus when you click outside."
                    </li>
                    <li>
                        <Link href=routes::doc::interactions::UsePreventScroll.materialize()>"use_prevent_scroll"</Link>
                        " locks page scrolling while a "<Link href=routes::doc::modal::Hook.materialize()>"modal"</Link>" or a modal "
                        <Link href=routes::doc::popover::Hook.materialize()>"popover"</Link>" is open."
                    </li>
                    <li>
                        <Link href=routes::doc::interactions::UseMove.materialize()>"use_move"</Link>
                        " powers thumb dragging in the "<Link href=routes::doc::slider::Hook.materialize()>"slider"</Link>" hooks."
                    </li>
                    <li>
                        <Link href=routes::doc::interactions::UseKeyboard.materialize()>"use_keyboard"</Link>
                        " handles arbitrary keys, while "<Link href=routes::doc::interactions::UsePress.materialize()>"use_press"</Link>
                        " handles Enter and Space as press activations."
                    </li>
                </ul>
            </Section>

            <Section title="Quick Start">
                <p>"The simplest interaction: a pressable element that counts presses."</p>

                <Demo description="A button counting presses with use_press" source=include_str!("demos/interactions.rs") source_open=true>
                    <InteractionsDomainDemo/>
                </Demo>
            </Section>
        </DocPage>
    }
}
