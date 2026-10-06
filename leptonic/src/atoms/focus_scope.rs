// Upstream: react-aria/src/focus/FocusScope.tsx @ 6f664fe911
#![cfg_attr(feature = "ssr", allow(dead_code, unused_imports))]

use leptos::{html, prelude::*};
use leptos_use::{use_document, use_event_listener};
use wasm_bindgen::JsCast;

use crate::{
    hooks::{FocusManager, FocusManagerOptions},
    utils::{
        classes::Classes,
        focus_scope_tree::{self, FocusScopeParentContext},
        scoped_context::scoped_view,
        styles::Styles,
    },
};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/react-aria/src/focus/FocusScope.tsx

// REACT-ARIA DEVIATIONS
//
// ## DIFFERENT BEHAVIOR
//
// - Wrapper `<div style="display: contents">` instead of sentinel `<span>` elements
//   React-aria uses hidden sentinel `<span>` elements with no wrapper around
//   children. Leptonic uses a wrapper `<div>` (needed for `NodeRef`, `contains()`,
//   and event listeners) with `display: contents` to remove it from CSS layout.
//
// - Custom event name `leptonic-focus-scope-restore` instead of
//   `react-aria-focus-scope-restore`

/// Custom event dispatched before a `FocusScope` restores focus.
///
/// Listeners can call `e.prevent_default()` to cancel restoration (e.g.,
/// virtual collections may intercept this to handle focus themselves).
/// Matches react-aria's `RESTORE_FOCUS_EVENT`.
pub const RESTORE_FOCUS_EVENT: &str = "leptonic-focus-scope-restore";

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
#[allow(clippy::too_many_lines)]
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

    #[prop(into, optional)] classes: Classes,

    #[prop(into, optional)] styles: Styles,

    /// The content of the focus scope.
    children: Children,
) -> impl IntoView {
    crate::hooks::track_interaction_modality();
    cfg_if::cfg_if! {
        if #[cfg(feature = "ssr")] {
            let _ = (contain, restore_focus, auto_focus);
            let scope_ref = NodeRef::<html::Div>::new();
            scoped_view(
                || {
                    provide_context(FocusScopeContext {
                        focus_manager: FocusManager::new(|| None),
                    });
                },
                move || {
                    view! {
                        <div node_ref=scope_ref class=classes style=styles.add_unchecked("display", "contents")>
                            {children()}
                        </div>
                    }
                },
            )
        } else {
            let scope_ref = NodeRef::<html::Div>::new();

            // ---- Scope tree integration ----
            // Discover parent scope (if any) via Leptos context.
            let parent_id = use_context::<FocusScopeParentContext>().map(|ctx| ctx.scope_id);
            let scope_id = focus_scope_tree::allocate_id();

    // Capture the currently focused element when the scope mounts (before registration).
    let previously_focused: StoredValue<Option<web_sys::Element>, LocalStorage> =
        StoredValue::new_local(None);

    if restore_focus
        && let Some(document) = use_document().as_ref() {
            previously_focused.set_value(document.active_element());
        }

    // Register in the scope tree once the DOM element is available.
    // Also store node_to_restore in the tree for stacked overlay propagation.
    Effect::new(move |_| {
        if scope_ref.get().is_some() {
            focus_scope_tree::register_scope(
                scope_id,
                parent_id,
                move || {
                    scope_ref
                        .try_get_untracked()
                        .flatten()
                        .map(|el| -> web_sys::Element { el.into() })
                },
                contain,
            );
            if restore_focus {
                focus_scope_tree::set_node_to_restore(scope_id, previously_focused.get_value());
            }
            // Focus may have moved in before the scope was registered (e.g. a dialog focusing
            // itself on mount); react-aria tracks the active scope from a layout effect, before
            // such effects run. Later focus moves are tracked by a document-level listener.
            focus_scope_tree::ensure_active_scope_tracking();
            mark_active_if_focus_within(scope_id);
        }
    });

    // Create focus manager with a getter that reads from the NodeRef.
    // The focus manager can outlive the scope (e.g. in pending focus callbacks): a disposed
    // scope has no element.
    let focus_manager = FocusManager::new(move || {
        scope_ref
            .try_get_untracked()
            .flatten()
            .map(|el| -> web_sys::Element { el.into() })
    });

    // Auto-focus on mount (Issue 6: skip if focus is already within the scope).
    // Uses `focus_safely` (preventScroll: true) instead of the FocusManager's
    // standard `focus_first` (which allows scroll) to avoid triggering
    // scroll-based overlay dismissal via `use_close_on_scroll`.
    // Matches react-aria's FocusScope auto-focus behavior.
    if auto_focus {
        let fm = focus_manager.clone();
        Effect::new(move |prev_run: Option<bool>| {
            if prev_run.is_some() {
                return true;
            }

            if let Some(scope_el) = scope_ref.get() {
                let scope_node: web_sys::Node = scope_el.into();
                let already_focused = use_document()
                    .as_ref()
                    .and_then(web_sys::Document::active_element)
                    .and_then(|active| active.dyn_ref::<web_sys::Node>().cloned())
                    .is_some_and(|active| scope_node.contains(Some(&active)));

                if !already_focused {
                    // Try tabbable elements first, then fall back to any focusable
                    // element (e.g., tabindex="-1"). Matches react-aria's
                    // focusFirstInScope fallback behavior.
                    let element = fm
                        .find_first(FocusManagerOptions {
                            tabbable: true,
                            ..Default::default()
                        })
                        .or_else(|| {
                            fm.find_first(FocusManagerOptions {
                                tabbable: false,
                                ..Default::default()
                            })
                        });
                    if let Some(el) = element {
                        crate::utils::focus::focus_safely(&el);
                    }
                }
            }

            true
        });
    }

    // Focus containment via keydown handler and focusin listener.
    if contain {
        // Track the last focused element within the scope, so we can restore
        // focus to it when recapturing (rather than always jumping to the first).
        let focused_node: StoredValue<Option<web_sys::HtmlElement>, LocalStorage> =
            StoredValue::new_local(None);

        let fm = focus_manager.clone();

        // Keydown listener on document (Issue 2: document-level so we catch Tab
        // even when browser would move focus out of the scope element).
        let _keydown_cleanup = use_event_listener(
            use_document(),
            leptos::ev::keydown,
            move |e: web_sys::KeyboardEvent| {
                // Issue 1: Ignore modified Tab and composing input.
                if e.key() != "Tab"
                    || e.alt_key()
                    || e.ctrl_key()
                    || e.meta_key()
                    || e.is_composing()
                {
                    return;
                }

                // Only handle if this is the innermost containing scope.
                // If a nested child scope also has contain=true, it handles Tab.
                if !focus_scope_tree::is_innermost_container(scope_id) {
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

        // Scope-level focusin: track focused element within the containing scope.
        Effect::new(move |_| {
            let Some(scope_el) = scope_ref.get() else {
                return;
            };

            let _scope_focusin_cleanup = use_event_listener(
                scope_el,
                leptos::ev::focusin,
                move |e: web_sys::FocusEvent| {
                    if let Some(target) = e.target()
                        && let Some(html_el) = target.dyn_ref::<web_sys::HtmlElement>() {
                            focused_node.set_value(Some(html_el.clone()));
                        }
                },
            );
        });

        // Document-level focusin: detect when focus escapes and recapture.
        // Issue 5: Use scope tree to respect nested scopes.
        let fm_contain = focus_manager.clone();
        let _focusin_cleanup = use_event_listener(
            use_document(),
            leptos::ev::focusin,
            move |e: web_sys::FocusEvent| {
                // Only the scope that should contain focus recaptures.
                if !focus_scope_tree::should_contain_focus(scope_id) {
                    return;
                }

                let target = e.target();

                let focus_is_within = target.as_ref().is_some_and(|t| {
                    t.dyn_ref::<web_sys::Element>().is_some_and(|el| {
                        focus_scope_tree::is_element_in_scope_or_descendant(el, scope_id)
                    })
                });

                if focus_is_within {
                    // Focus moved within scope — update tracked node.
                    if let Some(t) = target.as_ref()
                        && let Some(html_el) = t.dyn_ref::<web_sys::HtmlElement>() {
                            focused_node.set_value(Some(html_el.clone()));
                        }
                } else {
                    // Focus escaped — recapture. Try the last focused node first,
                    // falling back to the first focusable element. After this `focusin`
                    // dispatch: refocusing synchronously would dispatch a nested `focusin` into
                    // this listener's closure, which is still running (and can't be re-entered).
                    let fm = send_wrapper::SendWrapper::new(fm_contain.clone());
                    queue_microtask(move || {
                        // The scope may have been unmounted (or stopped containing focus) since.
                        if !focus_scope_tree::should_contain_focus(scope_id) {
                            return;
                        }
                        let Some(last_focused) = focused_node.try_get_value() else {
                            return;
                        };
                        let recaptured = last_focused.is_some_and(|node| node.focus().is_ok());
                        if !recaptured {
                            fm.focus_first(FocusManagerOptions {
                                tabbable: true,
                                ..Default::default()
                            });
                        }
                    });
                }
            },
        );

        // Issue 3: Focusout handler — when focus leaves the scope (e.g., via
        // programmatic focus or click outside), recapture after focus settles.
        let fm_focusout = focus_manager.clone();
        Effect::new(move |_| {
            let Some(scope_el) = scope_ref.get() else {
                return;
            };

            let fm_focusout = fm_focusout.clone();
            let _focusout_cleanup = use_event_listener(
                scope_el,
                leptos::ev::focusout,
                move |_e: web_sys::FocusEvent| {
                    if !focus_scope_tree::should_contain_focus(scope_id) {
                        return;
                    }

                    // Defer check to after focus settles via requestAnimationFrame.
                    let fm = fm_focusout.clone();
                    request_animation_frame(move || {
                        // Android TalkBack workaround: skip focus restore when
                        // in virtual/unknown modality on Android Chrome (Chrome
                        // bug #384844019). Matches react-aria behavior.
                        let modality = crate::hooks::get_modality();
                        if matches!(
                            modality,
                            crate::hooks::Modality::Virtual | crate::hooks::Modality::Unknown
                        ) && crate::utils::platform::device::is_android()
                            && crate::utils::platform::browser::is_chrome()
                        {
                            return;
                        }

                        let active = use_document()
                            .as_ref()
                            .and_then(web_sys::Document::active_element);

                        let in_scope = active.as_ref().is_some_and(|a| {
                            focus_scope_tree::is_element_in_scope_or_descendant(a, scope_id)
                        });

                        if !in_scope {
                            fm.focus_first(FocusManagerOptions {
                                tabbable: true,
                                ..Default::default()
                            });
                        }
                    });
                },
            );
        });
    }

    // Tab restoration for non-containing restore_focus scopes.
    // When Tab is pressed inside such a scope, redirect focus to the element
    // after node_to_restore rather than the next element in the scope. This
    // enables clean Tab-out from overlays (matching react-aria's useRestoreFocus).
    if restore_focus && !contain {
        let _tab_cleanup = use_event_listener(
            use_document(),
            leptos::ev::keydown,
            move |e: web_sys::KeyboardEvent| {
                if e.key() != "Tab"
                    || e.alt_key()
                    || e.ctrl_key()
                    || e.meta_key()
                    || e.is_composing()
                {
                    return;
                }

                // Only handle if focus is within this scope's subtree.
                let active = use_document()
                    .as_ref()
                    .and_then(web_sys::Document::active_element);
                let focus_in_scope = active.as_ref().is_some_and(|a| {
                    focus_scope_tree::is_element_in_scope_or_descendant(a, scope_id)
                });
                if !focus_in_scope {
                    return;
                }

                let Some(node_to_restore) = focus_scope_tree::get_node_to_restore(scope_id) else {
                    return;
                };

                // Get the scope element for checking containment.
                let Some(scope_el) = scope_ref.get_untracked() else {
                    return;
                };
                let scope_element: web_sys::Element = scope_el.into();

                // Create a TreeWalker on body to find the next tabbable element
                // from the node_to_restore position.
                let Some(body) = use_document().as_ref().and_then(web_sys::Document::body) else {
                    return;
                };
                let body_el: &web_sys::Element = body.unchecked_ref();
                let Ok(walker) = body
                    .owner_document()
                    .unwrap()
                    .create_tree_walker_with_what_to_show(body_el.as_ref(), 0x1)
                else {
                    return;
                };

                // Position walker at node_to_restore and walk to find next/previous
                // tabbable element outside the scope.
                walker.set_current_node(&node_to_restore);

                let next_el = if e.shift_key() {
                    walk_to_tabbable_outside_scope(&walker, &scope_element, false)
                } else {
                    walk_to_tabbable_outside_scope(&walker, &scope_element, true)
                };

                e.prevent_default();
                e.stop_propagation();

                if let Some(el) = next_el {
                    crate::utils::focus::focus_element(&el, false);
                } else if let Some(html_el) = node_to_restore.dyn_ref::<web_sys::HtmlElement>() {
                    let _ = html_el.focus();
                }
            },
        );
    }

    // Unregister from scope tree and restore focus on unmount.
    on_cleanup(move || {
        if restore_focus {
            // Determine whether to restore focus.
            //
            // Bug 1 fix: Only restore if the active element is within this scope's
            // subtree (including nested child scopes). If the user moved focus
            // outside the scope before it unmounted, leave focus where it is.
            //
            // Bug 3 fix: Also restore if focus is on document.body AND this scope
            // is the appropriate restorer (no intermediate nested scope will
            // handle it). This covers the case where a child scope unmounted
            // first, leaving focus on body.
            let active_element = use_document()
                .as_ref()
                .and_then(web_sys::Document::active_element);

            let active_in_scope = active_element.as_ref().is_some_and(|active| {
                focus_scope_tree::is_element_in_scope_or_descendant(active, scope_id)
            });

            let active_is_body = active_element
                .as_ref()
                .is_some_and(|active| active.dyn_ref::<web_sys::HtmlBodyElement>().is_some());

            let should_restore = active_in_scope
                || (active_is_body && focus_scope_tree::should_restore_focus(scope_id));

            // Collect the candidates now (the scope tree forgets this scope below); restore
            // after the next frame, as react-aria does: by then, effects that ran because of the
            // unmount (e.g. a popover making the page interactive again) are done. Only restore
            // if focus fell to the body; otherwise it was moved on purpose.
            let candidates = if should_restore {
                focus_scope_tree::nodes_to_restore(scope_id)
            } else {
                Vec::new()
            };

            focus_scope_tree::unregister_scope(scope_id);

            if !candidates.is_empty() {
                request_animation_frame(move || {
                    let focus_on_body = use_document()
                        .as_ref()
                        .and_then(web_sys::Document::active_element)
                        .is_none_or(|active| active.dyn_ref::<web_sys::HtmlBodyElement>().is_some());
                    if !focus_on_body {
                        return;
                    }
                    if let Some(element) = candidates.iter().find(|el| el.is_connected())
                        && let Some(html_el) = element.dyn_ref::<web_sys::HtmlElement>()
                        // Listeners can call preventDefault() to cancel restoration.
                        && dispatch_restore_focus_event(html_el)
                    {
                        let _ = html_el.focus();
                    }
                });
            }
        } else {
            focus_scope_tree::unregister_scope(scope_id);
        }
    });

            // For the children only: our scope ID, so that nested scopes discover us as parent,
            // and the focus manager.
            scoped_view(
                move || {
                    provide_context(FocusScopeParentContext { scope_id });
                    provide_context(FocusScopeContext { focus_manager });
                },
                move || {
                    view! {
                        <div node_ref=scope_ref class=classes style=styles.add_unchecked("display", "contents")>
                            {children()}
                        </div>
                    }
                },
            )
        }
    }
}

/// Makes `scope_id` the active scope if focus is already within it.
#[cfg(not(feature = "ssr"))]
fn mark_active_if_focus_within(scope_id: focus_scope_tree::ScopeId) {
    if let Some(active) = use_document()
        .as_ref()
        .and_then(web_sys::Document::active_element)
        && focus_scope_tree::is_element_in_scope_or_descendant(&active, scope_id)
    {
        focus_scope_tree::set_active_scope_if_deepest(scope_id, &active);
    }
}

/// Dispatch the [`RESTORE_FOCUS_EVENT`] on the target element.
/// Returns `true` if the event was NOT cancelled (restoration should proceed).
fn dispatch_restore_focus_event(target: &web_sys::HtmlElement) -> bool {
    let init = web_sys::CustomEventInit::new();
    init.set_bubbles(true);
    init.set_cancelable(true);
    let Ok(event) = web_sys::CustomEvent::new_with_event_init_dict(RESTORE_FOCUS_EVENT, &init)
    else {
        return true;
    };
    // dispatch_event returns true if no handler called preventDefault.
    target.dispatch_event(&event).unwrap_or(true)
}

/// Walk a `TreeWalker` forward or backward to find the next tabbable element
/// that is NOT inside `scope_element`.
fn walk_to_tabbable_outside_scope(
    walker: &web_sys::TreeWalker,
    scope_element: &web_sys::Element,
    forward: bool,
) -> Option<web_sys::Element> {
    let scope_node: &web_sys::Node = scope_element.unchecked_ref();
    loop {
        let node = if forward {
            walker.next_node()
        } else {
            walker.previous_node()
        };
        let Ok(Some(node)) = node else {
            return None;
        };
        let Some(el) = node.dyn_ref::<web_sys::Element>() else {
            continue;
        };
        // Skip elements inside the scope.
        if scope_node.contains(Some(&node)) {
            continue;
        }
        if crate::utils::focusability::is_tabbable(el) {
            return Some(el.clone());
        }
    }
}
