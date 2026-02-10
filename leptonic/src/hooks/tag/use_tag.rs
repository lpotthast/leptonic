use leptos::attr;
use leptos::attr::Attr;
use leptos::ev;
use leptos::ev::{On, SharedEventCallback};
use leptos::prelude::*;
use web_sys::KeyboardEvent;

use crate::utils::aria::{AriaDisabled, AriaSelected};
use crate::utils::EventHandler;

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
    pub row_props: UseTagRowProps,

    /// Props for the tag cell element.
    pub cell_props: UseTagCellProps,

    /// Props for the remove button (if removable).
    pub remove_button_props: UseTagRemoveButtonProps,

    /// The tag key.
    pub tag_key: String,

    /// Whether the tag is selected.
    pub is_selected: Signal<bool>,
}

/// Props from `use_tag` for the row element that can be extracted and merged programmatically.
#[derive(Debug, Clone)]
pub struct UseTagRowProps {
    pub role: &'static str,
    pub aria_selected: Signal<Option<AriaSelected>>,
    pub aria_disabled: Signal<Option<AriaDisabled>>,
    pub tabindex: Signal<&'static str>,
    pub on_click: EventHandler<web_sys::MouseEvent>,
    pub on_keydown: EventHandler<KeyboardEvent>,
    pub on_focus: EventHandler<web_sys::FocusEvent>,
}

impl UseTagRowProps {
    /// Convert to spreadable attributes for Leptos views, cloning internally.
    #[must_use]
    pub fn to_attrs(&self) -> UseTagRowAttrs {
        self.clone().into_attrs()
    }

    /// Convert to spreadable attributes for Leptos views, consuming self.
    #[must_use]
    pub fn into_attrs(self) -> UseTagRowAttrs {
        (
            Attr(attr::Role, self.role),
            Attr(attr::AriaSelected, self.aria_selected),
            Attr(attr::AriaDisabled, self.aria_disabled),
            Attr(attr::Tabindex, self.tabindex),
            self.on_click.into_on(ev::click),
            self.on_keydown.into_on(ev::keydown),
            self.on_focus.into_on(ev::focus),
        )
    }
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

/// Props from `use_tag` for the cell element that can be extracted and merged programmatically.
#[derive(Debug, Clone)]
pub struct UseTagCellProps {
    pub role: &'static str,
}

impl UseTagCellProps {
    /// Convert to spreadable attributes for Leptos views, cloning internally.
    #[must_use]
    pub fn to_attrs(&self) -> UseTagCellAttrs {
        self.clone().into_attrs()
    }

    /// Convert to spreadable attributes for Leptos views, consuming self.
    #[must_use]
    pub fn into_attrs(self) -> UseTagCellAttrs {
        (Attr(attr::Role, self.role),)
    }
}

/// Attributes for the tag cell element.
pub type UseTagCellAttrs = (Attr<attr::Role, &'static str>,);

/// Props from `use_tag` for the remove button that can be extracted and merged programmatically.
#[derive(Debug, Clone)]
pub struct UseTagRemoveButtonProps {
    pub aria_label: &'static str,
    pub tabindex: &'static str,
    pub on_click: EventHandler<web_sys::MouseEvent>,
}

impl UseTagRemoveButtonProps {
    /// Convert to spreadable attributes for Leptos views, cloning internally.
    #[must_use]
    pub fn to_attrs(&self) -> UseTagRemoveButtonAttrs {
        self.clone().into_attrs()
    }

    /// Convert to spreadable attributes for Leptos views, consuming self.
    #[must_use]
    pub fn into_attrs(self) -> UseTagRemoveButtonAttrs {
        (
            Attr(attr::AriaLabel, self.aria_label),
            Attr(attr::Tabindex, self.tabindex),
            self.on_click.into_on(ev::click),
        )
    }
}

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
        row_props: UseTagRowProps {
            role: "row",
            aria_selected,
            aria_disabled,
            tabindex,
            on_click: EventHandler::new(handle_click),
            on_keydown: EventHandler::new(handle_keydown),
            on_focus: EventHandler::new(handle_focus),
        },
        cell_props: UseTagCellProps { role: "gridcell" },
        remove_button_props: UseTagRemoveButtonProps {
            aria_label: "Remove",
            tabindex: "-1",
            on_click: EventHandler::new(handle_remove_click),
        },
        tag_key,
        is_selected,
    }
}
