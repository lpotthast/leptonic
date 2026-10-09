// Upstream: react-aria/src/focus/useFocusRing.ts @ 99e6102368
// Upstream: react-aria-components/test/ShadowDOMFocus.browser.test.tsx @ 99e6102368
use leptos::{
    attr::custom::{CustomAttr, custom_attribute},
    ev,
    prelude::*,
};
use web_sys::FocusEvent;

use crate::{
    EventHandler, IntoAttrs, OnEvent,
    hooks::focus::{
        use_focus::{UseFocusInput, use_focus},
        use_focus_within::{FocusWithinEvent, UseFocusWithinInput, use_focus_within},
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - `target: FocusRingTarget` instead of `within: bool`.
// - No `auto_focus`: react-aria's `autoFocus` has no effect on `useFocusRing` (the value it
//   starts with is replaced when the element gets focus).
//
// ## ADDITIONS
// - `is_disabled`, `on_focus`, `on_blur` and `on_focus_change` are forwarded to the underlying
//   `use_focus`/`use_focus_within`, so callers needn't add a separate `use_focus`.
// - The props render `data-focus-visible` for CSS-only styling (react-aria returns only
//   `isFocusVisible`).
//
// =============================================================================

/// What a focus ring tracks.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum FocusRingTarget {
    /// The element itself (`focus`/`blur` events): the ring shows while the element has focus.
    #[default]
    Element,
    /// The element's subtree (`focusin`/`focusout`, through `use_focus_within`): the ring shows
    /// while any descendant has focus, e.g. for form groups or toolbars.
    Within,
}

/// Input parameters for the `use_focus_ring` hook.
#[derive(Debug, Clone, Copy, Default)]
pub struct UseFocusRingInput {
    /// Whether the focus ring is disabled.
    pub is_disabled: Signal<bool>,

    /// What the ring tracks: the element itself (default) or focus within its subtree.
    pub target: FocusRingTarget,

    /// Whether the element is a text input. When `true`, only Tab/Escape keys
    /// trigger focus-visible; other keyboard events are suppressed. This is
    /// used for compound text-input components (e.g., a date picker where focus
    /// is on a button but the component should use text-input focus rules).
    pub is_text_input: bool,

    /// Optional callback when the element receives focus (or focus enters, for
    /// [`FocusRingTarget::Within`]).
    pub on_focus: Option<Callback<FocusEvent>>,

    /// Optional callback when the element loses focus (or focus leaves, for
    /// [`FocusRingTarget::Within`]).
    pub on_blur: Option<Callback<FocusEvent>>,

    /// Optional callback when focus state changes.
    pub on_focus_change: Option<Callback<bool>>,
}

/// The return value of the `use_focus_ring` hook.
#[derive(Debug)]
pub struct UseFocusRingReturn {
    /// Whether the focus ring should be visible.
    pub is_focus_visible: Signal<bool>,

    /// Whether the element is currently focused (or has focus within, for [`FocusRingTarget::Within`]).
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
    OnEvent<ev::focus>,
    OnEvent<ev::blur>,
    OnEvent<ev::focusin>,
    OnEvent<ev::focusout>,
    CustomAttr<&'static str, Signal<Option<&'static str>>>,
);

/// Determines whether a focus ring should be displayed for an element.
///
/// This hook combines focus state tracking with focus visibility detection.
/// A focus ring should be shown when:
/// 1. The element is focused (or has focus within, for [`FocusRingTarget::Within`]), AND
/// 2. The user is interacting via keyboard (not mouse/touch)
///
/// This helps maintain accessibility while avoiding visual clutter from
/// focus rings on mouse/touch interactions.
///
/// [`FocusRingTarget::Element`] (default) tracks focus via `focus`/`blur` events on the element
/// itself, [`FocusRingTarget::Within`] via `focusin`/`focusout` events on its subtree.
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
        is_disabled: disabled,
        target,
        is_text_input,
        on_focus,
        on_blur,
        on_focus_change,
    } = input;

    let (focused, set_focused) = signal(false);
    // Disabled, the focus handlers are off: the element no longer counts as focused (it would stay
    // so until the next blur after enabling).
    Effect::new(move || {
        if disabled.get() {
            set_focused.set(false);
        }
    });

    // Whether focus is visible, from the modality when the element got focus (react-aria's
    // `onFocusChange` reads `isFocusVisible()`), then following the change notifications. Read
    // only while focused (react-aria: `useFocusVisibleListener` with `enabled: isFocused`).
    #[cfg(not(feature = "ssr"))]
    let (focus_visible, mark_focused) = {
        use crate::hooks::focus::use_focus_visible::{FocusState, FocusVisibleSince, ListenerKind};

        crate::hooks::focus::use_focus_visible::track_interaction_modality();
        let kind = ListenerKind::from_is_text_input(is_text_input);
        let since = StoredValue::new(None::<FocusVisibleSince>);
        let changes = FocusState::get().changes.clone();
        let focus_visible = Memo::new(move |_| {
            focused.get()
                && since
                    .get_value()
                    .is_some_and(|since| since.visible(&changes))
        });
        let mark_focused = move |is_focused: bool| {
            if is_focused {
                let state = FocusState::get();
                let _ = since.try_set_value(Some(FocusVisibleSince::now(
                    state,
                    kind,
                    state.is_focus_visible(),
                )));
            }
            set_focused.set(is_focused);
        };
        (Signal::from(focus_visible), mark_focused)
    };
    #[cfg(feature = "ssr")]
    let (focus_visible, mark_focused) = {
        let _ = is_text_input;
        (Signal::stored(false), move |is_focused: bool| {
            set_focused.set(is_focused);
        })
    };

    let (handle_focus, handle_blur, handle_focusin, handle_focusout) = track_focus(
        target,
        on_focus,
        on_blur,
        on_focus_change,
        disabled,
        mark_focused,
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
    target: FocusRingTarget,
    on_focus: Option<Callback<FocusEvent>>,
    on_blur: Option<Callback<FocusEvent>>,
    on_focus_change: Option<Callback<bool>>,
    disabled: Signal<bool>,
    set_focused: impl Fn(bool) + Copy + Send + Sync + 'static,
) -> (
    EventHandler<FocusEvent>,
    EventHandler<FocusEvent>,
    EventHandler<FocusEvent>,
    EventHandler<FocusEvent>,
) {
    if target == FocusRingTarget::Within {
        // Track focus within the element's subtree.
        let focus_within = use_focus_within(UseFocusWithinInput {
            is_disabled: disabled,
            on_focus_within: Some(Callback::new(move |e: FocusWithinEvent| {
                set_focused(true);
                if let Some(on_focus) = on_focus {
                    on_focus.run(e.event);
                }
            })),
            on_blur_within: Some(Callback::new(move |e: FocusWithinEvent| {
                set_focused(false);
                if let Some(on_blur) = on_blur {
                    on_blur.try_run(e.event);
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
            is_disabled: disabled,
            on_focus: Some(Callback::new(move |e| {
                set_focused(true);
                if let Some(on_focus) = on_focus {
                    on_focus.run(e);
                }
            })),
            on_blur: Some(Callback::new(move |e| {
                set_focused(false);
                if let Some(on_blur) = on_blur {
                    on_blur.try_run(e);
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
