use std::collections::HashSet;

use leptos::{context::Provider, prelude::*};

use super::{
    field::{FieldContext, LabelContext},
    form::use_validation_behavior,
    input::{InputContext, InputState},
    listbox::ListBoxParent,
    popover::{PopoverDialogLabel, PopoverParts, render_popover},
};
use crate::{
    Out,
    atoms::field::LabelPresence,
    hooks::{
        ComboBoxFilter, ComboBoxMenuTrigger, ComboBoxState, ComboBoxValue, IntoAttrs, Placement,
        PopoverModality, SelectMode, UseButtonInput, UseComboBoxInput, UseComboBoxReturn,
        UseComboBoxStateInput, UseHoverInput, UsePopoverInput, UsePopoverReturn,
        UseTextFieldReturn, ValidateFn, ValidationBehavior,
        collections::{CollectionMemo, Key},
        use_button, use_combobox, use_combobox_state, use_hover, use_popover, use_text_field,
    },
    utils::{
        CapturedElement, ValueBinding, classes::Classes, data_attributes::flag,
        default_class::with_default_class, styles::Styles,
    },
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
///
/// Default class: `leptonic-ComboBox`.
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
    /// The selected keys (controlled): a value or any signal.
    #[prop(into, optional)]
    value: Option<Signal<Vec<Key>>>,
    /// Receives the new state: an `RwSignal`, `WriteSignal`, closure, `Callback`, ...
    #[prop(into, optional)]
    set_value: Option<Out<Vec<Key>>>,
    #[prop(into, optional)] on_change: Option<Callback<Vec<Key>>>,
    /// The initial input text. Default: the selected option's text. Ignored when `input_value` is
    /// bound.
    #[prop(into, optional)]
    default_input_value: Option<String>,
    /// The input's text (controlled): a value or any signal.
    #[prop(into, optional)]
    input_value: Option<Signal<String>>,
    /// Receives the new state: an `RwSignal`, `WriteSignal`, closure, `Callback`, ...
    #[prop(into, optional)]
    set_input_value: Option<Out<String>>,
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
    let classes = with_default_class("leptonic-ComboBox", classes);
    let (input_value, on_input_change) =
        ValueBinding::from_state_props(input_value, set_input_value, on_input_change);
    let (value, on_change) = ValueBinding::from_state_props(value, set_value, on_change);
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
        collection,
        should_close_on_blur: true,
        on_open_change: None,
    });

    let popover = CapturedElement::new();
    // As in react-aria-components: a visible label is expected unless an ARIA label is given.
    let label_presence = LabelPresence::new(aria_label, aria_labelledby.as_ref());
    let has_label = label_presence.has_label;
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
        state,
        id: None,
        aria_describedby: None,
        should_focus_wrap: false,
        keyboard_delegate: None,
        on_focus: None,
        on_blur: None,
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
    let label = LabelContext::label(label_props).with_presence(label_presence);
    let field = FieldContext {
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
                <Provider value=label><Provider value=field>
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
                </Provider></Provider>
            </Provider>
        </Provider>
    }
}

/// The button opening the popover (not in the tab order: the keyboard uses ArrowDown on the
/// input). Data attributes: `data-open`, `data-pressed`, `data-hovered`, `data-disabled`,
/// `data-focus-visible`.
///
/// Default class: `leptonic-ComboBoxButton`.
#[component]
pub fn ComboBoxButton(
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let classes = with_default_class("leptonic-ComboBoxButton", classes);
    let ctx = expect_context::<ComboBoxCtx>();
    let input = ctx.parts.with_value(|p| p.button.clone());
    let is_disabled = input.is_disabled;
    let button = use_button(input);
    let hover = use_hover(UseHoverInput {
        is_disabled,
        ..UseHoverInput::default()
    });
    let (attrs, button_styles) = button.props.into_parts();
    let styles = button_styles.merge(styles);
    let state = ctx.state;

    view! {
        <button
            {..attrs}
            {..hover.props.into_attrs()}
            class=classes
            style=styles
            data-open=flag(Signal::derive(move || state.is_open()))
            data-pressed=flag(button.is_pressed)
            data-hovered=flag(hover.is_hovered)
            data-disabled=flag(is_disabled)
        >
            {children()}
        </button>
    }
}

/// The popover with the options' [`ListBox`](super::listbox::ListBox), positioned at the input
/// and mounted while open. It is non-modal (react-aria-components): focus stays in the input, and
/// the page stays usable; scrolling closes it.
///
/// Default class: `leptonic-ComboBoxPopover`.
#[component]
#[allow(clippy::needless_pass_by_value)]
pub fn ComboBoxPopover(
    /// Where the popover goes relative to the input.
    #[prop(into, default = Signal::stored(Placement::BottomStart))]
    placement: Signal<Placement>,
    /// The popover's maximum height. Default: the room available.
    #[prop(into, optional)]
    max_height: Signal<Option<f64>>,
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
    let classes = with_default_class("leptonic-ComboBoxPopover", classes);
    let ctx = expect_context::<ComboBoxCtx>();
    let UsePopoverReturn {
        props,
        arrow_props,
        placement: resolved_placement,
        trigger_anchor_point,
        ..
    } = use_popover(UsePopoverInput {
        trigger: ctx.anchor,
        placement,
        max_height,
        offset,
        cross_offset,
        container_padding,
        should_flip,
        modality: PopoverModality::NonModal,
        state: ctx.state,
        arrow_size: Signal::stored(None),
        arrow_boundary_offset: Signal::stored(0.0),
        boundary: None,
        target_rect: Signal::stored(None),
        is_keyboard_dismiss_disabled: Signal::stored(false),
        should_close_on_interact_outside: None,
        group: None,
        is_submenu: false,
    });
    render_popover(
        ctx.state,
        PopoverParts {
            props,
            arrow_props,
            placement: resolved_placement,
            trigger_anchor_point,
            trigger: ctx.anchor,
            trigger_name: Some("ComboBox"),
        },
        PopoverModality::NonModal,
        ctx.popover,
        super::popover::PopoverGroup::Root(CapturedElement::new()),
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
