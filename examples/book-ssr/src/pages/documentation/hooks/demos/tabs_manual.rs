use leptonic::{
    CapturedElement, IntoAttrs, Orientation,
    hooks::{
        collections::{Key, SelectOnPressUp, use_collection},
        tabs::{
            KeyboardActivation, TabListState, UseTabInput, UseTabListInput, UseTabListReturn,
            UseTabListStateInput, UseTabPanelInput, UseTabReturn, use_tab, use_tab_list,
            use_tab_list_state, use_tab_panel,
        },
    },
};
use leptos::prelude::*;

const TABS: [(&str, &str, &str); 3] = [
    ("inbox", "Inbox", "3 unread messages."),
    ("drafts", "Drafts", "1 draft, saved yesterday."),
    ("archive", "Archive", "Everything older than 30 days."),
];

/// One tab, rendered with `use_tab`.
#[component]
fn DemoTab(state: TabListState, key: &'static str, label: &'static str) -> impl IntoView {
    let UseTabReturn {
        props: tab_props, ..
    } = use_tab(UseTabInput {
        state,
        key: Key::from(key),
        is_disabled: Signal::stored(false),
        should_select_on_press_up: SelectOnPressUp::Auto,
        on_focus: None,
        on_blur: None,
        on_focus_change: None,
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

    // Manual activation: the arrow keys only move focus, Enter or Space selects the focused tab.
    let UseTabListReturn { props } = use_tab_list(UseTabListInput {
        keyboard_activation: KeyboardActivation::Manual,
        aria_label: "Mailbox".into(),
        state,
        element: CapturedElement::new(),
        orientation: Orientation::Horizontal.into(),
        aria_labelledby: None,
    });
    let panel = use_tab_panel(UseTabPanelInput {
        state,
        key: None,
        aria_label: MaybeProp::default(),
        aria_describedby: None,
        aria_details: None,
    });

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
                .map(|(key, label, _)| view! { <DemoTab state=state key=key label=label/> })
                .collect_view()}
        </div>
        <div {..panel.props.into_attrs()} class="demo-tab-panel">
            <p>{content}</p>
        </div>

        <p class="demo-status">
            "Focused: " {move || key_text(selection.is_focused().then(|| selection.focused_key()).flatten())}
            " \u{b7} Selected: " {move || key_text(state.selected_key())}
        </p>
    }
}
