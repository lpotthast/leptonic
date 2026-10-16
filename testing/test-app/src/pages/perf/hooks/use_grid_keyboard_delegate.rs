//! A hook grid whose keyboard delegate comes from `use_grid_keyboard_delegate`, for its
//! performance test (`leptonic/tests/perf_tests/hooks/use_grid_keyboard_delegate.rs`): the grid
//! of the `use_grid` fixture ([`super::use_grid`]: [`SMALL`] rows "Row <i>" of three cells, in row
//! focus mode with multiple selection) with the delegate handed to `use_grid`, in an
//! `I18nProvider`, between two plain buttons (`#perf-before`, `#perf-after`) to tab from and to,
//! mounted at first. Its controls ([`Control`]) mount and unmount the grid, set its size while
//! unmounted (`small`, `large`: [`LARGE`] rows) and switch the locale to a right-to-left one
//! (`rtl`: ar-AE; `ltr`: en-US), which the delegate reads.

use leptonic::{
    CapturedElement, I18nProvider, IntoAttrs, Locale,
    hooks::{
        collections::{CollectionOptions, SelectionMode, SelectionOptions},
        grid::{
            GridFocusMode, GridState, UseGridInput, UseGridKeyboardDelegateInput, UseGridReturn,
            UseGridStateInput, use_grid, use_grid_keyboard_delegate, use_grid_state,
        },
        gridlist::KeyboardNavigationBehavior,
    },
    use_i18n,
};
use leptos::prelude::*;

use super::use_grid::{HookGridRowWithCells, LARGE, SMALL, grid_collection};
use crate::pages::perf::{Control, PerfControls};

/// The grid: `use_grid` with the delegate of `use_grid_keyboard_delegate`, on a `div`.
#[component]
fn PerfGrid(state: GridState, rows: RwSignal<usize>) -> impl IntoView {
    let element = CapturedElement::new();
    let delegate = use_grid_keyboard_delegate(UseGridKeyboardDelegateInput { state, element });
    let UseGridReturn { props, data } = use_grid(UseGridInput {
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
        <div {..props.into_attrs()}>
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
