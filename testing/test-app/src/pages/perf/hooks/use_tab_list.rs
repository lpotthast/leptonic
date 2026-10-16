//! `use_tab_list` on a plain `<div class="perf-tab-list">` with [`FEW`] tabs (or [`MANY`], control
//! `use-many` while unmounted; `use_tab` on plain `<div>`s, "Tab 1", ... with the keys `tab-1`,
//! ...), for its performance test (`leptonic/tests/perf_tests/hooks/use_tab_list.rs`): its state
//! (`use_tab_list_state`) created once, outside of what mounts; between two plain buttons
//! (`#perf-before`, `#perf-after`) to tab from and to, mounted at first. Its controls
//! ([`Control`]) mount and unmount it, set and unset its label (`aria_label`), and change its
//! orientation.

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

/// The tabs of the small tab list.
const FEW: usize = 3;
/// The tabs of the large tab list.
const MANY: usize = 100;

fn key(i: usize) -> Key {
    Key::from(format!("tab-{i}"))
}

/// A tab: `use_tab` on a plain `<div>`.
#[component]
fn HookTab(state: TabListState, i: usize) -> impl IntoView {
    let tab = use_tab(UseTabInput {
        state,
        key: key(i),
        is_disabled: Signal::stored(false),
        should_select_on_press_up: SelectOnPressUp::Auto,
        on_focus: None,
        on_blur: None,
        on_focus_change: None,
    });
    let (attrs, styles) = tab.props.into_parts();
    view! { <div {..attrs} style=styles>{format!("Tab {i}")}</div> }
}

/// The tab list: `use_tab_list` on a plain `<div>`, with its tabs.
#[component]
fn HookTabList(
    state: TabListState,
    count: usize,
    label: RwSignal<Option<String>>,
    orientation: RwSignal<Orientation>,
) -> impl IntoView {
    let list = use_tab_list(UseTabListInput {
        state,
        element: CapturedElement::new(),
        orientation: orientation.into(),
        keyboard_activation: KeyboardActivation::Automatic,
        aria_label: label.into(),
        aria_labelledby: Signal::default(),
    });
    view! {
        <div {..list.props.into_attrs()} class="perf-tab-list">
            {(1..=count).map(|i| view! { <HookTab state i /> }).collect_view()}
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
    let count = RwSignal::new(FEW);
    let label = RwSignal::new(None::<String>);
    let orientation = RwSignal::new(Orientation::Horizontal);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("use-many", move || count.set(MANY)),
        Control::action("set-label", move || label.set(Some("Sections".to_owned()))),
        Control::action("unset-label", move || label.set(None)),
        Control::action("make-vertical", move || {
            orientation.set(Orientation::Vertical);
        }),
        Control::action("make-horizontal", move || {
            orientation.set(Orientation::Horizontal);
        }),
    ];
    let collection = use_collection(move |b| {
        for i in 1..=count.get() {
            b.item(key(i), format!("Tab {i}"));
        }
    });
    let state = use_tab_list_state(UseTabListStateInput {
        collection,
        default_selected_key: None,
        selected_key: None,
        on_selection_change: None,
        disabled_keys: Signal::default(),
        is_disabled: Signal::stored(false),
    });
    view! {
        <h1>"use_tab_list performance"</h1>
        <PerfControls controls />
        <button id="perf-before">"Before"</button>
        <Show when=move || mounted.get()>
            <HookTabList state count=count.get_untracked() label orientation />
        </Show>
        <HookTabPanel state />
        <button id="perf-after">"After"</button>
    }
}
