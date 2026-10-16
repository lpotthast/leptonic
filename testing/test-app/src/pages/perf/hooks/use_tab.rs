//! `use_tab` on a plain `<div class="perf-tab">` ("Tab 2"), the second tab of a tab list
//! (`use_tab_list_state` and `use_tab_list`, manual activation: a focused tab is selected by
//! Enter or Space, not by focus) after a first tab (`.perf-first`, selected at first), for its
//! performance test (`leptonic/tests/perf_tests/hooks/use_tab.rs`): its pressed and focused state
//! shown (`data-pressed`, `data-focused`), between two plain buttons (`#perf-before`,
//! `#perf-after`) to tab from and to, mounted at first. Its controls ([`Control`]) mount and
//! unmount it, disable and enable it (its `is_disabled`), and read how often the selection
//! changed (`changes`).

use leptonic::{
    CapturedElement, IntoAttrs, Orientation, flag,
    hooks::{
        collections::{Key, SelectOnPressUp, use_collection},
        tabs::{
            KeyboardActivation, TabListState, UseTabInput, UseTabListInput, UseTabListStateInput,
            UseTabPanelInput, use_tab, use_tab_list, use_tab_list_state, use_tab_panel,
        },
    },
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// A tab: `use_tab` on a plain `<div class=..>`.
#[component]
fn HookTab(
    state: TabListState,
    class_name: &'static str,
    key: &'static str,
    is_disabled: Signal<bool>,
    children: Children,
) -> impl IntoView {
    let tab = use_tab(UseTabInput {
        state,
        key: Key::from(key),
        is_disabled,
        should_select_on_press_up: SelectOnPressUp::Auto,
        on_focus: None,
        on_blur: None,
        on_focus_change: None,
    });
    let (attrs, styles) = tab.props.into_parts();
    view! {
        <div
            {..attrs}
            class=class_name
            style=styles
            data-pressed=flag(tab.is_pressed)
            data-focused=flag(tab.is_focused)
        >
            {children()}
        </div>
    }
}

/// The selected tab's panel: `use_tab_panel` on a plain `<div>` (the tabs control it).
#[component]
fn HookTabPanel(state: TabListState) -> impl IntoView {
    let panel = use_tab_panel(UseTabPanelInput {
        state,
        key: None,
        aria_label: MaybeProp::default(),
        aria_describedby: Signal::default(),
        aria_details: Signal::default(),
    });
    view! { <div {..panel.props.into_attrs()}>"Panel"</div> }
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let disabled = RwSignal::new(false);
    let change_count = StoredValue::new(0_u32);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("disable", move || disabled.set(true)),
        Control::action("enable", move || disabled.set(false)),
        Control::read("changes", move || f64::from(change_count.get_value())),
    ];
    let collection = use_collection(|b| {
        b.item("tab-1", "Tab 1");
        b.item("tab-2", "Tab 2");
    });
    let state = use_tab_list_state(UseTabListStateInput {
        collection,
        default_selected_key: None,
        selected_key: None,
        on_selection_change: Some(Callback::new(move |_| {
            change_count.update_value(|count| *count += 1);
        })),
        disabled_keys: Signal::default(),
        is_disabled: Signal::stored(false),
    });
    let list = use_tab_list(UseTabListInput {
        state,
        element: CapturedElement::new(),
        orientation: Orientation::Horizontal.into(),
        keyboard_activation: KeyboardActivation::Manual,
        aria_label: "Tabs".into(),
        aria_labelledby: Signal::default(),
    });
    view! {
        <h1>"use_tab performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <div {..list.props.into_attrs()}>
            <HookTab state class_name="perf-first" key="tab-1" is_disabled=Signal::stored(false)>
                "Tab 1"
            </HookTab>
            <Show when=move || mounted.get()>
                <HookTab state class_name="perf-tab" key="tab-2" is_disabled=disabled.into()>
                    "Tab 2"
                </HookTab>
            </Show>
        </div>
        <HookTabPanel state />
        <button id="perf-after">"After"</button>
    }
}
