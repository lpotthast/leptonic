//! The `Select` alone (`.leptonic-Select`, labelled "Option"), for its performance test
//! (`leptonic/tests/perf_tests/atoms/select.rs`), with its parts: a `SelectTrigger` showing a
//! `SelectValue`, a `SelectPopover` with a `ListBox` of the options "Option 0", "Option 1", ...
//! (`ListBoxItem`s, `data-perf-option` holding the number; [`SMALL`] at first, [`LARGE`] after the
//! control `large`) and a `HiddenSelect` (the field `option`). Its value and open state are
//! controlled. Between two plain buttons (`#perf-before`, `#perf-after`) to tab from and to,
//! mounted at first.
//!
//! [`SelectFixture`] renders it for the performance tests of its parts too: the controls `mount`
//! and `unmount` mount the part (the rest stays mounted). The other controls ([`Control`]) change
//! the options and the select's reactive props, set its value and open or close it, and read its
//! value (`value`: the selected option's number, `-1` for none).

use std::collections::HashSet;

use leptonic::{
    atoms::{
        listbox::{ListBox, ListBoxItem},
        select::{HiddenSelect, Select, SelectPopover, SelectTrigger, SelectValue},
    },
    hooks::collections::{Key, UseListCollectionInput, use_list_collection},
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// The options at first.
pub const SMALL: u32 = 10;
/// The options after the control `large`.
pub const LARGE: u32 = 1_000;

/// The part of the select that the controls `mount` and `unmount` mount.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Part {
    /// The whole select.
    Select,
    Trigger,
    Value,
    Popover,
    HiddenSelect,
}

#[component]
pub fn PagePerf() -> impl IntoView {
    view! { <SelectFixture title="Select performance" part=Part::Select /> }
}

/// The select (see the module documentation), `part` mounted by the controls.
#[component]
pub fn SelectFixture(title: &'static str, part: Part) -> impl IntoView {
    let mounted = RwSignal::new(true);
    let is_mounted = move |of: Part| move || of != part || mounted.get();
    let items = RwSignal::new((0..SMALL).collect::<Vec<u32>>());
    let collection = use_list_collection(UseListCollectionInput {
        items: items.into(),
        key: |i| Key::from(*i),
        text_value: |i| format!("Option {i}"),
    });
    let value = RwSignal::new(None::<u32>);
    let open = RwSignal::new(false);
    let disabled = RwSignal::new(false);
    let invalid = RwSignal::new(false);
    let required = RwSignal::new(false);
    let disabled_keys = RwSignal::new(HashSet::<Key>::new());
    let placeholder = RwSignal::new("Pick an option".to_owned());
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("large", move || items.set((0..LARGE).collect())),
        Control::action("select-first", move || value.set(Some(0))),
        Control::action("select-last", move || {
            value.set(items.with_untracked(|items| items.last().copied()));
        }),
        Control::action("clear-value", move || value.set(None)),
        Control::action("open", move || open.set(true)),
        Control::action("close", move || open.set(false)),
        Control::action("disable", move || disabled.set(true)),
        Control::action("enable", move || disabled.set(false)),
        Control::action("invalidate", move || invalid.set(true)),
        Control::action("validate", move || invalid.set(false)),
        Control::action("require", move || required.set(true)),
        Control::action("unrequire", move || required.set(false)),
        Control::action("disable-first", move || {
            disabled_keys.set(HashSet::from([Key::from(0_u32)]));
        }),
        Control::action("enable-all", move || disabled_keys.set(HashSet::new())),
        Control::action("rename-placeholder", move || {
            placeholder.set("Choose an option".to_owned());
        }),
        Control::read("value", move || {
            value.get_untracked().map_or(-1.0, f64::from)
        }),
    ];
    view! {
        <h1>{title}</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=is_mounted(Part::Select)>
            <Select
                collection=collection
                value=value
                set_value=value
                is_open=open
                set_open=open
                is_disabled=disabled
                is_invalid=invalid
                is_required=required
                disabled_keys=disabled_keys
                aria_label="Option"
                name="option"
            >
                <Show when=is_mounted(Part::Trigger)>
                    <SelectTrigger>
                        <Show when=is_mounted(Part::Value)>
                            <SelectValue placeholder=placeholder />
                        </Show>
                    </SelectTrigger>
                </Show>
                <Show when=is_mounted(Part::Popover)>
                    <SelectPopover>
                        <ListBox>
                            <For each=move || items.get() key=|i| *i let:i>
                                <ListBoxItem key=i attr:data-perf-option=i>
                                    {format!("Option {i}")}
                                </ListBoxItem>
                            </For>
                        </ListBox>
                    </SelectPopover>
                </Show>
                <Show when=is_mounted(Part::HiddenSelect)>
                    <HiddenSelect />
                </Show>
            </Select>
        </Show>
        <button id="perf-after">"After"</button>
    }
}
