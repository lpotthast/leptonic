use leptonic::hooks::{
    FocusStrategy, MenuTriggerType, OverlayTriggerType, UseMenuTriggerInput,
    UseMenuTriggerStateInput, use_button, use_menu_trigger, use_menu_trigger_state,
};
use leptos::prelude::*;

/// A menu trigger without a menu: the test observes the open state and focus strategy directly.
#[component]
pub fn PageHookMenuTrigger() -> impl IntoView {
    let state = use_menu_trigger_state(UseMenuTriggerStateInput::default());
    let menu_trigger = use_menu_trigger(UseMenuTriggerInput {
        menu_type: OverlayTriggerType::Menu,
        is_disabled: false.into(),
        trigger: MenuTriggerType::Press,
        state,
    });
    let (attrs, styles) = use_button(menu_trigger.button).props.into_parts();
    let menu_id = menu_trigger.menu_props.id;

    let strategy = move || match state.focus_strategy.get() {
        Some(FocusStrategy::First) => "first",
        Some(FocusStrategy::Last) => "last",
        None => "none",
    };

    view! {
        <div id="test-page-hook-menu-trigger">
            <h1>"use_menu_trigger"</h1>
            <button id="test-mt-before">"Before"</button>
            <button {..attrs} style=styles data-testid="trigger">"Actions"</button>
            <button id="test-mt-close" on:click=move |_| state.close()>"Close"</button>
            <div id=menu_id>"(menu)"</div>
            <div>"Open: " <span id="test-mt-is-open">{move || state.is_open().to_string()}</span></div>
            <div>"Focus strategy: " <span id="test-mt-strategy">{strategy}</span></div>
        </div>
    }
}
