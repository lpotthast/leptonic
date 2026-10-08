// Upstream: react-aria/src/button/useToggleButtonGroup.ts @ 99e6102368
use leptos::{attr, attr::Attr, prelude::*};

use super::{
    use_button::UseButtonInput,
    use_toggle_button::{UseToggleButtonInput, use_toggle_button},
    use_toggle_group_state::{ToggleGroupSelectionMode, ToggleGroupState},
};
use crate::{
    hooks::{
        IntoAttrs, UseToolbarAttrs, UseToolbarInput, UseToolbarProps, collections::Key,
        form::ToggleState, use_toolbar,
    },
    utils::aria::{AriaChecked, AriaDisabled, AriaRole},
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

/// Output of [`use_toggle_button_group`].
#[derive(Debug)]
pub struct UseToggleButtonGroupReturn {
    /// Props for the group element.
    pub props: UseToggleButtonGroupProps,
}

/// Props for the toggle button group element.
#[derive(Debug)]
pub struct UseToggleButtonGroupProps {
    pub toolbar: UseToolbarProps,
    pub aria_disabled: Signal<Option<AriaDisabled>>,
}

pub type UseToggleButtonGroupAttrs = (
    UseToolbarAttrs,
    Attr<attr::AriaDisabled, Signal<Option<AriaDisabled>>>,
);

impl IntoAttrs for UseToggleButtonGroupProps {
    type Attrs = UseToggleButtonGroupAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            self.toolbar.into_attrs(),
            Attr(attr::AriaDisabled, self.aria_disabled),
        )
    }
}

/// Provides the behavior and accessibility of a group of toggle buttons: a toolbar (arrow key
/// navigation), or a `radiogroup` when only one button can be selected. Render the buttons
/// with [`use_toggle_button_group_item`].
pub fn use_toggle_button_group(input: UseToggleButtonGroupInput) -> UseToggleButtonGroupReturn {
    let UseToggleButtonGroupInput { state, toolbar } = input;
    let mut toolbar = use_toolbar(toolbar).props;
    if state.selection_mode == ToggleGroupSelectionMode::Single {
        toolbar.role = Signal::stored(AriaRole::Radiogroup);
    }
    let is_disabled = state.is_disabled;
    UseToggleButtonGroupReturn {
        props: UseToggleButtonGroupProps {
            toolbar,
            aria_disabled: Signal::derive(move || is_disabled.get().then_some(AriaDisabled::True)),
        },
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
    let UseToggleButtonGroupItemInput {
        group,
        key,
        mut button,
    } = input;
    let selected_key = key.clone();
    let is_selected = Signal::derive(move || group.is_selected(&selected_key));
    let state = ToggleState::new(
        is_selected,
        false,
        Callback::new(move |selected: bool| group.set_selected(&key, selected)),
    );
    let is_disabled = button.is_disabled;
    button.is_disabled = Signal::derive(move || is_disabled.get() || group.is_disabled.get());
    let mut button = use_toggle_button(UseToggleButtonInput { state, button });
    if group.selection_mode == ToggleGroupSelectionMode::Single {
        button.role = Some(AriaRole::Radio);
        button.aria_checked = Signal::derive(move || Some(AriaChecked::from(is_selected.get())));
        button.aria_pressed = Signal::stored(None);
    }
    button
}
