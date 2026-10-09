// Upstream: react-aria/src/selection/DOMLayoutDelegate.ts @ 99e6102368
use wasm_bindgen::JsCast;

use super::{ItemElements, Key};
use crate::CapturedElement;

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

/// A corner of a [`Rect`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RectCorner {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

// Upstream: react-stately/src/virtualizer/Rect.ts @ 99e6102368
impl Rect {
    pub const fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    /// The maximum x-coordinate in the rectangle.
    pub fn max_x(&self) -> f64 {
        self.x + self.width
    }

    /// The maximum y-coordinate in the rectangle.
    pub fn max_y(&self) -> f64 {
        self.y + self.height
    }

    pub fn area(&self) -> f64 {
        self.width * self.height
    }

    /// The position of `corner`.
    pub fn corner(&self, corner: RectCorner) -> crate::utils::point::Point {
        use crate::utils::point::Point;
        match corner {
            RectCorner::TopLeft => Point::new(self.x, self.y),
            RectCorner::TopRight => Point::new(self.max_x(), self.y),
            RectCorner::BottomLeft => Point::new(self.x, self.max_y()),
            RectCorner::BottomRight => Point::new(self.max_x(), self.max_y()),
        }
    }

    /// Whether this rectangle and `rect` overlap (rectangles without area never do).
    pub fn intersects(&self, rect: &Rect) -> bool {
        self.area() > 0.0
            && rect.area() > 0.0
            && self.x <= rect.x + rect.width
            && rect.x <= self.x + self.width
            && self.y <= rect.y + rect.height
            && rect.y <= self.y + self.height
    }

    /// Whether this rectangle fully contains `rect`.
    pub fn contains_rect(&self, rect: &Rect) -> bool {
        self.x <= rect.x
            && self.y <= rect.y
            && self.max_x() >= rect.max_x()
            && self.max_y() >= rect.max_y()
    }

    pub fn contains_point(&self, point: crate::utils::point::Point) -> bool {
        self.x <= point.x && self.y <= point.y && self.max_x() >= point.x && self.max_y() >= point.y
    }

    /// The first corner of this rectangle (top to bottom, left to right) inside `rect`.
    pub fn corner_in_rect(&self, rect: &Rect) -> Option<RectCorner> {
        [
            RectCorner::TopLeft,
            RectCorner::TopRight,
            RectCorner::BottomLeft,
            RectCorner::BottomRight,
        ]
        .into_iter()
        .find(|corner| rect.contains_point(self.corner(*corner)))
    }

    /// Whether the positions are equal.
    pub fn point_equals(&self, other: &Rect) -> bool {
        self.x == other.x && self.y == other.y
    }

    /// Whether the sizes are equal.
    pub fn size_equals(&self, size: Size) -> bool {
        self.width == size.width && self.height == size.height
    }

    /// The smallest rectangle containing both.
    #[must_use]
    pub fn union(&self, other: &Rect) -> Rect {
        let x = self.x.min(other.x);
        let y = self.y.min(other.y);
        Rect::new(
            x,
            y,
            self.max_x().max(other.max_x()) - x,
            self.max_y().max(other.max_y()) - y,
        )
    }

    /// The overlap of both; all zero if they don't intersect.
    #[must_use]
    pub fn intersection(&self, other: &Rect) -> Rect {
        if !self.intersects(other) {
            return Rect::default();
        }
        let x = self.x.max(other.x);
        let y = self.y.max(other.y);
        Rect::new(
            x,
            y,
            self.max_x().min(other.max_x()) - x,
            self.max_y().min(other.max_y()) - y,
        )
    }
}

// Upstream: react-stately/src/virtualizer/Size.ts @ 99e6102368
impl Size {
    /// A size, negative values clamped to 0.
    pub fn new(width: f64, height: f64) -> Self {
        Self {
            width: width.max(0.0),
            height: height.max(0.0),
        }
    }

    pub fn area(&self) -> f64 {
        self.width * self.height
    }
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
