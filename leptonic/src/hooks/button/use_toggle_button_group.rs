// Upstream: react-aria/src/button/useToggleButtonGroup.ts @ 99e6102368
// Upstream: react-aria-components/test/ToggleButtonGroup.test.js @ 99e6102368
use leptos::prelude::*;

use super::{
    use_button::UseButtonInput,
    use_toggle_button::{UseToggleButtonInput, use_toggle_button},
    use_toggle_group_state::{ToggleGroupSelectionMode, ToggleGroupState},
};
use crate::{
    IntoAttrs, ValueBinding,
    hooks::{
        collections::Key,
        form::{UseToggleStateInput, use_toggle_state},
        toolbar::{UseToolbarAttrs, UseToolbarInput, UseToolbarProps, use_toolbar},
    },
    utils::{
        aria::{AriaChecked, AriaDisabled, AriaRole, aria_true},
        part::{Part, Remount},
        probe::either,
        signal_attr::{SignalAttr, signal_attr},
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - `use_toggle_button_group_item` returns the `UseButtonInput` for `use_button` (the button's
//   own hook), as composite hooks do here (project-wide convention).
//
// =============================================================================

/// Input of [`use_toggle_button_group`].
#[derive(Debug, Clone)]
pub struct UseToggleButtonGroupInput {
    pub state: ToggleGroupState,
    /// The toolbar settings (orientation, label).
    pub toolbar: UseToolbarInput,
}

/// The return value of [`use_toggle_button_group`].
#[derive(Debug)]
pub struct UseToggleButtonGroupReturn {
    /// The group element.
    pub group: Part<UseToggleButtonGroupProps>,
}

/// Props for the toggle button group element.
#[derive(Debug)]
pub struct UseToggleButtonGroupProps {
    pub toolbar: UseToolbarProps,
    pub aria_disabled: Signal<Option<AriaDisabled>>,
}

impl Remount for UseToggleButtonGroupProps {
    fn remount(&self) -> Self {
        Self {
            toolbar: self.toolbar.remount(),
            aria_disabled: self.aria_disabled,
        }
    }
}

pub type UseToggleButtonGroupAttrs = (UseToolbarAttrs, SignalAttr<Option<AriaDisabled>>);

impl IntoAttrs for UseToggleButtonGroupProps {
    type Attrs = UseToggleButtonGroupAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            self.toolbar.into_attrs(),
            signal_attr("aria-disabled", self.aria_disabled),
        )
    }
}

/// Provides the behavior and accessibility of a group of toggle buttons: a toolbar (arrow key
/// navigation), or a `radiogroup` when only one button can be selected. Render the buttons
/// with [`use_toggle_button_group_item`].
pub fn use_toggle_button_group(input: UseToggleButtonGroupInput) -> UseToggleButtonGroupReturn {
    crate::metrics::subject!();
    let UseToggleButtonGroupInput { state, toolbar } = input;
    let mut toolbar = use_toolbar(toolbar).toolbar.into_props();
    if state.selection_mode == ToggleGroupSelectionMode::Single {
        toolbar.role = Signal::stored(AriaRole::Radiogroup);
    }
    let is_disabled = state.is_disabled;
    UseToggleButtonGroupReturn {
        group: Part::stored(UseToggleButtonGroupProps {
            toolbar,
            aria_disabled: aria_true(is_disabled),
        }),
    }
}

/// Input of [`use_toggle_button_group_item`].
#[derive(Debug, Clone)]
pub struct UseToggleButtonGroupItemInput {
    pub group: ToggleGroupState,
    /// The button's key in the group.
    pub key: Key,
    /// The button's further settings.
    pub button: UseButtonInput,
}

/// A toggle button in a [`use_toggle_button_group`]: selects or deselects its key. In a
/// single-selection group, it is a `radio` with `aria-checked`. Returns the input of
/// [`use_button`](fn@super::use_button).
pub fn use_toggle_button_group_item(input: UseToggleButtonGroupItemInput) -> UseButtonInput {
    crate::metrics::subject!();
    let UseToggleButtonGroupItemInput {
        group,
        key,
        mut button,
    } = input;
    // Read by the toggle state and the ARIA attributes; notifies only when this key's selection
    // changes (`performance.md` R1).
    let selected_key = key.clone();
    let is_selected = Signal::derive(move || group.is_selected(&selected_key));
    let state = use_toggle_state(UseToggleStateInput {
        value: Some(ValueBinding::new(
            is_selected,
            Callback::new(move |selected: bool| group.set_selected(&key, selected)),
        )),
        ..UseToggleStateInput::default()
    });
    button.is_disabled = either(button.is_disabled, group.is_disabled);
    let mut button = use_toggle_button(UseToggleButtonInput { state, button });
    if group.selection_mode == ToggleGroupSelectionMode::Single {
        button.role = Some(AriaRole::Radio);
        button.aria_checked = Signal::derive(move || Some(AriaChecked::from(is_selected.get())));
        button.aria_pressed = Signal::stored(None);
    }
    button
}
