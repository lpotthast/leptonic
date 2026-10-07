// No upstream: extension traits for DOM events and elements (react-aria's `nodeContains` and
// `setEventTarget` live in `shadow_dom` and here).
use wasm_bindgen::JsCast;

/// Extension trait for accessing event targets inside DOM event handler closures,
/// where `.target()` and `.current_target()` are guaranteed to be `Some`.
///
/// **Exception:** `current_target` becomes `null` after the handler returns (per DOM spec),
/// so stored/deferred events must use `if let Some(...)` for `.current_target()`.
pub(crate) trait EventAccessors {
    fn expect_target(&self) -> web_sys::EventTarget;
    fn expect_current_target(&self) -> web_sys::EventTarget;
}

impl<T: AsRef<web_sys::Event>> EventAccessors for T {
    fn expect_target(&self) -> web_sys::EventTarget {
        self.as_ref().target().expect("called in event handler")
    }
    fn expect_current_target(&self) -> web_sys::EventTarget {
        self.as_ref()
            .current_target()
            .expect("called in event handler")
    }
}

pub(crate) trait ElementExt {
    fn is_anchor_link(&self) -> bool;
    #[cfg(not(feature = "ssr"))]
    fn disable_text_selection(&self);
    #[cfg(not(feature = "ssr"))]
    fn restore_text_selection(&self);
}

impl ElementExt for web_sys::Element {
    /// True for any element of type `<a href=[...]>`.
    fn is_anchor_link(&self) -> bool {
        let tag_name = self.tag_name();
        (&tag_name == "A" || &tag_name == "a") && self.has_attribute("href")
    }

    #[cfg(not(feature = "ssr"))]
    fn disable_text_selection(&self) {
        super::text_selection::disable_text_selection(Some(self));
    }

    #[cfg(not(feature = "ssr"))]
    fn restore_text_selection(&self) {
        super::text_selection::restore_text_selection(Some(self));
    }
}

pub(crate) trait EventTargetExt {
    #[cfg(not(feature = "ssr"))]
    fn as_element(&self) -> Option<&web_sys::Element>;
    fn to_element(&self) -> Option<web_sys::Element>;
    #[cfg(not(feature = "ssr"))]
    fn as_html_element(&self) -> Option<web_sys::HtmlElement>;
    fn as_node(&self) -> Option<web_sys::Node>;
    /// The target's owner document, else the global document (`None` during SSR).
    fn get_owner_document(&self) -> Option<web_sys::Document>;
}

impl EventTargetExt for web_sys::EventTarget {
    #[cfg(not(feature = "ssr"))]
    fn as_element(&self) -> Option<&web_sys::Element> {
        self.dyn_ref::<web_sys::Element>()
    }

    fn to_element(&self) -> Option<web_sys::Element> {
        self.clone().dyn_into::<web_sys::Element>().ok()
    }

    #[cfg(not(feature = "ssr"))]
    fn as_html_element(&self) -> Option<web_sys::HtmlElement> {
        self.clone().dyn_into::<web_sys::HtmlElement>().ok()
    }

    fn as_node(&self) -> Option<web_sys::Node> {
        self.clone().dyn_into::<web_sys::Node>().ok()
    }

    fn get_owner_document(&self) -> Option<web_sys::Document> {
        self.to_element()
            .and_then(|el| el.owner_document())
            .or_else(|| leptos_use::use_document().as_ref().cloned())
    }
}

/// Whether `node` contains `other_node`, across shadow DOM boundaries (react-aria's
/// `nodeContains`, see [`shadow_dom::node_contains`](super::shadow_dom::node_contains)). `None`
/// when either is missing.
pub(crate) fn node_contains(
    node: Option<&web_sys::Node>,
    other_node: Option<&web_sys::Node>,
) -> Option<bool> {
    Some(super::shadow_dom::node_contains(node?, other_node?))
}

pub trait ContainsTarget {
    fn current_target_contains_target(&self) -> bool;
}

impl ContainsTarget for web_sys::Event {
    fn current_target_contains_target(&self) -> bool {
        node_contains(
            self.expect_current_target().as_node().as_ref(),
            self.expect_target().as_node().as_ref(),
        )
        .unwrap_or(true)
    }
}

/// Override the `target` and `currentTarget` properties of an event using
/// `Object.defineProperty`. This is necessary because these properties are
/// read-only on native events.
///
/// Used for synthetic blur events in `use_focus_within` where we need the
/// event to appear as if it originated from the tracked element.
/// (react-aria's `setEventTarget`).
#[cfg(not(feature = "ssr"))]
pub(crate) fn set_event_target(
    event: &web_sys::Event,
    target: &web_sys::EventTarget,
    current_target: &web_sys::EventTarget,
) {
    let descriptor = js_sys::Object::new();
    let _ = js_sys::Reflect::set(&descriptor, &"configurable".into(), &true.into());

    // Set target
    let _ = js_sys::Reflect::set(&descriptor, &"value".into(), target);
    let _ = js_sys::Object::define_property(event, &"target".into(), &descriptor);

    // Set currentTarget
    let _ = js_sys::Reflect::set(&descriptor, &"value".into(), current_target);
    let _ = js_sys::Object::define_property(event, &"currentTarget".into(), &descriptor);
}
