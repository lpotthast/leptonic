//! `use_tab_panel` on a plain `<div class="perf-panel">` (`key: None`: the selected tab's panel)
//! after a tab list of two tabs (`use_tab_list_state`, `use_tab_list` and `use_tab` on plain
//! `<div>`s, the first selected), for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_tab_panel.rs`): between two plain buttons
//! (`#perf-before`, `#perf-after`) to tab from and to, mounted at first. Its controls
//! ([`Control`]) mount and unmount it, select the second and the first tab, set and unset its
//! label (`aria_label`), and add and remove a button inside it (`#perf-panel-button`: a tabbable
//! child takes it out of the tab order).

use leptonic::{
    CapturedElement, IntoAttrs, Orientation,
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

/// A tab: `use_tab` on a plain `<div>`.
#[component]
fn HookTab(state: TabListState, key: &'static str, children: Children) -> impl IntoView {
    let tab = use_tab(UseTabInput {
        state,
        key: Key::from(key),
        is_disabled: Signal::stored(false),
        should_select_on_press_up: SelectOnPressUp::Auto,
        on_focus: None,
        on_blur: None,
        on_focus_change: None,
    });
    let (attrs, styles) = tab.props.into_parts();
    view! {
        <div {..attrs} style=styles>
            {children()}
        </div>
    }
}

/// The panel: `use_tab_panel` on a plain `<div>`.
#[component]
fn HookTabPanel(
    state: TabListState,
    label: RwSignal<Option<String>>,
    with_button: RwSignal<bool>,
) -> impl IntoView {
    let panel = use_tab_panel(UseTabPanelInput {
        state,
        key: None,
        aria_label: label.into(),
        aria_describedby: Signal::default(),
        aria_details: Signal::default(),
    });
    view! {
        <div {..panel.props.into_attrs()} class="perf-panel">
            {move || {
                state.selected_key().map(|key| format!("Panel of {key}")).unwrap_or_default()
            }}
            <Show when=move || with_button.get()>
                <button id="perf-panel-button">"Inside"</button>
            </Show>
        </div>
    }
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let label = RwSignal::new(None::<String>);
    let with_button = RwSignal::new(false);
    let collection = use_collection(|b| {
        b.item("tab-1", "Tab 1");
        b.item("tab-2", "Tab 2");
    });
    let state = use_tab_list_state(UseTabListStateInput {
        collection,
        default_selected_key: None,
        selected_key: None,
        on_selection_change: None,
        disabled_keys: Signal::default(),
        is_disabled: Signal::stored(false),
    });
    let list = use_tab_list(UseTabListInput {
        state,
        element: CapturedElement::new(),
        orientation: Orientation::Horizontal.into(),
        keyboard_activation: KeyboardActivation::Automatic,
        aria_label: "Tabs".into(),
        aria_labelledby: Signal::default(),
    });
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("select-second", move || {
            state.list.set_selected_key(Some(Key::from("tab-2")));
        }),
        Control::action("select-first", move || {
            state.list.set_selected_key(Some(Key::from("tab-1")));
        }),
        Control::action("set-label", move || label.set(Some("Details".to_owned()))),
        Control::action("unset-label", move || label.set(None)),
        Control::action("add-button", move || with_button.set(true)),
        Control::action("remove-button", move || with_button.set(false)),
    ];
    view! {
        <h1>"use_tab_panel performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <div {..list.props.into_attrs()}>
            <HookTab state key="tab-1">
                "Tab 1"
            </HookTab>
            <HookTab state key="tab-2">
                "Tab 2"
            </HookTab>
        </div>
        <Show when=move || mounted.get()>
            <HookTabPanel state label with_button />
        </Show>
        <button id="perf-after">"After"</button>
    }
}
