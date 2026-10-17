//! A hook grid whose keyboard delegate comes from `use_grid_keyboard_delegate`, for its performance
//! test (`leptonic/tests/perf_tests/hooks/use_grid_keyboard_delegate.rs`): a grid of [`SMALL`] rows
//! "Row <i>" of three cells, in row focus mode with multiple selection, with the delegate handed to
//! `use_grid`, in an `I18nProvider`, between two plain buttons (`#perf-before`, `#perf-after`) to
//! tab from and to, mounted at first. Its controls ([`Control`]) mount and unmount the grid, set
//! its size while unmounted (`small`, `large`: [`LARGE`] rows) and switch the locale to a
//! right-to-left one (`rtl`: ar-AE; `ltr`: en-US), which the delegate reads.

use std::sync::Arc;

use leptonic::{
    CapturedElement, I18nProvider, Locale, flag,
    hooks::{
        collections::{
            Collection, CollectionMemo, CollectionOptions, Key, SelectionMode, SelectionOptions,
        },
        grid::{
            CellFocusMode, GridData, GridFocusMode, GridState, UseGridCellInput, UseGridCellReturn,
            UseGridInput, UseGridKeyboardDelegateInput, UseGridReturn, UseGridRowInput,
            UseGridRowReturn, UseGridStateInput, use_grid, use_grid_cell,
            use_grid_keyboard_delegate, use_grid_row, use_grid_state,
        },
        gridlist::KeyboardNavigationBehavior,
    },
    use_i18n,
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// The rows of the small grid.
const SMALL: usize = 10;

/// The rows of the large grid.
const LARGE: usize = 1_000;

/// `rows` rows ("Row <i>", key `i`), each with the cells "Row <i>", "Value <i>" and
/// "Group <i % 3>".
fn grid_collection(rows: RwSignal<usize>) -> CollectionMemo {
    Memo::new(move |_| {
        Arc::new(Collection::build(|b| {
            for i in 0..rows.get() {
                b.row(i, format!("Row {i}"), |r| {
                    r.cell(format!("Row {i}"));
                    r.cell(format!("Value {i}"));
                    r.cell(format!("Group {}", i % 3));
                });
            }
        }))
    })
}

/// A row: `use_grid_row` on a `div`, its pressed state in `data-pressed`.
#[component]
fn HookGridRow(grid: GridData, key: Key, children: Children) -> impl IntoView {
    let UseGridRowReturn {
        row: row_props,
        is_pressed,
        ..
    } = use_grid_row(UseGridRowInput {
        grid,
        key,
        on_context_menu: None,
    });
    let (attrs, styles) = row_props.attrs();
    view! {
        <div {..attrs} style=styles data-pressed=flag(is_pressed)>
            {children()}
        </div>
    }
}

/// A cell: `use_grid_cell` on a `div`, its pressed state in `data-pressed`.
#[component]
fn HookGridCell(
    grid: GridData,
    key: Key,
    #[prop(optional)] focus_mode: Option<CellFocusMode>,
    children: Children,
) -> impl IntoView {
    let UseGridCellReturn {
        cell: grid_cell_props,
        is_pressed,
    } = use_grid_cell(UseGridCellInput {
        should_select_on_press_up: grid.should_select_on_press_up,
        grid,
        key,
        id: None,
        focus_mode,
        allows_arrow_navigation: false,
    });
    let (attrs, styles) = grid_cell_props.attrs();
    view! {
        <div {..attrs} style=styles data-pressed=flag(is_pressed)>
            {children()}
        </div>
    }
}

/// Row `i` with its three cells.
#[component]
fn HookGridRowWithCells(grid: GridData, i: usize) -> impl IntoView {
    let row = Key::from(i);
    view! {
        <HookGridRow grid=grid.clone() key=row.clone()>
            <HookGridCell grid=grid.clone() key=Key::cell(&row, 0)>
                {format!("Row {i}")}
            </HookGridCell>
            <HookGridCell grid=grid.clone() key=Key::cell(&row, 1)>
                {format!("Value {i}")}
            </HookGridCell>
            <HookGridCell grid=grid key=Key::cell(&row, 2)>
                {format!("Group {}", i % 3)}
            </HookGridCell>
        </HookGridRow>
    }
}

/// The grid: `use_grid` with the delegate of `use_grid_keyboard_delegate`, on a `div`.
#[component]
fn PerfGrid(state: GridState, rows: RwSignal<usize>) -> impl IntoView {
    let element = CapturedElement::new();
    let delegate = use_grid_keyboard_delegate(UseGridKeyboardDelegateInput { state, element });
    let UseGridReturn { grid: props, data } = use_grid(UseGridInput {
        state,
        element,
        id: None,
        aria_label: "Perf grid".into(),
        aria_labelledby: Signal::stored(None),
        keyboard_delegate: Some(delegate),
        options: CollectionOptions::default(),
        keyboard_navigation_behavior: KeyboardNavigationBehavior::Arrow,
        should_select_on_press_up: false,
        on_row_action: None,
        on_cell_action: None,
    });
    view! {
        <div {..props.attrs()}>
            <For each=move || 0..rows.get() key=|i| *i let:i>
                <HookGridRowWithCells grid=data.clone() i=i />
            </For>
        </div>
    }
}

/// The page inside its `I18nProvider`.
#[component]
fn Fixture() -> impl IntoView {
    let i18n = use_i18n().expect("inside an I18nProvider");
    let mounted = RwSignal::new(true);
    let rows = RwSignal::new(SMALL);
    let state = use_grid_state(UseGridStateInput {
        collection: grid_collection(rows),
        selection: SelectionOptions {
            selection_mode: Signal::stored(SelectionMode::Multiple),
            ..SelectionOptions::default()
        },
        focus_mode: GridFocusMode::Row,
    });
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
            <PerfGrid state rows />
        </Show>
        <button id="perf-after">"After"</button>
    }
}

#[component]
pub fn PagePerf() -> impl IntoView {
    view! {
        <h1>"use_grid_keyboard_delegate performance"</h1>
        <I18nProvider>
            <Fixture />
        </I18nProvider>
    }
}
