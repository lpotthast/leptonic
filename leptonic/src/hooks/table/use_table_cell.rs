use leptos::attr;
use leptos::attr::Attr;
use leptos::ev;
use leptos::ev::{On, SharedEventCallback};
use leptos::prelude::*;
use web_sys::{FocusEvent, KeyboardEvent};

use crate::hooks::focus::use_focus_ring::{use_focus_ring, UseFocusRingInput, UseFocusRingReturn};
use crate::utils::aria::AriaDisabled;
use crate::utils::EventHandler;

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/table/src/useTableCell.ts

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// No intentional deviations from the react-aria implementation.
//
// =============================================================================

/// Input parameters for the `use_table_cell` hook.
#[derive(Debug, Clone)]
pub struct UseTableCellInput {
    /// The column index (for aria-colindex).
    pub column_index: usize,

    /// Whether the cell is focused.
    pub is_focused: Signal<bool>,

    /// Whether the cell is disabled.
    pub is_disabled: Signal<bool>,

    /// Whether the cell contains an interactive element.
    pub is_interactive: bool,

    /// Callback when the cell receives focus.
    pub on_focus: Option<Callback<()>>,

    /// Callback when navigating to the next cell.
    pub on_focus_next: Option<Callback<()>>,

    /// Callback when navigating to the previous cell.
    pub on_focus_previous: Option<Callback<()>>,
}

impl Default for UseTableCellInput {
    fn default() -> Self {
        Self {
            column_index: 0,
            is_focused: Signal::derive(|| false),
            is_disabled: Signal::derive(|| false),
            is_interactive: false,
            on_focus: None,
            on_focus_next: None,
            on_focus_previous: None,
        }
    }
}

/// The return value of the `use_table_cell` hook.
pub struct UseTableCellReturn {
    /// Props for programmatic merging. Call `.into_attrs()` for view spreading.
    pub cell_props: UseTableCellProps,

    /// Whether the cell is focused.
    pub is_focused: Signal<bool>,

    /// Whether the focus ring should be visible (keyboard navigation only).
    pub is_focus_visible: Signal<bool>,
}

/// Props from `use_table_cell` that can be extracted and merged programmatically.
#[derive(Debug)]
pub struct UseTableCellProps {
    pub role: &'static str,
    pub aria_colindex: String,
    pub tabindex: Signal<&'static str>,
    pub aria_disabled: Signal<Option<AriaDisabled>>,
    pub on_keydown: EventHandler<KeyboardEvent>,
    pub on_focus: EventHandler<FocusEvent>,
    pub on_blur: EventHandler<FocusEvent>,
    pub on_focusin: EventHandler<FocusEvent>,
    pub on_focusout: EventHandler<FocusEvent>,
    pub data_focus_visible: Signal<Option<&'static str>>,
}

impl UseTableCellProps {
    /// Convert to spreadable attributes for Leptos views, consuming self.
    #[must_use]
    pub fn into_attrs(self) -> UseTableCellAttrs {
        (
            Attr(attr::Role, self.role),
            Attr(attr::AriaColindex, self.aria_colindex),
            Attr(attr::Tabindex, self.tabindex),
            Attr(attr::AriaDisabled, self.aria_disabled),
            self.on_keydown.into_on(ev::keydown),
            self.on_focus.into_on(ev::focus),
            self.on_blur.into_on(ev::blur),
            self.on_focusin.into_on(ev::focusin),
            self.on_focusout.into_on(ev::focusout),
            attr::custom::custom_attribute("data-focus-visible", self.data_focus_visible),
        )
    }
}

/// Attributes for the table cell element.
pub type UseTableCellAttrs = (
    Attr<attr::Role, &'static str>,
    Attr<attr::AriaColindex, String>,
    Attr<attr::Tabindex, Signal<&'static str>>,
    Attr<attr::AriaDisabled, Signal<Option<AriaDisabled>>>,
    On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
    On<ev::focus, SharedEventCallback<FocusEvent>>,
    On<ev::blur, SharedEventCallback<FocusEvent>>,
    On<ev::focusin, SharedEventCallback<FocusEvent>>,
    On<ev::focusout, SharedEventCallback<FocusEvent>>,
    attr::custom::CustomAttr<&'static str, Signal<Option<&'static str>>>,
);

/// Provides the behavior and accessibility for a table cell.
///
/// A table cell displays data within a row and column intersection.
///
/// # Example
///
/// ```ignore
/// let cell = use_table_cell(UseTableCellInput {
///     column_index: col_idx,
///     is_focused: is_cell_focused.into(),
///     is_disabled: Signal::derive(|| false),
///     ..Default::default()
/// });
///
/// view! {
///     <td {..cell.cell_props.into_attrs()}>
///         {data}
///     </td>
/// }
/// ```
#[allow(clippy::needless_pass_by_value)]
pub fn use_table_cell(input: UseTableCellInput) -> UseTableCellReturn {
    let UseTableCellInput {
        column_index,
        is_focused,
        is_disabled: disabled,
        is_interactive,
        on_focus,
        on_focus_next,
        on_focus_previous,
    } = input;

    // Compute tabindex
    let tabindex = Signal::derive(move || if is_focused.get() { "0" } else { "-1" });

    // Compute aria-disabled
    let aria_disabled = Signal::derive(move || disabled.get().then_some(AriaDisabled::True));

    // Handle keyboard navigation
    let handle_keydown = move |e: KeyboardEvent| {
        if disabled.get_untracked() {
            return;
        }

        // If the cell contains interactive content, let it handle events
        if is_interactive {
            return;
        }

        let key = e.key();
        match key.as_str() {
            "ArrowRight" => {
                e.prevent_default();
                if let Some(on_next) = on_focus_next {
                    on_next.run(());
                }
            }
            "ArrowLeft" => {
                e.prevent_default();
                if let Some(on_prev) = on_focus_previous {
                    on_prev.run(());
                }
            }
            _ => {}
        }
    };

    // Use focus ring for keyboard focus visibility with user callback
    let UseFocusRingReturn {
        props: focus_ring_props,
        is_focus_visible,
        is_focused: _,
    } = use_focus_ring(UseFocusRingInput {
        disabled,
        within: false,
        auto_focus: false,
        on_focus: on_focus.map(|cb| Callback::new(move |_| cb.run(()))),
        on_blur: None,
        on_focus_change: None,
    });

    // Column index is 1-based for ARIA
    let aria_colindex = (column_index + 1).to_string();

    UseTableCellReturn {
        cell_props: UseTableCellProps {
            role: "gridcell",
            aria_colindex,
            tabindex,
            aria_disabled,
            on_keydown: EventHandler::new(handle_keydown),
            on_focus: focus_ring_props.on_focus,
            on_blur: focus_ring_props.on_blur,
            on_focusin: focus_ring_props.on_focusin,
            on_focusout: focus_ring_props.on_focusout,
            data_focus_visible: focus_ring_props.data_focus_visible,
        },
        is_focused,
        is_focus_visible,
    }
}
