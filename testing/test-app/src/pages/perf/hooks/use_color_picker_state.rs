//! `use_color_picker_state` alone (`#00FFFF` at first), for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_color_picker_state.rs`): its color shown in
//! `#perf-color` (`RRGGBB`), mounted at first; owning its color, or (control `use-binding` while
//! unmounted) bound to the page's color (a `ValueBinding`). Its controls ([`Control`]) mount and
//! unmount it, set its color (`set_color`), change the page's bound color, and read how often it
//! called `on_change` (`changes`).

use leptonic::{
    Color, RGB8, ValueBinding,
    hooks::color::{ColorPickerState, UseColorPickerStateInput, use_color_picker_state},
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

const CYAN: RGB8 = RGB8 {
    r: 0,
    g: 255,
    b: 255,
};
const RED: RGB8 = RGB8 { r: 255, g: 0, b: 0 };

/// The state, its color shown. Hands the state to the page's controls.
#[component]
fn State(
    state: StoredValue<Option<ColorPickerState>>,
    bound: Option<RwSignal<Color>>,
    change_count: StoredValue<u32>,
) -> impl IntoView {
    let picker = use_color_picker_state(UseColorPickerStateInput {
        default_value: Color::from(CYAN),
        value: bound.map(ValueBinding::from),
        on_change: Some(Callback::new(move |_: Color| {
            change_count.update_value(|count| *count += 1);
        })),
    });
    state.set_value(Some(picker));
    on_cleanup(move || state.set_value(None));
    view! {
        <p>
            "Color: "
            <span id="perf-color">{move || format!("{:X}", picker.color.get().to::<RGB8>())}</span>
        </p>
    }
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let use_binding = RwSignal::new(false);
    let color = RwSignal::new(Color::from(CYAN));
    let state = StoredValue::new(None::<ColorPickerState>);
    let change_count = StoredValue::new(0_u32);
    let set_color = move |color: RGB8| {
        if let Some(state) = state.get_value() {
            state.set_color(Color::from(color));
        }
    };
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("use-binding", move || use_binding.set(true)),
        Control::action("set-red", move || set_color(RED)),
        Control::action("set-cyan", move || set_color(CYAN)),
        Control::action("bind-red", move || color.set(Color::from(RED))),
        Control::action("bind-cyan", move || color.set(Color::from(CYAN))),
        Control::read("changes", move || f64::from(change_count.get_value())),
    ];
    view! {
        <h1>"use_color_picker_state performance"</h1>
        <PerfControls controls />
        <Show when=move || mounted.get()>
            <State
                state
                bound=use_binding.get_untracked().then_some(color)
                change_count
            />
        </Show>
    }
}
