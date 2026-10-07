use leptonic::hooks::Orientation;
use leptonic::hooks::collections::SelectOnPressUp;
use leptonic::{
    hooks::{
        IntoAttrs, Key, KeyboardActivation, TabListData, TabListItemData, UseTabInput,
        UseTabListInput, UseTabListReturn, UseTabListStateInput, UseTabPanelInput, UseTabReturn,
        use_collection, use_tab, use_tab_list, use_tab_list_state, use_tab_panel,
    },
    utils::CapturedElement,
};
use leptos::prelude::*;

const TABS: [(&str, &str, &str); 3] = [
    ("inbox", "Inbox", "3 unread messages."),
    ("drafts", "Drafts", "1 draft, saved yesterday."),
    ("archive", "Archive", "Everything older than 30 days."),
];

/// One tab, rendered with `use_tab`.
#[component]
fn DemoTab(list: TabListItemData, key: &'static str, label: &'static str) -> impl IntoView {
    let UseTabReturn { tab_props, .. } = use_tab(UseTabInput {
        list,
        key: Key::from(key),
        is_disabled: Signal::stored(false),
        should_select_on_press_up: SelectOnPressUp::Auto,
    });
    let (attrs, styles) = tab_props.into_parts();

    view! { <div {..attrs} class="demo-hook-tab" style=styles>{label}</div> }
}

#[component]
pub fn TabsManualDemo() -> impl IntoView {
    let collection = use_collection(|b| {
        for (key, label, _) in TABS {
            b.item(key, label);
        }
    });
    let state = use_tab_list_state(UseTabListStateInput {
        default_selected_key: Some(Key::from("drafts")),
        collection,
        selected_key: None,
        on_selection_change: None,
        disabled_keys: Signal::default(),
        is_disabled: Signal::stored(false),
    });
    let tabs = TabListData::new(state);

    // Manual activation: the arrow keys only move focus, Enter or Space selects the focused tab.
    let UseTabListReturn { props, data } = use_tab_list(UseTabListInput {
        keyboard_activation: KeyboardActivation::Manual,
        aria_label: "Mailbox".into(),
        tabs: tabs.clone(),
        element: CapturedElement::new(),
        orientation: Orientation::Horizontal,
        aria_labelledby: None,
    });
    let panel = use_tab_panel(UseTabPanelInput { tabs, key: None });

    let selection = state.list.list.selection;
    let key_text =
        |key: Option<Key>| key.map_or_else(|| "\u{2014}".to_owned(), |key| key.to_string());
    let content = move || {
        let selected = state.selected_key();
        TABS.iter()
            .find(|(key, ..)| selected.as_ref() == Some(&Key::from(*key)))
            .map(|(_, _, text)| *text)
    };

    view! {
        <div {..props.into_attrs()} class="demo-tab-list">
            {TABS
                .map(|(key, label, _)| view! { <DemoTab list=data.clone() key=key label=label/> })
                .collect_view()}
        </div>
        <div {..panel.tab_panel_props.into_attrs()} class="demo-tab-panel">
            <p>{content}</p>
        </div>

        <p class="demo-status">
            "Focused: " {move || key_text(selection.is_focused().then(|| selection.focused_key()).flatten())}
            " \u{b7} Selected: " {move || key_text(state.selected_key())}
        </p>
    }
}
