use educe::Educe;
use leptos::attr::custom::CustomAttr;
use leptos::attr::Attr;
use leptos::ev::{On, SharedEventCallback};
use leptos::oco::Oco;
use leptos::prelude::*;
use leptos::{attr, ev};
use leptos_use::{use_document, use_window};
use wasm_bindgen::JsValue;
use web_sys::{FocusEvent, KeyboardEvent, MouseEvent, PointerEvent, ScrollIntoViewOptions};

use crate::utils::EventHandler;
use crate::utils::{aria::*, scroll_behavior::ScrollBehavior};

use super::focus::use_focus_ring::{use_focus_ring, UseFocusRingInput};
use super::{use_press, UseFocusRingReturn, UsePressInput, UsePressReturn};

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

    /// How the browser should scroll to the referenced anchor element. Doe not perform any scrolling when set to None.
    pub scroll_behavior: Option<ScrollBehavior>,

    /// Whether the link is disabled.
    pub disabled: Signal<bool>,

    /// Description of this anchor for accessibility.
    /// If text is provided in children, this could be omitted.
    /// If no children are provided, this component renders a single `#`,
    /// which should be described using this field.
    pub description: Option<Oco<'static, str>>,

    /// Links are enforced to have the "press" behavior.
    pub use_press_input: UsePressInput,
}

#[derive(Debug)]
pub struct UseAnchorLinkReturn {
    /// Props for programmatic merging. Call `.to_attrs()` or `.into_attrs()` for view spreading.
    pub props: UseAnchorLinkProps,

    pub is_pressed: Signal<bool>,

    /// Whether the focus ring should be visible (keyboard navigation only).
    pub is_focus_visible: Signal<bool>,
}

/// Props from `use_anchor_link` that can be extracted and merged programmatically.
#[derive(Clone, Educe)]
#[educe(Debug)]
pub struct UseAnchorLinkProps {
    pub role: &'static str,
    pub hreflang: Oco<'static, str>,
    pub aria_label: Option<Oco<'static, str>>,
    pub aria_disabled: Signal<&'static str>,
    pub on_keydown: EventHandler<KeyboardEvent>,
    pub on_click: EventHandler<MouseEvent>,
    pub on_pointerdown: EventHandler<PointerEvent>,
    pub on_focus: EventHandler<FocusEvent>,
    pub on_blur: EventHandler<FocusEvent>,
    pub on_focusin: EventHandler<FocusEvent>,
    pub on_focusout: EventHandler<FocusEvent>,
    #[educe(Debug(ignore))]
    pub data_focus_visible: CustomAttr<&'static str, Signal<Option<&'static str>>>,
}

impl UseAnchorLinkProps {
    /// Convert to spreadable attributes for Leptos views, cloning internally.
    #[must_use]
    pub fn to_attrs(&self) -> UseAnchorLinkAttrs {
        (
            Attr(attr::Role, self.role),
            Attr(attr::Hreflang, self.hreflang.clone()),
            Attr(attr::AriaLabel, self.aria_label.clone()),
            Attr(attr::AriaDisabled, self.aria_disabled),
            self.on_keydown.to_on(ev::keydown),
            self.on_click.to_on(ev::click),
            self.on_pointerdown.to_on(ev::pointerdown),
            self.on_focus.to_on(ev::focus),
            self.on_blur.to_on(ev::blur),
            self.on_focusin.to_on(ev::focusin),
            self.on_focusout.to_on(ev::focusout),
            self.data_focus_visible.clone(),
        )
    }

    /// Convert to spreadable attributes for Leptos views, consuming self.
    #[must_use]
    pub fn into_attrs(self) -> UseAnchorLinkAttrs {
        (
            Attr(attr::Role, self.role),
            Attr(attr::Hreflang, self.hreflang),
            Attr(attr::AriaLabel, self.aria_label),
            Attr(attr::AriaDisabled, self.aria_disabled),
            self.on_keydown.into_on(ev::keydown),
            self.on_click.into_on(ev::click),
            self.on_pointerdown.into_on(ev::pointerdown),
            self.on_focus.into_on(ev::focus),
            self.on_blur.into_on(ev::blur),
            self.on_focusin.into_on(ev::focusin),
            self.on_focusout.into_on(ev::focusout),
            self.data_focus_visible,
        )
    }
}

pub type UseAnchorLinkAttrs = (
    Attr<attr::Role, &'static str>,
    Attr<attr::Hreflang, Oco<'static, str>>,
    Attr<attr::AriaLabel, Option<Oco<'static, str>>>,
    Attr<attr::AriaDisabled, Signal<&'static str>>,
    On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
    On<ev::click, SharedEventCallback<MouseEvent>>,
    On<ev::pointerdown, SharedEventCallback<PointerEvent>>,
    On<ev::focus, SharedEventCallback<FocusEvent>>,
    On<ev::blur, SharedEventCallback<FocusEvent>>,
    On<ev::focusin, SharedEventCallback<FocusEvent>>,
    On<ev::focusout, SharedEventCallback<FocusEvent>>,
    CustomAttr<&'static str, Signal<Option<&'static str>>>,
);

// TODO: Add proper focus behavior!
/// # Panics
///
/// Panics if the browser history or location API calls fail.
pub fn use_anchor_link(input: UseAnchorLinkInput) -> UseAnchorLinkReturn {
    // Note: expects href to look like "#foo".
    let update_url = move |href: &Href| {
        if let Some(window) = use_window().as_ref() {
            if let Ok(history) = window.history() {
                history
                    .replace_state_with_url(&JsValue::null(), "", Some(href.0.as_str()))
                    .unwrap();
            } else {
                window.location().set_hash(href.0.as_str()).unwrap();
            }
        }
    };

    let mut press_input = input.use_press_input;

    let href: Href = input.href.clone();
    let original_on_press = press_input.on_press;
    press_input.on_press = Callback::new(move |e| {
        if !input.disabled.get() {
            if let Some(scroll_behavior) = input.scroll_behavior {
                if let Some(document) = use_document().as_ref() {
                    let el_id = href.0.replace('#', "");
                    if let Some(el) = document.get_element_by_id(el_id.as_str()) {
                        el.scroll_into_view_with_scroll_into_view_options(&{
                            let opts = ScrollIntoViewOptions::new();
                            opts.set_behavior(web_sys::ScrollBehavior::from(scroll_behavior));
                            opts
                        });
                    } else {
                        tracing::warn!(
                            "AnchorLink could not find anchor (element) with id '{el_id}'."
                        );
                    }
                }
            }
            update_url(&href);
        }
        original_on_press.run(e);
    });

    let UsePressReturn {
        props: press_props,
        is_pressed,
    } = use_press(press_input);

    let UseFocusRingReturn {
        props: focus_ring_props,
        is_focus_visible,
        is_focused: _,
    } = use_focus_ring(UseFocusRingInput {
        disabled: input.disabled,
        within: false,
        auto_focus: false,
        on_focus: None,
        on_blur: None,
        on_focus_change: None,
    });

    let href: Href = input.href;

    UseAnchorLinkReturn {
        props: UseAnchorLinkProps {
            role: AriaRole::Link.into_attribute_value(),
            hreflang: href.0,
            aria_label: input.description,
            aria_disabled: Signal::derive(move || {
                if input.disabled.get() {
                    "true"
                } else {
                    "false"
                }
            }),
            on_keydown: press_props.on_keydown,
            on_click: press_props.on_click,
            on_pointerdown: press_props.on_pointerdown,
            on_focus: focus_ring_props.handle_focus,
            on_blur: focus_ring_props.handle_blur,
            on_focusin: focus_ring_props.handle_focusin,
            on_focusout: focus_ring_props.handle_focusout,
            data_focus_visible: focus_ring_props.data_focus_visible,
        },
        is_pressed,
        is_focus_visible,
    }
}
