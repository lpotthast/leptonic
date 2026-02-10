use crate::hooks::{
    link_rel_to_string, use_focus_ring, use_focusable, use_press, FocusHandle, LinkRel, LinkTarget,
    MergedFocusablePressFocusRingAttrs, MergedFocusablePressFocusRingProps, PressEvent,
    UseFocusRingInput, UseFocusRingReturn, UseFocusableInput, UseFocusableReturn, UsePressInput,
    UsePressReturn,
};
use crate::utils::aria::{AriaCurrent, AriaDisabled};
use crate::utils::{ElementCaptureAttr, MergeWith};
use educe::Educe;
use leptos::attr::Attr;
use leptos::oco::Oco;
use leptos::attr;
use reactive_graph::callback::{Callable, Callback};
use reactive_graph::prelude::Get;
use reactive_graph::wrappers::read::Signal;

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/link/src/useLink.ts
//
// ## DEVIATIONS FROM REACT-ARIA
//
// - Client-side router integration is NOT handled at the hook level.
//   Rationale: Leptos router handles client-side navigation at the component
//   level via `<A>`. The hook focuses purely on interaction, ARIA attributes,
//   and focus management, matching its role as a low-level building block.
//
// - `use_focus_ring` is composed in addition to `use_focusable` and `use_press`.
//   Rationale: React-aria handles focus ring visibility at the component level.
//   Leptonic includes it in the hook for consistency with `use_button`.

/// Input parameters for the `use_link` hook.
#[derive(Debug, Clone)]
pub struct UseLinkInput {
    /// The href for the link.
    pub href: Option<String>,

    /// The `target` attribute for the link (e.g. `"_blank"`).
    pub target: Option<LinkTarget>,

    /// The `rel` attribute values for the link.
    pub rel: Vec<LinkRel>,

    /// Whether the link is disabled.
    pub is_disabled: Signal<bool>,

    /// The element type (for non-anchor elements).
    pub element_type: LinkElementType,

    /// Identifies the element as the current item within a set (e.g. navigation).
    pub aria_current: Option<AriaCurrent>,

    /// Optional press callback.
    pub on_press: Option<Callback<PressEvent>>,

    /// Optional callback fired when the press starts.
    pub on_press_start: Option<Callback<PressEvent>>,

    /// Optional callback fired when the press ends.
    pub on_press_end: Option<Callback<PressEvent>>,
}

/// The element type for a link.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LinkElementType {
    /// An anchor (<a>) element.
    #[default]
    Anchor,
    /// A span element styled as a link.
    Span,
    /// A button element styled as a link.
    Button,
}

/// The return value of the `use_link` hook.
#[derive(Clone, Educe)]
#[educe(Debug)]
pub struct UseLinkReturn {
    /// Props for the link element. Call `.to_attrs()` or `.into_attrs()` for view spreading.
    pub props: UseLinkProps,

    /// Shorthand for `props.to_attrs()`.
    #[educe(Debug(ignore))]
    pub link_props: UseLinkAttrs,

    /// Whether the link is disabled.
    pub is_disabled: Signal<bool>,

    /// Whether the link is currently being pressed.
    pub is_pressed: Signal<bool>,

    /// Whether the focus ring should be visible (keyboard navigation only).
    pub is_focus_visible: Signal<bool>,

    /// Handle for programmatically focusing the element.
    pub focus_handle: FocusHandle,
}

/// Props from `use_link` that can be extracted and merged programmatically.
#[derive(Debug, Clone)]
pub struct UseLinkProps {
    pub href: Option<String>,
    pub target: Option<LinkTarget>,
    pub rel: Option<String>,
    pub role: Option<&'static str>,
    pub aria_current: Option<AriaCurrent>,
    pub aria_disabled: Signal<Option<AriaDisabled>>,
    pub merged: MergedFocusablePressFocusRingProps,
}

impl UseLinkProps {
    /// Convert to spreadable attributes for Leptos views, cloning internally.
    #[must_use]
    pub fn to_attrs(&self) -> UseLinkAttrs {
        self.clone().into_attrs()
    }

    /// Convert to spreadable attributes for Leptos views, consuming self.
    #[must_use]
    pub fn into_attrs(self) -> UseLinkAttrs {
        (
            (
                Attr(attr::Href, self.href),
                Attr(attr::Target, self.target.map(|it| it.to_oco())),
                Attr(attr::Rel, self.rel),
                Attr(attr::Role, self.role),
                Attr(attr::AriaCurrent, self.aria_current),
                Attr(attr::AriaDisabled, self.aria_disabled),
            ),
            self.merged.into_attrs(),
        )
    }
}

/// These attributes must be spread onto the target element using the spread syntax `<a {..attrs}/>`.
pub type UseLinkAttrs = (
    (
        Attr<attr::Href, Option<String>>,
        Attr<attr::Target, Option<Oco<'static, str>>>,
        Attr<attr::Rel, Option<String>>,
        Attr<attr::Role, Option<&'static str>>,
        Attr<attr::AriaCurrent, Option<AriaCurrent>>,
        Attr<attr::AriaDisabled, Signal<Option<AriaDisabled>>>,
    ),
    MergedFocusablePressFocusRingAttrs,
);

/// Provides the behavior and accessibility for a link.
///
/// A link allows users to navigate to another page or resource.
/// This hook composes `use_focusable`, `use_press`, and `use_focus_ring`
/// to provide robust interaction handling, focus management, and accessibility.
///
/// # Example
///
/// ```ignore
/// let link = use_link(UseLinkInput {
///     href: Some("https://example.com".to_string()),
///     target: Some("_blank"),
///     rel: vec![LinkRel::NoOpener, LinkRel::NoReferrer],
///     ..Default::default()
/// });
///
/// view! {
///     <a {..link.props.into_attrs()}>
///         "Visit Example"
///     </a>
/// }
/// ```
pub fn use_link(input: UseLinkInput) -> UseLinkReturn {
    let UseLinkInput {
        href,
        target,
        rel,
        is_disabled,
        element_type,
        aria_current,
        on_press,
        on_press_start,
        on_press_end,
    } = input;

    let rel = link_rel_to_string(&rel);

    // Non-anchor elements need role="link".
    let role = match element_type {
        LinkElementType::Anchor => None,
        LinkElementType::Span | LinkElementType::Button => Some("link"),
    };

    let aria_disabled = Signal::derive(move || is_disabled.get().then_some(AriaDisabled::True));

    // Compose sub-hooks.
    let UseFocusableReturn {
        props: focusable_props,
        focus_handle,
    } = use_focusable(UseFocusableInput {
        disabled: is_disabled,
        ..UseFocusableInput::default()
    });

    let UsePressReturn {
        props: press_props,
        is_pressed,
    } = use_press(UsePressInput {
        disabled: is_disabled,
        force_prevent_default: false,
        // Without setting this, Leptos' client-side navigation would not take place.
        allow_propagation: true,
        allow_text_selection_on_press: false,
        should_cancel_on_pointer_exit: false,
        on_press: Callback::new(move |e: PressEvent| {
            if let Some(on_press) = on_press {
                on_press.run(e);
            }
        }),
        on_press_up: None,
        on_press_start,
        on_press_end,
        on_press_change: None,
        on_double_press: None,
        on_long_press_start: None,
        on_long_press: None,
        on_long_press_end: None,
        long_press_threshold: None,
        long_press_accessibility_description: None,
    });

    let UseFocusRingReturn {
        props: focus_ring_props,
        is_focus_visible,
        is_focused: _,
    } = use_focus_ring(UseFocusRingInput {
        disabled: is_disabled,
        ..UseFocusRingInput::default()
    });

    let merged = focusable_props
        .merge_with(press_props)
        .merge_with(focus_ring_props);

    #[cfg(debug_assertions)]
    let merged = MergedFocusablePressFocusRingProps {
        element_capture: merged.element_capture.clone().chain(
            ElementCaptureAttr::new(move |el| {
                super::debug_validate_element_type(element_type, &el);
            }),
        ),
        ..merged
    };

    let props = UseLinkProps {
        href,
        target,
        rel,
        role,
        aria_current,
        aria_disabled,
        merged,
    };

    let link_props = props.to_attrs();

    UseLinkReturn {
        props,
        link_props,
        is_disabled,
        is_pressed,
        is_focus_visible,
        focus_handle,
    }
}
