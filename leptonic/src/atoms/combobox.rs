use std::collections::HashSet;

use leptos::{context::Provider, prelude::*};

use super::{
    field::{FieldContext, FieldLabelProps},
    form::use_validation_behavior,
    input::{InputContext, InputState},
    listbox::ListBoxParent,
    popover::{PopoverDialogLabel, PopoverParts, render_popover},
};
use crate::{
    hooks::{
        ComboBoxFilter, ComboBoxMenuTrigger, ComboBoxState, ComboBoxValue, IntoAttrs, PlacementX,
        PlacementY, PopoverModality, SelectMode, UseButtonInput, UseComboBoxInput,
        UseComboBoxReturn, UseComboBoxStateInput, UsePopoverInput, UsePopoverReturn,
        UseTextFieldReturn, ValidateFn, ValidationBehavior,
        collections::{CollectionMemo, Key},
        use_button, use_combobox, use_combobox_state, use_popover, use_text_field,
    },
    utils::ValueBinding,
    utils::data_attributes::flag,
    utils::{CapturedElement, classes::Classes, styles::Styles},
};

/// Context from [`ComboBox`] to its parts.
#[derive(Clone)]
pub struct ComboBoxCtx {
    pub state: ComboBoxState,
    pub is_disabled: Signal<bool>,
    /// The element the popover is positioned at (the input).
    pub anchor: CapturedElement,
    /// The popover element.
    pub popover: CapturedElement,
    parts: StoredValue<Parts>,
}

/// The `use_combobox` and `use_text_field` outputs for the parts.
#[derive(Clone)]
struct Parts {
    button: UseButtonInput,
}

/// A headless combo box: a text input with a popover of suggestions to pick from.
///
/// Compose it from a [`Label`](super::field::Label), an [`Input`](super::input::Input), a
/// [`ComboBoxButton`],
/// [`ComboBoxPopover`] (containing a [`ListBox`](super::listbox::ListBox) with one
/// `ListBoxItem` per option), a [`Description`](super::field::Description) and a
/// [`FieldError`](super::field::FieldError).
#[component]
#[allow(
    clippy::too_many_lines,
    clippy::fn_params_excessive_bools,
    clippy::implicit_hasher
)]
pub fn ComboBox(
    /// All options.
    #[prop(into)]
    collection: CollectionMemo,
    /// Shows the options matching the input (e.g. [`use_contains_filter`](crate::hooks::use_contains_filter)).
    /// Without a filter, all options are shown.
    #[prop(optional)]
    filter: Option<ComboBoxFilter>,
    #[prop(optional)] selection_mode: SelectMode,
    /// The initially selected keys (at most one in `Single` mode). Ignored when `value` is bound.
    #[prop(into, optional)]
    default_value: Vec<Key>,
    /// The selected keys as app state (e.g. an `RwSignal<Vec<Key>>`), replacing `default_value`.
    #[prop(into, optional)]
    value: Option<ValueBinding<Vec<Key>>>,
    #[prop(into, optional)] on_change: Option<Callback<Vec<Key>>>,
    /// The initial input text. Default: the selected option's text. Ignored when `input_value` is
    /// bound.
    #[prop(into, optional)]
    default_input_value: Option<String>,
    /// The input text as app state (e.g. an `RwSignal<String>`), replacing `default_input_value`.
    #[prop(into, optional)]
    input_value: Option<ValueBinding<String>>,
    #[prop(into, optional)] on_input_change: Option<Callback<String>>,
    #[prop(into, optional)] disabled_keys: Option<Signal<HashSet<Key>>>,
    #[prop(optional)] menu_trigger: ComboBoxMenuTrigger,
    #[prop(optional)] allows_empty_collection: bool,
    #[prop(optional)] allows_custom_value: bool,
    #[prop(into, optional)] is_disabled: Signal<bool>,
    #[prop(into, optional)] is_read_only: Signal<bool>,
    #[prop(optional)] is_required: bool,
    /// Labels the combo box when there is no [`Label`](super::field::Label).
    #[prop(into, optional)]
    aria_label: MaybeProp<String>,
    #[prop(into, optional)] aria_labelledby: Option<String>,
    #[prop(into, optional)] placeholder: Option<String>,
    /// The form field name.
    #[prop(into, optional)]
    name: Option<String>,
    #[prop(into, optional)] is_invalid: Signal<bool>,
    #[prop(optional)] validate: Option<ValidateFn<ComboBoxValue>>,
    /// Default: the surrounding [`Form`](super::form::Form)'s, else `Native`.
    #[prop(optional)]
    validation_behavior: Option<ValidationBehavior>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let validation_behavior = use_validation_behavior(validation_behavior);
    let state = use_combobox_state(UseComboBoxStateInput {
        filter,
        selection_mode,
        default_value,
        value,
        on_change,
        default_input_value,
        input_value,
        on_input_change,
        disabled_keys: disabled_keys.unwrap_or_default(),
        menu_trigger,
        allows_empty_collection,
        allows_custom_value,
        is_read_only,
        is_invalid,
        validate,
        validation_behavior,
        name: name.clone(),
        ..UseComboBoxStateInput::new(collection)
    });

    let popover = CapturedElement::new();
    // As in react-aria-components: a visible label is expected unless an ARIA label is given.
    let has_label = aria_label.get_untracked().is_none() && aria_labelledby.is_none();
    let UseComboBoxReturn {
        input,
        input_props,
        button,
        listbox,
        ..
    } = use_combobox(UseComboBoxInput {
        is_disabled,
        is_read_only,
        is_required,
        has_label,
        aria_label,
        aria_labelledby,
        placeholder,
        name,
        popover,
        ..UseComboBoxInput::new(state)
    });
    let UseTextFieldReturn {
        label_props,
        input_props: field_props,
        description_props,
        error_message_props,
        is_focused,
        is_focus_visible,
        is_invalid,
        validation_errors,
        validation_details,
        ..
    } = use_text_field(input);

    let anchor = CapturedElement::new();
    let ctx = ComboBoxCtx {
        state,
        is_disabled,
        anchor,
        popover,
        parts: StoredValue::new(Parts { button }),
    };
    // The input: the text field's and the combo box's props, and the popover's anchor.
    let input = InputContext::new(
        move || {
            (
                field_props.clone().into_attrs(),
                input_props.clone().into_attrs(),
                anchor.attr(),
            )
        },
        InputState {
            is_disabled,
            is_invalid,
            is_focused,
            is_focus_visible,
        },
    );
    let field = FieldContext {
        label: FieldLabelProps::label(label_props),
        description: description_props,
        error_message: error_message_props,
        is_invalid,
        validation_errors,
        validation_details,
    };
    let listbox_parent = ListBoxParent {
        input: StoredValue::new(listbox),
    };
    let is_open = Signal::derive(move || state.is_open());

    view! {
        <Provider value=ctx>
            <Provider value=listbox_parent>
                <Provider value=field>
                    <Provider value=input>
                    <div
                        class=classes
                        style=styles
                        data-open=flag(is_open)
                        data-invalid=flag(state.validation.is_invalid)
                        data-disabled=flag(is_disabled)
                    >
                        {children()}
                    </div>
                    </Provider>
                </Provider>
            </Provider>
        </Provider>
    }
}

/// The button opening the popover (not in the tab order: the keyboard uses ArrowDown on the
/// input).
#[component]
pub fn ComboBoxButton(
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let ctx = expect_context::<ComboBoxCtx>();
    let button = use_button(ctx.parts.with_value(|p| p.button.clone()));
    let (attrs, button_styles) = button.props.into_parts();
    let styles = button_styles.merge(styles);
    let state = ctx.state;

    view! {
        <button
            {..attrs}
            class=classes
            style=styles
            data-open=flag(Signal::derive(move || state.is_open()))
            data-pressed=flag(button.is_pressed)
        >
            {children()}
        </button>
    }
}

/// The popover with the options' [`ListBox`](super::listbox::ListBox), positioned at the input
/// and mounted while open. It is non-modal (react-aria-components): focus stays in the input, and
/// the page stays usable; scrolling closes it.
#[component]
#[allow(clippy::needless_pass_by_value)]
pub fn ComboBoxPopover(
    #[prop(into, default = Signal::stored(PlacementX::Left))] placement_x: Signal<PlacementX>,
    #[prop(into, default = Signal::stored(PlacementY::Below))] placement_y: Signal<PlacementY>,
    /// The distance from the input, in pixels.
    #[prop(into, optional)]
    offset: Signal<f64>,
    #[prop(into, optional)] cross_offset: Signal<f64>,
    /// The minimum distance from the viewport edges, in pixels.
    #[prop(into, default = Signal::stored(12.0))]
    container_padding: Signal<f64>,
    /// Whether the popover flips above the input when there is no room below.
    #[prop(into, default = Signal::stored(true))]
    should_flip: Signal<bool>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: ChildrenFn,
) -> impl IntoView {
    let ctx = expect_context::<ComboBoxCtx>();
    let UsePopoverReturn {
        props,
        resolved_placement_x,
        resolved_placement_y,
        ..
    } = use_popover(UsePopoverInput {
        trigger: ctx.anchor,
        placement_x,
        placement_y,
        offset,
        cross_offset,
        container_padding,
        should_flip,
        modality: PopoverModality::NonModal,
        ..UsePopoverInput::new(ctx.state)
    });
    render_popover(
        ctx.state,
        PopoverParts {
            props,
            resolved_placement_x,
            resolved_placement_y,
        },
        PopoverModality::NonModal,
        ctx.popover,
        // Non-modal: never a dialog.
        PopoverDialogLabel {
            aria_label: MaybeProp::default(),
            aria_labelledby: || None,
        },
        classes,
        styles,
        children,
    )
}
