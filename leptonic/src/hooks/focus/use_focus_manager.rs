// Upstream: react-aria/src/focus/FocusScope.tsx @ 99e6102368
// Upstream: react-aria/test/focus/FocusScope.test.js @ 99e6102368
use std::sync::Arc;

use leptos::prelude::*;
use leptos_element_capture::{CapturedElement, ElementCaptureAttr};
use send_wrapper::SendWrapper;
#[cfg(not(feature = "ssr"))]
use wasm_bindgen::JsCast;

use crate::IntoAttrs;
pub use crate::utils::focusability::Focusability;
#[cfg(not(feature = "ssr"))]
use crate::utils::{
    focusable_tree_walker::{FocusableTreeWalkerOptions, get_focusable_tree_walker},
    shadow_dom,
    shadow_tree_walker::ShadowTreeWalker,
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - `create_focus_manager()` returns the manager plus an `ElementCaptureAttr` to spread onto the
//   scope element: no `RefObject` to create and pass (react-aria: `createFocusManager(ref)`).
//   `use_focus_manager_context()` is react-aria's `useFocusManager()`: the manager of the
//   enclosing `FocusScope` (or `FocusManagerProvider`), provided as a `FocusManager` context.
// - `FocusManager` is `Copy` (its state lives in a `StoredValue` of the owner that created it);
//   once that owner is gone, the methods find nothing.
// - The default options of `createFocusManager(ref, defaultOptions)` are a default filter only
//   (`FocusManager::with_default_accept`): `from`, `wrap` and `focusability` describe a single
//   call, and `FocusManagerOptions` names them all in each call (struct literal or `Default`).
// - `focusability: Focusability` instead of `tabbable?: boolean`.
// - `find_first`/`find_last` return the element without focusing it, for callers with their own
//   focus strategy (e.g. `focus_safely` without scrolling). React-aria has no such methods.
//
// =============================================================================

/// A filter of the elements a focus manager may move to.
pub type AcceptElement = Arc<dyn Fn(&web_sys::Element) -> bool + Send + Sync>;

/// Options for focus movement.
#[derive(Clone, Default)]
pub struct FocusManagerOptions {
    /// Element to start navigation from. Defaults to the focused element.
    pub from: Option<web_sys::Element>,

    /// Whether to wrap around when reaching the end.
    pub wrap: bool,

    /// Which elements to move to: every focusable one (default), or only tabbable ones.
    pub focusability: Focusability,

    /// Custom filter function for acceptable elements.
    /// Unlike a bare `fn` pointer, this accepts closures that capture state.
    pub accept: Option<AcceptElement>,
}

impl std::fmt::Debug for FocusManagerOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FocusManagerOptions")
            .field("from", &self.from)
            .field("wrap", &self.wrap)
            .field("focusability", &self.focusability)
            .field("accept", &self.accept.as_ref().map(|_| ".."))
            .finish()
    }
}

/// The state of a [`FocusManager`].
struct FocusManagerState {
    /// Returns the current scope element.
    #[cfg(not(feature = "ssr"))]
    get_scope: SendWrapper<Box<dyn Fn() -> Option<web_sys::Element>>>,
    /// The filter of calls that don't pass one (react-aria's default options of
    /// `createFocusManager`).
    default_accept: Option<AcceptElement>,
}

/// A focus manager provides methods for moving focus within a scope.
///
/// `Copy`: its state lives in a `StoredValue` of the reactive owner that created it. Once that
/// owner is disposed, the methods find nothing (a pending callback may still hold the manager of
/// an unmounted scope).
#[derive(Clone, Copy)]
pub struct FocusManager {
    state: StoredValue<FocusManagerState>,
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
        let _ = get_scope;
        Self {
            state: StoredValue::new(FocusManagerState {
                #[cfg(not(feature = "ssr"))]
                get_scope: SendWrapper::new(Box::new(get_scope)),
                default_accept: None,
            }),
        }
    }

    /// Only elements `accept` accepts, unless a call passes its own filter (e.g. a date range
    /// picker's segments without its button).
    #[must_use]
    pub fn with_default_accept(
        self,
        accept: impl Fn(&web_sys::Element) -> bool + Send + Sync + 'static,
    ) -> Self {
        self.state
            .update_value(|state| state.default_accept = Some(Arc::new(accept)));
        self
    }

    /// The scope element and the options with the default filter applied; `None` without a scope
    /// (also once the manager's owner is gone).
    #[cfg(not(feature = "ssr"))]
    fn resolve(
        self,
        mut opts: FocusManagerOptions,
    ) -> Option<(web_sys::Element, FocusManagerOptions)> {
        let scope = self.state.try_with_value(|state| {
            if opts.accept.is_none() {
                opts.accept.clone_from(&state.default_accept);
            }
            (state.get_scope)()
        })??;
        Some((scope, opts))
    }

    /// Move focus to the next focusable element: after `from` (or the focused element) if it is
    /// in the scope, else to the first one.
    #[allow(clippy::needless_pass_by_value)]
    pub fn focus_next(self, opts: FocusManagerOptions) -> Option<web_sys::Element> {
        #[cfg(feature = "ssr")]
        {
            let _ = opts;
            None
        }
        #[cfg(not(feature = "ssr"))]
        {
            let (scope, opts) = self.resolve(opts)?;
            let from = from_or_active(&scope, opts.from.clone());
            let mut walker = walker(&scope, &opts)?;
            if let Some(from) = from.filter(|from| shadow_dom::node_contains(&scope, from)) {
                walker.set_current_node(&from);
            }
            let mut next = walker_next(&mut walker);
            if next.is_none() && opts.wrap {
                walker.set_current_node(&scope);
                next = walker_next(&mut walker);
            }
            if let Some(next) = &next {
                focus_element(next);
            }
            next
        }
    }

    /// Move focus to the previous focusable element: before `from` (or the focused element) if it
    /// is in the scope, else to the last one.
    #[allow(clippy::needless_pass_by_value)]
    pub fn focus_previous(self, opts: FocusManagerOptions) -> Option<web_sys::Element> {
        #[cfg(feature = "ssr")]
        {
            let _ = opts;
            None
        }
        #[cfg(not(feature = "ssr"))]
        {
            let (scope, opts) = self.resolve(opts)?;
            let from = from_or_active(&scope, opts.from.clone());
            let mut walker = walker(&scope, &opts)?;
            let previous = match from.filter(|from| shadow_dom::node_contains(&scope, from)) {
                Some(from) => {
                    walker.set_current_node(&from);
                    walker_previous(&mut walker).or_else(|| {
                        opts.wrap.then(|| {
                            walker.set_current_node(&scope);
                            last(&mut walker)
                        })?
                    })
                }
                None => last(&mut walker),
            };
            if let Some(previous) = &previous {
                focus_element(previous);
            }
            previous
        }
    }

    /// Move focus to the first focusable element.
    pub fn focus_first(self, opts: FocusManagerOptions) -> Option<web_sys::Element> {
        let first = self.find_first(opts);
        #[cfg(not(feature = "ssr"))]
        if let Some(first) = &first {
            focus_element(first);
        }
        first
    }

    /// Move focus to the last focusable element.
    pub fn focus_last(self, opts: FocusManagerOptions) -> Option<web_sys::Element> {
        let last = self.find_last(opts);
        #[cfg(not(feature = "ssr"))]
        if let Some(last) = &last {
            focus_element(last);
        }
        last
    }

    /// Find the first focusable element without focusing it.
    ///
    /// Use this when the caller needs to apply its own focus strategy
    /// (e.g., `focus_safely` for overlay auto-focus instead of standard
    /// `focus()` which allows scrolling).
    #[allow(clippy::needless_pass_by_value)]
    pub fn find_first(self, opts: FocusManagerOptions) -> Option<web_sys::Element> {
        #[cfg(feature = "ssr")]
        {
            let _ = opts;
            None
        }
        #[cfg(not(feature = "ssr"))]
        {
            let (scope, opts) = self.resolve(opts)?;
            walker_next(&mut walker(&scope, &opts)?)
        }
    }

    /// Find the last focusable element without focusing it.
    ///
    /// Use this when the caller needs to apply its own focus strategy
    /// (e.g., `focus_safely` for overlay auto-focus).
    #[allow(clippy::needless_pass_by_value)]
    pub fn find_last(self, opts: FocusManagerOptions) -> Option<web_sys::Element> {
        #[cfg(feature = "ssr")]
        {
            let _ = opts;
            None
        }
        #[cfg(not(feature = "ssr"))]
        {
            let (scope, opts) = self.resolve(opts)?;
            last(&mut walker(&scope, &opts)?)
        }
    }
}

/// `from`, else the focused element of the scope's document.
#[cfg(not(feature = "ssr"))]
fn from_or_active(
    scope: &web_sys::Element,
    from: Option<web_sys::Element>,
) -> Option<web_sys::Node> {
    from.or_else(|| {
        scope
            .owner_document()
            .and_then(|document| shadow_dom::get_active_element(&document))
    })
    .map(Into::into)
}

/// A walker over the scope's elements the options accept, at the scope element.
#[cfg(not(feature = "ssr"))]
fn walker(scope: &web_sys::Element, opts: &FocusManagerOptions) -> Option<ShadowTreeWalker> {
    get_focusable_tree_walker(
        scope,
        FocusableTreeWalkerOptions {
            focusability: opts.focusability,
            accept: opts.accept.clone(),
        },
    )
}

/// Cast-wrapper: advance to the next node and try to cast to `Element`.
#[cfg(not(feature = "ssr"))]
fn walker_next(walker: &mut ShadowTreeWalker) -> Option<web_sys::Element> {
    walker
        .next_node()
        .and_then(|n| n.dyn_into::<web_sys::Element>().ok())
}

/// Cast-wrapper: move to the previous node and try to cast to `Element`.
#[cfg(not(feature = "ssr"))]
fn walker_previous(walker: &mut ShadowTreeWalker) -> Option<web_sys::Element> {
    walker
        .previous_node()
        .and_then(|n| n.dyn_into::<web_sys::Element>().ok())
}

/// The last accepted element below the walker's node: its last child, that one's last child,
/// ... (react-aria's `last`).
#[cfg(not(feature = "ssr"))]
fn last(walker: &mut ShadowTreeWalker) -> Option<web_sys::Element> {
    let mut last = None;
    while let Some(node) = walker.last_child() {
        last = Some(node);
    }
    last.and_then(|n| n.dyn_into::<web_sys::Element>().ok())
}

/// Focus an element, allowing the browser to scroll it into view.
///
/// React-aria's `createFocusManager` uses standard `element.focus()` (with scroll)
/// for programmatic navigation, rather than `focusSafely` (which prevents scroll).
#[cfg(not(feature = "ssr"))]
fn focus_element(element: &web_sys::Element) {
    crate::utils::focus::focus_element(element, false);
}

/// The return value of [`create_focus_manager`].
pub struct CreateFocusManagerReturn {
    /// The focus manager instance.
    pub focus_manager: FocusManager,

    /// Props for programmatic merging. Call `.into_attrs()` for view spreading.
    pub props: FocusManagerScopeProps,
}

impl std::fmt::Debug for CreateFocusManagerReturn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CreateFocusManagerReturn")
            .field("focus_manager", &self.focus_manager)
            .field("props", &self.props)
            .finish()
    }
}

/// Props of the scope element of [`create_focus_manager`].
#[derive(Debug)]
pub struct FocusManagerScopeProps {
    pub element_capture: ElementCaptureAttr,
}

impl IntoAttrs for FocusManagerScopeProps {
    type Attrs = FocusManagerScopeAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (self.element_capture,)
    }
}

/// These attributes must be spread onto the scope element: `<foo {..attrs} />`
pub type FocusManagerScopeAttrs = (ElementCaptureAttr,);

/// Creates a focus manager for navigating focus within a scope element (react-aria's
/// `createFocusManager`).
///
/// The focus manager provides methods to programmatically move focus
/// between focusable elements within a container.
///
/// The props capture the scope element through [`ElementCaptureAttr`], so you don't need to
/// create or pass a `NodeRef`: spread them onto the container.
///
/// # Example
///
/// ```ignore
/// let CreateFocusManagerReturn { focus_manager, props } = create_focus_manager();
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
pub fn create_focus_manager() -> CreateFocusManagerReturn {
    let scope_element = CapturedElement::new();

    CreateFocusManagerReturn {
        // The manager's state and the capture belong to the same owner: the manager reads the
        // capture only while it lives.
        focus_manager: FocusManager::new(move || {
            scope_element.get_untracked().map(SendWrapper::take)
        }),
        props: FocusManagerScopeProps {
            element_capture: scope_element.attr(),
        },
    }
}

/// The focus manager of the enclosing `FocusScope` (or `FocusManagerProvider`), react-aria's
/// `useFocusManager`. `None` outside of one.
pub fn use_focus_manager_context() -> Option<FocusManager> {
    use_context::<FocusManager>()
}
