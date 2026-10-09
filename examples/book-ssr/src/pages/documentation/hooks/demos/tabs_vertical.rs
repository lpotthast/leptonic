use leptonic::{
    CapturedElement, IntoAttrs, Orientation,
    hooks::{
        collections::{Key, SelectOnPressUp, use_collection},
        tabs::{
            KeyboardActivation, TabListState, UseTabInput, UseTabListInput, UseTabListReturn,
            UseTabListStateInput, UseTabPanelInput, use_tab, use_tab_list, use_tab_list_state,
            use_tab_panel,
        },
    },
};
use leptos::prelude::*;

const TABS: [(&str, &str, &str); 3] = [
    ("profile", "Profile", "Your name, photo and bio."),
    (
        "security",
        "Security",
        "Password and two-factor authentication.",
    ),
    ("billing", "Billing", "Payment methods and invoices."),
];

/// One tab, rendered with `use_tab`.
#[component]
fn DemoTab(state: TabListState, key: &'static str, label: &'static str) -> impl IntoView {
    let (attrs, styles) = use_tab(UseTabInput {
        state,
        key: Key::from(key),
        is_disabled: Signal::stored(false),
        should_select_on_press_up: SelectOnPressUp::Auto,
        on_focus: None,
        on_blur: None,
        on_focus_change: None,
    })
    .props
    .into_parts();

    view! { <div {..attrs} class="demo-hook-tab" style=styles>{label}</div> }
}

/// The panel of one tab, rendered with `use_tab_panel` while its tab is selected.
#[component]
fn DemoTabPanel(state: TabListState, key: &'static str, text: &'static str) -> impl IntoView {
    let is_selected = move || state.selected_key() == Some(Key::from(key));

    view! {
        <Show when=is_selected>
            {
                let panel = use_tab_panel(UseTabPanelInput { state, key: Some(Key::from(key)), aria_label: MaybeProp::default(), aria_describedby: None, aria_details: None });
                view! {
                    <div {..panel.props.into_attrs()} class="demo-tab-panel">
                        <p>{text}</p>
                    </div>
                }
            }
        </Show>
    }
}

#[component]
pub fn TabsVerticalDemo() -> impl IntoView {
    let collection = use_collection(|b| {
        for (key, label, _) in TABS {
            b.item(key, label);
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

    // Vertical: Arrow Up and Arrow Down move between the tabs (Arrow Left and Arrow Right still work).
    let UseTabListReturn { props } = use_tab_list(UseTabListInput {
        orientation: Orientation::Vertical.into(),
        aria_label: "Settings".into(),
        state,
        element: CapturedElement::new(),
        keyboard_activation: KeyboardActivation::Automatic,
        aria_labelledby: None,
    });

    view! {
        <div class="demo-tabs-vertical">
            <div {..props.into_attrs()} class="demo-tab-list">
                {TABS
                    .map(|(key, label, _)| view! { <DemoTab state=state key=key label=label/> })
                    .collect_view()}
            </div>
            // One panel per tab; only the selected tab's panel is rendered.
            {TABS
                .map(|(key, _, text)| view! { <DemoTabPanel state=state key=key text=text/> })
                .collect_view()}
        </div>

        <p class="demo-status">
            "Selected: " {move || state.selected_key().map(|key| key.to_string()).unwrap_or_default()}
        </p>
    }
}
