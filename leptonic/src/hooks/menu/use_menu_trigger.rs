// Upstream: react-aria/src/menu/useMenuTrigger.ts @ 99e6102368
use leptos::{oco::Oco, prelude::*};
use web_sys::KeyboardEvent;

use super::use_menu_trigger_state::MenuTriggerStateApi;
use crate::{
    hooks::{
        UseButtonInput,
        collections::{AutoFocus, FocusStrategy},
        interactions::use_press::{LongPressEvent, PressEvent},
        overlay::use_overlay_trigger::{
            OverlayTriggerType, UseOverlayTriggerInput, use_overlay_trigger,
        },
    },
    utils::{
        focus::focus_event_target,
        id::use_id,
        keyboard_shortcut::{KeyboardShortcuts, Shortcut},
        pointer_type::PointerType,
    },
};

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
// ## OMITTED FEATURES
// - `trigger="contextMenu"`: requires `use_context_menu` (not yet ported).
// - Localized long press description (English only, until leptonic has localized strings).
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
    /// Configuration for the trigger button. Pass it to [`use_button`](crate::hooks::use_button),
    /// adding your own settings with struct update syntax:
    /// `use_button(UseButtonInput { on_hover_start: .., ..menu_trigger.button })`.
    pub button: UseButtonInput,

    /// Props to pass to the menu.
    pub menu_props: UseMenuTriggerMenuProps,
}

/// Props for the menu opened by this trigger.
#[derive(Debug, Clone, Copy)]
pub struct UseMenuTriggerMenuProps {
    /// The unique ID for the menu element.
    /// This must be set on the menu so that `aria-controls` on the trigger points to it.
    pub id: Signal<String>,

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
///     disabled: Signal::derive(|| false),
///     trigger: MenuTriggerType::Press,
///     state,
/// });
///
/// view! {
///     <button {..menu_trigger.props.into_attrs()}>
///         "Open Menu"
///     </button>
///     <Show when=move || state.is_open()>
///         <Menu
///             aria_labelledby=menu_trigger.menu_props.aria_labelledby
///             auto_focus=menu_trigger.menu_props.auto_focus
///             on_close=menu_trigger.menu_props.on_close
///         />
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
        show: Signal::derive(move || state.is_open()),
        overlay_id: Oco::Owned(menu_id.clone()),
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
        MenuTriggerType::LongPress => UseButtonInput {
            on_long_press_start: Some(Callback::new(move |_: LongPressEvent| {
                state.close();
            })),
            on_long_press: Some(Callback::new(move |_: LongPressEvent| {
                state.open(Some(FocusStrategy::First));
            })),
            long_press_accessibility_description: Some(
                "Long press or press Alt + ArrowDown to open menu".into(),
            ),
            ..UseButtonInput::default()
        },
    };

    let aria_haspopup = overlay_trigger.props.aria_haspopup;
    let button = UseButtonInput {
        id: Some(Oco::Owned(menu_trigger_id.clone())),
        is_disabled: disabled,
        aria_haspopup: Signal::stored(aria_haspopup),
        aria_expanded: overlay_trigger.props.aria_expanded,
        aria_controls: overlay_trigger.props.aria_controls,
        shortcuts: Some(shortcuts),
        ..button
    };

    UseMenuTriggerReturn {
        button,
        menu_props: UseMenuTriggerMenuProps {
            id: Signal::stored(menu_id),
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
