//! `use_move` on a focusable `<div>` (`#perf-move`), for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_move.rs`): after a plain button (`#perf-before`) to
//! tab from, mounted at first, counting move starts, moves and move ends, and adding up the
//! moves' horizontal deltas. Its controls ([`Control`]) mount and unmount it, disable and enable
//! it, and read the counts (`starts`, `moves`, `ends`) and the sum (`delta-x`).

use leptonic::{
    IntoAttrs,
    hooks::interactions::{MoveEvent, UseMoveInput, use_move},
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// The move handling's input and counters.
#[derive(Clone, Copy)]
struct Inputs {
    disabled: RwSignal<bool>,
    start_count: StoredValue<u32>,
    move_count: StoredValue<u32>,
    end_count: StoredValue<u32>,
    delta_x: StoredValue<f64>,
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let inputs = Inputs {
        disabled: RwSignal::new(false),
        start_count: StoredValue::new(0),
        move_count: StoredValue::new(0),
        end_count: StoredValue::new(0),
        delta_x: StoredValue::new(0.0),
    };
    let Inputs {
        disabled,
        start_count,
        move_count,
        end_count,
        delta_x,
    } = inputs;
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("disable", move || disabled.set(true)),
        Control::action("enable", move || disabled.set(false)),
        Control::read("starts", move || f64::from(start_count.get_value())),
        Control::read("moves", move || f64::from(move_count.get_value())),
        Control::read("ends", move || f64::from(end_count.get_value())),
        Control::read("delta-x", move || delta_x.get_value()),
    ];
    view! {
        <h1>"use_move performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <Movable inputs />
        </Show>
    }
}

/// The element with `use_move`.
#[component]
fn Movable(inputs: Inputs) -> impl IntoView {
    let movable = use_move(UseMoveInput {
        is_disabled: inputs.disabled.into(),
        on_move_start: Some(Callback::new(move |_| {
            inputs.start_count.update_value(|count| *count += 1);
        })),
        on_move: Some(Callback::new(move |e: MoveEvent| {
            inputs.move_count.update_value(|count| *count += 1);
            inputs.delta_x.update_value(|sum| *sum += e.delta_x);
        })),
        on_move_end: Some(Callback::new(move |_| {
            inputs.end_count.update_value(|count| *count += 1);
        })),
    });
    view! {
        <div
            id="perf-move"
            tabindex="0"
            style="width: 200px; height: 100px"
            {..movable.props.into_attrs()}
        >
            "Move me"
        </div>
    }
}
