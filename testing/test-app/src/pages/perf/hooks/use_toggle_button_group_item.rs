//! `use_toggle_button_group_item` on three plain buttons (`#perf-a`, `#perf-b`, `#perf-c`, with
//! `use_button`) of a `use_toggle_button_group` (single selection: radios) on a `<div>`
//! (`#perf-group`), for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_toggle_button_group_item.rs`): between two plain buttons
//! (`#perf-before`, `#perf-after`) to tab from and to, mounted at first. Its controls
//! ([`Control`]) mount and unmount the group, and disable and enable the button `b`.

use leptonic::{
    CapturedElement, Orientation,
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
    let b_disabled = RwSignal::new(false);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("disable-b", move || b_disabled.set(true)),
        Control::action("enable-b", move || b_disabled.set(false)),
    ];
    view! {
        <h1>"use_toggle_button_group_item performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <Group b_disabled />
        </Show>
        <button id="perf-after">"After"</button>
    }
}

/// The group with `use_toggle_button_group`.
#[component]
fn Group(b_disabled: RwSignal<bool>) -> impl IntoView {
    let state = use_toggle_group_state(UseToggleGroupStateInput::default());
    let group = use_toggle_button_group(UseToggleButtonGroupInput {
        state,
        toolbar: UseToolbarInput {
            element: CapturedElement::new(),
            orientation: Signal::stored(Orientation::Horizontal),
            aria_label: MaybeProp::from("Alignment"),
            aria_labelledby: Signal::default(),
        },
    });
    view! {
        <div id="perf-group" {..group.group.attrs()}>
            <Item state key="a" disabled=Signal::stored(false) />
            <Item state key="b" disabled=b_disabled.into() />
            <Item state key="c" disabled=Signal::stored(false) />
        </div>
    }
}

/// A button of the group with `use_toggle_button_group_item`.
#[component]
fn Item(state: ToggleGroupState, key: &'static str, disabled: Signal<bool>) -> impl IntoView {
    let button = use_button(use_toggle_button_group_item(
        UseToggleButtonGroupItemInput {
            group: state,
            key: Key::from(key),
            button: UseButtonInput {
                id: Some(format!("perf-{key}").into()),
                is_disabled: disabled,
                ..UseButtonInput::default()
            },
        },
    ));
    let (attrs, styles) = button.button.attrs();
    view! {
        <button {..attrs} style=styles>
            {key}
        </button>
    }
}
