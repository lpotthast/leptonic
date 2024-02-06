pub mod aria;
pub mod math;
pub mod props;
pub mod time;

pub(crate) enum DomContainer {
    Node(web_sys::Node),
    Window(web_sys::Window),
}

impl DomContainer {
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

pub(crate) trait EventTargetExt {
    fn as_element(&self) -> Option<web_sys::Element>;
    fn as_html_element(&self) -> Option<web_sys::HtmlElement>;
    fn as_node(&self) -> Option<web_sys::Node>;
    fn as_container(&self) -> Option<DomContainer>;
}

impl EventTargetExt for web_sys::EventTarget {
    fn as_element(&self) -> Option<web_sys::Element> {
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
}

pub(crate) fn current_target_contains_target(
    current_target: Option<&web_sys::EventTarget>,
    target: Option<&web_sys::EventTarget>,
) -> Option<bool> {
    let current_target = current_target.or_else(|| {
        tracing::warn!("TouchEvent has no current_target.");
        None
    })?;

    let target = target.or_else(|| {
        tracing::warn!("TouchEvent has no target.");
        None
    })?;

    let current_target_container = current_target.as_container().or_else(|| {
        tracing::warn!(
            ?current_target,
            "TouchEvent's current_target was not a container."
        );
        None
    })?;

    let target_node = target.as_node().or_else(|| {
        tracing::warn!(?target, "TouchEvent's target was not a node.");
        None
    })?;

    Some(current_target_container.contains(target_node))
}
