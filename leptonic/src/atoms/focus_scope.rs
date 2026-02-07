use leptos::html;
use leptos::prelude::*;
use leptos_use::use_event_listener;
use wasm_bindgen::JsCast;

use crate::hooks::{FocusManager, FocusManagerOptions};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/focus/src/FocusScope.tsx

/// Context for accessing the focus manager within a `FocusScope`.
///
/// Child components can use this context to programmatically move focus.
///
/// # Example
///
/// ```ignore
/// <FocusScope contain=true>
///     <MyMenuComponent />
/// </FocusScope>
///
/// // In MyMenuComponent:
/// fn MyMenuComponent() -> impl IntoView {
///     let ctx = expect_context::<FocusScopeContext>();
///     // Use ctx.focus_manager to move focus
/// }
/// ```
#[derive(Clone)]
pub struct FocusScopeContext {
    /// The focus manager for the scope.
    pub focus_manager: FocusManager,
}

/// A scope that contains and manages focus.
///
/// `FocusScope` provides:
/// - **Focus containment**: When `contain` is true, Tab/Shift+Tab navigation
///   wraps within the scope, preventing focus from leaving.
/// - **Focus restoration**: When `restore_focus` is true, focus returns to
///   the previously focused element when the scope unmounts.
/// - **Auto-focus**: When `auto_focus` is true, focus moves to the first
///   focusable element in the scope when it mounts.
///
/// This is useful for dialogs, menus, and other overlays that need to trap
/// focus for accessibility.
///
/// # Example
///
/// ```ignore
/// <FocusScope contain=true restore_focus=true auto_focus=true>
///     <input type="text" placeholder="First" />
///     <button>"Action"</button>
///     <input type="text" placeholder="Last" />
/// </FocusScope>
/// ```
#[component]
pub fn FocusScope(
    /// Whether to contain focus within the scope.
    #[prop(default = false)]
    contain: bool,

    /// Whether to restore focus when the scope unmounts.
    #[prop(default = false)]
    restore_focus: bool,

    /// Whether to auto-focus the first focusable element.
    #[prop(default = false)]
    auto_focus: bool,

    /// The content of the focus scope.
    children: Children,
) -> impl IntoView {
    let scope_ref = NodeRef::<html::Div>::new();

    // Store the previously focused element for restoration
    let previously_focused: StoredValue<Option<web_sys::Element>, LocalStorage> =
        StoredValue::new_local(None);

    // Capture the currently focused element when the scope mounts
    if restore_focus {
        if let Some(document) = web_sys::window().and_then(|w| w.document()) {
            previously_focused.set_value(document.active_element());
        }
    }

    // Create focus manager with a getter that reads from the NodeRef
    let focus_manager =
        FocusManager::new(move || scope_ref.get().map(|el| -> web_sys::Element { el.into() }));

    // Provide context so child components can access the focus manager
    provide_context(FocusScopeContext {
        focus_manager: focus_manager.clone(),
    });

    // Auto-focus on mount
    if auto_focus {
        let fm = focus_manager.clone();
        Effect::new(move |prev_run: Option<bool>| {
            if prev_run.is_some() {
                return true;
            }

            if scope_ref.get().is_some() {
                fm.focus_first(FocusManagerOptions {
                    tabbable: true,
                    ..Default::default()
                });
            }

            true
        });
    }

    // Focus containment via keydown handler
    if contain {
        let fm = focus_manager.clone();

        Effect::new(move |_| {
            let Some(scope_el) = scope_ref.get() else {
                return;
            };

            let scope_element: web_sys::Element = scope_el.clone().into();
            let fm = fm.clone();

            let _cleanup = use_event_listener(
                scope_el,
                leptos::ev::keydown,
                move |e: web_sys::KeyboardEvent| {
                    if e.key() != "Tab" {
                        return;
                    }

                    // Check if focus is within the scope
                    let document = web_sys::window().and_then(|w| w.document());
                    let active_element = document
                        .as_ref()
                        .and_then(web_sys::Document::active_element);

                    let is_within = active_element.as_ref().is_some_and(|active| {
                        if let Some(active_node) = active.dyn_ref::<web_sys::Node>() {
                            scope_element
                                .dyn_ref::<web_sys::Node>()
                                .is_some_and(|scope_node| scope_node.contains(Some(active_node)))
                        } else {
                            false
                        }
                    });

                    if !is_within {
                        return;
                    }

                    e.prevent_default();

                    let opts = FocusManagerOptions {
                        wrap: true,
                        tabbable: true,
                        ..Default::default()
                    };

                    if e.shift_key() {
                        fm.focus_previous(opts);
                    } else {
                        fm.focus_next(opts);
                    }
                },
            );
        });
    }

    // Restore focus on unmount
    on_cleanup(move || {
        if restore_focus {
            if let Some(element) = previously_focused.get_value() {
                if let Some(html_el) = element.dyn_ref::<web_sys::HtmlElement>() {
                    let _ = html_el.focus();
                }
            }
        }
    });

    view! { <div node_ref=scope_ref>{children()}</div> }
}
