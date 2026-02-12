use send_wrapper::SendWrapper;
use std::sync::Arc;
use wasm_bindgen::JsCast;

use crate::utils::element_capture::{CapturedElement, ElementCaptureAttr};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/focus/src/FocusScope.tsx
//
// ## React-aria deviation
//
// **React-aria pattern**: `useFocusManager()` is used within a `FocusScopeContext` provider,
// and `createFocusManager(scopeRef)` is used elsewhere with an explicit ref.
//
// **Leptonic pattern**: `use_focus_manager(input)` returns props with `ElementCaptureAttr` that
// automatically captures the element when spread. This is a deliberate deviation for better
// ergonomics. Users don't need to manually create and wire up NodeRefs. The element is captured
// automatically when attributes are spread.

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// No intentional deviations from the react-aria implementation.
//
// =============================================================================

/// Options for focus movement.
#[derive(Debug, Clone, Default)]
pub struct FocusManagerOptions {
    /// Element to start navigation from. Defaults to document.activeElement.
    pub from: Option<web_sys::Element>,

    /// Whether to wrap around when reaching the end.
    pub wrap: bool,

    /// Whether to only consider tabbable elements (tabindex >= 0).
    pub tabbable: bool,

    /// Whether to accept the starting element itself if it matches.
    pub accept: Option<fn(&web_sys::Element) -> bool>,
}

/// A focus manager provides methods for moving focus within a scope.
#[allow(clippy::type_complexity)]
#[derive(Clone)]
pub struct FocusManager {
    /// Function to get the current scope element.
    get_scope: Arc<SendWrapper<Box<dyn Fn() -> Option<web_sys::Element>>>>,
}

impl std::fmt::Debug for FocusManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FocusManager").finish_non_exhaustive()
    }
}

impl FocusManager {
    /// Create a new focus manager with a scope getter function.
    pub fn new<F>(get_scope: F) -> Self
    where
        F: Fn() -> Option<web_sys::Element> + 'static,
    {
        Self {
            get_scope: Arc::new(SendWrapper::new(Box::new(get_scope))),
        }
    }

    /// Move focus to the next focusable element.
    pub fn focus_next(&self, opts: FocusManagerOptions) -> Option<web_sys::Element> {
        self.move_focus(Direction::Next, opts)
    }

    /// Move focus to the previous focusable element.
    pub fn focus_previous(&self, opts: FocusManagerOptions) -> Option<web_sys::Element> {
        self.move_focus(Direction::Previous, opts)
    }

    /// Move focus to the first focusable element.
    #[allow(clippy::needless_pass_by_value)]
    pub fn focus_first(&self, opts: FocusManagerOptions) -> Option<web_sys::Element> {
        let scope = (self.get_scope)()?;
        let elements = get_focusable_elements(&scope, opts.tabbable);

        for element in elements {
            if let Some(accept) = opts.accept {
                if !accept(&element) {
                    continue;
                }
            }
            focus_element(&element);
            return Some(element);
        }

        None
    }

    /// Move focus to the last focusable element.
    #[allow(clippy::needless_pass_by_value)]
    pub fn focus_last(&self, opts: FocusManagerOptions) -> Option<web_sys::Element> {
        let scope = (self.get_scope)()?;
        let elements = get_focusable_elements(&scope, opts.tabbable);

        for element in elements.into_iter().rev() {
            if let Some(accept) = opts.accept {
                if !accept(&element) {
                    continue;
                }
            }
            focus_element(&element);
            return Some(element);
        }

        None
    }

    /// Move focus in a direction.
    fn move_focus(
        &self,
        direction: Direction,
        opts: FocusManagerOptions,
    ) -> Option<web_sys::Element> {
        let scope = (self.get_scope)()?;
        let elements = get_focusable_elements(&scope, opts.tabbable);

        if elements.is_empty() {
            return None;
        }

        // Get the starting element
        // Use the scope's owner document to correctly handle iframes/shadow DOM
        let from = opts
            .from
            .or_else(|| scope.owner_document().and_then(|d| d.active_element()));

        // Find current index
        let current_index = from
            .as_ref()
            .and_then(|f| elements.iter().position(|e| e == f));

        let next_index = match (direction, current_index) {
            (Direction::Next, Some(idx)) => {
                let next = idx + 1;
                if next >= elements.len() {
                    if opts.wrap {
                        Some(0)
                    } else {
                        None
                    }
                } else {
                    Some(next)
                }
            }
            (Direction::Next, None) => Some(0),
            (Direction::Previous, Some(idx)) => {
                if idx == 0 {
                    if opts.wrap {
                        Some(elements.len() - 1)
                    } else {
                        None
                    }
                } else {
                    Some(idx - 1)
                }
            }
            (Direction::Previous, None) => Some(elements.len() - 1),
        };

        let next_index = next_index?;

        // Find next valid element
        let len = elements.len();
        let mut checked = 0;
        let mut idx = next_index;

        while checked < len {
            let element = &elements[idx];

            let is_valid = opts.accept.map_or(true, |accept| accept(element));

            if is_valid {
                focus_element(element);
                return Some(element.clone());
            }

            // Move to next element
            match direction {
                Direction::Next => {
                    idx = (idx + 1) % len;
                }
                Direction::Previous => {
                    idx = if idx == 0 { len - 1 } else { idx - 1 };
                }
            }
            checked += 1;

            // Stop if we've wrapped and wrap is disabled
            if !opts.wrap && checked > 0 {
                match direction {
                    Direction::Next if idx == 0 => break,
                    Direction::Previous if idx == len - 1 => break,
                    _ => {}
                }
            }
        }

        None
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Next,
    Previous,
}

/// Selector for focusable elements.
const FOCUSABLE_SELECTOR: &str = "input:not([disabled]):not([type=hidden]), select:not([disabled]), textarea:not([disabled]), button:not([disabled]), a[href], area[href], summary, iframe, object, embed, audio[controls], video[controls], [contenteditable]:not([contenteditable=false]), [tabindex]";

/// Get all focusable elements within a scope.
fn get_focusable_elements(scope: &web_sys::Element, tabbable_only: bool) -> Vec<web_sys::Element> {
    let Ok(nodes) = scope.query_selector_all(FOCUSABLE_SELECTOR) else {
        return Vec::new();
    };

    let mut elements = Vec::new();

    for i in 0..nodes.length() {
        let Some(node) = nodes.get(i) else {
            continue;
        };

        let Some(element) = node.dyn_ref::<web_sys::Element>().cloned() else {
            continue;
        };

        // Check visibility
        if let Some(html_el) = element.dyn_ref::<web_sys::HtmlElement>() {
            if !is_element_visible(html_el) {
                continue;
            }

            // Check tabindex for tabbable-only mode
            if tabbable_only && html_el.tab_index() < 0 {
                continue;
            }
        }

        elements.push(element);
    }

    // Sort by tabindex (elements with tabindex > 0 come first, in order)
    // Then elements with tabindex = 0 in DOM order
    elements.sort_by(|a, b| {
        let a_idx = a
            .dyn_ref::<web_sys::HtmlElement>()
            .map_or(0, web_sys::HtmlElement::tab_index);
        let b_idx = b
            .dyn_ref::<web_sys::HtmlElement>()
            .map_or(0, web_sys::HtmlElement::tab_index);

        match (a_idx, b_idx) {
            // Both positive: sort by tabindex
            (a, b) if a > 0 && b > 0 => a.cmp(&b),
            // One positive, one zero/negative: positive comes first
            (a, _) if a > 0 => std::cmp::Ordering::Less,
            (_, b) if b > 0 => std::cmp::Ordering::Greater,
            // Both zero or negative: maintain DOM order (already in order from querySelectorAll)
            _ => std::cmp::Ordering::Equal,
        }
    });

    elements
}

/// Focus an element.
fn focus_element(element: &web_sys::Element) {
    if let Some(html_el) = element.dyn_ref::<web_sys::HtmlElement>() {
        let _ = html_el.focus();
    }
}

/// Check if an element is visible.
fn is_element_visible(element: &web_sys::HtmlElement) -> bool {
    // Use owner document's default view to correctly handle iframes/shadow DOM
    let Some(window) = element.owner_document().and_then(|d| d.default_view()) else {
        return true;
    };

    let Ok(Some(style)) = window.get_computed_style(element) else {
        return true;
    };

    if let Ok(display) = style.get_property_value("display") {
        if display == "none" {
            return false;
        }
    }

    if let Ok(visibility) = style.get_property_value("visibility") {
        if visibility == "hidden" || visibility == "collapse" {
            return false;
        }
    }

    true
}

/// Input parameters for the `use_focus_manager` hook.
#[derive(Debug, Clone, Copy, Default)]
pub struct UseFocusManagerInput {}

/// The return value of the `use_focus_manager` hook.
pub struct UseFocusManagerReturn {
    /// The focus manager instance.
    pub focus_manager: FocusManager,

    /// Props for programmatic merging. Call `.into_attrs()` for view spreading.
    pub props: UseFocusManagerProps,
}

impl std::fmt::Debug for UseFocusManagerReturn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UseFocusManagerReturn")
            .field("focus_manager", &self.focus_manager)
            .field("props", &self.props)
            .finish()
    }
}

/// Props from `use_focus_manager` that can be extracted and merged programmatically.
#[derive(Debug)]
pub struct UseFocusManagerProps {
    pub element_capture: ElementCaptureAttr,
}

impl UseFocusManagerProps {
    /// Convert to spreadable attributes for Leptos views, consuming self.
    #[must_use]
    pub fn into_attrs(self) -> UseFocusManagerAttrs {
        (self.element_capture,)
    }
}

/// These attributes must be spread onto the target element: `<foo {..attrs} />`
pub type UseFocusManagerAttrs = (ElementCaptureAttr,);

/// Creates a focus manager for navigating focus within a scope.
///
/// The focus manager provides methods to programmatically move focus
/// between focusable elements within a container.
///
/// The hook automatically captures the DOM element through [`ElementCaptureAttr`],
/// so you don't need to create or pass a `NodeRef`. Just spread the props
/// onto your element and focus management works automatically.
///
/// # Example
///
/// ```ignore
/// let UseFocusManagerReturn { focus_manager, props } =
///     use_focus_manager(UseFocusManagerInput::default());
///
/// view! {
///     <div {..props.into_attrs()}>
///         <button>"First"</button>
///         <button>"Second"</button>
///         <button on:click=move |_| {
///             focus_manager.focus_next(FocusManagerOptions::default());
///         }>"Next"</button>
///     </div>
/// }
/// ```
pub fn use_focus_manager(_input: UseFocusManagerInput) -> UseFocusManagerReturn {
    let scope_element = CapturedElement::new();

    UseFocusManagerReturn {
        focus_manager: FocusManager::new(move || {
            scope_element.get_untracked().map(SendWrapper::take)
        }),
        props: UseFocusManagerProps {
            element_capture: scope_element.attr(),
        },
    }
}
