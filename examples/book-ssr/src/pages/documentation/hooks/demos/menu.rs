use std::collections::HashSet;

use leptonic::{
    atoms::focus_scope::FocusScope,
    hooks::{PlacementX, PlacementY, *},
    prelude::AriaHasPopup,
    utils::locale::WritingDirection,
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
            style=move || {
                format!(
                    "padding: 0.75em 1em; cursor: {}; list-style: none; transition: background 0.15s; {}{}",
                    if is_disabled.get() { "not-allowed" } else { "pointer" },
                    if is_focused.get() { "background: #e8f4fc;" } else { "" },
                    if is_focus_visible.get() {
                        "outline: 2px solid #0066cc; outline-offset: -2px;"
                    } else {
                        ""
                    },
                )
            }
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

    // Set up the button (ARIA haspopup/expanded are set to defaults -- the merge
    // with menu_trigger discards them in favour of menu trigger's ARIA attributes).
    let button = use_button(UseButtonInput {
        disabled: false.into(),
        aria_haspopup: Signal::stored(AriaHasPopup::default()),
        aria_expanded: Signal::stored(None),
        use_press_input: UsePressInput {
            disabled: false.into(),
            force_prevent_default: false,
            force_propagation: false,
            allow_text_selection_on_press: false,
            should_cancel_on_pointer_exit: false,
            prevent_focus_on_press: false,
            force_is_pressed: None,
            on_press: Callback::new(|_| {}),
            on_press_up: None,
            on_press_start: None,
            on_press_end: None,
            on_press_change: None,
            on_double_press: None,
            on_long_press_start: None,
            on_long_press: None,
            on_long_press_end: None,
            long_press_threshold: None,
            long_press_accessibility_description: None,
        },
        use_hover_input: UseHoverInput {
            disabled: false.into(),
            on_hover_start: None,
            on_hover_end: None,
            on_hover_change: None,
        },
        use_focus_ring_input: UseFocusRingInput::default(),
    });

    // Set up the menu trigger with menu-specific behavior
    let menu_trigger = use_menu_trigger(UseMenuTriggerInput {
        menu_type: OverlayTriggerType::Menu,
        disabled: false.into(),
        trigger: MenuTriggerType::Press,
        state,
    });

    // Capture the menu id so `aria-controls` on the trigger points to the `<ul>`.
    let menu_id = menu_trigger.menu_props.id;

    // Extract button and menu trigger attrs/styles separately for spreading.
    let (button_props, button_styles) = button.props.into_parts();
    let (menu_trigger_props, menu_trigger_styles) = menu_trigger.props.into_parts();
    let trigger_styles = button_styles
        .merge(menu_trigger_styles)
        .add("padding", "0.75em 1.5em")
        .add("border-radius", "8px")
        .add("cursor", "pointer")
        .add("background", "var(--brand-color)")
        .add("color", "white")
        .add("border", "none")
        .add("font-size", "1em")
        .add("display", "flex")
        .add("align-items", "center")
        .add("gap", "0.5em");

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
        <div style="display: inline-block; margin: 1em 0;">
            <button
                {..button_props}
                {..menu_trigger_props}
                {..popover_trigger_attrs.get_value()}
                style=trigger_styles
            >
                "Actions"
                <span style=move || {
                    format!(
                        "display: inline-block; transition: transform 0.2s; {}",
                        if state.is_open.get() { "transform: rotate(180deg);" } else { "" },
                    )
                }>"\u{25bc}"</span>
            </button>

            <Portal>
                <Show when=move || state.is_open.get()>
                    {
                        let items = menu_items.get_value();
                        let menu_attrs = menu_attrs.get_value();
                        view! {
                            // Underlay captures outside clicks to dismiss
                            <div
                                {..underlay_attrs.get_value()}
                                style="position: fixed; inset: 0; z-index: 999;"
                            />
                            // Popover container with overlay positioning
                            <div
                                {..popover_attrs.get_value()}
                                style=popover_styles.get_value()
                            >
                                <FocusScope contain=true restore_focus=true>
                                    <ul
                                        {..menu_attrs}
                                        id=menu_id
                                        style="margin: 0; padding: 0.25em 0; min-width: 180px; background: white; border: 1px solid #ddd; border-radius: 8px; box-shadow: 0 4px 12px rgba(0,0,0,0.15);"
                                    >
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
                                                        on_focus=Callback::new(move |key| set_focused_key.run((key, None)))
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
