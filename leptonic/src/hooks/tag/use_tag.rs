use leptos::attr;
use leptos::attr::{Attr, Attribute};
use leptos::ev;
use leptos::ev::{on, On, SharedEventCallback};
use leptos::prelude::*;
use web_sys::KeyboardEvent;

use crate::utils::aria::{AriaDisabled, AriaSelected};

/// Input parameters for the `use_tag` hook.
#[derive(Debug, Clone)]
pub struct UseTagInput {
    /// The unique key for this tag.
    pub tag_key: String,

    /// Whether the tag is selected.
    pub is_selected: Signal<bool>,

    /// Whether the tag is focused.
    pub is_focused: Signal<bool>,

    /// Whether the tag is disabled.
    pub is_disabled: Signal<bool>,

    /// Whether the tag can be removed.
    pub allow_removal: bool,

    /// Callback when the tag is selected.
    pub on_select: Option<Callback<()>>,

    /// Callback when the tag is removed.
    pub on_remove: Option<Callback<()>>,

    /// Callback to navigate to the next tag.
    pub on_focus_next: Option<Callback<()>>,

    /// Callback to navigate to the previous tag.
    pub on_focus_previous: Option<Callback<()>>,
}

/// The return value of the `use_tag` hook.
#[derive(Debug, Clone)]
pub struct UseTagReturn {
    /// Props for the tag row element.
    pub row_props: UseTagRowAttrs,

    /// Props for the tag cell element.
    pub cell_props: UseTagCellAttrs,

    /// Props for the remove button (if removable).
    pub remove_button_props: UseTagRemoveButtonAttrs,

    /// The tag key.
    pub tag_key: String,

    /// Whether the tag is selected.
    pub is_selected: Signal<bool>,
}

/// Attributes for the tag row element.
pub type UseTagRowAttrs = (
    Attr<attr::Role, &'static str>,
    Attr<attr::AriaSelected, Signal<Option<AriaSelected>>>,
    Attr<attr::AriaDisabled, Signal<Option<AriaDisabled>>>,
    Attr<attr::Tabindex, Signal<&'static str>>,
    On<ev::click, SharedEventCallback<web_sys::MouseEvent>>,
    On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
    On<ev::focus, SharedEventCallback<web_sys::FocusEvent>>,
);

/// Attributes for the tag cell element.
pub type UseTagCellAttrs = (Attr<attr::Role, &'static str>,);

/// Attributes for the tag remove button.
pub type UseTagRemoveButtonAttrs = (
    Attr<attr::AriaLabel, &'static str>,
    Attr<attr::Tabindex, &'static str>,
    On<ev::click, SharedEventCallback<web_sys::MouseEvent>>,
);

/// Provides the behavior and accessibility for a single tag.
#[allow(clippy::needless_pass_by_value)]
pub fn use_tag(input: UseTagInput) -> UseTagReturn {
    let UseTagInput {
        tag_key,
        is_selected,
        is_focused,
        is_disabled: disabled,
        allow_removal,
        on_select,
        on_remove,
        on_focus_next,
        on_focus_previous,
    } = input;

    let aria_selected = Signal::derive(move || {
        if on_select.is_some() {
            Some(AriaSelected::from(is_selected.get()))
        } else {
            None
        }
    });

    let aria_disabled = Signal::derive(move || disabled.get().then_some(AriaDisabled::True));

    let tabindex = Signal::derive(move || if is_focused.get() { "0" } else { "-1" });

    let handle_click = move |_e: web_sys::MouseEvent| {
        if disabled.get_untracked() {
            return;
        }
        if let Some(on_select) = on_select {
            on_select.run(());
        }
    };

    let handle_keydown = move |e: KeyboardEvent| {
        if disabled.get_untracked() {
            return;
        }

        let key = e.key();
        match key.as_str() {
            "Enter" | " " => {
                e.prevent_default();
                if let Some(on_select) = on_select {
                    on_select.run(());
                }
            }
            "Delete" | "Backspace" => {
                if allow_removal {
                    e.prevent_default();
                    if let Some(on_remove) = on_remove {
                        on_remove.run(());
                    }
                }
            }
            "ArrowRight" | "ArrowDown" => {
                e.prevent_default();
                if let Some(on_next) = on_focus_next {
                    on_next.run(());
                }
            }
            "ArrowLeft" | "ArrowUp" => {
                e.prevent_default();
                if let Some(on_prev) = on_focus_previous {
                    on_prev.run(());
                }
            }
            _ => {}
        }
    };

    let handle_focus = move |_e: web_sys::FocusEvent| {
        // Focus is managed by parent
    };

    let handle_remove_click = move |e: web_sys::MouseEvent| {
        e.stop_propagation();
        if disabled.get_untracked() || !allow_removal {
            return;
        }
        if let Some(on_remove) = on_remove {
            on_remove.run(());
        }
    };

    UseTagReturn {
        row_props: (
            Attr(attr::Role, "row"),
            Attr(attr::AriaSelected, aria_selected),
            Attr(attr::AriaDisabled, aria_disabled),
            Attr(attr::Tabindex, tabindex),
            on(ev::click, handle_click).into_cloneable(),
            on(ev::keydown, handle_keydown).into_cloneable(),
            on(ev::focus, handle_focus).into_cloneable(),
        ),
        cell_props: (Attr(attr::Role, "gridcell"),),
        remove_button_props: (
            Attr(attr::AriaLabel, "Remove"),
            Attr(attr::Tabindex, "-1"),
            on(ev::click, handle_remove_click).into_cloneable(),
        ),
        tag_key,
        is_selected,
    }
}
