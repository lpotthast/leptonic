use educe::Educe;
use leptos::{attr, attr::Attr, oco::Oco, prelude::*};
use leptos_use::{use_document, use_window};
use reactive_graph::callback::{Callable, Callback};
use wasm_bindgen::JsValue;
use web_sys::ScrollIntoViewOptions;

use super::LinkElementType;
use crate::{
    hooks::{
        use_focus_ring, use_focusable, use_press, FocusHandle, IntoAttrs,
        MergedFocusablePressFocusRingAttrs, MergedFocusablePressFocusRingProps, PressEvent,
        UseFocusRingInput, UseFocusRingReturn, UseFocusableInput, UseFocusableReturn,
        UsePressInput, UsePressReturn,
    },
    utils::{aria::*, scroll_behavior::ScrollBehavior, ElementCaptureAttr, MergeWith},
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// No intentional deviations from the react-aria implementation.
//
// =============================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Href(Oco<'static, str>);

impl Href {
    /// # Errors
    ///
    /// Returns an error if the href does not start with `'#'`.
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(str: Oco<'static, str>) -> Result<Self, String> {
        if !str.starts_with('#') {
            return Err(format!("Href must start with '#', got: {str}"));
        }
        Ok(Self(str))
    }
}

#[derive(Clone, Educe)]
#[educe(Debug)]
pub struct UseAnchorLinkInput {
    /// The anchor link. For example: "#my-anchor". Known to be of the aforementioned format, always starting with a '#'.
    pub href: Href,

    /// How the browser should scroll to the referenced anchor element. Does not perform any scrolling when set to None.
    pub scroll_behavior: Option<ScrollBehavior>,

    /// Whether the link is disabled.
    pub disabled: Signal<bool>,

    /// The element type. Default is `Anchor`. Non-anchor elements get `role="link"`.
    pub element_type: LinkElementType,

    /// Description of this anchor for accessibility.
    /// If text is provided in children, this could be omitted.
    /// If no children are provided, this component renders a single `#`,
    /// which should be described using this field.
    pub description: Option<Oco<'static, str>>,

    /// Optional press callback. Called after the anchor scroll and URL update.
    #[educe(Debug(ignore))]
    pub on_press: Option<Callback<PressEvent>>,

    /// Optional callback fired when the press starts.
    #[educe(Debug(ignore))]
    pub on_press_start: Option<Callback<PressEvent>>,

    /// Optional callback fired when the press ends.
    #[educe(Debug(ignore))]
    pub on_press_end: Option<Callback<PressEvent>>,
}

#[derive(Debug)]
pub struct UseAnchorLinkReturn {
    /// Props for programmatic merging. Call `.into_attrs()` for view spreading.
    pub props: UseAnchorLinkProps,

    pub is_pressed: Signal<bool>,

    /// Whether the focus ring should be visible (keyboard navigation only).
    pub is_focus_visible: Signal<bool>,

    /// Handle for programmatically focusing the element.
    pub focus_handle: FocusHandle,
}

/// Props from `use_anchor_link` that can be extracted and merged programmatically.
#[derive(Debug)]
pub struct UseAnchorLinkProps {
    pub href: Oco<'static, str>,
    pub role: Option<&'static str>,
    pub aria_label: Option<Oco<'static, str>>,
    pub aria_disabled: Signal<Option<AriaDisabled>>,
    pub merged: MergedFocusablePressFocusRingProps,
}

impl IntoAttrs for UseAnchorLinkProps {
    type Attrs = UseAnchorLinkAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            (
                Attr(attr::Href, self.href),
                Attr(attr::Role, self.role),
                Attr(attr::AriaLabel, self.aria_label),
                Attr(attr::AriaDisabled, self.aria_disabled),
            ),
            self.merged.into_attrs(),
        )
    }
}

pub type UseAnchorLinkAttrs = (
    (
        Attr<attr::Href, Oco<'static, str>>,
        Attr<attr::Role, Option<&'static str>>,
        Attr<attr::AriaLabel, Option<Oco<'static, str>>>,
        Attr<attr::AriaDisabled, Signal<Option<AriaDisabled>>>,
    ),
    MergedFocusablePressFocusRingAttrs,
);

/// Update the browser URL hash without a page reload.
fn update_url(href: &Href) {
    if let Some(window) = use_window().as_ref() {
        if let Ok(history) = window.history() {
            if let Err(e) =
                history.replace_state_with_url(&JsValue::null(), "", Some(href.0.as_str()))
            {
                tracing::warn!("Failed to update URL via history.replaceState: {e:?}");
            }
        } else if let Err(e) = window.location().set_hash(href.0.as_str()) {
            tracing::warn!("Failed to update URL hash: {e:?}");
        }
    }
}

/// Scroll to the element referenced by `href` using the given scroll behavior.
fn scroll_to_anchor(href: &Href, scroll_behavior: ScrollBehavior) {
    if let Some(document) = use_document().as_ref() {
        let el_id = href.0.replace('#', "");
        if let Some(el) = document.get_element_by_id(el_id.as_str()) {
            el.scroll_into_view_with_scroll_into_view_options(&{
                let opts = ScrollIntoViewOptions::new();
                opts.set_behavior(web_sys::ScrollBehavior::from(scroll_behavior));
                opts
            });
        } else {
            tracing::warn!("AnchorLink could not find anchor (element) with id '{el_id}'.");
        }
    }
}

/// Provides the behavior and accessibility for an in-page anchor link.
///
/// Composes `use_focusable`, `use_press`, and `use_focus_ring` to provide
/// robust interaction handling, focus management, and accessibility for
/// anchor links that scroll to a target element on the same page.
pub fn use_anchor_link(input: UseAnchorLinkInput) -> UseAnchorLinkReturn {
    let UseAnchorLinkInput {
        href,
        scroll_behavior,
        disabled,
        element_type,
        description,
        on_press,
        on_press_start,
        on_press_end,
    } = input;

    let href_for_scroll = href.clone();
    let user_on_press = on_press;
    let on_press = Callback::new(move |e: PressEvent| {
        if !disabled.get() {
            if let Some(behavior) = scroll_behavior {
                scroll_to_anchor(&href_for_scroll, behavior);
            }
            update_url(&href_for_scroll);
        }
        if let Some(cb) = user_on_press {
            cb.run(e);
        }
    });

    // Non-anchor elements need role="link".
    let role = match element_type {
        LinkElementType::Anchor => None,
        LinkElementType::Span | LinkElementType::Button => Some("link"),
    };

    // Compose sub-hooks.
    let UseFocusableReturn {
        props: focusable_props,
        focus_handle,
    } = use_focusable(UseFocusableInput {
        disabled,
        ..UseFocusableInput::default()
    });

    let UsePressReturn {
        props: press_props,
        is_pressed,
    } = use_press(UsePressInput {
        disabled,
        // Anchor links always need prevent_default for custom scroll behavior.
        force_prevent_default: true,
        force_propagation: false,
        allow_text_selection_on_press: false,
        should_cancel_on_pointer_exit: false,
        prevent_focus_on_press: false,
        force_is_pressed: None,
        on_press,
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
        disabled,
        ..UseFocusRingInput::default()
    });

    let merged = focusable_props
        .merge_with(press_props)
        .merge_with(focus_ring_props);

    #[cfg(debug_assertions)]
    let merged = MergedFocusablePressFocusRingProps {
        element_capture: merged
            .element_capture
            .clone()
            .chain(ElementCaptureAttr::new(move |el| {
                super::debug_validate_element_type(element_type, &el);
            })),
        ..merged
    };

    UseAnchorLinkReturn {
        props: UseAnchorLinkProps {
            href: href.0,
            role,
            aria_label: description,
            aria_disabled: Signal::derive(move || disabled.get().then_some(AriaDisabled::True)),
            merged,
        },
        is_pressed,
        is_focus_visible,
        focus_handle,
    }
}
