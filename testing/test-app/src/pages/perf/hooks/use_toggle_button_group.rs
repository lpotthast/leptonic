//! `use_toggle_button_group` (single selection: a `radiogroup`) on a plain `<div>`
//! (`#perf-group`) of three buttons (`#perf-a`, `#perf-b`, `#perf-c`, each with
//! `use_toggle_button_group_item` and `use_button`), for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_toggle_button_group.rs`): between two plain buttons
//! (`#perf-before`, `#perf-after`) to tab from and to, mounted at first. Its controls
//! ([`Control`]) mount and unmount it, disable and enable it, and change its orientation.

use leptonic::{
    CapturedElement, IntoAttrs, Orientation,
    hooks::{
        button::{
            ToggleGroupState, UseButtonInput, UseToggleButtonGroupInput,
            UseToggleButtonGroupItemInput, UseToggleGroupStateInput, use_button,
            use_toggle_button_group, use_toggle_button_group_item, use_toggle_group_state,
        },
        collections::Key,
        toolbar::UseToolbarInput,
    },
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let disabled = RwSignal::new(false);
    let orientation = RwSignal::new(Orientation::Horizontal);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("disable", move || disabled.set(true)),
        Control::action("enable", move || disabled.set(false)),
        Control::action("vertical", move || orientation.set(Orientation::Vertical)),
        Control::action("horizontal", move || {
            orientation.set(Orientation::Horizontal);
        }),
    ];
    view! {
        <h1>"use_toggle_button_group performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <Group disabled orientation />
        </Show>
        <button id="perf-after">"After"</button>
    }
}

/// The group with `use_toggle_button_group`.
#[component]
fn Group(disabled: RwSignal<bool>, orientation: RwSignal<Orientation>) -> impl IntoView {
    let state = use_toggle_group_state(UseToggleGroupStateInput {
        is_disabled: disabled.into(),
        ..UseToggleGroupStateInput::default()
    });
    let group = use_toggle_button_group(UseToggleButtonGroupInput {
        state,
        toolbar: UseToolbarInput {
            element: CapturedElement::new(),
            orientation: orientation.into(),
            aria_label: MaybeProp::from("Alignment"),
            aria_labelledby: Signal::default(),
        },
    });
    view! {
        <div id="perf-group" {..group.props.into_attrs()}>
            <Item state key="a" />
            <Item state key="b" />
            <Item state key="c" />
        </div>
    }
}

/// A button of the group.
#[component]
fn Item(state: ToggleGroupState, key: &'static str) -> impl IntoView {
    let button = use_button(use_toggle_button_group_item(
        UseToggleButtonGroupItemInput {
            group: state,
            key: Key::from(key),
            button: UseButtonInput {
                id: Some(format!("perf-{key}")),
                ..UseButtonInput::default()
            },
        },
    ));
    let (attrs, styles) = button.props.into_parts();
    view! {
        <button {..attrs} style=styles>
            {key}
        </button>
    }
}
