// Upstream: react-aria/src/dnd/useClipboard.ts @ 99e6102368
// Upstream: react-aria/test/dnd/useClipboard.test.js @ 99e6102368
use std::{cell::RefCell, rc::Rc};

use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::ClipboardEvent;

use crate::{
    hooks::{
        dnd::{
            DragItem, DropItem,
            utils::{read_from_data_transfer, write_to_data_transfer},
        },
        focus::use_focus::{UseFocusInput, UseFocusProps, use_focus},
    },
    utils::event_listeners::{Listener, listen},
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// No intentional deviations from the react-aria implementation.
//
// =============================================================================

/// Whether data is cut or copied.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClipboardAction {
    Cut,
    Copy,
}

/// Input of [`use_clipboard`].
#[derive(Clone, Default)]
pub struct UseClipboardInput {
    /// The data to cut or copy. Without it, cut and copy keep their default.
    pub get_items: Option<Callback<ClipboardAction, Vec<DragItem>>>,
    pub on_copy: Option<Callback<()>>,
    /// Remove the cut data. Without it, cut keeps its default.
    pub on_cut: Option<Callback<()>>,
    /// Pasted data. Without it, paste keeps its default.
    pub on_paste: Option<Callback<Vec<DropItem>>>,
    pub is_disabled: Signal<bool>,
}

/// Return value of [`use_clipboard`].
#[derive(Debug)]
pub struct UseClipboardReturn {
    /// Tracks whether the element has focus (clipboard events only act on it while it has).
    pub clipboard_props: UseFocusProps,
}

type Handler = Rc<dyn Fn(&ClipboardEvent)>;

/// One document listener per clipboard event, shared by all elements using the clipboard.
struct GlobalEvent {
    _listener: Listener,
    handlers: Rc<RefCell<Vec<(u64, Handler)>>>,
}

thread_local! {
    static GLOBAL_EVENTS: RefCell<Vec<(&'static str, GlobalEvent)>> = const { RefCell::new(Vec::new()) };
    static NEXT_HANDLER: std::cell::Cell<u64> = const { std::cell::Cell::new(0) };
}

fn add_global_event_listener(event: &'static str, handler: Handler) -> u64 {
    let id = NEXT_HANDLER.with(|n| {
        let id = n.get();
        n.set(id.wrapping_add(1));
        id
    });
    GLOBAL_EVENTS.with(|events| {
        let mut events = events.borrow_mut();
        if !events.iter().any(|(e, _)| *e == event)
            && let Some(document) = leptos_use::use_document().as_ref()
        {
            let handlers: Rc<RefCell<Vec<(u64, Handler)>>> = Rc::default();
            let dispatch = handlers.clone();
            // The handlers run the app's callbacks untracked, as Leptos runs `on:` handlers.
            let listener = listen(document.unchecked_ref(), event, false, move |e| {
                let current: Vec<Handler> =
                    dispatch.borrow().iter().map(|(_, h)| h.clone()).collect();
                untrack(|| {
                    for handler in current {
                        handler(e.unchecked_ref());
                    }
                });
            });
            events.push((
                event,
                GlobalEvent {
                    _listener: listener,
                    handlers,
                },
            ));
        }
        if let Some((_, global)) = events.iter().find(|(e, _)| *e == event) {
            global.handlers.borrow_mut().push((id, handler));
        }
    });
    id
}

fn remove_global_event_listener(event: &'static str, id: u64) {
    GLOBAL_EVENTS.with(|events| {
        let mut events = events.borrow_mut();
        if let Some(position) = events.iter().position(|(e, _)| *e == event) {
            let empty = {
                let mut handlers = events[position].1.handlers.borrow_mut();
                handlers.retain(|(i, _)| *i != id);
                handlers.is_empty()
            };
            if empty {
                events.remove(position);
            }
        }
    });
}

/// Cut, copy and paste for an element (e.g. a collection) while it has focus, using the same
/// data format as drag and drop.
pub fn use_clipboard(input: UseClipboardInput) -> UseClipboardReturn {
    let UseClipboardInput {
        get_items,
        on_copy,
        on_cut,
        on_paste,
        is_disabled,
    } = input;
    let is_focused = StoredValue::new(false);
    let clipboard_props = use_focus(UseFocusInput {
        is_disabled: Signal::stored(false),
        on_focus: None,
        on_blur: None,
        on_focus_change: Some(Callback::new(move |focused| is_focused.set_value(focused))),
    })
    .props;
    let focused = move || is_focused.try_get_value().unwrap_or(false);

    let registered: StoredValue<Vec<(&'static str, u64)>> = StoredValue::new(Vec::new());
    let unregister = move || {
        for (event, id) in registered
            .try_update_value(std::mem::take)
            .unwrap_or_default()
        {
            remove_global_event_listener(event, id);
        }
    };
    Effect::new(move || {
        unregister();
        if is_disabled.get() {
            return;
        }
        let handlers: [(&'static str, Handler); 6] = [
            (
                "beforecopy",
                Rc::new(move |e: &ClipboardEvent| {
                    if focused() && get_items.is_some() {
                        e.prevent_default();
                    }
                }),
            ),
            (
                "copy",
                Rc::new(move |e: &ClipboardEvent| {
                    let Some(get_items) = get_items.filter(|_| focused()) else {
                        return;
                    };
                    e.prevent_default();
                    if let Some(data) = e.clipboard_data() {
                        write_to_data_transfer(&data, &get_items.run(ClipboardAction::Copy));
                        if let Some(on_copy) = on_copy {
                            on_copy.run(());
                        }
                    }
                }),
            ),
            (
                "beforecut",
                Rc::new(move |e: &ClipboardEvent| {
                    if focused() && on_cut.is_some() && get_items.is_some() {
                        e.prevent_default();
                    }
                }),
            ),
            (
                "cut",
                Rc::new(move |e: &ClipboardEvent| {
                    let (Some(get_items), Some(on_cut)) = (get_items, on_cut) else {
                        return;
                    };
                    if !focused() {
                        return;
                    }
                    e.prevent_default();
                    if let Some(data) = e.clipboard_data() {
                        write_to_data_transfer(&data, &get_items.run(ClipboardAction::Cut));
                        on_cut.run(());
                    }
                }),
            ),
            (
                "beforepaste",
                Rc::new(move |e: &ClipboardEvent| {
                    if focused() && on_paste.is_some() {
                        e.prevent_default();
                    }
                }),
            ),
            (
                "paste",
                Rc::new(move |e: &ClipboardEvent| {
                    let Some(on_paste) = on_paste.filter(|_| focused()) else {
                        return;
                    };
                    e.prevent_default();
                    if let Some(data) = e.clipboard_data() {
                        on_paste.run(read_from_data_transfer(&data));
                    }
                }),
            ),
        ];
        let ids = handlers
            .into_iter()
            .map(|(event, handler)| (event, add_global_event_listener(event, handler)))
            .collect();
        registered.set_value(ids);
    });
    on_cleanup(unregister);

    UseClipboardReturn { clipboard_props }
}
