//! Hook column headers (`use_table_column_header` on `<th>`s, in `use_table_header_row`s) of a
//! hook table, for the performance tests of `use_table_column_header` and
//! `use_table_header_row` (`leptonic/tests/perf_tests/hooks/use_table_column_header.rs`,
//! `.../use_table_header_row.rs`, whose fixture this is too): the table of the `use_table`
//! fixture ([`super::use_table`]: [`SMALL`] rows "Row <i>" in the sortable columns Name, Value
//! and Group, with multiple selection), between two plain buttons (`#perf-before`,
//! `#perf-after`) to tab from and to, mounted at first. Its controls ([`Control`]) mount and
//! unmount the header alone (`mount-header`, `unmount-header`) or the table (`mount`,
//! `unmount`), set the size while unmounted (`small`, `large`: [`LARGE`] rows) and prepare the
//! live announcer (`prepare-announcer`: its first announcement waits for its live regions).

use std::collections::HashSet;

use leptonic::{
    Assertiveness, CapturedElement, IntoAttrs, announce,
    hooks::{
        collections::CollectionOptions,
        gridlist::KeyboardNavigationBehavior,
        table::{TableState, UseTableInput, UseTableReturn, use_table},
    },
};
use leptos::prelude::*;

use super::use_table::{HookTableBody, HookTableHeader, LARGE, SMALL, hook_table_state};
use crate::pages::perf::{Control, PerfControls};

/// The table: `use_table` on a `<table>`, its header mounted while `header`.
#[component]
fn PerfTable(state: TableState, rows: RwSignal<usize>, header: RwSignal<bool>) -> impl IntoView {
    let UseTableReturn { props, data } = use_table(UseTableInput {
        state,
        element: CapturedElement::new(),
        id: None,
        aria_label: "Perf table".into(),
        aria_labelledby: Signal::stored(None),
        keyboard_delegate: None,
        options: CollectionOptions::default(),
        keyboard_navigation_behavior: KeyboardNavigationBehavior::Arrow,
        should_select_on_press_up: false,
        on_row_action: None,
        on_cell_action: None,
    });
    let header_table = data.clone();
    view! {
        <table {..props.into_attrs()}>
            <Show when=move || header.get()>
                <HookTableHeader table=header_table.clone() />
            </Show>
            <HookTableBody table=data rows />
        </table>
    }
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let header = RwSignal::new(true);
    let rows = RwSignal::new(SMALL);
    let state = hook_table_state(rows, false, Signal::stored(HashSet::new()));
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("mount-header", move || header.set(true)),
        Control::action("unmount-header", move || header.set(false)),
        Control::action("small", move || rows.set(SMALL)),
        Control::action("large", move || rows.set(LARGE)),
        Control::action("prepare-announcer", || announce("", Assertiveness::Polite)),
    ];
    view! {
        <h1>"use_table_column_header performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <PerfTable state rows header />
        </Show>
        <button id="perf-after">"After"</button>
    }
}
