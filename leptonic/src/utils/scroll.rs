// Based on:
// - https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/utils/src/scrollIntoView.ts
// - https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/utils/src/isScrollable.ts
// - https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/utils/src/getScrollParent.ts
// - https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/utils/src/getScrollParents.ts

use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement};

use crate::utils::platform::{browser, device};

/// Alignment position for scrolling along a single axis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[allow(dead_code)]
pub enum ScrollAlignment {
    Start,
    Center,
    End,
    #[default]
    Nearest,
}

/// Options for [`scroll_into_view`].
#[derive(Debug, Clone, Copy, Default)]
pub struct ScrollIntoViewOpts {
    /// The position to align items along the block axis.
    pub block: ScrollAlignment,
    /// The position to align items along the inline axis.
    pub inline: ScrollAlignment,
}

/// Options for [`scroll_into_viewport`].
#[derive(Default)]
pub struct ScrollIntoViewportOpts {
    /// The optional containing element to center in the viewport prior to
    /// scrolling the target element into view.
    pub containing_element: Option<Element>,
}

/// Parse a CSS pixel value like `"10px"` into `10.0`. Returns `0.0` for
/// non-numeric or empty values (matching `parseInt(value, 10) || 0` in JS).
fn parse_css_px(value: &str) -> f64 {
    let trimmed = value.trim();
    let numeric = trimmed.strip_suffix("px").unwrap_or(trimmed);
    numeric.parse::<f64>().unwrap_or(0.0)
}

/// Get the root scrolling element for the document.
fn get_root_element(document: &web_sys::Document) -> Option<Element> {
    document
        .scrolling_element()
        .or_else(|| document.document_element())
}

/// Returns `true` if the element is scrollable.
///
/// Checks computed `overflow`, `overflow-x`, and `overflow-y` styles for
/// `auto` or `scroll`. The root element is always considered scrollable
/// unless `overflow: hidden`.
///
/// When `check_for_overflow` is `true`, also verifies that actual content
/// overflows the element's client area.
pub fn is_scrollable(node: &Element, check_for_overflow: bool) -> bool {
    let Some(window) = node.owner_document().and_then(|doc| doc.default_view()) else {
        return false;
    };

    let Ok(Some(style)) = window.get_computed_style(node) else {
        return false;
    };

    let root = node.owner_document().and_then(|d| get_root_element(&d));

    let overflow = format!(
        "{}{}{}",
        style.get_property_value("overflow").unwrap_or_default(),
        style.get_property_value("overflow-x").unwrap_or_default(),
        style.get_property_value("overflow-y").unwrap_or_default(),
    );

    let mut scrollable = overflow.contains("auto") || overflow.contains("scroll");

    // Root element has `visible` overflow by default, but is scrollable nonetheless.
    if root.as_ref() == Some(node)
        && style.get_property_value("overflow").unwrap_or_default() != "hidden"
    {
        scrollable = true;
    }

    if scrollable && check_for_overflow {
        scrollable = node.scroll_height() != node.client_height()
            || node.scroll_width() != node.client_width();
    }

    scrollable
}

/// Returns the nearest scrollable ancestor of `node`, skipping `node` itself.
///
/// Falls back to the document's root scrolling element.
pub fn get_scroll_parent(node: &Element, check_for_overflow: bool) -> Element {
    // Skip the node itself if it's scrollable — we want an *ancestor*.
    let mut current = if is_scrollable(node, check_for_overflow) {
        node.parent_element()
    } else {
        Some(node.clone())
    };

    while let Some(ref el) = current {
        if is_scrollable(el, check_for_overflow) {
            return el.clone();
        }
        current = el.parent_element();
    }

    node.owner_document()
        .and_then(|d| get_root_element(&d))
        .unwrap_or_else(|| node.clone())
}

/// Returns all scrollable ancestors of `node`, stopping before the root element.
///
/// Includes `node` itself if it is scrollable.
pub fn get_scroll_parents(node: &Element, check_for_overflow: bool) -> Vec<Element> {
    let mut parents = Vec::new();
    let root = node.owner_document().and_then(|d| get_root_element(&d));

    let mut current: Option<Element> = Some(node.clone());
    while let Some(el) = current {
        if root.as_ref() == Some(&el) {
            break;
        }
        if is_scrollable(&el, check_for_overflow) {
            parents.push(el.clone());
        }
        current = el.parent_element();
    }

    parents
}

/// Scrolls `scroll_view` so that `element` is visible within it.
///
/// Similar to `element.scrollIntoView({block: 'nearest'})` but doesn't affect
/// parents above `scroll_view`. Handles scroll margins/padding, borders,
/// scrollbar widths, and RTL layouts.
#[allow(clippy::too_many_lines)]
pub fn scroll_into_view(
    scroll_view: &HtmlElement,
    element: &HtmlElement,
    opts: ScrollIntoViewOpts,
) {
    let scroll_el: &Element = scroll_view;
    let element_el: &Element = element;

    if scroll_el == element_el {
        return;
    }

    let Some(window) = scroll_view.owner_document().and_then(|d| d.default_view()) else {
        return;
    };

    let Some(root) = scroll_view
        .owner_document()
        .and_then(|d| get_root_element(&d))
    else {
        return;
    };

    let Ok(Some(item_style)) = window.get_computed_style(element_el) else {
        return;
    };
    let Ok(Some(view_style)) = window.get_computed_style(scroll_el) else {
        return;
    };

    let mut y = scroll_el.scroll_top();
    let mut x = scroll_el.scroll_left();

    let target = element_el.get_bounding_client_rect();
    let view = scroll_el.get_bounding_client_rect();

    let is_root = scroll_el == &root;
    let view_top = if is_root { 0.0 } else { view.top() };
    let view_bottom = if is_root {
        f64::from(scroll_el.client_height())
    } else {
        view.bottom()
    };
    let view_left = if is_root { 0.0 } else { view.left() };
    let view_right = if is_root {
        f64::from(scroll_el.client_width())
    } else {
        view.right()
    };

    // Scroll margins on the target element.
    let scroll_margin_top = parse_css_px(
        &item_style
            .get_property_value("scroll-margin-top")
            .unwrap_or_default(),
    );
    let scroll_margin_bottom = parse_css_px(
        &item_style
            .get_property_value("scroll-margin-bottom")
            .unwrap_or_default(),
    );
    let scroll_margin_left = parse_css_px(
        &item_style
            .get_property_value("scroll-margin-left")
            .unwrap_or_default(),
    );
    let scroll_margin_right = parse_css_px(
        &item_style
            .get_property_value("scroll-margin-right")
            .unwrap_or_default(),
    );

    // Scroll padding on the scroll container.
    let scroll_padding_top = parse_css_px(
        &view_style
            .get_property_value("scroll-padding-top")
            .unwrap_or_default(),
    );
    let scroll_padding_bottom = parse_css_px(
        &view_style
            .get_property_value("scroll-padding-bottom")
            .unwrap_or_default(),
    );
    let scroll_padding_left = parse_css_px(
        &view_style
            .get_property_value("scroll-padding-left")
            .unwrap_or_default(),
    );
    let scroll_padding_right = parse_css_px(
        &view_style
            .get_property_value("scroll-padding-right")
            .unwrap_or_default(),
    );

    // Border widths on the scroll container.
    let border_top_width = parse_css_px(
        &view_style
            .get_property_value("border-top-width")
            .unwrap_or_default(),
    );
    let border_bottom_width = parse_css_px(
        &view_style
            .get_property_value("border-bottom-width")
            .unwrap_or_default(),
    );
    let border_left_width = parse_css_px(
        &view_style
            .get_property_value("border-left-width")
            .unwrap_or_default(),
    );
    let border_right_width = parse_css_px(
        &view_style
            .get_property_value("border-right-width")
            .unwrap_or_default(),
    );

    // Scroll area includes the target element plus its scroll margins.
    let scroll_area_top = target.top() - scroll_margin_top;
    let scroll_area_bottom = target.bottom() + scroll_margin_bottom;
    let scroll_area_left = target.left() - scroll_margin_left;
    let scroll_area_right = target.right() + scroll_margin_right;

    // Scrollbar dimensions.
    let scrollbar_offset_x = if is_root {
        0.0
    } else {
        border_left_width + border_right_width
    };
    let scrollbar_offset_y = if is_root {
        0.0
    } else {
        border_top_width + border_bottom_width
    };
    let scrollbar_width = f64::from(scroll_view.offset_width())
        - f64::from(scroll_el.client_width())
        - scrollbar_offset_x;
    let scrollbar_height = f64::from(scroll_view.offset_height())
        - f64::from(scroll_el.client_height())
        - scrollbar_offset_y;

    // Scroll port: the visible content area within the container.
    let scroll_port_top = view_top + border_top_width + scroll_padding_top;
    let scroll_port_bottom =
        view_bottom - border_bottom_width - scroll_padding_bottom - scrollbar_height;
    let mut scroll_port_left = view_left + border_left_width + scroll_padding_left;
    let mut scroll_port_right = view_right - border_right_width - scroll_padding_right;

    // iOS always positions the scrollbar on the right.
    if view_style
        .get_property_value("direction")
        .unwrap_or_default()
        == "rtl"
        && !device::is_ios()
    {
        scroll_port_left += scrollbar_width;
    } else {
        scroll_port_right -= scrollbar_width;
    }

    let should_scroll_block =
        scroll_area_top < scroll_port_top || scroll_area_bottom > scroll_port_bottom;
    let should_scroll_inline =
        scroll_area_left < scroll_port_left || scroll_area_right > scroll_port_right;

    // Block axis adjustment.
    if should_scroll_block {
        match opts.block {
            ScrollAlignment::Start => {
                y += scroll_area_top - scroll_port_top;
            }
            ScrollAlignment::Center => {
                y += scroll_area_top.midpoint(scroll_area_bottom)
                    - scroll_port_top.midpoint(scroll_port_bottom);
            }
            ScrollAlignment::End => {
                y += scroll_area_bottom - scroll_port_bottom;
            }
            ScrollAlignment::Nearest => {
                let start = scroll_area_top - scroll_port_top;
                let end = scroll_area_bottom - scroll_port_bottom;
                y += if start.abs() <= end.abs() { start } else { end };
            }
        }
    }

    // Inline axis adjustment.
    if should_scroll_inline {
        match opts.inline {
            ScrollAlignment::Start => {
                x += scroll_area_left - scroll_port_left;
            }
            ScrollAlignment::Center => {
                x += scroll_area_left.midpoint(scroll_area_right)
                    - scroll_port_left.midpoint(scroll_port_right);
            }
            ScrollAlignment::End => {
                x += scroll_area_right - scroll_port_right;
            }
            ScrollAlignment::Nearest => {
                let start = scroll_area_left - scroll_port_left;
                let end = scroll_area_right - scroll_port_right;
                x += if start.abs() <= end.abs() { start } else { end };
            }
        }
    }

    let opts = web_sys::ScrollToOptions::new();
    opts.set_left(x);
    opts.set_top(y);
    scroll_el.scroll_to_with_scroll_to_options(&opts);
}

/// Scrolls the viewport so that `target` is visible.
///
/// When scrolling is not prevented on the body, uses the native `scrollIntoView`.
/// When scrolling is prevented (e.g., in an overlay), scrolls only the scroll
/// parents of the target up to (but not including) the body.
///
/// If a `containing_element` is provided in `opts`, it will be centered in the
/// viewport before scrolling the target into view.
pub fn scroll_into_viewport(target: Option<&Element>, opts: &ScrollIntoViewportOpts) {
    let Some(target) = target else { return };
    if !target.is_connected() {
        return;
    }

    let Some(document) = target.owner_document() else {
        return;
    };
    let Some(window) = document.default_view() else {
        return;
    };
    let Some(root) = get_root_element(&document) else {
        return;
    };

    let is_scroll_prevented = window
        .get_computed_style(&root)
        .ok()
        .flatten()
        .and_then(|s| s.get_property_value("overflow").ok())
        .is_some_and(|o| o == "hidden");

    // When scrolling is not prevented and we're not on Chrome (due to
    // https://issues.chromium.org/issues/40074749), use native scrollIntoView.
    if !is_scroll_prevented && !browser::is_chrome() {
        scroll_into_viewport_native(target, opts);
    } else {
        scroll_into_viewport_manual(target, opts);
    }
}

/// Use native `scrollIntoView` to bring the target element into view.
fn scroll_into_viewport_native(target: &Element, opts: &ScrollIntoViewportOpts) {
    let original = target.get_bounding_client_rect();
    let original_left = original.left();
    let original_top = original.top();

    let nearest_opts = web_sys::ScrollIntoViewOptions::new();
    nearest_opts.set_block(web_sys::ScrollLogicalPosition::Nearest);
    target.scroll_into_view_with_scroll_into_view_options(&nearest_opts);

    let after = target.get_bounding_client_rect();
    // Account for sub-pixel rounding differences.
    if (original_left - after.left()).abs() > 1.0 || (original_top - after.top()).abs() > 1.0 {
        if let Some(containing) = &opts.containing_element {
            let center_opts = web_sys::ScrollIntoViewOptions::new();
            center_opts.set_block(web_sys::ScrollLogicalPosition::Center);
            center_opts.set_inline(web_sys::ScrollLogicalPosition::Center);
            containing.scroll_into_view_with_scroll_into_view_options(&center_opts);
            target.scroll_into_view_with_scroll_into_view_options(&nearest_opts);
        }
    }
}

/// Manually scroll all scroll parents when native scrolling is prevented
/// (e.g., body has `overflow: hidden` for an overlay).
fn scroll_into_viewport_manual(target: &Element, opts: &ScrollIntoViewportOpts) {
    let original = target.get_bounding_client_rect();
    let original_left = original.left();
    let original_top = original.top();

    // Scroll only scroll parents, not the body (would move overlay off-screen).
    let scroll_parents = get_scroll_parents(target, true);
    for parent in &scroll_parents {
        if let (Some(sv), Some(el)) = (
            parent.dyn_ref::<HtmlElement>(),
            target.dyn_ref::<HtmlElement>(),
        ) {
            scroll_into_view(sv, el, ScrollIntoViewOpts::default());
        }
    }

    let after = target.get_bounding_client_rect();
    // Account for sub-pixel rounding differences.
    if (original_left - after.left()).abs() > 1.0 || (original_top - after.top()).abs() > 1.0 {
        if let Some(containing) = &opts.containing_element {
            let container_parents = get_scroll_parents(containing, true);
            for parent in &container_parents {
                if let (Some(sv), Some(ce)) = (
                    parent.dyn_ref::<HtmlElement>(),
                    containing.dyn_ref::<HtmlElement>(),
                ) {
                    scroll_into_view(
                        sv,
                        ce,
                        ScrollIntoViewOpts {
                            block: ScrollAlignment::Center,
                            inline: ScrollAlignment::Center,
                        },
                    );
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::*;

    #[test]
    fn parse_css_px_parses_integer_pixels() {
        assert_that(parse_css_px("10px")).is_equal_to(10.0);
    }

    #[test]
    fn parse_css_px_parses_zero() {
        assert_that(parse_css_px("0px")).is_equal_to(0.0);
        assert_that(parse_css_px("0")).is_equal_to(0.0);
    }

    #[test]
    fn parse_css_px_parses_fractional_pixels() {
        assert_that(parse_css_px("10.5px")).is_equal_to(10.5);
    }

    #[test]
    fn parse_css_px_returns_zero_for_empty_or_non_numeric() {
        assert_that(parse_css_px("")).is_equal_to(0.0);
        assert_that(parse_css_px("auto")).is_equal_to(0.0);
    }

    #[test]
    fn parse_css_px_trims_whitespace() {
        assert_that(parse_css_px("  10px  ")).is_equal_to(10.0);
    }
}
