// Upstream: react-aria/src/interactions/useFocusable.tsx @ 99e6102368
use std::sync::Arc;

use leptos::{
    attr,
    attr::{
        Attr, Attribute, NextAttribute,
        any_attribute::{AnyAttribute, AnyAttributeState},
    },
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
        keyboard_shortcut::KeyboardShortcuts,
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - `FocusableContext` is provided with `provide_context` or the `Focusable` atom; there is no
//   `FocusableProvider` component.
//
// ## DIFFERENT BEHAVIOR
// - The handlers are always attached and check `is_disabled` when they run (react-aria attaches
//   none without callbacks, and drops every interaction prop, including the context's, when
//   disabled): `is_disabled` is reactive. The context's handlers are skipped while disabled too.
//
// =============================================================================

/// Input parameters for the `use_focusable` hook.
#[derive(Debug, Clone)]
pub struct UseFocusableInput {
    /// Whether focus should be disabled.
    pub is_disabled: Signal<bool>,

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

    /// Keyboard shortcuts, see [`UseKeyboardInput::shortcuts`].
    pub shortcuts: Option<KeyboardShortcuts>,

    /// See [`UseKeyboardInput::allow_repeats`].
    pub allow_shortcut_repeats: bool,
}

impl Default for UseFocusableInput {
    fn default() -> Self {
        Self {
            is_disabled: Signal::derive(|| false),
            auto_focus: false,
            exclude_from_tab_order: Signal::derive(|| false),
            on_focus: None,
            on_blur: None,
            on_focus_change: None,
            on_key_down: None,
            on_key_up: None,
            shortcuts: None,
            allow_shortcut_repeats: false,
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
    /// Describes the focusable element (e.g. a tooltip), in addition to its own description.
    pub aria_describedby: Option<Signal<Option<String>>>,
    /// Further attributes for the focusable element (e.g. pointer handlers), spread as given.
    pub attrs: Option<FocusableContextAttrs>,
    /// Parent's element capture — the focusable element will be captured here too.
    pub element: Option<CapturedElement>,
}

/// Further attributes a [`FocusableContext`] gives the focusable element (react-aria-components'
/// `FocusableProvider` passes arbitrary DOM props): a factory, as attributes are spread once per
/// render of the element.
#[derive(Clone)]
pub struct FocusableContextAttrs(Arc<dyn Fn() -> AnyAttribute + Send + Sync>);

impl FocusableContextAttrs {
    pub fn new(attrs: impl Fn() -> AnyAttribute + Send + Sync + 'static) -> Self {
        Self(Arc::new(attrs))
    }

    /// The attributes, to spread onto the focusable element.
    pub fn attrs(&self) -> AnyAttribute {
        (self.0)()
    }
}

impl std::fmt::Debug for FocusableContextAttrs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("FocusableContextAttrs")
    }
}

/// The attribute spreading a [`FocusableContext`]'s further attributes (nothing without a
/// context). It keeps the thread-safe factory and creates the type-erased attributes only when the
/// element renders, so the attributes of focusable hooks stay `Send + Sync` (storable in a
/// `StoredValue`).
#[derive(Debug, Clone)]
pub struct FocusableContextAttr(pub Option<FocusableContextAttrs>);

impl Attribute for FocusableContextAttr {
    const MIN_LENGTH: usize = 0;

    type State = Option<AnyAttributeState>;
    type AsyncOutput = Self;
    type Cloneable = Self;
    type CloneableOwned = Self;

    fn html_len(&self) -> usize {
        0
    }

    fn to_html(
        self,
        buf: &mut String,
        class: &mut String,
        style: &mut String,
        inner_html: &mut String,
    ) {
        if let Some(attrs) = self.0 {
            attrs.attrs().to_html(buf, class, style, inner_html);
        }
    }

    fn hydrate<const FROM_SERVER: bool>(self, el: &web_sys::Element) -> Self::State {
        self.0.map(|attrs| attrs.attrs().hydrate::<FROM_SERVER>(el))
    }

    fn build(self, el: &web_sys::Element) -> Self::State {
        self.0.map(|attrs| attrs.attrs().build(el))
    }

    fn rebuild(self, state: &mut Self::State) {
        // Whether there is a context doesn't change for an element.
        if let (Some(attrs), Some(state)) = (self.0, state.as_mut()) {
            attrs.attrs().rebuild(state);
        }
    }

    fn into_cloneable(self) -> Self::Cloneable {
        self
    }

    fn into_cloneable_owned(self) -> Self::CloneableOwned {
        self
    }

    fn dry_resolve(&mut self) {
        // The attributes are created when rendering; nothing to resolve beforehand.
    }

    fn resolve(self) -> impl Future<Output = Self::AsyncOutput> + Send {
        std::future::ready(self)
    }
}

impl NextAttribute for FocusableContextAttr {
    type Output<NewAttr: Attribute> = (Self, NewAttr);

    fn add_any_attr<NewAttr: Attribute>(self, new_attr: NewAttr) -> Self::Output<NewAttr> {
        (self, new_attr)
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

    /// The element capture behind this handle, for hooks that need the element too.
    pub fn element(&self) -> CapturedElement {
        self.element
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
#[derive(Debug, Clone)]
pub struct UseFocusableProps {
    pub tabindex: Signal<Option<i32>>,
    pub on_focus: EventHandler<FocusEvent>,
    pub on_blur: EventHandler<FocusEvent>,
    pub on_keydown: EventHandler<KeyboardEvent>,
    pub on_keyup: EventHandler<KeyboardEvent>,
    pub element_capture: ElementCaptureAttr,
    /// The [`FocusableContext`]'s description of the element. Not part of the attributes: hooks
    /// rendering `aria-describedby` merge it into theirs, the others render it.
    pub context_aria_describedby: Signal<Option<String>>,
    /// The [`FocusableContext`]'s further attributes (none without a context).
    pub context_attrs: Option<FocusableContextAttrs>,
}

impl IntoAttrs for UseFocusableProps {
    type Attrs = UseFocusableAttrs;

    fn into_attrs(self) -> Self::Attrs {
        let context_attrs = FocusableContextAttr(self.context_attrs);
        (
            Attr(attr::Tabindex, self.tabindex),
            self.on_focus.into_on(ev::focus),
            self.on_blur.into_on(ev::blur),
            self.on_keydown.into_on(ev::keydown),
            self.on_keyup.into_on(ev::keyup),
            self.element_capture,
            context_attrs,
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
    FocusableContextAttr,
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
        is_disabled: disabled,
        auto_focus,
        exclude_from_tab_order,
        on_focus,
        on_blur,
        on_focus_change,
        on_key_down,
        on_key_up,
        shortcuts,
        allow_shortcut_repeats,
    } = input;

    let element = CapturedElement::new();

    // Create the focus handle
    let focus_handle = FocusHandle { element };

    // Read parent-provided context (e.g., from TooltipTrigger).
    let ctx = use_context::<FocusableContext>();

    // Use the focus hook
    let focus = use_focus(UseFocusInput {
        is_disabled: disabled,
        on_focus,
        on_blur,
        on_focus_change,
    });

    // Use the keyboard hook
    let keyboard = use_keyboard(UseKeyboardInput {
        is_disabled: disabled,
        on_key_down,
        on_key_up,
        shortcuts,
        allow_repeats: allow_shortcut_repeats,
        allow_composing: false,
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

    // As react-aria: a disabled element gets none of the context's props.
    let context_aria_describedby = match ctx.as_ref().and_then(|c| c.aria_describedby) {
        Some(describedby) => {
            Signal::derive(move || (!disabled.get()).then(|| describedby.get()).flatten())
        }
        None => Signal::stored(None),
    };
    let context_attrs = ctx
        .as_ref()
        .and_then(|c| c.attrs.clone())
        .filter(|_| !disabled.get_untracked());

    UseFocusableReturn {
        props: UseFocusableProps {
            tabindex: tab_index,
            on_focus,
            on_blur,
            on_keydown,
            on_keyup,
            element_capture,
            context_aria_describedby,
            context_attrs,
        },
        focus_handle,
    }
}
