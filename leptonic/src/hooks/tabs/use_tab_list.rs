use leptos::attr;
use leptos::attr::Attr;
use leptos::ev;
use leptos::ev::{On, SharedEventCallback};
use leptos::prelude::*;
use web_sys::KeyboardEvent;

use super::use_tabs::TabsOrientation;
use crate::utils::aria::{AriaDisabled, AriaOrientation};
use crate::utils::EventHandler;

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/tabs/src/useTabList.ts

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// No intentional deviations from the react-aria implementation.
//
// =============================================================================

/// Input parameters for the `use_tab_list` hook.
#[derive(Debug, Clone)]
pub struct UseTabListInput {
    /// The ID base from the parent tabs.
    pub id_base: String,

    /// The orientation of the tab list.
    pub orientation: TabsOrientation,

    /// Whether the tab list is disabled.
    pub is_disabled: Signal<bool>,

    /// The label for the tab list.
    pub label: Option<String>,

    /// Callback to navigate to the next tab.
    pub on_focus_next: Option<Callback<()>>,

    /// Callback to navigate to the previous tab.
    pub on_focus_previous: Option<Callback<()>>,

    /// Callback to navigate to the first tab.
    pub on_focus_first: Option<Callback<()>>,

    /// Callback to navigate to the last tab.
    pub on_focus_last: Option<Callback<()>>,
}

/// The return value of the `use_tab_list` hook.
#[derive(Debug)]
pub struct UseTabListReturn {
    /// Props for programmatic merging. Call `.into_attrs()` for view spreading.
    pub props: UseTabListProps,

    /// The ID of the tab list.
    pub tab_list_id: String,
}

/// Props from `use_tab_list` that can be extracted and merged programmatically.
#[derive(Debug)]
pub struct UseTabListProps {
    pub id: String,
    pub role: &'static str,
    pub aria_label: Option<String>,
    pub aria_orientation: AriaOrientation,
    pub aria_disabled: Signal<Option<AriaDisabled>>,
    pub on_keydown: EventHandler<KeyboardEvent>,
}

impl UseTabListProps {
    /// Convert to spreadable attributes for Leptos views, consuming self.
    #[must_use]
    pub fn into_attrs(self) -> UseTabListAttrs {
        (
            Attr(attr::Id, self.id),
            Attr(attr::Role, self.role),
            Attr(attr::AriaLabel, self.aria_label),
            Attr(attr::AriaOrientation, self.aria_orientation),
            Attr(attr::AriaDisabled, self.aria_disabled),
            self.on_keydown.into_on(ev::keydown),
        )
    }
}

/// Attributes for the tab list element.
pub type UseTabListAttrs = (
    Attr<attr::Id, String>,
    Attr<attr::Role, &'static str>,
    Attr<attr::AriaLabel, Option<String>>,
    Attr<attr::AriaOrientation, AriaOrientation>,
    Attr<attr::AriaDisabled, Signal<Option<AriaDisabled>>>,
    On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
);

/// Provides the behavior and accessibility for a tab list.
///
/// A tab list contains the tab buttons that users can interact with to switch panels.
///
/// # Example
///
/// ```ignore
/// let tab_list = use_tab_list(UseTabListInput {
///     id_base: tabs.id_base.clone(),
///     orientation: TabsOrientation::Horizontal,
///     is_disabled: Signal::derive(|| false),
///     ..Default::default()
/// });
///
/// view! {
///     <div {..tab_list.props.into_attrs()}>
///         // Tab buttons...
///     </div>
/// }
/// ```
pub fn use_tab_list(input: UseTabListInput) -> UseTabListReturn {
    let UseTabListInput {
        id_base,
        orientation,
        is_disabled: disabled,
        label,
        on_focus_next,
        on_focus_previous,
        on_focus_first,
        on_focus_last,
    } = input;

    let tab_list_id = format!("{id_base}-tablist");

    let aria_orientation = AriaOrientation::from(orientation);

    let aria_disabled = Signal::derive(move || disabled.get().then_some(AriaDisabled::True));

    // Handle keyboard navigation
    let handle_keydown = move |e: KeyboardEvent| {
        if disabled.get_untracked() {
            return;
        }

        let key = e.key();
        let is_horizontal = orientation == TabsOrientation::Horizontal;

        match key.as_str() {
            "ArrowRight" if is_horizontal => {
                e.prevent_default();
                if let Some(on_next) = on_focus_next {
                    on_next.run(());
                }
            }
            "ArrowLeft" if is_horizontal => {
                e.prevent_default();
                if let Some(on_prev) = on_focus_previous {
                    on_prev.run(());
                }
            }
            "ArrowDown" if !is_horizontal => {
                e.prevent_default();
                if let Some(on_next) = on_focus_next {
                    on_next.run(());
                }
            }
            "ArrowUp" if !is_horizontal => {
                e.prevent_default();
                if let Some(on_prev) = on_focus_previous {
                    on_prev.run(());
                }
            }
            "Home" => {
                e.prevent_default();
                if let Some(on_first) = on_focus_first {
                    on_first.run(());
                }
            }
            "End" => {
                e.prevent_default();
                if let Some(on_last) = on_focus_last {
                    on_last.run(());
                }
            }
            _ => {}
        }
    };

    UseTabListReturn {
        props: UseTabListProps {
            id: tab_list_id.clone(),
            role: "tablist",
            aria_label: label,
            aria_orientation,
            aria_disabled,
            on_keydown: EventHandler::new(handle_keydown),
        },
        tab_list_id,
    }
}
