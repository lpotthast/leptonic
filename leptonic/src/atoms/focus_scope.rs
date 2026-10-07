// Upstream: react-aria/src/focus/FocusScope.tsx @ 99e6102368
use leptos::{html, prelude::*};
#[cfg(not(feature = "ssr"))]
use leptos_use::{
    UseEventListenerOptions, use_document, use_event_listener, use_event_listener_with_options,
};
#[cfg(not(feature = "ssr"))]
use wasm_bindgen::JsCast;

use crate::{
    hooks::FocusManager,
    utils::{classes::Classes, scoped_context::scoped_view, styles::Styles},
};
#[cfg(not(feature = "ssr"))]
use crate::{
    hooks::FocusManagerOptions,
    utils::{
        EventAccessors, focus_scope_tree,
        focusable_tree_walker::{FocusableTreeWalkerOptions, get_focusable_tree_walker},
        key::{KeyboardEventKey, KeyboardKey},
        shadow_dom::{get_active_element, node_contains},
        shadow_tree_walker::ShadowTreeWalker,
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - The restore event is `leptonic-focus-scope-restore` ([`RESTORE_FOCUS_EVENT`]; react-aria:
//   `react-aria-focus-scope-restore`): leptonic's own event, so it can't collide with an
//   embedded react-aria.
// - The scope's element takes the atom's `classes` and `styles`. React-aria renders no element.
//
// ## DIFFERENT BEHAVIOR
// - A wrapper `<div style="display: contents">` instead of the hidden sentinel `<span>`s around
//   the children: the scope needs one element for its `NodeRef`, containment checks and
//   listeners; `display: contents` keeps it out of the layout.
// - Scopes register in the component body (react-aria: a layout effect), which likewise
//   registers all scopes mounting together before any auto-focuses.
// - Restoring skips a node to restore that is the body (focus was already lost when the scope
//   mounted: Leptos may remove the element that opened the scope before it creates the scope,
//   where React captures it during render) and falls back to the first element of the nearest
//   ancestor scope.
//
// ## LEPTOS-SPECIFIC ADAPTATIONS
// - Cleanup order: Leptos cleans up child owners before their parent's `on_cleanup` (React runs a
//   parent's layout cleanups first). A scope inside another one unregisters first, so the outer
//   scope decides about restoring with the inner one already gone from the tree.
// - A Tab another scope handled already (`defaultPrevented`) is ignored: Leptos registers the
//   scopes' document listeners outer scope first (React: inner first), so an outer scope moving
//   focus into an inner one would have the inner scope move it on again.
// - Focus escaping a containing scope is recaptured in a microtask after the `focusin` dispatch
//   (react-aria: synchronously): refocusing inside the listener would dispatch a nested `focusin`
//   into it ("No Nested Dispatch of the Same Event Type").
//
// =============================================================================

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
    /// Whether to contain focus within the scope. May change while mounted (a non-modal popover
    /// starts containing focus once a dialog is inside).
    #[prop(into, optional)]
    contain: Signal<bool>,

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
            let parent_id = use_context::<focus_scope_tree::FocusScopeParentContext>()
                .map(|ctx| ctx.scope_id);
            let scope_id = focus_scope_tree::allocate_id();

    // Capture the currently focused element when the scope mounts (before registration).
    let previously_focused: StoredValue<Option<web_sys::Element>, LocalStorage> =
        StoredValue::new_local(None);

    if restore_focus
        && let Some(document) = use_document().as_ref() {
            previously_focused.set_value(document.active_element());
        }

    // Register in the scope tree right away (the element is read when needed), like react-aria's
    // layout effect: scopes mounting together all register before any of them auto-focuses, so
    // only a scope mounting later (while another one is active) is re-parented onto the active
    // one. Also store node_to_restore in the tree for stacked overlay propagation.
    focus_scope_tree::register_scope(
        scope_id,
        parent_id,
        move || {
            scope_ref
                .try_get_untracked()
                .flatten()
                .map(|el| -> web_sys::Element { el.into() })
        },
        contain.get_untracked(),
        restore_focus,
    );
    if restore_focus {
        focus_scope_tree::set_node_to_restore(scope_id, previously_focused.get_value());
    }
    Effect::new(move |_| {
        if scope_ref.get().is_some() {
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

    // Follow changes of `contain` (the scope tree's flag gates all containment handlers below).
    Effect::new(move |_| focus_scope_tree::set_contain(scope_id, contain.get()));

    // Focus containment via keydown handler and focusin listener, active while the scope contains
    // focus.
    {
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
                if e.typed_key() != KeyboardKey::Tab
                    || e.alt_key()
                    || e.ctrl_key()
                    || e.meta_key()
                    || e.is_composing()
                {
                    return;
                }

                // Another scope moved focus for this Tab already: the scopes' document
                // listeners run in registration order (outer first), and an outer scope's move
                // into an inner one makes the inner scope active before its listener runs.
                if e.default_prevented() {
                    return;
                }

                // Only handle if this is the innermost containing scope.
                // If a nested child scope also has contain=true, it handles Tab.
                if !focus_scope_tree::is_innermost_container(scope_id) {
                    return;
                }
                // With focus outside the scope (e.g. in a top layer), Tab is the browser's.
                let focus_in_scope = scope_ref.get_untracked().is_some_and(|scope| {
                    scope
                        .owner_document()
                        .and_then(|document| get_active_element(&document))
                        .is_some_and(|focused| node_contains(scope.as_ref(), focused.as_ref()))
                });
                if !focus_in_scope {
                    return;
                }

                e.prevent_default();

                let opts = FocusManagerOptions {
                    wrap: true,
                    tabbable: true,
                    ..Default::default()
                };

                let next = if e.shift_key() {
                    fm.focus_previous(opts)
                } else {
                    fm.focus_next(opts)
                };
                // As the browser does when tabbing into a text field.
                if let Some(input) = next.and_then(|next| next.dyn_into::<web_sys::HtmlInputElement>().ok()) {
                    input.select();
                }
            },
        );

        // A restore event from inside this scope doesn't reach parent scopes (react-aria).
        Effect::new(move |_| {
            let Some(scope_el) = scope_ref.get() else {
                return;
            };
            let _restore_cleanup = use_event_listener(
                scope_el,
                leptos::ev::Custom::<web_sys::CustomEvent>::new(RESTORE_FOCUS_EVENT),
                |e: web_sys::CustomEvent| e.stop_propagation(),
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

                let target = e.expect_target();

                let focus_is_within = target.dyn_ref::<web_sys::Element>().is_some_and(|el| {
                    focus_scope_tree::is_element_in_scope_or_descendant(el, scope_id)
                });

                if focus_is_within {
                    // Focus moved within scope — update tracked node.
                    if let Some(html_el) = target.dyn_ref::<web_sys::HtmlElement>() {
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
                        // As upstream: a node removed in the meantime can't take focus back.
                        let recaptured = last_focused.is_some_and(|node| {
                            node.is_connected() && {
                                crate::utils::focus::focus_safely(&node);
                                true
                            }
                        });
                        if !recaptured {
                            focus_first_safely(&fm);
                        }
                    });
                }
            },
        );

        let pending_blur = StoredValue::new(None::<AnimationFrameRequestHandle>);
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
                move |e: web_sys::FocusEvent| {
                    if !focus_scope_tree::should_contain_focus(scope_id) {
                        return;
                    }

                    // Defer check to after focus settles via requestAnimationFrame. Only the
                    // latest blur counts (react-aria cancels the pending frame): an earlier one
                    // would bring focus back to an element that lost it before.
                    let fm = fm_focusout.clone();
                    let blurred = e.expect_target().dyn_into::<web_sys::Element>().ok();
                    if let Some(pending) = pending_blur.get_value() {
                        pending.cancel();
                    }
                    let handle = request_animation_frame_with_handle(move || {
                        let _ = pending_blur.try_set_value(None);
                        // The scope may have unmounted (or stopped containing focus) since.
                        if !focus_scope_tree::should_contain_focus(scope_id) {
                            return;
                        }
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

                        // As react-aria: focus goes back to the element that lost it (else the
                        // first one in the scope), without scrolling.
                        if !in_scope {
                            focus_scope_tree::set_active_scope(scope_id);
                            match blurred.filter(|blurred| blurred.is_connected()) {
                                Some(blurred) => {
                                    let _ = focused_node.try_set_value(
                                        blurred.dyn_ref::<web_sys::HtmlElement>().cloned(),
                                    );
                                    crate::utils::focus::focus_safely(&blurred);
                                }
                                None => focus_first_safely(&fm),
                            }
                        }
                    })
                    .ok();
                    pending_blur.set_value(handle);
                },
            );
        });
    }

    // Tabbing out of a non-containing scope that restores focus continues after the node to
    // restore (react-aria's `useRestoreFocus`): e.g. Tab in a popover moves on from its trigger.
    if restore_focus {
        let _tab_cleanup = use_event_listener_with_options(
            use_document(),
            leptos::ev::keydown,
            move |e: web_sys::KeyboardEvent| {
                if contain.get_untracked()
                    || e.typed_key() != KeyboardKey::Tab
                    || e.alt_key()
                    || e.ctrl_key()
                    || e.meta_key()
                    || e.is_composing()
                {
                    return;
                }
                let Some(document) = use_document().as_ref().cloned() else {
                    return;
                };
                let Some(focused) = document.active_element() else {
                    return;
                };
                let in_scope = |el: &web_sys::Element| {
                    focus_scope_tree::is_element_in_scope_or_descendant(el, scope_id)
                };
                if !in_scope(&focused) || !focus_scope_tree::should_restore_focus(scope_id) {
                    return;
                }

                // A node to restore that is gone (or the body) is forgotten.
                let node_to_restore = focus_scope_tree::get_node_to_restore(scope_id).filter(|node| {
                    node.is_connected() && node.dyn_ref::<web_sys::HtmlBodyElement>().is_none()
                });
                if node_to_restore.is_none() {
                    focus_scope_tree::set_node_to_restore(scope_id, None);
                }

                let Some(body) = document.body() else {
                    return;
                };
                let Some(mut walker) = get_focusable_tree_walker(
                    &body,
                    FocusableTreeWalkerOptions {
                        tabbable: true,
                        ..Default::default()
                    },
                ) else {
                    return;
                };
                let step = |walker: &mut ShadowTreeWalker| {
                    if e.shift_key() {
                        walker.previous_node()
                    } else {
                        walker.next_node()
                    }
                    .and_then(|node| node.dyn_into::<web_sys::Element>().ok())
                };
                walker.set_current_node(&focused);
                let next = step(&mut walker);

                // Only when the next element is outside the scope (or there is none).
                let Some(node_to_restore) = node_to_restore else {
                    return;
                };
                if next.as_ref().is_some_and(in_scope) {
                    return;
                }
                // Skip the scope's own elements, in case it immediately follows the node to restore.
                walker.set_current_node(&node_to_restore);
                let next = loop {
                    match step(&mut walker) {
                        Some(el) if in_scope(&el) => {}
                        other => break other,
                    }
                };

                e.prevent_default();
                e.stop_propagation();
                if let Some(next) = next {
                    crate::utils::focus::focus_element(&next, false);
                } else if !focus_scope_tree::is_element_in_any_scope(&node_to_restore) {
                    // Leaving the top-level scope: focus goes to the body.
                    if let Some(focused) = focused.dyn_ref::<web_sys::HtmlElement>() {
                        let _ = focused.blur();
                    }
                } else {
                    // E.g. a menu in a popover: Tab closes the menu, back to its trigger.
                    crate::utils::focus::focus_element(&node_to_restore, false);
                }
            },
            UseEventListenerOptions::default().capture(true),
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
            // Without a node to restore in the DOM, focus goes to the first tabbable element of
            // the nearest ancestor scope still mounted then.
            let should_restore =
                should_restore && focus_scope_tree::get_node_to_restore(scope_id).is_some();
            let (candidates, ancestors) = if should_restore {
                (
                    focus_scope_tree::nodes_to_restore(scope_id),
                    focus_scope_tree::ancestors(scope_id),
                )
            } else {
                (Vec::new(), Vec::new())
            };

            focus_scope_tree::unregister_scope(scope_id);

            if should_restore {
                request_animation_frame(move || {
                    let focus_on_body = use_document()
                        .as_ref()
                        .and_then(web_sys::Document::active_element)
                        .is_none_or(|active| active.dyn_ref::<web_sys::HtmlBodyElement>().is_some());
                    if !focus_on_body {
                        return;
                    }
                    // The body (focus was already lost when the scope mounted, e.g. the item that
                    // opened it was removed first) can't take focus: the fallback applies.
                    let target = candidates
                        .into_iter()
                        .find(|element| {
                            element.is_connected()
                                && element.dyn_ref::<web_sys::HtmlBodyElement>().is_none()
                        })
                        .or_else(|| {
                            // As react-aria's `getFirstInScope`: tabbable, else focusable.
                            ancestors.into_iter().find_map(|id| {
                                let scope = focus_scope_tree::scope_element(id)?;
                                let manager = FocusManager::new(move || Some(scope.clone()));
                                [true, false].into_iter().find_map(|tabbable| {
                                    manager.find_first(FocusManagerOptions {
                                        tabbable,
                                        ..Default::default()
                                    })
                                })
                            })
                        });
                    // Listeners can call preventDefault() to cancel restoration.
                    if let Some(element) = target
                        && dispatch_restore_focus_event(&element)
                    {
                        crate::utils::focus::focus_safely(&element);
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
                    provide_context(focus_scope_tree::FocusScopeParentContext { scope_id });
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

/// Focuses the first tabbable element of a scope without scrolling (react-aria's
/// `focusFirstInScope`; the focus manager's own `focus_first` scrolls, as upstream's).
#[cfg(not(feature = "ssr"))]
fn focus_first_safely(focus_manager: &FocusManager) {
    if let Some(element) = focus_manager.find_first(FocusManagerOptions {
        tabbable: true,
        ..Default::default()
    }) {
        crate::utils::focus::focus_safely(&element);
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
#[cfg(not(feature = "ssr"))]
fn dispatch_restore_focus_event(target: &web_sys::Element) -> bool {
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
