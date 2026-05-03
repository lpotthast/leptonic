use leptos::{
    attr,
    attr::Attr,
    ev,
    ev::{On, SharedEventCallback},
    oco::Oco,
    prelude::*,
};
use uuid::Uuid;
use web_sys::{KeyboardEvent, MouseEvent, PointerEvent};

use super::use_menu_trigger_state::UseMenuTriggerStateReturn;
use crate::{
    hooks::{
        IntoAttrs, PropsWithStyles,
        interactions::use_press::{LongPressEvent, PressEvent, UsePressInput, use_press},
        overlay::use_overlay_trigger::{
            OverlayTriggerType, UseOverlayTriggerInput, use_overlay_trigger,
        },
        selection::use_selection_state::FocusStrategy,
    },
    prelude::AriaHasPopup,
    utils::{
        EventHandler, aria::AriaExpanded, focus::focus_event_target, pointer_type::PointerType,
    },
};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/%40react-aria/menu/src/useMenuTrigger.ts

// No intentional deviations from the react-aria implementation.

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
    pub menu_type: OverlayTriggerType,

    /// Whether the menu trigger is disabled.
    pub disabled: Signal<bool>,

    /// How the menu is triggered.
    pub trigger: MenuTriggerType,

    /// The state from `use_menu_trigger_state`.
    pub state: UseMenuTriggerStateReturn,
}

/// The return value of the `use_menu_trigger` hook.
#[derive(Debug)]
pub struct UseMenuTriggerReturn {
    /// Props for the menu trigger element. Call `.into_parts()` for view spreading and styles.
    pub props: PropsWithStyles<UseMenuTriggerProps>,

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
#[derive(Debug)]
pub struct UseMenuTriggerProps {
    /// Unique identifier for the trigger element.
    pub id: String,
    /// The type of popup this trigger opens (e.g., "menu").
    pub aria_haspopup: Option<AriaHasPopup>,
    /// Whether the popup is currently expanded.
    pub aria_expanded: Signal<Option<AriaExpanded>>,
    /// ID of the controlled popup element.
    pub aria_controls: Signal<Option<String>>,
    /// Keyboard event handler for menu navigation (Enter/Space/Arrow keys).
    pub on_keydown: EventHandler<KeyboardEvent>,
    /// Click event handler for menu toggle.
    pub on_click: EventHandler<MouseEvent>,
    /// Pointer down event handler for menu toggle.
    pub on_pointerdown: EventHandler<PointerEvent>,
}

impl IntoAttrs for UseMenuTriggerProps {
    type Attrs = UseMenuTriggerAttrs;

    fn into_attrs(self) -> Self::Attrs {
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
    Attr<attr::AriaHaspopup, Option<AriaHasPopup>>,
    Attr<attr::AriaExpanded, Signal<Option<AriaExpanded>>>,
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
    let UseMenuTriggerInput {
        menu_type,
        disabled,
        trigger,
        state,
    } = input;

    let menu_trigger_id = format!("menu-trigger-{}", Uuid::new_v4());
    let menu_id = format!("menu-{}", Uuid::new_v4());

    // Get overlay trigger ARIA attributes
    let overlay_trigger = use_overlay_trigger(UseOverlayTriggerInput {
        show: state.is_open,
        overlay_id: Oco::Owned(menu_id.clone()),
        overlay_type: menu_type,
    });

    // Handle keyboard navigation
    let trigger_type = trigger;

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
                // Don't open menu on Enter/Space for long press trigger.
                // Also skip if default was already prevented (e.g. by typeahead consuming Space).
                if trigger_type == MenuTriggerType::LongPress || e.default_prevented() {
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
    let (press_on_keydown, press_on_click, press_on_pointerdown, press_styles) = if trigger
        == MenuTriggerType::Press
    {
        let press = use_press(UsePressInput {
            disabled,
            force_prevent_default: false,
            force_propagation: true,
            allow_text_selection_on_press: false,
            should_cancel_on_pointer_exit: false,
            prevent_focus_on_press: true,
            force_is_pressed: None,
            on_press: Callback::new(move |e: PressEvent| {
                // Touch triggers toggle on press
                if e.pointer_type == PointerType::Touch {
                    // Focus the trigger before opening so FocusScope can restore to it
                    focus_event_target(&e.target, true);
                    state.toggle.run(None);
                }
            }),
            on_press_start: Some(Callback::new(move |e: PressEvent| {
                // Mouse/virtual pointer opens on press start (not toggle — the overlay
                // dismiss mechanism handles closing; the trigger only opens).
                if e.pointer_type != PointerType::Touch && e.pointer_type != PointerType::Keyboard {
                    // Focus the trigger before opening so FocusScope can restore to it
                    focus_event_target(&e.target, true);
                    let strategy = if e.pointer_type == PointerType::Virtual {
                        Some(FocusStrategy::First)
                    } else {
                        None
                    };
                    state.open.run(strategy);
                }
            })),
            on_press_up: None,
            on_press_end: None,
            on_press_change: None,
            on_double_press: None,
            on_long_press_start: None,
            on_long_press: None,
            on_long_press_end: None,
            long_press_threshold: None,
            long_press_accessibility_description: None,
        });
        let (press_props, press_styles) = press.props.into_inner();
        (
            press_props.on_keydown,
            press_props.on_click,
            press_props.on_pointerdown,
            press_styles,
        )
    } else {
        // Long press trigger — use use_press with long press fields.
        // Note: For long press, we can't focus the trigger before opening because
        // the long press callback is fired from a timeout without access to the target.
        // Focus restoration will still work if the trigger had focus when FocusScope mounted.
        let press = use_press(UsePressInput {
            disabled,
            force_prevent_default: false,
            force_propagation: true,
            allow_text_selection_on_press: false,
            should_cancel_on_pointer_exit: false,
            prevent_focus_on_press: false,
            force_is_pressed: None,
            on_press: Callback::new(|_| {}),
            on_press_up: None,
            on_press_start: None,
            on_press_end: None,
            on_press_change: None,
            on_double_press: None,
            on_long_press_start: Some(Callback::new(move |_: LongPressEvent| {
                // Close any open menu when starting a new long press
                state.close.run(());
            })),
            on_long_press: Some(Callback::new(move |_: LongPressEvent| {
                state.open.run(Some(FocusStrategy::First));
            })),
            on_long_press_end: None,
            long_press_threshold: None,
            long_press_accessibility_description: Some("Long press to open menu".into()),
        });
        let (press_props, press_styles) = press.props.into_inner();
        (
            press_props.on_keydown,
            press_props.on_click,
            press_props.on_pointerdown,
            press_styles,
        )
    };

    let menu_trigger_id_signal = Signal::derive({
        let id = menu_trigger_id.clone();
        move || id.clone()
    });

    let menu_id_signal = Signal::derive({
        let id = menu_id;
        move || id.clone()
    });

    // Chain press keydown with menu keydown (press first, then menu)
    let combined_keydown = press_on_keydown.chain(menu_keydown_handler);

    UseMenuTriggerReturn {
        props: PropsWithStyles::new(
            UseMenuTriggerProps {
                id: menu_trigger_id,
                aria_haspopup: overlay_trigger.props.aria_haspopup,
                aria_expanded: overlay_trigger.props.aria_expanded,
                aria_controls: overlay_trigger.props.aria_controls,
                on_keydown: combined_keydown,
                on_click: press_on_click,
                on_pointerdown: press_on_pointerdown,
            },
            press_styles,
        ),
        menu_props: UseMenuTriggerMenuProps {
            id: menu_id_signal,
            aria_labelledby: menu_trigger_id_signal,
            auto_focus: state.focus_strategy,
            on_close: state.close,
        },
    }
}
