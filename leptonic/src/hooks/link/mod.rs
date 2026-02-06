use leptos::attr;
use leptos::attr::{Attr, Attribute};
use leptos::ev;
use leptos::ev::{on, On, SharedEventCallback};
use leptos::prelude::*;
use web_sys::{FocusEvent, KeyboardEvent, MouseEvent};

use super::focus::use_focus_ring::{use_focus_ring, UseFocusRingInput, UseFocusRingReturn};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/link/src/useLink.ts

/// Input parameters for the `use_link` hook.
#[derive(Debug, Clone)]
pub struct UseLinkInput {
    /// The href for the link.
    pub href: Option<String>,

    /// Whether the link opens in a new tab.
    pub is_external: bool,

    /// Whether the link is disabled.
    pub is_disabled: Signal<bool>,

    /// The element type (for non-anchor elements).
    pub element_type: LinkElementType,

    /// Callback when the link is pressed.
    pub on_press: Option<Callback<()>>,
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

impl Default for UseLinkInput {
    fn default() -> Self {
        Self {
            href: None,
            is_external: false,
            is_disabled: Signal::derive(|| false),
            element_type: LinkElementType::Anchor,
            on_press: None,
        }
    }
}

/// The return value of the `use_link` hook.
#[derive(Debug, Clone)]
pub struct UseLinkReturn {
    /// Props for the link element.
    pub link_props: UseLinkAttrs,

    /// Whether the link is disabled.
    pub is_disabled: Signal<bool>,

    /// Whether the focus ring should be visible (keyboard navigation only).
    pub is_focus_visible: Signal<bool>,
}

/// Attributes for the link element.
pub type UseLinkAttrs = (
    Attr<attr::Href, Option<String>>,
    Attr<attr::Target, Option<&'static str>>,
    Attr<attr::Rel, Option<&'static str>>,
    Attr<attr::Role, Option<&'static str>>,
    Attr<attr::Tabindex, &'static str>,
    Attr<attr::AriaDisabled, Signal<&'static str>>,
    attr::custom::CustomAttr<&'static str, Signal<Option<&'static str>>>,
    On<ev::click, SharedEventCallback<MouseEvent>>,
    On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
    On<ev::focus, SharedEventCallback<FocusEvent>>,
    On<ev::blur, SharedEventCallback<FocusEvent>>,
);

/// Provides the behavior and accessibility for a link.
///
/// A link allows users to navigate to another page or resource.
///
/// # Example
///
/// ```ignore
/// let link = use_link(UseLinkInput {
///     href: Some("https://example.com".to_string()),
///     is_external: true,
///     on_press: Some(Callback::new(|_| { /* track click */ })),
///     ..Default::default()
/// });
///
/// view! {
///     <a {..link.link_props}>
///         "Visit Example"
///     </a>
/// }
/// ```
#[allow(clippy::needless_pass_by_value)]
pub fn use_link(input: UseLinkInput) -> UseLinkReturn {
    let is_disabled = input.is_disabled;
    let on_press = input.on_press;
    let href = input.href.clone();
    let is_external = input.is_external;
    let element_type = input.element_type;

    // External links should open in new tab with security attributes
    let target = if is_external { Some("_blank") } else { None };
    let rel = if is_external {
        Some("noopener noreferrer")
    } else {
        None
    };

    // Non-anchor elements need role="link"
    let role = match element_type {
        LinkElementType::Anchor => None,
        LinkElementType::Span | LinkElementType::Button => Some("link"),
    };

    let aria_disabled = Signal::derive(move || if is_disabled.get() { "true" } else { "false" });

    let handle_click = move |e: MouseEvent| {
        if is_disabled.get_untracked() {
            e.prevent_default();
            return;
        }
        if let Some(on_press) = on_press {
            on_press.run(());
        }
    };

    let handle_keydown = move |e: KeyboardEvent| {
        if is_disabled.get_untracked() {
            return;
        }

        // For non-anchor elements, handle Enter/Space
        if element_type != LinkElementType::Anchor {
            let key = e.key();
            if key == "Enter" || key == " " {
                e.prevent_default();
                if let Some(on_press) = on_press {
                    on_press.run(());
                }
            }
        }
    };

    let UseFocusRingReturn {
        props: focus_ring_props,
        is_focus_visible,
        is_focused: _,
    } = use_focus_ring(UseFocusRingInput {
        disabled: is_disabled,
        within: false,
        auto_focus: false,
        on_focus: None,
        on_blur: None,
        on_focus_change: None,
    });
    let (on_focus, on_blur, data_focus_visible) = focus_ring_props.into_attrs();

    UseLinkReturn {
        link_props: (
            Attr(attr::Href, href),
            Attr(attr::Target, target),
            Attr(attr::Rel, rel),
            Attr(attr::Role, role),
            Attr(attr::Tabindex, "0"),
            Attr(attr::AriaDisabled, aria_disabled),
            data_focus_visible,
            on(ev::click, handle_click).into_cloneable(),
            on(ev::keydown, handle_keydown).into_cloneable(),
            on_focus,
            on_blur,
        ),
        is_disabled,
        is_focus_visible,
    }
}
