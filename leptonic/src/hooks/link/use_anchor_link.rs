// No upstream: links to an element on the same page, scrolling it into view and updating the URL
// fragment (react-aria leaves in-page anchors to the browser).
use leptos::{oco::Oco, prelude::*};
use leptos_use::{use_document, use_window};
use wasm_bindgen::JsValue;
use web_sys::{MouseEvent, ScrollIntoViewOptions};

use crate::{
    hooks::{PressEvent, UseLinkInput, UseLinkReturn, use_link},
    utils::{EventHandler, scroll_behavior::ScrollBehavior},
};

/// The target of an anchor link: an element on the current page, addressed by the URL fragment
/// (`#id`). Converts from strings with or without the leading `#`: `"#section"` and `"section"`
/// are the same target.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Href(Oco<'static, str>);

impl Href {
    /// The target with the id `fragment` (a leading `#` is optional).
    pub fn new(fragment: impl Into<Oco<'static, str>>) -> Self {
        let fragment = fragment.into();
        if fragment.starts_with('#') {
            Self(fragment)
        } else {
            Self(Oco::Owned(format!("#{fragment}")))
        }
    }

    /// The href, starting with `#`.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// The id of the target element (the href without its `#`).
    pub fn fragment(&self) -> &str {
        &self.0[1..]
    }
}

impl From<&'static str> for Href {
    fn from(fragment: &'static str) -> Self {
        Self::new(fragment)
    }
}

impl From<String> for Href {
    fn from(fragment: String) -> Self {
        Self::new(fragment)
    }
}

impl From<Oco<'static, str>> for Href {
    fn from(fragment: Oco<'static, str>) -> Self {
        Self::new(fragment)
    }
}

impl std::fmt::Display for Href {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

/// Input of [`use_anchor_link`].
#[derive(Debug, Clone)]
pub struct UseAnchorLinkInput {
    /// The element to link to.
    pub href: Href,

    /// How to scroll to the element. `None`: no scrolling, only the URL fragment changes.
    pub scroll_behavior: Option<ScrollBehavior>,

    /// The link's other settings. Its `href` is replaced by the anchor's; its `on_press` runs
    /// after scrolling.
    pub link: UseLinkInput,
}

impl UseAnchorLinkInput {
    /// A link to `href`, scrolling smoothly by default.
    pub fn new(href: impl Into<Href>) -> Self {
        Self {
            href: href.into(),
            scroll_behavior: Some(ScrollBehavior::default()),
            link: UseLinkInput::default(),
        }
    }
}

/// Update the browser URL hash without a page reload.
fn update_url(href: &Href) {
    if let Some(window) = use_window().as_ref() {
        if let Ok(history) = window.history() {
            // Keep the entry's state (routers keep theirs there).
            let state = history.state().unwrap_or(JsValue::NULL);
            if let Err(e) = history.replace_state_with_url(&state, "", Some(href.as_str())) {
                tracing::warn!("Failed to update URL via history.replaceState: {e:?}");
            }
        } else if let Err(e) = window.location().set_hash(href.as_str()) {
            tracing::warn!("Failed to update URL hash: {e:?}");
        }
    }
}

/// Scroll to the element referenced by `href` using the given scroll behavior.
fn scroll_to_anchor(href: &Href, scroll_behavior: ScrollBehavior) {
    if let Some(document) = use_document().as_ref() {
        let el_id = href.fragment();
        if let Some(el) = document.get_element_by_id(el_id) {
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

/// A link to an element on the same page: pressing it scrolls the element into view and
/// replaces the URL fragment (without a history entry or the browser's jump).
pub fn use_anchor_link(input: UseAnchorLinkInput) -> UseLinkReturn {
    let UseAnchorLinkInput {
        href,
        scroll_behavior,
        link,
    } = input;
    let target = StoredValue::new(href.clone());
    let user_on_press = link.on_press;
    let mut anchor = use_link(UseLinkInput {
        href: Signal::stored(Some(href.as_str().to_owned())),
        on_press: Some(Callback::new(move |e: PressEvent| {
            target.with_value(|href| {
                if let Some(behavior) = scroll_behavior {
                    scroll_to_anchor(href, behavior);
                }
                update_url(href);
            });
            if let Some(on_press) = user_on_press {
                on_press.run(e);
            }
        })),
        ..link
    });
    // The press scrolls: no jump by the browser, no navigation by a router.
    let on_click = std::mem::take(&mut anchor.props.props.on_click);
    anchor.props.props.on_click =
        EventHandler::new(|e: MouseEvent| e.prevent_default()).chain(on_click);
    anchor
}
