use leptos::prelude::*;

use super::demos::overlays::OverlayDismissDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageOverlayBehavior() -> impl IntoView {
    view! {
        <DocPage title="Overlay Behavior">
            <p>
                "Building blocks for content that appears above the page: its open state, dismissing it, connecting it to the "
                "element that opens it, positioning it next to that element and keeping the rest of the page out of the way. "
                "Concepts like "<Link href=routes::doc::Popover.materialize()>"Popover"</Link>", "
                <Link href=routes::doc::Modal.materialize()>"Modal"</Link>", "
                <Link href=routes::doc::Tooltip.materialize()>"Tooltip"</Link>", "
                <Link href=routes::doc::Menu.materialize()>"Menu"</Link>" and "
                <Link href=routes::doc::Select.materialize()>"Select"</Link>" are built from them. Reach for them when you "
                "build an overlay none of those concepts covers."
            </p>

            <Section title="Pages">
                <SectionMembers overview=routes::doc::OverlayBehavior.materialize()/>
            </Section>

            <Section title="Relationships">
                <p>
                    "Each building block does one job and is used on its own as well as combined. "
                    <Link href=routes::doc::overlay_behavior::UseOverlayTriggerState.materialize()>"use_overlay_trigger_state"</Link>
                    " holds whether the overlay is open, "
                    <Link href=routes::doc::overlay_behavior::UseOverlay.materialize()>"use_overlay"</Link>" closes it, "
                    <Link href=routes::doc::overlay_behavior::UseOverlayTrigger.materialize()>"use_overlay_trigger"</Link>
                    " describes the trigger to assistive technology and "
                    <Link href=routes::doc::overlay_behavior::UseOverlayPosition.materialize()>"use_overlay_position"</Link>
                    " places the overlay. Modal overlays add "
                    <Link href=routes::doc::overlay_behavior::UsePreventScroll.materialize()>"use_prevent_scroll"</Link>" and "
                    <Link href=routes::doc::overlay_behavior::AriaHideOutside.materialize()>"aria_hide_outside"</Link>
                    "; non-modal ones close on scroll with "
                    <Link href=routes::doc::overlay_behavior::UseCloseOnScroll.materialize()>"use_close_on_scroll"</Link>
                    ". Focus is managed by a "<Link href=routes::doc::focus::FocusScope.materialize()>"FocusScope"</Link>
                    " from the "<Link href=routes::doc::Focus.materialize()>"Focus"</Link>" building blocks, not by the overlay hooks."
                </p>

                <p>"The overlays of leptonic combine them like this:"</p>

                <DocTable headers=&["Built on them", "Uses"]>
                    <TableRow>
                        <TableCell><Link href=routes::doc::popover::Hook.materialize()>"use_popover"</Link></TableCell>
                        <TableCell>
                            <Code inline=true>"use_overlay"</Code>" (outside presses close only modal popovers and submenus), "
                            <Code inline=true>"use_overlay_position"</Code>" (closing on scroll when non-modal); "
                            <Code inline=true>"use_prevent_scroll"</Code>" and "<Code inline=true>"aria_hide_outside"</Code>
                            " when modal, "<Code inline=true>"keep_visible"</Code>" when not"
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::popover::Atom.materialize()>"Popover"</Link>" atom"</TableCell>
                        <TableCell>
                            <Code inline=true>"use_popover"</Code>", an "<Code inline=true>"OverlayFocusContain"</Code>
                            " context and "<Code inline=true>"DismissButton"</Code>"s; its open state comes from a "
                            <Code inline=true>"DialogTrigger"</Code>
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::dialog::Atom.materialize()>"DialogTrigger"</Link>" atom"</TableCell>
                        <TableCell>
                            <Code inline=true>"use_overlay_trigger_state"</Code>"; it sets "<Code inline=true>"aria-expanded"</Code>
                            " and "<Code inline=true>"aria-controls"</Code>" on its trigger itself"
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::dialog::Hook.materialize()>"use_dialog"</Link></TableCell>
                        <TableCell><Code inline=true>"use_overlay_focus_contain"</Code></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>
                            <Link href=format!("{}#use-modal-backdrop", routes::doc::modal::Hook.materialize())>"use_modal_backdrop"</Link>
                        </TableCell>
                        <TableCell>
                            <Code inline=true>"use_overlay"</Code>" (not on blur), "<Code inline=true>"use_prevent_scroll"</Code>" and "
                            <Code inline=true>"aria_hide_outside"</Code>
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::tooltip::Atom.materialize()>"Tooltip"</Link>" atom"</TableCell>
                        <TableCell>
                            <Code inline=true>"use_overlay_position"</Code>"; its trigger state builds on "
                            <Code inline=true>"use_overlay_trigger_state"</Code>
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>
                            <Link href=format!("{}#use-menu-trigger", routes::doc::menu::Hook.materialize())>"use_menu_trigger"</Link>
                        </TableCell>
                        <TableCell>
                            <Code inline=true>"use_overlay_trigger"</Code>" (also for the select and combo box listboxes)"
                        </TableCell>
                    </TableRow>
                </DocTable>

                <p>
                    "Outside presses are detected by "
                    <Link href=routes::doc::interactions::UseInteractOutside.materialize()>"use_interact_outside"</Link>
                    " from the "<Link href=routes::doc::Interactions.materialize()>"Interactions"</Link>" building blocks. "
                    <Link href=routes::doc::overlay_behavior::DismissButton.materialize()>"DismissButton"</Link>
                    " gives screen reader users without an Escape key a way to close an overlay."
                </p>
            </Section>

            <Section title="Quick Start">
                <p>
                    "A button that opens a panel: "<Code inline=true>"use_overlay"</Code>" closes it on Escape, on a press "
                    "outside and when focus leaves it, "<Code inline=true>"use_overlay_trigger"</Code>" sets "
                    <Code inline=true>"aria-expanded"</Code>" and "<Code inline=true>"aria-controls"</Code>
                    " on the button, and a "<Code inline=true>"FocusScope"</Code>" moves focus into the panel and back."
                </p>

                <Demo
                    description="A button toggling a panel that closes on Escape, outside presses and blur"
                    source=include_str!("demos/overlays.rs")
                    source_open=true
                >
                    <OverlayDismissDemo/>
                </Demo>
            </Section>
        </DocPage>
    }
}
