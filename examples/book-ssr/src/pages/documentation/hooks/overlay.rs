use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::{overlay_basic::BasicOverlayDemo, overlay_positioning::PositioningDemo};
use crate::{kit::*, routes};

#[component]
pub fn PageUseOverlay() -> impl IntoView {
    view! {
        <DocPage title="Overlay Hooks">
            <p>
                "Low-level primitives for floating content: dismiss behavior, ARIA attributes for the trigger and positioning "
                "next to a target. "<Link href=routes::doc::popover::Hook.materialize()>"use_popover"</Link>", "
                <Link href=routes::doc::modal::Hook.materialize()>"use_modal_backdrop"</Link>" and the "
                <Link href=routes::doc::popover::Atom.materialize()>"Popover atom"</Link>" are built from them. "
                "See the "<Link href=routes::doc::Overlays.materialize()>"Overlays overview"</Link>" for domain guidance."
            </p>

            <ReactAria hook="useOverlay"/>

            <Section title="Hook Composition">
                <p>"The overlay system is split into single-responsibility hooks:"</p>

                <DocTable headers=&["Hook", "Responsibility"]>
                    <TableRow>
                        <TableCell><a href="#use-overlay"><Code inline=true>"use_overlay"</Code></a></TableCell>
                        <TableCell>
                            "Dismiss behavior (Escape, a press outside, blur) and overlay stacking: when several overlays are "
                            "open, only the topmost one closes."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><a href="#use-overlay-trigger"><Code inline=true>"use_overlay_trigger"</Code></a></TableCell>
                        <TableCell>
                            "ARIA attributes of the trigger element: "<Code inline=true>"aria-haspopup"</Code>", "
                            <Code inline=true>"aria-expanded"</Code>" and "<Code inline=true>"aria-controls"</Code>"."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><a href="#use-overlay-position"><Code inline=true>"use_overlay_position"</Code></a></TableCell>
                        <TableCell>"Positions the overlay relative to a target element, flipping it when there is not enough space."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><a href="#use-close-on-scroll"><Code inline=true>"use_close_on_scroll"</Code></a></TableCell>
                        <TableCell>"Closes the overlay when a scrollable ancestor of the trigger scrolls."</TableCell>
                    </TableRow>
                </DocTable>

                <p>
                    "You can use them individually or combine them. "<Code inline=true>"use_overlay"</Code>
                    " is the core; trigger attributes and positioning are optional, depending on what you build."
                </p>
            </Section>

            <Section title="When to Use">
                <p>"These hooks are low-level primitives. Reach for a higher-level hook when one fits:"</p>

                <DocTable headers=&["Use case", "Hook(s)"]>
                    <TableRow>
                        <TableCell>"Custom overlay with full control"</TableCell>
                        <TableCell>
                            <Code inline=true>"use_overlay"</Code>" + "<Code inline=true>"use_overlay_trigger"</Code>" + "
                            <Code inline=true>"use_overlay_position"</Code>
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Popover anchored to a trigger"</TableCell>
                        <TableCell><Link href=routes::doc::popover::Hook.materialize()>"use_popover"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Menu with keyboard navigation"</TableCell>
                        <TableCell>
                            <Link href=routes::doc::menu::Hook.materialize()>"use_menu"</Link>" + "
                            <Code inline=true>"use_menu_trigger"</Code>
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Modal dialog"</TableCell>
                        <TableCell>
                            <Link href=routes::doc::modal::Hook.materialize()>"use_modal"</Link>" + "
                            <Code inline=true>"use_modal_backdrop"</Code>" + "<Code inline=true>"use_dialog"</Code>
                        </TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Demo">
                <p>
                    "Only "<Code inline=true>"use_overlay"</Code>": a panel with an underlay that closes on Escape and on a "
                    "press outside. A "<Link href=routes::doc::focus::FocusScope.materialize()>"FocusScope"</Link>
                    " moves focus into the panel, so the Escape key reaches it, and restores focus when it closes."
                </p>

                <Demo
                    description="Overlay panel dismissed with Escape or a press on the underlay"
                    source=include_str!("demos/overlay_basic.rs")
                >
                    <BasicOverlayDemo/>
                </Demo>
            </Section>

            <Section title="use_overlay">
                <p>
                    "Handles dismissing an overlay. Spread "<Code inline=true>"props"</Code>" onto the overlay element. An "
                    "underlay behind it (to catch presses on the page) is a plain element that needs no props."
                </p>

                <Section title="Input" id="use-overlay-input">
                    <p>"All fields are required; "<Code inline=true>"UseOverlayInput"</Code>" has no default."</p>

                    <ApiTable kind=ApiKind::Input of="UseOverlayInput">
                        <ApiRow name="is_open" ty="Signal<bool>">"Whether the overlay is open."</ApiRow>
                        <ApiRow name="on_close" ty="Callback<()>">"Called when the overlay should close."</ApiRow>
                        <ApiRow name="is_dismissable" ty="bool">
                            "Close the overlay when the user presses outside of it."
                        </ApiRow>
                        <ApiRow name="should_close_on_blur" ty="bool">"Close the overlay when focus leaves it."</ApiRow>
                        <ApiRow name="is_keyboard_dismiss_disabled" ty="bool">
                            "Don\u{2019}t close the overlay on Escape."
                        </ApiRow>
                        <ApiRow name="should_close_on_interact_outside" ty="Option<Callback<web_sys::Element, bool>>">
                            "Filter for outside interactions and blur: receives the element the user interacted with (or "
                            "that received focus) and returns whether the overlay should close. "<Code inline=true>"None"</Code>
                            " closes on every outside interaction."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-overlay-return">
                    <ApiTable kind=ApiKind::Return of="UseOverlayReturn">
                        <ApiRow name="props" ty="UseOverlayProps">
                            "The overlay\u{2019}s "<Code inline=true>"id"</Code>", its element capture and the "
                            <Code inline=true>"keydown"</Code>", "<Code inline=true>"focusin"</Code>" and "
                            <Code inline=true>"focusout"</Code>" handlers. Spread "<Code inline=true>"{..props.into_attrs()}"</Code>
                            " onto the overlay element."
                        </ApiRow>
                        <ApiRow name="id" ty="Oco<'static, str>">
                            "The overlay\u{2019}s id. Pass it to "<Code inline=true>"use_overlay_trigger"</Code>" as "
                            <Code inline=true>"overlay_id"</Code>"."
                        </ApiRow>
                        <ApiRow name="overlay_element" ty="CapturedElement">
                            "The overlay element once it is rendered, e.g. for "<Code inline=true>"aria_hide_outside"</Code>"."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Example" id="use-overlay-example">
                    <Code language=Language::Rust>
                        {indoc!(r#"
                            let (is_open, set_is_open) = signal(false);

                            let UseOverlayReturn { props, .. } = use_overlay(UseOverlayInput {
                                is_open: is_open.into(),
                                on_close: Callback::new(move |()| set_is_open.set(false)),
                                is_dismissable: true,
                                should_close_on_blur: false,
                                is_keyboard_dismiss_disabled: false,
                                should_close_on_interact_outside: None,
                            });
                            let attrs = StoredValue::new(props.into_attrs());

                            view! {
                                <Show when=move || is_open.get()>
                                    <div {..attrs.get_value()}>"Overlay content"</div>
                                </Show>
                            }
                        "#)}
                    </Code>
                </Section>

                <Section title="Dismiss Behavior">
                    <p><Code inline=true>"use_overlay"</Code>" provides three independent ways to dismiss an overlay."</p>

                    <Section title="Escape Key">
                        <p>
                            "Pressing Escape while focus is inside the overlay closes the topmost overlay. The event is stopped "
                            "and its default prevented, so it doesn\u{2019}t reach outer overlays. Escape is ignored while an IME "
                            "composition is in progress. Disable it with "<Code inline=true>"is_keyboard_dismiss_disabled: true"</Code>"."
                        </p>
                        <p>
                            "The key handler sits on the overlay element, so move focus into the overlay when it opens, for example "
                            "with a "<Link href=routes::doc::focus::FocusScope.materialize()>"FocusScope"</Link>"."
                        </p>
                    </Section>

                    <Section title="Click Outside">
                        <p>
                            "Enabled with "<Code inline=true>"is_dismissable: true"</Code>". A press outside the overlay closes it "
                            "if it was the topmost overlay when the press started. The press isn\u{2019}t cancelled: the outside "
                            "element still gets its default action (a link navigates, a checkbox toggles), but its own event "
                            "handlers don\u{2019}t run. "
                            <Code inline=true>"should_close_on_interact_outside"</Code>" lets you exclude elements, for example "
                            "a trigger that toggles the overlay itself."
                        </p>
                    </Section>

                    <Section title="Blur">
                        <p>
                            "Enabled with "<Code inline=true>"should_close_on_blur: true"</Code>". The overlay closes when focus "
                            "moves to an element outside of it. Unlike Escape and outside presses, blur closes any overlay, not "
                            "only the topmost. Focus moving into a child focus scope (e.g. a menu opened from a dialog) or "
                            "being lost to the page body (e.g. when switching browser tabs) does not close the overlay."
                        </p>
                    </Section>

                    <Section title="Overlay Stacking">
                        <p>
                            "Open overlays are tracked on a stack in the order they opened. Escape and outside presses only close "
                            "the topmost overlay, so opening a nested overlay doesn\u{2019}t close its parent."
                        </p>
                    </Section>
                </Section>
            </Section>

            <Section title="use_overlay_trigger">
                <p>"Sets the ARIA attributes that connect a trigger element to the overlay it opens."</p>

                <Section title="Input" id="use-overlay-trigger-input">
                    <ApiTable kind=ApiKind::Input of="UseOverlayTriggerInput">
                        <ApiRow name="show" ty="Signal<bool>">"Whether the overlay is shown."</ApiRow>
                        <ApiRow name="overlay_id" ty="Oco<'static, str>">
                            "The id of the overlay, as returned by "<Code inline=true>"use_overlay"</Code>"."
                        </ApiRow>
                        <ApiRow name="overlay_type" ty="OverlayTriggerType">
                            "What the trigger opens: "<Code inline=true>"Dialog"</Code>", "<Code inline=true>"Menu"</Code>", "
                            <Code inline=true>"Listbox"</Code>", "<Code inline=true>"Tree"</Code>" or "<Code inline=true>"Grid"</Code>"."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-overlay-trigger-return">
                    <ApiTable kind=ApiKind::Return of="UseOverlayTriggerReturn">
                        <ApiRow name="props" ty="UseOverlayTriggerProps">
                            "Spread "<Code inline=true>"{..props.into_attrs()}"</Code>" onto the trigger. Sets "
                            <Code inline=true>"aria-expanded"</Code>", "<Code inline=true>"aria-controls"</Code>
                            " (only while the overlay is shown) and "<Code inline=true>"aria-haspopup"</Code>" ("
                            <Code inline=true>"true"</Code>" for menus, "<Code inline=true>"listbox"</Code>
                            " for listboxes, omitted otherwise, because screen readers announce other values as a menu)."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Example" id="use-overlay-trigger-example">
                    <Code language=Language::Rust>
                        {indoc!(r#"
                            let UseOverlayReturn { props: overlay_props, id, .. } = use_overlay(/* ... */);

                            let UseOverlayTriggerReturn { props: trigger_props } = use_overlay_trigger(UseOverlayTriggerInput {
                                show: is_open.into(),
                                overlay_id: id,
                                overlay_type: OverlayTriggerType::Dialog,
                            });

                            view! { <button {..trigger_props.into_attrs()}>"Open"</button> }
                        "#)}
                    </Code>
                </Section>
            </Section>

            <Section title="use_overlay_position">
                <p>
                    "Positions the overlay relative to a target element. The overlay uses "<Code inline=true>"position: fixed"</Code>
                    ", so render it in a "<Code inline=true>"<Portal>"</Code>". The position follows the target when the page scrolls "
                    "or the target, the overlay or the viewport is resized."
                </p>

                <Section title="Input" id="use-overlay-position-input">
                    <p>"All fields are required; "<Code inline=true>"UseOverlayPositionInput"</Code>" has no default."</p>

                    <ApiTable kind=ApiKind::Input of="UseOverlayPositionInput">
                        <ApiRow name="target" ty="CapturedElement">
                            "The element to position the overlay against. Spread "<Code inline=true>"{..target.attr()}"</Code>
                            " onto it. The overlay element is captured through the returned props."
                        </ApiRow>
                        <ApiRow name="placement_x" ty="Signal<PlacementX>">
                            "Horizontal placement: "<Code inline=true>"OuterLeft"</Code>", "<Code inline=true>"Left"</Code>", "
                            <Code inline=true>"Center"</Code>", "<Code inline=true>"Right"</Code>", "<Code inline=true>"OuterRight"</Code>
                            ", or the writing-direction-aware "<Code inline=true>"OuterStart"</Code>", "<Code inline=true>"Start"</Code>", "
                            <Code inline=true>"End"</Code>" and "<Code inline=true>"OuterEnd"</Code>", which follow the locale of the "
                            "enclosing "<Code inline=true>"I18nProvider"</Code>" (left-to-right without one)."
                        </ApiRow>
                        <ApiRow name="placement_y" ty="Signal<PlacementY>">
                            "Vertical placement: "<Code inline=true>"Above"</Code>", "<Code inline=true>"Top"</Code>", "
                            <Code inline=true>"Center"</Code>", "<Code inline=true>"Bottom"</Code>" or "<Code inline=true>"Below"</Code>"."
                        </ApiRow>
                        <ApiRow name="offset" ty="Signal<f64>">
                            "Additional distance along the main axis, away from the target (usually "<Code inline=true>"0.0"</Code>")."
                        </ApiRow>
                        <ApiRow name="cross_offset" ty="Signal<f64>">
                            "Additional offset along the cross axis (usually "<Code inline=true>"0.0"</Code>")."
                        </ApiRow>
                        <ApiRow name="container_padding" ty="Signal<f64>">
                            "Minimum distance between the overlay and the viewport edges, e.g. "<Code inline=true>"12.0"</Code>"."
                        </ApiRow>
                        <ApiRow name="should_flip" ty="Signal<bool>">
                            "Flip the overlay to the opposite side when there isn\u{2019}t enough space."
                        </ApiRow>
                        <ApiRow name="max_height" ty="Option<Signal<f64>>">
                            "Maximum height of the overlay. With "<Code inline=true>"None"</Code>
                            ", it is computed from the available space."
                        </ApiRow>
                        <ApiRow name="is_open" ty="Signal<bool>">
                            "Whether the overlay is open. No position is computed while it is closed."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-overlay-position-return">
                    <ApiTable kind=ApiKind::Return of="UseOverlayPositionReturn">
                        <ApiRow name="props" ty="PropsWithStyles<UseOverlayPositionProps>">
                            "The overlay\u{2019}s element capture and its styles ("<Code inline=true>"position: fixed"</Code>", "
                            <Code inline=true>"z-index"</Code>", "<Code inline=true>"top"</Code>", "<Code inline=true>"left"</Code>", "
                            <Code inline=true>"max-height"</Code>"). Call "<Code inline=true>"props.into_parts()"</Code>
                            " to get "<Code inline=true>"(attrs, styles)"</Code>"."
                        </ApiRow>
                        <ApiRow name="resolved_placement_x" ty="Memo<PhysicalPlacementX>">
                            "The horizontal placement after resolving the writing direction and flipping."
                        </ApiRow>
                        <ApiRow name="resolved_placement_y" ty="Memo<PlacementY>">
                            "The vertical placement after flipping."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Example" id="use-overlay-position-example">
                    <Code language=Language::Rust>
                        {indoc!(r#"
                            let target = CapturedElement::new();

                            let UseOverlayPositionReturn { props, .. } = use_overlay_position(UseOverlayPositionInput {
                                target,
                                placement_x: PlacementX::Center.into(),
                                placement_y: PlacementY::Below.into(),
                                offset: 4.0.into(),
                                cross_offset: 0.0.into(),
                                container_padding: 12.0.into(),
                                should_flip: true.into(),
                                max_height: None,
                                is_open: is_open.into(),
                            });
                            let (attrs, styles) = props.into_parts();

                            view! {
                                <button {..target.attr()}>"Target"</button>
                                <Portal>
                                    <div {..attrs} style=styles>"Positioned overlay"</div>
                                </Portal>
                            }
                        "#)}
                    </Code>
                </Section>

                <Section title="Positioning Demo">
                    <p>
                        "All three hooks combined: "<Code inline=true>"use_overlay"</Code>" for dismissing, "
                        <Code inline=true>"use_overlay_trigger"</Code>" for the trigger\u{2019}s ARIA attributes and "
                        <Code inline=true>"use_overlay_position"</Code>" for the placement. Choose a placement and open the overlay. "
                        "Scroll the page to see it flip when there is not enough space."
                    </p>

                    <Demo
                        description="Overlay positioned next to a trigger, with selectable horizontal and vertical placement"
                        source=include_str!("demos/overlay_positioning.rs")
                    >
                        <PositioningDemo/>
                    </Demo>
                </Section>
            </Section>

            <Section title="use_close_on_scroll">
                <p>
                    "Closes an overlay when a scrollable ancestor of its trigger scrolls, so a positioned overlay doesn\u{2019}t "
                    "stay behind when its trigger scrolls away. Scrolling inside inputs and text areas is ignored. The "
                    <Link href=routes::doc::popover::Atom.materialize()>"Popover atom"</Link>" uses it."
                </p>

                <Section title="Input" id="use-close-on-scroll-input">
                    <ApiTable kind=ApiKind::Input of="UseCloseOnScrollInput">
                        <ApiRow name="is_open" ty="Signal<bool>">"Whether the overlay is open. Listens only while open."</ApiRow>
                        <ApiRow name="trigger_element" ty="CapturedElement">"The trigger whose scroll ancestors are watched."</ApiRow>
                        <ApiRow name="on_close" ty="Callback<()>">"Called when an ancestor of the trigger scrolls."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Accessibility">
                <ul>
                    <li>
                        <Code inline=true>"use_overlay_trigger"</Code>" connects the trigger to the overlay with "
                        <Code inline=true>"aria-expanded"</Code>" and "<Code inline=true>"aria-controls"</Code>"."
                    </li>
                    <li>
                        "The overlay id is generated by "<Code inline=true>"use_overlay"</Code>" and passed to the trigger hook as "
                        <Code inline=true>"overlay_id"</Code>"."
                    </li>
                    <li>"Escape closes the topmost overlay, but not during IME composition."</li>
                    <li>
                        "The hooks don\u{2019}t trap focus. Combine them with the "
                        <Link href=routes::doc::focus::FocusScope.materialize()>"FocusScope"</Link>
                        " atom when focus must stay inside the overlay, and add a "
                        <Link href=routes::doc::overlays::DismissButton.materialize()>"DismissButton"</Link>
                        " for screen reader users who can\u{2019}t press Escape."
                    </li>
                </ul>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Overlays.materialize()>"Overlays overview"</Link></li>
                <li><Link href=routes::doc::Popover.materialize()>"Popover overview"</Link></li>
                <li><Link href=routes::doc::Modal.materialize()>"Modal overview"</Link></li>
                <li><Link href=routes::doc::overlays::DismissButton.materialize()>"DismissButton"</Link></li>
                <li><Link href=routes::doc::focus::FocusScope.materialize()>"FocusScope"</Link></li>
                <li><Link href=routes::doc::interactions::UseInteractOutside.materialize()>"use_interact_outside"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
