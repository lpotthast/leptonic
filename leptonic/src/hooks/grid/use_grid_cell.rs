use crate::utils::EventHandler;
use leptos::attr::Attr;
use leptos::ev::{On, SharedEventCallback};
use leptos::{attr, ev};
use reactive_graph::callback::{Callable, Callback};
use reactive_graph::prelude::{Get, GetUntracked};
use reactive_graph::wrappers::read::Signal;
use web_sys::{FocusEvent, KeyboardEvent, MouseEvent};

/// Input for a grid cell.
#[derive(Debug, Clone)]
pub struct UseGridCellInput {
    /// The unique key for this cell.
    pub cell_key: String,

    /// The row index.
    pub row_index: usize,

    /// The column index.
    pub column_index: usize,

    /// Whether the cell is selected.
    pub is_selected: Signal<bool>,

    /// Whether the cell is focused.
    pub is_focused: Signal<bool>,

    /// Whether the cell is disabled.
    pub is_disabled: Signal<bool>,

    /// Callback when navigating (direction: "up", "down", "left", "right").
    pub on_navigate: Option<Callback<&'static str>>,

    /// Callback when the cell is activated.
    pub on_action: Option<Callback<()>>,

    /// Callback when selection changes.
    pub on_selection_change: Option<Callback<bool>>,
}

/// Return value for a grid cell.
#[derive(Debug, Clone)]
pub struct UseGridCellReturn {
    /// Props for the cell element.
    pub cell_props: UseGridCellProps,

    /// The cell key.
    pub cell_key: String,

    /// Whether the cell is selected.
    pub is_selected: Signal<bool>,

    /// Whether the cell is focused.
    pub is_focused: Signal<bool>,
}

#[derive(Debug, Clone)]
pub struct UseGridCellProps {
    pub role: &'static str,
    pub tabindex: Signal<&'static str>,
    pub aria_rowindex: String,
    pub aria_colindex: String,
    pub aria_selected: Signal<Option<&'static str>>,
    pub aria_disabled: Signal<bool>,

    pub on_click: EventHandler<MouseEvent>,
    pub on_keydown: EventHandler<KeyboardEvent>,
    pub on_focus: EventHandler<FocusEvent>,
}

impl UseGridCellProps {
    pub fn into_attrs(self) -> UseGridCellAttrs {
        (
            Attr(attr::Role, self.role),
            Attr(attr::Tabindex, self.tabindex),
            Attr(attr::AriaRowindex, self.aria_rowindex),
            Attr(attr::AriaColindex, self.aria_colindex),
            Attr(attr::AriaSelected, self.aria_selected),
            Attr(attr::AriaDisabled, self.aria_disabled),
            self.on_click.into_on(ev::click),
            self.on_keydown.into_on(ev::keydown),
            self.on_focus.into_on(ev::focus),
        )
    }
}

/// Attributes for a grid cell.
pub type UseGridCellAttrs = (
    Attr<attr::Role, &'static str>,
    Attr<attr::Tabindex, Signal<&'static str>>,
    Attr<attr::AriaRowindex, String>,
    Attr<attr::AriaColindex, String>,
    Attr<attr::AriaSelected, Signal<Option<&'static str>>>,
    Attr<attr::AriaDisabled, Signal<bool>>,
    On<ev::click, SharedEventCallback<MouseEvent>>,
    On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
    On<ev::focus, SharedEventCallback<FocusEvent>>,
);

/// Provides the behavior and accessibility for a grid cell.
#[allow(clippy::needless_pass_by_value)]
pub fn use_grid_cell(input: UseGridCellInput) -> UseGridCellReturn {
    let cell_key = input.cell_key.clone();
    let is_selected = input.is_selected;
    let is_focused = input.is_focused;
    let is_disabled = input.is_disabled;
    let on_navigate = input.on_navigate;
    let on_action = input.on_action;
    let on_selection_change = input.on_selection_change;

    // Compute aria-selected (None if selection not supported)
    let aria_selected = Signal::derive(move || {
        if on_selection_change.is_some() {
            if is_selected.get() {
                Some("true")
            } else {
                Some("false")
            }
        } else {
            None
        }
    });

    // Compute tabindex
    let tabindex = Signal::derive(move || if is_focused.get() { "0" } else { "-1" });

    // Handle click
    let handle_click = move |_e: MouseEvent| {
        if is_disabled.get_untracked() {
            return;
        }
        if let Some(on_selection_change) = on_selection_change {
            on_selection_change.run(!is_selected.get_untracked());
        }
    };

    // Handle keyboard
    let handle_keydown = move |e: KeyboardEvent| {
        if is_disabled.get_untracked() {
            return;
        }

        let key = e.key();
        match key.as_str() {
            "Enter" => {
                e.prevent_default();
                if let Some(on_action) = on_action {
                    on_action.run(());
                }
            }
            " " => {
                e.prevent_default();
                if let Some(on_selection_change) = on_selection_change {
                    on_selection_change.run(!is_selected.get_untracked());
                }
            }
            "ArrowUp" => {
                e.prevent_default();
                if let Some(on_nav) = on_navigate {
                    on_nav.run("up");
                }
            }
            "ArrowDown" => {
                e.prevent_default();
                if let Some(on_nav) = on_navigate {
                    on_nav.run("down");
                }
            }
            "ArrowLeft" => {
                e.prevent_default();
                if let Some(on_nav) = on_navigate {
                    on_nav.run("left");
                }
            }
            "ArrowRight" => {
                e.prevent_default();
                if let Some(on_nav) = on_navigate {
                    on_nav.run("right");
                }
            }
            _ => {}
        }
    };

    // Handle focus
    let handle_focus = move |_e: FocusEvent| {
        // Focus is managed by parent
    };

    // Indices are 1-based for ARIA
    let aria_rowindex = (input.row_index + 1).to_string();
    let aria_colindex = (input.column_index + 1).to_string();

    UseGridCellReturn {
        cell_props: UseGridCellProps {
            role: "gridcell",
            tabindex,
            aria_rowindex,
            aria_colindex,
            aria_selected,
            aria_disabled: is_disabled,
            on_click: EventHandler::new(handle_click),
            on_keydown: EventHandler::new(handle_keydown),
            on_focus: EventHandler::new(handle_focus),
        },
        cell_key,
        is_selected,
        is_focused,
    }
}
