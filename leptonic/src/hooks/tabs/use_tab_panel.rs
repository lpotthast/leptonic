// Upstream: react-aria/src/tabs/useTabPanel.ts @ 99e6102368
use leptos::{
    attr::{self, Attr},
    prelude::*,
};

use super::TabListState;
use crate::{
    IntoAttrs,
    hooks::{
        collections::Key,
        focus::use_has_tabbable_child::{
            UseHasTabbableChildAttrs, UseHasTabbableChildInput, UseHasTabbableChildProps,
            use_has_tabbable_child,
        },
    },
    labels,
    utils::aria::AriaRole,
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - `key` names the panel's tab, so that several panels can be rendered at once (force-mounted);
//   `None` is the selected tab, as react-aria's single panel.
//
// ## DIFFERENT BEHAVIOR
// - A panel with a `key` is labelled by its own tab and gets its own id (react-aria: by the
//   selected tab, with the selected tab's panel id). Reason: a force-mounted panel of an
//   unselected tab must not claim the selected tab; react-aria-components drops the panel props
//   from unselected panels instead, as the `TabPanel` atom does.
//
// =============================================================================

/// Input of [`use_tab_panel`].
#[derive(Debug, Clone)]
pub struct UseTabPanelInput {
    /// The tab list's state.
    pub state: TabListState,
    /// The panel's tab. `None`: the selected tab (a single panel showing the selected tab's
    /// content).
    pub key: Option<Key>,
    /// Names the panel, next to its tab (the panel then labels itself too).
    pub aria_label: MaybeProp<String>,
    /// The ids of elements describing the panel.
    pub aria_describedby: Option<String>,
    /// The id of an element with details about the panel.
    pub aria_details: Option<String>,
}

/// Return value of [`use_tab_panel`].
#[derive(Debug)]
pub struct UseTabPanelReturn {
    /// For the panel element.
    pub props: UseTabPanelProps,
}

/// Props for the tab panel element.
#[derive(Debug)]
pub struct UseTabPanelProps {
    /// `None` without tabs.
    pub id: Signal<Option<String>>,
    pub role: AriaRole,
    pub aria_label: Signal<Option<String>>,
    /// The panel's tab labels it (and the panel itself, next to an `aria_label`).
    pub aria_labelledby: Signal<Option<String>>,
    /// The panel is a tab stop unless it contains tabbable elements.
    pub tabindex: Signal<Option<i32>>,
    pub aria_describedby: Option<String>,
    pub aria_details: Option<String>,
    pub tabbable_child: UseHasTabbableChildProps,
}

pub type UseTabPanelAttrs = (
    Attr<attr::Id, Signal<Option<String>>>,
    Attr<attr::Role, AriaRole>,
    Attr<attr::AriaLabel, Signal<Option<String>>>,
    Attr<attr::AriaLabelledby, Signal<Option<String>>>,
    Attr<attr::Tabindex, Signal<Option<i32>>>,
    Attr<attr::AriaDescribedby, Option<String>>,
    Attr<attr::AriaDetails, Option<String>>,
    UseHasTabbableChildAttrs,
);

impl IntoAttrs for UseTabPanelProps {
    type Attrs = UseTabPanelAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Id, self.id),
            Attr(attr::Role, self.role),
            Attr(attr::AriaLabel, self.aria_label),
            Attr(attr::AriaLabelledby, self.aria_labelledby),
            Attr(attr::Tabindex, self.tabindex),
            Attr(attr::AriaDescribedby, self.aria_describedby),
            Attr(attr::AriaDetails, self.aria_details),
            self.tabbable_child.into_attrs(),
        )
    }
}

/// The content of a tab, labelled by the tab: a tab stop unless it contains tabbable elements.
pub fn use_tab_panel(input: UseTabPanelInput) -> UseTabPanelReturn {
    let UseTabPanelInput {
        state,
        key,
        aria_label,
        aria_describedby,
        aria_details,
    } = input;
    let tabbable_child = use_has_tabbable_child(UseHasTabbableChildInput::default());
    let has_tabbable_child = tabbable_child.has_tabbable_child;
    let key = StoredValue::new(key);
    let panel_key = move || key.get_value().or_else(|| state.selected_key());
    let labelling = Memo::new(move |_| {
        let key = panel_key()?;
        let id = state.tab_panel_id(&key);
        let labelling = labels(&id, aria_label.get(), Some(&state.tab_id(&key)));
        Some((id, labelling))
    });
    UseTabPanelReturn {
        props: UseTabPanelProps {
            id: Signal::derive(move || labelling.with(|l| l.as_ref().map(|(id, _)| id.clone()))),
            role: AriaRole::Tabpanel,
            aria_label: Signal::derive(move || {
                labelling.with(|l| l.as_ref().and_then(|(_, l)| l.aria_label.clone()))
            }),
            aria_labelledby: Signal::derive(move || {
                labelling.with(|l| l.as_ref().and_then(|(_, l)| l.aria_labelledby.clone()))
            }),
            tabindex: Signal::derive(move || (!has_tabbable_child.get()).then_some(0)),
            aria_describedby,
            aria_details,
            tabbable_child: tabbable_child.props,
        },
    }
}
