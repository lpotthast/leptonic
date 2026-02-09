use leptos::attr;
use leptos::attr::{Attr, Attribute};
use leptos::ev;
use leptos::ev::{on, On, SharedEventCallback};
use leptos::prelude::*;
use uuid::Uuid;
use web_sys::KeyboardEvent;

use crate::utils::aria::{AriaDisabled, AriaSelected};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/tag/src/useTagGroup.ts

/// The selection mode for a tag group.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TagGroupSelectionMode {
    /// No selection allowed.
    #[default]
    None,
    /// Single tag selection.
    Single,
    /// Multiple tag selection.
    Multiple,
}

/// Input parameters for the `use_tag_group` hook.
#[derive(Debug, Clone)]
pub struct UseTagGroupInput {
    /// The label for the tag group.
    pub label: Option<String>,

    /// The selection mode.
    pub selection_mode: TagGroupSelectionMode,

    /// Whether the tag group is disabled.
    pub is_disabled: Signal<bool>,

    /// The currently selected tag keys.
    pub selected_keys: Signal<Vec<String>>,

    /// Callback when selection changes.
    pub on_selection_change: Option<Callback<Vec<String>>>,

    /// Callback when a tag is removed.
    pub on_remove: Option<Callback<String>>,

    /// Whether tags can be removed.
    pub allow_removal: bool,
}

impl Default for UseTagGroupInput {
    fn default() -> Self {
        Self {
            label: None,
            selection_mode: TagGroupSelectionMode::None,
            is_disabled: Signal::derive(|| false),
            selected_keys: Signal::derive(Vec::new),
            on_selection_change: None,
            on_remove: None,
            allow_removal: false,
        }
    }
}

/// The return value of the `use_tag_group` hook.
#[derive(Debug, Clone)]
pub struct UseTagGroupReturn {
    /// Props for the tag group container element.
    pub group_props: UseTagGroupAttrs,

    /// Props for the label element.
    pub label_props: UseTagGroupLabelProps,

    /// The ID of the tag group.
    pub group_id: String,

    /// The currently focused tag key.
    pub focused_key: Signal<Option<String>>,

    /// Set the focused tag.
    pub set_focused_key: Callback<Option<String>>,
}

/// Attributes for the tag group container element.
pub type UseTagGroupAttrs = (
    Attr<attr::Id, String>,
    Attr<attr::Role, &'static str>,
    Attr<attr::AriaLabel, Option<String>>,
    Attr<attr::AriaLabelledby, Option<String>>,
    Attr<attr::AriaDisabled, Signal<Option<AriaDisabled>>>,
    On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
);

/// Props for the tag group label element.
#[derive(Debug, Clone)]
pub struct UseTagGroupLabelProps {
    /// The ID of the label.
    pub id: String,
}

/// Provides the behavior and accessibility for a tag group.
///
/// A tag group displays a list of tags that can be selected or removed.
///
/// # Example
///
/// ```ignore
/// let tag_group = use_tag_group(UseTagGroupInput {
///     label: Some("Categories".to_string()),
///     selection_mode: TagGroupSelectionMode::Multiple,
///     allow_removal: true,
///     ..Default::default()
/// });
///
/// view! {
///     <div>
///         <label id=tag_group.label_props.id>"Categories"</label>
///         <div {..tag_group.group_props}>
///             // Tags...
///         </div>
///     </div>
/// }
/// ```
#[allow(clippy::needless_pass_by_value)]
pub fn use_tag_group(input: UseTagGroupInput) -> UseTagGroupReturn {
    let base_id = Uuid::new_v4();
    let group_id = format!("tag-group-{base_id}");
    let label_id = format!("tag-group-label-{base_id}");

    let is_disabled = input.is_disabled;

    // Track focused tag
    let (focused_key, set_focused_key_signal) = signal::<Option<String>>(None);

    let set_focused_key = Callback::new(move |key: Option<String>| {
        set_focused_key_signal.set(key);
    });

    let aria_disabled = Signal::derive(move || is_disabled.get().then_some(AriaDisabled::True));

    let aria_labelledby = if input.label.is_some() {
        Some(label_id.clone())
    } else {
        None
    };

    let handle_keydown = move |_e: KeyboardEvent| {
        // Navigation is handled at the tag level
    };

    UseTagGroupReturn {
        group_props: (
            Attr(attr::Id, group_id.clone()),
            Attr(attr::Role, "grid"),
            Attr(attr::AriaLabel, None),
            Attr(attr::AriaLabelledby, aria_labelledby),
            Attr(attr::AriaDisabled, aria_disabled),
            on(ev::keydown, handle_keydown).into_cloneable(),
        ),
        label_props: UseTagGroupLabelProps { id: label_id },
        group_id,
        focused_key: focused_key.into(),
        set_focused_key,
    }
}

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
    let tag_key = input.tag_key.clone();
    let is_selected = input.is_selected;
    let is_focused = input.is_focused;
    let is_disabled = input.is_disabled;
    let allow_removal = input.allow_removal;
    let on_select = input.on_select;
    let on_remove = input.on_remove;
    let on_focus_next = input.on_focus_next;
    let on_focus_previous = input.on_focus_previous;

    let aria_selected = Signal::derive(move || {
        if on_select.is_some() {
            Some(AriaSelected::from(is_selected.get()))
        } else {
            None
        }
    });

    let aria_disabled = Signal::derive(move || is_disabled.get().then_some(AriaDisabled::True));

    let tabindex = Signal::derive(move || if is_focused.get() { "0" } else { "-1" });

    let handle_click = move |_e: web_sys::MouseEvent| {
        if is_disabled.get_untracked() {
            return;
        }
        if let Some(on_select) = on_select {
            on_select.run(());
        }
    };

    let handle_keydown = move |e: KeyboardEvent| {
        if is_disabled.get_untracked() {
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
        if is_disabled.get_untracked() || !allow_removal {
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
