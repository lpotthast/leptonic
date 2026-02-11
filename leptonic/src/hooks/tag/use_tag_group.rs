use leptos::attr;
use leptos::attr::Attr;
use leptos::ev;
use leptos::ev::{On, SharedEventCallback};
use leptos::prelude::*;
use uuid::Uuid;
use web_sys::KeyboardEvent;

use crate::utils::aria::AriaDisabled;
use crate::utils::EventHandler;

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/tag/src/useTagGroup.ts

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// No intentional deviations from the react-aria implementation.
//
// =============================================================================

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
    pub group_props: UseTagGroupProps,

    /// Props for the label element.
    pub label_props: UseTagGroupLabelProps,

    /// The ID of the tag group.
    pub group_id: String,

    /// The currently focused tag key.
    pub focused_key: Signal<Option<String>>,

    /// Set the focused tag.
    pub set_focused_key: Callback<Option<String>>,
}

/// Props from `use_tag_group` that can be extracted and merged programmatically.
#[derive(Debug, Clone)]
pub struct UseTagGroupProps {
    pub id: String,
    pub role: &'static str,
    pub aria_label: Option<String>,
    pub aria_labelledby: Option<String>,
    pub aria_disabled: Signal<Option<AriaDisabled>>,
    pub on_keydown: EventHandler<KeyboardEvent>,
}

impl UseTagGroupProps {
    /// Convert to spreadable attributes for Leptos views, cloning internally.
    #[must_use]
    pub fn to_attrs(&self) -> UseTagGroupAttrs {
        self.clone().into_attrs()
    }

    /// Convert to spreadable attributes for Leptos views, consuming self.
    #[must_use]
    pub fn into_attrs(self) -> UseTagGroupAttrs {
        (
            Attr(attr::Id, self.id),
            Attr(attr::Role, self.role),
            Attr(attr::AriaLabel, self.aria_label),
            Attr(attr::AriaLabelledby, self.aria_labelledby),
            Attr(attr::AriaDisabled, self.aria_disabled),
            self.on_keydown.into_on(ev::keydown),
        )
    }
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
    let UseTagGroupInput {
        label,
        selection_mode,
        is_disabled: disabled,
        selected_keys,
        on_selection_change,
        on_remove,
        allow_removal,
    } = input;

    let base_id = Uuid::new_v4();
    let group_id = format!("tag-group-{base_id}");
    let label_id = format!("tag-group-label-{base_id}");

    // Track focused tag
    let (focused_key, set_focused_key_signal) = signal::<Option<String>>(None);

    let set_focused_key = Callback::new(move |key: Option<String>| {
        set_focused_key_signal.set(key);
    });

    let aria_disabled = Signal::derive(move || disabled.get().then_some(AriaDisabled::True));

    let aria_labelledby = if label.is_some() {
        Some(label_id.clone())
    } else {
        None
    };

    let handle_keydown = move |_e: KeyboardEvent| {
        // Navigation is handled at the tag level
    };

    UseTagGroupReturn {
        group_props: UseTagGroupProps {
            id: group_id.clone(),
            role: "grid",
            aria_label: None,
            aria_labelledby,
            aria_disabled,
            on_keydown: EventHandler::new(handle_keydown),
        },
        label_props: UseTagGroupLabelProps { id: label_id },
        group_id,
        focused_key: focused_key.into(),
        set_focused_key,
    }
}
