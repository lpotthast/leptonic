use leptos::attr::custom::{custom_attribute, CustomAttr};
use leptos::ev;
use leptos::ev::{On, SharedEventCallback};
use leptos::prelude::*;
use web_sys::FocusEvent;

use crate::hooks::focus::use_focus::{use_focus, UseFocusInput};
use crate::hooks::focus::use_focus_visible::{
    is_focus_visible, use_focus_visible, UseFocusVisibleInput,
};
use crate::utils::EventHandler;

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/focus/src/useFocusRing.ts

/// Input parameters for the `use_focus_ring` hook.
#[derive(Debug, Clone, Copy)]
pub struct UseFocusRingInput {
    /// Whether the focus ring is disabled.
    pub disabled: Signal<bool>,

    /// Whether to show the focus ring when the element is focused
    /// via any method (not just keyboard). When true, the focus ring
    /// is always visible when focused. When false, only visible during
    /// keyboard navigation.
    pub within: bool,

    /// Whether to auto-focus the element.
    pub auto_focus: bool,

    /// Optional callback when the element receives focus.
    pub on_focus: Option<Callback<FocusEvent>>,

    /// Optional callback when the element loses focus.
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
            on_focus: None,
            on_blur: None,
            on_focus_change: None,
        }
    }
}

/// The return value of the `use_focus_ring` hook.
#[derive(Debug, Clone)]
pub struct UseFocusRingReturn {
    /// Whether the focus ring should be visible.
    pub is_focus_visible: Signal<bool>,

    /// Whether the element is currently focused.
    pub is_focused: Signal<bool>,

    /// Props for programmatic merging. Call `.to_attrs()` or `.into_attrs()` for view spreading.
    pub props: UseFocusRingProps,
}

/// Props from `use_focus_ring` that can be extracted and merged programmatically.
#[derive(Clone, educe::Educe)]
#[educe(Debug)]
pub struct UseFocusRingProps {
    pub on_focus: EventHandler<FocusEvent>,
    pub on_blur: EventHandler<FocusEvent>,
    #[educe(Debug(ignore))]
    pub data_focus_visible: CustomAttr<&'static str, Signal<Option<&'static str>>>,
}

impl UseFocusRingProps {
    /// Convert to spreadable attributes for Leptos views, cloning internally.
    #[must_use]
    pub fn to_attrs(&self) -> UseFocusRingAttrs {
        (
            self.on_focus.to_on(ev::focus),
            self.on_blur.to_on(ev::blur),
            self.data_focus_visible.clone(),
        )
    }

    /// Convert to spreadable attributes for Leptos views, consuming self.
    #[must_use]
    pub fn into_attrs(self) -> UseFocusRingAttrs {
        (
            self.on_focus.into_on(ev::focus),
            self.on_blur.into_on(ev::blur),
            self.data_focus_visible,
        )
    }
}

/// These attributes must be spread onto the target element: `<foo {..attrs} />`
pub type UseFocusRingAttrs = (
    On<ev::focus, SharedEventCallback<FocusEvent>>,
    On<ev::blur, SharedEventCallback<FocusEvent>>,
    CustomAttr<&'static str, Signal<Option<&'static str>>>,
);

/// Determines whether a focus ring should be displayed for an element.
///
/// This hook combines focus state tracking with focus visibility detection.
/// A focus ring should be shown when:
/// 1. The element is focused, AND
/// 2. The user is interacting via keyboard (not mouse/touch)
///
/// This helps maintain accessibility while avoiding visual clutter from
/// focus rings on mouse/touch interactions.
///
/// # Example
///
/// ```ignore
/// let focus_ring = use_focus_ring(UseFocusRingInput::default());
///
/// view! {
///     <button
///         {..focus_ring.attrs}
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
    let (is_focused, set_is_focused) = signal(false);
    let (is_focus_visible_state, set_is_focus_visible) =
        signal(input.auto_focus && is_focus_visible());

    let disabled = input.disabled;
    let user_on_focus = input.on_focus;
    let user_on_blur = input.on_blur;
    let user_on_focus_change = input.on_focus_change;

    // Track focus visible modality
    let focus_visible = use_focus_visible(UseFocusVisibleInput {
        auto_focus: input.auto_focus,
    });

    // Track focus state, calling user callbacks in addition to internal tracking
    let focus = use_focus(UseFocusInput {
        disabled,
        on_focus: Some(Callback::new(move |e| {
            set_is_focused.set(true);
            if let Some(on_focus) = user_on_focus {
                on_focus.run(e);
            }
        })),
        on_blur: Some(Callback::new(move |e| {
            set_is_focused.set(false);
            if let Some(on_blur) = user_on_blur {
                on_blur.run(e);
            }
        })),
        on_focus_change: user_on_focus_change,
    });

    // Compute whether focus ring should be visible
    // If `within` is true, always show when focused
    // Otherwise, only show when focused AND using keyboard
    let within = input.within;

    Effect::new(move |_| {
        let focused = is_focused.get();
        let visible = focus_visible.is_focus_visible.get();

        if within {
            // For focus-within, just check if any descendant is focused
            set_is_focus_visible.set(focused);
        } else {
            // Normal case: show focus ring only during keyboard navigation
            set_is_focus_visible.set(focused && visible);
        }
    });

    // Create a reactive data attribute that is present when focus ring should be shown
    let data_focus_visible = Signal::derive(move || {
        if is_focus_visible_state.get() {
            Some("true")
        } else {
            None
        }
    });

    UseFocusRingReturn {
        is_focus_visible: is_focus_visible_state.into(),
        is_focused: is_focused.into(),
        props: UseFocusRingProps {
            on_focus: focus.props.on_focus,
            on_blur: focus.props.on_blur,
            data_focus_visible: custom_attribute("data-focus-visible", data_focus_visible),
        },
    }
}
