use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::{
    popover_basic::BasicPopoverDemo, popover_non_modal::NonModalPopoverDemo,
    popover_placement::PlacementPopoverDemo,
};
use crate::{kit::*, routes};

#[component]
pub fn PageUsePopoverHook() -> impl IntoView {
    view! {
        <DocPage title="use_popover">
            <p>
                "The "<Code inline=true>"use_popover"</Code>" hook provides the behavior of an overlay positioned next to a "
                "trigger: placement, dismissal, scroll prevention and hiding the rest of the page from assistive technology. "
                "See the "<Link href=routes::doc::Popover.materialize()>"Popover overview"</Link>" for concept guidance."
            </p>

            <ReactAria hook="usePopover"/>

            <Section title="Input">
                <p>
                    <Code inline=true>"UsePopoverInput::new(state)"</Code>" sets the defaults (below the trigger, centered, flipping, "
                    "modal); change single fields with struct update syntax."
                </p>

                <ApiTable kind=ApiKind::Input of="UsePopoverInput">
                    <ApiRow name="state" ty="OverlayTriggerState">
                        "Whether the popover is open ("<Code inline=true>"use_overlay_trigger_state"</Code>", or "<Code inline=true>"(read, write).into()"</Code>
                        "). Escape, outside interaction and blur close it; a non-modal popover also closes when the page scrolls."
                    </ApiRow>
                    <ApiRow name="trigger" ty="CapturedElement">
                        "The trigger the popover is positioned at. Spread "<Code inline=true>"trigger_props"</Code>" onto it, or pass an "
                        "element you capture already."
                    </ApiRow>
                    <ApiRow name="placement_x" ty="Signal<PlacementX>">"Horizontal placement relative to the trigger."</ApiRow>
                    <ApiRow name="placement_y" ty="Signal<PlacementY>">"Vertical placement relative to the trigger."</ApiRow>
                    <ApiRow name="offset" ty="Signal<f64>">
                        "Extra distance from the trigger along the main axis, in pixels. Usually "<Code inline=true>"0.0"</Code>"."
                    </ApiRow>
                    <ApiRow name="cross_offset" ty="Signal<f64>">
                        "Shift along the cross axis, in pixels. Usually "<Code inline=true>"0.0"</Code>"."
                    </ApiRow>
                    <ApiRow name="container_padding" ty="Signal<f64>">
                        "Minimum distance to the viewport edge, in pixels. Usually "<Code inline=true>"12.0"</Code>"."
                    </ApiRow>
                    <ApiRow name="should_flip" ty="Signal<bool>">
                        "Flip to the opposite side when there is not enough space. Usually "<Code inline=true>"true"</Code>"."
                    </ApiRow>
                    <ApiRow name="modality" ty="PopoverModality">
                        <Code inline=true>"NonModal"</Code>" keeps the page interactive while the popover is open."
                    </ApiRow>
                    <ApiRow name="is_keyboard_dismiss_disabled" ty="bool">"Ignore Escape."</ApiRow>
                    <ApiRow name="should_close_on_interact_outside" ty="Option<Callback<web_sys::Element, bool>>">
                        "Decides per outside element whether interacting with it closes the popover. "
                        <Code inline=true>"None"</Code>" closes on every outside interaction."
                    </ApiRow>
                </ApiTable>
            </Section>

            <Section title="Return">
                <ApiTable kind=ApiKind::Return of="UsePopoverReturn">
                    <ApiRow name="popover_element" ty="CapturedElement">"The popover element, once rendered."</ApiRow>
                    <ApiRow name="props" ty="PropsWithStyles<UsePopoverProps>">
                        "Attributes, event handlers and position styles for the popover element. Call "
                        <Code inline=true>"props.into_parts()"</Code>" to get "<Code inline=true>"(attrs, styles)"</Code>
                        ", then spread "<Code inline=true>"{..attrs}"</Code>" and set "<Code inline=true>"style=styles"</Code>"."
                    </ApiRow>
                    <ApiRow name="trigger_props" ty="UsePopoverTriggerProps">
                        "Spread "<Code inline=true>"trigger_props.into_attrs()"</Code>
                        " onto the trigger, so the hook can position the popover next to it."
                    </ApiRow>
                    <ApiRow name="id" ty="Oco<'static, str>">
                        "The id of the popover element. Pass it to "<Code inline=true>"use_overlay_trigger"</Code>" as "
                        <Code inline=true>"overlay_id"</Code>"."
                    </ApiRow>
                    <ApiRow name="resolved_placement_x" ty="Memo<PhysicalPlacementX>">
                        "The horizontal placement after flipping, with logical placements resolved to left or right."
                    </ApiRow>
                    <ApiRow name="resolved_placement_y" ty="Memo<PlacementY>">"The vertical placement after flipping."</ApiRow>
                </ApiTable>
            </Section>

            <Section title="Example">
                <p>
                    "A modal popover below a button. The popover renders into a "<Code inline=true>"Portal"</Code>
                    ", so it is not clipped by the page layout. Press Escape or click outside to close it. The trigger "
                    "combines three hooks: "<Link href=routes::doc::button::Hook.materialize()>"use_button"</Link>
                    " toggles the popover on press, "<Code inline=true>"use_overlay_trigger"</Code>" provides its "
                    <Code inline=true>"aria-expanded"</Code>" and "<Code inline=true>"aria-controls"</Code>
                    ", which go into "<Code inline=true>"UseButtonInput"</Code>", and "<Code inline=true>"trigger_props"</Code>
                    " capture the button for positioning."
                </p>

                <Demo description="Modal popover opened by a button" source=include_str!("demos/popover_basic.rs") source_open=true>
                    <BasicPopoverDemo/>
                </Demo>
            </Section>

            <Section title="Positioning">
                <p>"The placement is set on two independent axes:"</p>

                <DocTable headers=&["Axis", "Values"]>
                    <TableRow>
                        <TableCell><Code inline=true>"PlacementX"</Code></TableCell>
                        <TableCell>
                            <Code inline=true>"OuterLeft"</Code>", "<Code inline=true>"OuterStart"</Code>", "
                            <Code inline=true>"Start"</Code>", "<Code inline=true>"Left"</Code>", "
                            <Code inline=true>"Center"</Code>", "<Code inline=true>"Right"</Code>", "
                            <Code inline=true>"End"</Code>", "<Code inline=true>"OuterEnd"</Code>", "
                            <Code inline=true>"OuterRight"</Code>
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"PlacementY"</Code></TableCell>
                        <TableCell>
                            <Code inline=true>"Above"</Code>", "<Code inline=true>"Top"</Code>", "
                            <Code inline=true>"Center"</Code>", "<Code inline=true>"Bottom"</Code>", "
                            <Code inline=true>"Below"</Code>
                        </TableCell>
                    </TableRow>
                </DocTable>

                <p>
                    "The "<Code inline=true>"Outer"</Code>" values and "<Code inline=true>"Above"</Code>" / "
                    <Code inline=true>"Below"</Code>" place the popover next to the trigger; the others align it with an edge "
                    "or the center of the trigger. "<Code inline=true>"Start"</Code>" and "<Code inline=true>"End"</Code>
                    " follow the writing direction of the enclosing "<Code inline=true>"I18nProvider"</Code>
                    "\u{2019}s locale: in a left-to-right layout (also without a provider), "
                    <Code inline=true>"Start"</Code>" means "<Code inline=true>"Left"</Code>", in a right-to-left layout "
                    <Code inline=true>"Right"</Code>"."
                </p>

                <p>
                    "When "<Code inline=true>"should_flip"</Code>" is set and the popover does not fit, it moves to the "
                    "opposite side. "<Code inline=true>"resolved_placement_x"</Code>" and "
                    <Code inline=true>"resolved_placement_y"</Code>" tell you where it ended up, for example to point an "
                    "arrow at the trigger."
                </p>

                <Section title="Placement Demo">
                    <p>
                        "Pick a placement and open the popover. The logical values ("<Code inline=true>"Start"</Code>", "
                        <Code inline=true>"End"</Code>", \u{2026}) work the same and are left out here."
                    </p>

                    <Demo description="Popover placement chosen with radio buttons" source=include_str!("demos/popover_placement.rs")>
                        <PlacementPopoverDemo/>
                    </Demo>
                </Section>
            </Section>

            <Section title="Dismiss Behavior">
                <p>
                    "Dismissal is handled by "<Link href=routes::doc::overlays::UseOverlay.materialize()>"use_overlay"</Link>
                    ", which keeps a stack of open overlays. Only the topmost one reacts:"
                </p>

                <ul>
                    <li>
                        <b>"Escape"</b>" closes the popover, unless an IME composition is in progress or "
                        <Code inline=true>"is_keyboard_dismiss_disabled"</Code>" is set."
                    </li>
                    <li>
                        <b>"Outside interaction"</b>" (pointer down followed by a click outside the popover) closes a modal "
                        "popover. Use "<Code inline=true>"should_close_on_interact_outside"</Code>
                        " to exempt elements, such as a toolbar that should keep the popover open."
                    </li>
                    <li><b>"Blur"</b>": the popover closes when focus moves out of it. This is always on."</li>
                </ul>
            </Section>

            <Section title="Modal vs Non-Modal">
                <p><Code inline=true>"modality"</Code>" decides how much the popover takes over the page:"</p>

                <DocTable headers=&["Behavior", "Modal", "NonModal"]>
                    <TableRow><TableCell>"Page scrolling"</TableCell><TableCell>"Prevented"</TableCell><TableCell>"Allowed; closes the popover"</TableCell></TableRow>
                    <TableRow><TableCell>"Outside interaction closes"</TableCell><TableCell>"Yes"</TableCell><TableCell>"No"</TableCell></TableRow>
                    <TableRow><TableCell>"Blur closes"</TableCell><TableCell>"Yes"</TableCell><TableCell>"Yes"</TableCell></TableRow>
                    <TableRow>
                        <TableCell>"Rest of the page for assistive technology"</TableCell>
                        <TableCell>"Hidden"</TableCell>
                        <TableCell>"Visible (kept visible even when another modal overlay hides the page)"</TableCell>
                    </TableRow>
                    <TableRow><TableCell>"Underlay element"</TableCell><TableCell>"Recommended"</TableCell><TableCell>"Not needed"</TableCell></TableRow>
                </DocTable>

                <Section title="Non-Modal Demo">
                    <p>
                        "While the non-modal popover is open, you can still use the counter. Scrolling the page closes the "
                        "popover, so that it doesn\u{2019}t drift away from its trigger."
                    </p>

                    <Demo description="Non-modal popover next to a counter button" source=include_str!("demos/popover_non_modal.rs")>
                        <NonModalPopoverDemo/>
                    </Demo>
                </Section>
            </Section>

            <Section title="Underlay Element">
                <p>
                    "A modal popover usually gets an underlay: a "<Code inline=true>"position: fixed; inset: 0"</Code>
                    " element behind the popover that catches clicks on the page, so they close the popover instead of "
                    "reaching what\u{2019}s below. It needs no props. Non-modal popovers don\u{2019}t use one."
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

            <Section title="Hook Composition">
                <p><Code inline=true>"use_popover"</Code>" combines lower-level hooks:"</p>

                <ul>
                    <li>
                        <Link href=routes::doc::overlays::UseOverlay.materialize()>"use_overlay"</Link>
                        " for dismissal and the overlay stack."
                    </li>
                    <li><Code inline=true>"use_overlay_position"</Code>" for placing the popover next to the trigger."</li>
                    <li>
                        <Link href=routes::doc::interactions::UsePreventScroll.materialize()>"use_prevent_scroll"</Link>
                        " to stop page scrolling while a modal popover is open."
                    </li>
                </ul>

                <p>
                    "It does not render or manage the trigger. Pair it with "<Code inline=true>"use_overlay_trigger"</Code>
                    " for the trigger\u{2019}s ARIA attributes. For menus, selects and comboboxes, use their dedicated hooks: "
                    "they add keyboard navigation and the matching roles."
                </p>

                <DocTable headers=&["Use case", "Hooks"]>
                    <TableRow><TableCell>"Info popover, profile card, small form"</TableCell><TableCell><Code inline=true>"use_popover"</Code></TableCell></TableRow>
                    <TableRow>
                        <TableCell>"Menu with keyboard navigation"</TableCell>
                        <TableCell>
                            <Link href=routes::doc::menu::Hook.materialize()>"use_menu_trigger"</Link>", "
                            <Code inline=true>"use_menu"</Code>", "<Code inline=true>"use_menu_item"</Code>
                        </TableCell>
                    </TableRow>
                    <TableRow><TableCell>"Pick a value from a dropdown"</TableCell><TableCell><Link href=routes::doc::select::Hook.materialize()>"use_select"</Link></TableCell></TableRow>
                    <TableRow><TableCell>"Autocomplete"</TableCell><TableCell><Code inline=true>"use_combobox"</Code></TableCell></TableRow>
                    <TableRow>
                        <TableCell>"Modal dialog"</TableCell>
                        <TableCell><Link href=routes::doc::modal::Hook.materialize()>"use_modal_backdrop, use_modal, use_dialog"</Link></TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Accessibility">
                <ul>
                    <li>
                        "Pass the returned "<Code inline=true>"id"</Code>" to "<Code inline=true>"use_overlay_trigger"</Code>
                        ". It sets "<Code inline=true>"aria-expanded"</Code>" and "<Code inline=true>"aria-controls"</Code>
                        " on the trigger ("<Code inline=true>"aria-haspopup"</Code>" only for menu and listbox popups)."
                    </li>
                    <li>
                        "A modal popover hides the rest of the page from assistive technology while it is open. Wrap its content "
                        "in a "<Link href=routes::doc::focus::FocusScope.materialize()>"FocusScope"</Link>" with "
                        <Code inline=true>"contain=true"</Code>" to keep keyboard focus inside."
                    </li>
                    <li>
                        "Popovers don\u{2019}t set "<Code inline=true>"aria-modal"</Code>". For a modal dialog, use the "
                        <Link href=routes::doc::modal::Hook.materialize()>"modal hooks"</Link>"."
                    </li>
                </ul>

                <KeyboardTable>
                    <KeyRow keys="Escape">"Close the popover."</KeyRow>
                    <KeyRow keys="Tab">"Move focus. Leaving the popover closes it."</KeyRow>
                </KeyboardTable>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Popover.materialize()>"Popover overview"</Link></li>
                <li><Link href=routes::doc::popover::Atom.materialize()>"Popover atom"</Link></li>
                <li><Link href=routes::doc::popover::Component.materialize()>"Popover component"</Link></li>
                <li><Link href=routes::doc::Overlays.materialize()>"Overlays overview"</Link></li>
                <li><Link href=routes::doc::overlays::UseOverlay.materialize()>"use_overlay"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
