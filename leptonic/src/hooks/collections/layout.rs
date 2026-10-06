// Upstream: react-aria/src/selection/DOMLayoutDelegate.ts @ 99e6102368
use wasm_bindgen::JsCast;

use super::{ItemElements, Key};
use crate::utils::CapturedElement;

/// A rectangle in the collection's content coordinates (scroll offsets included).
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Size {
    pub width: f64,
    pub height: f64,
}

/// Where items are and what is visible: the geometry keyboard navigation needs for grid layouts
/// and page up/down.
pub trait LayoutDelegate: Send + Sync {
    /// The rectangle of an item, if it is rendered.
    fn item_rect(&self, key: &Key) -> Option<Rect>;
    /// The visible part of the collection.
    fn visible_rect(&self) -> Rect;
    /// The size of the scrollable content.
    fn content_size(&self) -> Size;
    /// Whether the collection scrolls. If it doesn't, page up/down jump to the first/last item.
    fn is_scrollable(&self) -> bool {
        let visible = self.visible_rect();
        let content = self.content_size();
        content.height > visible.height || content.width > visible.width
    }
}

/// A [`LayoutDelegate`] that measures the rendered DOM: the collection container and the item
/// elements registered in [`ItemElements`]. Reports nothing during server-side rendering.
#[derive(Debug, Clone, Copy)]
pub struct DomLayoutDelegate {
    container: CapturedElement,
    items: ItemElements,
}

impl DomLayoutDelegate {
    pub fn new(container: CapturedElement, items: ItemElements) -> Self {
        Self { container, items }
    }

    fn container(&self) -> Option<web_sys::HtmlElement> {
        self.container
            .get_untracked()
            .and_then(|el| (*el).clone().dyn_into::<web_sys::HtmlElement>().ok())
    }
}

impl LayoutDelegate for DomLayoutDelegate {
    fn item_rect(&self, key: &Key) -> Option<Rect> {
        let container = self.container()?;
        let item = self.items.get(key)?;
        let container_rect = container.get_bounding_client_rect();
        let item_rect = item.get_bounding_client_rect();
        Some(Rect {
            x: item_rect.left() - container_rect.left() - f64::from(container.client_left())
                + container.scroll_left(),
            y: item_rect.top() - container_rect.top() - f64::from(container.client_top())
                + container.scroll_top(),
            width: item_rect.width(),
            height: item_rect.height(),
        })
    }

    fn visible_rect(&self) -> Rect {
        self.container().map_or_else(Rect::default, |c| Rect {
            x: c.scroll_left(),
            y: c.scroll_top(),
            width: f64::from(c.client_width()),
            height: f64::from(c.client_height()),
        })
    }

    fn content_size(&self) -> Size {
        self.container().map_or_else(Size::default, |c| Size {
            width: f64::from(c.scroll_width()),
            height: f64::from(c.scroll_height()),
        })
    }

    fn is_scrollable(&self) -> bool {
        self.container()
            .is_some_and(|c| crate::utils::scroll::is_scrollable(&c, false))
    }
}
