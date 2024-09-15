use leptos::{
    document,
    ev::{pointerdown, pointerup},
    on_cleanup, Callable, Callback, SignalGetUntracked, SignalSet,
};
use leptos_reactive::{create_effect, create_signal, MaybeSignal, SignalGet};
use leptos_use::{
    core::ElementMaybeSignal, use_event_listener_with_options, UseEventListenerOptions,
};
use typed_builder::TypedBuilder;
use web_sys::PointerEvent;

use crate::utils::{attributes::Attributes, event_handlers::EventHandlers, EventTargetExt};

#[derive(Debug, Clone, Copy)]
pub struct InteractOutsideStartEvent {}

#[derive(Debug, Clone, Copy)]
pub struct InteractOutsideEvent {}

#[derive(Debug, Clone, Copy, TypedBuilder)]
pub struct UseInteractOutsideInput {
    /// Whether interact-outside events should be disabled.
    #[builder(setter(into))]
    pub(crate) disabled: MaybeSignal<bool>,

    /// Called whenever an interaction is initiated (pointer is lowered down).
    #[builder(default, setter(into, strip_option))]
    pub(crate) on_interact_outside_start: Option<Callback<InteractOutsideStartEvent>>,

    /// Called whenever an interaction is completed (pointer is lifted up).
    #[builder(setter(into))]
    pub(crate) on_interact_outside: Callback<InteractOutsideEvent>,
}

#[derive(Debug)]
pub struct UseInteractOutsideReturn {
    pub props: UseInteractOutsideProps,
}

#[derive(Debug)]
pub struct UseInteractOutsideProps {
    pub attrs: Attributes,
    pub handlers: EventHandlers,
}

pub fn use_interact_outside<El, T>(
    el: El,
    input: UseInteractOutsideInput,
) -> UseInteractOutsideReturn
where
    El: Into<ElementMaybeSignal<T, web_sys::Element>> + 'static,
    T: Into<web_sys::Element> + Clone + 'static,
{
    let (pointer_down, set_pointer_down) = create_signal(false);

    let el = el.into();
    // We use an effect to reactively access `el` and our other reactive inputs (disabled, ...).
    let e = create_effect(
        move |prev: Option<Option<(Box<dyn Fn()>, Box<dyn Fn()>)>>| {
            // We might have listeners from a previous effect execution. Remove them before continuing.
            if let Some((remove_pointer_down_listener, remove_pointer_up_listener)) = prev.flatten()
            {
                (remove_pointer_down_listener)();
                (remove_pointer_up_listener)();
            }

            // Do not any event listeners when disabled, making this hook do nothing when disabled.
            if input.disabled.get() {
                return None;
            }

            // Reactively access our element.
            if let Some(el) = el.get() {
                let el: web_sys::Element = el.into();
                let el_clone = el.clone();

                // `owner_document` should only return `None` when the element is itself a document.
                // Using `document()` is safe, as we run inside an effect, which is never executed on the server!
                let owner_doc = el.owner_document().unwrap_or_else(|| document());

                let out: (Box<dyn Fn()>, Box<dyn Fn()>) = {
                    let on_document_pointer_down = move |e: PointerEvent| {
                        if !pointer_down.get_untracked() && is_event_valid(&el, e) {
                            if let Some(on_interact_outside_start) = input.on_interact_outside_start
                            {
                                on_interact_outside_start.call(InteractOutsideStartEvent {});
                            }
                            set_pointer_down.set(true);
                        }
                    };
                    let on_document_pointer_up = move |e: PointerEvent| {
                        if pointer_down.get_untracked() && is_event_valid(&el_clone, e) {
                            input.on_interact_outside.call(InteractOutsideEvent {});
                            set_pointer_down.set(false);
                        }
                    };

                    let remove_pointer_down_listener = use_event_listener_with_options(
                        owner_doc.clone(),
                        pointerdown,
                        on_document_pointer_down,
                        // Revert event traversal. Begin at root node, do not bubble-up from originating element.
                        // We want to act first on these events!
                        UseEventListenerOptions::default().capture(true),
                    );
                    let remove_pointer_up_listener = use_event_listener_with_options(
                        owner_doc,
                        pointerup,
                        on_document_pointer_up,
                        // Revert event traversal. Begin at root node, do not bubble-up from originating element.
                        // We want to act first on these events!
                        UseEventListenerOptions::default().capture(true),
                    );

                    let a: Box<dyn Fn()> = Box::new(remove_pointer_down_listener);
                    let b: Box<dyn Fn()> = Box::new(remove_pointer_up_listener);
                    (a, b)
                };

                Some(out)
            } else {
                None
            }
        },
    );

    on_cleanup(move || {
        // TODO: Fix this. Do we leak listeners when not doing this? This implementation would currently lead to panics when an interaction outside triggers a route change.
        // Clean up our last listeners, if any exist.
        /*e.with_value_mut(|prev| {
            if let Some(prev) = prev {
                if let Some((remove_pointer_down_listener, remove_pointer_up_listener)) = prev {
                    (remove_pointer_down_listener)();
                    (remove_pointer_up_listener)();
                }
            }
        });*/
    });

    UseInteractOutsideReturn {
        props: UseInteractOutsideProps {
            attrs: Attributes::new(),
            handlers: EventHandlers::builder().build(),
        },
    }
}

fn is_event_valid(el: &web_sys::Element, e: PointerEvent) -> bool {
    // We only act when the "main" button (typically the left mouse button) is used.
    // We do not want to interfere with right clicks (typically used for context menu actions).
    // This is also expected to be 0 when no mouse triggered this pointer event.
    if e.button() != 0 {
        return false;
    }

    let target = e.target();
    let target_node = target.as_ref().and_then(|t| t.as_node());

    // TODO: Is this really necessary? Why?
    if let Some(target) = &target {
        if !target.get_owner_document().contains(target_node.as_ref()) {
            tracing::debug!("Ignoring potential use_interact_outside event trigger: Event target is no longer part of the document.");
            return false;
        }
    }

    // Only act on interactions with elements outside the element on which this hook is used!
    // If there is no `target`, `contains` is called with `None` and will return false,
    // letting us return true and therefore let the interaction count.
    !el.contains(target_node.as_ref())
}
