use leptos::attr;
use leptos::attr::{Attr, Attribute};
use leptos::ev;
use leptos::ev::{on, On, SharedEventCallback};
use leptos::prelude::*;
use uuid::Uuid;
use web_sys::{FocusEvent, KeyboardEvent, MouseEvent};

use super::focus::use_focus_ring::{use_focus_ring, UseFocusRingInput, UseFocusRingReturn};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/disclosure/src/useDisclosure.ts

/// Input parameters for the `use_disclosure` hook.
#[derive(Debug, Clone, Copy)]
pub struct UseDisclosureInput {
    /// Whether the disclosure is expanded.
    pub is_expanded: Signal<bool>,

    /// Whether the disclosure is disabled.
    pub is_disabled: Signal<bool>,

    /// Callback when the expanded state changes.
    pub on_expanded_change: Option<Callback<bool>>,
}

impl Default for UseDisclosureInput {
    fn default() -> Self {
        Self {
            is_expanded: Signal::derive(|| false),
            is_disabled: Signal::derive(|| false),
            on_expanded_change: None,
        }
    }
}

/// The return value of the `use_disclosure` hook.
#[derive(Debug, Clone)]
pub struct UseDisclosureReturn {
    /// Props for the disclosure trigger button.
    pub trigger_props: UseDisclosureTriggerAttrs,

    /// Props for the disclosure content panel.
    pub content_props: UseDisclosureContentAttrs,

    /// The ID of the trigger.
    pub trigger_id: String,

    /// The ID of the content.
    pub content_id: String,

    /// Whether the disclosure is expanded.
    pub is_expanded: Signal<bool>,

    /// Toggle the disclosure.
    pub toggle: Callback<()>,

    /// Whether the focus ring should be visible on the trigger (keyboard navigation only).
    pub is_focus_visible: Signal<bool>,
}

/// Attributes for the disclosure trigger button.
pub type UseDisclosureTriggerAttrs = (
    Attr<attr::Id, String>,
    Attr<attr::AriaExpanded, Signal<&'static str>>,
    Attr<attr::AriaControls, String>,
    Attr<attr::AriaDisabled, Signal<&'static str>>,
    On<ev::click, SharedEventCallback<MouseEvent>>,
    On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
    On<ev::focus, SharedEventCallback<FocusEvent>>,
    On<ev::blur, SharedEventCallback<FocusEvent>>,
    attr::custom::CustomAttr<&'static str, Signal<Option<&'static str>>>,
);

/// Attributes for the disclosure content panel.
pub type UseDisclosureContentAttrs = (
    Attr<attr::Id, String>,
    Attr<attr::Role, &'static str>,
    Attr<attr::AriaLabelledby, String>,
    Attr<attr::AriaHidden, Signal<&'static str>>,
);

/// Provides the behavior and accessibility for a disclosure component.
///
/// A disclosure shows or hides content when a trigger button is pressed.
///
/// # Example
///
/// ```ignore
/// let (is_expanded, set_is_expanded) = signal(false);
///
/// let disclosure = use_disclosure(UseDisclosureInput {
///     is_expanded: is_expanded.into(),
///     on_expanded_change: Some(Callback::new(move |expanded| set_is_expanded.set(expanded))),
///     ..Default::default()
/// });
///
/// view! {
///     <button {..disclosure.trigger_props}>
///         {move || if disclosure.is_expanded.get() { "Hide" } else { "Show" }}
///     </button>
///     <div {..disclosure.content_props}>
///         "Hidden content"
///     </div>
/// }
/// ```
pub fn use_disclosure(input: UseDisclosureInput) -> UseDisclosureReturn {
    let base_id = Uuid::new_v4();
    let trigger_id = format!("disclosure-trigger-{base_id}");
    let content_id = format!("disclosure-content-{base_id}");

    let is_expanded = input.is_expanded;
    let is_disabled = input.is_disabled;
    let on_expanded_change = input.on_expanded_change;

    let toggle = Callback::new(move |_| {
        if is_disabled.get_untracked() {
            return;
        }
        if let Some(on_change) = on_expanded_change {
            on_change.run(!is_expanded.get_untracked());
        }
    });

    let aria_expanded = Signal::derive(move || if is_expanded.get() { "true" } else { "false" });

    let aria_disabled = Signal::derive(move || if is_disabled.get() { "true" } else { "false" });

    let aria_hidden = Signal::derive(move || if is_expanded.get() { "false" } else { "true" });

    let handle_click = move |_e: MouseEvent| {
        if is_disabled.get_untracked() {
            return;
        }
        if let Some(on_change) = on_expanded_change {
            on_change.run(!is_expanded.get_untracked());
        }
    };

    let handle_keydown = move |e: KeyboardEvent| {
        if is_disabled.get_untracked() {
            return;
        }

        let key = e.key();
        if key == "Enter" || key == " " {
            e.prevent_default();
            if let Some(on_change) = on_expanded_change {
                on_change.run(!is_expanded.get_untracked());
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

    UseDisclosureReturn {
        trigger_props: (
            Attr(attr::Id, trigger_id.clone()),
            Attr(attr::AriaExpanded, aria_expanded),
            Attr(attr::AriaControls, content_id.clone()),
            Attr(attr::AriaDisabled, aria_disabled),
            on(ev::click, handle_click).into_cloneable(),
            on(ev::keydown, handle_keydown).into_cloneable(),
            on_focus,
            on_blur,
            data_focus_visible,
        ),
        content_props: (
            Attr(attr::Id, content_id.clone()),
            Attr(attr::Role, "region"),
            Attr(attr::AriaLabelledby, trigger_id.clone()),
            Attr(attr::AriaHidden, aria_hidden),
        ),
        trigger_id,
        content_id,
        is_expanded,
        toggle,
        is_focus_visible,
    }
}

/// State for managing disclosure visibility.
#[derive(Clone, Copy)]
pub struct UseDisclosureStateReturn {
    /// Whether the disclosure is expanded.
    pub is_expanded: Signal<bool>,

    /// Expand the disclosure.
    pub expand: Callback<()>,

    /// Collapse the disclosure.
    pub collapse: Callback<()>,

    /// Toggle the disclosure.
    pub toggle: Callback<()>,
}

/// Creates internal state for a disclosure.
pub fn use_disclosure_state(default_expanded: bool) -> UseDisclosureStateReturn {
    let (is_expanded, set_is_expanded) = signal(default_expanded);

    UseDisclosureStateReturn {
        is_expanded: is_expanded.into(),
        expand: Callback::new(move |_| set_is_expanded.set(true)),
        collapse: Callback::new(move |_| set_is_expanded.set(false)),
        toggle: Callback::new(move |_| set_is_expanded.update(|v| *v = !*v)),
    }
}
