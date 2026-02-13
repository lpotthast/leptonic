use leptos::{
    attr,
    attr::{
        custom::{custom_attribute, CustomAttr},
        Attr,
    },
    ev,
    ev::{On, SharedEventCallback},
    prelude::*,
};
use uuid::Uuid;
use web_sys::{FocusEvent, KeyboardEvent, MouseEvent};

use crate::{
    hooks::{
        focus::use_focus_ring::{use_focus_ring, UseFocusRingInput, UseFocusRingReturn},
        IntoAttrs,
    },
    utils::{
        aria::{AriaDisabled, AriaExpanded, AriaHidden},
        EventHandler,
    },
};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/disclosure/src/useDisclosure.ts

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// No intentional deviations from the react-aria implementation.
//
// =============================================================================

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
#[derive(Debug)]
pub struct UseDisclosureReturn {
    /// Props for the disclosure trigger button.
    pub trigger_props: UseDisclosureTriggerProps,

    /// Props for the disclosure content panel.
    pub content_props: UseDisclosureContentProps,

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

/// Props from `use_disclosure` for the trigger that can be extracted and merged programmatically.
#[derive(Debug)]
pub struct UseDisclosureTriggerProps {
    pub id: String,
    pub aria_expanded: Signal<Option<AriaExpanded>>,
    pub aria_controls: String,
    pub aria_disabled: Signal<Option<AriaDisabled>>,
    pub on_click: EventHandler<MouseEvent>,
    pub on_keydown: EventHandler<KeyboardEvent>,
    pub on_focus: EventHandler<FocusEvent>,
    pub on_blur: EventHandler<FocusEvent>,
    pub on_focusin: EventHandler<FocusEvent>,
    pub on_focusout: EventHandler<FocusEvent>,
    pub data_focus_visible: Signal<Option<&'static str>>,
}

impl IntoAttrs for UseDisclosureTriggerProps {
    type Attrs = UseDisclosureTriggerAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Id, self.id),
            Attr(attr::AriaExpanded, self.aria_expanded),
            Attr(attr::AriaControls, self.aria_controls),
            Attr(attr::AriaDisabled, self.aria_disabled),
            self.on_click.into_on(ev::click),
            self.on_keydown.into_on(ev::keydown),
            self.on_focus.into_on(ev::focus),
            self.on_blur.into_on(ev::blur),
            self.on_focusin.into_on(ev::focusin),
            self.on_focusout.into_on(ev::focusout),
            custom_attribute("data-focus-visible", self.data_focus_visible),
        )
    }
}

/// Attributes for the disclosure trigger button.
pub type UseDisclosureTriggerAttrs = (
    Attr<attr::Id, String>,
    Attr<attr::AriaExpanded, Signal<Option<AriaExpanded>>>,
    Attr<attr::AriaControls, String>,
    Attr<attr::AriaDisabled, Signal<Option<AriaDisabled>>>,
    On<ev::click, SharedEventCallback<MouseEvent>>,
    On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
    On<ev::focus, SharedEventCallback<FocusEvent>>,
    On<ev::blur, SharedEventCallback<FocusEvent>>,
    On<ev::focusin, SharedEventCallback<FocusEvent>>,
    On<ev::focusout, SharedEventCallback<FocusEvent>>,
    CustomAttr<&'static str, Signal<Option<&'static str>>>,
);

/// Props from `use_disclosure` for the content panel that can be extracted and merged programmatically.
#[derive(Debug)]
pub struct UseDisclosureContentProps {
    pub id: String,
    pub role: &'static str,
    pub aria_labelledby: String,
    pub aria_hidden: Signal<Option<AriaHidden>>,
}

impl IntoAttrs for UseDisclosureContentProps {
    type Attrs = UseDisclosureContentAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Id, self.id),
            Attr(attr::Role, self.role),
            Attr(attr::AriaLabelledby, self.aria_labelledby),
            Attr(attr::AriaHidden, self.aria_hidden),
        )
    }
}

/// Attributes for the disclosure content panel.
pub type UseDisclosureContentAttrs = (
    Attr<attr::Id, String>,
    Attr<attr::Role, &'static str>,
    Attr<attr::AriaLabelledby, String>,
    Attr<attr::AriaHidden, Signal<Option<AriaHidden>>>,
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
    let UseDisclosureInput {
        is_expanded,
        is_disabled: disabled,
        on_expanded_change,
    } = input;

    let base_id = Uuid::new_v4();
    let trigger_id = format!("disclosure-trigger-{base_id}");
    let content_id = format!("disclosure-content-{base_id}");

    let toggle = Callback::new(move |_| {
        if disabled.get_untracked() {
            return;
        }
        if let Some(on_change) = on_expanded_change {
            on_change.run(!is_expanded.get_untracked());
        }
    });

    let aria_expanded = Signal::derive(move || Some(AriaExpanded::from(is_expanded.get())));

    let aria_disabled = Signal::derive(move || disabled.get().then_some(AriaDisabled::True));

    let aria_hidden = Signal::derive(move || (!is_expanded.get()).then_some(AriaHidden::True));

    let handle_click = move |_e: MouseEvent| {
        if disabled.get_untracked() {
            return;
        }
        if let Some(on_change) = on_expanded_change {
            on_change.run(!is_expanded.get_untracked());
        }
    };

    let handle_keydown = move |e: KeyboardEvent| {
        if disabled.get_untracked() {
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
        disabled,
        within: false,
        auto_focus: false,
        is_text_input: false,
        on_focus: None,
        on_blur: None,
        on_focus_change: None,
    });

    UseDisclosureReturn {
        trigger_props: UseDisclosureTriggerProps {
            id: trigger_id.clone(),
            aria_expanded,
            aria_controls: content_id.clone(),
            aria_disabled,
            on_click: EventHandler::new(handle_click),
            on_keydown: EventHandler::new(handle_keydown),
            on_focus: focus_ring_props.on_focus,
            on_blur: focus_ring_props.on_blur,
            on_focusin: focus_ring_props.on_focusin,
            on_focusout: focus_ring_props.on_focusout,
            data_focus_visible: focus_ring_props.data_focus_visible,
        },
        content_props: UseDisclosureContentProps {
            id: content_id.clone(),
            role: "region",
            aria_labelledby: trigger_id.clone(),
            aria_hidden,
        },
        trigger_id,
        content_id,
        is_expanded,
        toggle,
        is_focus_visible,
    }
}
