use std::sync::Arc;

use send_wrapper::SendWrapper;
use wasm_bindgen::JsCast;

use crate::{
    hooks::IntoAttrs,
    utils::{
        dom_ext::node_contains,
        element_capture::{CapturedElement, ElementCaptureAttr},
        focusability,
        focusable_tree_walker::{FocusableTreeWalkerOptions, get_focusable_tree_walker},
        shadow_dom,
        shadow_tree_walker::ShadowTreeWalker,
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
    #[cfg(not(feature = "ssr"))]
    get_scope: Arc<SendWrapper<Box<dyn Fn() -> Option<web_sys::Element>>>>,
    #[cfg(feature = "ssr")]
    get_scope: Arc<dyn Fn() -> Option<web_sys::Element> + Send + Sync>,
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
        #[cfg(feature = "ssr")]
        {
            let _ = get_scope;
            Self {
                get_scope: Arc::new(|| None),
            }
        }

        #[cfg(not(feature = "ssr"))]
        {
            Self {
                get_scope: Arc::new(SendWrapper::new(Box::new(get_scope))),
            }
        }
    }

    /// Move focus to the next focusable element.
    #[allow(clippy::needless_pass_by_value)]
    pub fn focus_next(&self, opts: FocusManagerOptions) -> Option<web_sys::Element> {
        let scope = (self.get_scope)()?;

        let from = opts.from.or_else(|| {
            scope
                .owner_document()
                .and_then(|d| shadow_dom::get_active_element(&d))
        });

        let from_radio_group = from.as_ref().and_then(focusability::get_radio_group_name);

        let from_in_scope = from.as_ref().is_some_and(|from_el| {
            node_contains(Some(scope.as_ref()), Some(from_el.as_ref())).unwrap_or(false)
        });

        if let Some(ref from_el) = from {
            if from_in_scope {
                // Normal case: advance from current position.
                let mut walker = get_focusable_tree_walker(
                    &scope,
                    FocusableTreeWalkerOptions {
                        tabbable: opts.tabbable,
                        from: Some(from_el.clone()),
                        from_radio_group,
                        accept: opts.accept.clone(),
                    },
                )?;

                if let Some(el) = walker_next(&mut walker) {
                    focus_element(&el);
                    return Some(el);
                }

                // Wrap around: create a fresh walker without from_radio_group.
                if opts.wrap {
                    let mut wrap_walker = get_focusable_tree_walker(
                        &scope,
                        FocusableTreeWalkerOptions {
                            tabbable: opts.tabbable,
                            accept: opts.accept,
                            ..Default::default()
                        },
                    )?;
                    let result = walker_next(&mut wrap_walker);
                    if let Some(ref el) = result {
                        focus_element(el);
                    }
                    return result;
                }

                return None;
            }
        }

        // From outside scope or no from: focus first element (react-aria behavior).
        let mut walker = get_focusable_tree_walker(
            &scope,
            FocusableTreeWalkerOptions {
                tabbable: opts.tabbable,
                accept: opts.accept,
                ..Default::default()
            },
        )?;
        let result = walker_next(&mut walker);
        if let Some(ref el) = result {
            focus_element(el);
        }
        result
    }

    /// Move focus to the previous focusable element.
    #[allow(clippy::needless_pass_by_value)]
    pub fn focus_previous(&self, opts: FocusManagerOptions) -> Option<web_sys::Element> {
        let scope = (self.get_scope)()?;

        let from = opts.from.or_else(|| {
            scope
                .owner_document()
                .and_then(|d| shadow_dom::get_active_element(&d))
        });

        let from_radio_group = from.as_ref().and_then(focusability::get_radio_group_name);

        let from_in_scope = from.as_ref().is_some_and(|from_el| {
            node_contains(Some(scope.as_ref()), Some(from_el.as_ref())).unwrap_or(false)
        });

        if let Some(ref from_el) = from {
            if from_in_scope {
                // Normal case: walk backwards from current position.
                let mut walker = get_focusable_tree_walker(
                    &scope,
                    FocusableTreeWalkerOptions {
                        tabbable: opts.tabbable,
                        from: Some(from_el.clone()),
                        from_radio_group,
                        accept: opts.accept.clone(),
                    },
                )?;

                if let Some(el) = walker_previous(&mut walker) {
                    focus_element(&el);
                    return Some(el);
                }

                // Wrap around: create a fresh walker without from_radio_group.
                if opts.wrap {
                    let mut wrap_walker = get_focusable_tree_walker(
                        &scope,
                        FocusableTreeWalkerOptions {
                            tabbable: opts.tabbable,
                            accept: opts.accept,
                            ..Default::default()
                        },
                    )?;
                    let result = find_last_focusable(&mut wrap_walker, &scope);
                    if let Some(ref el) = result {
                        focus_element(el);
                    }
                    return result;
                }

                return None;
            }
        }

        // From outside scope or no from: focus last element (react-aria behavior).
        let mut walker = get_focusable_tree_walker(
            &scope,
            FocusableTreeWalkerOptions {
                tabbable: opts.tabbable,
                accept: opts.accept,
                ..Default::default()
            },
        )?;
        let result = find_last_focusable(&mut walker, &scope);
        if let Some(ref el) = result {
            focus_element(el);
        }
        result
    }

    /// Move focus to the first focusable element.
    #[allow(clippy::needless_pass_by_value)]
    pub fn focus_first(&self, opts: FocusManagerOptions) -> Option<web_sys::Element> {
        let result = self.find_first(opts);
        if let Some(ref el) = result {
            focus_element(el);
        }
        result
    }

    /// Move focus to the last focusable element.
    #[allow(clippy::needless_pass_by_value)]
    pub fn focus_last(&self, opts: FocusManagerOptions) -> Option<web_sys::Element> {
        let result = self.find_last(opts);
        if let Some(ref el) = result {
            focus_element(el);
        }
        result
    }

    /// Find the first focusable element without focusing it.
    ///
    /// Use this when the caller needs to apply its own focus strategy
    /// (e.g., `focus_safely` for overlay auto-focus instead of standard
    /// `focus()` which allows scrolling).
    #[allow(clippy::needless_pass_by_value)]
    pub fn find_first(&self, opts: FocusManagerOptions) -> Option<web_sys::Element> {
        let scope = (self.get_scope)()?;
        let mut walker = get_focusable_tree_walker(
            &scope,
            FocusableTreeWalkerOptions {
                tabbable: opts.tabbable,
                accept: opts.accept,
                ..Default::default()
            },
        )?;
        walker_next(&mut walker)
    }

    /// Find the last focusable element without focusing it.
    ///
    /// Use this when the caller needs to apply its own focus strategy
    /// (e.g., `focus_safely` for overlay auto-focus).
    #[allow(clippy::needless_pass_by_value)]
    pub fn find_last(&self, opts: FocusManagerOptions) -> Option<web_sys::Element> {
        let scope = (self.get_scope)()?;
        let mut walker = get_focusable_tree_walker(
            &scope,
            FocusableTreeWalkerOptions {
                tabbable: opts.tabbable,
                accept: opts.accept,
                ..Default::default()
            },
        )?;
        find_last_focusable(&mut walker, &scope)
    }
}

/// Cast-wrapper: advance to the next node and try to cast to `Element`.
fn walker_next(walker: &mut ShadowTreeWalker) -> Option<web_sys::Element> {
    walker
        .next_node()
        .and_then(|n| n.dyn_into::<web_sys::Element>().ok())
}

/// Cast-wrapper: move to the previous node and try to cast to `Element`.
fn walker_previous(walker: &mut ShadowTreeWalker) -> Option<web_sys::Element> {
    walker
        .previous_node()
        .and_then(|n| n.dyn_into::<web_sys::Element>().ok())
}

/// Navigate the walker to the deepest last descendant of `scope`.
fn set_walker_to_last_descendant(walker: &ShadowTreeWalker, scope: &web_sys::Element) {
    let mut node: web_sys::Node = scope.clone().into();
    while let Some(last) = node.last_child() {
        node = last;
    }
    walker.set_current_node(&node);
}

/// Find the last focusable element within the scope.
///
/// Positions the walker at the deepest last descendant and walks backwards.
/// The current node at that position might itself be focusable, so we check
/// it via `matches_filter` before falling back to `walker_previous`.
fn find_last_focusable(
    walker: &mut ShadowTreeWalker,
    scope: &web_sys::Element,
) -> Option<web_sys::Element> {
    set_walker_to_last_descendant(walker, scope);
    let last_node = walker.current_node();
    if walker.matches_filter(&last_node) {
        return last_node.dyn_into::<web_sys::Element>().ok();
    }
    walker_previous(walker)
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
