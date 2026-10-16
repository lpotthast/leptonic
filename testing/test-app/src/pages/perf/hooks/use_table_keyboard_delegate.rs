//! A hook table whose keyboard delegate comes from `use_table_keyboard_delegate`, for its
//! performance test (`leptonic/tests/perf_tests/hooks/use_table_keyboard_delegate.rs`): the table
//! of the `use_table` fixture ([`super::use_table`]: [`SMALL`] rows "Row <i>" in the columns Name,
//! Value and Group, with multiple selection) with the delegate handed to `use_table`, in an
//! `I18nProvider`, between two plain buttons (`#perf-before`, `#perf-after`) to tab from and to,
//! mounted at first. Its controls ([`Control`]) mount and unmount the table, set its size while
//! unmounted (`small`, `large`: [`LARGE`] rows) and switch the locale to a right-to-left one
//! (`rtl`: ar-AE; `ltr`: en-US), which the delegate reads.

use std::collections::HashSet;

use leptonic::{
    CapturedElement, I18nProvider, IntoAttrs, Locale,
    hooks::{
        collections::CollectionOptions,
        gridlist::KeyboardNavigationBehavior,
        table::{
            TableState, UseTableInput, UseTableKeyboardDelegateInput, UseTableReturn, use_table,
            use_table_keyboard_delegate,
        },
    },
    use_i18n,
};
use leptos::prelude::*;

use super::use_table::{HookTableBody, HookTableHeader, LARGE, SMALL, hook_table_state};
use crate::pages::perf::{Control, PerfControls};

/// The table: `use_table` with the delegate of `use_table_keyboard_delegate`, on a `<table>`.
#[component]
fn PerfTable(state: TableState, rows: RwSignal<usize>) -> impl IntoView {
    let element = CapturedElement::new();
    let delegate = use_table_keyboard_delegate(UseTableKeyboardDelegateInput { state, element });
    let UseTableReturn { props, data } = use_table(UseTableInput {
        state,
        element,
        id: None,
        aria_label: "Perf table".into(),
        aria_labelledby: Signal::stored(None),
        keyboard_delegate: Some(delegate),
        options: CollectionOptions::default(),
        keyboard_navigation_behavior: KeyboardNavigationBehavior::Arrow,
        should_select_on_press_up: false,
        on_row_action: None,
        on_cell_action: None,
    });
    view! {
        <table {..props.into_attrs()}>
            <HookTableHeader table=data.clone() />
            <HookTableBody table=data rows />
        </table>
    }
}

/// The page inside its `I18nProvider`.
#[component]
fn Fixture() -> impl IntoView {
    let i18n = use_i18n().expect("inside an I18nProvider");
    let mounted = RwSignal::new(true);
    let rows = RwSignal::new(SMALL);
    let state = hook_table_state(rows, false, Signal::stored(HashSet::new()));
    let arabic: Locale = "ar-AE".parse().expect("a locale");
    let english: Locale = "en-US".parse().expect("a locale");
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("small", move || rows.set(SMALL)),
        Control::action("large", move || rows.set(LARGE)),
        Control::action("rtl", move || i18n.set_locale(arabic.clone())),
        Control::action("ltr", move || i18n.set_locale(english.clone())),
    ];
    view! {
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <PerfTable state rows />
        </Show>
        <button id="perf-after">"After"</button>
    }
}

#[component]
pub fn PagePerf() -> impl IntoView {
    view! {
        <h1>"use_table_keyboard_delegate performance"</h1>
        <I18nProvider>
            <Fixture />
        </I18nProvider>
    }
}
