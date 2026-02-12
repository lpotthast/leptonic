use leptos::attr;
use leptos::attr::Attr;
use leptos::prelude::*;

use crate::utils::aria::AriaHidden;

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/select/src/useHiddenSelect.ts

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// No intentional deviations from the react-aria implementation.
//
// =============================================================================

/// Input parameters for the `use_hidden_select` hook.
#[derive(Clone)]
pub struct UseHiddenSelectInput<K>
where
    K: Clone + Send + Sync + 'static,
{
    /// The name attribute for form submission.
    pub name: Option<&'static str>,

    /// Whether the field is disabled.
    pub is_disabled: Signal<bool>,

    /// Whether the field is required.
    pub is_required: bool,

    /// The currently selected key.
    pub selected_key: Signal<Option<K>>,

    /// A function to convert the key to a string value.
    pub key_to_value: Option<Callback<K, String>>,

    /// The ID of the select trigger element.
    pub trigger_id: String,

    /// The label for the select (for accessibility).
    pub label: Option<String>,
}

impl<K: Clone + Send + Sync + 'static + Default> Default for UseHiddenSelectInput<K> {
    fn default() -> Self {
        Self {
            name: None,
            is_disabled: Signal::derive(|| false),
            is_required: false,
            selected_key: Signal::derive(|| None),
            key_to_value: None,
            trigger_id: String::new(),
            label: None,
        }
    }
}

/// The return value of the `use_hidden_select` hook.
pub struct UseHiddenSelectReturn {
    /// Props for the container div that holds the hidden elements.
    pub container_props: UseHiddenSelectContainerProps,

    /// Props for the hidden input element.
    pub input_props: UseHiddenSelectInputProps,

    /// Props for the hidden select element.
    pub select_props: UseHiddenSelectSelectProps,

    /// The current value for the hidden input.
    pub value: Signal<String>,
}

/// Props from `use_hidden_select` for the container that can be extracted and merged programmatically.
#[derive(Debug)]
pub struct UseHiddenSelectContainerProps {
    pub aria_hidden: AriaHidden,
}

impl UseHiddenSelectContainerProps {
    /// Convert to spreadable attributes for Leptos views, consuming self.
    #[must_use]
    pub fn into_attrs(self) -> UseHiddenSelectContainerAttrs {
        (Attr(attr::AriaHidden, self.aria_hidden),)
    }
}

/// Attributes for the hidden container.
pub type UseHiddenSelectContainerAttrs = (Attr<attr::AriaHidden, AriaHidden>,);

/// The style string for hiding the container visually.
pub const HIDDEN_SELECT_CONTAINER_STYLE: &str = "position: absolute; width: 1px; height: 1px; padding: 0; margin: -1px; overflow: hidden; clip: rect(0, 0, 0, 0); white-space: nowrap; border: 0;";

/// Props from `use_hidden_select` for the input element that can be extracted and merged programmatically.
#[derive(Debug)]
pub struct UseHiddenSelectInputProps {
    pub r#type: &'static str,
    pub name: Option<&'static str>,
    pub value: Signal<String>,
    pub disabled: Signal<bool>,
    pub required: bool,
    pub tabindex: &'static str,
}

impl UseHiddenSelectInputProps {
    /// Convert to spreadable attributes for Leptos views, consuming self.
    #[must_use]
    pub fn into_attrs(self) -> UseHiddenSelectInputAttrs {
        (
            Attr(attr::Type, self.r#type),
            Attr(attr::Name, self.name),
            Attr(attr::Value, self.value),
            Attr(attr::Disabled, self.disabled),
            Attr(attr::Required, self.required),
            Attr(attr::Tabindex, self.tabindex),
        )
    }
}

/// Attributes for the hidden input element.
pub type UseHiddenSelectInputAttrs = (
    Attr<attr::Type, &'static str>,
    Attr<attr::Name, Option<&'static str>>,
    Attr<attr::Value, Signal<String>>,
    Attr<attr::Disabled, Signal<bool>>,
    Attr<attr::Required, bool>,
    Attr<attr::Tabindex, &'static str>,
);

/// Props from `use_hidden_select` for the select element that can be extracted and merged programmatically.
#[derive(Debug)]
pub struct UseHiddenSelectSelectProps {
    pub name: Option<&'static str>,
    pub disabled: Signal<bool>,
    pub required: bool,
    pub tabindex: &'static str,
    pub aria_labelledby: Option<String>,
}

impl UseHiddenSelectSelectProps {
    /// Convert to spreadable attributes for Leptos views, consuming self.
    #[must_use]
    pub fn into_attrs(self) -> UseHiddenSelectSelectAttrs {
        (
            Attr(attr::Name, self.name),
            Attr(attr::Disabled, self.disabled),
            Attr(attr::Required, self.required),
            Attr(attr::Tabindex, self.tabindex),
            Attr(attr::AriaLabelledby, self.aria_labelledby),
        )
    }
}

/// Attributes for the hidden select element.
pub type UseHiddenSelectSelectAttrs = (
    Attr<attr::Name, Option<&'static str>>,
    Attr<attr::Disabled, Signal<bool>>,
    Attr<attr::Required, bool>,
    Attr<attr::Tabindex, &'static str>,
    Attr<attr::AriaLabelledby, Option<String>>,
);

/// Provides a hidden native select element for form submission and mobile accessibility.
///
/// This is useful for integrating with native form submission and providing
/// a better experience on mobile devices where native selects can be triggered.
///
/// # Example
///
/// ```ignore
/// let hidden = use_hidden_select(UseHiddenSelectInput {
///     name: Some("fruit"),
///     selected_key: selected.into(),
///     trigger_id: select.trigger_id.clone(),
///     key_to_value: Some(Callback::new(|k: String| k)),
///     ..Default::default()
/// });
///
/// view! {
///     <div {..hidden.container_props.into_attrs()}>
///         <input {..hidden.input_props.into_attrs()} />
///     </div>
/// }
/// ```
#[allow(clippy::needless_pass_by_value)]
pub fn use_hidden_select<K>(input: UseHiddenSelectInput<K>) -> UseHiddenSelectReturn
where
    K: Clone + Send + Sync + 'static,
{
    let UseHiddenSelectInput {
        name,
        is_disabled,
        is_required,
        selected_key,
        key_to_value,
        trigger_id,
        label,
    } = input;

    // Compute the value for the hidden input
    let value = Signal::derive(move || {
        selected_key.get().map_or(String::new(), |key| {
            if let Some(converter) = key_to_value {
                converter.run(key)
            } else {
                String::new()
            }
        })
    });

    // Build aria-labelledby
    let aria_labelledby = if label.is_some() {
        Some(trigger_id)
    } else {
        None
    };

    UseHiddenSelectReturn {
        container_props: UseHiddenSelectContainerProps {
            aria_hidden: AriaHidden::True,
        },
        input_props: UseHiddenSelectInputProps {
            r#type: "hidden",
            name,
            value,
            disabled: is_disabled,
            required: is_required,
            tabindex: "-1",
        },
        select_props: UseHiddenSelectSelectProps {
            name,
            disabled: is_disabled,
            required: is_required,
            tabindex: "-1",
            aria_labelledby,
        },
        value,
    }
}
