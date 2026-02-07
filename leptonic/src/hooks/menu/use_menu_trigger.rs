use crate::utils::EventHandler;
use leptos::attr;
use leptos::attr::Attr;
use leptos::ev;
use leptos::ev::{On, SharedEventCallback};
use leptos::oco::Oco;
use leptos::prelude::*;
use uuid::Uuid;
use web_sys::{KeyboardEvent, MouseEvent, PointerEvent};

use crate::hooks::interactions::use_long_press::{
    use_long_press, LongPressEvent, UseLongPressInput,
};
use crate::hooks::interactions::use_press::{use_press, PressEvent, UsePressInput};
use crate::hooks::overlay::use_overlay_trigger::{use_overlay_trigger, UseOverlayTriggerInput};
use crate::hooks::selection::use_selectable_collection::FocusStrategy;
use crate::prelude::AriaHasPopup;
use crate::utils::focus::focus_event_target;
use crate::utils::pointer_type::PointerType;

use super::use_menu_trigger_state::UseMenuTriggerStateReturn;

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/%40react-aria/menu/src/useMenuTrigger.ts

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
pub struct UseMenuTriggerInput {
    /// The type of menu that the menu trigger opens.
    pub menu_type: AriaHasPopup,

    /// Whether the menu trigger is disabled.
    pub disabled: Signal<bool>,

    /// How the menu is triggered.
    pub trigger: MenuTriggerType,

    /// The state from `use_menu_trigger_state`.
    pub state: UseMenuTriggerStateReturn,
}

/// The return value of the `use_menu_trigger` hook.
#[derive(Debug, Clone)]
pub struct UseMenuTriggerReturn {
    /// Props for the menu trigger element. Call `.to_attrs()` or `.into_attrs()` for view spreading.
    pub props: UseMenuTriggerProps,

    /// Props to pass to the menu.
    pub menu_props: UseMenuTriggerMenuProps,
}

/// Props for the menu opened by this trigger.
#[derive(Debug, Clone, Copy)]
pub struct UseMenuTriggerMenuProps {
    /// The id that labels this menu.
    pub aria_labelledby: Signal<String>,

    /// The focus strategy signal - updates when menu opens.
    pub auto_focus: Signal<Option<FocusStrategy>>,

    /// Callback to close the menu.
    pub on_close: Callback<()>,
}

/// Props from `use_menu_trigger` that can be extracted and merged programmatically.
///
/// This type includes:
/// - `id`: Unique identifier for the trigger element
/// - Menu ARIA: `aria-haspopup`, `aria-expanded`, `aria-controls`
/// - Event handlers: `on_keydown`, `on_click`, `on_pointerdown`
#[derive(Debug, Clone)]
pub struct UseMenuTriggerProps {
    /// Unique identifier for the trigger element.
    pub id: String,
    /// The type of popup this trigger opens (e.g., "menu").
    pub aria_haspopup: &'static str,
    /// Whether the popup is currently expanded.
    pub aria_expanded: Signal<&'static str>,
    /// ID of the controlled popup element.
    pub aria_controls: Signal<Option<String>>,
    /// Keyboard event handler for menu navigation (Enter/Space/Arrow keys).
    pub on_keydown: EventHandler<KeyboardEvent>,
    /// Click event handler for menu toggle.
    pub on_click: EventHandler<MouseEvent>,
    /// Pointer down event handler for menu toggle.
    pub on_pointerdown: EventHandler<PointerEvent>,
}

impl UseMenuTriggerProps {
    /// Convert to spreadable attributes for Leptos views, cloning internally.
    #[must_use]
    pub fn to_attrs(&self) -> UseMenuTriggerAttrs {
        self.clone().into_attrs()
    }

    /// Convert to spreadable attributes for Leptos views, consuming self.
    #[must_use]
    pub fn into_attrs(self) -> UseMenuTriggerAttrs {
        (
            Attr(attr::Id, self.id),
            Attr(attr::AriaHaspopup, self.aria_haspopup),
            Attr(attr::AriaExpanded, self.aria_expanded),
            Attr(attr::AriaControls, self.aria_controls),
            self.on_keydown.into_on(ev::keydown),
            self.on_click.into_on(ev::click),
            self.on_pointerdown.into_on(ev::pointerdown),
        )
    }
}

/// These attributes must be spread onto the menu trigger element.
pub type UseMenuTriggerAttrs = (
    Attr<attr::Id, String>,
    Attr<attr::AriaHaspopup, &'static str>,
    Attr<attr::AriaExpanded, Signal<&'static str>>,
    Attr<attr::AriaControls, Signal<Option<String>>>,
    On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
    On<ev::click, SharedEventCallback<MouseEvent>>,
    On<ev::pointerdown, SharedEventCallback<PointerEvent>>,
);

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
///     menu_type: AriaHasPopup::Menu,
///     disabled: Signal::derive(|| false),
///     trigger: MenuTriggerType::Press,
///     state,
/// });
///
/// view! {
///     <button {..menu_trigger.props.into_attrs()}>
///         "Open Menu"
///     </button>
///     <Show when=move || state.is_open.get()>
///         <Menu
///             aria_labelledby=menu_trigger.menu_props.aria_labelledby
///             auto_focus=menu_trigger.menu_props.auto_focus
///             on_close=menu_trigger.menu_props.on_close
///         />
///     </Show>
/// }
/// ```
#[allow(clippy::too_many_lines)]
pub fn use_menu_trigger(input: UseMenuTriggerInput) -> UseMenuTriggerReturn {
    let menu_trigger_id = format!("menu-trigger-{}", Uuid::new_v4());
    let menu_id = format!("menu-{}", Uuid::new_v4());

    let state = input.state;

    // Get overlay trigger ARIA attributes
    let overlay_trigger = use_overlay_trigger(UseOverlayTriggerInput {
        show: state.is_open,
        overlay_id: Oco::Owned(menu_id.clone()),
        overlay_type: input.menu_type,
    });

    // Handle keyboard navigation
    let disabled = input.disabled;
    let trigger_type = input.trigger;

    let menu_keydown_handler = EventHandler::new(move |e: KeyboardEvent| {
        if disabled.get_untracked() {
            return;
        }

        // For long press trigger, only respond to Alt+Arrow keys
        if trigger_type == MenuTriggerType::LongPress && !e.alt_key() {
            return;
        }

        match e.key().as_str() {
            "Enter" | " " => {
                // Don't open menu on Enter/Space for long press trigger
                if trigger_type == MenuTriggerType::LongPress {
                    return;
                }
                e.prevent_default();
                e.stop_propagation();
                state.toggle.run(Some(FocusStrategy::First));
            }
            "ArrowDown" => {
                e.prevent_default();
                e.stop_propagation();
                state.toggle.run(Some(FocusStrategy::First));
            }
            "ArrowUp" => {
                e.prevent_default();
                e.stop_propagation();
                state.toggle.run(Some(FocusStrategy::Last));
            }
            _ => {}
        }
    });

    // Handle press events based on trigger type
    let (press_on_keydown, press_on_click, press_on_pointerdown) = if input.trigger
        == MenuTriggerType::Press
    {
        let disabled_for_press = input.disabled;

        let press = use_press(UsePressInput {
            disabled: disabled_for_press,
            force_prevent_default: false,
            allow_propagation: true,
            allow_text_selection_on_press: false,
            should_cancel_on_pointer_exit: false,
            on_press: Callback::new(move |e: PressEvent| {
                // Touch triggers toggle on press
                if e.pointer_type == PointerType::Touch {
                    // Focus the trigger before opening so FocusScope can restore to it
                    if let Some(ref target) = e.target {
                        focus_event_target(target, true);
                    }
                    state.toggle.run(None);
                }
            }),
            on_press_start: Some(Callback::new(move |e: PressEvent| {
                // Mouse/virtual pointer toggles on press start
                if e.pointer_type != PointerType::Touch && e.pointer_type != PointerType::Keyboard {
                    // Focus the trigger before opening so FocusScope can restore to it
                    if let Some(ref target) = e.target {
                        focus_event_target(target, true);
                    }
                    let strategy = if e.pointer_type == PointerType::Virtual {
                        Some(FocusStrategy::First)
                    } else {
                        None
                    };
                    state.toggle.run(strategy);
                }
            })),
            on_press_up: None,
            on_press_end: None,
            on_press_change: None,
        });
        (
            press.props.on_keydown,
            press.props.on_click,
            press.props.on_pointerdown,
        )
    } else {
        // Long press trigger
        // Note: For long press, we can't focus the trigger before opening because
        // the long press callback is fired from a timeout without access to the target.
        // Focus restoration will still work if the trigger had focus when FocusScope mounted.
        let long_press = use_long_press(UseLongPressInput {
            disabled: input.disabled,
            on_long_press_start: Some(Callback::new(move |_: LongPressEvent| {
                // Close any open menu when starting a new long press
                state.close.run(());
            })),
            on_long_press: Some(Callback::new(move |_: LongPressEvent| {
                state.open.run(Some(FocusStrategy::First));
            })),
            on_long_press_end: None,
            threshold: None,
        });
        (
            long_press.props.on_keydown,
            long_press.props.on_click,
            long_press.props.on_pointerdown,
        )
    };

    let menu_trigger_id_signal = Signal::derive({
        let id = menu_trigger_id.clone();
        move || id.clone()
    });

    // Chain press keydown with menu keydown (press first, then menu)
    let combined_keydown = press_on_keydown.chain(menu_keydown_handler);

    UseMenuTriggerReturn {
        props: UseMenuTriggerProps {
            id: menu_trigger_id,
            aria_haspopup: overlay_trigger.props.aria_haspopup,
            aria_expanded: overlay_trigger.props.aria_expanded,
            aria_controls: overlay_trigger.props.aria_controls,
            on_keydown: combined_keydown,
            on_click: press_on_click,
            on_pointerdown: press_on_pointerdown,
        },
        menu_props: UseMenuTriggerMenuProps {
            aria_labelledby: menu_trigger_id_signal,
            auto_focus: state.focus_strategy,
            on_close: state.close,
        },
    }
}
