use leptos::{
    attr::custom::{CustomAttr, custom_attribute},
    ev,
    ev::{On, SharedEventCallback},
    prelude::*,
};
use web_sys::FocusEvent;

use crate::{
    hooks::{
        IntoAttrs, UseFocusVisibleReturn,
        focus::{
            use_focus::{UseFocusInput, use_focus},
            use_focus_visible::{UseFocusVisibleInput, use_focus_visible},
            use_focus_within::{FocusWithinEvent, UseFocusWithinInput, use_focus_within},
        },
    },
    utils::EventHandler,
};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/focus/src/useFocusRing.ts

// ## ADDITIONAL FUNCTIONALITY
//
// - Additional `disabled`, `on_focus`, `on_blur`, `on_focus_change` props
//   React-aria's `useFocusRing` does not accept these; it only exposes an
//   internal `onFocusChange`. Leptonic forwards these to the underlying
//   `use_focus` / `use_focus_within` hooks as a convenience so consumers can
//   receive focus events without a separate `use_focus` call.
//
// - `data-focus-visible` attribute output
//   React-aria does not output any data attributes; consumers style based on the
//   `isFocusVisible` boolean. Leptonic outputs a `data-focus-visible="true"`
//   custom attribute for CSS-only styling. This is an ergonomic addition.

/// Input parameters for the `use_focus_ring` hook.
#[derive(Debug, Clone, Copy)]
pub struct UseFocusRingInput {
    /// Whether the focus ring is disabled.
    pub disabled: Signal<bool>,

    /// Whether to track focus within descendants rather than just the element itself.
    ///
    /// When `false` (default), the focus ring tracks focus on the element itself via
    /// `focus`/`blur` events. The ring shows only when the element is focused and the
    /// user is navigating via keyboard.
    ///
    /// When `true`, the focus ring tracks focus within the element's subtree via
    /// `focusin`/`focusout` events (using `use_focus_within`). The ring shows when
    /// any descendant is focused and the user is navigating via keyboard. This is
    /// useful for container elements like form groups or toolbars.
    pub within: bool,

    /// Whether to auto-focus the element.
    pub auto_focus: bool,

    /// Whether the element is a text input. When `true`, only Tab/Escape keys
    /// trigger focus-visible; other keyboard events are suppressed. This is
    /// used for compound text-input components (e.g., a date picker where focus
    /// is on a button but the component should use text-input focus rules).
    pub is_text_input: bool,

    /// Optional callback when the element receives focus (or focus enters when `within=true`).
    pub on_focus: Option<Callback<FocusEvent>>,

    /// Optional callback when the element loses focus (or focus leaves when `within=true`).
    pub on_blur: Option<Callback<FocusEvent>>,

    /// Optional callback when focus state changes.
    pub on_focus_change: Option<Callback<bool>>,
}

impl Default for UseFocusRingInput {
    fn default() -> Self {
        Self {
            disabled: Signal::derive(|| false),
            within: false,
            auto_focus: false,
            is_text_input: false,
            on_focus: None,
            on_blur: None,
            on_focus_change: None,
        }
    }
}

/// The return value of the `use_focus_ring` hook.
#[derive(Debug)]
pub struct UseFocusRingReturn {
    /// Whether the focus ring should be visible.
    pub is_focus_visible: Signal<bool>,

    /// Whether the element is currently focused (or has focus within when `within=true`).
    pub is_focused: Signal<bool>,

    /// Props for programmatic merging. Call `.into_attrs()` for view spreading.
    pub props: UseFocusRingProps,
}

/// Props from `use_focus_ring` that can be extracted and merged programmatically.
#[derive(Debug)]
pub struct UseFocusRingProps {
    pub on_focus: EventHandler<FocusEvent>,
    pub on_blur: EventHandler<FocusEvent>,
    pub on_focusin: EventHandler<FocusEvent>,
    pub on_focusout: EventHandler<FocusEvent>,
    pub data_focus_visible: Signal<Option<&'static str>>,
}

impl IntoAttrs for UseFocusRingProps {
    type Attrs = UseFocusRingAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            self.on_focus.into_on(ev::focus),
            self.on_blur.into_on(ev::blur),
            self.on_focusin.into_on(ev::focusin),
            self.on_focusout.into_on(ev::focusout),
            custom_attribute("data-focus-visible", self.data_focus_visible),
        )
    }
}

/// These attributes must be spread onto the target element: `<foo {..attrs} />`
pub type UseFocusRingAttrs = (
    On<ev::focus, SharedEventCallback<FocusEvent>>,
    On<ev::blur, SharedEventCallback<FocusEvent>>,
    On<ev::focusin, SharedEventCallback<FocusEvent>>,
    On<ev::focusout, SharedEventCallback<FocusEvent>>,
    CustomAttr<&'static str, Signal<Option<&'static str>>>,
);

/// Determines whether a focus ring should be displayed for an element.
///
/// This hook combines focus state tracking with focus visibility detection.
/// A focus ring should be shown when:
/// 1. The element is focused (or has focus within, when `within=true`), AND
/// 2. The user is interacting via keyboard (not mouse/touch)
///
/// This helps maintain accessibility while avoiding visual clutter from
/// focus rings on mouse/touch interactions.
///
/// When `within=false` (default), focus is tracked via `focus`/`blur` events
/// on the element itself. When `within=true`, focus is tracked via
/// `focusin`/`focusout` events on the element's subtree.
///
/// # Example
///
/// ```ignore
/// let focus_ring = use_focus_ring(UseFocusRingInput::default());
///
/// view! {
///     <button
///         {..focus_ring.props.into_attrs()}
///         tabindex="0"
///     >
///         "Focus me"
///     </button>
/// }
/// ```
///
/// The `data-focus-visible` attribute is automatically added when the focus ring
/// should be shown. Style it with CSS:
///
/// ```css
/// [data-focus-visible="true"] {
///     outline: 3px solid var(--brand-color);
///     outline-offset: 2px;
/// }
/// ```
pub fn use_focus_ring(input: UseFocusRingInput) -> UseFocusRingReturn {
    let UseFocusRingInput {
        disabled,
        within,
        auto_focus,
        is_text_input,
        on_focus,
        on_blur,
        on_focus_change,
    } = input;

    let (focused, set_focused) = signal(false);

    let UseFocusVisibleReturn {
        focus_should_be_visible,
        modality: _,
    } = use_focus_visible(UseFocusVisibleInput {
        auto_focus,
        enabled: focused.into(),
        is_text_input,
    });

    let focus_visible = Signal::derive(move || {
        let is_focused = focused.get();
        let should_be_visible = focus_should_be_visible.get();
        is_focused && should_be_visible
    });

    let (handle_focus, handle_blur, handle_focusin, handle_focusout) = track_focus(
        within,
        on_focus,
        on_blur,
        on_focus_change,
        disabled,
        set_focused,
    );

    let data_focus_visible = Signal::derive(move || {
        if focus_visible.get() {
            Some("true")
        } else {
            None
        }
    });

    UseFocusRingReturn {
        is_focus_visible: focus_visible,
        is_focused: focused.into(),
        props: UseFocusRingProps {
            on_focus: handle_focus,
            on_blur: handle_blur,
            on_focusin: handle_focusin,
            on_focusout: handle_focusout,
            data_focus_visible,
        },
    }
}

fn track_focus(
    within: bool,
    on_focus: Option<Callback<FocusEvent>>,
    on_blur: Option<Callback<FocusEvent>>,
    on_focus_change: Option<Callback<bool>>,
    disabled: Signal<bool>,
    set_focused: WriteSignal<bool>,
) -> (
    EventHandler<FocusEvent>,
    EventHandler<FocusEvent>,
    EventHandler<FocusEvent>,
    EventHandler<FocusEvent>,
) {
    if within {
        // Track focus within the element's subtree.
        let focus_within = use_focus_within(UseFocusWithinInput {
            disabled,
            on_focus_within: Some(Callback::new(move |e: FocusWithinEvent| {
                set_focused.set(true);
                if let Some(on_focus) = on_focus {
                    on_focus.run(e.event);
                }
            })),
            on_blur_within: Some(Callback::new(move |e: FocusWithinEvent| {
                set_focused.set(false);
                if let Some(on_blur) = on_blur {
                    on_blur.run(e.event);
                }
            })),
            on_focus_within_change: on_focus_change,
        });

        (
            EventHandler::empty(),
            EventHandler::empty(),
            focus_within.props.on_focusin,
            focus_within.props.on_focusout,
        )
    } else {
        // Track focus on the element itself.
        let focus = use_focus(UseFocusInput {
            disabled,
            on_focus: Some(Callback::new(move |e| {
                set_focused.set(true);
                if let Some(on_focus) = on_focus {
                    on_focus.run(e);
                }
            })),
            on_blur: Some(Callback::new(move |e| {
                set_focused.set(false);
                if let Some(on_blur) = on_blur {
                    on_blur.run(e);
                }
            })),
            on_focus_change,
        });

        (
            focus.props.on_focus,
            focus.props.on_blur,
            EventHandler::empty(),
            EventHandler::empty(),
        )
    }
}
