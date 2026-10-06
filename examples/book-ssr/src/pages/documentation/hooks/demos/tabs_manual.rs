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
    let UseTabReturn { tab_props, .. } = use_tab(UseTabInput::new(list, Key::from(key)));
    let (attrs, styles) = tab_props.into_parts();

    view! { <div {..attrs} class="demo-tab demo-hook-tab" style=styles>{label}</div> }
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
        ..UseTabListStateInput::new(collection)
    });
    let tabs = TabListData::new(state);

    // Manual activation: the arrow keys only move focus, Enter or Space selects the focused tab.
    let UseTabListReturn { props, data } = use_tab_list(UseTabListInput {
        keyboard_activation: KeyboardActivation::Manual,
        aria_label: "Mailbox".into(),
        ..UseTabListInput::new(tabs.clone(), CapturedElement::new())
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

        <div class="demo-tabs-controls">
            <p class="demo-tabs-status">
                "Focused: " {move || key_text(selection.is_focused().then(|| selection.focused_key()).flatten())}
                " \u{b7} Selected: " {move || key_text(state.selected_key())}
            </p>
        </div>
    }
}
