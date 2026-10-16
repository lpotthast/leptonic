//! `use_button` on a plain `<button>` (`#perf-button`), for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_button.rs`): between two plain buttons (`#perf-before`,
//! `#perf-after`) to tab from and to, mounted at first, its pressed and hovered state as
//! `data-pressed` and `data-hovered`. Its controls ([`Control`]) mount and unmount it, change its
//! reactive inputs, and read how often it was pressed (`presses`).

use leptonic::{
    AriaExpanded, AriaPressed, flag,
    hooks::button::{UseButtonInput, use_button},
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// The button's reactive inputs and press counter.
#[derive(Clone, Copy)]
struct Inputs {
    disabled: RwSignal<bool>,
    pending: RwSignal<bool>,
    excluded: RwSignal<bool>,
    aria_pressed: RwSignal<Option<AriaPressed>>,
    expanded: RwSignal<Option<AriaExpanded>>,
    press_count: StoredValue<u32>,
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let inputs = Inputs {
        disabled: RwSignal::new(false),
        pending: RwSignal::new(false),
        excluded: RwSignal::new(false),
        aria_pressed: RwSignal::new(None),
        expanded: RwSignal::new(None),
        press_count: StoredValue::new(0),
    };
    let Inputs {
        disabled,
        pending,
        excluded,
        aria_pressed,
        expanded,
        press_count,
    } = inputs;
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("disable", move || disabled.set(true)),
        Control::action("enable", move || disabled.set(false)),
        Control::action("start-pending", move || pending.set(true)),
        Control::action("end-pending", move || pending.set(false)),
        Control::action("exclude-from-tab-order", move || excluded.set(true)),
        Control::action("include-in-tab-order", move || excluded.set(false)),
        Control::action("set-aria-pressed", move || {
            aria_pressed.set(Some(AriaPressed::True));
        }),
        Control::action("unset-aria-pressed", move || aria_pressed.set(None)),
        Control::action("expand", move || expanded.set(Some(AriaExpanded::True))),
        Control::action("collapse", move || expanded.set(Some(AriaExpanded::False))),
        Control::read("presses", move || f64::from(press_count.get_value())),
    ];
    view! {
        <h1>"use_button performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <PlainButton inputs />
        </Show>
        <button id="perf-after">"After"</button>
    }
}

/// The `<button>` with `use_button`.
#[component]
fn PlainButton(inputs: Inputs) -> impl IntoView {
    let button = use_button(UseButtonInput {
        id: Some("perf-button".to_owned()),
        is_disabled: inputs.disabled.into(),
        is_pending: inputs.pending.into(),
        exclude_from_tab_order: inputs.excluded.into(),
        aria_pressed: inputs.aria_pressed.into(),
        aria_expanded: inputs.expanded.into(),
        on_press: Some(Callback::new(move |_| {
            inputs.press_count.update_value(|count| *count += 1);
        })),
        ..UseButtonInput::default()
    });
    let (attrs, styles) = button.props.into_parts();
    view! {
        <button
            {..attrs}
            style=styles
            data-pressed=flag(button.is_pressed)
            data-hovered=flag(button.is_hovered)
        >
            "Press me"
        </button>
    }
}
