// Upstream: react-aria-components/src/ToggleButton.tsx @ 99e6102368
// Upstream: react-aria-components/src/ToggleButtonGroup.tsx @ 99e6102368
use std::collections::HashSet;

use leptos::{context::Provider, prelude::*};
use leptos_classes::Classes;
use web_sys::FocusEvent;

use super::typed_values::{KeyedStateProps, keyed_state_props};
use crate::{
    CapturedElement, IntoAttrs, Orientation, Out,
    hooks::{
        button::{
            ToggleGroupSelectionMode, ToggleGroupState, UseButtonInput, UseToggleButtonGroupInput,
            UseToggleButtonGroupItemInput, UseToggleButtonInput, UseToggleGroupStateInput,
            use_button, use_toggle_button, use_toggle_button_group, use_toggle_button_group_item,
            use_toggle_group_state,
        },
        collections::{Key, SelectionValue},
        form::{UseToggleStateInput, use_toggle_state},
        interactions::{HoverEndEvent, HoverStartEvent, KeyboardEventWrapper, PressEvent},
        toolbar::UseToolbarInput,
    },
    utils::{
        aria::{AriaExpanded, AriaHasPopup},
        data_attributes::flag,
        default_class::with_default_class,
        styles::Styles,
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - State (C4): `default_selected` + `on_change`, or `is_selected` + `set_selected`; the group's
//   typed value (`V: SelectionValue`, the set of the selected buttons' values) as `default_value`
//   + `on_change`, or `value` + `set_value`, as every value-like selection (react-aria:
//   `isSelected`/`selectedKeys` + `onChange`/`onSelectionChange`).
// - In a group, the button's key is `value` (react-aria: `id`, which is also the DOM id).
// - Like react-aria's toggle buttons, no form attributes (`name`, `value`, `form*`): a toggle
//   button doesn't submit a value; use a checkbox or switch for form data.
// - Render props become `data-*` attributes plus plain children.
//
// =============================================================================

/// Context from [`ToggleButtonGroup`] to its buttons.
#[derive(Debug, Clone, Copy)]
pub struct ToggleButtonGroupContext {
    pub state: ToggleGroupState,
}

/// A headless toggle button (`aria-pressed`).
///
/// Data attributes: `data-selected`, `data-pressed`, `data-hovered`, `data-focused`,
/// `data-focus-visible`, `data-disabled`.
///
/// Inside a [`ToggleButtonGroup`], give it a `value`: the group then holds the selection
/// (`default_selected`, `is_selected`, `set_selected` and `on_change` don't apply). Without a
/// `value` it is a standalone toggle there too (and warns in debug builds).
///
/// Default class: `leptonic-ToggleButton`.
#[allow(clippy::needless_pass_by_value, clippy::too_many_lines)]
#[component]
pub fn ToggleButton(
    /// The button's key in its [`ToggleButtonGroup`].
    #[prop(into, optional)]
    value: Option<Key>,
    #[prop(optional)] default_selected: bool,
    /// Called when the button is selected or deselected.
    #[prop(into, optional)]
    on_change: Option<Callback<bool>>,
    /// Whether the toggle is selected (controlled): a value or any signal.
    #[prop(into, optional)]
    is_selected: Option<Signal<bool>>,
    /// Receives the selection: an `RwSignal`, `WriteSignal`, closure, `Callback`, ...
    #[prop(into, optional)]
    set_selected: Option<Out<bool>>,
    #[prop(into, optional)] is_disabled: Signal<bool>,
    #[prop(into, optional)] on_press: Option<Callback<PressEvent>>,
    #[prop(into, optional)] on_press_start: Option<Callback<PressEvent>>,
    #[prop(into, optional)] on_press_end: Option<Callback<PressEvent>>,
    #[prop(into, optional)] on_press_up: Option<Callback<PressEvent>>,
    #[prop(into, optional)] on_press_change: Option<Callback<bool>>,
    #[prop(into, optional)] on_hover_start: Option<Callback<HoverStartEvent>>,
    #[prop(into, optional)] on_hover_end: Option<Callback<HoverEndEvent>>,
    #[prop(into, optional)] on_hover_change: Option<Callback<bool>>,
    #[prop(into, optional)] on_focus: Option<Callback<FocusEvent>>,
    #[prop(into, optional)] on_blur: Option<Callback<FocusEvent>>,
    #[prop(into, optional)] on_focus_change: Option<Callback<bool>>,
    #[prop(into, optional)] on_key_down: Option<Callback<KeyboardEventWrapper>>,
    #[prop(into, optional)] on_key_up: Option<Callback<KeyboardEventWrapper>>,
    /// Focus the button when it mounts.
    #[prop(optional)]
    auto_focus: bool,
    /// Don't move focus to the button when it is pressed.
    #[prop(into, optional)]
    prevent_focus_on_press: Signal<bool>,
    #[prop(into, optional)] exclude_from_tab_order: Signal<bool>,
    #[prop(into, optional)] aria_label: MaybeProp<String>,
    #[prop(into, optional)] aria_labelledby: Option<String>,
    #[prop(into, optional)] aria_describedby: Signal<Option<String>>,
    #[prop(into, optional)] aria_controls: Signal<Option<String>>,
    #[prop(into, optional)] aria_expanded: Signal<Option<AriaExpanded>>,
    #[prop(into, optional)] aria_haspopup: Signal<Option<AriaHasPopup>>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let classes = with_default_class("leptonic-ToggleButton", classes);
    let button = UseButtonInput {
        aria_label,
        aria_labelledby: Signal::stored(aria_labelledby),
        aria_describedby,
        aria_controls,
        aria_expanded,
        aria_haspopup,
        is_disabled,
        auto_focus,
        prevent_focus_on_press,
        exclude_from_tab_order,
        on_press,
        on_press_start,
        on_press_end,
        on_press_up,
        on_press_change,
        on_hover_start,
        on_hover_end,
        on_hover_change,
        on_focus,
        on_blur,
        on_focus_change,
        on_key_down,
        on_key_up,
        ..UseButtonInput::default()
    };
    let group = use_context::<ToggleButtonGroupContext>();
    if group.is_some() && value.is_none() {
        crate::utils::dev_warn!(
            "A <ToggleButton> in a <ToggleButtonGroup> needs a `value`; without one it toggles on \
             its own."
        );
    }
    let (input, is_selected) = if let (Some(group), Some(value)) = (group, value) {
        let selected_value = value.clone();
        let is_selected = Signal::derive(move || group.state.is_selected(&selected_value));
        let input = use_toggle_button_group_item(UseToggleButtonGroupItemInput {
            button,
            group: group.state,
            key: value,
        });
        (input, is_selected)
    } else {
        let (value, on_change) =
            crate::ValueBinding::from_state_props(is_selected, set_selected, on_change);
        let state = use_toggle_state(UseToggleStateInput {
            default_selected,
            value,
            on_change,
            ..UseToggleStateInput::default()
        });
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
///
/// The value is the set of the selected buttons' values (typed, `V: SelectionValue`; the buttons
/// take theirs as their `value`, a `Key`).
// A component prop can't be generic over hashers.
/// Default class: `leptonic-ToggleButtonGroup`.
#[allow(clippy::too_many_arguments, clippy::implicit_hasher)]
#[component]
pub fn ToggleButtonGroup<V: SelectionValue>(
    #[prop(optional)] selection_mode: ToggleGroupSelectionMode,
    /// Whether the last selected button can't be deselected.
    #[prop(into, optional)]
    disallow_empty_selection: Signal<bool>,
    /// The initially selected buttons. Ignored with `value`.
    #[prop(optional)]
    default_value: HashSet<V>,
    /// The selected buttons (controlled): a value or any signal.
    #[prop(into, optional)]
    value: Option<Signal<HashSet<V>>>,
    /// Receives the new state: an `RwSignal`, `WriteSignal`, closure, `Callback`, ...
    #[prop(into, optional)]
    set_value: Option<Out<HashSet<V>>>,
    #[prop(into, optional)] on_change: Option<Callback<HashSet<V>>>,
    #[prop(into, optional)] is_disabled: Signal<bool>,
    /// The axis of the arrow keys (react-aria's default: horizontal).
    #[prop(into, default = Orientation::Horizontal.into())]
    orientation: Signal<Orientation>,
    #[prop(into, optional)] aria_label: MaybeProp<String>,
    #[prop(into, optional)] aria_labelledby: Option<String>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let classes = with_default_class("leptonic-ToggleButtonGroup", classes);
    let KeyedStateProps {
        default_value,
        value,
        set_value,
        on_change,
        ..
    } = keyed_state_props(Some(default_value), value, set_value, on_change, None);
    let (selected_keys, on_selection_change) =
        crate::ValueBinding::from_state_props(value, set_value, on_change);
    let state = use_toggle_group_state(UseToggleGroupStateInput {
        selection_mode,
        disallow_empty_selection,
        default_selected_keys: default_value.unwrap_or_default(),
        selected_keys,
        on_selection_change,
        is_disabled,
    });
    let group = use_toggle_button_group(UseToggleButtonGroupInput {
        state,
        toolbar: UseToolbarInput {
            element: CapturedElement::new(),
            orientation,
            aria_label,
            aria_labelledby,
        },
    });

    view! {
        <Provider value=ToggleButtonGroupContext { state }>
            <div
                {..group.props.into_attrs()}
                class=classes
                style=styles
                data-orientation=move || orientation.get().as_str()
                data-disabled=flag(state.is_disabled)
            >
                {children()}
            </div>
        </Provider>
    }
}
