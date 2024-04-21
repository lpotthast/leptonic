use educe::Educe;
use leptos::{NodeRef, SignalGetUntracked};
use leptos_use::core::ElementMaybeSignal;
use typed_builder::TypedBuilder;
use web_sys::KeyboardEvent;

use crate::{
    state::{selection::SelectionManager, CollectionFocusStrategy, FocusStrategy, Key},
    utils::{
        attributes::Attributes, event_handlers::EventHandlers, locale::WritingDirection,
        EventTargetExt,
    },
};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/%40react-aria/selection/src/useSelectableCollection.ts

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LinkBehavior {
    Action,
    Selection,
    Override,
}

#[derive(Clone, Educe, TypedBuilder)]
#[educe(Debug)]
pub struct UseSelectableCollectionInput {
    // TODO keyboard delegate?
    /// A delegate object that implements behavior for keyboard focus movement.
    //keyboard_delegate: KeyboardDelegate,

    /// The ref attached to the element representing the collection.
    // el: NodeRef<leptos::html::AnyElement>,

    /// Whether the collection or one of its items should be automatically focused upon render. Defaults to None.
    #[builder(default = None)]
    auto_focus: Option<CollectionFocusStrategy>,

    /// Whether focus should wrap around when the end/start is reached. Defaults to false.
    #[builder(default = false)]
    should_focus_wrap: bool,

    /// Whether the collection allows empty selection. Defaults to true.
    #[builder(default = true)]
    allow_empty_selection: bool,

    /// Whether the collection allows the user to select all items via keyboard shortcut. Defaults to true.
    #[builder(default = true)]
    allow_select_all: bool,

    // TODO: Clarify meaning of `occur`!
    /// Whether selection should occur automatically on focus. Defaults to false.
    #[builder(default = false)]
    select_on_focus: bool, // TODO: react-aria default was: `selection_manager.selection_behavior == SelectionBehavior::Replace`

    /// Whether typeahead is allowed/enabled. Defaults to true.
    #[builder(default = true)]
    allow_type_ahead: bool,

    /// Whether the collection items should use virtual focus instead of being focused directly. Defaults to false.
    #[builder(default = false)]
    should_use_virtual_focus: bool,

    /// Whether navigation through tab key is enabled. Defaults to true.
    #[builder(default = true)]
    allow_tab_navigation: bool,

    /// Whether the collection items are contained in a virtual scroller.
    is_virtualized: bool,

    /// The ref attached to the scrollable body. Used to provide automatic scrolling on item focus for non-virtualized collections.
    /// If not provided, defaults to the collection ref.
    #[educe(Debug(ignore))]
    scroll_ref: NodeRef<leptos::html::AnyElement>,

    /// The behavior of links in the collection.
    /// - 'action': link behaves like onAction.
    /// - 'selection': link follows selection interactions (e.g. if URL drives selection).
    /// - 'override': links override all other interactions (link items are not selectable).
    /// Defaults to 'action'.
    link_behavior: LinkBehavior,
}

pub struct UseSelectableCollectionReturn {
    /// Props for the collection element.
    props: UseSelectableCollectionProps,
}

pub struct UseSelectableCollectionProps {
    attrs: Attributes,
    handlers: EventHandlers,
}

/// Attach this hook to a collection node.
/// Children under this node must register themselves in the context provided. TODO: Rephrase.
pub fn use_selectable_collection<El, T, N>(
    el: El,
    manager: SelectionManager<N>,
    input: UseSelectableCollectionInput,
) -> UseSelectableCollectionReturn
where
    El: Into<ElementMaybeSignal<T, web_sys::EventTarget>> + 'static,
    T: Into<web_sys::EventTarget> + Clone + 'static,
{
    let el: ElementMaybeSignal<T, web_sys::EventTarget> = el.into();

    // TODO: Use a hook? Get this through input.
    let direction = WritingDirection::Ltr;

    let on_key_down = Box::new(move |e: KeyboardEvent| {
        let key = e.key();

        // Prevent option + tab from doing anything since it doesn't move focus to the cells, only buttons/checkboxes
        if e.alt_key() && key.as_str() == "Tab" {
            e.prevent_default();
        }

        // Keyboard events bubble through portals. Don't handle keyboard events
        // for elements outside the collection (e.g. menus).
        if !el
            .get_untracked()
            .and_then(|el: T| {
                Some(
                    el.into()
                        .as_element()
                        .unwrap()
                        .contains(e.target().unwrap().as_node().as_ref()),
                )
            })
            .unwrap_or(false)
        {
            tracing::info!("KeyDown event ignored. Element which received the event was outside the collection.");
            return;
        }

        let navigate_to_key = |key: Key, child_focus: Option<FocusStrategy>| {
            // TODO: Handle links
            /*if (manager.isLink(key) && linkBehavior === 'selection' && selectOnFocus && !isNonContiguousSelectionModifier(e)) {
            // Set focused key and re-render synchronously to bring item into view if needed.
                flushSync(() => {
                    manager.setFocusedKey(key, childFocus);
                });

                let item = scrollRef.current.querySelector(`[data-key="${CSS.escape(key.toString())}"]`);
                let itemProps = manager.getItemProps(key);
                router.open(item, e, itemProps.href, itemProps.routerOptions);

                return;
            }*/

            manager.set_focused_key(key, child_focus);

            // TODO: Handle links
            /*if (manager.isLink(key) && linkBehavior === 'override') {
                return;
            }*/

            // TODO: implement
            /*if e.shift_key() && manager.selection_mode() == SelectionMode::Multiple {
                manager.extend_selection(key);
            } else if input.select_on_focus && !isNonContiguousSelectionModifier(e) {
                manager.replace_selection(key);
            }*/
        };

        // NOTES:
        // 1) Always `prevent_default` when we could handle the event by finding a key to navigate to.
        /*
        match key.as_str() {
            "ArrowDown" => {
                let mut next_key = match manager.focused_key() {
                    // TODO: Why the indirection?
                    Some(focused_key) => manager.collection.get_key_below(focused_key),
                    None => manager.collection.get_first_key(),
                };
                if let None = next_key {
                    if input.should_focus_wrap {
                        next_key = manager.collection.get_first_key() // TODO: TS code passed `manager.focused_key()`. Why? focused_key() returns an Option...
                    }
                }
                if let Some(next_key) = next_key {
                    e.prevent_default();
                    navigate_to_key(next_key, None); // TODO: ok to pass none as second arg?
                }
            }
            "ArrowUp" => {
                let mut next_key = match manager.focused_key() {
                    // TODO: Why the indirection?
                    Some(focused_key) => manager.collection.get_key_above(focused_key),
                    None => manager.collection.get_last_key(),
                };
                if let None = next_key {
                    if input.should_focus_wrap {
                        next_key = manager.collection.get_last_key() // TODO: TS code passed `manager.focused_key()`. Why? focused_key() returns an Option...
                    }
                }
                if let Some(next_key) = next_key {
                    e.prevent_default();
                    navigate_to_key(next_key, None); // TODO: ok to pass none as second arg?
                }
            }
            "ArrowLeft" => {
                let mut next_key = match manager.focused_key() {
                    // TODO: Why the indirection?
                    Some(focused_key) => manager.collection.get_key_left_of(focused_key),
                    None => manager.collection.get_first_key(), // TODO: Is `get_first_key` correct?
                };
                if let None = next_key {
                    if input.should_focus_wrap {
                        next_key = match direction {
                            WritingDirection::Ltr => manager.collection.get_first_key(), // TODO: TS code passed `manager.focused_key()`. Why? focused_key() returns an Option...
                            WritingDirection::Rtl => manager.collection.get_last_key(), // TODO: TS code passed `manager.focused_key()`. Why? focused_key() returns an Option...
                        }
                    }
                }
                if let Some(next_key) = next_key {
                    e.prevent_default();
                    navigate_to_key(
                        next_key,
                        Some(match direction {
                            WritingDirection::Ltr => FocusStrategy::Last,
                            WritingDirection::Rtl => FocusStrategy::First,
                        }),
                    );
                }
            }
            "ArrowRight" => {
                let mut next_key = match manager.focused_key() {
                    // TODO: Why the indirection?
                    Some(focused_key) => manager.collection.get_key_right_of(focused_key),
                    None => manager.collection.get_first_key(), // TODO: Is `get_first_key` correct?
                };
                if let None = next_key {
                    if input.should_focus_wrap {
                        next_key = match direction {
                            WritingDirection::Ltr => manager.collection.get_first_key(), // TODO: TS code passed `manager.focused_key()`. Why? focused_key() returns an Option...
                            WritingDirection::Rtl => manager.collection.get_last_key(), // TODO: TS code passed `manager.focused_key()`. Why? focused_key() returns an Option...
                        }
                    }
                }
                if let Some(next_key) = next_key {
                    e.prevent_default();
                    navigate_to_key(
                        next_key,
                        Some(match direction {
                            WritingDirection::Ltr => FocusStrategy::First,
                            WritingDirection::Rtl => FocusStrategy::Last,
                        }),
                    );
                }
            }
              "Home" => {
                  e.prevent_default();
                  let first_key = manager.collection.get_first_key(); // TODO: Provide args `manager.focusedKey, e.is_ctrl_key_pressed()`.
                  manager.set_focused_key(first_key, None); // TODO: Is None correct?
                  if e.is_ctrl_key_pressed() && e.shift_key() && manager.selection_mode() == SelectionMode::Multiple {
                    manager.extend_selection(first_key);
                  } else if input.select_on_focus {
                    manager.replace_selection(first_key);
                  }
              }
              "End" => {
                  e.prevent_default();
                  let last_key = manager.collection.get_first_key(); // TODO: Provide args `manager.focusedKey, e.is_ctrl_key_pressed()`.
                  manager.set_focused_key(last_key, None); // TODO: Is None correct?
                  if e.is_ctrl_key_pressed() && e.shift_key() && manager.selection_mode() == SelectionMode::Multiple {
                    manager.extend_selection(last_key);
                  } else if input.select_on_focus {
                    manager.replace_selection(last_key);
                  }
              }
              "PageDown" => {
                if (delegate.getKeyPageBelow) {
                  e.prevent_default();
                  let next_key = delegate.getKeyPageBelow(manager.focusedKey);
                  navigate_to_key(next_key);
                }
              }
              "PageUp" => {
                if (delegate.getKeyPageAbove) {
                  e.prevent_default();
                  let next_key = delegate.getKeyPageAbove(manager.focusedKey);
                  navigate_to_key(next_key);
                }
              }
              "a" => {
                if input.allow_select_all && e.is_ctrl_key_pressed() && manager.selection_mode() == SelectionMode::Multiple {
                  e.prevent_default();
                  manager.select_all();
                }
              }
              "Escape" => {
                if input.allow_empty_selection && manager.selected_keys().size != 0 {
                  e.stop_propagation();
                  e.prevent_default();
                  manager.clear_selection();
                }
              }
              "Tab" => {
                if !input.allow_tab_navigation {
                  // There may be elements that are "tabbable" inside a collection (e.g. in a grid cell).
                  // However, collections should be treated as a single tab stop, with arrow key navigation internally.
                  // We don't control the rendering of these, so we can't override the tabIndex to prevent tabbing.
                  // Instead, we handle the Tab key, and move focus manually to the first/last tabbable element
                  // in the collection, so that the browser default behavior will apply starting from that element
                  // rather than the currently focused one.
                  if e.shift_key() {
                    ref.current.focus();
                  } else {
                    let walker = getFocusableTreeWalker(ref.current, {tabbable: true});
                    let next: FocusableElement;
                    let last: FocusableElement;
                    do {
                      last = walker.lastChild() as FocusableElement;
                      if (last) {
                        next = last;
                      }
                    } while (last);

                    if (next && !next.contains(document.activeElement)) {
                      focusWithoutScrolling(next);
                    }
                  }
                }
              }
        } */
    });

    let attrs = Attributes::new();
    let handlers = EventHandlers::builder()
      //.on_key_down(on_key_down)
      .build();

    UseSelectableCollectionReturn {
        props: UseSelectableCollectionProps { attrs, handlers },
    }
}
