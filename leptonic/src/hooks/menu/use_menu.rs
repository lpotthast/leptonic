// Upstream: react-aria/src/menu/useMenu.ts @ 740c6c5c4a
// Upstream: react-aria/src/menu/utils.ts @ 740c6c5c4a
// Upstream: react-aria/test/menu/useMenu.test.tsx @ 740c6c5c4a
// Upstream: react-aria-components/test/Menu.test.tsx @ 740c6c5c4a
// Upstream: @adobe/react-spectrum/test/menu/Menu.test.js @ 740c6c5c4a
use std::sync::Arc;

use leptos::{
    attr::{self, Attr},
    prelude::*,
};
use web_sys::KeyboardEvent;

use crate::{
    CapturedElement, EventHandler, IntoAttrs, Orientation,
    hooks::collections::{
        AutoFocus, CollectionOptions, FocusStrategy, Key, KeyboardDelegate, LinkBehavior,
        ListLayout, ListState, UseSelectableCollectionAttrs, UseSelectableCollectionProps,
        UseSelectableListInput, use_selectable_list,
    },
    utils::{aria::AriaRole, id::use_id},
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - The menu's items come from a list state (react-aria: a tree state, for submenus).
// - Items get the menu's settings through the returned `MenuData` (react-aria: a `WeakMap`
//   keyed by the state), which the caller hands to `use_menu_item`.
//
// - A submenu gets its id, label, closing and keyboard handling as `submenu` (from
//   `use_submenu_trigger`); its items come from its own list state.
// - `should_focus_wrap` is part of `options` (`CollectionOptions`, default `false`); react-aria's
//   `useMenu` defaults it to `true`.
//
// ## OMITTED FEATURES
// - Virtual focus.
//
// =============================================================================

/// Input of [`use_menu`].
#[derive(Clone)]
pub struct UseMenuInput {
    /// The items and their selection (`SelectionMode::None` for action menus).
    pub state: ListState,
    /// The menu element; the hook's props capture it.
    pub element: CapturedElement,
    /// The element id. Generated when `None`. A menu trigger provides one (`menu_props.id`).
    pub id: Option<String>,
    pub aria_label: MaybeProp<String>,
    /// The ids of the elements naming the menu. A menu trigger provides them
    /// (`menu_props.aria_labelledby`); a signal, as the trigger's id may change.
    pub aria_labelledby: Signal<Option<String>>,
    /// Keyboard and focus behavior. react-aria's `useMenu` lets the arrow keys wrap around by
    /// default: set `should_focus_wrap` (the `Menu` atom does).
    pub options: CollectionOptions,
    /// Replaces the list keyboard delegate.
    pub keyboard_delegate: Option<Signal<Arc<dyn KeyboardDelegate>>>,
    /// Called with the key of an activated item.
    pub on_action: Option<Callback<Key>>,
    /// Called when an item asks the menu to close (after its action).
    pub on_close: Option<Callback<()>>,
    /// Makes the menu a submenu (from `use_submenu_trigger`): it takes the submenu's id, label and
    /// focus on opening, closes the whole menu tree after an action, and returns to its trigger
    /// on the arrow key towards the parent menu and on Escape.
    pub submenu: Option<super::SubmenuProps>,
}

/// What items need to know about their menu. Pass it to `use_menu_item`.
#[derive(Debug, Clone)]
pub struct MenuData {
    pub state: ListState,
    /// See `UseSelectableItemInput::collection_id`.
    pub collection_id: String,
    pub on_action: Option<Callback<Key>>,
    pub on_close: Option<Callback<()>>,
    /// The level in the menu tree of the submenus this menu's items open: 0 in the root menu,
    /// one more than the menu's own in a submenu (for `use_submenu_trigger_state`).
    pub submenu_level: usize,
}

/// Return value of [`use_menu`].
#[derive(Debug)]
pub struct UseMenuReturn {
    pub props: UseMenuProps,
    pub data: MenuData,
}

/// Props for the menu element.
#[derive(Debug)]
pub struct UseMenuProps {
    pub id: String,
    pub role: AriaRole,
    pub aria_label: MaybeProp<String>,
    pub aria_labelledby: Signal<Option<String>>,
    /// Keyboard navigation, type-ahead and focus handling (`use_selectable_list`).
    pub collection: UseSelectableCollectionProps,
}

pub type UseMenuAttrs = (
    Attr<attr::Id, String>,
    Attr<attr::Role, AriaRole>,
    Attr<attr::AriaLabel, MaybeProp<String>>,
    Attr<attr::AriaLabelledby, Signal<Option<String>>>,
    UseSelectableCollectionAttrs,
);

impl IntoAttrs for UseMenuProps {
    type Attrs = UseMenuAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Id, self.id),
            Attr(attr::Role, self.role),
            Attr(attr::AriaLabel, self.aria_label),
            Attr(attr::AriaLabelledby, self.aria_labelledby),
            self.collection.into_attrs(),
        )
    }
}

/// A menu: a list of actions (or of options to check), navigated with arrow keys and
/// type-ahead. Render one `use_menu_item` per collection item, in collection order.
///
/// Escape is left to the surrounding overlay, which closes the menu.
pub fn use_menu(input: UseMenuInput) -> UseMenuReturn {
    let UseMenuInput {
        state,
        element,
        id,
        aria_label,
        aria_labelledby,
        mut options,
        keyboard_delegate,
        on_action,
        mut on_close,
        submenu,
    } = input;
    let (mut id, mut aria_labelledby) = (id, aria_labelledby);
    let mut submenu_keyboard = None;
    let submenu_level = submenu.as_ref().map_or(0, |submenu| submenu.level + 1);
    if let Some(submenu) = submenu {
        id = Some(submenu.id);
        aria_labelledby = Signal::stored(Some(submenu.aria_labelledby));
        // The menu's own `on_close`, then the whole tree closes.
        let close_all = submenu.on_close;
        let own = on_close;
        on_close = Some(Callback::new(move |()| {
            if let Some(own) = own {
                own.run(());
            }
            close_all.run(());
        }));
        let auto_focus = submenu.auto_focus;
        options.auto_focus = Signal::derive(move || {
            auto_focus.get().map(|strategy| match strategy {
                FocusStrategy::First => AutoFocus::First,
                FocusStrategy::Last => AutoFocus::Last,
            })
        });
        submenu_keyboard = submenu.keyboard;
    }

    // Checked once mounted: a label may arrive after creation (a menu trigger's id).
    #[cfg(debug_assertions)]
    Effect::new(move |_| {
        if aria_label.get_untracked().is_none() && aria_labelledby.get_untracked().is_none() {
            crate::utils::dev_warn!(
                "use_menu: an aria_label or aria_labelledby is required for accessibility"
            );
        }
    });

    // Link items open on press, never by moving focus.
    options.link_behavior = Signal::stored(LinkBehavior::Override);

    let mut collection = use_selectable_list(UseSelectableListInput {
        state,
        element,
        orientation: Orientation::Vertical.into(),
        layout: ListLayout::Stack,
        layout_delegate: None,
        keyboard_delegate,
        options,
    })
    .props;

    // Escape bubbles to the overlay (which closes the menu) instead of clearing the selection.
    let list_keydown = collection.on_keydown;
    collection.on_keydown = EventHandler::new(move |e: KeyboardEvent| {
        use crate::utils::key::{KeyboardEventKey, KeyboardKey};
        // A submenu handles the arrow key towards its parent and Escape first.
        if let Some(keyboard) = &submenu_keyboard {
            keyboard.on_keydown.call(e.clone());
        }
        if e.typed_key() != KeyboardKey::Escape {
            list_keydown.call(e);
        }
    });

    UseMenuReturn {
        data: MenuData {
            state,
            collection_id: collection.collection_id.clone(),
            on_action,
            on_close,
            submenu_level,
        },
        props: UseMenuProps {
            id: id.unwrap_or_else(|| use_id("menu")),
            role: AriaRole::Menu,
            aria_label,
            aria_labelledby,
            collection,
        },
    }
}
