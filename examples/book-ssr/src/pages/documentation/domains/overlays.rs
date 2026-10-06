use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::overlays::OverlaysDomainDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageOverlays() -> impl IntoView {
    let overlay_hooks = StoredValue::new(routes::doc::overlays::UseOverlay.materialize());

    view! {
        <DocPage title="Overlays">
            <p>
                "Building blocks for floating content that appears above the page: dismissing it, connecting it to the "
                "element that opens it and positioning it next to that element. Higher-level concepts like "
                <Link href=routes::doc::Popover.materialize()>"Popover"</Link>", "
                <Link href=routes::doc::Modal.materialize()>"Modal"</Link>", "
                <Link href=routes::doc::Menu.materialize()>"Menu"</Link>" and "
                <Link href=routes::doc::Select.materialize()>"Select"</Link>" are built from them. Reach for these "
                "primitives when you build an overlay none of those concepts covers."
            </p>

            <Section title="Pages">
                <SectionMembers overview=routes::doc::Overlays.materialize()/>

                <Section title="Overlay Hooks">
                    <p>"The "<Link href=overlay_hooks.get_value()>"Overlay Hooks"</Link>" page documents these primitives:"</p>
                    <DocTable headers=&["Hook", "Purpose"]>
                        <TableRow>
                            <TableCell><Link href=format!("{}#use-overlay", overlay_hooks.get_value())>"use_overlay"</Link></TableCell>
                            <TableCell>"Dismisses an overlay on Escape, a press outside or blur, and keeps track of stacked overlays."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Link href=format!("{}#use-overlay-trigger", overlay_hooks.get_value())>"use_overlay_trigger"</Link></TableCell>
                            <TableCell>"ARIA attributes that connect a trigger to the overlay it opens."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Link href=format!("{}#use-overlay-position", overlay_hooks.get_value())>"use_overlay_position"</Link></TableCell>
                            <TableCell>"Positions an overlay next to a target element and flips it when space runs out."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Link href=format!("{}#use-close-on-scroll", overlay_hooks.get_value())>"use_close_on_scroll"</Link></TableCell>
                            <TableCell>"Closes an overlay when its trigger scrolls."</TableCell>
                        </TableRow>
                    </DocTable>
                </Section>
            </Section>

            <Section title="Relationships">
                <Section title="Composition">
                    <p>"The higher-level overlays combine the primitives:"</p>

                    <DocTable headers=&["Built on overlays", "Uses"]>
                        <TableRow>
                            <TableCell><Link href=routes::doc::popover::Hook.materialize()>"use_popover"</Link></TableCell>
                            <TableCell>
                                <Code inline=true>"use_overlay"</Code>", "<Code inline=true>"use_overlay_position"</Code>", "
                                <Code inline=true>"use_prevent_scroll"</Code>" and "<Code inline=true>"aria_hide_outside"</Code>
                                " (the last two only for modal popovers)"
                            </TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Link href=routes::doc::popover::Atom.materialize()>"Popover atom"</Link></TableCell>
                            <TableCell>
                                <Code inline=true>"use_overlay"</Code>", "<Code inline=true>"use_overlay_trigger"</Code>", "
                                <Code inline=true>"use_overlay_position"</Code>", "<Code inline=true>"use_prevent_scroll"</Code>" and "
                                <Code inline=true>"use_close_on_scroll"</Code>
                            </TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Link href=routes::doc::modal::Hook.materialize()>"use_modal_backdrop"</Link></TableCell>
                            <TableCell>
                                <Code inline=true>"use_overlay"</Code>", "<Code inline=true>"use_prevent_scroll"</Code>" and "
                                <Code inline=true>"aria_hide_outside"</Code>
                            </TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Link href=routes::doc::menu::Hook.materialize()>"use_menu_trigger"</Link></TableCell>
                            <TableCell><Code inline=true>"use_overlay_trigger"</Code></TableCell>
                        </TableRow>
                    </DocTable>
                </Section>

                <Section title="Cross-Domain">
                    <ul>
                        <li>
                            <Link href=routes::doc::interactions::UseInteractOutside.materialize()>"use_interact_outside"</Link>
                            " (from the "<Link href=routes::doc::Interactions.materialize()>"Interactions"</Link>
                            " domain) detects the presses outside an overlay that "<Code inline=true>"use_overlay"</Code>" reacts to."
                        </li>
                        <li>
                            <Link href=routes::doc::interactions::UsePreventScroll.materialize()>"use_prevent_scroll"</Link>
                            " (from the "<Link href=routes::doc::Interactions.materialize()>"Interactions"</Link>
                            " domain) locks page scrolling while a modal or modal popover is open."
                        </li>
                        <li>
                            <Link href=routes::doc::focus::FocusScope.materialize()>"FocusScope"</Link>" (from the "
                            <Link href=routes::doc::Focus.materialize()>"Focus"</Link>" domain) moves focus into an overlay, "
                            "keeps it there for modal dialogs and restores it when the overlay closes. The overlay hooks don\u{2019}t "
                            "manage focus themselves."
                        </li>
                        <li>
                            <Link href=routes::doc::overlays::DismissButton.materialize()>"DismissButton"</Link>
                            " goes inside popovers and modals, so that screen reader users without an Escape key can close them."
                        </li>
                    </ul>
                </Section>
            </Section>

            <Section title="Quick Start">
                <p>
                    "A button that opens a panel. "<Code inline=true>"use_overlay"</Code>" closes it on Escape, on a press "
                    "outside and when focus leaves it, "<Code inline=true>"use_overlay_trigger"</Code>" sets "
                    <Code inline=true>"aria-expanded"</Code>" and "<Code inline=true>"aria-controls"</Code>
                    " on the button, and a "<Code inline=true>"FocusScope"</Code>" moves focus into the panel and back."
                </p>

                <Demo
                    description="Button toggling a panel that closes on Escape, outside presses and blur"
                    source=include_str!("demos/overlays.rs")
                    source_open=true
                >
                    <OverlaysDomainDemo/>
                </Demo>
            </Section>
        </DocPage>
    }
}
