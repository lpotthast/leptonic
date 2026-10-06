// Upstream: react-aria/src/selection/useSelectableCollection.ts @ 99e6102368
// Upstream: react-aria/src/selection/useSelectableList.ts @ 99e6102368
use std::sync::Arc;

use leptos::{
    attr,
    attr::{
        Attr,
        custom::{CustomAttr, custom_attribute},
    },
    ev,
    ev::{On, SharedEventCallback},
    prelude::*,
};
use wasm_bindgen::JsCast;
use web_sys::{Event, FocusEvent, KeyboardEvent, MouseEvent};

use super::{
    FocusStrategy, Key, KeyboardDelegate, LinkBehavior, NavigationOptions, SelectionBehavior,
    SelectionManager, SelectionMode,
    modifiers::{is_ctrl_key_pressed, is_non_contiguous_selection_modifier_keyboard},
    use_type_select::{UseTypeSelectInput, UseTypeSelectProps, use_type_select},
};
use crate::{
    hooks::{
        IntoAttrs,
        focus::use_focus_visible::{Modality, get_modality},
        interactions::use_keyboard::{UseKeyboardInput, UseKeyboardReturn, use_keyboard},
    },
    utils::{
        CapturedElement, ElementCaptureAttr, EventAccessors, EventHandler,
        focus::{focus_element, focus_safely},
        focusability::is_tabbable,
        focusable_tree_walker::{FocusableTreeWalkerOptions, get_focusable_tree_walker},
        i18n::use_direction,
        keyboard_shortcut::{KeyboardShortcuts, Shortcut, ShortcutOutcome},
        locale::WritingDirection,
        modifiers::EventModifiers,
        node_contains,
        platform::device,
        scroll::{
            ScrollIntoViewOpts, ScrollIntoViewportOpts, scroll_into_view, scroll_into_viewport,
        },
        shadow_dom::get_active_element,
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - `auto_focus` is `Option<FocusStrategy>` (react-aria: `boolean | 'first' | 'last'`).
// - `select_on_focus: Option<bool>`: `None` selects on focus exactly when the selection behavior
//   is `Replace` (react-aria's default).
// - Item elements are looked up in the `ItemElements` registry instead of `[data-key]` queries.
//
// ## OMITTED FEATURES
// - The virtual focus events `react-aria-focus` / `react-aria-clear-focus` (sent by react-aria's
//   Autocomplete to the collection it controls): added with an autocomplete.
// - `scrollRef` (a scroll container different from the collection element) and
//   `UNSTABLE_focusOnEntry`.
//
// =============================================================================

/// What Escape does in a collection.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum EscapeKeyBehavior {
    /// Clear the selection (unless empty selections are disallowed).
    #[default]
    ClearSelection,
    /// Nothing; the key press bubbles (e.g. to close a popover).
    None,
}

/// Which item gets focus when a collection mounts with [`CollectionOptions::auto_focus`]. A
/// selected item always takes precedence.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AutoFocus {
    /// The (first) selected item, or else the collection element itself.
    Selected,
    /// The first item.
    First,
    /// The last item.
    Last,
}

/// How a collection behaves; shared by the collection hooks (`use_selectable_collection`,
/// `use_selectable_list`, `use_listbox`, ...).
// Independent on/off switches of one configuration, as in react-aria.
#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Clone, Copy, Default)]
pub struct CollectionOptions {
    /// Move focus into the collection when it mounts (read once, at mount).
    pub auto_focus: Signal<Option<AutoFocus>>,
    /// Arrow keys wrap around at the ends.
    pub should_focus_wrap: bool,
    /// Keep Escape from clearing the selection, in addition to the selection manager's
    /// `disallow_empty_selection`.
    pub disallow_empty_selection: bool,
    /// Disable Ctrl/Cmd+A.
    pub disallow_select_all: bool,
    pub escape_key_behavior: EscapeKeyBehavior,
    /// Select items as keyboard focus moves to them. `None`: when the selection behavior is
    /// `Replace`.
    pub select_on_focus: Option<bool>,
    pub disallow_type_ahead: bool,
    /// Let Tab move between focusable elements inside items, instead of leaving the collection.
    pub allows_tab_navigation: bool,
    /// DOM focus stays elsewhere (e.g. in a combo box input); items are focused virtually,
    /// through `aria-activedescendant`.
    pub should_use_virtual_focus: bool,
    /// How link items behave (see `use_selectable_item`).
    pub link_behavior: LinkBehavior,
}

/// Input of [`use_selectable_collection`].
#[derive(Clone)]
pub struct UseSelectableCollectionInput {
    pub selection: SelectionManager,
    /// Item elements, registered by the item hooks.
    pub item_elements: super::ItemElements,
    /// Keyboard navigation. A signal, as delegates depend on e.g. the locale (reading direction,
    /// collation).
    pub delegate: Signal<Arc<dyn KeyboardDelegate>>,
    /// The collection element; the hook's props capture it.
    pub element: CapturedElement,
    pub options: CollectionOptions,
}

/// Return value of [`use_selectable_collection`].
#[derive(Debug)]
pub struct UseSelectableCollectionReturn {
    pub props: UseSelectableCollectionProps,
}

/// Props for the collection element.
#[derive(Debug)]
pub struct UseSelectableCollectionProps {
    /// `0` while no item is focused (so Tab reaches the collection), else `-1` (the focused item
    /// is the tab stop).
    pub tabindex: Signal<Option<i32>>,
    /// Marks the collection, so items can tell their own children from nested collections.
    pub collection_id: String,
    pub element_capture: ElementCaptureAttr,
    pub on_keydown_capture: EventHandler<KeyboardEvent>,
    pub on_keydown: EventHandler<KeyboardEvent>,
    pub on_keyup: EventHandler<KeyboardEvent>,
    pub on_focusin: EventHandler<FocusEvent>,
    pub on_focusout: EventHandler<FocusEvent>,
    pub on_mousedown: EventHandler<MouseEvent>,
    pub on_scroll: EventHandler<Event>,
}

pub type UseSelectableCollectionAttrs = (
    Attr<attr::Tabindex, Signal<Option<i32>>>,
    CustomAttr<&'static str, String>,
    ElementCaptureAttr,
    On<ev::Capture<ev::keydown>, SharedEventCallback<KeyboardEvent>>,
    On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
    On<ev::keyup, SharedEventCallback<KeyboardEvent>>,
    On<ev::focusin, SharedEventCallback<FocusEvent>>,
    On<ev::focusout, SharedEventCallback<FocusEvent>>,
    On<ev::mousedown, SharedEventCallback<MouseEvent>>,
    On<ev::scroll, SharedEventCallback<Event>>,
);

impl IntoAttrs for UseSelectableCollectionProps {
    type Attrs = UseSelectableCollectionAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Tabindex, self.tabindex),
            custom_attribute("data-collection", self.collection_id),
            self.element_capture,
            self.on_keydown_capture.into_on(ev::capture(ev::keydown)),
            self.on_keydown.into_on(ev::keydown),
            self.on_keyup.into_on(ev::keyup),
            // React's `onFocus`/`onBlur` see focus moving onto items; native `focus`/`blur`
            // don't bubble.
            self.on_focusin.into_on(ev::focusin),
            self.on_focusout.into_on(ev::focusout),
            self.on_mousedown.into_on(ev::mousedown),
            self.on_scroll.into_on(ev::scroll),
        )
    }
}

/// Shortcuts for `key` alone and with the selection modifiers: Shift (extend), Ctrl/Option
/// (move focus without selecting), and both.
fn with_selection_modifiers(key: &'static str) -> [Shortcut; 4] {
    let base = Shortcut::key(key);
    let modifier = |s: Shortcut| if device::is_mac() { s.alt() } else { s.ctrl() };
    [
        base.clone(),
        base.clone().shift(),
        modifier(base.clone()),
        modifier(base.shift()),
    ]
}

/// Keyboard navigation, selection and focus management for a collection element: arrow keys,
/// Home/End, PageUp/PageDown, type-ahead, Ctrl/Cmd+A, Escape, and moving focus into and out of
/// the collection as a single tab stop.
#[allow(clippy::too_many_lines)]
pub fn use_selectable_collection(
    input: UseSelectableCollectionInput,
) -> UseSelectableCollectionReturn {
    crate::hooks::track_interaction_modality();
    let UseSelectableCollectionInput {
        selection,
        item_elements,
        delegate: delegate_signal,
        element,
        options:
            CollectionOptions {
                auto_focus,
                should_focus_wrap,
                disallow_empty_selection,
                disallow_select_all,
                escape_key_behavior,
                select_on_focus,
                disallow_type_ahead,
                allows_tab_navigation,
                should_use_virtual_focus,
                link_behavior,
            },
    } = input;
    let direction = use_direction();

    let collection_id = crate::utils::id::use_id("collection");
    let delegate = move || delegate_signal.get_untracked();
    let select_on_focus = move || {
        select_on_focus.unwrap_or_else(|| {
            untrack(|| selection.selection_behavior()) == SelectionBehavior::Replace
        })
    };
    let focused = move || untrack(|| selection.focused_key());

    // Move focus to `key` and update the selection as keyboard navigation does. Returns whether
    // the key press was handled.
    let navigate_to_key =
        move |e: &KeyboardEvent, key: Option<Key>, child: Option<FocusStrategy>| {
            let Some(key) = key else {
                return false;
            };
            let is_link = untrack(|| selection.is_link(&key));
            // Selecting a link item opens it: moving focus onto it navigates.
            if is_link
                && link_behavior == LinkBehavior::Selection
                && select_on_focus()
                && !is_non_contiguous_selection_modifier_keyboard(e)
            {
                selection.set_focused_key(Some(key.clone()), child);
                let link = untrack(|| {
                    selection
                        .collection()
                        .with(|c| c.get(&key).and_then(|n| n.link.clone()))
                });
                if let (Some(item), Some(link)) = (item_elements.get(&key), link) {
                    link.open(&item, e.modifiers());
                    return true;
                }
                return false;
            }
            selection.set_focused_key(Some(key.clone()), child);
            if is_link && link_behavior == LinkBehavior::Override {
                return false;
            }
            if e.shift_key() && untrack(|| selection.selection_mode()) == SelectionMode::Multiple {
                selection.extend_selection(&key);
            } else if select_on_focus() && !is_non_contiguous_selection_modifier_keyboard(e) {
                selection.replace_selection(&key);
            }
            true
        };

    let nav = NavigationOptions::default();
    let arrow = {
        move |e: &KeyboardEvent, key_name: &str| {
            let rtl = direction.get_untracked() == WritingDirection::Rtl;
            let (next, wrap, child) = match key_name {
                "ArrowDown" => (
                    focused().map_or_else(
                        || delegate().first_key(None, false),
                        |k| delegate().key_below(&k, nav),
                    ),
                    delegate().first_key(focused().as_ref(), false),
                    None,
                ),
                "ArrowUp" => (
                    focused().map_or_else(
                        || delegate().last_key(None, false),
                        |k| delegate().key_above(&k, nav),
                    ),
                    delegate().last_key(focused().as_ref(), false),
                    None,
                ),
                "ArrowLeft" => (
                    focused().map_or_else(
                        || delegate().first_key(None, false),
                        |k| delegate().key_left_of(&k, nav),
                    ),
                    if rtl {
                        delegate().first_key(focused().as_ref(), false)
                    } else {
                        delegate().last_key(focused().as_ref(), false)
                    },
                    Some(if rtl {
                        FocusStrategy::First
                    } else {
                        FocusStrategy::Last
                    }),
                ),
                _ => (
                    focused().map_or_else(
                        || delegate().first_key(None, false),
                        |k| delegate().key_right_of(&k, nav),
                    ),
                    if rtl {
                        delegate().last_key(focused().as_ref(), false)
                    } else {
                        delegate().first_key(focused().as_ref(), false)
                    },
                    Some(if rtl {
                        FocusStrategy::Last
                    } else {
                        FocusStrategy::First
                    }),
                ),
            };
            let next = next.or_else(|| should_focus_wrap.then_some(wrap).flatten());
            navigate_to_key(e, next, child)
        }
    };

    let page = {
        move |e: &KeyboardEvent, down: bool| {
            let Some(current) = focused() else {
                return false;
            };
            let next = if down {
                delegate().key_page_below(&current)
            } else {
                delegate().key_page_above(&current)
            };
            navigate_to_key(e, next, None)
        }
    };

    let home_end = {
        move |e: &KeyboardEvent, home: bool| {
            let current = focused();
            if current.is_none() && e.shift_key() {
                return false;
            }
            let global = is_ctrl_key_pressed(e.modifiers());
            let target = if home {
                delegate().first_key(current.as_ref(), global)
            } else {
                delegate().last_key(current.as_ref(), global)
            };
            selection.set_focused_key(target.clone(), None);
            let Some(target) = target else {
                return false;
            };
            if global
                && e.shift_key()
                && untrack(|| selection.selection_mode()) == SelectionMode::Multiple
            {
                selection.extend_selection(&target);
            } else if select_on_focus() {
                selection.replace_selection(&target);
            }
            true
        }
    };

    let mut repeatable = KeyboardShortcuts::new();
    for key in ["ArrowDown", "ArrowUp", "ArrowLeft", "ArrowRight"] {
        for shortcut in with_selection_modifiers(key) {
            repeatable = repeatable.on(shortcut, move |e| arrow(e, key));
        }
    }
    for (key, down) in [("PageDown", true), ("PageUp", false)] {
        for shortcut in with_selection_modifiers(key) {
            repeatable = repeatable.on(shortcut, move |e| page(e, down));
        }
    }

    let mut once = KeyboardShortcuts::new();
    for (key, home) in [("Home", true), ("End", false)] {
        // Upstream also takes Cmd (+ Shift) on macOS: its `isCtrlKeyPressed` is Meta there,
        // which goes to the collection's first/last item (and extends the selection).
        let cmd = device::is_mac()
            .then(|| {
                [
                    Shortcut::key(key).primary(),
                    Shortcut::key(key).primary().shift(),
                ]
            })
            .into_iter()
            .flatten();
        for shortcut in with_selection_modifiers(key).into_iter().chain(cmd) {
            once = once.on(shortcut, move |e| home_end(e, home));
        }
    }
    once = once
        .on(Shortcut::key("a").primary(), move |_| {
            if untrack(|| selection.selection_mode()) == SelectionMode::Multiple
                && !disallow_select_all
            {
                selection.select_all();
                true
            } else {
                false
            }
        })
        .on(Shortcut::key("Escape"), move |_| {
            if escape_key_behavior == EscapeKeyBehavior::ClearSelection
                && !disallow_empty_selection
                && !untrack(|| selection.disallow_empty_selection() || selection.is_empty())
            {
                selection.clear_selection();
                true
            } else {
                false
            }
        })
        // Tab leaves the collection (it is a single tab stop): focus the last tabbable element
        // inside, so the browser's default Tab moves past the collection.
        .on(Shortcut::key("Tab"), move |_| {
            if !allows_tab_navigation && let Some(container) = element.get_untracked() {
                focus_last_tabbable(&container);
            }
            ShortcutOutcome::Ignored
        })
        // Shift+Tab: focus the collection itself, so the browser moves before it.
        .on(Shortcut::key("Tab").shift(), move |_| {
            if !allows_tab_navigation && let Some(container) = element.get_untracked() {
                focus_element(&container, true);
            }
            ShortcutOutcome::Ignored
        });

    let UseKeyboardReturn {
        props: repeat_keyboard,
    } = use_keyboard(UseKeyboardInput {
        shortcuts: Some(repeatable),
        allow_repeats: true,
        ..UseKeyboardInput::default()
    });
    let UseKeyboardReturn { props: keyboard } = use_keyboard(UseKeyboardInput {
        shortcuts: Some(once),
        ..UseKeyboardInput::default()
    });

    // Remember the scroll position, so focusing the collection again doesn't jump.
    let scroll_position = StoredValue::new((0.0, 0.0));
    let on_scroll = move |_: Event| {
        if let Some(el) = element.get_untracked() {
            scroll_position.set_value((el.scroll_top(), el.scroll_left()));
        }
    };

    let on_focusin = move |e: FocusEvent| {
        let current_target = e.expect_current_target();
        let inside = node_contains(
            current_target.dyn_ref::<web_sys::Node>(),
            e.target()
                .as_ref()
                .and_then(|t| t.dyn_ref::<web_sys::Node>()),
        )
        .unwrap_or(false);
        if untrack(|| selection.is_focused()) {
            if !inside {
                selection.set_focused(false);
            }
            return;
        }
        if !inside {
            return;
        }

        let modality = get_modality();
        selection.set_focused(true);

        if focused().is_none() {
            // Entering from below (Shift+Tab) focuses the last selected / last item.
            let from_below = e
                .related_target()
                .and_then(|t| t.dyn_into::<web_sys::Node>().ok())
                .zip(current_target.dyn_ref::<web_sys::Node>().cloned())
                .is_some_and(|(related, container)| {
                    container.compare_document_position(&related)
                        & web_sys::Node::DOCUMENT_POSITION_FOLLOWING
                        != 0
                });
            let key = if from_below {
                untrack(|| selection.last_selected_key())
                    .or_else(|| delegate().last_key(None, false))
            } else {
                untrack(|| selection.first_selected_key())
                    .or_else(|| delegate().first_key(None, false))
            };
            if let Some(key) = key {
                selection.set_focused_key(Some(key.clone()), None);
                if select_on_focus() && !untrack(|| selection.is_selected(&key)) {
                    selection.replace_selection(&key);
                }
            }
        } else if let Some(el) = element.get_untracked()
            && let Some(el) = el.dyn_ref::<web_sys::HtmlElement>()
        {
            let (top, left) = scroll_position.get_value();
            el.set_scroll_top(top);
            el.set_scroll_left(left);
        }

        // Focus the focused item, unless focus is already in it.
        if let Some(key) = focused()
            && let Some(item) = item_elements.get(&key)
        {
            let active = item.owner_document().as_ref().and_then(get_active_element);
            let focus_within = active
                .as_ref()
                .is_some_and(|a| item.contains(Some(a.unchecked_ref())));
            if !focus_within && !should_use_virtual_focus {
                // After this `focusin` dispatch: focusing the item synchronously would dispatch a
                // nested `focusin` into Leptos' delegated listener, which is still running (and
                // can't be re-entered).
                let item = send_wrapper::SendWrapper::new(item.clone());
                queue_microtask(move || focus_element(&item, true));
            }
            if modality == Modality::Keyboard {
                scroll_into_viewport(
                    Some(&item),
                    &ScrollIntoViewportOpts {
                        containing_element: element.get_untracked().map(|el| (*el).clone()),
                    },
                );
            }
        }
    };

    let on_focusout = move |e: FocusEvent| {
        let stays_within = e
            .related_target()
            .and_then(|t| t.dyn_into::<web_sys::Node>().ok())
            .zip(e.expect_current_target().dyn_into::<web_sys::Node>().ok())
            .is_some_and(|(related, container)| container.contains(Some(&related)));
        if stays_within {
            return;
        }
        // Removing the focused item (e.g. deleting a tag) blurs it. That isn't the user leaving
        // the collection: focus moves on to a neighbor (react-aria never sees this blur, as React
        // ignores events during its commit). Tell both apart once the DOM update is done.
        let blurred = e.expect_target().dyn_into::<web_sys::Node>().ok();
        let blurred = send_wrapper::SendWrapper::new(blurred);
        queue_microtask(move || {
            if blurred.as_ref().is_none_or(web_sys::Node::is_connected) {
                selection.set_focused(false);
            }
        });
    };

    // Clicking the scrollbar must not move focus away from the focused item.
    let on_mousedown = move |e: MouseEvent| {
        let on_container = element.get_untracked().is_some_and(|el| {
            e.target()
                .is_some_and(|t| t.unchecked_ref::<web_sys::Element>() == &*el)
        });
        if on_container {
            e.prevent_default();
        }
    };

    // -- Auto focus (once, as soon as there are items) --
    let did_auto_focus = StoredValue::new(false);
    let auto_focus = auto_focus.get_untracked();
    let pending_auto_focus = StoredValue::new(auto_focus.is_some());
    {
        Effect::new(move |_| {
            let size = selection.collection().with(|c| c.size());
            if !pending_auto_focus.get_value() {
                return;
            }
            let selected = untrack(|| selection.selected_keys());
            let mut key = match auto_focus {
                Some(AutoFocus::First) => delegate().first_key(None, false),
                Some(AutoFocus::Last) => delegate().last_key(None, false),
                Some(AutoFocus::Selected) | None => None,
            };
            if let Some(first_selectable) = untrack(|| {
                selection
                    .collection()
                    .with(|c| c.items().map(|n| n.key.clone()).collect::<Vec<_>>())
            })
            .into_iter()
            .find(|k| selected.contains(k) && untrack(|| selection.can_select_item(k)))
            {
                key = Some(first_selectable);
            }
            selection.set_focused(true);
            selection.set_focused_key(key.clone(), None);
            if let Some(key) = &key
                && select_on_focus()
                && selected.is_empty()
                && untrack(|| selection.can_select_item(key))
            {
                selection.replace_selection(key);
            }
            if key.is_none()
                && !should_use_virtual_focus
                && let Some(el) = element.get_untracked()
            {
                focus_safely(&el);
            }
            if size > 0 {
                pending_auto_focus.set_value(false);
                did_auto_focus.set_value(true);
            }
        });
    }

    // -- Scroll the focused item into view (keyboard navigation and auto focus) --
    {
        let last_focused: StoredValue<Option<Key>> = StoredValue::new(None);
        Effect::new(move |_| {
            let current = selection.focused_key();
            let is_focused = untrack(|| selection.is_focused());
            if is_focused
                && let Some(key) = &current
                && (Some(key) != last_focused.get_value().as_ref() || did_auto_focus.get_value())
                && (get_modality() == Modality::Keyboard || did_auto_focus.get_value())
                && let Some(container) = element.get_untracked()
                && let Some(item) = item_elements.get(key)
            {
                let item = (*item).clone();
                let container = (*container).clone();
                request_animation_frame(move || {
                    if let (Some(container_html), Some(item_html)) = (
                        container.dyn_ref::<web_sys::HtmlElement>(),
                        item.dyn_ref::<web_sys::HtmlElement>(),
                    ) {
                        scroll_into_view(container_html, item_html, ScrollIntoViewOpts::default());
                    }
                    if get_modality() != Modality::Virtual {
                        scroll_into_viewport(
                            Some(&item),
                            &ScrollIntoViewportOpts {
                                containing_element: Some(container),
                            },
                        );
                    }
                });
            }
            // The focused item disappeared while focus was inside: keep focus in the collection.
            if is_focused
                && !should_use_virtual_focus
                && current.is_none()
                && last_focused.get_value().is_some()
                && let Some(el) = element.get_untracked()
            {
                focus_safely(&el);
            }
            last_focused.set_value(current);
            did_auto_focus.set_value(false);
        });
    }

    // -- Type-ahead --
    let (on_keydown_capture, type_select_keydown) = if disallow_type_ahead {
        (EventHandler::empty(), EventHandler::empty())
    } else {
        let UseTypeSelectProps {
            on_keydown_capture,
            on_keydown,
        } = use_type_select(UseTypeSelectInput {
            delegate: delegate_signal,
            selection,
            on_type_select: None,
        })
        .props;
        (on_keydown_capture, on_keydown)
    };

    UseSelectableCollectionReturn {
        props: UseSelectableCollectionProps {
            // With virtual focus, the collection isn't focusable itself.
            tabindex: Signal::derive(move || {
                (!should_use_virtual_focus).then(|| {
                    if selection.focused_key().is_some() {
                        -1
                    } else {
                        0
                    }
                })
            }),
            collection_id,
            element_capture: element.attr(),
            on_keydown_capture,
            on_keydown: type_select_keydown
                .chain(keyboard.on_keydown)
                .chain(repeat_keyboard.on_keydown),
            on_keyup: keyboard.on_keyup.chain(repeat_keyboard.on_keyup),
            on_focusin: EventHandler::new(on_focusin),
            on_focusout: EventHandler::new(on_focusout),
            on_mousedown: EventHandler::new(on_mousedown),
            on_scroll: EventHandler::new(on_scroll),
        },
    }
}

/// Focus the last tabbable element inside `container`, unless focus already is in it and on a
/// tabbable element.
fn focus_last_tabbable(container: &web_sys::Element) {
    let Some(mut walker) = get_focusable_tree_walker(
        container,
        FocusableTreeWalkerOptions {
            tabbable: true,
            ..FocusableTreeWalkerOptions::default()
        },
    ) else {
        return;
    };
    let mut last: Option<web_sys::Node> = None;
    while let Some(node) = walker.last_child() {
        last = Some(node);
    }
    let Some(last) = last.and_then(|n| n.dyn_into::<web_sys::Element>().ok()) else {
        return;
    };
    let active = last.owner_document().as_ref().and_then(get_active_element);
    let focus_within_last = active
        .as_ref()
        .is_some_and(|a| last.contains(Some(a.unchecked_ref())));
    if !focus_within_last || active.as_ref().is_some_and(|a| !is_tabbable(a)) {
        focus_element(&last, true);
    }
}
