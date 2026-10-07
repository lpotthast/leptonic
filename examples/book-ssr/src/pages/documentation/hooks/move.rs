use indoc::indoc;
use leptos::prelude::*;

use super::demos::{
    move_axis::AxisExample, move_basic::BasicMovementExample,
    move_constrain_center::ConstrainCenterExample, move_constrained::ConstrainedBasicExample,
    move_container_click::ContainerClickExample, move_programmatic::ProgrammaticExample,
};
use crate::{kit::*, routes};

#[component]
pub fn PageUseMove() -> impl IntoView {
    view! {
        <DocPage title="use_move">
            <p>
                "The "<Code inline=true>"use_move"</Code>" hook reports pointer drags and arrow key presses on an element as "
                "movement deltas. Optionally, it keeps the element inside a container and tracks its position for you. "
                "See the "<Link href=routes::doc::Interactions.materialize()>"Interactions overview"</Link>" to compare it with the other interaction building blocks."
            </p>

            <ReactAria hook="useMove"/>

            <Section title="Input">
                <p>
                    <Code inline=true>"UseMoveInput"</Code>" implements "<Code inline=true>"Default"</Code>
                    " (free movement in both axes, no callbacks), so you name only the fields you need."
                </p>

                <ApiTable kind=ApiKind::Input of="UseMoveInput">
                    <ApiRow name="is_disabled" ty="Signal<bool>" default="false">
                        "Ignore pointer and keyboard movement while "<Code inline=true>"true"</Code>"."
                    </ApiRow>
                    <ApiRow name="axis" ty="Signal<MoveAxis>" default="MoveAxis::Both">
                        "Lock movement to "<Code inline=true>"MoveAxis::Horizontal"</Code>" or "<Code inline=true>"MoveAxis::Vertical"</Code>
                        ". "<Code inline=true>"MoveAxis::Both"</Code>" allows both directions."
                    </ApiRow>
                    <ApiRow name="on_move_start" ty="Option<Callback<MoveStartEvent>>" default="None">
                        "Called when movement starts. The event has "<Code inline=true>"pointer_type"</Code>", "
                        <Code inline=true>"modifiers"</Code>", "<Code inline=true>"page_x"</Code>" and "<Code inline=true>"page_y"</Code>"."
                    </ApiRow>
                    <ApiRow name="on_move" ty="Option<Callback<MoveEvent>>" default="None">
                        "Called for every movement step with "<Code inline=true>"delta_x"</Code>", "<Code inline=true>"delta_y"</Code>", "
                        <Code inline=true>"pointer_type"</Code>" and "<Code inline=true>"modifiers"</Code>"."
                    </ApiRow>
                    <ApiRow name="on_move_end" ty="Option<Callback<MoveEndEvent>>" default="None">
                        "Called when movement ends, with "<Code inline=true>"pointer_type"</Code>" and "<Code inline=true>"modifiers"</Code>"."
                    </ApiRow>
                </ApiTable>
            </Section>

            <Section title="Return">
                <ApiTable kind=ApiKind::Return of="UseMoveReturn">
                    <ApiRow name="props" ty="UseMoveProps">
                        "Pointer and key handlers for the movable element. Spread them with "
                        <Code inline=true>"{..props.into_attrs()}"</Code>". The element needs a "<Code inline=true>"tabindex"</Code>
                        " to receive arrow keys."
                    </ApiRow>
                    <ApiRow name="is_moving" ty="Signal<bool>">"Whether the element is currently being moved."</ApiRow>
                </ApiTable>
            </Section>

            <Section title="Example">
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::{hooks::*, utils::data_attributes::flag};
                        use leptos::prelude::*;

                        let offset = RwSignal::new((0.0, 0.0));

                        let UseMoveReturn { props, is_moving, .. } = use_move(UseMoveInput {
                            on_move: Some(Callback::new(move |e: MoveEvent| {
                                offset.update(|(x, y)| {
                                    *x += e.delta_x;
                                    *y += e.delta_y;
                                });
                            })),
                            ..UseMoveInput::default()
                        });

                        view! {
                            <div {..props.into_attrs()} tabindex="0" data-moving=flag(is_moving)>"Drag me"</div>
                        }
                    "#)}
                </Code>
            </Section>

            <Section title="Demo">
                <p>
                    "The hook only reports deltas and you decide what to do with them. This demo adds "
                    "them to the element\u{2019}s offset and keeps it inside the area itself."
                </p>

                <Demo description="Unconstrained drag and arrow key movement with an event log" source=include_str!("demos/move_basic.rs")>
                    <BasicMovementExample/>
                </Demo>
            </Section>

            <Section title="use_constrained_move">
                <p>
                    <Code inline=true>"use_constrained_move(input, options)"</Code>" takes the same "
                    <Code inline=true>"UseMoveInput"</Code>" plus "<AnchorLink href="#moveconstraintoptions">"MoveConstraintOptions"</AnchorLink>
                    ", keeps the element inside a container and tracks its position. Spread the returned "
                    <Code inline=true>"container_props"</Code>" onto the container (it needs "<Code inline=true>"position: relative"</Code>
                    ") and position the element absolutely at "<Code inline=true>"pixel_position"</Code>"."
                </p>
                <p>
                    "In a right-to-left layout (the locale of an enclosing "<Code inline=true>"I18nProvider"</Code>
                    "), the horizontal axis of "<Code inline=true>"normalized_position"</Code>" is reversed: "
                    <Code inline=true>"x = 0.0"</Code>" is the right edge."
                </p>

                <Section title="MoveConstraintOptions">
                    <p>
                        <Code inline=true>"MoveConstraintOptions::new(mode)"</Code>" starts at the top left corner, without "
                        "container clicks; change the rest with struct update syntax."
                    </p>

                    <ApiTable kind=ApiKind::Fields of="MoveConstraintOptions">
                        <ApiRow name="mode" ty="MoveConstraint">
                            "Required. "<Code inline=true>"MoveConstraint::Bounds"</Code>" keeps the whole element inside, "
                            <Code inline=true>"MoveConstraint::Center"</Code>" only its center."
                        </ApiRow>
                        <ApiRow name="allow_container_click" ty="bool" default="false">
                            "Move the element to where the container is pressed."
                        </ApiRow>
                        <ApiRow name="initial_position" ty="NormalizedPosition" default="(0.0, 0.0)">
                            "Where the element starts, normalized to the container."
                        </ApiRow>
                        <ApiRow name="on_position_change" ty="Option<Callback<NormalizedPosition>>" default="None">
                            "Called whenever the constrained position changes: by dragging, container clicks, the keyboard or "
                            <Code inline=true>"set_position"</Code>"."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="UseConstrainedMoveReturn">
                    <ApiTable kind=ApiKind::Return of="UseConstrainedMoveReturn">
                        <ApiRow name="props" ty="UseMoveProps">
                            "Pointer and key handlers for the movable element, as with "<Code inline=true>"use_move"</Code>"."
                        </ApiRow>
                        <ApiRow name="is_moving" ty="Signal<bool>">"Whether the element is currently being moved."</ApiRow>
                        <ApiRow name="container_props" ty="UseMoveContainerProps">
                            "Props for the container. Spread them with "<Code inline=true>"{..container_props.into_attrs()}"</Code>"."
                        </ApiRow>
                        <ApiRow name="normalized_position" ty="Signal<NormalizedPosition>">
                            "Position with "<Code inline=true>"x"</Code>" and "<Code inline=true>"y"</Code>" between "
                            <Code inline=true>"0.0"</Code>" (top left) and "<Code inline=true>"1.0"</Code>" (bottom right)."
                        </ApiRow>
                        <ApiRow name="pixel_position" ty="Signal<Point>">"Position in pixels relative to the container."</ApiRow>
                        <ApiRow name="set_position" ty="Callback<NormalizedPosition>">"Moves the element programmatically."</ApiRow>
                    </ApiTable>

                    <Demo description="Movement constrained to a container with position readout" source=include_str!("demos/move_constrained.rs")>
                        <ConstrainedBasicExample/>
                    </Demo>
                </Section>

                <Section title="Axis">
                    <p>"Lock movement to one axis with "<Code inline=true>"axis"</Code>"."</p>

                    <Demo description="Horizontal-only and vertical-only movement" source=include_str!("demos/move_axis.rs")>
                        <AxisExample/>
                    </Demo>
                </Section>

                <Section title="Container Click">
                    <p>"With "<Code inline=true>"allow_container_click"</Code>", pressing the container moves the element there."</p>

                    <Demo description="Clicking the container moves the element to the click position" source=include_str!("demos/move_container_click.rs")>
                        <ContainerClickExample/>
                    </Demo>
                </Section>

                <Section title="Bounds or Center">
                    <p>
                        <Code inline=true>"MoveConstraint::Bounds"</Code>" keeps the whole element inside the container. "
                        <Code inline=true>"MoveConstraint::Center"</Code>" only keeps its center inside, so the element can "
                        "reach over the edge, like a slider thumb."
                    </p>

                    <Demo description="MoveConstraint::Bounds compared with MoveConstraint::Center" source=include_str!("demos/move_constrain_center.rs")>
                        <ConstrainCenterExample/>
                    </Demo>
                </Section>

                <Section title="Programmatic Position">
                    <p>"Call "<Code inline=true>"set_position"</Code>" to move the element from code."</p>

                    <Demo description="Buttons moving the element with set_position" source=include_str!("demos/move_programmatic.rs")>
                        <ProgrammaticExample/>
                    </Demo>
                </Section>
            </Section>

            <Section title="Keyboard">
                <KeyboardTable>
                    <KeyRow keys="ArrowLeft / ArrowRight">
                        "Move by one pixel horizontally (ignored when "<Code inline=true>"axis"</Code>" is vertical)."
                    </KeyRow>
                    <KeyRow keys="ArrowUp / ArrowDown">
                        "Move by one pixel vertically (ignored when "<Code inline=true>"axis"</Code>" is horizontal)."
                    </KeyRow>
                </KeyboardTable>

                <p>
                    "Each key press fires a complete "<Code inline=true>"on_move_start"</Code>", "<Code inline=true>"on_move"</Code>", "
                    <Code inline=true>"on_move_end"</Code>" sequence with the pointer type "<Code inline=true>"Keyboard"</Code>"."
                </p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Interactions.materialize()>"Interactions overview"</Link></li>
                <li><Link href=routes::doc::interactions::UsePress.materialize()>"use_press"</Link></li>
                <li><Link href=routes::doc::DragAndDrop.materialize()>"Drag & Drop"</Link></li>
                <li><Link href=routes::doc::slider::Hook.materialize()>"Slider Hooks"</Link>" (built on use_move)"</li>
                <li><Link href=routes::doc::color_area::Hook.materialize()>"Color Area Hooks"</Link>" (built on use_move)"</li>
            </SeeAlso>
        </DocPage>
    }
}
