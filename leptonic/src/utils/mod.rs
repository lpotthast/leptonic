pub mod aria;
pub mod callback;
pub mod color;
pub mod locale;
pub mod math;
pub mod pointer_type;
pub mod props;
pub mod scroll_behavior;
pub mod signals;
pub(crate) mod text_selection;
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
        self.get_attribute("role").as_ref().map(|a| a.as_str()) == Some("link")
    }

    /// True for any element of type `<a href=[...]>`.
    fn is_anchor_link(&self) -> bool {
        self.tag_name().as_str() == "A" && self.has_attribute("href")
    }

    fn disable_text_selection(&self) {
        text_selection::disable_text_selection(self);
    }

    fn restore_text_selection(&self) {
        text_selection::restore_text_selection(self);
    }
}

pub(crate) trait EventTargetExt {
    fn as_element(&self) -> Option<web_sys::Element>;
    fn as_html_element(&self) -> Option<web_sys::HtmlElement>;
    fn as_node(&self) -> Option<web_sys::Node>;
    fn as_container(&self) -> Option<DomContainer>;
    fn get_owner_document(&self) -> web_sys::Document;
    fn is_over(&self, e: &impl EventExt, element: web_sys::Element) -> bool;
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

    fn get_owner_document(&self) -> web_sys::Document {
        self.as_element()
            .and_then(|el| el.owner_document())
            .unwrap_or_else(|| leptos::document())
    }

    fn is_over(&self, e: &impl EventExt, element: web_sys::Element) -> bool {
        let el_rect = element.get_bounding_client_rect().into();
        let point_rect = e.get_client_interaction_rect();
        overlapping(el_rect, point_rect)
    }
}

fn overlapping(a: RectPrecise, b: RectPrecise) -> bool {
    if a.left > b.right || b.left > a.right {
        return false;
    }
    // NOTE: Coordinate system starts in upper-left corner!
    if a.top > b.bottom || b.top > a.bottom {
        return false;
    }
    true
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

#[derive(Debug, Clone, Copy)]
pub struct RectPrecise {
    top: f64,
    right: f64,
    bottom: f64,
    left: f64,
}

impl From<web_sys::DomRect> for RectPrecise {
    fn from(value: web_sys::DomRect) -> Self {
        Self {
            top: value.top(),
            right: value.right(),
            bottom: value.bottom(),
            left: value.left(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Modifiers {
    /// Whether the shift keyboard modifier was held during the event.
    pub shift_key: bool,

    /// Whether the ctrl keyboard modifier was held during the event.
    pub ctrl_key: bool,

    /// Whether the meta keyboard modifier was held during the event.
    pub meta_key: bool,

    /// Whether the alt keyboard modifier was held during the  event.
    pub alt_key: bool,
}

pub trait EventModifiers {
    fn modifiers(&self) -> Modifiers;
}

pub trait EventExt {
    fn current_target_contains_target(&self) -> bool;

    fn get_client_interaction_rect(&self) -> RectPrecise;
}

impl EventModifiers for web_sys::MouseEvent {
    fn modifiers(&self) -> Modifiers {
        Modifiers {
            shift_key: self.shift_key(),
            ctrl_key: self.ctrl_key(),
            meta_key: self.meta_key(),
            alt_key: self.alt_key(),
        }
    }
}

impl EventModifiers for web_sys::TouchEvent {
    fn modifiers(&self) -> Modifiers {
        Modifiers {
            shift_key: self.shift_key(),
            ctrl_key: self.ctrl_key(),
            meta_key: self.meta_key(),
            alt_key: self.alt_key(),
        }
    }
}

impl EventModifiers for web_sys::KeyboardEvent {
    fn modifiers(&self) -> Modifiers {
        Modifiers {
            shift_key: self.shift_key(),
            ctrl_key: self.ctrl_key(),
            meta_key: self.meta_key(),
            alt_key: self.alt_key(),
        }
    }
}

impl EventModifiers for web_sys::PointerEvent {
    fn modifiers(&self) -> Modifiers {
        Modifiers {
            shift_key: self.shift_key(),
            ctrl_key: self.ctrl_key(),
            meta_key: self.meta_key(),
            alt_key: self.alt_key(),
        }
    }
}

impl EventExt for web_sys::PointerEvent {
    fn current_target_contains_target(&self) -> bool {
        current_target_contains_target(self.current_target().as_ref(), self.target().as_ref())
            .unwrap_or(true)
    }

    fn get_client_interaction_rect(&self) -> RectPrecise {
        let offset_x = self.width() as f64 / 2.0;
        let offset_y = self.height() as f64 / 2.0;
        RectPrecise {
            top: self.client_y() as f64 - offset_y,
            right: self.client_x() as f64 + offset_x,
            bottom: self.client_y() as f64 + offset_y,
            left: self.client_x() as f64 - offset_x,
        }
    }
}
