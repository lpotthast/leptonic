use indoc::indoc;
use leptonic::components::prelude::*;
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
                "See the "<Link href=routes::doc::Interactions.materialize()>"Interactions overview"</Link>" for domain guidance."
            </p>

            <ReactAria hook="useMove"/>

            <Section title="Input">
                <p>
                    <Code inline=true>"UseMoveInput"</Code>" has no "<Code inline=true>"Default"</Code>
                    " implementation, so you set every field."
                </p>

                <ApiTable kind=ApiKind::Input of="UseMoveInput">
                    <ApiRow name="is_disabled" ty="Signal<bool>">"Ignore pointer and keyboard movement while "<Code inline=true>"true"</Code>"."</ApiRow>
                    <ApiRow name="axis" ty="Signal<Option<MoveAxis>>">
                        "Lock movement to "<Code inline=true>"MoveAxis::Horizontal"</Code>" or "<Code inline=true>"MoveAxis::Vertical"</Code>
                        ". "<Code inline=true>"None"</Code>" and "<Code inline=true>"MoveAxis::Both"</Code>" allow both directions."
                    </ApiRow>
                    <ApiRow name="on_move_start" ty="Option<Callback<MoveStartEvent>>">
                        "Called when movement starts. The event has "<Code inline=true>"pointer_type"</Code>", "
                        <Code inline=true>"modifiers"</Code>", "<Code inline=true>"page_x"</Code>" and "<Code inline=true>"page_y"</Code>"."
                    </ApiRow>
                    <ApiRow name="on_move" ty="Option<Callback<MoveEvent>>">
                        "Called for every movement step with "<Code inline=true>"delta_x"</Code>", "<Code inline=true>"delta_y"</Code>", "
                        <Code inline=true>"pointer_type"</Code>" and "<Code inline=true>"modifiers"</Code>"."
                    </ApiRow>
                    <ApiRow name="on_move_end" ty="Option<Callback<MoveEndEvent>>">
                        "Called when movement ends, with "<Code inline=true>"pointer_type"</Code>" and "<Code inline=true>"modifiers"</Code>"."
                    </ApiRow>
                    <ApiRow name="constraint" ty="Option<MoveConstraint>">
                        "Keep the element inside a container: "<Code inline=true>"MoveConstraint::Bounds"</Code>
                        " keeps the whole element inside, "<Code inline=true>"MoveConstraint::Center"</Code>" only its center. "
                        "Enables the "<Code inline=true>"constraint"</Code>" return value."
                    </ApiRow>
                    <ApiRow name="on_position_change" ty="Option<Callback<NormalizedPosition>>">
                        "Called whenever the constrained position changes: by dragging, container clicks, the keyboard or "
                        <Code inline=true>"set_position"</Code>". Constrained mode only."
                    </ApiRow>
                    <ApiRow name="allow_container_click" ty="bool">
                        "Move the element to where the container is clicked. Constrained mode only."
                    </ApiRow>
                    <ApiRow name="initial_position" ty="Option<NormalizedPosition>">
                        "Starting position in constrained mode. Defaults to the top left corner."
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
                    <ApiRow name="constraint" ty="Option<UseMoveConstraintReturn>">
                        <Code inline=true>"Some"</Code>" when "<Code inline=true>"constraint"</Code>" is set, see "
                        <a href="#constrained-movement">"Constrained movement"</a>"."
                    </ApiRow>
                </ApiTable>
            </Section>

            <Section title="Example">
                <Code language=Language::Rust>
                    {indoc!(r#"
                        let UseMoveReturn { props, is_moving, .. } = use_move(UseMoveInput {
                            is_disabled: false.into(),
                            axis: None.into(),
                            on_move_start: None,
                            on_move: Some(Callback::new(move |e: MoveEvent| {
                                set_x.update(|x| *x += e.delta_x);
                                set_y.update(|y| *y += e.delta_y);
                            })),
                            on_move_end: None,
                            on_position_change: None,
                            constraint: None,
                            allow_container_click: false,
                            initial_position: None,
                        });

                        view! {
                            <div {..props.into_attrs()} tabindex="0">"Drag me"</div>
                        }
                    "#)}
                </Code>
            </Section>

            <Section title="Demo">
                <p>
                    "Without a constraint, the hook only reports deltas and you decide what to do with them. This demo adds "
                    "them to the element\u{2019}s offset and keeps it inside the area itself."
                </p>

                <Demo description="Unconstrained drag and arrow key movement with an event log" source=include_str!("demos/move_basic.rs")>
                    <BasicMovementExample/>
                </Demo>
            </Section>

            <Section title="Constrained movement">
                <p>
                    "With a "<Code inline=true>"constraint"</Code>", the hook also tracks the element\u{2019}s position inside "
                    "a container. Spread "<Code inline=true>"container_props"</Code>" onto the container (it needs "
                    <Code inline=true>"position: relative"</Code>") and position the element absolutely at "
                    <Code inline=true>"pixel_position"</Code>"."
                </p>
                <p>
                    "In a right-to-left layout (the locale of an enclosing "<Code inline=true>"I18nProvider"</Code>
                    "), the horizontal axis of "<Code inline=true>"normalized_position"</Code>" is reversed: "
                    <Code inline=true>"x = 0.0"</Code>" is the right edge."
                </p>

                <ApiTable kind=ApiKind::Return of="UseMoveConstraintReturn">
                    <ApiRow name="container_props" ty="UseMoveContainerProps">
                        "Props for the container. Spread them with "<Code inline=true>"{..container_props.into_attrs()}"</Code>"."
                    </ApiRow>
                    <ApiRow name="normalized_position" ty="Signal<NormalizedPosition>">
                        "Position with "<Code inline=true>"x"</Code>" and "<Code inline=true>"y"</Code>" between "
                        <Code inline=true>"0.0"</Code>" (top left) and "<Code inline=true>"1.0"</Code>" (bottom right)."
                    </ApiRow>
                    <ApiRow name="pixel_position" ty="Signal<(f64, f64)>">"Position in pixels relative to the container."</ApiRow>
                    <ApiRow name="set_position" ty="Callback<NormalizedPosition>">"Moves the element programmatically."</ApiRow>
                </ApiTable>

                <Demo description="Movement constrained to a container with position readout" source=include_str!("demos/move_constrained.rs")>
                    <ConstrainedBasicExample/>
                </Demo>

                <Section title="Axis">
                    <p>"Lock movement to one axis with "<Code inline=true>"axis"</Code>"."</p>

                    <Demo description="Horizontal-only and vertical-only movement" source=include_str!("demos/move_axis.rs")>
                        <AxisExample/>
                    </Demo>
                </Section>

                <Section title="Container click">
                    <p>"With "<Code inline=true>"allow_container_click"</Code>", clicking the container moves the element there."</p>

                    <Demo description="Clicking the container moves the element to the click position" source=include_str!("demos/move_container_click.rs")>
                        <ContainerClickExample/>
                    </Demo>
                </Section>

                <Section title="Bounds or center">
                    <p>
                        <Code inline=true>"MoveConstraint::Bounds"</Code>" keeps the whole element inside the container. "
                        <Code inline=true>"MoveConstraint::Center"</Code>" only keeps its center inside, so the element can "
                        "reach over the edge, like a slider thumb."
                    </p>

                    <Demo description="MoveConstraint::Bounds compared with MoveConstraint::Center" source=include_str!("demos/move_constrain_center.rs")>
                        <ConstrainCenterExample/>
                    </Demo>
                </Section>

                <Section title="Programmatic position">
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
                <li><Link href=routes::doc::interactions::Dnd.materialize()>"Drag and drop"</Link></li>
                <li><Link href=routes::doc::slider::Hook.materialize()>"use_slider"</Link>" (uses use_move)"</li>
            </SeeAlso>
        </DocPage>
    }
}
