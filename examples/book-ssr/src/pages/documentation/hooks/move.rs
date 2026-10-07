use indoc::indoc;
use leptos::prelude::*;

use super::demos::move_basic::BasicMovementExample;
use crate::{kit::*, routes};

#[component]
pub fn PageUseMove() -> impl IntoView {
    view! {
        <DocPage title="use_move">
            <p>
                "The "<Code inline=true>"use_move"</Code>" hook reports pointer drags and arrow key presses on an element as "
                "movement deltas; what a movement changes (the element\u{2019}s position, a value) is up to you. "
                "See the "<Link href=routes::doc::Interactions.materialize()>"Interactions overview"</Link>" to compare it with the other interaction building blocks."
            </p>

            <ReactAria hook="useMove"/>

            <Section title="Input">
                <p>
                    <Code inline=true>"UseMoveInput"</Code>" implements "<Code inline=true>"Default"</Code>
                    " (enabled, no callbacks), so you name only the fields you need."
                </p>

                <ApiTable kind=ApiKind::Input of="UseMoveInput">
                    <ApiRow name="is_disabled" ty="Signal<bool>" default="false">
                        "Ignore pointer and keyboard movement while "<Code inline=true>"true"</Code>"."
                    </ApiRow>
                    <ApiRow name="on_move_start" ty="Option<Callback<MoveStartEvent>>" default="None">
                        "Called when movement starts (on the first movement, not on the press), with "<Code inline=true>"pointer_type"</Code>
                        " and "<Code inline=true>"modifiers"</Code>"."
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
                </ApiTable>
            </Section>

            <Section title="Example">
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::{hooks::*, utils::data_attributes::flag};
                        use leptos::prelude::*;

                        let offset = RwSignal::new((0.0, 0.0));
                        let is_moving = RwSignal::new(false);

                        let UseMoveReturn { props } = use_move(UseMoveInput {
                            on_move_start: Some(Callback::new(move |_| is_moving.set(true))),
                            on_move: Some(Callback::new(move |e: MoveEvent| {
                                offset.update(|(x, y)| {
                                    *x += e.delta_x;
                                    *y += e.delta_y;
                                });
                            })),
                            on_move_end: Some(Callback::new(move |_| is_moving.set(false))),
                            ..UseMoveInput::default()
                        });

                        view! {
                            <div {..props.into_attrs()} tabindex="0" data-moving=flag(is_moving.into())>"Drag me"</div>
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

            <Section title="Keyboard">
                <KeyboardTable>
                    <KeyRow keys="ArrowLeft / ArrowRight">
                        "Move by one pixel horizontally."
                    </KeyRow>
                    <KeyRow keys="ArrowUp / ArrowDown">
                        "Move by one pixel vertically."
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
