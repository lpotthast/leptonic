// Upstream: react-aria/src/tabs/useTabPanel.ts @ 99e6102368
use leptos::{
    attr::{self, Attr},
    prelude::*,
};

use super::TabListData;
use crate::{
    hooks::{
        IntoAttrs,
        collections::Key,
        focus::use_has_tabbable_child::{
            UseHasTabbableChildAttrs, UseHasTabbableChildInput, UseHasTabbableChildProps,
            use_has_tabbable_child,
        },
    },
    utils::aria::AriaRole,
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// No intentional deviations from the react-aria implementation.
//
// =============================================================================

/// Input of [`use_tab_panel`].
#[derive(Debug, Clone)]
pub struct UseTabPanelInput {
    /// The tab list.
    pub tabs: TabListData,
    /// The panel's tab. `None`: the selected tab (a single panel showing the selected tab's
    /// content).
    pub key: Option<Key>,
}

/// Return value of [`use_tab_panel`].
#[derive(Debug)]
pub struct UseTabPanelReturn {
    pub tab_panel_props: UseTabPanelProps,
}

/// Props for the tab panel element.
#[derive(Debug)]
pub struct UseTabPanelProps {
    pub id: Signal<String>,
    pub role: AriaRole,
    /// The panel's tab labels it.
    pub aria_labelledby: Signal<String>,
    /// The panel is a tab stop unless it contains tabbable elements.
    pub tabindex: Signal<Option<i32>>,
    pub tabbable_child: UseHasTabbableChildProps,
}

pub type UseTabPanelAttrs = (
    Attr<attr::Id, Signal<String>>,
    Attr<attr::Role, AriaRole>,
    Attr<attr::AriaLabelledby, Signal<String>>,
    Attr<attr::Tabindex, Signal<Option<i32>>>,
    UseHasTabbableChildAttrs,
);

impl IntoAttrs for UseTabPanelProps {
    type Attrs = UseTabPanelAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Id, self.id),
            Attr(attr::Role, self.role),
            Attr(attr::AriaLabelledby, self.aria_labelledby),
            Attr(attr::Tabindex, self.tabindex),
            self.tabbable_child.into_attrs(),
        )
    }
}

/// The content of a tab, labelled by the tab.
pub fn use_tab_panel(input: UseTabPanelInput) -> UseTabPanelReturn {
    let UseTabPanelInput { tabs, key } = input;
    let tabbable_child = use_has_tabbable_child(UseHasTabbableChildInput::default());
    let has_tabbable_child = tabbable_child.has_tabbable_child;
    let state = tabs.state;
    let key = StoredValue::new(key);
    let panel_key = move || {
        key.get_value()
            .or_else(|| state.selected_key())
            .map_or_else(String::new, |k| k.to_string())
    };
    let id_tabs = tabs.clone();
    let label_tabs = tabs;
    UseTabPanelReturn {
        tab_panel_props: UseTabPanelProps {
            id: Signal::derive(move || id_tabs.tab_panel_id(&Key::from(panel_key()))),
            role: AriaRole::Tabpanel,
            aria_labelledby: Signal::derive(move || label_tabs.tab_id(&Key::from(panel_key()))),
            tabindex: Signal::derive(move || (!has_tabbable_child.get()).then_some(0)),
            tabbable_child: tabbable_child.props,
        },
    }
}
