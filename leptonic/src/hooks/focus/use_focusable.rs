use leptos::attr;
use leptos::attr::Attr;
use leptos::ev;
use leptos::ev::{On, SharedEventCallback};
use leptos::prelude::*;
use web_sys::{FocusEvent, KeyboardEvent};

use crate::hooks::focus::use_focus::{use_focus, UseFocusInput};
use crate::hooks::interactions::use_keyboard::{
    use_keyboard, KeyboardEventWrapper, UseKeyboardInput,
};
use crate::utils::element_capture::{CapturedElement, ElementCaptureAttr};
use crate::utils::focus::focus_element;
use crate::utils::EventHandler;

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/%40react-aria/interactions/src/useFocusable.tsx
//
// ## React-aria deviation
//
// **React-aria pattern**: `useFocusable(props, domRef)` - caller passes a ref as parameter.
//
// **Leptonic pattern**: `use_focusable(input)` returns props with `ElementCaptureAttr` that
// automatically captures the element when spread.
//
// This is a deliberate deviation for better ergonomics - users don't need to manually create
// and wire up NodeRefs. The element is captured automatically when attributes are spread.

/// Input parameters for the `use_focusable` hook.
#[derive(Debug, Clone, Copy)]
pub struct UseFocusableInput {
    /// Whether focus should be disabled.
    pub disabled: Signal<bool>,

    /// Whether the element should be focused on mount.
    pub auto_focus: bool,

    /// Whether to exclude the element from the tab order.
    /// When true, the element will have tabIndex=-1.
    pub exclude_from_tab_order: Signal<bool>,

    /// Handler called when the element receives focus.
    pub on_focus: Option<Callback<FocusEvent>>,

    /// Handler called when the element loses focus.
    pub on_blur: Option<Callback<FocusEvent>>,

    /// Handler called when the focus state changes.
    pub on_focus_change: Option<Callback<bool>>,

    /// Handler called when a key is pressed.
    pub on_key_down: Option<Callback<KeyboardEventWrapper>>,

    /// Handler called when a key is released.
    pub on_key_up: Option<Callback<KeyboardEventWrapper>>,
}

impl Default for UseFocusableInput {
    fn default() -> Self {
        Self {
            disabled: Signal::derive(|| false),
            auto_focus: false,
            exclude_from_tab_order: Signal::derive(|| false),
            on_focus: None,
            on_blur: None,
            on_focus_change: None,
            on_key_down: None,
            on_key_up: None,
        }
    }
}

/// A handle for programmatically focusing an element.
///
/// Obtained from the `use_focusable` hook, this handle allows you to
/// focus the element from anywhere in your code.
///
/// # Example
///
/// ```ignore
/// let UseFocusableReturn { props, focus_handle } = use_focusable(UseFocusableInput::default());
///
/// // Later, programmatically focus the element
/// focus_handle.focus();
/// ```
#[derive(Copy, Clone)]
pub struct FocusHandle {
    element: CapturedElement,
}

impl FocusHandle {
    /// Focuses the element.
    ///
    /// This uses `focus_safely` which prevents scrolling.
    /// If the element hasn't been captured yet (e.g., during SSR), this is a no-op.
    pub fn focus(&self) {
        if let Some(el) = self.element.get_untracked() {
            focus_element(&el, true);
        }
    }

    /// Focuses the element without scrolling the page.
    ///
    /// This is an alias for `focus()` since `focus()` already prevents scrolling.
    pub fn focus_without_scrolling(&self) {
        self.focus();
    }

    /// Returns whether the element has been captured (non-reactive).
    ///
    /// The element is captured when the view is built (attrs are spread).
    /// During SSR, this will always return false.
    #[must_use]
    pub fn has_element(&self) -> bool {
        self.element.get_untracked().is_some()
    }

    /// Reactively read the captured element.
    ///
    /// Tracks changes, so calling this inside an Effect will cause the
    /// Effect to re-run when the element is captured.
    pub fn get_element(&self) -> Option<send_wrapper::SendWrapper<web_sys::Element>> {
        self.element.get()
    }
}

impl std::fmt::Debug for FocusHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FocusHandle")
            .field("has_element", &self.has_element())
            .finish()
    }
}

/// The return value of the `use_focusable` hook.
#[derive(Clone)]
pub struct UseFocusableReturn {
    /// Props for programmatic merging. Call `.to_attrs()` or `.into_attrs()` for view spreading.
    pub props: UseFocusableProps,

    /// Handle for programmatically focusing the element.
    pub focus_handle: FocusHandle,
}

impl std::fmt::Debug for UseFocusableReturn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UseFocusableReturn")
            .field("props", &self.props)
            .field("focus_handle", &self.focus_handle)
            .finish()
    }
}

/// Props from `use_focusable` that can be extracted and merged programmatically.
#[derive(Debug, Clone)]
pub struct UseFocusableProps {
    pub tabindex: Signal<Option<i32>>,
    pub on_focus: EventHandler<FocusEvent>,
    pub on_blur: EventHandler<FocusEvent>,
    pub on_keydown: EventHandler<KeyboardEvent>,
    pub on_keyup: EventHandler<KeyboardEvent>,
    pub element_capture: ElementCaptureAttr,
}

impl UseFocusableProps {
    /// Convert to spreadable attributes for Leptos views, cloning internally.
    #[must_use]
    pub fn to_attrs(&self) -> UseFocusableAttrs {
        (
            Attr(attr::Tabindex, self.tabindex),
            self.on_focus.to_on(ev::focus),
            self.on_blur.to_on(ev::blur),
            self.on_keydown.to_on(ev::keydown),
            self.on_keyup.to_on(ev::keyup),
            self.element_capture.clone(),
        )
    }

    /// Convert to spreadable attributes for Leptos views, consuming self.
    #[must_use]
    pub fn into_attrs(self) -> UseFocusableAttrs {
        (
            Attr(attr::Tabindex, self.tabindex),
            self.on_focus.into_on(ev::focus),
            self.on_blur.into_on(ev::blur),
            self.on_keydown.into_on(ev::keydown),
            self.on_keyup.into_on(ev::keyup),
            self.element_capture,
        )
    }
}

/// These attributes must be spread onto the target element.
pub type UseFocusableAttrs = (
    Attr<attr::Tabindex, Signal<Option<i32>>>,
    On<ev::focus, SharedEventCallback<FocusEvent>>,
    On<ev::blur, SharedEventCallback<FocusEvent>>,
    On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
    On<ev::keyup, SharedEventCallback<KeyboardEvent>>,
    ElementCaptureAttr,
);

/// Used to make an element focusable and capable of auto focus.
///
/// This hook combines `use_focus` and `use_keyboard` to provide a complete
/// solution for making elements focusable. It handles:
/// - Focus and blur events
/// - Keyboard events
/// - Auto-focus on mount
/// - Tab index management
/// - Programmatic focus via `FocusHandle`
///
/// The hook automatically captures the DOM element through [`ElementCaptureAttr`],
/// so you don't need to create or pass a `NodeRef`. Just spread the props
/// onto your element and focus management works automatically.
///
/// # Example
///
/// ```ignore
/// let UseFocusableReturn { props, focus_handle } = use_focusable(UseFocusableInput {
///     disabled: Signal::derive(|| false),
///     auto_focus: false,
///     exclude_from_tab_order: Signal::derive(|| false),
///     on_focus: Some(Callback::new(|_| {
///         // Element received focus
///     })),
///     on_blur: None,
///     on_focus_change: None,
///     on_key_down: Some(Callback::new(|e| {
///         if e.key() == "Enter" {
///             // Handle enter key
///         } else {
///             e.continue_propagation();
///         }
///     })),
///     on_key_up: None,
/// });
///
/// view! {
///     <div role="button" {..props.into_attrs()}>
///         "Click me"
///     </div>
///
///     // Programmatically focus the element
///     <button on:click=move |_| focus_handle.focus()>
///         "Focus the element"
///     </button>
/// }
/// ```
pub fn use_focusable(input: UseFocusableInput) -> UseFocusableReturn {
    let disabled = input.disabled;
    let auto_focus = input.auto_focus;
    let exclude_from_tab_order = input.exclude_from_tab_order;

    let element = CapturedElement::new();

    // Create the focus handle
    let focus_handle = FocusHandle { element };

    // Use the focus hook
    let focus = use_focus(UseFocusInput {
        disabled,
        on_focus: input.on_focus,
        on_blur: input.on_blur,
        on_focus_change: input.on_focus_change,
    });

    // Use the keyboard hook
    let keyboard = use_keyboard(UseKeyboardInput {
        disabled,
        on_key_down: input.on_key_down,
        on_key_up: input.on_key_up,
    });

    // Handle auto-focus.
    // Uses `get_element()` (reactive) so the Effect re-runs when the element
    // is captured — critical for client-side navigation.
    if auto_focus {
        let auto_focus_done: StoredValue<bool, LocalStorage> = StoredValue::new_local(false);

        Effect::new(move |_| {
            if auto_focus_done.get_value() {
                return;
            }

            if focus_handle.get_element().is_some() {
                focus_handle.focus();
                auto_focus_done.set_value(true);
            }
        });
    }

    // Compute the tab index
    let tab_index = Signal::derive(move || {
        if disabled.get() {
            None
        } else if exclude_from_tab_order.get() {
            Some(-1)
        } else {
            Some(0)
        }
    });

    UseFocusableReturn {
        props: UseFocusableProps {
            tabindex: tab_index,
            on_focus: focus.props.on_focus,
            on_blur: focus.props.on_blur,
            on_keydown: keyboard.props.on_keydown,
            on_keyup: keyboard.props.on_keyup,
            element_capture: element.attr(),
        },
        focus_handle,
    }
}
