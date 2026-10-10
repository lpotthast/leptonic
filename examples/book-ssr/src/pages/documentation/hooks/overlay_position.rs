use indoc::indoc;
use leptos::prelude::*;

use super::demos::overlay_position::PositioningDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageUseOverlayPosition() -> impl IntoView {
    view! {
        <DocPage title="use_overlay_position">
            <p>
                "The "<Code inline=true>"use_overlay_position"</Code>" hook places an overlay next to a target element and "
                "keeps it there. See the "<Link href=routes::doc::OverlayBehavior.materialize()>"Overlay Behavior overview"</Link>
                " for the other overlay building blocks."
            </p>

            <ReactAriaSource path="overlays/useOverlayPosition.ts"/>

            <Section title="Input">
                <p>
                    "Pass a "<Code inline=true>"UseOverlayPositionInput"</Code>" with every field named; the Default column "
                    "gives the value for fields you don\u{2019}t need."
                </p>

                <ApiTable kind=ApiKind::Input of="UseOverlayPositionInput">
                    <ApiRow name="target" ty="CapturedElement">
                        "The element to position the overlay at. Spread "<Code inline=true>"{..target.attr()}"</Code>
                        " onto it. Required."
                    </ApiRow>
                    <ApiRow name="is_open" ty="Signal<bool>">"Whether the overlay is open; it is positioned only while open. Required."</ApiRow>
                    <ApiRow name="target_rect" ty="Signal<Option<Rect>>" default="None">
                        "Replaces the target\u{2019}s bounding rectangle (viewport coordinates), e.g. with a point for a context menu."
                    </ApiRow>
                    <ApiRow name="scroll" ty="Option<CapturedElement>" default="None">
                        "The scrollable element inside the overlay whose focused content keeps its place when the overlay moves. "
                        <Code inline=true>"None"</Code>": the overlay."
                    </ApiRow>
                    <ApiRow name="on_close" ty="Option<Callback<()>>" default="None">
                        "Called when an ancestor of the target scrolls (not while the visual viewport resizes, as when a "
                        "virtual keyboard opens), to close the overlay; see "
                        <Link href=routes::doc::overlay_behavior::UseCloseOnScroll.materialize()>"use_close_on_scroll"</Link>
                        ". "<Code inline=true>"None"</Code>": the overlay stays open."
                    </ApiRow>
                    <ApiRow name="position" ty="OverlayPositionOptions" default="OverlayPositionOptions::default()">"Placement, offsets, boundary and update behavior for the overlay."</ApiRow>
                </ApiTable>
                <Section title="OverlayPositionOptions">
                    <ApiTable kind=ApiKind::Fields of="OverlayPositionOptions">
                        <ApiRow name="placement" ty="Signal<Placement>" default="Bottom">
                            "The side of the target and the alignment along it, see "<AnchorLink href="#placements">"Placements"</AnchorLink>"."
                        </ApiRow>
                        <ApiRow name="offset" ty="Signal<f64>" default="0.0">"The distance from the target, along the main axis, in pixels."</ApiRow>
                        <ApiRow name="cross_offset" ty="Signal<f64>" default="0.0">"The shift along the target\u{2019}s side, in pixels."</ApiRow>
                        <ApiRow name="container_padding" ty="Signal<f64>" default="12.0">"The minimum distance from the boundary\u{2019}s edges, in pixels."</ApiRow>
                        <ApiRow name="should_flip" ty="Signal<bool>" default="true">"Flip to the other side when that has more room."</ApiRow>
                        <ApiRow name="boundary" ty="Option<CapturedElement>" default="None">
                            "The element the overlay must stay within. "<Code inline=true>"None"</Code>": the document body."
                        </ApiRow>
                        <ApiRow name="max_height" ty="Signal<Option<f64>>" default="None">
                            "A maximum height; the room available limits it further. "<Code inline=true>"None"</Code>": the room available."
                        </ApiRow>
                        <ApiRow name="arrow_size" ty="Signal<Option<f64>>" default="None">
                            "The arrow\u{2019}s size across the main axis. "<Code inline=true>"None"</Code>": the width of the element "
                            "captured by "<Code inline=true>"arrow_props"</Code>" (0 without one)."
                        </ApiRow>
                        <ApiRow name="arrow_boundary_offset" ty="Signal<f64>" default="0.0">
                            "The minimum distance between the arrow and the overlay\u{2019}s edges."
                        </ApiRow>
                        <ApiRow name="should_update_position" ty="Signal<bool>" default="true">
                            "Whether the position follows changes (resizes, a virtual keyboard opening)."
                        </ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Return">
                <ApiTable kind=ApiKind::Return of="UseOverlayPositionReturn">
                    <ApiRow name="props" ty="PropsWithStyles<UseOverlayPositionProps>">
                        "The overlay\u{2019}s element capture and its position styles. "<Code inline=true>"props.into_parts()"</Code>
                        " gives "<Code inline=true>"(attrs, styles)"</Code>"."
                    </ApiRow>
                    <ApiRow name="arrow_props" ty="PropsWithStyles<UseOverlayArrowProps>">
                        "For an arrow inside the overlay, see "<AnchorLink href="#arrow">"Arrow"</AnchorLink>"."
                    </ApiRow>
                    <ApiRow name="placement" ty="Signal<Option<PlacementAxis>>">
                        "The side of the target the overlay is on, after flipping; "<Code inline=true>"None"</Code>" until positioned."
                    </ApiRow>
                    <ApiRow name="trigger_anchor_point" ty="Signal<Option<Point>>">
                        "The point of the overlay closest to the target, in the overlay\u{2019}s own coordinates: a "
                        <Code inline=true>"transform-origin"</Code>" for animations growing out of the target."
                    </ApiRow>
                    <ApiRow name="overlay_element" ty="CapturedElement">"The overlay element, once rendered."</ApiRow>
                    <ApiRow name="updater" ty="PositionUpdater">
                        "Its "<Code inline=true>"update()"</Code>" positions the overlay again at once, e.g. after its content changed "
                        "size in a way no observer sees."
                    </ApiRow>
                </ApiTable>
            </Section>

            <Section title="Example">
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::{
                            CapturedElement,
                            hooks::overlay::{OverlayPositionOptions, Placement, UseOverlayPositionInput, UseOverlayPositionReturn, use_overlay_position},
                        };
                        use leptos::portal::Portal;

                        let target = CapturedElement::new();
                        let (is_open, set_is_open) = signal(false);

                        let UseOverlayPositionReturn { props, .. } = use_overlay_position(UseOverlayPositionInput {
                            target,
                            is_open: is_open.into(),
                            position: OverlayPositionOptions {
                                placement: Signal::stored(Placement::BottomStart),
                                offset: Signal::stored(4.0),
                                ..Default::default()
                            },
                            target_rect: Signal::stored(None),
                            scroll: None,
                            on_close: None,
                        });

                        // Stored, so that the portal can render them again whenever the overlay opens.
                        let (attrs, styles) = props.into_parts();
                        let (attrs, styles) = (StoredValue::new(attrs), StoredValue::new(styles));

                        view! {
                            <span {..target.attr()}>"Target"</span>
                            <Portal>
                                <Show when=move || is_open.get()>
                                    <div {..attrs.get_value()} style=styles.get_value()>"Positioned overlay"</div>
                                </Show>
                            </Portal>
                        }
                    "#)}
                </Code>
                <p>
                    "The overlay is positioned absolutely within its containing block, usually the document, so render it in a "
                    <Code inline=true>"<Portal>"</Code>"."
                </p>
            </Section>

            <Section title="Demo">
                <p>
                    "Choose a placement and open the overlay. Scroll the page until the button nears the top or bottom edge to "
                    "see the overlay flip. "<Link href=routes::doc::overlay_behavior::UseOverlay.materialize()>"use_overlay"</Link>
                    " dismisses it and "<Link href=routes::doc::overlay_behavior::UseOverlayTrigger.materialize()>"use_overlay_trigger"</Link>
                    " connects the button to it."
                </p>

                <Demo
                    description="An overlay positioned next to its trigger, with a selectable side and alignment"
                    source=include_str!("demos/overlay_position.rs")
                >
                    <PositioningDemo/>
                </Demo>
            </Section>

            <Section title="Behavior">
                <p>
                    "The overlay is positioned while it is open, and again when the window, the overlay or the target resizes "
                    "(for the overlay and the target, in the next animation frame) and when a virtual keyboard opens. It flips to the other side when that has more room, stays within the "
                    "boundary and gets a maximum height for the room available."
                </p>
            </Section>

            <Section title="Placements">
                <p>
                    <Code inline=true>"Placement"</Code>" names the side of the target first, then the alignment along it. "
                    <Code inline=true>"Start"</Code>" and "<Code inline=true>"End"</Code>" follow the writing direction of the "
                    "enclosing "<Code inline=true>"I18nProvider"</Code>" (left-to-right without one)."
                </p>
                <DocTable headers=&["Side", "Placements"]>
                    <TableRow>
                        <TableCell>"Above or below"</TableCell>
                        <TableCell>
                            <Code inline=true>"Top"</Code>", "<Code inline=true>"TopStart"</Code>", "<Code inline=true>"TopEnd"</Code>", "
                            <Code inline=true>"TopLeft"</Code>", "<Code inline=true>"TopRight"</Code>" (and the same for "
                            <Code inline=true>"Bottom"</Code>")"
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Left or right"</TableCell>
                        <TableCell>
                            <Code inline=true>"Left"</Code>", "<Code inline=true>"LeftTop"</Code>", "<Code inline=true>"LeftBottom"</Code>
                            " (and the same for "<Code inline=true>"Right"</Code>")"
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Before or after"</TableCell>
                        <TableCell>
                            <Code inline=true>"Start"</Code>", "<Code inline=true>"StartTop"</Code>", "<Code inline=true>"StartBottom"</Code>
                            " (and the same for "<Code inline=true>"End"</Code>")"
                        </TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Arrow">
                <p>
                    "Spread "<Code inline=true>"arrow_props"</Code>" onto an arrow element inside the overlay. It is hidden from "
                    "assistive technology and placed along the edge facing the target, pointing at the target\u{2019}s center as "
                    "far as "<Code inline=true>"arrow_boundary_offset"</Code>" allows. Pin it to that edge yourself (e.g. "
                    <Code inline=true>"top: 100%"</Code>" for an overlay above the target, using the returned "
                    <Code inline=true>"placement"</Code>"), or use the "
                    <Link href=routes::doc::popover::Atom.materialize()>"OverlayArrow"</Link>" atom inside a "
                    <Code inline=true>"Popover"</Code>" or "<Code inline=true>"Tooltip"</Code>"."
                </p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::OverlayBehavior.materialize()>"Overlay Behavior overview"</Link></li>
                <li><Link href=routes::doc::overlay_behavior::UseCloseOnScroll.materialize()>"use_close_on_scroll"</Link></li>
                <li><Link href=routes::doc::popover::Hook.materialize()>"use_popover"</Link></li>
                <li><Link href=routes::doc::tooltip::Atom.materialize()>"Tooltip Atoms"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
