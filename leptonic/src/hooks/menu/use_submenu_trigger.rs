// Upstream: react-aria/src/menu/useSubmenuTrigger.ts @ 99e6102368
use std::time::Duration;

use leptos::prelude::*;
use web_sys::KeyboardEvent;

use super::{SubmenuTriggerState, UseSafelyMouseToSubmenuInput, use_safely_mouse_to_submenu};
use crate::{
    hooks::{
        InteractOutsideFilter, PressEvent,
        collections::FocusStrategy,
        interactions::use_keyboard::{UseKeyboardInput, UseKeyboardProps, use_keyboard},
    },
    utils::{
        CapturedElement, EventAccessors,
        aria::AriaHasPopup,
        focus::focus_element,
        i18n::use_locale,
        id::use_id,
        keyboard_shortcut::{KeyboardShortcuts, Shortcut},
        pointer_type::PointerType,
        shadow_dom::{get_active_element, node_contains},
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Element references are `CapturedElement`s; the deprecated `node` prop is not offered.
// - `kind` is the `SubmenuKind` enum (react-aria: `type: 'menu' | 'dialog'`), `delay` a
//   `Duration`.
// - The popover props are the outside-interaction filter only: a submenu's popover is always
//   non-modal (`use_popover`'s `is_submenu`).
//
// ## OMITTED FEATURES
// - Virtual focus (`shouldUseVirtualFocus`), as in `use_menu`.
//
// =============================================================================

/// What a submenu trigger opens.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SubmenuKind {
    /// A menu (keyboard navigation continues in it).
    #[default]
    Menu,
    /// A dialog.
    Dialog,
}

impl From<SubmenuKind> for AriaHasPopup {
    fn from(kind: SubmenuKind) -> Self {
        match kind {
            SubmenuKind::Menu => Self::Menu,
            SubmenuKind::Dialog => Self::Dialog,
        }
    }
}

/// Input of [`use_submenu_trigger`].
#[derive(Debug, Clone, Copy)]
pub struct UseSubmenuTriggerInput {
    pub state: SubmenuTriggerState,
    /// The trigger item.
    pub trigger: CapturedElement,
    /// The menu containing the trigger item.
    pub parent_menu: CapturedElement,
    /// The submenu (or subdialog) the trigger opens.
    pub submenu: CapturedElement,
    pub kind: SubmenuKind,
    pub is_disabled: Signal<bool>,
    /// How long hovering the trigger takes to open the submenu. Default: 200 ms.
    pub delay: Duration,
}

/// What the trigger item needs (pass it to `use_menu_item`).
#[derive(Debug, Clone)]
pub struct SubmenuTriggerItem {
    pub id: String,
    /// The trigger item's element: the item captures it.
    pub element: CapturedElement,
    pub aria_controls: Signal<Option<String>>,
    pub aria_haspopup: Signal<Option<AriaHasPopup>>,
    pub is_open: Signal<bool>,
    /// ArrowRight (ArrowLeft in right-to-left text) opens the submenu and moves focus into it;
    /// the other arrow closes it.
    pub keyboard: UseKeyboardProps,
    pub on_press_start: Callback<PressEvent>,
    pub on_press: Callback<PressEvent>,
    pub on_hover_change: Callback<bool>,
}

/// What the submenu needs (pass it to `use_menu`).
#[derive(Debug, Clone)]
pub struct SubmenuProps {
    pub id: String,
    /// The trigger item names the submenu.
    pub aria_labelledby: String,
    /// The submenu's level in the menu tree.
    pub level: usize,
    /// Closes the whole menu tree (after an item's action).
    pub on_close: Callback<()>,
    /// Which item to focus when the submenu opens.
    pub auto_focus: Signal<Option<FocusStrategy>>,
    /// The arrow key towards the parent menu and Escape close the submenu, returning focus to
    /// the trigger. `None` for a subdialog.
    pub keyboard: Option<UseKeyboardProps>,
}

/// Return value of [`use_submenu_trigger`].
#[derive(Debug, Clone)]
pub struct UseSubmenuTriggerReturn {
    pub trigger: SubmenuTriggerItem,
    pub submenu: SubmenuProps,
    /// For the submenu's popover: an interaction outside it, except on the trigger, closes it.
    pub should_close_on_interact_outside: InteractOutsideFilter,
}

/// The behavior of a menu item opening a submenu (or a dialog): opening by press, hover (after
/// `delay`) and the arrow key pointing into the submenu, closing by the other arrow key, Escape
/// or focusing another item of the menu, and letting the pointer travel to the submenu.
#[allow(clippy::too_many_lines)]
pub fn use_submenu_trigger(input: UseSubmenuTriggerInput) -> UseSubmenuTriggerReturn {
    let UseSubmenuTriggerInput {
        state,
        trigger,
        parent_menu,
        submenu,
        kind,
        is_disabled,
        delay,
    } = input;
    let trigger_id = use_id("submenu-trigger");
    let overlay_id = use_id("submenu");
    let locale = use_locale();
    let is_rtl = move || locale.with_untracked(crate::utils::i18n::Locale::is_rtl);

    let open_timeout: StoredValue<Option<TimeoutHandle>> = StoredValue::new(None);
    let cancel_open_timeout = move || {
        if let Some(timeout) = open_timeout.try_update_value(Option::take).flatten() {
            timeout.clear();
        }
    };
    on_cleanup(cancel_open_timeout);
    let open = move |focus_strategy: Option<FocusStrategy>| {
        cancel_open_timeout();
        state.open(focus_strategy);
    };
    let close = move || {
        cancel_open_timeout();
        state.close();
    };
    let focus_trigger = move || {
        if let Some(trigger) = trigger.get_untracked() {
            focus_element(&trigger, true);
        }
    };

    // Whether the event's current target (the submenu) has focus, and the event comes from
    // inside it. Otherwise focus is elsewhere (e.g. an input in the popover).
    let from_inside = |e: &KeyboardEvent| {
        let Some(current) = e
            .current_target()
            .and_then(|t| wasm_bindgen::JsCast::dyn_into::<web_sys::Element>(t).ok())
        else {
            return false;
        };
        let focus_within = current
            .owner_document()
            .as_ref()
            .and_then(get_active_element)
            .is_some_and(|active| node_contains(&current, &active));
        let target = e.expect_target();
        focus_within
            && wasm_bindgen::JsCast::dyn_ref::<web_sys::Node>(&target)
                .is_some_and(|target| node_contains(&current, target))
    };
    let close_towards_parent = move |e: &KeyboardEvent, rtl: bool| {
        if !from_inside(e) || is_rtl() != rtl {
            return false;
        }
        close();
        focus_trigger();
        true
    };
    let submenu_keyboard = (kind == SubmenuKind::Menu).then(|| {
        use_keyboard(UseKeyboardInput {
            shortcuts: Some(
                KeyboardShortcuts::new()
                    .on(Shortcut::key("ArrowLeft"), move |e| {
                        close_towards_parent(e, false)
                    })
                    .on(Shortcut::key("ArrowRight"), move |e| {
                        close_towards_parent(e, true)
                    })
                    .on(Shortcut::key("Escape"), move |e| {
                        if !from_inside(e) {
                            return false;
                        }
                        close();
                        focus_trigger();
                        true
                    }),
            ),
            ..UseKeyboardInput::default()
        })
        .props
    });

    // On the trigger: the arrow into the submenu opens it (focusing its first item), the other
    // one closes it.
    let open_or_close = move |into_submenu: bool| {
        if is_disabled.get_untracked() {
            return false;
        }
        if into_submenu {
            if !state.is_open.get_untracked() {
                open(Some(FocusStrategy::First));
            }
            let trigger_focused = trigger.get_untracked().is_some_and(|trigger| {
                trigger
                    .owner_document()
                    .as_ref()
                    .and_then(get_active_element)
                    .is_some_and(|active| active == *trigger)
            });
            if kind == SubmenuKind::Menu
                && trigger_focused
                && let Some(submenu) = submenu.get_untracked()
            {
                focus_element(&submenu, true);
            }
            true
        } else if state.is_open.get_untracked() {
            close();
            true
        } else {
            false
        }
    };
    let trigger_keyboard = use_keyboard(UseKeyboardInput {
        shortcuts: Some(
            KeyboardShortcuts::new()
                .on(Shortcut::key("ArrowRight"), move |_| {
                    open_or_close(!is_rtl())
                })
                .on(Shortcut::key("ArrowLeft"), move |_| open_or_close(is_rtl())),
        ),
        ..UseKeyboardInput::default()
    })
    .props;

    // Keyboard and screen readers open on press start (focusing the first item); mouse and
    // touch on press, after the pointer is released.
    let on_press_start = Callback::new(move |e: PressEvent| {
        if !is_disabled.get_untracked()
            && matches!(e.pointer_type, PointerType::Virtual | PointerType::Keyboard)
        {
            open(Some(FocusStrategy::First));
        }
    });
    let on_press = Callback::new(move |e: PressEvent| {
        if !is_disabled.get_untracked()
            && matches!(e.pointer_type, PointerType::Touch | PointerType::Mouse)
        {
            open(None);
        }
    });
    let on_hover_change = Callback::new(move |is_hovered: bool| {
        if is_disabled.get_untracked() {
            return;
        }
        if is_hovered && !state.is_open.get_untracked() {
            if open_timeout.with_value(Option::is_none) {
                let handle = set_timeout_with_handle(
                    move || {
                        open_timeout.set_value(None);
                        open(None);
                    },
                    delay,
                )
                .ok();
                open_timeout.set_value(handle);
            }
        } else if !is_hovered {
            cancel_open_timeout();
        }
    });

    // Focus moving to another item of the parent menu (e.g. by hovering it) closes the submenu.
    #[cfg(not(feature = "ssr"))]
    {
        use leptos::ev;
        use send_wrapper::SendWrapper;

        use crate::utils::event_listeners::{Listener, listen_to};

        let listener: StoredValue<Option<SendWrapper<Listener>>> = StoredValue::new(None);
        Effect::new(move || {
            listener.set_value(None);
            let Some(menu) = parent_menu.get() else {
                return;
            };
            let menu_el = (*menu).clone();
            let handle = listen_to(&menu, ev::focusin, false, move |e: web_sys::FocusEvent| {
                let target = e.expect_target();
                let Some(target) = wasm_bindgen::JsCast::dyn_ref::<web_sys::Node>(&target) else {
                    return;
                };
                let is_trigger = trigger
                    .get_untracked()
                    .is_some_and(|trigger| AsRef::<web_sys::Node>::as_ref(&**trigger) == target);
                if state.is_open.get_untracked() && node_contains(&menu_el, target) && !is_trigger {
                    close();
                }
            });
            listener.set_value(Some(SendWrapper::new(handle)));
        });
        on_cleanup(move || listener.set_value(None));
    }
    #[cfg(feature = "ssr")]
    let _ = parent_menu;

    use_safely_mouse_to_submenu(UseSafelyMouseToSubmenuInput {
        menu: parent_menu,
        submenu,
        is_open: state.is_open,
        is_disabled,
    });

    let is_open = state.is_open;
    let controls_id = overlay_id.clone();
    UseSubmenuTriggerReturn {
        trigger: SubmenuTriggerItem {
            id: trigger_id.clone(),
            element: trigger,
            aria_controls: Signal::derive(move || is_open.get().then(|| controls_id.clone())),
            aria_haspopup: Signal::derive(move || (!is_disabled.get()).then_some(kind.into())),
            is_open,
            keyboard: trigger_keyboard,
            on_press_start,
            on_press,
            on_hover_change,
        },
        submenu: SubmenuProps {
            id: overlay_id,
            aria_labelledby: trigger_id,
            level: state.level,
            on_close: Callback::new(move |()| state.close_all()),
            auto_focus: state.focus_strategy,
            keyboard: submenu_keyboard,
        },
        should_close_on_interact_outside: InteractOutsideFilter::new(move |target| {
            trigger
                .get_untracked()
                .is_none_or(|trigger| *trigger != *target)
        }),
    }
}
