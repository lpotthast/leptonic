// Upstream: react-aria-components/src/Menu.tsx @ 99e6102368
use std::collections::HashSet;

use leptos::{context::Provider, prelude::*};

use super::{
    dialog::DialogTriggerContext,
    press::{PressResponder, PressResponderProps},
};
use crate::{
    hooks::{
        IntoAttrs, MenuData, MenuTriggerType, OverlayTriggerType, PressResponderTrigger,
        SelectionBehavior, SelectionMode, UseMenuInput, UseMenuItemInput, UseMenuItemReturn,
        UseMenuReturn, UseMenuSectionInput, UseMenuSectionReturn, UseMenuTriggerInput,
        UseMenuTriggerMenuProps, UseMenuTriggerReturn, UseMenuTriggerStateInput,
        collections::{
            AutoFocus, CollectionMemo, CollectionOptions, Key, ListState, Node, Selection,
            SelectionOptions, UseListStateInput, use_list_state,
        },
        use_menu, use_menu_item, use_menu_section, use_menu_trigger, use_menu_trigger_state,
    },
    utils::{
        CapturedElement, SlotProps, ValueBinding, classes::Classes, data_attributes::flag,
        scoped_context::scoped_view, styles::Styles,
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - The menu's items come from a collection (`collection` prop, as the `ListBox` atom); render one
//   `MenuItem`/`MenuSection` per entry (or `MenuItems`). Item content uses `MenuItemLabel`,
//   `MenuItemDescription` and `MenuItemShortcut` instead of `Text`/`Keyboard` slots.
// - `MenuTrigger`'s open state is hook-owned (`default_open` + `on_open_change`, or `state`
//   bound to app state).
// - The menu is labelled by its trigger's rendered id (its own `attr:id`, else a generated one).
//
// ## OMITTED FEATURES
// - Submenus (`SubmenuTrigger`), `Separator` items,
//   virtualized and render-prop item content, `onClose`/`shouldCloseOnSelect` per menu.
//
// =============================================================================

/// Context from a [`MenuTrigger`] to its [`Menu`].
#[derive(Debug, Clone, Copy)]
struct MenuTriggerContext {
    menu_props: UseMenuTriggerMenuProps,
    trigger: DialogTriggerContext,
}

/// Context from [`MenuItem`] to its label, description and shortcut.
#[derive(Debug, Clone)]
struct MenuItemCtx {
    label: StoredValue<Option<SlotProps>>,
    description: StoredValue<Option<SlotProps>>,
    shortcut: StoredValue<Option<SlotProps>>,
}

/// Opens the [`Menu`] in the [`Popover`](super::popover::Popover) inside it when its pressable child
/// (a `Button`) is pressed. The button gets `aria-haspopup="menu"`, `aria-expanded` and
/// `aria-controls`; Enter, Space and the arrow keys open the menu with its first (or last) item
/// focused.
///
/// ```ignore
/// <MenuTrigger>
///     <Button aria_label="Actions">"☰"</Button>
///     <Popover>
///         <Menu collection=actions on_action=move |key| log!("{key}")>
///             <MenuItems let:node>{node.text_value.to_string()}</MenuItems>
///         </Menu>
///     </Popover>
/// </MenuTrigger>
/// ```
#[component]
pub fn MenuTrigger(
    #[prop(into, optional)] is_disabled: Signal<bool>,
    /// How the button opens the menu: by a press (default), or by a long press (then a press
    /// performs the button's own action, and Alt+ArrowDown opens the menu by keyboard).
    #[prop(optional)]
    trigger: MenuTriggerType,
    /// Whether the menu starts open. Ignored when `state` is given.
    #[prop(optional)]
    default_open: bool,
    /// Called when the menu opens or closes.
    #[prop(into, optional)]
    on_open_change: Option<Callback<bool>>,
    /// The open state as app state (e.g. an `RwSignal<bool>`), replacing `default_open`.
    #[prop(into, optional)]
    state: Option<ValueBinding<bool>>,
    children: Children,
) -> impl IntoView {
    let menu_state = use_menu_trigger_state(UseMenuTriggerStateInput {
        default_open,
        value: state,
        on_open_change,
    });
    let UseMenuTriggerReturn { button, menu_props } = use_menu_trigger(UseMenuTriggerInput {
        menu_type: OverlayTriggerType::Menu,
        is_disabled,
        trigger,
        state: menu_state,
    });
    // The popover finds the state and the trigger element here; closing the overlay state closes
    // the menu (and its submenus).
    let overlay_trigger = DialogTriggerContext::new(menu_state.overlay, CapturedElement::new());
    let press_trigger = PressResponderTrigger {
        aria_haspopup: button.aria_haspopup,
        aria_expanded: button.aria_expanded,
        aria_controls: button.aria_controls,
        element: overlay_trigger.trigger,
    };
    // The button's press handlers, ARIA props and shortcuts (react-aria-components' `PressResponder`
    // with `menuTriggerProps`). Built from its props struct (`view!` can't pass `Option`s), inside
    // the contexts for the popover and the menu.
    let menu_context = MenuTriggerContext {
        menu_props,
        trigger: overlay_trigger,
    };
    scoped_view(
        move || {
            provide_context(overlay_trigger);
            provide_context(menu_context);
        },
        move || {
            PressResponder(PressResponderProps {
                on_press: button.on_press,
                on_press_start: button.on_press_start,
                on_press_end: None,
                on_press_up: None,
                on_press_change: None,
                on_long_press_start: button.on_long_press_start,
                on_long_press: button.on_long_press,
                on_long_press_end: button.on_long_press_end,
                long_press_accessibility_description: button.long_press_accessibility_description,
                is_disabled: Some(is_disabled),
                force_is_pressed: Some(menu_state.overlay.is_open),
                prevent_focus_on_press: Some(button.prevent_focus_on_press),
                should_cancel_on_pointer_exit: None,
                allow_text_selection_on_press: None,
                trigger: Some(press_trigger),
                shortcuts: button.shortcuts,
                children,
            })
        },
    )
}

/// A headless menu: a list of actions (or of options to check). Arrow keys and type-ahead move
/// focus; pressing an item (or Enter/Space) performs `on_action` and closes the menu of a
/// [`MenuTrigger`].
///
/// The items come from `collection`: render one [`MenuItem`] (or [`MenuSection`]) per collection
/// entry, in collection order, or [`MenuItems`]. Inside a `MenuTrigger`, the menu is labelled by
/// the trigger and focuses its first, last or selected item when it opens.
///
/// Data attributes of items: `data-focused`, `data-focus-visible`, `data-selected`,
/// `data-disabled`, `data-pressed`.
#[component]
#[allow(clippy::too_many_lines, clippy::implicit_hasher)]
pub fn Menu(
    /// The items.
    #[prop(into, optional)]
    collection: Option<CollectionMemo>,
    /// Use an existing list state instead of creating one from `collection` and the selection
    /// props.
    #[prop(optional)]
    state: Option<ListState>,
    /// `None` (default) for a menu of actions; `Single`/`Multiple` for checkable items.
    #[prop(into, optional)]
    selection_mode: Signal<SelectionMode>,
    /// The initially selected keys.
    #[prop(into, optional)]
    default_selected_keys: Vec<Key>,
    /// The selection as app state (e.g. an `RwSignal<Selection>`), replacing
    /// `default_selected_keys`.
    #[prop(into, optional)]
    selection: Option<ValueBinding<Selection>>,
    #[prop(into, optional)] on_selection_change: Option<Callback<Selection>>,
    #[prop(into, optional)] disabled_keys: Option<Signal<HashSet<Key>>>,
    #[prop(into, optional)] aria_label: MaybeProp<String>,
    /// The ids of the elements naming the menu. Default inside a `MenuTrigger`: the trigger.
    #[prop(into, optional)]
    aria_labelledby: MaybeProp<String>,
    /// Focus an item when the menu mounts. Default inside a `MenuTrigger`: as the trigger opened
    /// it (the first or last item by keyboard, else the menu).
    #[prop(optional)]
    auto_focus: Option<AutoFocus>,
    /// Called with the key of an activated item.
    #[prop(into, optional)]
    on_action: Option<Callback<Key>>,
    /// Called when an item asks the menu to close (after its action). Inside a `MenuTrigger` the
    /// menu closes as well.
    #[prop(into, optional)]
    on_close: Option<Callback<()>>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let trigger_ctx = use_context::<MenuTriggerContext>();
    let state = state.unwrap_or_else(|| {
        let collection = collection.unwrap_or_else(|| {
            crate::utils::dev_warn!("Menu: no `collection` given");
            Memo::new(|_| std::sync::Arc::default())
        });
        use_list_state(UseListStateInput {
            collection,
            selection: SelectionOptions {
                selection_mode,
                selection_behavior: SelectionBehavior::Toggle,
                default_selection: Selection::keys(default_selected_keys),
                selection,
                on_selection_change,
                disabled_keys: disabled_keys.unwrap_or_default(),
                ..SelectionOptions::default()
            },
        })
    });

    // Inside a trigger: labelled by the trigger's rendered id, ensured once the menu is rendered
    // (before `use_menu` checks that the menu has a name).
    let trigger_id = RwSignal::new(None::<String>);
    if let Some(ctx) = trigger_ctx {
        Effect::new(move |_| trigger_id.set(ctx.trigger.ensure_trigger_id()));
    }
    let labelledby = MaybeProp::derive(move || {
        aria_labelledby.get().or_else(|| {
            aria_label
                .read()
                .is_none()
                .then(|| trigger_id.get())
                .flatten()
        })
    });
    let auto_focus = match (auto_focus, trigger_ctx) {
        (Some(auto_focus), _) => Signal::stored(Some(auto_focus)),
        (None, Some(ctx)) => ctx.menu_props.auto_focus,
        (None, None) => Signal::stored(None),
    };
    let on_close = match (on_close, trigger_ctx) {
        (Some(own), Some(ctx)) => Some(Callback::new(move |()| {
            own.run(());
            ctx.menu_props.on_close.run(());
        })),
        (own, ctx) => own.or(ctx.map(|ctx| ctx.menu_props.on_close)),
    };

    let UseMenuReturn { props, data } = use_menu(UseMenuInput {
        id: trigger_ctx.map(|ctx| ctx.menu_props.id.get_untracked()),
        aria_label,
        aria_labelledby: labelledby,
        options: CollectionOptions {
            auto_focus,
            should_focus_wrap: true,
            ..CollectionOptions::default()
        },
        on_action,
        on_close,
        ..UseMenuInput::new(state, CapturedElement::new())
    });

    view! {
        <Provider value=data>
            <div {..props.into_attrs()} class=classes style=styles>
                {children()}
            </div>
        </Provider>
    }
}

/// An item of a [`Menu`], for the collection item `key`: `menuitem`, or `menuitemcheckbox` /
/// `menuitemradio` in a menu with selection.
#[component]
pub fn MenuItem(
    /// The item's key in the menu's collection.
    #[prop(into)]
    key: Key,
    /// Whether activating the item closes the menu. Default: unless the menu allows multiple
    /// selection, or the item was checked with Space.
    #[prop(optional)]
    should_close_on_select: Option<bool>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let menu = expect_context::<MenuData>();
    let UseMenuItemReturn {
        props,
        label_props,
        description_props,
        keyboard_shortcut_props,
        is_focused,
        is_focus_visible,
        is_selected,
        is_pressed,
        is_disabled,
    } = use_menu_item(UseMenuItemInput {
        menu,
        key,
        should_close_on_select,
    });
    let ctx = MenuItemCtx {
        label: StoredValue::new(Some(label_props)),
        description: StoredValue::new(Some(description_props)),
        shortcut: StoredValue::new(Some(keyboard_shortcut_props)),
    };
    let (attrs, item_styles) = props.into_parts();
    let styles = item_styles.merge(styles);

    view! {
        <Provider value=ctx>
            <div
                {..attrs}
                class=classes
                style=styles
                data-focused=flag(is_focused)
                data-focus-visible=flag(is_focus_visible)
                data-selected=flag(is_selected)
                data-disabled=flag(is_disabled)
                data-pressed=flag(is_pressed)
            >
                {children()}
            </div>
        </Provider>
    }
}

/// One [`MenuItem`] per item of the menu's collection, rendered by `children`.
///
/// ```ignore
/// <Menu collection=actions>
///     <MenuItems let:node>{node.text_value.to_string()}</MenuItems>
/// </Menu>
/// ```
#[component]
pub fn MenuItems<F, IV>(
    /// Renders an item's content.
    children: F,
    /// CSS classes of each item.
    #[prop(into, optional)]
    classes: Classes,
) -> impl IntoView
where
    F: Fn(Node) -> IV + Send + Sync + 'static,
    IV: IntoView + 'static,
{
    let menu = expect_context::<MenuData>();
    let collection = menu.state.collection;
    let children = std::sync::Arc::new(children);
    view! {
        <For
            each=move || collection.with(|c| c.items().cloned().collect::<Vec<_>>())
            key=|node| node.key.clone()
            let:node
        >
            {
                let children = children.clone();
                let key = node.key.clone();
                view! { <MenuItem key=key classes=classes.clone()>{children(node)}</MenuItem> }
            }
        </For>
    }
}

/// The main text of a [`MenuItem`] (labels the item).
#[component]
pub fn MenuItemLabel(
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let ctx = expect_context::<MenuItemCtx>();
    slot(ctx.label, "MenuItemLabel", classes, styles, children)
}

/// Secondary text of a [`MenuItem`] (describes the item).
#[component]
pub fn MenuItemDescription(
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let ctx = expect_context::<MenuItemCtx>();
    slot(
        ctx.description,
        "MenuItemDescription",
        classes,
        styles,
        children,
    )
}

/// The keyboard shortcut of a [`MenuItem`] (e.g. "Ctrl+C"), announced with the item.
#[component]
pub fn MenuItemShortcut(
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let ctx = expect_context::<MenuItemCtx>();
    slot(ctx.shortcut, "MenuItemShortcut", classes, styles, children)
}

/// Renders a label/description/shortcut slot. The slot's props go to the first such element only.
fn slot(
    props: StoredValue<Option<SlotProps>>,
    component: &str,
    classes: Classes,
    styles: Styles,
    children: Children,
) -> AnyView {
    if let Some(props) = props.try_update_value(Option::take).flatten() {
        view! {
            <span {..props.into_attrs()} class=classes style=styles>
                {children()}
            </span>
        }
        .into_any()
    } else {
        crate::utils::dev_warn!("{component}: only one per MenuItem is supported");
        view! {
            <span class=classes style=styles>
                {children()}
            </span>
        }
        .into_any()
    }
}

/// A group of items in a [`Menu`], for the collection section `key`. Renders the section's header
/// (if the collection has one) followed by the children.
#[component]
pub fn MenuSection(
    /// The section's key in the menu's collection.
    #[prop(into)]
    key: Key,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    #[prop(into, optional)] heading_classes: Classes,
    children: Children,
) -> impl IntoView {
    let menu = expect_context::<MenuData>();
    let UseMenuSectionReturn {
        item_props,
        heading_props,
        group_props,
        heading,
    } = use_menu_section(UseMenuSectionInput { menu, key });

    view! {
        <div {..item_props.into_attrs()}>
            {heading_props
                .map(|props| {
                    view! {
                        <div {..props.into_attrs()} class=heading_classes>
                            {heading}
                        </div>
                    }
                })}
            <div {..group_props.into_attrs()} class=classes style=styles>
                {children()}
            </div>
        </div>
    }
}
