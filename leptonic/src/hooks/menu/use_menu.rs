use std::rc::Rc;

use educe::Educe;
use leptos::{html, provide_context, Callback, NodeRef};
use typed_builder::TypedBuilder;

use crate::{
    hooks::selection::use_selectable_list::{
        use_selectable_list, UseSelectableListInput, UseSelectableListReturn,
    },
    state::{selection::SelectionManager, tree::TreeState, CollectionFocusStrategy, Key},
    utils::{aria::AriaAccessibleName, attributes::Attributes, event_handlers::EventHandlers},
};

// Based on: https://github.com/adobe/react-spectrum/blob/main/packages/%40react-aria/menu/src/useMenu.ts

#[derive(Debug, TypedBuilder)]
pub struct UseMenuInput {
    /// An accessible name for this menu.
    pub(crate) accessible_name: AriaAccessibleName,

    /// Whether the collection or one of its items should be automatically focused upon render. Defaults to None.
    #[builder(default = None)]
    pub(crate) auto_focus: Option<CollectionFocusStrategy>,

    /// Whether focus should wrap around when the end/start is reached / keyboard navigation is circular. Defaults to true.
    #[builder(default = true)]
    pub(crate) should_focus_wrap: bool,

    /// Handler that is called when an item is selected. // TODO: Is `selected` the right word? Name it `clicked` or `activated` instead?
    pub(crate) on_action: Callback<Key>,

    /// Handler that is called when the menu should close after selecting an item. Defaults to None.
    #[builder(default = None)]
    pub(crate) on_close: Option<Callback<()>>,
    // is_virtualized ...
    // keyboard_delegate ...
}

pub struct UseMenuReturn {
    pub attrs: Attributes,
    pub handlers: EventHandlers,
}

#[derive(Clone, Educe)]
#[educe(Debug)]
pub(crate) struct UseMenuContext {
    pub(crate) on_action: Callback<Key>,
    pub(crate) on_close: Option<Callback<()>>,

    #[educe(Debug(ignore))]
    pub(crate) selection_manager: Rc<SelectionManager<NodeRef<html::AnyElement>>>,
}

/// Provides the behavior and accessibility implementation for a menu component.
/// A menu displays a list of actions or options that a user can choose.
pub fn use_menu(
    input: UseMenuInput, // TODO: state, as returned by use_list_state() ???
    state: TreeState,
) -> UseMenuReturn {
    // TODO: Do we really need statically known disabled_keys?
    let UseSelectableListReturn {
        attrs: list_attrs,
        handlers: list_handlers,
    } = use_selectable_list(UseSelectableListInput {
        disabled_keys: vec![],
    });

    let selection_manager = todo!(); // TODO

    // TODO: We could also return this value and let the atom provide this using a `<Provider>`!
    provide_context(UseMenuContext {
        on_action: input.on_action,
        on_close: input.on_close,
        selection_manager,
    });

    UseMenuReturn {
        attrs: list_attrs,
        handlers: list_handlers,
    }
}
