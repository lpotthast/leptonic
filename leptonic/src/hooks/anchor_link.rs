use educe::Educe;
use leptos::{Attribute, Callable, Callback, IntoAttribute, SignalGet};
use leptos_reactive::{MaybeSignal, Oco};
use leptos_use::{use_document, use_window};
use typed_builder::TypedBuilder;
use wasm_bindgen::JsValue;
use web_sys::ScrollIntoViewOptions;

use crate::utils::{
    aria::*, attributes::Attributes, event_handlers::EventHandlers,
    scroll_behavior::ScrollBehavior, signals::MaybeSignalExt,
};

use super::{use_press, UsePressInput};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Href(Oco<'static, str>);

impl Href {
    pub fn from_str(str: Oco<'static, str>) -> Result<Self, String> {
        if !str.starts_with('#') {
            return Err(format!("Href must start with '#', got: {str}"));
        }
        Ok(Self(str))
    }
}

#[derive(Clone, Educe, TypedBuilder)]
#[educe(Debug)]
pub struct UseAnchorLinkInput {
    /// The anchor link. For example: "#my-anchor". Known to be of the aforementioned format, always starting with a '#'.
    pub(crate) href: Href,

    /// How the browser should scroll to the referenced anchor element. Doe not perform any scrolling when set to None.
    #[builder(default = Some(ScrollBehavior::Smooth))]
    pub(crate) scroll_behavior: Option<ScrollBehavior>,

    /// Wether the link is disabled.
    #[builder(setter(into))]
    pub(crate) disabled: MaybeSignal<bool>,

    /// Description of this anchor for accessibility.
    /// If text is provided in children, this could be omitted.
    /// If no children are provided, this component renders a single `#`,
    /// which should be described using this field.
    #[builder(default = None)]
    pub(crate) description: Option<Oco<'static, str>>,

    /// Links are enforced to have the "press" behavior.
    pub(crate) use_press_input: UsePressInput,
}

#[derive(Debug)]
pub struct UseAnchorLinkReturn {
    /// Spread these props onto your link using the spread syntax: `<foo {..props}>...`
    pub props: UseAnchorLinkProps,
}

#[derive(Educe)]
#[educe(Debug)]
pub struct UseAnchorLinkProps {
    /// These attributes must be spread onto the target element: `<foo {..props.attrs} />`
    pub attrs: Attributes,
    /// These handlers must be spread onto the target element: `<foo {..props.handlers} />`
    pub handlers: EventHandlers,
}

// TODO: Add proper focus behavior!
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
    press_input.on_press = Some(Callback::new(move |e| {
        if !input.disabled.get() {
            if let Some(scroll_behavior) = input.scroll_behavior {
                if let Some(document) = use_document().as_ref() {
                    let el_id = href.0.replace('#', "");
                    if let Some(el) = document.get_element_by_id(el_id.as_str()) {
                        el.scroll_into_view_with_scroll_into_view_options(
                            ScrollIntoViewOptions::new()
                                .behavior(web_sys::ScrollBehavior::from(scroll_behavior)),
                        );
                    } else {
                        tracing::warn!(
                            "AnchorLink could not find anchor (element) with id '{el_id}'."
                        )
                    }
                }
            }
            update_url(&href);
        }
        if let Some(original_on_press) = &original_on_press {
            Callback::call(original_on_press, e);
        }
    }));

    let press = use_press(press_input);

    let href: Href = input.href;
    let mut attrs = Attributes::new()
        .insert("role", AriaRole::Link)
        .insert("href", Attribute::String(href.0));
    if let Some(description) = input.description {
        attrs = attrs.insert("aria-label", Attribute::String(description)); // TODO: Use aria-description instead?
    }
    /*attrs.insert(
        "tabindex",
        input
            .disabled
            .map(|it| match it {
                true => Attribute::Option(None),
                false => Attribute::String(Oco::Borrowed("0")),
            })
            .into_attribute(),
    );
    attrs.insert("disabled", input.disabled.into_attribute());*/
    attrs = attrs
        .insert(
            "aria-disabled",
            input
                .disabled
                .map(|it| match it {
                    true => "true",
                    false => "false",
                })
                .into_attribute(),
        )
        .merge(press.props.attrs);

    UseAnchorLinkReturn {
        props: UseAnchorLinkProps {
            attrs,
            handlers: press.props.handlers,
        },
    }
}
