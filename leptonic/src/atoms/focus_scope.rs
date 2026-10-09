// Upstream: react-aria/src/focus/FocusScope.tsx @ 99e6102368
// Upstream: react-aria/test/focus/FocusScope.test.js @ 99e6102368
// Upstream: react-aria/test/focus/FocusScopeOwnerDocument.test.js @ 99e6102368
use std::borrow::Cow;

use leptos::{ev::EventDescriptor, html, prelude::*};
use leptos_classes::Classes;

use crate::{
    hooks::focus::FocusManager,
    utils::{scoped_context::scoped_view, styles::Styles},
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - The restore event is [`RestoreFocusEvent`] (`leptonic-focus-scope-restore`; react-aria:
//   `react-aria-focus-scope-restore`): leptonic's own event, so it can't collide with an
//   embedded react-aria; typed, for `on(RestoreFocusEvent, ..)` and leptos-use listeners.
// - The scope provides its `FocusManager` as a context (`use_focus_manager_context`, react-aria's
//   `useFocusManager`).
// - The scope's element takes the atom's `classes` and `styles`. React-aria renders no element.
//
// ## DIFFERENT BEHAVIOR
// - A wrapper `<div style="display: contents">` instead of the hidden sentinel `<span>`s around
//   the children: the scope needs one element for its `NodeRef`, containment checks and
//   listeners; `display: contents` keeps it out of the layout.
// - Scopes register in the component body (react-aria: a layout effect), which likewise
//   registers all scopes mounting together before any auto-focuses.
// - The active scope is tracked by one listener for all scopes (`focus_scope_tree::track_focus`,
//   see its deviations).
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

/// The name of the [`RestoreFocusEvent`].
const RESTORE_FOCUS_EVENT: &str = "leptonic-focus-scope-restore";

/// The event a `FocusScope` dispatches on the element it is about to restore focus to (react-aria's
/// `RESTORE_FOCUS_EVENT`): a bubbling, cancelable `CustomEvent`. A listener calling
/// `prevent_default()` cancels the restoration (e.g. a virtual collection that reuses its DOM
/// elements restores focus itself). It doesn't leave the `FocusScope`s it is dispatched in.
///
/// ```ignore
/// view! { <div {..on(RestoreFocusEvent, |e: web_sys::CustomEvent| e.prevent_default())}>..</div> }
/// ```
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RestoreFocusEvent;

impl EventDescriptor for RestoreFocusEvent {
    type EventType = web_sys::CustomEvent;
    const BUBBLES: bool = true;

    fn name(&self) -> Cow<'static, str> {
        Cow::Borrowed(RESTORE_FOCUS_EVENT)
    }

    fn event_delegation_key(&self) -> Cow<'static, str> {
        Cow::Borrowed("$$$leptonic-focus-scope-restore")
    }
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
/// - A [`FocusManager`] for its children ([`use_focus_manager_context`]).
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
///
/// [`use_focus_manager_context`]: crate::hooks::focus::use_focus_manager_context
#[component]
pub fn FocusScope(
    /// Whether to contain focus within the scope. May change while mounted (a non-modal popover
    /// starts containing focus once a dialog is inside).
    #[prop(into, optional)]
    contain: Signal<bool>,

    /// Whether to restore focus, when the scope unmounts, to the element that had focus when it
    /// mounted. May change while mounted.
    #[prop(into, optional)]
    restore_focus: Signal<bool>,

    /// Whether to focus the first focusable element when the scope mounts.
    #[prop(default = false)]
    auto_focus: bool,

    #[prop(into, optional)] classes: Classes,

    #[prop(into, optional)] styles: Styles,

    /// The content of the focus scope.
    children: Children,
) -> impl IntoView {
    crate::hooks::focus::use_focus_visible::track_interaction_modality();
    let scope_ref = NodeRef::<html::Div>::new();
    // The manager can outlive the scope (e.g. in pending focus callbacks): a disposed scope has
    // no element.
    let focus_manager = FocusManager::new(move || {
        scope_ref
            .try_get_untracked()
            .flatten()
            .map(|el| -> web_sys::Element { el.into() })
    });

    #[cfg(feature = "ssr")]
    let _ = (contain, restore_focus, auto_focus);
    #[cfg(not(feature = "ssr"))]
    let scope_id = client::set_up(scope_ref, contain, restore_focus, auto_focus);

    // For the children only: our scope ID, so that nested scopes discover us as parent, and the
    // focus manager.
    scoped_view(
        move || {
            #[cfg(not(feature = "ssr"))]
            provide_context(crate::utils::focus_scope_tree::FocusScopeParentContext { scope_id });
            provide_context(focus_manager);
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

#[cfg(not(feature = "ssr"))]
mod client {
    use leptos::{html, prelude::*};
    use leptos_use::{
        UseEventListenerOptions, use_document, use_event_listener, use_event_listener_with_options,
    };
    use wasm_bindgen::JsCast;

    use super::RestoreFocusEvent;
    use crate::utils::{
        focus::{focus_element, focus_safely},
        focus_scope_tree::{self, ScopeId},
        focusability::Focusability,
        focusable_tree_walker::{FocusableTreeWalkerOptions, get_focusable_tree_walker},
        key::{KeyboardEventKey, KeyboardKey},
        shadow_dom::{get_active_element, get_event_target, node_contains},
        shadow_tree_walker::ShadowTreeWalker,
    };

    /// The scope's element.
    fn element(scope_ref: NodeRef<html::Div>) -> Option<web_sys::Element> {
        scope_ref
            .try_get_untracked()
            .flatten()
            .map(|el| -> web_sys::Element { el.into() })
    }

    /// The focused element of the scope's document (across shadow roots).
    fn active_element() -> Option<web_sys::Element> {
        use_document().as_ref().and_then(get_active_element)
    }

    /// Whether a Tab key event is one the scope handles (no modifier, not composing).
    fn is_plain_tab(e: &web_sys::KeyboardEvent) -> bool {
        e.typed_key() == KeyboardKey::Tab
            && !e.alt_key()
            && !e.ctrl_key()
            && !e.meta_key()
            && !e.is_composing()
    }

    /// Registers the scope and sets up containment, restoration and auto focus.
    #[allow(clippy::too_many_lines)]
    pub(super) fn set_up(
        scope_ref: NodeRef<html::Div>,
        contain: Signal<bool>,
        restore_focus: Signal<bool>,
        auto_focus: bool,
    ) -> ScopeId {
        // Discover the parent scope (if any) via Leptos context.
        let parent_id =
            use_context::<focus_scope_tree::FocusScopeParentContext>().map(|ctx| ctx.scope_id);
        let scope_id = focus_scope_tree::allocate_id();

        // The focused element when the scope mounts, before children auto-focus (react-aria's
        // `nodeToRestoreRef`, created during render).
        let previously_focused: StoredValue<Option<web_sys::Element>, LocalStorage> =
            StoredValue::new_local(active_element());

        // Register in the scope tree right away (the element is read when needed), like
        // react-aria's layout effect: scopes mounting together all register before any of them
        // auto-focuses, so only a scope mounting later (while another one is active) is
        // re-parented onto the active one.
        focus_scope_tree::register_scope(
            scope_id,
            parent_id,
            move || element(scope_ref),
            contain.get_untracked(),
            restore_focus.get_untracked(),
        );
        // Follow `contain` and `restore_focus` (the tree's flags gate the handlers below).
        Effect::new(move |_| focus_scope_tree::set_contain(scope_id, contain.get()));
        Effect::new(move |_| {
            let restore = restore_focus.get();
            focus_scope_tree::set_restore(scope_id, restore);
            focus_scope_tree::set_node_to_restore(
                scope_id,
                restore.then(|| previously_focused.get_value()).flatten(),
            );
        });
        Effect::new(move |_| {
            if scope_ref.get().is_some() {
                // Focus may have moved in before the scope was registered (e.g. a dialog
                // focusing itself on mount); react-aria tracks the active scope from a layout
                // effect, before such effects run. Later focus moves are tracked by a
                // document-level listener.
                focus_scope_tree::ensure_active_scope_tracking();
                if let Some(active) = active_element()
                    && focus_scope_tree::is_element_in_scope_or_descendant(&active, scope_id)
                {
                    focus_scope_tree::set_active_scope_if_deepest(scope_id, &active);
                }
            }
        });

        // Auto focus on mount: the scope becomes active, and focus moves to its first element
        // unless it is inside already (react-aria's `useAutoFocus`). `focus_safely`
        // (preventScroll), so no scroll-based overlay dismissal runs.
        if auto_focus {
            Effect::new(move |prev_run: Option<()>| {
                if prev_run.is_some() {
                    return;
                }
                let Some(scope) = scope_ref.get().map(web_sys::Element::from) else {
                    return;
                };
                focus_scope_tree::set_active_scope(scope_id);
                let focus_is_inside = active_element().is_some_and(|a| node_contains(&scope, &a));
                if !focus_is_inside && let Some(first) = first_in_scope(&scope) {
                    focus_safely(&first);
                }
            });
        }

        // The last focused element within the scope, to return focus to when recapturing.
        let focused_node: StoredValue<Option<web_sys::HtmlElement>, LocalStorage> =
            StoredValue::new_local(None);

        // Tab containment: a document listener, so it sees the Tab even when the browser would
        // move focus out of the scope.
        let _ = use_event_listener(
            use_document(),
            leptos::ev::keydown,
            move |e: web_sys::KeyboardEvent| {
                if !is_plain_tab(&e)
                    // Another scope moved focus for this Tab already: the scopes' document
                    // listeners run in registration order (outer first), and an outer scope's
                    // move into an inner one makes the inner scope active before its listener
                    // runs.
                    || e.default_prevented()
                    || !focus_scope_tree::is_containing(scope_id)
                {
                    return;
                }
                let Some(scope) = element(scope_ref) else {
                    return;
                };
                // With focus outside the scope (e.g. in a top layer), Tab is the browser's.
                let Some(focused) = active_element().filter(|a| node_contains(&scope, a)) else {
                    return;
                };
                let Some(mut walker) = get_focusable_tree_walker(
                    &scope,
                    FocusableTreeWalkerOptions {
                        focusability: Focusability::Tabbable,
                        ..Default::default()
                    },
                ) else {
                    return;
                };
                walker.set_current_node(&focused);
                let step = |walker: &mut ShadowTreeWalker| {
                    if e.shift_key() {
                        walker.previous_node()
                    } else {
                        walker.next_node()
                    }
                    .and_then(|node| node.dyn_into::<web_sys::Element>().ok())
                };
                // Wraps around: from the scope element forwards, from its last node backwards.
                let next = step(&mut walker).or_else(|| {
                    if e.shift_key() {
                        last_in(&mut walker, &scope)
                    } else {
                        walker.set_current_node(&scope);
                        step(&mut walker)
                    }
                });

                e.prevent_default();
                if let Some(next) = next {
                    focus_element(&next, false);
                    // As the browser does when tabbing into a text field.
                    if let Some(input) = next.dyn_ref::<web_sys::HtmlInputElement>() {
                        input.select();
                    }
                }
            },
        );

        // A restore event from inside this scope doesn't reach parent scopes (react-aria).
        Effect::new(move |_| {
            let Some(scope_el) = scope_ref.get() else {
                return;
            };
            let _ = use_event_listener(scope_el, RestoreFocusEvent, |e: web_sys::CustomEvent| {
                e.stop_propagation();
            });
        });

        // Focus escaping a containing scope is recaptured.
        let _ = use_event_listener(
            use_document(),
            leptos::ev::focusin,
            move |e: web_sys::FocusEvent| {
                if !focus_scope_tree::is_containing(scope_id) {
                    return;
                }
                let Some(target) = get_event_target(&e)
                    .and_then(|target| target.dyn_into::<web_sys::Element>().ok())
                else {
                    return;
                };
                if focus_scope_tree::is_element_in_scope_or_descendant(&target, scope_id) {
                    // Focus moved within the scope: remember it.
                    focused_node.set_value(target.dyn_into::<web_sys::HtmlElement>().ok());
                    return;
                }
                // Focus escaped: back to the last focused node, else to the first element of the
                // active scope. After this `focusin` dispatch: refocusing synchronously would
                // dispatch a nested `focusin` into this listener's closure, which is still
                // running (and can't be re-entered).
                queue_microtask(move || {
                    // The scope may have been unmounted (or stopped containing focus) since.
                    if !focus_scope_tree::is_containing(scope_id) {
                        return;
                    }
                    let Some(last_focused) = focused_node.try_get_value() else {
                        return;
                    };
                    // As upstream: a node removed in the meantime can't take focus back.
                    match last_focused.filter(|node| node.is_connected()) {
                        Some(node) => focus_safely(&node),
                        None => {
                            if let Some(first) = focus_scope_tree::active_scope_element()
                                .and_then(|active| first_in_scope(&active))
                            {
                                focus_safely(&first);
                            }
                        }
                    }
                });
            },
        );

        // Focus leaving the scope (a click outside, a script blurring the focused element) is
        // recaptured once focus settled, in the next frame. Only the latest blur counts
        // (react-aria cancels the pending frame): an earlier one would bring focus back to an
        // element that lost it before.
        let pending_blur = StoredValue::new_local(None::<AnimationFrameRequestHandle>);
        let cancel_pending_blur = move || {
            if let Some(pending) = pending_blur.try_update_value(Option::take).flatten() {
                pending.cancel();
            }
        };
        // Not containing anymore: a pending recapture is void.
        Effect::new(move |_| {
            if !contain.get() {
                cancel_pending_blur();
            }
        });
        on_cleanup(cancel_pending_blur);
        Effect::new(move |_| {
            let Some(scope_el) = scope_ref.get() else {
                return;
            };
            let _ = use_event_listener(
                scope_el,
                leptos::ev::focusout,
                move |e: web_sys::FocusEvent| {
                    if !focus_scope_tree::is_containing(scope_id) {
                        return;
                    }
                    let blurred = get_event_target(&e)
                        .and_then(|target| target.dyn_into::<web_sys::Element>().ok());
                    cancel_pending_blur();
                    let handle = request_animation_frame_with_handle(move || {
                        let _ = pending_blur.try_set_value(None);
                        // Android TalkBack: skip the recapture in virtual (or no) modality on
                        // Android Chrome, which would trap the virtual cursor (Chrome bug
                        // #384844019). As react-aria.
                        if matches!(
                            crate::hooks::focus::get_modality(),
                            None | Some(crate::hooks::focus::Modality::Virtual)
                        ) && crate::utils::platform::device::is_android()
                            && crate::utils::platform::browser::is_chrome()
                        {
                            return;
                        }
                        // Use the active element (not the related target) to see clicks into
                        // iframes.
                        let Some(active) = active_element() else {
                            return;
                        };
                        if !focus_scope_tree::is_containing(scope_id)
                            || focus_scope_tree::is_element_in_scope_or_descendant(
                                &active, scope_id,
                            )
                        {
                            return;
                        }
                        // As react-aria: focus goes back to the element that lost it (else the
                        // first one in the scope), without scrolling.
                        focus_scope_tree::set_active_scope(scope_id);
                        match blurred.filter(|blurred| blurred.is_connected()) {
                            Some(blurred) => {
                                let _ = focused_node.try_set_value(
                                    blurred.dyn_ref::<web_sys::HtmlElement>().cloned(),
                                );
                                focus_safely(&blurred);
                            }
                            None => {
                                if let Some(first) =
                                    element(scope_ref).and_then(|scope| first_in_scope(&scope))
                                {
                                    focus_safely(&first);
                                }
                            }
                        }
                    })
                    .ok();
                    let _ = pending_blur.try_set_value(handle);
                },
            );
        });

        // Tabbing out of a non-containing scope that restores focus continues after the node to
        // restore (react-aria's `useRestoreFocus`): e.g. Tab in a popover moves on from its
        // trigger.
        // Only while restoring without containing (react-aria registers it only then).
        Effect::new(move |_| {
            if !restore_focus.get() || contain.get() {
                return;
            }
            let _ = use_event_listener_with_options(
                use_document(),
                leptos::ev::keydown,
                move |e: web_sys::KeyboardEvent| {
                    if !is_plain_tab(&e)
                    // A containing scope between the active one and this one handles Tab.
                    || !focus_scope_tree::should_contain_focus(scope_id)
                    {
                        return;
                    }
                    let Some(document) = use_document().as_ref().cloned() else {
                        return;
                    };
                    let Some(focused) = get_active_element(&document) else {
                        return;
                    };
                    let in_scope = |el: &web_sys::Element| {
                        focus_scope_tree::is_element_in_scope_or_descendant(el, scope_id)
                    };
                    if !in_scope(&focused) || !focus_scope_tree::should_restore_focus(scope_id) {
                        return;
                    }

                    // A node to restore that is gone (or the body) is forgotten.
                    let node_to_restore =
                        focus_scope_tree::get_node_to_restore(scope_id).filter(|node| {
                            node.is_connected()
                                && node.dyn_ref::<web_sys::HtmlBodyElement>().is_none()
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
                            focusability: Focusability::Tabbable,
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
                    // Skip the scope's own elements, in case it immediately follows the node to
                    // restore.
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
                        focus_element(&next, false);
                    } else if !focus_scope_tree::is_element_in_any_scope(&node_to_restore) {
                        // Leaving the top-level scope: focus goes to the body.
                        if let Some(focused) = focused.dyn_ref::<web_sys::HtmlElement>() {
                            let _ = focused.blur();
                        }
                    } else {
                        // E.g. a menu in a popover: Tab closes the menu, back to its trigger.
                        focus_element(&node_to_restore, false);
                    }
                },
                UseEventListenerOptions::default().capture(true),
            );
        });

        // Unregister from the scope tree and restore focus on unmount.
        on_cleanup(move || restore_on_unmount(scope_id));

        scope_id
    }

    /// Unregisters the scope; if it restores focus and focus is inside it (or already fell to the
    /// body while it was the scope to restore), restores focus in the next frame (react-aria's
    /// `useRestoreFocus` cleanup).
    fn restore_on_unmount(scope_id: ScopeId) {
        let active = active_element();
        let should_restore = focus_scope_tree::restores(scope_id)
            && focus_scope_tree::get_node_to_restore(scope_id).is_some()
            && active.as_ref().is_some_and(|active| {
                focus_scope_tree::is_element_in_scope_or_descendant(active, scope_id)
                    || (active.dyn_ref::<web_sys::HtmlBodyElement>().is_some()
                        && focus_scope_tree::should_restore_focus(scope_id))
            });
        // Collect the candidates now (the scope tree forgets this scope below); restore after the
        // next frame, as react-aria does: by then, effects that ran because of the unmount (e.g. a
        // popover making the page interactive again) are done. Without a node to restore in the
        // DOM, focus goes to the first element of the nearest ancestor scope still mounted then.
        let (candidates, ancestors) = if should_restore {
            (
                focus_scope_tree::nodes_to_restore(scope_id),
                focus_scope_tree::ancestors(scope_id),
            )
        } else {
            (Vec::new(), Vec::new())
        };
        focus_scope_tree::unregister_scope(scope_id);
        if !should_restore {
            return;
        }
        request_animation_frame(move || {
            // Only if focus fell to the body; otherwise it was moved on purpose.
            let focus_on_body = active_element()
                .is_none_or(|active| active.dyn_ref::<web_sys::HtmlBodyElement>().is_some());
            if !focus_on_body {
                return;
            }
            // The body (focus was already lost when the scope mounted, e.g. the item that opened
            // it was removed first) can't take focus: the fallback applies.
            let target = candidates
                .into_iter()
                .find(|element| {
                    element.is_connected()
                        && element.dyn_ref::<web_sys::HtmlBodyElement>().is_none()
                })
                .or_else(|| {
                    ancestors
                        .into_iter()
                        .find_map(|id| first_in_scope(&focus_scope_tree::scope_element(id)?))
                });
            // Listeners can call preventDefault() to cancel restoration.
            if let Some(element) = target
                && dispatch_restore_focus_event(&element)
            {
                focus_safely(&element);
            }
        });
    }

    /// The first tabbable element of a scope, else its first focusable one (react-aria's
    /// `getFirstInScope`).
    pub(super) fn first_in_scope(scope: &web_sys::Element) -> Option<web_sys::Element> {
        [Focusability::Tabbable, Focusability::Focusable]
            .into_iter()
            .find_map(|focusability| {
                get_focusable_tree_walker(
                    scope,
                    FocusableTreeWalkerOptions {
                        focusability,
                        ..Default::default()
                    },
                )?
                .next_node()?
                .dyn_into::<web_sys::Element>()
                .ok()
            })
    }

    /// The last element of the walker below `scope` (react-aria's `last`).
    fn last_in(
        walker: &mut ShadowTreeWalker,
        scope: &web_sys::Element,
    ) -> Option<web_sys::Element> {
        walker.set_current_node(scope);
        let mut last = None;
        while let Some(node) = walker.last_child() {
            last = Some(node);
        }
        last.and_then(|node| node.dyn_into::<web_sys::Element>().ok())
    }

    /// Dispatch the [`RestoreFocusEvent`] on the target element.
    /// Returns `true` if the event was NOT cancelled (restoration should proceed).
    fn dispatch_restore_focus_event(target: &web_sys::Element) -> bool {
        use leptos::ev::EventDescriptor;

        let init = web_sys::CustomEventInit::new();
        init.set_bubbles(true);
        init.set_cancelable(true);
        let Ok(event) =
            web_sys::CustomEvent::new_with_event_init_dict(&RestoreFocusEvent.name(), &init)
        else {
            return true;
        };
        // dispatch_event returns true if no handler called preventDefault.
        target.dispatch_event(&event).unwrap_or(true)
    }
}
