// Upstream: react-aria-components/src/ComboBox.tsx @ 99e6102368
// Upstream: react-aria-components/test/ComboBox.test.js @ 99e6102368
// Upstream: react-aria-components/test/ComboBox.browser.test.tsx @ 99e6102368
// Upstream: react-aria-components/test/ComboBox.ssr.test.js @ 99e6102368
use std::collections::HashSet;

use leptos::{context::Provider, prelude::*};
use leptos_classes::Classes;

use super::{
    field::{FieldContext, LabelContext},
    form::use_validation_behavior,
    input::{InputContext, InputState},
    listbox::ListBoxParent,
    popover::{PopoverDialogLabel, PopoverParts, render_popover},
    typed_values::{KeyedStateProps, SelectedValues, selected_state_props},
};
use crate::{
    CapturedElement, IntoAttrs, Out, ValueBinding,
    atoms::field::LabelPresence,
    hooks::{
        button::{UseButtonInput, use_button},
        collections::{CollectionMemo, Key},
        combobox::{
            ComboBoxFilter, ComboBoxFormValue, ComboBoxMenuTrigger, ComboBoxOpenChange,
            ComboBoxState, ComboBoxValue, UseComboBoxInput, UseComboBoxReturn,
            UseComboBoxStateInput, use_combobox, use_combobox_state,
        },
        form::{UseTextFieldReturn, ValidateFn, ValidationBehavior, use_text_field},
        overlay::{
            OverlayPositionOptions, Placement, PopoverModality, UsePopoverInput, UsePopoverReturn,
            use_popover,
        },
    },
    utils::{
        data_attributes::flag,
        default_class::with_default_class,
        list_formatter::{ListFormatOptions, ListFormatter},
        scoped_context::{ClearContexts, clear_context},
        styles::Styles,
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - The value is typed and its type is the selection mode (`S: SelectedValues`: `Option<V>`
//   selects one value, `HashSet<V>` several; react-aria: `selectionMode` with `Key | null` or
//   `Key[]`); the collection's keys are the values'. `validate` gets both the input text and the
//   typed value (`ComboBoxValue<S>`).
// - State (C4): `default_value` + `on_change`, or `value` + `set_value`; the input text's
//   `default_input_value` + `on_input_value_change`, or `input_value` + `set_input_value`.
// - The parts are atoms reading the combo box's context (`ComboBoxButton`, `ComboBoxPopover`,
//   `ComboBoxValue`, the `Input`; react-aria-components: contexts consumed by `Button`,
//   `Popover`, `Input`, `ComboBoxValue`). `ComboBoxValue` renders the selected texts or its
//   `placeholder` (no render function of the selected items).
//
// ## DIFFERENT BEHAVIOR
// - The popover is positioned at the input and as wide as the input and the button together
//   (react-aria-components: at the `Group` around them if there is one; leptonic has no `Group`
//   atom yet).
//
// =============================================================================

/// Context from [`ComboBox`] to its parts.
#[derive(Clone)]
pub struct ComboBoxContext {
    pub state: ComboBoxState,
    pub is_disabled: Signal<bool>,
    /// The element the popover is positioned at (the input).
    pub anchor: CapturedElement,
    /// The popover element.
    pub popover: CapturedElement,
    /// The button element (the popover is as wide as the input and the button together).
    pub button: CapturedElement,
    /// The list box's element: the popover keeps its focused option in place when it moves.
    pub listbox: CapturedElement,
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
/// With a `name`, the selected keys are submitted in hidden inputs rendered inside the combo box
/// (`form_value`: [`ComboBoxFormValue::Key`]), or the input's text under that name
/// ([`ComboBoxFormValue::Text`], always with `allows_custom_value`).
///
/// Data attributes: `data-open`, `data-focused`, `data-invalid`, `data-disabled`,
/// `data-readonly`, `data-required`.
///
/// Default class: `leptonic-ComboBox`.
#[component]
#[allow(
    clippy::too_many_lines,
    clippy::fn_params_excessive_bools,
    clippy::implicit_hasher
)]
pub fn ComboBox<S: SelectedValues>(
    /// All options; their keys are the values' (`SelectionValue::to_key`).
    #[prop(into)]
    collection: CollectionMemo,
    /// Shows the options matching the input (e.g. [`use_contains_filter`](crate::hooks::combobox::use_contains_filter)).
    /// Without a filter, all options are shown.
    #[prop(optional)]
    filter: Option<ComboBoxFilter>,
    /// The initially selected value(s). Ignored when `value` is bound.
    #[prop(optional)]
    default_value: S,
    /// The selected value(s) (controlled): a value or any signal. `Option<V>` selects one value,
    /// `HashSet<V>` several.
    #[prop(into, optional)]
    value: Option<Signal<S>>,
    /// Receives the new state: an `RwSignal`, `WriteSignal`, closure, `Callback`, ...
    #[prop(into, optional)]
    set_value: Option<Out<S>>,
    #[prop(into, optional)] on_change: Option<Callback<S>>,
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
    #[prop(into, optional)] on_input_value_change: Option<Callback<String>>,
    #[prop(into, optional)] disabled_keys: Option<Signal<HashSet<Key>>>,
    /// When the popover opens: when the user types (default), when the input gets focus, or
    /// only by the button and the arrow keys.
    #[prop(into, optional)]
    menu_trigger: Signal<ComboBoxMenuTrigger>,
    /// Open the popover even without options (e.g. to show the `ListBox`'s empty state).
    #[prop(into, optional)]
    allows_empty_collection: Signal<bool>,
    /// Keep typed text that matches no option (clearing the value) instead of reverting it.
    /// Whether the form gets the text or the key is decided when the combo box is created.
    #[prop(into, optional)]
    allows_custom_value: Signal<bool>,
    /// Whether the arrow keys wrap around at the ends of the options.
    #[prop(optional)]
    should_focus_wrap: bool,
    #[prop(into, optional)] is_disabled: Signal<bool>,
    #[prop(into, optional)] is_read_only: Signal<bool>,
    #[prop(into, optional)] is_required: Signal<bool>,
    /// Called when the popover opens (with what opened it) or closes.
    #[prop(into, optional)]
    on_open_change: Option<Callback<ComboBoxOpenChange>>,
    /// Labels the combo box when there is no [`Label`](super::field::Label).
    #[prop(into, optional)]
    aria_label: MaybeProp<String>,
    #[prop(into, optional)] aria_labelledby: Option<String>,
    #[prop(into, optional)] placeholder: MaybeProp<String>,
    /// The form field name.
    #[prop(into, optional)]
    name: Option<String>,
    /// What the form submits: the selected keys (default) or the input's text.
    #[prop(optional)]
    form_value: ComboBoxFormValue,
    /// The id of the form the combo box belongs to, if it is outside of it.
    #[prop(into, optional)]
    form: Option<String>,
    #[prop(into, optional)] is_invalid: Signal<bool>,
    #[prop(optional)] validate: Option<ValidateFn<ComboBoxValue<S>>>,
    /// Default: the surrounding [`Form`](super::form::Form)'s, else `Native`.
    #[prop(optional)]
    validation_behavior: Option<ValidationBehavior>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let classes = with_default_class("leptonic-ComboBox", classes);
    let (input_value, on_input_change) =
        ValueBinding::from_state_props(input_value, set_input_value, on_input_value_change);
    let KeyedStateProps {
        default_value,
        value,
        set_value,
        on_change,
        ..
    } = selected_state_props(Some(default_value), value, set_value, on_change, None);
    let validate = validate.map(|validate| -> ValidateFn<ComboBoxValue> {
        std::sync::Arc::new(move |keyed: &ComboBoxValue| {
            validate(&ComboBoxValue {
                input_value: keyed.input_value.clone(),
                value: S::from_key_list(&keyed.value),
            })
        })
    });
    let (value, on_change) = ValueBinding::from_state_props(value, set_value, on_change);
    let validation_behavior = use_validation_behavior(validation_behavior);
    let state = use_combobox_state(UseComboBoxStateInput {
        filter,
        selection_mode: S::MODE,
        default_value: default_value.unwrap_or_default(),
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
        name,
        collection,
        should_close_on_blur: true,
        on_open_change,
    });

    let popover = CapturedElement::new();
    let button_element = CapturedElement::new();
    // As in react-aria-components: a visible label is expected unless an ARIA label is given.
    let label_presence = LabelPresence::new(aria_label, aria_labelledby.as_ref());
    let has_label = label_presence.has_label;
    let UseComboBoxReturn {
        input,
        input_props,
        button,
        listbox,
        form_values,
        ..
    } = use_combobox(UseComboBoxInput {
        is_disabled,
        is_required,
        has_label,
        aria_label,
        aria_labelledby,
        placeholder,
        form_value,
        form: form.clone(),
        popover,
        button_element,
        state,
        id: None,
        aria_describedby: None,
        should_focus_wrap,
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
    let ctx = ComboBoxContext {
        state,
        is_disabled,
        anchor,
        popover,
        button: button_element,
        listbox: listbox.element,
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
    let is_focused = Signal::derive(move || state.is_focused());
    let hidden_name = state.name();

    view! {
        <Provider value=ctx>
            <Provider value=listbox_parent>
                <Provider value=label>
                    <Provider value=field>
                        <Provider value=input>
                            <div
                                class=classes
                                style=styles
                                data-open=flag(is_open)
                                data-focused=flag(is_focused)
                                data-invalid=flag(state.validation.is_invalid)
                                data-disabled=flag(is_disabled)
                                data-readonly=flag(is_read_only)
                                data-required=flag(is_required)
                            >
                                {children()}
                                // The selected keys for the form (react-aria-components' `ComboBox`).
                                {move || {
                                    form_values
                                        .get()
                                        .into_iter()
                                        .map(|value| {
                                            view! {
                                                <input
                                                    type="hidden"
                                                    name=hidden_name.clone()
                                                    form=form.clone()
                                                    value=value
                                                />
                                            }
                                        })
                                        .collect_view()
                                }}
                            </div>
                        </Provider>
                    </Provider>
                </Provider>
            </Provider>
        </Provider>
    }
}

/// The button opening the popover (not in the tab order: the keyboard uses ArrowDown on the
/// input). It counts as pressed while the popover is open (react-aria-components).
///
/// Data attributes: `data-open`, `data-pressed`, `data-hovered`, `data-focused`,
/// `data-focus-visible`, `data-disabled`.
///
/// Default class: `leptonic-ComboBoxButton`.
#[component]
pub fn ComboBoxButton(
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let classes = with_default_class("leptonic-ComboBoxButton", classes);
    let Some(ctx) = use_context::<ComboBoxContext>() else {
        crate::utils::dev_warn!("A <ComboBoxButton> must be inside a <ComboBox>.");
        return None;
    };
    let input = ctx.parts.with_value(|p| p.button.clone());
    let is_disabled = input.is_disabled;
    let button = use_button(input);
    let (attrs, button_styles) = button.props.into_parts();
    let styles = button_styles.merge(styles);
    let state = ctx.state;
    let is_open = Signal::derive(move || state.is_open());

    Some(view! {
        <button
            {..attrs}
            {..ctx.button.attr()}
            class=classes
            style=styles
            data-open=flag(is_open)
            data-pressed=flag(Signal::derive(move || is_open.get() || button.is_pressed.get()))
            data-hovered=flag(button.is_hovered)
            data-focused=flag(button.is_focused)
            data-focus-visible=flag(button.is_focus_visible)
            data-disabled=flag(is_disabled)
        >
            {children()}
        </button>
    })
}

/// The text of the selected options (of a combo box selecting several), or `placeholder`, listed
/// in the locale's way ("Cat, Dog, and Kangaroo"). Exposes `data-placeholder` while nothing is
/// selected.
///
/// Default class: `leptonic-ComboBoxValue`.
#[component]
pub fn ComboBoxValue(
    /// Shown while nothing is selected.
    #[prop(into, optional)]
    placeholder: MaybeProp<String>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
) -> impl IntoView {
    let classes = with_default_class("leptonic-ComboBoxValue", classes);
    let Some(ctx) = use_context::<ComboBoxContext>() else {
        crate::utils::dev_warn!("A <ComboBoxValue> must be inside a <ComboBox>.");
        return None;
    };
    let state = ctx.state;
    let locale = crate::utils::i18n::use_locale();
    let selected_items = Memo::new(move |_| state.selected_items());
    let selected_text = Memo::new(move |_| {
        selected_items.with(|items| {
            let texts: Vec<&str> = items
                .iter()
                .map(|node| &*node.text_value)
                .filter(|text| !text.is_empty())
                .collect();
            locale.with(|locale| {
                ListFormatter::new(locale, &ListFormatOptions::default()).format(&texts)
            })
        })
    });
    let is_placeholder = Signal::derive(move || selected_items.with(Vec::is_empty));
    let text = move || {
        let text = selected_text.get();
        if text.is_empty() {
            placeholder.get().unwrap_or_default()
        } else {
            text
        }
    };

    Some(view! {
        <div class=classes style=styles data-placeholder=flag(is_placeholder)>
            {text}
        </div>
    })
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
    let ctx = expect_context::<ComboBoxContext>();
    let UsePopoverReturn {
        props,
        arrow_props,
        placement: resolved_placement,
        trigger_anchor_point,
        ..
    } = use_popover(UsePopoverInput {
        trigger: ctx.anchor,
        position: OverlayPositionOptions {
            placement,
            offset,
            cross_offset,
            container_padding,
            should_flip,
            max_height,
            ..OverlayPositionOptions::default()
        },
        modality: PopoverModality::NonModal,
        state: ctx.state,
        target_rect: Signal::stored(None),
        scroll: Some(ctx.listbox),
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
            on_enter: None,
            on_exit: None,
            width_with: Some(ctx.button),
            // A label, input or text inside the popover isn't the combo box's
            // (react-aria-components' `clearContexts`).
            clear_contexts: ClearContexts(&[
                clear_context::<LabelContext>,
                clear_context::<FieldContext>,
                clear_context::<InputContext>,
            ]),
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
