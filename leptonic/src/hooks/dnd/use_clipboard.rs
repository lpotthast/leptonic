//! Clipboard hook for cut/copy/paste operations on collections.
//!
//! Provides element-level event handlers for clipboard events, reusing the
//! same serialization format as drag-and-drop for consistency.
//!
//! Based on react-aria's `useClipboard` from
//! `@react-aria/dnd/src/useClipboard.ts`.

use leptos::{
    attr,
    attr::Attr,
    ev,
    ev::{On, SharedEventCallback},
    prelude::*,
};
use web_sys::ClipboardEvent;

use super::drop_item::{DropItem, read_drop_items_from_data_transfer};
use crate::hooks::{DragItem, IntoAttrs, write_to_data_transfer};
use crate::utils::EventHandler;

//
// ## RUST-NATIVE DESIGN
//
// - Uses element-level event handlers (consistent with other leptonic hooks
//   via the attrs pattern) instead of react-aria's global event listeners.
// - `ClipboardAction` is a simple enum instead of a string union.
//
// ## OMITTED
//
// - `beforecopy`/`beforecut`/`beforepaste` events for enabling browser menu
//   items. These are non-standard and browser support is inconsistent.
//

/// The clipboard action being performed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClipboardAction {
    /// Copying items.
    Copy,
    /// Cutting items (copy + mark for removal).
    Cut,
}

/// Input for the [`use_clipboard`] hook.
#[derive(Clone)]
pub struct UseClipboardInput {
    /// Returns the items to write to the clipboard for the given action.
    /// If `None`, copy/cut are not supported.
    pub get_items: Option<Callback<ClipboardAction, Vec<DragItem>>>,

    /// Called after a successful copy.
    pub on_copy: Option<Callback<()>>,

    /// Called after a successful cut.
    pub on_cut: Option<Callback<()>>,

    /// Called when items are pasted.
    pub on_paste: Option<Callback<Vec<DropItem>>>,

    /// Whether clipboard operations are disabled.
    pub is_disabled: Signal<bool>,
}

impl Default for UseClipboardInput {
    fn default() -> Self {
        Self {
            get_items: None,
            on_copy: None,
            on_cut: None,
            on_paste: None,
            is_disabled: Signal::derive(|| false),
        }
    }
}

/// Return value of the [`use_clipboard`] hook.
pub struct UseClipboardReturn {
    /// Props for the element that should receive clipboard events.
    pub clipboard_props: UseClipboardProps,
}

/// Props from `use_clipboard` that can be spread onto an element.
#[derive(Debug)]
pub struct UseClipboardProps {
    /// Tabindex to make the element focusable (clipboard events require focus).
    pub tabindex: Signal<i32>,
    /// Copy event handler.
    pub on_copy: EventHandler<ClipboardEvent>,
    /// Cut event handler.
    pub on_cut: EventHandler<ClipboardEvent>,
    /// Paste event handler.
    pub on_paste: EventHandler<ClipboardEvent>,
}

impl IntoAttrs for UseClipboardProps {
    type Attrs = UseClipboardAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Tabindex, self.tabindex),
            self.on_copy.into_on(ev::copy),
            self.on_cut.into_on(ev::cut),
            self.on_paste.into_on(ev::paste),
        )
    }
}

/// Attributes for a clipboard-enabled element.
pub type UseClipboardAttrs = (
    Attr<attr::Tabindex, Signal<i32>>,
    On<ev::copy, SharedEventCallback<ClipboardEvent>>,
    On<ev::cut, SharedEventCallback<ClipboardEvent>>,
    On<ev::paste, SharedEventCallback<ClipboardEvent>>,
);

/// Provides clipboard (cut/copy/paste) behavior for a collection.
///
/// Attaches event handlers for `copy`, `cut`, and `paste` events on the
/// element. Uses the same serialization format as drag-and-drop for
/// consistency.
///
/// # Example
///
/// ```ignore
/// let clipboard = use_clipboard(UseClipboardInput {
///     get_items: Some(Callback::new(|action: ClipboardAction| {
///         vec![DragItem::text("Selected content")]
///     })),
///     on_paste: Some(Callback::new(|items: Vec<DropItem>| {
///         for item in &items {
///             if let Some(text) = item.as_text() {
///                 tracing::info!("Pasted: {:?}", text.get("text/plain"));
///             }
///         }
///     })),
///     ..Default::default()
/// });
///
/// view! {
///     <div {..clipboard.clipboard_props.into_attrs()}>
///         "Clipboard-enabled content"
///     </div>
/// }
/// ```
#[allow(clippy::needless_pass_by_value)]
pub fn use_clipboard(input: UseClipboardInput) -> UseClipboardReturn {
    let UseClipboardInput {
        get_items,
        on_copy,
        on_cut,
        on_paste,
        is_disabled: disabled,
    } = input;

    let tabindex = Signal::derive(move || if disabled.get() { -1_i32 } else { 0 });

    let handle_copy = move |e: ClipboardEvent| {
        if disabled.get_untracked() {
            return;
        }
        let Some(get_items) = get_items else {
            return;
        };
        let Some(dt) = e.clipboard_data() else {
            return;
        };

        e.prevent_default();

        let items = get_items.run(ClipboardAction::Copy);
        write_to_data_transfer(&dt, &items);

        if let Some(on_copy) = on_copy {
            on_copy.run(());
        }
    };

    let handle_cut = move |e: ClipboardEvent| {
        if disabled.get_untracked() {
            return;
        }
        let Some(get_items) = get_items else {
            return;
        };
        let Some(dt) = e.clipboard_data() else {
            return;
        };

        e.prevent_default();

        let items = get_items.run(ClipboardAction::Cut);
        write_to_data_transfer(&dt, &items);

        if let Some(on_cut) = on_cut {
            on_cut.run(());
        }
    };

    let handle_paste = move |e: ClipboardEvent| {
        if disabled.get_untracked() {
            return;
        }
        let Some(on_paste) = on_paste else {
            return;
        };
        let Some(dt) = e.clipboard_data() else {
            return;
        };

        e.prevent_default();

        let items = read_drop_items_from_data_transfer(&dt);
        on_paste.run(items);
    };

    UseClipboardReturn {
        clipboard_props: UseClipboardProps {
            tabindex,
            on_copy: EventHandler::new(handle_copy),
            on_cut: EventHandler::new(handle_cut),
            on_paste: EventHandler::new(handle_paste),
        },
    }
}
