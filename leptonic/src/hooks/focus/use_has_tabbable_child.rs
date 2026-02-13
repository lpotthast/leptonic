use leptos::prelude::*;
use wasm_bindgen::{prelude::*, JsCast};

use crate::{
    hooks::IntoAttrs,
    utils::{
        element_capture::{CapturedElement, ElementCaptureAttr},
        focusability, shadow_tree_walker,
    },
};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/focus/src/useHasTabbableChild.ts
//
// ## React-aria deviation
//
// **React-aria pattern**: `useHasTabbableChild(scopeRef)` - caller passes a ref as parameter.
//
// **Leptonic pattern**: `use_has_tabbable_child(input)` returns props with `ElementCaptureAttr` that
// automatically captures the element when spread.
//
// This is a deliberate deviation for better ergonomics - users don't need to manually create
// and wire up NodeRefs. The element is captured automatically when attributes are spread.

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## DIFFERENT BEHAVIOR
//
// - Element capture pattern vs caller-provided ref
//   React-aria: `useHasTabbableChild(scopeRef)` takes a ref as parameter.
//   Leptonic: Returns `ElementCaptureAttr` for automatic element capture.
//
// ## OMITTED FUNCTIONALITY
//
// - No radio button group handling
//   React-aria's `isTabbableRadio()` ensures only one radio per group is tabbable.
//   `focusability::is_tabbable_radio` is now available but is not used here because
//   at least one radio in any group is always tabbable, so the boolean result of
//   `has_tabbable_child` is unaffected.
//
// =============================================================================

/// Input parameters for the `use_has_tabbable_child` hook.
#[derive(Debug, Clone, Copy)]
pub struct UseHasTabbableChildInput {
    /// Whether the check should be disabled.
    pub disabled: Signal<bool>,
}

impl Default for UseHasTabbableChildInput {
    fn default() -> Self {
        Self {
            disabled: Signal::derive(|| false),
        }
    }
}

/// The return value of the `use_has_tabbable_child` hook.
#[derive(Debug)]
pub struct UseHasTabbableChildReturn {
    /// Whether the element has at least one tabbable child.
    pub has_tabbable_child: Signal<bool>,

    /// Props for programmatic merging. Call `.into_attrs()` for view spreading.
    pub props: UseHasTabbableChildProps,
}

/// Props from `use_has_tabbable_child` that can be extracted and merged programmatically.
#[derive(Debug)]
pub struct UseHasTabbableChildProps {
    pub element_capture: ElementCaptureAttr,
}

impl IntoAttrs for UseHasTabbableChildProps {
    type Attrs = UseHasTabbableChildAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (self.element_capture,)
    }
}

/// These attributes must be spread onto the target element: `<foo {..attrs} />`
pub type UseHasTabbableChildAttrs = (ElementCaptureAttr,);

/// Checks whether an element has any tabbable (focusable via Tab key) children.
///
/// This is useful for determining whether to make a container itself focusable
/// when it might contain focusable children.
///
/// The hook automatically captures the DOM element through [`ElementCaptureAttr`],
/// so you don't need to create or pass a `NodeRef`. Just spread the props
/// onto your element and the detection works automatically.
///
/// A `MutationObserver` watches for child additions/removals and attribute changes
/// (`tabindex`, `disabled`) to keep the result up-to-date with dynamic content.
///
/// # Example
///
/// ```ignore
/// let UseHasTabbableChildReturn { has_tabbable_child, props } =
///     use_has_tabbable_child(UseHasTabbableChildInput::default());
///
/// view! {
///     <div
///         {..props.into_attrs()}
///         tabindex=move || if has_tabbable_child.get() { -1 } else { 0 }
///     >
///         <button>"Child button"</button>
///     </div>
/// }
/// ```
pub fn use_has_tabbable_child(input: UseHasTabbableChildInput) -> UseHasTabbableChildReturn {
    let UseHasTabbableChildInput { disabled } = input;

    let (has_tabbable_child, set_has_tabbable_child) = signal(false);

    let element = CapturedElement::new();

    // Store cleanup for MutationObserver.
    #[cfg(not(feature = "ssr"))]
    let observer_cleanup: StoredValue<Option<Box<dyn Fn()>>, LocalStorage> =
        StoredValue::new_local(None);

    // Check for tabbable children when element is captured or disabled changes.
    // Also sets up a MutationObserver for dynamic child changes.
    Effect::new(move |_| {
        // Clean up previous observer.
        #[cfg(not(feature = "ssr"))]
        observer_cleanup.update_value(|cleanup| {
            if let Some(cleanup_fn) = cleanup.take() {
                cleanup_fn();
            }
        });

        if disabled.get() {
            set_has_tabbable_child.set(false);
            return;
        }

        let Some(el) = element.get() else {
            set_has_tabbable_child.set(false);
            return;
        };

        // Initial check.
        let has_tabbable = has_tabbable_element(&el);
        set_has_tabbable_child.set(has_tabbable);

        // Set up MutationObserver to detect dynamic child changes.
        #[cfg(not(feature = "ssr"))]
        {
            let el_clone = (*el).clone();
            let callback: Closure<dyn FnMut(js_sys::Array, web_sys::MutationObserver)> =
                Closure::new(
                    move |_mutations: js_sys::Array, _observer: web_sys::MutationObserver| {
                        let has = has_tabbable_element(&el_clone);
                        set_has_tabbable_child.set(has);
                    },
                );

            if let Ok(observer) = web_sys::MutationObserver::new(callback.as_ref().unchecked_ref())
            {
                let options = web_sys::MutationObserverInit::new();
                options.set_subtree(true);
                options.set_child_list(true);
                options.set_attributes(true);
                let filter = js_sys::Array::new();
                filter.push(&"tabindex".into());
                filter.push(&"disabled".into());
                options.set_attribute_filter(&filter);

                if observer.observe_with_options(&el, &options).is_ok() {
                    callback.forget();
                    observer_cleanup.set_value(Some(Box::new(move || {
                        observer.disconnect();
                    })));
                }
            }
        }
    });

    // Cleanup observer on unmount.
    #[cfg(not(feature = "ssr"))]
    on_cleanup(move || {
        observer_cleanup.update_value(|cleanup| {
            if let Some(cleanup_fn) = cleanup.take() {
                cleanup_fn();
            }
        });
    });

    UseHasTabbableChildReturn {
        has_tabbable_child: has_tabbable_child.into(),
        props: UseHasTabbableChildProps {
            element_capture: element.attr(),
        },
    }
}

/// Check if an element has any tabbable descendant elements.
///
/// Uses a `ShadowTreeWalker` to lazily iterate descendant elements (descending into
/// shadow roots), stopping at the first tabbable match. This mirrors react-aria's
/// approach of using `getFocusableTreeWalker` with `{tabbable: true}` followed by
/// `!!walker.nextNode()`.
fn has_tabbable_element(element: &web_sys::Element) -> bool {
    // 0x1 = NodeFilter.SHOW_ELEMENT
    let Some(mut walker) =
        shadow_tree_walker::create_shadow_tree_walker(element.as_ref(), 0x1, None)
    else {
        return false;
    };

    while let Some(node) = walker.next_node() {
        if let Some(el) = node.dyn_ref::<web_sys::Element>() {
            if focusability::is_tabbable(el) {
                return true;
            }
        }
    }

    false
}
