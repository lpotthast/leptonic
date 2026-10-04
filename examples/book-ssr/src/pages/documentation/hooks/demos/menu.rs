use std::collections::HashSet;

use leptonic::{
    atoms::focus_scope::FocusScope,
    hooks::{PlacementX, PlacementY, *},
    utils::{classes::Classes, locale::WritingDirection},
};
use leptos::{portal::Portal, prelude::*};

/// A single menu item component that uses the `use_menu_item` hook.
#[component]
fn MenuItem(
    /// The key/label for this menu item.
    item: String,
    /// Whether this item is disabled.
    #[prop(into)]
    is_disabled: Signal<bool>,
    /// The currently focused key in the menu.
    focused_key: Signal<Option<String>>,
    /// The current selection state.
    selected_keys: Signal<Selection<String>>,
    /// The selection mode from the parent menu.
    selection_mode: SelectionMode,
    /// Callback when this item gains focus.
    on_focus: Callback<Option<String>>,
    /// Callback when this item is activated.
    on_action: Callback<String>,
    /// Callback to close the menu.
    on_close: Callback<()>,
) -> impl IntoView {
    let UseMenuItemReturn {
        item_props,
        label_props: _,
        description_props: _,
        keyboard_shortcut_props: _,
        is_focused,
        is_selected: _,
        is_disabled,
        is_focus_visible,
    } = use_menu_item(UseMenuItemInput {
        key: item.clone(),
        is_disabled,
        focused_key,
        selected_keys,
        on_focus,
        on_action,
        on_close: Some(on_close),
        close_on_select: None,
        selection_mode,
        has_description: false,
        has_keyboard_shortcut: false,
    });

    let item_label = item.clone();

    view! {
        <li
            {..item_props.into_attrs()}
            class=Classes::from("demo-menu-item")
                .add_reactive("disabled", is_disabled)
                .add_reactive("focused", is_focused)
                .add_reactive("focus-visible", is_focus_visible)
        >
            {item_label}
        </li>
    }
}

#[component]
pub fn MenuDemo() -> impl IntoView {
    // State for the menu - use the new state hook
    let state = use_menu_trigger_state(UseMenuTriggerStateInput::default());
    let (selected, set_selected) = signal::<Option<String>>(None);

    // Menu items
    let items = vec![
        "Edit".to_string(),
        "Duplicate".to_string(),
        "Archive".to_string(),
        "Delete".to_string(),
    ];
    let items_signal = Signal::derive({
        let items = items.clone();
        move || items.clone()
    });

    // The menu trigger configures the button: one `use_button` call renders the trigger.
    let menu_trigger = use_menu_trigger(UseMenuTriggerInput {
        menu_type: OverlayTriggerType::Menu,
        disabled: false.into(),
        trigger: MenuTriggerType::Press,
        state,
    });
    let button = use_button(menu_trigger.button);

    // Capture the menu id so `aria-controls` on the trigger points to the `<ul>`.
    let menu_id = menu_trigger.menu_props.id;

    let (button_props, trigger_styles) = button.props.into_parts();

    // Set up the popover for overlay positioning and dismiss behavior.
    let popover = use_popover(UsePopoverInput {
        is_open: state.is_open,
        on_close: state.close,
        placement_x: Signal::derive(|| PlacementX::Start),
        placement_y: Signal::derive(|| PlacementY::Below),
        writing_direction: Signal::derive(|| WritingDirection::Ltr),
        offset: 4.0.into(),
        cross_offset: 0.0.into(),
        container_padding: 12.0.into(),
        should_flip: true.into(),
        is_non_modal: false,
        is_keyboard_dismiss_disabled: false,
        should_close_on_interact_outside: None,
    });

    // Memoize popover attrs before the Show boundary (Props is non-Clone, Attrs is Clone).
    let popover_trigger_attrs = StoredValue::new(popover.trigger_props.into_attrs());
    let (popover_props_attrs, popover_styles) = popover.props.into_parts();
    let popover_attrs = StoredValue::new(popover_props_attrs);
    let popover_styles = StoredValue::new(popover_styles);
    let underlay_attrs = StoredValue::new(popover.underlay_props.into_attrs());

    // Set up the menu with auto_focus connected to state.focus_strategy
    let menu = use_menu(UseMenuInput {
        aria_label: Some("Actions".to_string()),
        all_keys: items_signal,
        disabled_keys: Signal::derive(HashSet::new),
        get_key_label: Callback::new(|k: String| k.clone()),
        on_action: Some(Callback::new(move |key: String| {
            set_selected.set(Some(key));
        })),
        on_close: Some(state.close),
        auto_focus: state.focus_strategy,
        ..Default::default()
    });

    // Extract values we need for the MenuItem components
    let focused_key = menu.list.collection.selection_state.focused_key;
    let selected_keys = menu.list.collection.selection_state.selected_keys;
    let set_focused_key = menu.list.collection.selection_state.set_focused_key;
    let selection_mode = menu.selection_mode;

    // Store menu attrs and items in StoredValue so Portal's Fn children closure can access them.
    let menu_attrs = StoredValue::new(menu.menu_props.into_attrs());
    let menu_items = StoredValue::new(items.clone());

    view! {
        <div class="demo-menu-anchor">
            <button
                {..button_props}
                {..popover_trigger_attrs.get_value()}
                class="demo-btn-primary demo-menu-trigger"
                style=trigger_styles
            >
                "Actions"
                <span class=Classes::from("demo-disclosure-arrow")
                    .add_reactive("open", state.is_open)>"\u{25bc}"</span>
            </button>

            <Portal>
                <Show when=move || {
                    state.is_open.get()
                }>
                    {
                        let items = menu_items.get_value();
                        let menu_attrs = menu_attrs.get_value();
                        view! {
                            // Underlay captures outside clicks to dismiss
                            <div {..underlay_attrs.get_value()} class="demo-popover-underlay" />
                            // Popover container with overlay positioning
                            <div {..popover_attrs.get_value()} style=popover_styles.get_value()>
                                <FocusScope contain=true restore_focus=true>
                                    <ul {..menu_attrs} id=menu_id class="demo-menu-list">
                                        {items
                                            .into_iter()
                                            .map(|item| {
                                                view! {
                                                    <MenuItem
                                                        item=item
                                                        is_disabled=false
                                                        focused_key=focused_key
                                                        selected_keys=selected_keys
                                                        selection_mode=selection_mode
                                                        on_focus=Callback::new(move |key| {
                                                            set_focused_key.run((key, None));
                                                        })
                                                        on_action=Callback::new(move |key: String| {
                                                            set_selected.set(Some(key));
                                                        })
                                                        on_close=state.close
                                                    />
                                                }
                                            })
                                            .collect_view()}
                                    </ul>
                                </FocusScope>
                            </div>
                        }
                    }
                </Show>
            </Portal>
        </div>

        <p>
            "Selected: "
            <strong>{move || selected.get().unwrap_or_else(|| "None".to_string())}</strong>
        </p>
    }
}
