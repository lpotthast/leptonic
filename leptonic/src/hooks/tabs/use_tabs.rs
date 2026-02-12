use leptos::attr;
use leptos::attr::Attr;
use leptos::prelude::*;
use uuid::Uuid;

use crate::utils::aria::AriaOrientation;
use crate::hooks::IntoAttrs;

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/tabs/src/useTabList.ts

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// No intentional deviations from the react-aria implementation.
//
// =============================================================================

/// The orientation of the tab list.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TabsOrientation {
    /// Horizontal tabs.
    #[default]
    Horizontal,
    /// Vertical tabs.
    Vertical,
}

impl From<TabsOrientation> for AriaOrientation {
    fn from(value: TabsOrientation) -> Self {
        match value {
            TabsOrientation::Horizontal => Self::Horizontal,
            TabsOrientation::Vertical => Self::Vertical,
        }
    }
}

/// The keyboard activation mode for tabs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TabsActivationMode {
    /// Tabs are activated automatically when focused.
    #[default]
    Automatic,
    /// Tabs must be explicitly activated with Enter/Space.
    Manual,
}

/// Input parameters for the `use_tabs` hook.
#[derive(Debug, Clone)]
pub struct UseTabsInput {
    /// The currently selected tab key.
    pub selected_key: Signal<Option<String>>,

    /// The default selected tab key.
    pub default_selected_key: Option<String>,

    /// Whether the tabs are disabled.
    pub is_disabled: Signal<bool>,

    /// The orientation of the tab list.
    pub orientation: TabsOrientation,

    /// The activation mode.
    pub activation_mode: TabsActivationMode,

    /// Callback when the selected tab changes.
    pub on_selection_change: Option<Callback<String>>,
}

impl Default for UseTabsInput {
    fn default() -> Self {
        Self {
            selected_key: Signal::derive(|| None),
            default_selected_key: None,
            is_disabled: Signal::derive(|| false),
            orientation: TabsOrientation::Horizontal,
            activation_mode: TabsActivationMode::Automatic,
            on_selection_change: None,
        }
    }
}

/// The return value of the `use_tabs` hook.
pub struct UseTabsReturn {
    /// Props for programmatic merging. Call `.into_attrs()` for view spreading.
    pub props: UseTabsProps,

    /// The ID base for the tabs.
    pub id_base: String,

    /// The currently selected tab key.
    pub selected_key: Signal<Option<String>>,

    /// The orientation.
    pub orientation: TabsOrientation,

    /// The activation mode.
    pub activation_mode: TabsActivationMode,

    /// Select a tab.
    pub select_tab: Callback<String>,
}

/// Props from `use_tabs` that can be extracted and merged programmatically.
#[derive(Debug)]
pub struct UseTabsProps {
    pub id: String,
}

impl IntoAttrs for UseTabsProps {
    type Attrs = UseTabsAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (Attr(attr::Id, self.id),)
    }
}

/// Attributes for the tabs container element.
pub type UseTabsAttrs = (Attr<attr::Id, String>,);

/// Provides the behavior and accessibility for a tabs component.
///
/// Tabs organize content into separate views where only one is visible at a time.
///
/// # Example
///
/// ```ignore
/// let (selected, set_selected) = signal(Some("tab1".to_string()));
///
/// let tabs = use_tabs(UseTabsInput {
///     selected_key: selected.into(),
///     on_selection_change: Some(Callback::new(move |key| set_selected.set(Some(key)))),
///     ..Default::default()
/// });
///
/// view! {
///     <div {..tabs.props.into_attrs()}>
///         // Tab list and panels...
///     </div>
/// }
/// ```
#[allow(clippy::needless_pass_by_value)]
pub fn use_tabs(input: UseTabsInput) -> UseTabsReturn {
    let UseTabsInput {
        selected_key,
        default_selected_key,
        is_disabled: disabled,
        orientation,
        activation_mode,
        on_selection_change,
    } = input;

    let id_base = format!("tabs-{}", Uuid::new_v4());

    let select_tab = Callback::new(move |key: String| {
        if let Some(on_change) = on_selection_change {
            on_change.run(key);
        }
    });

    UseTabsReturn {
        props: UseTabsProps {
            id: id_base.clone(),
        },
        id_base,
        selected_key,
        orientation,
        activation_mode,
        select_tab,
    }
}
