use leptos::{
    attr,
    attr::Attr,
    ev,
    ev::{On, SharedEventCallback},
    prelude::*,
};
use uuid::Uuid;
use web_sys::KeyboardEvent;

use crate::{
    hooks::IntoAttrs,
    utils::{
        aria::{AriaDisabled, AriaOrientation},
        EventHandler,
    },
};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/toolbar/src/useToolbar.ts

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// No intentional deviations from the react-aria implementation.
//
// =============================================================================

/// The orientation of a toolbar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ToolbarOrientation {
    /// Horizontal toolbar.
    #[default]
    Horizontal,
    /// Vertical toolbar.
    Vertical,
}

impl From<ToolbarOrientation> for AriaOrientation {
    fn from(value: ToolbarOrientation) -> Self {
        match value {
            ToolbarOrientation::Horizontal => Self::Horizontal,
            ToolbarOrientation::Vertical => Self::Vertical,
        }
    }
}

/// Input parameters for the `use_toolbar` hook.
#[derive(Debug, Clone)]
pub struct UseToolbarInput {
    /// The label for the toolbar.
    pub label: Option<String>,

    /// The orientation of the toolbar.
    pub orientation: ToolbarOrientation,

    /// Whether the toolbar is disabled.
    pub is_disabled: Signal<bool>,

    /// Callback to navigate to the next item.
    pub on_focus_next: Option<Callback<()>>,

    /// Callback to navigate to the previous item.
    pub on_focus_previous: Option<Callback<()>>,

    /// Callback to navigate to the first item.
    pub on_focus_first: Option<Callback<()>>,

    /// Callback to navigate to the last item.
    pub on_focus_last: Option<Callback<()>>,
}

impl Default for UseToolbarInput {
    fn default() -> Self {
        Self {
            label: None,
            orientation: ToolbarOrientation::Horizontal,
            is_disabled: Signal::derive(|| false),
            on_focus_next: None,
            on_focus_previous: None,
            on_focus_first: None,
            on_focus_last: None,
        }
    }
}

/// The return value of the `use_toolbar` hook.
#[derive(Debug)]
pub struct UseToolbarReturn {
    /// Props for the toolbar element.
    pub toolbar_props: UseToolbarProps,

    /// The ID of the toolbar.
    pub toolbar_id: String,

    /// The orientation.
    pub orientation: ToolbarOrientation,
}

/// Props from `use_toolbar` that can be extracted and merged programmatically.
#[derive(Debug)]
pub struct UseToolbarProps {
    pub id: String,
    pub role: &'static str,
    pub aria_label: Option<String>,
    pub aria_orientation: AriaOrientation,
    pub aria_disabled: Signal<Option<AriaDisabled>>,
    pub on_keydown: EventHandler<KeyboardEvent>,
}

impl IntoAttrs for UseToolbarProps {
    type Attrs = UseToolbarAttrs;

    fn into_attrs(self) -> Self::Attrs {
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

/// Attributes for the toolbar element.
pub type UseToolbarAttrs = (
    Attr<attr::Id, String>,
    Attr<attr::Role, &'static str>,
    Attr<attr::AriaLabel, Option<String>>,
    Attr<attr::AriaOrientation, AriaOrientation>,
    Attr<attr::AriaDisabled, Signal<Option<AriaDisabled>>>,
    On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
);

/// Provides the behavior and accessibility for a toolbar.
///
/// A toolbar groups related controls (buttons, menus, etc.) together.
///
/// # Example
///
/// ```ignore
/// let toolbar = use_toolbar(UseToolbarInput {
///     label: Some("Formatting".to_string()),
///     ..Default::default()
/// });
///
/// view! {
///     <div {..toolbar.toolbar_props}>
///         <button>"Bold"</button>
///         <button>"Italic"</button>
///         <button>"Underline"</button>
///     </div>
/// }
/// ```
pub fn use_toolbar(input: UseToolbarInput) -> UseToolbarReturn {
    let UseToolbarInput {
        label,
        orientation,
        is_disabled: disabled,
        on_focus_next,
        on_focus_previous,
        on_focus_first,
        on_focus_last,
    } = input;

    let toolbar_id = format!("toolbar-{}", Uuid::new_v4());

    let aria_orientation = AriaOrientation::from(orientation);

    let aria_disabled = Signal::derive(move || disabled.get().then_some(AriaDisabled::True));

    let handle_keydown = move |e: KeyboardEvent| {
        if disabled.get_untracked() {
            return;
        }

        let key = e.key();
        let is_horizontal = orientation == ToolbarOrientation::Horizontal;

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

    UseToolbarReturn {
        toolbar_props: UseToolbarProps {
            id: toolbar_id.clone(),
            role: "toolbar",
            aria_label: label,
            aria_orientation,
            aria_disabled,
            on_keydown: EventHandler::new(handle_keydown),
        },
        toolbar_id,
        orientation,
    }
}
