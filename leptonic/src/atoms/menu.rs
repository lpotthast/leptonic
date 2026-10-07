// Upstream: react-aria-components/src/Menu.tsx @ 99e6102368
use std::collections::HashSet;

use leptos::{context::Provider, prelude::*};

use super::{
    dialog::DialogTriggerContext,
    popover::{PopoverDefaults, SubmenuPopoverContext},
    press::{PressResponder, PressResponderProps},
    separator::SeparatorContext,
};
use crate::{
    Out,
    hooks::{
        ContextMenuEvent, IntoAttrs, MenuData, MenuTriggerState, MenuTriggerStateApi,
        MenuTriggerType, OverlayTriggerType, Placement, PressResponderTrigger, SelectionBehavior,
        SelectionMode, SeparatorElementType, SubmenuKind, SubmenuProps, SubmenuTriggerItem,
        UseMenuInput, UseMenuItemInput, UseMenuItemReturn, UseMenuReturn, UseMenuSectionInput,
        UseMenuSectionReturn, UseMenuTriggerInput, UseMenuTriggerMenuProps, UseMenuTriggerReturn,
        UseMenuTriggerStateInput, UseSubmenuTriggerInput, UseSubmenuTriggerReturn,
        UseSubmenuTriggerStateInput, close_context_menu_on_outside_right_click,
        collections::{
            AutoFocus, CloseOnSelect, CollectionMemo, CollectionOptions, FocusStrategy, Key,
            ListState, Node, Selection, SelectionOptions, UseListStateInput, use_list_state,
        },
        use_menu, use_menu_item, use_menu_section, use_menu_trigger, use_menu_trigger_state,
        use_submenu_trigger, use_submenu_trigger_state,
    },
    utils::{
        CapturedElement, SlotProps, ValueBinding, classes::Classes, data_attributes::flag,
        default_class::with_default_class, point::Point, scoped_context::scoped_view,
        styles::Styles,
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
// - `MenuTrigger`'s open state (C4): `default_open` + `on_open_change`, or `is_open` + `set_open`.
// - The menu is labelled by its trigger's rendered id (its own `attr:id`, else a generated one).
//
// ## DIFFERENT BEHAVIOR
// - An explicit `aria_label` names the menu instead of the trigger: no `aria-labelledby` then.
//   react-aria-components renders both, so `aria-labelledby` wins and the label is ignored.
//
// - `SubmenuTrigger` names its trigger item's `key` (the `MenuItem` child must have it), and
//   the submenu's `Menu` has its own collection (react-aria-components: the submenu's items are a
//   branch of the parent menu's collection). Reason: collections are built flat per menu.
//
// - A `MenuSection` with a `selection_mode` of its own keeps the menu's disabled keys
//   (react-aria-components' group selection drops them).
//
// ## OMITTED FEATURES
// - Virtualized and render-prop item content.
//
// =============================================================================

/// Context from a [`MenuTrigger`] to its [`Menu`].
#[derive(Debug, Clone)]
struct MenuTriggerContext {
    menu_props: UseMenuTriggerMenuProps,
    trigger: DialogTriggerContext,
}

/// The state of the menu tree's root: its trigger's, else the root menu's own
/// (react-aria-components' `RootMenuTriggerStateContext`).
#[derive(Debug, Clone, Copy)]
struct RootMenuState(MenuTriggerState);

/// The element of the menu around, for its submenu triggers.
#[derive(Debug, Clone, Copy)]
struct ParentMenuElement(CapturedElement);

/// From a [`SubmenuTrigger`] to its trigger [`MenuItem`]; `None` inside a menu (so its items don't
/// see an outer trigger).
#[derive(Debug, Clone)]
struct SubmenuItemContext(Option<(Key, SubmenuTriggerItem)>);

/// Whether activating an item closes the menu, as set by the [`Menu`] or [`MenuSection`] around
/// (react-aria-components' `MenuItemContext`); an item's own setting wins.
#[derive(Debug, Clone, Copy)]
struct MenuCloseOnSelect(CloseOnSelect);

/// From a [`SubmenuTrigger`] to the [`Menu`] it opens; `None` inside a menu.
#[derive(Debug, Clone)]
struct SubmenuMenuContext(Option<(SubmenuProps, CapturedElement)>);

/// Context from [`MenuItem`] to its label, description and shortcut, and its state for the item's
/// content (e.g. a check mark while selected: `use_context::<MenuItemCtx>()`).
#[derive(Debug, Clone)]
pub struct MenuItemCtx {
    label: StoredValue<Option<SlotProps>>,
    description: StoredValue<Option<SlotProps>>,
    shortcut: StoredValue<Option<SlotProps>>,
    pub is_selected: Signal<bool>,
    pub is_focused: Signal<bool>,
    pub is_focus_visible: Signal<bool>,
    pub is_disabled: Signal<bool>,
    pub is_pressed: Signal<bool>,
    /// The menu's selection mode (the item is a `menuitemcheckbox` or `menuitemradio` with one).
    pub selection_mode: Signal<SelectionMode>,
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
    /// Whether the menu starts open. Ignored with `is_open`.
    #[prop(optional)]
    default_open: bool,
    /// Called when the menu opens or closes.
    #[prop(into, optional)]
    on_open_change: Option<Callback<bool>>,
    /// Whether the menu is open (controlled): a value or any signal.
    #[prop(into, optional)]
    is_open: Option<Signal<bool>>,
    /// Receives the new state: an `RwSignal`, `WriteSignal`, closure, `Callback`, ...
    #[prop(into, optional)]
    set_open: Option<Out<bool>>,
    children: Children,
) -> impl IntoView {
    let (is_open, on_open_change) =
        ValueBinding::from_state_props(is_open, set_open, on_open_change);
    let menu_state = use_menu_trigger_state(UseMenuTriggerStateInput {
        default_open,
        value: is_open,
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
            provide_context(RootMenuState(menu_state));
            // A menu's popover starts at the trigger's start edge; a context menu's right at the
            // point it opened (react-aria-components).
            provide_context(Some(PopoverDefaults {
                placement: Placement::BottomStart,
                offset: if trigger == MenuTriggerType::ContextMenu {
                    0.0
                } else {
                    8.0
                },
            }));
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
                prevent_focus_on_press: Some(Signal::stored(button.prevent_focus_on_press)),
                should_cancel_on_pointer_exit: None,
                allow_text_selection_on_press: None,
                trigger: Some(press_trigger),
                shortcuts: button.shortcuts,
                on_context_menu: button.on_context_menu,
                children,
            })
        },
    )
}

/// What the items inside a [`ContextMenuTrigger`] call when a context menu is requested on
/// them: their key, the request, and their element (the menu is labelled by and placed at it).
#[derive(Clone, Copy)]
pub(crate) struct ContextMenuTargetContext {
    pub(crate) open: Callback<(Key, ContextMenuEvent)>,
}

impl ContextMenuTargetContext {
    /// The `on_context_menu` of the item `key` inside a `ContextMenuTrigger`, if any.
    pub(crate) fn for_item(key: &Key) -> Option<Callback<ContextMenuEvent>> {
        let context = use_context::<Self>()?;
        let key = key.clone();
        Some(Callback::new(move |e: ContextMenuEvent| {
            context.open.run((key.clone(), e));
        }))
    }
}

/// The item a [`ContextMenuTrigger`]'s menu was opened for.
#[derive(Debug, Clone, Copy)]
pub struct ContextMenuTarget(pub Signal<Option<Key>>);

/// The item the [`ContextMenuTrigger`] around opened its menu for (e.g. for the menu's
/// `on_action`); `None` outside of one.
pub fn use_context_menu_target() -> Signal<Option<Key>> {
    use_context::<ContextMenuTarget>().map_or_else(|| Signal::stored(None), |target| target.0)
}

/// Opens its menu where a context menu is requested on an item of the collection inside: a
/// `GridListItem`, `TableRow` or `ListBoxItem` (right click, Shift+F10, the context menu key, a
/// long press on iOS). Put the collection and a `Popover` with a `Menu` inside; the menu is
/// labelled by the item, opens at the point (at the item's center from the keyboard), and the
/// focus returns to the item when it closes. [`use_context_menu_target`] tells the item.
///
/// react-aria-components has no row-level context menus (an addition): a `MenuTrigger` with
/// `trigger=ContextMenu` opens at one trigger element.
#[component]
pub fn ContextMenuTrigger(
    #[prop(into, optional)] is_disabled: Signal<bool>,
    /// Called when the menu opens or closes.
    #[prop(into, optional)]
    on_open_change: Option<Callback<bool>>,
    children: Children,
) -> impl IntoView {
    let menu_state = use_menu_trigger_state(UseMenuTriggerStateInput {
        on_open_change,
        ..UseMenuTriggerStateInput::default()
    });
    close_context_menu_on_outside_right_click(menu_state);
    // The item the menu is for: its key, element (the popover's anchor) and id (the menu's name).
    let target_key = RwSignal::new(None::<Key>);
    let target_id = RwSignal::new(String::new());
    let target = CapturedElement::new();
    let open = Callback::new(move |(key, e): (Key, ContextMenuEvent)| {
        if is_disabled.get_untracked() {
            return;
        }
        let element: &web_sys::Element = &e.target;
        // The focus returns to the item when the menu closes.
        crate::utils::focus::focus_element(element, true);
        target.set(element.clone());
        target_id.set(element.id());
        target_key.set(Some(key));
        let rect = element.get_bounding_client_rect();
        menu_state.set_point(Some(Point {
            x: rect.x() + e.x,
            y: rect.y() + e.y,
        }));
        menu_state.open(None);
    });
    let overlay_trigger = DialogTriggerContext::new(menu_state.overlay, target);
    let menu_id = crate::utils::id::use_id("menu");
    let menu_context = MenuTriggerContext {
        menu_props: UseMenuTriggerMenuProps {
            id: menu_id,
            aria_labelledby: target_id.into(),
            auto_focus: Signal::derive(move || {
                Some(match menu_state.focus_strategy() {
                    Some(FocusStrategy::First) => AutoFocus::First,
                    Some(FocusStrategy::Last) => AutoFocus::Last,
                    None => AutoFocus::Selected,
                })
            }),
            on_close: Callback::new(move |()| menu_state.close()),
        },
        trigger: overlay_trigger,
    };
    scoped_view(
        move || {
            provide_context(overlay_trigger);
            provide_context(menu_context);
            provide_context(RootMenuState(menu_state));
            provide_context(ContextMenuTargetContext { open });
            provide_context(ContextMenuTarget(target_key.into()));
            // At the point, below and after it (react-aria-components' context menus).
            provide_context(Some(PopoverDefaults {
                placement: Placement::BottomStart,
                offset: 0.0,
            }));
        },
        children,
    )
}

/// A headless menu: a list of actions (or of options to check). Arrow keys and type-ahead move
/// focus; pressing an item (or Enter/Space) performs `on_action` and closes the menu of a
/// [`MenuTrigger`].
///
/// The items come from `collection`: render one [`MenuItem`] (or [`MenuSection`]) per collection
/// entry, in collection order, or [`MenuItems`]. A [`Separator`](super::separator::Separator)
/// between them renders as `role="separator"`. Inside a `MenuTrigger`, the menu is labelled by
/// the trigger and focuses its first, last or selected item when it opens.
///
/// Data attributes: `data-empty` on the menu; on items `data-focused`, `data-focus-visible`,
/// `data-selected`, `data-disabled`, `data-pressed`, `data-selection-mode` (`single`/`multiple`).
///
/// Default class: `leptonic-Menu`.
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
    /// The selection (controlled), replacing `default_selected_keys`: a value or any signal.
    #[prop(into, optional)]
    selection: Option<Signal<Selection>>,
    /// Receives the new state: an `RwSignal`, `WriteSignal`, closure, `Callback`, ...
    #[prop(into, optional)]
    set_selection: Option<Out<Selection>>,
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
    /// Whether activating an item closes the menu. Default: unless the menu allows multiple
    /// selection, or the item was checked with Space. Sections and items can override it.
    #[prop(into, optional)]
    should_close_on_select: CloseOnSelect,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let classes = with_default_class("leptonic-Menu", classes);
    let (selection, on_selection_change) =
        ValueBinding::from_state_props(selection, set_selection, on_selection_change);
    // A submenu takes its settings from its `SubmenuTrigger`, not from the root trigger.
    let submenu = use_context::<SubmenuMenuContext>().and_then(|ctx| ctx.0);
    let trigger_ctx = if submenu.is_some() {
        None
    } else {
        use_context::<MenuTriggerContext>()
    };
    // The root of a menu tree without trigger keeps the state of its submenus itself.
    let root = use_context::<RootMenuState>().unwrap_or_else(|| {
        RootMenuState(use_menu_trigger_state(UseMenuTriggerStateInput::default()))
    });
    let (submenu, element) = match submenu {
        Some((props, element)) => (Some(props), element),
        None => (None, CapturedElement::new()),
    };
    let state = state.unwrap_or_else(|| {
        let collection = collection.unwrap_or_else(|| {
            crate::utils::dev_warn!("Menu: no `collection` given");
            Memo::new(|_| std::sync::Arc::default())
        });
        use_list_state(UseListStateInput {
            collection,
            selection: SelectionOptions {
                selection_mode,
                selection_behavior: Signal::stored(SelectionBehavior::Toggle),
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
    if let Some(trigger) = trigger_ctx.as_ref().map(|ctx| ctx.trigger) {
        Effect::new(move |_| trigger_id.set(trigger.ensure_trigger_id()));
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
    let auto_focus = match (auto_focus, trigger_ctx.as_ref()) {
        (Some(auto_focus), _) => Signal::stored(Some(auto_focus)),
        (None, Some(ctx)) => ctx.menu_props.auto_focus,
        (None, None) => Signal::stored(None),
    };
    let on_close = match (
        on_close,
        trigger_ctx.as_ref().map(|ctx| ctx.menu_props.on_close),
    ) {
        (Some(own), Some(trigger_close)) => Some(Callback::new(move |()| {
            own.run(());
            trigger_close.run(());
        })),
        (own, trigger_close) => own.or(trigger_close),
    };

    let collection = state.collection;
    let is_empty = Signal::derive(move || collection.with(|c| c.items().next().is_none()));
    let UseMenuReturn { props, data } = use_menu(UseMenuInput {
        id: trigger_ctx.as_ref().map(|ctx| ctx.menu_props.id.clone()),
        aria_label,
        aria_labelledby: labelledby,
        options: CollectionOptions {
            auto_focus,
            should_focus_wrap: true,
            ..CollectionOptions::default()
        },
        on_action,
        on_close,
        submenu,
        state,
        element,
        keyboard_delegate: None,
    });
    view! {
        <Provider value=data>
            <Provider value=root>
            <Provider value=ParentMenuElement(element)>
            // The items of this menu don't belong to an outer submenu trigger.
            <Provider value=SubmenuItemContext(None)>
            <Provider value=SubmenuMenuContext(None)>
            <Provider value=MenuCloseOnSelect(should_close_on_select)>
            // Separators between the items are `<div role="separator">`s.
            <Provider value=SeparatorContext {
                element_type: SeparatorElementType::Div,
            }>
                <div {..props.into_attrs()} class=classes style=styles data-empty=flag(is_empty)>
                    {children()}
                </div>
            </Provider>
            </Provider>
            </Provider>
            </Provider>
            </Provider>
            </Provider>
        </Provider>
    }
}

/// An item of a [`Menu`], for the collection item `key`: `menuitem`, or `menuitemcheckbox` /
/// `menuitemradio` in a menu with selection.
///
/// Default class: `leptonic-MenuItem`.
#[component]
pub fn MenuItem(
    /// The item's key in the menu's collection.
    #[prop(into)]
    key: Key,
    /// Whether activating the item closes the menu. Default: as its `MenuSection` or `Menu` says,
    /// else unless the menu allows multiple selection, or the item was checked with Space.
    #[prop(into, optional)]
    should_close_on_select: CloseOnSelect,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let classes = with_default_class("leptonic-MenuItem", classes);
    let menu = expect_context::<MenuData>();
    // The trigger item of a `SubmenuTrigger`.
    let submenu_trigger = use_context::<SubmenuItemContext>()
        .and_then(|ctx| ctx.0)
        .map(|(trigger_key, trigger)| {
            if trigger_key != key {
                crate::utils::dev_warn!(
                    "MenuItem: the trigger item of a SubmenuTrigger needs the trigger's key \
                     {trigger_key:?}, has {key:?}"
                );
            }
            trigger
        });
    let has_submenu = submenu_trigger.is_some();
    let should_close_on_select = match should_close_on_select {
        CloseOnSelect::Auto => {
            use_context::<MenuCloseOnSelect>().map_or(CloseOnSelect::Auto, |inherited| inherited.0)
        }
        own => own,
    };
    let selection = menu.state.selection;
    let selection_mode = Signal::derive(move || selection.selection_mode());
    let is_open = submenu_trigger
        .as_ref()
        .map_or_else(|| Signal::stored(false), |trigger| trigger.is_open);
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
        submenu_trigger,
    });
    let ctx = MenuItemCtx {
        label: StoredValue::new(Some(label_props)),
        description: StoredValue::new(Some(description_props)),
        shortcut: StoredValue::new(Some(keyboard_shortcut_props)),
        is_selected,
        is_focused,
        is_focus_visible,
        is_disabled,
        is_pressed,
        selection_mode,
    };
    let data_selection_mode = move || match selection_mode.get() {
        SelectionMode::None => None,
        SelectionMode::Single => Some("single"),
        SelectionMode::Multiple => Some("multiple"),
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
                data-has-submenu=has_submenu.then_some("true")
                data-open=flag(is_open)
                data-selection-mode=data_selection_mode
            >
                {children()}
            </div>
        </Provider>
    }
}

/// Opens a submenu from an item of a [`Menu`]: its children are the trigger [`MenuItem`] (with
/// this `key`) and a [`Popover`](super::popover::Popover) with the submenu's [`Menu`]. The submenu
/// opens when the item is pressed, hovered (after `delay`), or on ArrowRight (ArrowLeft in
/// right-to-left text); ArrowLeft, Escape and focusing another item of the menu close it.
///
/// ```ignore
/// <Menu collection=actions>
///     <MenuItem key="copy">"Copy"</MenuItem>
///     <SubmenuTrigger key="share">
///         <MenuItem key="share">"Share"</MenuItem>
///         <Popover>
///             <Menu collection=targets>
///                 <MenuItems let:node>{node.text_value.to_string()}</MenuItems>
///             </Menu>
///         </Popover>
///     </SubmenuTrigger>
/// </Menu>
/// ```
///
/// The trigger item gets `data-has-submenu` and, while its submenu is open, `data-open`.
#[component]
pub fn SubmenuTrigger(
    /// The key of the trigger item (the `MenuItem` child) in the menu's collection.
    #[prop(into)]
    key: Key,
    /// How long hovering the trigger item takes to open the submenu. Default: 200 ms.
    #[prop(optional)]
    delay: Option<std::time::Duration>,
    /// What the trigger opens. Default: a menu.
    #[prop(optional)]
    kind: SubmenuKind,
    children: Children,
) -> impl IntoView {
    let root = expect_context::<RootMenuState>().0;
    let parent_menu = expect_context::<ParentMenuElement>().0;
    let menu = expect_context::<MenuData>();
    let selection = menu.state.selection;
    let is_disabled = {
        let key = key.clone();
        Signal::derive(move || selection.is_disabled(&key))
    };
    let state = use_submenu_trigger_state(UseSubmenuTriggerStateInput {
        trigger_key: key.clone(),
        root,
    });
    let (trigger, submenu) = (CapturedElement::new(), CapturedElement::new());
    let UseSubmenuTriggerReturn {
        trigger: item,
        submenu: submenu_props,
        should_close_on_interact_outside,
    } = use_submenu_trigger(UseSubmenuTriggerInput {
        kind,
        is_disabled,
        delay: delay.unwrap_or(std::time::Duration::from_millis(200)),
        state,
        trigger,
        parent_menu,
        submenu,
    });
    let popover = SubmenuPopoverContext {
        should_close_on_interact_outside: StoredValue::new(should_close_on_interact_outside),
        aria_labelledby: StoredValue::new(submenu_props.aria_labelledby.clone()),
    };
    scoped_view(
        move || {
            provide_context(SubmenuItemContext(Some((key, item))));
            provide_context(SubmenuMenuContext(Some((submenu_props, submenu))));
            // The popover finds the submenu's state and the trigger item here.
            provide_context(DialogTriggerContext::new(state.overlay, trigger));
            provide_context(Some(popover));
        },
        children,
    )
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
///
/// Default class: `leptonic-MenuItemLabel`.
#[component]
pub fn MenuItemLabel(
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let classes = with_default_class("leptonic-MenuItemLabel", classes);
    let ctx = expect_context::<MenuItemCtx>();
    slot(ctx.label, "MenuItemLabel", classes, styles, children)
}

/// Secondary text of a [`MenuItem`] (describes the item).
///
/// Default class: `leptonic-MenuItemDescription`.
#[component]
pub fn MenuItemDescription(
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let classes = with_default_class("leptonic-MenuItemDescription", classes);
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
///
/// Default class: `leptonic-MenuItemShortcut`.
#[component]
pub fn MenuItemShortcut(
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let classes = with_default_class("leptonic-MenuItemShortcut", classes);
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

/// A group of items in a [`Menu`], for the collection section `key`: a `<section role="group">`
/// starting with the section's header (if the collection has one), as a `<header>` naming the
/// group, followed by the children.
///
/// With a `selection_mode` of its own, the section's items have a selection of their own
/// (react-aria-components' `MenuSection` selection props): e.g. a single choice next to a multiple
/// choice in one menu. `should_close_on_select` applies to the section's items.
///
/// Default classes: `leptonic-MenuSection`, the header `leptonic-MenuSectionHeading`.
#[component]
#[allow(clippy::needless_pass_by_value, clippy::implicit_hasher)]
pub fn MenuSection(
    /// The section's key in the menu's collection.
    #[prop(into)]
    key: Key,
    /// A selection mode of the section's own; its items then have their own selection. Default:
    /// the menu's selection.
    #[prop(into, optional)]
    selection_mode: Option<SelectionMode>,
    /// The initially selected keys of the section's own selection.
    #[prop(into, optional)]
    default_selected_keys: Vec<Key>,
    /// The section's own selection (controlled), replacing `default_selected_keys`: a value or
    /// any signal.
    #[prop(into, optional)]
    selection: Option<Signal<Selection>>,
    /// Receives the section's new selection: an `RwSignal`, `WriteSignal`, closure, `Callback`, ...
    #[prop(into, optional)]
    set_selection: Option<Out<Selection>>,
    /// Called with the section's new selection.
    #[prop(into, optional)]
    on_selection_change: Option<Callback<Selection>>,
    /// Whether the section's own selection can't become empty.
    #[prop(into, optional)]
    disallow_empty_selection: Signal<bool>,
    /// Whether activating one of the section's items closes the menu. Default: as the menu says.
    #[prop(into, optional)]
    should_close_on_select: CloseOnSelect,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    #[prop(into, optional)] heading_classes: Classes,
    children: Children,
) -> impl IntoView {
    let classes = with_default_class("leptonic-MenuSection", classes);
    let heading_classes = with_default_class("leptonic-MenuSectionHeading", heading_classes);
    let menu = expect_context::<MenuData>();
    let UseMenuSectionReturn {
        heading_props,
        group_props,
        heading,
        ..
    } = use_menu_section(UseMenuSectionInput {
        menu: menu.clone(),
        key,
    });
    // The section's own selection, focus shared with the menu.
    let section_menu = selection_mode.map(|mode| {
        let (selection, on_selection_change) =
            ValueBinding::from_state_props(selection, set_selection, on_selection_change);
        let mut section_menu = menu.clone();
        section_menu.state.selection = menu.state.selection.with_own_selection(SelectionOptions {
            selection_mode: Signal::stored(mode),
            selection_behavior: Signal::stored(SelectionBehavior::Toggle),
            default_selection: Selection::keys(default_selected_keys),
            selection,
            on_selection_change,
            disallow_empty_selection,
            ..SelectionOptions::default()
        });
        section_menu
    });
    let close_on_select = match should_close_on_select {
        CloseOnSelect::Auto => {
            use_context::<MenuCloseOnSelect>().map_or(CloseOnSelect::Auto, |inherited| inherited.0)
        }
        own => own,
    };
    let heading = heading_props.map(|props| {
        view! {
            <header {..props.into_attrs()} class=heading_classes>
                {heading}
            </header>
        }
    });

    view! {
        <section {..group_props.into_attrs()} class=classes style=styles>
            {heading}
            <Provider value=MenuCloseOnSelect(close_on_select)>
                {match section_menu {
                    Some(section_menu) => {
                        view! { <Provider value=section_menu>{children()}</Provider> }.into_any()
                    }
                    None => children().into_any(),
                }}
            </Provider>
        </section>
    }
}
