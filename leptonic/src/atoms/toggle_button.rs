// Upstream: react-aria-components/src/ToggleButton.tsx @ 99e6102368
// Upstream: react-aria-components/src/ToggleButtonGroup.tsx @ 99e6102368
use std::collections::HashSet;

use leptos::{context::Provider, prelude::*};

use crate::{
    hooks::{
        IntoAttrs, Orientation, ToggleGroupSelectionMode, ToggleGroupState, ToggleState,
        UseButtonInput, UseToggleButtonGroupInput, UseToggleButtonGroupItemInput,
        UseToggleButtonInput, UseToggleGroupStateInput, UseToggleStateInput, UseToolbarInput,
        collections::Key, use_button, use_toggle_button, use_toggle_button_group,
        use_toggle_button_group_item, use_toggle_group_state, use_toggle_state,
    },
    utils::data_attributes::flag,
    utils::{classes::Classes, styles::Styles},
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - No controlled `isSelected`/`selectedKeys` (hook-owned state, project-wide convention): a
//   toggle button bound to app state takes a `state` (`ToggleState::from(rw_signal)`, or
//   `ToggleState::new`).
// - In a group, the button's key is `value` (react-aria: `id`, which is also the DOM id).
// - Render props become `data-*` attributes plus plain children.
//
// =============================================================================

/// Context from [`ToggleButtonGroup`] to its buttons.
#[derive(Debug, Clone, Copy)]
pub struct ToggleButtonGroupCtx {
    pub state: ToggleGroupState,
}

/// A headless toggle button (`aria-pressed`).
///
/// Data attributes: `data-selected`, `data-pressed`, `data-hovered`, `data-focused`,
/// `data-focus-visible`, `data-disabled`.
///
/// Inside a [`ToggleButtonGroup`], `value` is required and the group holds the selection
/// (`default_selected`, `on_change` and `state` don't apply).
#[allow(clippy::needless_pass_by_value)]
#[component]
pub fn ToggleButton(
    /// The button's key in its [`ToggleButtonGroup`].
    #[prop(into, optional)]
    value: Option<Key>,
    #[prop(optional)] default_selected: bool,
    /// Called when the button is selected or deselected.
    #[prop(into, optional)]
    on_change: Option<Callback<bool>>,
    /// External selection state, replacing `default_selected`. Bind a signal with
    /// `state=ToggleState::from(rw_signal)`.
    #[prop(into, optional)]
    state: Option<ToggleState>,
    #[prop(into, optional)] is_disabled: Signal<bool>,
    #[prop(into, optional)] aria_label: MaybeProp<String>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let button = UseButtonInput {
        aria_label,
        is_disabled,
        ..UseButtonInput::default()
    };
    let (input, is_selected) = if let Some(group) = use_context::<ToggleButtonGroupCtx>() {
        let value = value.expect("a <ToggleButton> in a <ToggleButtonGroup> needs a `value`");
        let selected_value = value.clone();
        let is_selected = Signal::derive(move || group.state.is_selected(&selected_value));
        let input = use_toggle_button_group_item(UseToggleButtonGroupItemInput {
            button,
            ..UseToggleButtonGroupItemInput::new(group.state, value)
        });
        (input, is_selected)
    } else {
        let state = if let Some(state) = state {
            on_change.map_or(state, |on_change| state.with_on_change(on_change))
        } else {
            use_toggle_state(UseToggleStateInput {
                default_selected,
                on_change,
                ..UseToggleStateInput::default()
            })
        };
        let input = use_toggle_button(UseToggleButtonInput { state, button });
        (input, state.is_selected)
    };
    let is_disabled = input.is_disabled;
    let button = use_button(input);
    let (attrs, button_styles) = button.props.into_parts();

    view! {
        <button
            {..attrs}
            class=classes
            style=button_styles.merge(styles)
            data-selected=flag(is_selected)
            data-pressed=flag(button.is_pressed)
            data-hovered=flag(button.is_hovered)
            data-focused=flag(button.is_focused)
            data-disabled=flag(is_disabled)
        >
            {children()}
        </button>
    }
}

/// A headless group of [`ToggleButton`]s: a toolbar (the arrow keys move focus), or a
/// `radiogroup` when only one button can be selected.
///
/// Data attributes: `data-orientation`, `data-disabled`.
// The keys are the group state's `HashSet<Key>`; a component prop can't be generic over hashers.
#[allow(clippy::too_many_arguments, clippy::implicit_hasher)]
#[component]
pub fn ToggleButtonGroup(
    #[prop(optional)] selection_mode: ToggleGroupSelectionMode,
    /// Whether the last selected button can't be deselected.
    #[prop(optional)]
    disallow_empty_selection: bool,
    #[prop(into, optional)] default_selected_keys: HashSet<Key>,
    #[prop(into, optional)] on_selection_change: Option<Callback<HashSet<Key>>>,
    #[prop(into, optional)] is_disabled: Signal<bool>,
    /// The axis of the arrow keys (react-aria's default: horizontal).
    #[prop(default = Orientation::Horizontal)]
    orientation: Orientation,
    #[prop(into, optional)] aria_label: MaybeProp<String>,
    #[prop(into, optional)] aria_labelledby: Option<String>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let state = use_toggle_group_state(UseToggleGroupStateInput {
        selection_mode,
        disallow_empty_selection,
        default_selected_keys,
        on_selection_change,
        is_disabled,
    });
    let group = use_toggle_button_group(UseToggleButtonGroupInput {
        state,
        toolbar: UseToolbarInput {
            orientation,
            aria_label,
            aria_labelledby,
        },
    });

    view! {
        <Provider value=ToggleButtonGroupCtx { state }>
            <div
                {..group.props.into_attrs()}
                class=classes
                style=styles
                data-orientation=orientation.as_str()
                data-disabled=flag(state.is_disabled)
            >
                {children()}
            </div>
        </Provider>
    }
}
