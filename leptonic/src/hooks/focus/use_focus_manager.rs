use std::sync::Arc;

use send_wrapper::SendWrapper;
use wasm_bindgen::JsCast;

use crate::{
    hooks::IntoAttrs,
    utils::{
        dom_ext::node_contains,
        element_capture::{CapturedElement, ElementCaptureAttr},
        focusability, shadow_dom,
        shadow_tree_walker::{self, ShadowTreeWalker},
    },
};

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
// ## DIFFERENT BEHAVIOR
//
// - Element capture pattern vs scope context
//   React-aria: `useFocusManager()` reads from `FocusContext` provided by a parent
//   `FocusScope`. `createFocusManager(ref)` takes an explicit ref.
//   Leptonic: `use_focus_manager(input)` returns `ElementCaptureAttr` that auto-
//   captures the element when spread. No wrapper or manual NodeRef needed.
//
// ## OMITTED FUNCTIONALITY
//
// - No `defaultOptions` merging
//   React-aria's `createFocusManager(ref, defaultOptions)` merges defaults with
//   per-call options. Leptonic requires full options on each method call.
//
// =============================================================================

/// Options for focus movement.
#[derive(Clone, Default)]
pub struct FocusManagerOptions {
    /// Element to start navigation from. Defaults to document.activeElement.
    pub from: Option<web_sys::Element>,

    /// Whether to wrap around when reaching the end.
    pub wrap: bool,

    /// Whether to only consider tabbable elements (tabindex >= 0).
    pub tabbable: bool,

    /// Custom filter function for acceptable elements.
    /// Unlike a bare `fn` pointer, this accepts closures that capture state.
    pub accept: Option<Arc<dyn Fn(&web_sys::Element) -> bool + Send + Sync>>,
}

impl std::fmt::Debug for FocusManagerOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FocusManagerOptions")
            .field("from", &self.from)
            .field("wrap", &self.wrap)
            .field("tabbable", &self.tabbable)
            .field("accept", &self.accept.as_ref().map(|_| ".."))
            .finish()
    }
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
    #[allow(clippy::needless_pass_by_value)]
    pub fn focus_next(&self, opts: FocusManagerOptions) -> Option<web_sys::Element> {
        let scope = (self.get_scope)()?;
        let mut walker = create_scope_walker(&scope)?;

        let from = opts.from.or_else(|| {
            scope
                .owner_document()
                .and_then(|d| shadow_dom::get_active_element(&d))
        });

        let from_radio_group = from.as_ref().and_then(focusability::get_radio_group_name);

        if let Some(ref from_el) = from {
            if node_contains(Some(scope.as_ref()), Some(from_el.as_ref())).unwrap_or(false) {
                walker.set_current_node(from_el);
            } else {
                // Active element is outside scope — focus first element (react-aria behavior).
                let result = find_first_focusable(
                    &mut walker,
                    &scope,
                    opts.tabbable,
                    opts.accept.as_ref(),
                    None,
                );
                if let Some(ref el) = result {
                    focus_element(el);
                }
                return result;
            }
        }

        let next = walker_next_focusable(
            &mut walker,
            opts.tabbable,
            opts.accept.as_ref(),
            from_radio_group.as_deref(),
        );

        let result = if next.is_none() && opts.wrap {
            find_first_focusable(
                &mut walker,
                &scope,
                opts.tabbable,
                opts.accept.as_ref(),
                None,
            )
        } else {
            next
        };

        if let Some(ref el) = result {
            focus_element(el);
        }
        result
    }

    /// Move focus to the previous focusable element.
    #[allow(clippy::needless_pass_by_value)]
    pub fn focus_previous(&self, opts: FocusManagerOptions) -> Option<web_sys::Element> {
        let scope = (self.get_scope)()?;
        let mut walker = create_scope_walker(&scope)?;

        let from = opts.from.or_else(|| {
            scope
                .owner_document()
                .and_then(|d| shadow_dom::get_active_element(&d))
        });

        let from_radio_group = from.as_ref().and_then(focusability::get_radio_group_name);

        if let Some(ref from_el) = from {
            if node_contains(Some(scope.as_ref()), Some(from_el.as_ref())).unwrap_or(false) {
                walker.set_current_node(from_el);
            } else {
                // Active element is outside scope — focus last element (react-aria behavior).
                let result = find_last_focusable(
                    &mut walker,
                    &scope,
                    opts.tabbable,
                    opts.accept.as_ref(),
                    None,
                );
                if let Some(ref el) = result {
                    focus_element(el);
                }
                return result;
            }
        }

        let prev = walker_previous_focusable(
            &mut walker,
            opts.tabbable,
            opts.accept.as_ref(),
            from_radio_group.as_deref(),
        );

        let result = if prev.is_none() && opts.wrap {
            find_last_focusable(
                &mut walker,
                &scope,
                opts.tabbable,
                opts.accept.as_ref(),
                None,
            )
        } else {
            prev
        };

        if let Some(ref el) = result {
            focus_element(el);
        }
        result
    }

    /// Move focus to the first focusable element.
    #[allow(clippy::needless_pass_by_value)]
    pub fn focus_first(&self, opts: FocusManagerOptions) -> Option<web_sys::Element> {
        let scope = (self.get_scope)()?;
        let mut walker = create_scope_walker(&scope)?;

        let result = walker_next_focusable(&mut walker, opts.tabbable, opts.accept.as_ref(), None);

        if let Some(ref el) = result {
            focus_element(el);
        }
        result
    }

    /// Move focus to the last focusable element.
    #[allow(clippy::needless_pass_by_value)]
    pub fn focus_last(&self, opts: FocusManagerOptions) -> Option<web_sys::Element> {
        let scope = (self.get_scope)()?;
        let mut walker = create_scope_walker(&scope)?;

        let result = find_last_focusable(
            &mut walker,
            &scope,
            opts.tabbable,
            opts.accept.as_ref(),
            None,
        );

        if let Some(ref el) = result {
            focus_element(el);
        }
        result
    }
}

/// Create a `ShadowTreeWalker` rooted at `scope` that visits all element nodes,
/// descending into shadow roots.
fn create_scope_walker(scope: &web_sys::Element) -> Option<ShadowTreeWalker> {
    // 0x1 = NodeFilter.SHOW_ELEMENT
    shadow_tree_walker::create_shadow_tree_walker(scope.as_ref(), 0x1, None)
}

/// Advance to the next focusable/tabbable element using the walker.
fn walker_next_focusable(
    walker: &mut ShadowTreeWalker,
    tabbable_only: bool,
    accept: Option<&Arc<dyn Fn(&web_sys::Element) -> bool + Send + Sync>>,
    from_radio_group: Option<&str>,
) -> Option<web_sys::Element> {
    while let Some(node) = walker.next_node() {
        let Some(el) = node.dyn_ref::<web_sys::Element>() else {
            continue;
        };

        if is_acceptable(el, tabbable_only, accept, from_radio_group) {
            return Some(el.clone());
        }
    }
    None
}

/// Walk backwards to the previous focusable/tabbable element.
fn walker_previous_focusable(
    walker: &mut ShadowTreeWalker,
    tabbable_only: bool,
    accept: Option<&Arc<dyn Fn(&web_sys::Element) -> bool + Send + Sync>>,
    from_radio_group: Option<&str>,
) -> Option<web_sys::Element> {
    while let Some(node) = walker.previous_node() {
        let Some(el) = node.dyn_ref::<web_sys::Element>() else {
            continue;
        };

        if is_acceptable(el, tabbable_only, accept, from_radio_group) {
            return Some(el.clone());
        }
    }
    None
}

/// Check whether an element passes all focusability, tabbability, radio group,
/// and accept-filter checks.
fn is_acceptable(
    el: &web_sys::Element,
    tabbable_only: bool,
    accept: Option<&Arc<dyn Fn(&web_sys::Element) -> bool + Send + Sync>>,
    from_radio_group: Option<&str>,
) -> bool {
    if tabbable_only {
        // Radio group handling: skip non-tabbable radios and radios in the
        // same group as the starting element.
        if let Some(input) = el.dyn_ref::<web_sys::HtmlInputElement>() {
            if input.type_() == "radio" {
                if !focusability::is_tabbable_radio(input) {
                    return false;
                }
                if let Some(group) = from_radio_group {
                    if input.name() == group {
                        return false;
                    }
                }
            }
        }
        if !focusability::is_tabbable(el) {
            return false;
        }
    } else if !focusability::is_focusable(el) {
        return false;
    }

    if let Some(accept_fn) = accept {
        if !accept_fn(el) {
            return false;
        }
    }

    true
}

/// Navigate the walker to the deepest last descendant of `scope`.
fn set_walker_to_last_descendant(walker: &ShadowTreeWalker, scope: &web_sys::Element) {
    let mut node: web_sys::Node = scope.clone().into();
    while let Some(last) = node.last_child() {
        node = last;
    }
    walker.set_current_node(&node);
}

/// Find the first focusable element within the scope.
fn find_first_focusable(
    walker: &mut ShadowTreeWalker,
    scope: &web_sys::Element,
    tabbable_only: bool,
    accept: Option<&Arc<dyn Fn(&web_sys::Element) -> bool + Send + Sync>>,
    from_radio_group: Option<&str>,
) -> Option<web_sys::Element> {
    walker.set_current_node(scope);
    walker_next_focusable(walker, tabbable_only, accept, from_radio_group)
}

/// Find the last focusable element within the scope.
fn find_last_focusable(
    walker: &mut ShadowTreeWalker,
    scope: &web_sys::Element,
    tabbable_only: bool,
    accept: Option<&Arc<dyn Fn(&web_sys::Element) -> bool + Send + Sync>>,
    from_radio_group: Option<&str>,
) -> Option<web_sys::Element> {
    set_walker_to_last_descendant(walker, scope);
    let last_node = walker.current_node();
    if let Some(el) = last_node.dyn_ref::<web_sys::Element>() {
        if is_acceptable(el, tabbable_only, accept, from_radio_group) {
            return Some(el.clone());
        }
    }
    walker_previous_focusable(walker, tabbable_only, accept, from_radio_group)
}

/// Focus an element, allowing the browser to scroll it into view.
///
/// React-aria's `createFocusManager` uses standard `element.focus()` (with scroll)
/// for programmatic navigation, rather than `focusSafely` (which prevents scroll).
fn focus_element(element: &web_sys::Element) {
    crate::utils::focus::focus_element(element, false);
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

impl IntoAttrs for UseFocusManagerProps {
    type Attrs = UseFocusManagerAttrs;

    fn into_attrs(self) -> Self::Attrs {
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
