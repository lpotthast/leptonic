use leptos::{attr, attr::Attr, prelude::*};

use crate::{
    hooks::IntoAttrs,
    utils::aria::{AriaHidden, AriaRole},
};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/tabs/src/useTabPanel.ts

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// No intentional deviations from the react-aria implementation.
//
// =============================================================================

/// Input parameters for the `use_tab_panel` hook.
#[derive(Debug, Clone)]
pub struct UseTabPanelInput {
    /// The unique key for this panel.
    pub panel_key: String,

    /// The ID base from the parent tabs.
    pub id_base: String,

    /// Whether this panel is visible (its tab is selected).
    pub is_selected: Signal<bool>,
}

/// The return value of the `use_tab_panel` hook.
pub struct UseTabPanelReturn {
    /// Props for programmatic merging. Call `.into_attrs()` for view spreading.
    pub props: UseTabPanelProps,

    /// The ID of the panel.
    pub panel_id: String,

    /// The ID of the associated tab.
    pub tab_id: String,

    /// Whether the panel is visible.
    pub is_selected: Signal<bool>,
}

/// Props from `use_tab_panel` that can be extracted and merged programmatically.
#[derive(Debug)]
pub struct UseTabPanelProps {
    pub id: String,
    pub role: AriaRole,
    pub aria_labelledby: String,
    pub tabindex: &'static str,
    pub aria_hidden: Signal<Option<AriaHidden>>,
}

impl IntoAttrs for UseTabPanelProps {
    type Attrs = UseTabPanelAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Id, self.id),
            Attr(attr::Role, self.role),
            Attr(attr::AriaLabelledby, self.aria_labelledby),
            Attr(attr::Tabindex, self.tabindex),
            Attr(attr::AriaHidden, self.aria_hidden),
        )
    }
}

/// Attributes for the tab panel element.
pub type UseTabPanelAttrs = (
    Attr<attr::Id, String>,
    Attr<attr::Role, AriaRole>,
    Attr<attr::AriaLabelledby, String>,
    Attr<attr::Tabindex, &'static str>,
    Attr<attr::AriaHidden, Signal<Option<AriaHidden>>>,
);

/// Provides the behavior and accessibility for a tab panel.
///
/// A tab panel contains the content associated with a tab.
///
/// # Example
///
/// ```ignore
/// let panel = use_tab_panel(UseTabPanelInput {
///     panel_key: "tab1".to_string(),
///     id_base: tabs.id_base.clone(),
///     is_selected: Signal::derive(move || selected.get() == Some("tab1".to_string())),
/// });
///
/// view! {
///     <div {..panel.props.into_attrs()}>
///         "Panel 1 content"
///     </div>
/// }
/// ```
#[allow(clippy::needless_pass_by_value)]
pub fn use_tab_panel(input: UseTabPanelInput) -> UseTabPanelReturn {
    let UseTabPanelInput {
        panel_key,
        id_base,
        is_selected,
    } = input;

    let panel_id = format!("{id_base}-panel-{panel_key}");
    let tab_id = format!("{id_base}-tab-{panel_key}");

    // Compute aria-hidden
    let aria_hidden = Signal::derive(move || (!is_selected.get()).then_some(AriaHidden::True));

    UseTabPanelReturn {
        props: UseTabPanelProps {
            id: panel_id.clone(),
            role: AriaRole::Tabpanel,
            aria_labelledby: tab_id.clone(),
            tabindex: "0",
            aria_hidden,
        },
        panel_id,
        tab_id,
        is_selected,
    }
}
