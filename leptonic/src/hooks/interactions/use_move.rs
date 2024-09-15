use leptos_reactive::{
    create_effect, on_cleanup, store_value, Callable, Callback, Signal, SignalDispose, SignalGet,
};
use typed_builder::TypedBuilder;
use web_sys::PointerEvent;

use crate::utils::{attributes::Attributes, event_handlers::EventHandlers};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/%40react-aria/interactions/src/useMove.ts

#[derive(Debug, Clone, Copy)]
pub struct MoveStartEvent {}

#[derive(Debug, Clone, Copy)]
pub struct MoveEvent {
    pub delta_x: f64,
    pub delta_y: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct MoveEndEvent {}

#[derive(Debug, Clone, Copy, TypedBuilder)]
pub struct UseMoveInput {
    #[builder(setter(into))]
    pub(crate) on_move_start: Callback<MoveStartEvent>,
    #[builder(setter(into))]
    pub(crate) on_move: Callback<MoveEvent>,
    #[builder(setter(into))]
    pub(crate) on_move_end: Callback<MoveEndEvent>,

    // TODO: Remove this
    #[builder(setter(into))]
    pub(crate) global_pointer_up: Signal<Option<PointerEvent>>,

    // TODO: Remove this
    #[builder(setter(into))]
    pub(crate) global_pointer_move: Signal<Option<PointerEvent>>,
}

#[derive(Debug)]
pub struct UseMoveProps {
    /// These attributes must be spread onto the target element: `<foo {..props.attrs} />`
    pub attrs: Attributes,
    /// These handlers must be spread onto the target element: `<foo {..props.handlers} />`
    pub handlers: EventHandlers,
}

#[derive(Debug)]
pub struct UseMoveReturn {
    pub props: UseMoveProps,
}

#[derive(Debug, Clone, Copy)]
struct MoveState {
    pointer_id: i32,
    moved: bool,
    last_pos: (f64, f64),
}

pub fn use_move(input: UseMoveInput) -> UseMoveReturn {
    // Note: There may be multiple pointers. Every pointer event contains a unique identifier of the pointer used for the interaction.
    // We start movement tracking by listening for on_pointer_down events.
    // Only movements from the pointer which initiated the tracking is propagated.

    let state = store_value(Option::<MoveState>::None);
    let active = store_value(false);

    let on_pointer_down = Box::new(move |e: PointerEvent| {
        let pointer_id = e.pointer_id();

        if e.button() == 0 && state.get_value().is_none() {
            e.stop_propagation();
            e.prevent_default();

            state.set_value(Some(MoveState {
                pointer_id,
                moved: false,
                last_pos: (e.page_x() as f64, e.page_y() as f64),
            }));
            active.set_value(true);
        }
    });

    let on_move = create_effect(move |_| {
        let e = input.global_pointer_move.get();
        if active.get_value() {
            if let Some(e) = e {
                let pointer_id = e.pointer_id();

                if state
                    .get_value()
                    .is_some_and(|s| s.pointer_id == pointer_id)
                {
                    let first_move = state.with_value(|s| !s.expect("present").moved);
                    let (old_x, old_y) = state.with_value(|s| s.expect("present").last_pos);
                    let (new_x, new_y) = (e.page_x() as f64, e.page_y() as f64); // TODO: This should already provide a f64!

                    state.update_value(move |s: &mut Option<MoveState>| {
                        let s = s.as_mut().expect("present");
                        s.moved = true;
                        s.last_pos = (new_x, new_y);
                    });

                    if first_move {
                        Callable::call(&input.on_move_start, MoveStartEvent {});
                    }
                    Callable::call(
                        &input.on_move,
                        MoveEvent {
                            delta_x: new_x - old_x,
                            delta_y: new_y - old_y,
                        },
                    );
                }
            }
        }
    });

    let on_cancel = create_effect(move |_| {
        let e = input.global_pointer_up.get();
        if active.get_value() {
            if let Some(e) = e {
                let pointer_id = e.pointer_id();

                if state
                    .get_value()
                    .is_some_and(|s| s.pointer_id == pointer_id)
                {
                    let moved = state.with_value(|s| s.expect("present").moved);

                    if moved {
                        Callable::call(&input.on_move_end, MoveEndEvent {});
                    }

                    state.set_value(None);
                    active.set_value(false);
                }
            }
        }
    });

    let on_up = create_effect(move |_| {
        let e = input.global_pointer_up.get();
        if active.get_value() {
            if let Some(e) = e {
                let pointer_id = e.pointer_id();

                if state
                    .get_value()
                    .is_some_and(|s| s.pointer_id == pointer_id)
                {
                    let moved = state.with_value(|s| s.expect("present").moved);

                    if moved {
                        Callable::call(&input.on_move_end, MoveEndEvent {});
                    }

                    state.set_value(None);
                    active.set_value(false);
                }
            }
        }
    });

    on_cleanup(move || {
        on_move.dispose();
        on_up.dispose();
        on_cancel.dispose();
    });

    UseMoveReturn {
        props: UseMoveProps {
            attrs: Attributes::new(),
            handlers: EventHandlers::builder()
                .on_pointer_down(on_pointer_down)
                .build(),
        },
    }
}
