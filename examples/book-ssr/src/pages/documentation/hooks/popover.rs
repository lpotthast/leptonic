use indoc::indoc;
use leptos::prelude::*;

use super::demos::{
    popover_basic::BasicPopoverDemo, popover_non_modal::NonModalPopoverDemo,
    popover_placement::PlacementPopoverDemo,
};
use crate::{kit::*, routes};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageUsePopoverHook() -> impl IntoView {
    view! {
        <DocPage title="use_popover">
            <p>
                "The "<Code inline=true>"use_popover"</Code>" hook gives an overlay you render yourself the behavior of a "
                "popover: placement next to its trigger, dismissal and, while modal, a locked page. See the "
                <Link href=routes::doc::Popover.materialize()>"Popover overview"</Link>" for when to use a popover."
            </p>

            <ReactAria hook="usePopover"/>

            <Section title="Input">
                <p>
                    "Pass a "<Code inline=true>"UsePopoverInput"</Code>" with every field named. The Default column gives the "
                    "value for fields you don\u{2019}t need: a modal popover below the trigger, centered on it."

                </p>

                <ApiTable kind=ApiKind::Input of="UsePopoverInput">
                    <ApiRow name="state" ty="S: OverlayState">
                        "Whether the popover is open, and how to close it. Required. "<Code inline=true>"S"</Code>
                        " defaults to "<Code inline=true>"OverlayTriggerState"</Code>". Usually an "
                        <Code inline=true>"OverlayTriggerState"</Code>" ("<Link href=routes::doc::overlay_behavior::UseOverlayTriggerState.materialize()>"use_overlay_trigger_state"</Link>
                        ", or "<Code inline=true>"OverlayTriggerState::from(rw_signal)"</Code>"); the menu, select and combo box "
                        "states work too."
                    </ApiRow>
                    <ApiRow name="trigger" ty="CapturedElement" default="a new capture">
                        "The trigger the popover is positioned at. Spread "<Code inline=true>"trigger_props"</Code>
                        " onto it, or pass an element you capture already."
                    </ApiRow>
                    <ApiRow name="placement" ty="Signal<Placement>" default="Bottom">
                        "Where the popover goes relative to the trigger, see "<Link href=format!("{}#placements", routes::doc::overlay_behavior::UseOverlayPosition.materialize())>"Placements"</Link>"."
                    </ApiRow>
                    <ApiRow name="offset" ty="Signal<f64>" default="0.0">"Distance from the trigger along the main axis, in pixels."</ApiRow>
                    <ApiRow name="cross_offset" ty="Signal<f64>" default="0.0">"Shift along the cross axis, in pixels."</ApiRow>
                    <ApiRow name="container_padding" ty="Signal<f64>" default="12.0">"Minimum distance from the viewport edges, in pixels."</ApiRow>
                    <ApiRow name="should_flip" ty="Signal<bool>" default="true">"Flip to the opposite side when there is not enough room."</ApiRow>
                    <ApiRow name="max_height" ty="Signal<Option<f64>>" default="None">"A maximum height; the room available limits it further."</ApiRow>
                    <ApiRow name="arrow_size" ty="Signal<Option<f64>>" default="None">
                        "The arrow\u{2019}s size across the main axis. "<Code inline=true>"None"</Code>
                        " measures the element captured by "<Code inline=true>"arrow_props"</Code>"."
                    </ApiRow>
                    <ApiRow name="arrow_boundary_offset" ty="Signal<f64>" default="0.0">
                        "The minimum distance between the arrow and the popover\u{2019}s edges."
                    </ApiRow>
                    <ApiRow name="boundary" ty="Option<CapturedElement>" default="None">
                        "The element the popover must stay within. "<Code inline=true>"None"</Code>": the document body."
                    </ApiRow>
                    <ApiRow name="target_rect" ty="Signal<Option<Rect>>" default="None">
                        "Replaces the trigger\u{2019}s rectangle (viewport coordinates). "<Code inline=true>"None"</Code>
                        ": the state\u{2019}s "<Code inline=true>"point"</Code>" (where a context menu opened), else the trigger."
                    </ApiRow>
                    <ApiRow name="modality" ty="PopoverModality" default="Modal">
                        "Whether the popover takes over the page, see "
                        <AnchorLink href="#modal-and-non-modal">"Modal and Non-Modal"</AnchorLink>"."
                    </ApiRow>
                    <ApiRow name="is_keyboard_dismiss_disabled" ty="Signal<bool>" default="false">
                        "Whether "<Keys keys="Escape"/>" no longer closes the popover."
                    </ApiRow>
                    <ApiRow name="should_close_on_interact_outside" ty="Option<InteractOutsideFilter>" default="None">
                        "Decides per element outside whether pressing it (in a modal popover) or moving focus to it closes "
                        "the popover. "<Code inline=true>"None"</Code>" closes for every element."
                    </ApiRow>
                    <ApiRow name="group" ty="Option<CapturedElement>" default="None">
                        "The group the popover belongs to: a root popover\u{2019}s container, which also holds the popovers of "
                        "its submenus. The overlay stack, outside interactions and hiding the rest of the page work on the "
                        "group. "<Code inline=true>"None"</Code>": the popover alone."
                    </ApiRow>
                    <ApiRow name="is_submenu" ty="bool" default="false">
                        "Whether this is a submenu\u{2019}s popover: closed by outside presses although non-modal."
                    </ApiRow>
                </ApiTable>
            </Section>

            <Section title="Return">
                <ApiTable kind=ApiKind::Return of="UsePopoverReturn">
                    <ApiRow name="props" ty="PropsWithStyles<UsePopoverProps>">
                        "Attributes, event handlers and position styles for the popover element. Call "
                        <Code inline=true>"props.into_parts()"</Code>" to get "<Code inline=true>"(attrs, styles)"</Code>
                        ", then spread "<Code inline=true>"{..attrs}"</Code>" and set "<Code inline=true>"style=styles"</Code>"."
                    </ApiRow>
                    <ApiRow name="trigger_props" ty="UsePopoverTriggerProps">
                        "Spread "<Code inline=true>"trigger_props.into_attrs()"</Code>
                        " onto the trigger, so the hook can position the popover next to it."
                    </ApiRow>
                    <ApiRow name="id" ty="String">
                        "The id of the popover element. Pass it to "
                        <Link href=routes::doc::overlay_behavior::UseOverlayTrigger.materialize()>"use_overlay_trigger"</Link>" as "<Code inline=true>"overlay_id"</Code>"."
                    </ApiRow>
                    <ApiRow name="popover_element" ty="CapturedElement">"The popover element, once rendered."</ApiRow>
                    <ApiRow name="arrow_props" ty="PropsWithStyles<UseOverlayArrowProps>">
                        "For an arrow element inside the popover: hidden from assistive technology and placed along the edge "
                        "facing the trigger."
                    </ApiRow>
                    <ApiRow name="placement" ty="Signal<Option<PlacementAxis>>">
                        "The side of the trigger the popover opened on, after flipping; "<Code inline=true>"None"</Code>
                        " until positioned."
                    </ApiRow>
                    <ApiRow name="trigger_anchor_point" ty="Signal<Option<Point>>">
                        "The point of the popover closest to the trigger, in its own coordinates, e.g. as "
                        <Code inline=true>"transform-origin"</Code>"."
                    </ApiRow>
                </ApiTable>
            </Section>

            <Section title="Example">
                <p>
                    "A modal popover below a button. "<Code inline=true>"use_popover"</Code>" positions and dismisses it; the "
                    "rest is wiring you do yourself:"
                </p>
                <ul>
                    <li>
                        "The trigger is a button built with "<Link href=routes::doc::button::Hook.materialize()>"use_button"</Link>
                        " that toggles the state. "<Link href=routes::doc::overlay_behavior::UseOverlayTrigger.materialize()>"use_overlay_trigger"</Link>
                        " gives it "<Code inline=true>"aria-expanded"</Code>" and "<Code inline=true>"aria-controls"</Code>
                        ", and "<Code inline=true>"trigger_props"</Code>" capture it for positioning."
                    </li>
                    <li>
                        "The popover renders into a "<Code inline=true>"Portal"</Code>", so the page layout doesn\u{2019}t clip "
                        "it, behind an "<AnchorLink href="#underlay">"underlay"</AnchorLink>"."
                    </li>
                    <li>
                        "A "<Link href=routes::doc::focus::FocusScope.materialize()>"FocusScope"</Link>" moves focus into the "
                        "popover (the hook\u{2019}s "<Keys keys="Escape"/>" handler sits on it), keeps it inside and returns it "
                        "to the button. The popover is a dialog named by its heading."
                    </li>
                </ul>

                <Demo description="Modal popover opened by a button" source=include_str!("demos/popover_basic.rs") source_open=true>
                    <BasicPopoverDemo/>
                </Demo>
            </Section>

            <Section title="Positioning">
                <p>
                    <Code inline=true>"placement"</Code>" names the side of the trigger and the alignment along it, e.g. "
                    <Code inline=true>"Placement::BottomStart"</Code>" (below, start edges aligned) or "
                    <Code inline=true>"Placement::Right"</Code>" (right of it, vertically centered). See "
                    <Link href=format!("{}#placements", routes::doc::overlay_behavior::UseOverlayPosition.materialize())>"Placements"</Link>" for all of them. "<Code inline=true>"Start"</Code>" and "
                    <Code inline=true>"End"</Code>" follow the writing direction of the enclosing "
                    <Code inline=true>"I18nProvider"</Code>". Positioning comes from "
                    <Link href=routes::doc::overlay_behavior::UseOverlayPosition.materialize()>"use_overlay_position"</Link>"."
                </p>
                <p>
                    "When "<Code inline=true>"should_flip"</Code>" is set and the popover doesn\u{2019}t fit, it moves to the "
                    "opposite side. The returned "<Code inline=true>"placement"</Code>" tells where it ended up, e.g. to "
                    "point an arrow at the trigger ("<Code inline=true>"arrow_props"</Code>" positions one)."
                </p>

                <Demo description="Popover placement chosen with radio buttons" source=include_str!("demos/popover_placement.rs")>
                    <PlacementPopoverDemo/>
                </Demo>
            </Section>

            <Section title="Dismiss Behavior">
                <p>
                    "Dismissal comes from "<Link href=routes::doc::overlay_behavior::UseOverlay.materialize()>"use_overlay"</Link>", which keeps a stack of "
                    "open overlays:"
                </p>

                <ul>
                    <li>
                        <Keys keys="Escape"/>" closes the topmost popover while focus is inside it, unless an IME composition "
                        "is in progress or "<Code inline=true>"is_keyboard_dismiss_disabled"</Code>" is set."
                    </li>
                    <li>
                        "A press outside closes a modal popover if it is the topmost overlay. Use "
                        <Code inline=true>"should_close_on_interact_outside"</Code>" to exempt elements, such as a toolbar "
                        "that should keep the popover open."
                    </li>
                    <li>
                        "Moving focus out of the popover closes it, modal or not. Focus moving into a nested overlay, or "
                        "lost to the page body, doesn\u{2019}t."
                    </li>
                    <li>"A non-modal popover also closes when a scrollable ancestor of its trigger scrolls."</li>
                </ul>
            </Section>

            <Section title="Modal and Non-Modal">
                <p><Code inline=true>"modality"</Code>" decides how much the popover takes over the page:"</p>

                <DocTable headers=&["Behavior", "Modal", "NonModal"]>
                    <TableRow><TableCell>"Page scrolling"</TableCell><TableCell>"Prevented"</TableCell><TableCell>"Allowed; closes the popover"</TableCell></TableRow>
                    <TableRow><TableCell>"A press outside closes it"</TableCell><TableCell>"Yes"</TableCell><TableCell>"No"</TableCell></TableRow>
                    <TableRow><TableCell>"Focus leaving closes it"</TableCell><TableCell>"Yes"</TableCell><TableCell>"Yes"</TableCell></TableRow>
                    <TableRow>
                        <TableCell>"The rest of the page"</TableCell>
                        <TableCell>"Inert: hidden from assistive technology and not interactive"</TableCell>
                        <TableCell>"Usable; kept visible even when another modal overlay hides the page"</TableCell>
                    </TableRow>
                    <TableRow><TableCell>"Underlay element"</TableCell><TableCell>"Recommended"</TableCell><TableCell>"None"</TableCell></TableRow>
                </DocTable>

                <p>
                    "The hook doesn\u{2019}t move or contain focus in either case: wrap the content in a "
                    <Link href=routes::doc::focus::FocusScope.materialize()>"FocusScope"</Link>" ("
                    <Code inline=true>"contain"</Code>" for a modal popover)."
                </p>

                <Section title="Non-Modal Demo">
                    <p>
                        "A press on \u{201c}Add item\u{201d} reaches the page while the popover is open: it counts, and as it "
                        "moves focus out of the popover, closes it."
                    </p>

                    <Demo description="Non-modal popover next to a button that keeps working" source=include_str!("demos/popover_non_modal.rs")>
                        <NonModalPopoverDemo/>
                    </Demo>
                </Section>
            </Section>

            <Section title="Underlay">
                <p>
                    "A modal popover gets an underlay: a "<Code inline=true>"position: fixed; inset: 0"</Code>
                    " element behind the popover that catches presses on the page, so they close the popover instead of "
                    "reaching what\u{2019}s below. It needs no props. A non-modal popover has none."
                </p>

                <Code language=Language::Rust>
                    {indoc!(r#"
                        <Show when=move || is_open.get()>
                            <div class="underlay"/>
                            <div {..popover_attrs.get_value()} style=popover_styles.get_value()>
                                "Popover content"
                            </div>
                        </Show>
                    "#)}
                </Code>
            </Section>

            <Section title="Composition">
                <p><Code inline=true>"use_popover"</Code>" combines lower-level hooks:"</p>
                <ul>
                    <li><Link href=routes::doc::overlay_behavior::UseOverlay.materialize()>"use_overlay"</Link>" for dismissal and the overlay stack."</li>
                    <li><Link href=routes::doc::overlay_behavior::UseOverlayPosition.materialize()>"use_overlay_position"</Link>" for placing the popover next to the trigger."</li>
                    <li>
                        <Link href=routes::doc::overlay_behavior::UsePreventScroll.materialize()>"use_prevent_scroll"</Link>
                        " to stop page scrolling while a modal popover is open."
                    </li>
                    <li>
                        <Link href=routes::doc::overlay_behavior::AriaHideOutside.materialize()>"aria_hide_outside"</Link>
                        " to make the rest of the page inert while a modal popover is open; a non-modal popover stays "
                        "visible ("<Code inline=true>"keep_visible"</Code>") even when another modal overlay hid the page."
                    </li>
                </ul>
                <p>
                    "It renders nothing and leaves the trigger to you. The "<Link href=routes::doc::menu::Hook.materialize()>"menu"</Link>", "
                    <Link href=routes::doc::select::Hook.materialize()>"select"</Link>" and "
                    <Link href=routes::doc::combobox::Hook.materialize()>"combo box"</Link>" hooks pair it with their own "
                    "states, triggers and keyboard navigation."
                </p>
            </Section>

            <Section title="Keyboard">
                <KeyboardTable>
                    <KeyRow keys="Escape">"Closes the popover."</KeyRow>
                    <KeyRow keys="Tab">"Moves focus; leaving the popover closes it. Contain focus with a FocusScope."</KeyRow>
                </KeyboardTable>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Popover.materialize()>"Popover overview"</Link></li>
                <li><Link href=routes::doc::popover::Atom.materialize()>"Popover Atoms"</Link></li>
                <li><Link href=routes::doc::OverlayBehavior.materialize()>"Overlay Behavior"</Link></li>
                <li><Link href=routes::doc::focus::FocusScope.materialize()>"FocusScope"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
