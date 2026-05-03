use leptos::{
    attr,
    attr::Attr,
    ev,
    ev::{On, SharedEventCallback},
    prelude::*,
};
use web_sys::{FocusEvent, KeyboardEvent};

use crate::{
    hooks::{
        IntoAttrs,
        focus::use_focus::{UseFocusInput, use_focus},
        interactions::use_keyboard::{KeyboardEventWrapper, UseKeyboardInput, use_keyboard},
    },
    utils::{
        EventHandler,
        element_capture::{CapturedElement, ElementCaptureAttr},
        focus::focus_safely,
    },
};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/%40react-aria/interactions/src/useFocusable.tsx

// ## DIFFERENT BEHAVIOR
//
// - Handler optimization when disabled
//   React-aria returns `undefined` props when no callbacks are provided,
//   relying on React's reconciliation to avoid attaching empty listeners.
//   Our `EventHandler` always attaches a listener but checks the disabled
//   state inside the handler. The overhead is negligible.
//
// - Context handler disabled guard
//   React-aria: `let interactionProps = props.isDisabled ? {} : domProps`
//   discards all interaction props (including context-provided handlers) when
//   disabled. Leptonic: context handlers are guarded with a `disabled` check
//   at chain time — the own handler (from `use_focus`) already checks disabled
//   internally, and context handlers are wrapped to also skip when disabled.
//
// - No FocusableProvider component
//   There is no wrapper component for providing FocusableContext to children.
//   Parent components must call `provide_context(FocusableContext { ... })`
//   directly before rendering focusable children.

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

/// Context for parent-to-child interaction prop forwarding.
///
/// Parent components (e.g., `TooltipTrigger`) that need to inject additional
/// event handlers or capture a focusable child's element **must** provide this
/// context via [`provide_context`].
///
/// When provided, [`use_focusable`] automatically reads the context and chains
/// the parent's handlers after its own. When not provided, `use_focusable`
/// uses only the handlers from its input.
///
/// # Example
///
/// ```ignore
/// // Parent component provides context:
/// provide_context(FocusableContext {
///     on_focus: Some(EventHandler::new(|_| { /* parent focus handler */ })),
///     element: Some(parent_element_capture),
///     ..Default::default()
/// });
///
/// // Child's use_focusable automatically reads and chains the context handlers.
/// ```
#[derive(Clone, Default)]
pub struct FocusableContext {
    /// Additional focus handler chained with the focusable element's own.
    pub on_focus: Option<EventHandler<FocusEvent>>,
    /// Additional blur handler chained with the focusable element's own.
    pub on_blur: Option<EventHandler<FocusEvent>>,
    /// Additional keydown handler chained with the focusable element's own.
    pub on_keydown: Option<EventHandler<KeyboardEvent>>,
    /// Additional keyup handler chained with the focusable element's own.
    pub on_keyup: Option<EventHandler<KeyboardEvent>>,
    /// Parent's element capture — the focusable element will be captured here too.
    pub element: Option<CapturedElement>,
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
    /// Focuses the element safely, deferring during screen reader interactions.
    ///
    /// Uses [`focus_safely`] which avoids page scrolling and defers focus during
    /// virtual (screen reader) modality to prevent `VoiceOver` scroll issues.
    /// If the element hasn't been captured yet (e.g., during SSR), this is a no-op.
    pub fn focus(&self) {
        if let Some(el) = self.element.get_untracked() {
            focus_safely(&el);
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
pub struct UseFocusableReturn {
    /// Props for programmatic merging. Call `.into_attrs()` for view spreading.
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
#[derive(Debug)]
pub struct UseFocusableProps {
    pub tabindex: Signal<Option<i32>>,
    pub on_focus: EventHandler<FocusEvent>,
    pub on_blur: EventHandler<FocusEvent>,
    pub on_keydown: EventHandler<KeyboardEvent>,
    pub on_keyup: EventHandler<KeyboardEvent>,
    pub element_capture: ElementCaptureAttr,
}

impl IntoAttrs for UseFocusableProps {
    type Attrs = UseFocusableAttrs;

    fn into_attrs(self) -> Self::Attrs {
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
/// # Context
///
/// If a [`FocusableContext`] is provided by an ancestor (via [`provide_context`]),
/// this hook automatically chains the context's handlers after its own and
/// captures the element for the parent. This is how parent components inject
/// interaction behavior into focusable children without the child needing to
/// know about the parent.
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
    let UseFocusableInput {
        disabled,
        auto_focus,
        exclude_from_tab_order,
        on_focus,
        on_blur,
        on_focus_change,
        on_key_down,
        on_key_up,
    } = input;

    let element = CapturedElement::new();

    // Create the focus handle
    let focus_handle = FocusHandle { element };

    // Read parent-provided context (e.g., from TooltipTrigger).
    let ctx = use_context::<FocusableContext>();

    // Use the focus hook
    let focus = use_focus(UseFocusInput {
        disabled,
        on_focus,
        on_blur,
        on_focus_change,
    });

    // Use the keyboard hook
    let keyboard = use_keyboard(UseKeyboardInput {
        disabled,
        on_key_down,
        on_key_up,
    });

    // Chain context handlers with own handlers (own first, context second).
    // Context handlers are guarded with a disabled check: when disabled is true,
    // context handlers are skipped. This matches react-aria's behavior where
    // `interactionProps = props.isDisabled ? {} : domProps` discards all
    // interaction props (including context-provided handlers) when disabled.
    // The own handlers (from use_focus/use_keyboard) already check disabled internally.
    let on_focus = match ctx.as_ref().and_then(|c| c.on_focus.clone()) {
        Some(ctx_handler) => focus.props.on_focus.chain(move |e: FocusEvent| {
            if !disabled.get_untracked() {
                ctx_handler.call(e);
            }
        }),
        None => focus.props.on_focus,
    };
    let on_blur = match ctx.as_ref().and_then(|c| c.on_blur.clone()) {
        Some(ctx_handler) => focus.props.on_blur.chain(move |e: FocusEvent| {
            if !disabled.get_untracked() {
                ctx_handler.call(e);
            }
        }),
        None => focus.props.on_blur,
    };
    let on_keydown = match ctx.as_ref().and_then(|c| c.on_keydown.clone()) {
        Some(ctx_handler) => keyboard.props.on_keydown.chain(move |e: KeyboardEvent| {
            if !disabled.get_untracked() {
                ctx_handler.call(e);
            }
        }),
        None => keyboard.props.on_keydown,
    };
    let on_keyup = match ctx.as_ref().and_then(|c| c.on_keyup.clone()) {
        Some(ctx_handler) => keyboard.props.on_keyup.chain(move |e: KeyboardEvent| {
            if !disabled.get_untracked() {
                ctx_handler.call(e);
            }
        }),
        None => keyboard.props.on_keyup,
    };

    // Create combined element capture that updates both own and parent's CapturedElement.
    let parent_element = ctx.as_ref().and_then(|c| c.element);
    let element_capture = ElementCaptureAttr::new(move |el| {
        element.set(el.clone());
        if let Some(parent) = parent_element {
            parent.set(el);
        }
    });

    // Handle auto-focus.
    // Uses `get_element()` (reactive) so the Effect re-runs when the element
    // is captured — critical for client-side navigation.
    if auto_focus {
        let auto_focus_done: StoredValue<bool> = StoredValue::new(false);

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
            on_focus,
            on_blur,
            on_keydown,
            on_keyup,
            element_capture,
        },
        focus_handle,
    }
}
