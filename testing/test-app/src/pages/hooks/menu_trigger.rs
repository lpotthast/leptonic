use leptonic::hooks::{
    button::use_button,
    collections::FocusStrategy,
    menu::{
        MenuTriggerState, MenuTriggerType, UseMenuTriggerInput, UseMenuTriggerStateInput,
        use_menu_trigger, use_menu_trigger_state,
    },
    overlay::OverlayTriggerType,
};
use leptos::prelude::*;

/// Menu triggers without a menu: the test observes the open state and focus strategy directly.
/// - "Actions" (`[data-testid=trigger]`): a press trigger; its state in `#test-mt-is-open` and
///   `#test-mt-strategy`, `#test-mt-close` closes it.
/// - "Disabled" (`[data-testid=disabled-trigger]`): a disabled press trigger
///   (`#test-mt-disabled-is-open`).
/// - "More" (`[data-testid=long-press-trigger]`): a long press trigger (`#test-mt-long-is-open`,
///   `#test-mt-long-strategy`).
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

    view! {
        <div id="test-page-hook-menu-trigger">
            <h1>"use_menu_trigger"</h1>
            <button id="test-mt-before">"Before"</button>
            <button {..attrs} style=styles data-testid="trigger">
                "Actions"
            </button>
            <button id="test-mt-close" on:click=move |_| state.close()>
                "Close"
            </button>
            <div id=menu_id>"(menu)"</div>
            <div>
                "Open: " <span id="test-mt-is-open">{move || state.is_open().to_string()}</span>
            </div>
            <div>
                "Focus strategy: " <span id="test-mt-strategy">{move || strategy(state)}</span>
            </div>
            <OtherTrigger
                name="disabled"
                label="Disabled"
                trigger=MenuTriggerType::Press
                is_disabled=true
            />
            <OtherTrigger
                name="long"
                label="More"
                trigger=MenuTriggerType::LongPress
                is_disabled=false
            />
        </div>
    }
}

fn strategy(state: MenuTriggerState) -> &'static str {
    match state.focus_strategy.get() {
        Some(FocusStrategy::First) => "first",
        Some(FocusStrategy::Last) => "last",
        None => "none",
    }
}

/// Another trigger (`[data-testid=<name>-trigger]`, the long press one `long-press-trigger`)
/// with its open state in `#test-mt-<name>-is-open` and focus strategy in
/// `#test-mt-<name>-strategy`.
#[component]
fn OtherTrigger(
    name: &'static str,
    label: &'static str,
    trigger: MenuTriggerType,
    is_disabled: bool,
) -> impl IntoView {
    let state = use_menu_trigger_state(UseMenuTriggerStateInput::default());
    let menu_trigger = use_menu_trigger(UseMenuTriggerInput {
        menu_type: OverlayTriggerType::Menu,
        is_disabled: is_disabled.into(),
        trigger,
        state,
    });
    let (attrs, styles) = use_button(menu_trigger.button).props.into_parts();
    let test_id = if trigger == MenuTriggerType::LongPress {
        "long-press-trigger".to_owned()
    } else {
        format!("{name}-trigger")
    };
    view! {
        <button {..attrs} style=styles data-testid=test_id>
            {label}
        </button>
        <div id=menu_trigger.menu_props.id>"(menu)"</div>
        <div>
            "Open: "
            <span id=format!("test-mt-{name}-is-open")>{move || state.is_open().to_string()}</span>
        </div>
        <div>
            "Focus strategy: "
            <span id=format!("test-mt-{name}-strategy")>{move || strategy(state)}</span>
        </div>
    }
}
