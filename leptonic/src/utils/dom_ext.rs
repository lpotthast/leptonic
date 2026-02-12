use leptos::prelude::document;
use wasm_bindgen::convert::FromWasmAbi;

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

pub(crate) enum DomContainer {
    Node(web_sys::Node),
    Window(web_sys::Window),
}

impl DomContainer {
    #[allow(clippy::needless_pass_by_value)]
    pub(crate) fn contains(&self, node: web_sys::Node) -> bool {
        match self {
            DomContainer::Node(node) => node.contains(Some(node)),
            DomContainer::Window(window) => node
                .owner_document()
                .and_then(move |d| d.default_view().map(move |w| w == *window))
                .unwrap_or_default(),
        }
    }
}

pub(crate) trait ElementExt {
    fn is_link(&self) -> bool;
    fn has_link_role(&self) -> bool;
    fn is_anchor_link(&self) -> bool;
    fn disable_text_selection(&self);
    fn restore_text_selection(&self);
}

impl ElementExt for web_sys::Element {
    /// True for any element having `role="link"` or being of type `<a href=[...]>`..
    fn is_link(&self) -> bool {
        self.has_link_role() || self.is_anchor_link()
    }

    /// True for any element having `role="link"`.
    fn has_link_role(&self) -> bool {
        self.get_attribute("role").as_deref() == Some("link")
    }

    /// True for any element of type `<a href=[...]>`.
    fn is_anchor_link(&self) -> bool {
        let tag_name = self.tag_name();
        (&tag_name == "A" || &tag_name == "a") && self.has_attribute("href")
    }

    fn disable_text_selection(&self) {
        super::text_selection::disable_text_selection(self);
    }

    fn restore_text_selection(&self) {
        super::text_selection::restore_text_selection(self);
    }
}

pub(crate) trait EventTargetExt {
    fn as_element(&self) -> Option<&web_sys::Element>;
    fn to_element(&self) -> Option<web_sys::Element>;
    #[allow(unused)]
    fn as_html_element(&self) -> Option<web_sys::HtmlElement>;
    fn as_node(&self) -> Option<web_sys::Node>;
    fn as_container(&self) -> Option<DomContainer>;
    fn get_owner_document(&self) -> web_sys::Document;
    /// Adds a one-time event listener for the given event name.
    fn listen_once<E>(&self, event_name: &str, callback: impl FnOnce(E) + 'static)
    where
        E: FromWasmAbi + 'static;
    /// Adds a one-time event listener that calls `prevent_default()` on the event.
    fn prevent_default_once(&self, event_name: &str);
}

impl EventTargetExt for web_sys::EventTarget {
    fn as_element(&self) -> Option<&web_sys::Element> {
        use wasm_bindgen::JsCast;
        self.dyn_ref::<web_sys::Element>()
    }

    fn to_element(&self) -> Option<web_sys::Element> {
        use wasm_bindgen::JsCast;
        self.clone().dyn_into::<web_sys::Element>().ok()
    }

    fn as_html_element(&self) -> Option<web_sys::HtmlElement> {
        use wasm_bindgen::JsCast;
        self.clone().dyn_into::<web_sys::HtmlElement>().ok()
    }

    fn as_node(&self) -> Option<web_sys::Node> {
        use wasm_bindgen::JsCast;
        self.clone().dyn_into::<web_sys::Node>().ok()
    }

    fn as_container(&self) -> Option<DomContainer> {
        use wasm_bindgen::JsCast;
        if let Ok(node) = self.clone().dyn_into::<web_sys::Node>() {
            return Some(DomContainer::Node(node));
        }
        if let Ok(window) = self.clone().dyn_into::<web_sys::Window>() {
            return Some(DomContainer::Window(window));
        }
        None
    }

    fn get_owner_document(&self) -> web_sys::Document {
        self.to_element()
            .and_then(|el| el.owner_document())
            .unwrap_or_else(document)
    }

    fn listen_once<E>(&self, event_name: &str, callback: impl FnOnce(E) + 'static)
    where
        E: FromWasmAbi + 'static,
    {
        use wasm_bindgen::closure::Closure;
        use wasm_bindgen::JsCast;

        let boxed: Box<dyn FnOnce(E)> = Box::new(callback);
        let closure = Closure::once(boxed);

        let options = web_sys::AddEventListenerOptions::new();
        options.set_once(true);

        let _ = self.add_event_listener_with_callback_and_add_event_listener_options(
            event_name,
            closure.as_ref().unchecked_ref(),
            &options,
        );

        closure.forget();
    }

    fn prevent_default_once(&self, event_name: &str) {
        self.listen_once(event_name, |e: web_sys::Event| {
            e.prevent_default();
        });
    }
}

/// # Deviations from react-aria
///
/// - We have no shadow DOM support.
/// - We are not walking from `other_node` upwards to find `node`.
/// - We are not recognizing `HTMLSlotElement` in the path from `other_node` to `node`.
pub(crate) fn node_contains(
    node: Option<&web_sys::Node>,
    other_node: Option<&web_sys::Node>,
) -> Option<bool> {
    let node = node.or_else(|| None)?;
    let other_node = other_node.or_else(|| None)?;
    let contained = node.contains(Some(other_node));
    Some(contained)
}

/// Get the owner document of a node, falling back to the global document.
/// This is useful for correctly handling elements in iframes or shadow DOM.
pub fn get_owner_document(node: &web_sys::Node) -> web_sys::Document {
    node.owner_document().unwrap_or_else(document)
}

/// Get the owner window of a node via its owner document.
/// Returns None if the document has no default view.
pub fn get_owner_window(node: &web_sys::Node) -> Option<web_sys::Window> {
    node.owner_document()?.default_view()
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
