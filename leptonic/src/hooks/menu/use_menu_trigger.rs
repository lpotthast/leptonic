// Upstream: react-aria/src/menu/useMenuTrigger.ts @ 99e6102368
use leptos::prelude::*;
use web_sys::KeyboardEvent;

use super::use_menu_trigger_state::MenuTriggerStateApi;
use crate::{
    hooks::{
        UseButtonInput,
        collections::{AutoFocus, FocusStrategy},
        interactions::{
            use_context_menu::ContextMenuEvent,
            use_press::{LongPressEvent, PressEvent},
        },
        overlay::use_overlay_trigger::{
            OverlayTriggerType, UseOverlayTriggerInput, use_overlay_trigger,
        },
    },
    utils::{
        focus::focus_event_target,
        id::use_id,
        keyboard_shortcut::{KeyboardShortcuts, Shortcut},
        point::Point,
        pointer_type::PointerType,
    },
};
use crate::utils::intl_strings::{MenuStrings, use_localized_strings};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/react-aria/src/menu/useMenuTrigger.ts

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Returns a `UseButtonInput` (`button`) instead of DOM props. React-aria's `menuTriggerProps`
//   are `AriaButtonProps` too, but also smuggle DOM handlers for long press through
//   `PressResponder`. Leptonic's `use_press` handles long presses itself, so everything fits into
//   the button input and the trigger element gets exactly one press handler.
//
// =============================================================================

/// How the menu is triggered.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MenuTriggerType {
    /// Menu opens on press (click/tap).
    #[default]
    Press,
    /// Menu opens on long press (touch and hold).
    LongPress,
    /// Menu opens as a context menu (right click, Shift+F10, long press on touch screens, ...) at
    /// the point it was requested. The trigger gets no `aria-haspopup`, `aria-expanded` or
    /// `aria-controls`: it doesn't open the menu on activation.
    ContextMenu,
}

/// Input parameters for the `use_menu_trigger` hook.
#[derive(Debug, Clone, Copy)]
pub struct UseMenuTriggerInput<S: MenuTriggerStateApi> {
    /// The type of menu that the menu trigger opens.
    pub menu_type: OverlayTriggerType,

    /// Whether the menu trigger is disabled.
    pub is_disabled: Signal<bool>,

    /// How the menu is triggered.
    pub trigger: MenuTriggerType,

    /// The state from `use_menu_trigger_state`, or a state with a menu (a select's, ...).
    pub state: S,
}

/// The return value of the `use_menu_trigger` hook.
#[derive(Debug)]
pub struct UseMenuTriggerReturn {
    /// Configuration for the trigger button. Pass it to [`use_button`](fn@crate::hooks::use_button),
    /// adding your own settings with struct update syntax:
    /// `use_button(UseButtonInput { on_hover_start: .., ..menu_trigger.button })`.
    pub button: UseButtonInput,

    /// Props to pass to the menu.
    pub menu_props: UseMenuTriggerMenuProps,
}

/// Props for the menu opened by this trigger.
#[derive(Debug, Clone)]
pub struct UseMenuTriggerMenuProps {
    /// The unique ID for the menu element.
    /// This must be set on the menu so that `aria-controls` on the trigger points to it.
    pub id: String,

    /// The id that labels this menu.
    pub aria_labelledby: Signal<String>,

    /// Where focus goes when the menu opens: the first or last item when opened by keyboard,
    /// else the menu itself (react-aria: `autoFocus: focusStrategy || true`). For
    /// `CollectionOptions::auto_focus`.
    pub auto_focus: Signal<Option<AutoFocus>>,

    /// Callback to close the menu.
    pub on_close: Callback<()>,
}

/// Provides the behavior and accessibility implementation for a menu trigger.
///
/// A menu trigger is an element (typically a button) that opens a menu when activated.
/// It supports both press-based and long-press-based triggering, and handles keyboard
/// navigation to open the menu with arrow keys.
///
/// # Example
///
/// ```ignore
/// let state = use_menu_trigger_state(UseMenuTriggerStateInput::default());
///
/// let menu_trigger = use_menu_trigger(UseMenuTriggerInput {
///     menu_type: OverlayTriggerType::Menu,
///     is_disabled: Signal::stored(false),
///     trigger: MenuTriggerType::Press,
///     state,
/// });
/// let (button_attrs, button_styles) = use_button(menu_trigger.button).props.into_parts();
/// let menu_props = menu_trigger.menu_props;
///
/// view! {
///     <button {..button_attrs} style=button_styles>"Open Menu"</button>
///     <Show when=move || state.is_open()>
///         // A menu built with `use_menu`, given `menu_props.id`, `aria_labelledby`,
///         // `auto_focus` and `on_close`.
///         <MyMenu props=menu_props.clone() />
///     </Show>
/// }
/// ```
pub fn use_menu_trigger<S: MenuTriggerStateApi>(
    input: UseMenuTriggerInput<S>,
) -> UseMenuTriggerReturn {
    let UseMenuTriggerInput {
        menu_type,
        is_disabled: disabled,
        trigger,
        state,
    } = input;

    let menu_trigger_id = use_id("menu-trigger");
    let menu_id = use_id("menu");

    let overlay_trigger = use_overlay_trigger(UseOverlayTriggerInput {
        is_open: Signal::derive(move || state.is_open()),
        overlay_id: menu_id.clone(),
        overlay_type: menu_type,
    });

    // Open (toggle) the menu, unless this key combination is not meant to open it in the current
    // trigger mode, or something else (e.g. type-ahead consuming Space) already handled the event.
    let open = move |should_open: bool, e: &KeyboardEvent, strategy: FocusStrategy| {
        if !should_open || e.default_prevented() {
            return false;
        }
        state.toggle(Some(strategy));
        true
    };
    let press = trigger == MenuTriggerType::Press;
    let long_press = trigger == MenuTriggerType::LongPress;
    let shortcuts = KeyboardShortcuts::new()
        .on(Shortcut::key("Enter"), move |e| {
            open(press, e, FocusStrategy::First)
        })
        .on(Shortcut::key(" "), move |e| {
            open(press, e, FocusStrategy::First)
        })
        .on(Shortcut::key("ArrowDown"), move |e| {
            open(press, e, FocusStrategy::First)
        })
        .on(Shortcut::key("ArrowUp"), move |e| {
            open(press, e, FocusStrategy::Last)
        })
        .on(Shortcut::key("Enter").alt(), move |e| {
            open(long_press, e, FocusStrategy::First)
        })
        .on(Shortcut::key(" ").alt(), move |e| {
            open(long_press, e, FocusStrategy::First)
        })
        // Alt+Arrow opens the menu in both modes. For long press triggers, it is the only way to
        // open the menu with the keyboard.
        .on(Shortcut::key("ArrowDown").alt(), move |e| {
            open(true, e, FocusStrategy::First)
        })
        .on(Shortcut::key("ArrowUp").alt(), move |e| {
            open(true, e, FocusStrategy::Last)
        });

    let button = match trigger {
        MenuTriggerType::Press => UseButtonInput {
            prevent_focus_on_press: true,
            // For consistency with native menus, open on mouse down / key down, but on touch up.
            on_press_start: Some(Callback::new(move |e: PressEvent| {
                if e.pointer_type != PointerType::Touch
                    && e.pointer_type != PointerType::Keyboard
                    && !disabled.get_untracked()
                {
                    // Focus the trigger before opening, so FocusScope can restore focus to it.
                    focus_event_target(&e.target, true);
                    // Screen reader users get the first item focused, others the menu itself.
                    state.open(
                        (e.pointer_type == PointerType::Virtual).then_some(FocusStrategy::First),
                    );
                }
            })),
            on_press: Some(Callback::new(move |e: PressEvent| {
                if e.pointer_type == PointerType::Touch && !disabled.get_untracked() {
                    focus_event_target(&e.target, true);
                    state.toggle(None);
                }
            })),
            ..UseButtonInput::default()
        },
        MenuTriggerType::ContextMenu => UseButtonInput {
            on_context_menu: Some(Callback::new(move |e: ContextMenuEvent| {
                let rect = e.target.get_bounding_client_rect();
                state.set_point(Some(Point {
                    x: rect.x() + e.x,
                    y: rect.y() + e.y,
                }));
                state.open(None);
            })),
            ..UseButtonInput::default()
        },
        MenuTriggerType::LongPress => UseButtonInput {
            on_long_press_start: Some(Callback::new(move |_: LongPressEvent| {
                state.close();
            })),
            on_long_press: Some(Callback::new(move |_: LongPressEvent| {
                state.open(Some(FocusStrategy::First));
            })),
            long_press_accessibility_description: {
                let strings = use_localized_strings::<MenuStrings>();
                Signal::derive(move || Some(strings.read().long_press_message())).into()
            },
            ..UseButtonInput::default()
        },
    };

    let aria_haspopup = overlay_trigger.props.aria_haspopup;
    let button = if trigger == MenuTriggerType::ContextMenu {
        // A context menu trigger isn't announced as opening a menu (it doesn't on activation), and
        // the keyboard opens it with the context menu shortcuts only.
        UseButtonInput {
            id: Some(menu_trigger_id.clone()),
            is_disabled: disabled,
            ..button
        }
    } else {
        UseButtonInput {
            id: Some(menu_trigger_id.clone()),
            is_disabled: disabled,
            aria_haspopup: Signal::stored(aria_haspopup),
            aria_expanded: overlay_trigger.props.aria_expanded,
            aria_controls: overlay_trigger.props.aria_controls,
            shortcuts: Some(shortcuts),
            ..button
        }
    };

    if trigger == MenuTriggerType::ContextMenu {
        close_context_menu_on_outside_right_click(state);
    }

    UseMenuTriggerReturn {
        button,
        menu_props: UseMenuTriggerMenuProps {
            id: menu_id,
            aria_labelledby: Signal::stored(menu_trigger_id),
            auto_focus: Signal::derive(move || {
                Some(match state.focus_strategy() {
                    Some(FocusStrategy::First) => AutoFocus::First,
                    Some(FocusStrategy::Last) => AutoFocus::Last,
                    None => AutoFocus::Selected,
                })
            }),
            on_close: Callback::new(move |()| state.close()),
        },
    }
}

/// While a context menu is open, a right click outside closes it, so the browser's context menu
/// appears instead. Everything outside the menu is inert, so the click's target is the body.
pub(crate) fn close_context_menu_on_outside_right_click<S: MenuTriggerStateApi>(state: S) {
    #[cfg(feature = "ssr")]
    let _ = state;
    #[cfg(not(feature = "ssr"))]
    {
        use leptos::ev;
        use leptos_use::use_document;
        use send_wrapper::SendWrapper;

        use crate::utils::event_listeners::{Listener, listen_to};

        let listener: StoredValue<Option<SendWrapper<Listener>>> = StoredValue::new(None);
        Effect::new(move || {
            listener.set_value(None);
            if !state.is_open() {
                return;
            }
            let Some(document) = use_document().as_ref().cloned() else {
                return;
            };
            let body = document.body();
            let handle = listen_to(
                &document,
                ev::mousedown,
                false,
                move |e: web_sys::MouseEvent| {
                    let is_context_click = e.button() == 2 || (e.button() == 0 && e.ctrl_key());
                    let on_body = body.as_ref().is_some_and(|body| {
                        e.target().as_ref() == Some(AsRef::<web_sys::EventTarget>::as_ref(body))
                    });
                    if is_context_click && on_body {
                        state.close();
                    }
                },
            );
            listener.set_value(Some(SendWrapper::new(handle)));
        });
        on_cleanup(move || listener.set_value(None));
    }
}
