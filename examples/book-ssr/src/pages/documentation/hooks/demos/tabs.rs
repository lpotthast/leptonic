use leptonic::hooks::KeyboardActivation;
use leptonic::hooks::Orientation;
use leptonic::hooks::collections::SelectOnPressUp;
use leptonic::{
    atoms::checkbox::{CheckboxButton, CheckboxField},
    hooks::{
        IntoAttrs, Key, TabListData, TabListItemData, UseTabInput, UseTabListInput,
        UseTabListReturn, UseTabListStateInput, UseTabPanelInput, UseTabReturn, use_collection,
        use_tab, use_tab_list, use_tab_list_state, use_tab_panel,
    },
    utils::CapturedElement,
};
use leptos::prelude::*;

const TABS: [(&str, &str); 4] = [
    ("details", "Details"),
    ("specs", "Specs"),
    ("reviews", "Reviews"),
    ("shipping", "Shipping"),
];

/// One tab, rendered with `use_tab`. Its state shows in `aria-selected` and `aria-disabled`.
#[component]
fn DemoTab(list: TabListItemData, key: &'static str, label: &'static str) -> impl IntoView {
    let UseTabReturn { props: tab_props, .. } = use_tab(UseTabInput {
        list,
        key: Key::from(key),
        is_disabled: Signal::stored(false),
        should_select_on_press_up: SelectOnPressUp::Auto,
    });
    let (attrs, styles) = tab_props.into_parts();

    view! { <div {..attrs} class="demo-hook-tab" style=styles>{label}</div> }
}

#[component]
pub fn TabsDemo() -> impl IntoView {
    let disabled = RwSignal::new(false);

    // The tabs, as a collection. "Reviews" is disabled: it can't be selected, and the arrow keys skip it.
    let collection = use_collection(|b| {
        for (key, label) in TABS {
            b.item(key, label).disabled(key == "reviews");
        }
    });
    // Without a `default_selected_key`, the first enabled tab is selected.
    let state = use_tab_list_state(UseTabListStateInput {
        is_disabled: disabled.into(),
        collection,
        default_selected_key: None,
        selected_key: None,
        on_selection_change: None,
        disabled_keys: Signal::default(),
    });
    let tabs = TabListData::new(state);

    let UseTabListReturn { props, data } = use_tab_list(UseTabListInput {
        aria_label: "Product".into(),
        tabs: tabs.clone(),
        element: CapturedElement::new(),
        orientation: Orientation::Horizontal.into(),
        keyboard_activation: KeyboardActivation::Automatic,
        aria_labelledby: None,
    });
    // `key: None`: one panel that always shows the selected tab.
    let panel = use_tab_panel(UseTabPanelInput { tabs, key: None, aria_describedby: None, aria_details: None });

    let content = move || {
        match state.selected_key().map(|key| key.to_string()).as_deref() {
        Some("specs") => view! {
            <h4>"Specs"</h4>
            <p>"Aluminium frame, 1.2 kg, USB-C charging."</p>
        }
        .into_any(),
        Some("shipping") => view! {
            <h4>"Shipping"</h4>
            <p>"This panel contains a checkbox, so Tab moves to the checkbox instead of the panel."</p>
            <CheckboxField>
                <CheckboxButton classes="demo-check">
                    <span class="demo-check-box" aria-hidden="true"></span>
                    "Express delivery"
                </CheckboxButton>
            </CheckboxField>
        }
        .into_any(),
        _ => view! {
            <h4>"Details"</h4>
            <p>"A lightweight, foldable reading lamp."</p>
        }
        .into_any(),
    }
    };

    view! {
        <div {..props.into_attrs()} class="demo-tab-list">
            {TABS
                .map(|(key, label)| view! { <DemoTab list=data.clone() key=key label=label/> })
                .collect_view()}
        </div>
        <div {..panel.props.into_attrs()} class="demo-tab-panel">{content}</div>

        <div class="demo-controls">
            <CheckboxField is_selected=disabled set_selected=disabled>
                <CheckboxButton classes="demo-check">
                    <span class="demo-check-box" aria-hidden="true"></span>
                    "Disabled"
                </CheckboxButton>
            </CheckboxField>
        </div>
        <p class="demo-status">
            "Selected: " {move || state.selected_key().map(|key| key.to_string()).unwrap_or_default()}
        </p>
    }
}
