use std::collections::HashSet;

use leptos::{context::Provider, prelude::*};

use super::{
    field::{FieldContext, FieldLabelProps},
    form::use_validation_behavior,
    popover::{PopoverDialogLabel, PopoverParts, render_popover},
};
use crate::{
    hooks::{
        IntoAttrs, PlacementX, PlacementY, PopoverModality, SelectMode, SelectState,
        UseHiddenSelectReturn, UseLabelProps, UseListBoxInput, UsePopoverInput, UsePopoverReturn,
        UseSelectInput, UseSelectReturn, UseSelectStateInput, UseSelectTriggerProps, ValidateFn,
        ValidationBehavior,
        collections::{CollectionMemo, Key},
        use_button, use_hidden_select, use_popover, use_select, use_select_state,
    },
    utils::ValueBinding,
    utils::data_attributes::flag,
    utils::{CapturedElement, classes::Classes, styles::Styles},
};

/// Context from [`Select`] to its parts.
#[derive(Clone)]
pub struct SelectCtx {
    pub state: SelectState,
    pub is_invalid: Signal<bool>,
    pub is_disabled: Signal<bool>,
    /// The trigger element (the popover's anchor).
    pub trigger_element: CapturedElement,
    parts: StoredValue<Parts>,
    listbox: StoredValue<UseListBoxInput>,
}

/// The `use_select` outputs for the parts.
#[derive(Clone)]
struct Parts {
    trigger: crate::hooks::UseButtonInput,
    trigger_props: UseSelectTriggerProps,
    value_id: String,
    hidden_select: crate::hooks::UseHiddenSelectInput,
}

impl SelectCtx {
    /// The configuration of the popover's listbox.
    pub fn listbox_input(&self) -> UseListBoxInput {
        self.listbox.get_value()
    }

    fn part<T>(&self, part: impl FnOnce(&Parts) -> T) -> T {
        self.parts.with_value(part)
    }
}

/// A headless select: a button showing the selected option, opening a popover with a
/// [`ListBox`](super::listbox::ListBox) of the options.
///
/// Compose it from a [`Label`](super::field::Label) (clicking it focuses the trigger),
/// [`SelectTrigger`] (containing [`SelectValue`]), [`SelectPopover`] (containing a `ListBox` with
/// one `ListBoxItem` per option), a [`Description`](super::field::Description), a
/// [`FieldError`](super::field::FieldError) and [`HiddenSelect`] (for forms).
#[component]
#[allow(
    clippy::too_many_lines,
    clippy::fn_params_excessive_bools,
    clippy::implicit_hasher
)]
pub fn Select(
    /// The options.
    #[prop(into)]
    collection: CollectionMemo,
    #[prop(optional)] selection_mode: SelectMode,
    /// The initially selected keys (at most one in `Single` mode). Ignored when `value` is bound.
    #[prop(into, optional)]
    default_value: Vec<Key>,
    /// The selected keys as app state (e.g. an `RwSignal<Vec<Key>>`), replacing `default_value`.
    #[prop(into, optional)]
    value: Option<ValueBinding<Vec<Key>>>,
    /// Called when the selected keys change.
    #[prop(into, optional)]
    on_change: Option<Callback<Vec<Key>>>,
    #[prop(into, optional)] disabled_keys: Option<Signal<HashSet<Key>>>,
    #[prop(into, optional)] is_disabled: Signal<bool>,
    #[prop(optional)] is_required: bool,
    #[prop(optional)] default_open: bool,
    #[prop(into, optional)] on_open_change: Option<Callback<bool>>,
    #[prop(optional)] allows_empty_collection: bool,
    /// Close the popover when an option is selected. Default: in `Single` mode.
    #[prop(optional)]
    should_close_on_select: Option<bool>,
    /// Labels the select when there is no [`SelectLabel`].
    #[prop(into, optional)]
    aria_label: MaybeProp<String>,
    #[prop(into, optional)] aria_labelledby: Option<String>,
    #[prop(into, optional)] is_invalid: Signal<bool>,
    #[prop(optional)] validate: Option<ValidateFn<Vec<Key>>>,
    /// Default: the surrounding [`Form`](super::form::Form)'s, else `Native`.
    #[prop(optional)]
    validation_behavior: Option<ValidationBehavior>,
    /// The form field name (used by [`HiddenSelect`]).
    #[prop(into, optional)]
    name: Option<String>,
    #[prop(into, optional)] form: Option<String>,
    #[prop(into, optional)] on_focus_change: Option<Callback<bool>>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let validation_behavior = use_validation_behavior(validation_behavior);
    let state = use_select_state(UseSelectStateInput {
        selection_mode,
        default_value,
        value,
        on_change,
        disabled_keys: disabled_keys.unwrap_or_default(),
        should_close_on_select,
        allows_empty_collection,
        default_open,
        on_open_change,
        is_invalid,
        validate,
        validation_behavior,
        name: name.clone(),
        ..UseSelectStateInput::new(collection)
    });

    // As in react-aria-components: a visible label is expected unless an ARIA label is given.
    let has_label = aria_label.get_untracked().is_none() && aria_labelledby.is_none();
    let UseSelectReturn {
        label_props,
        trigger,
        trigger_props,
        value_props,
        description_props,
        error_message_props,
        listbox,
        hidden_select,
        is_invalid,
        validation_errors,
    } = use_select(UseSelectInput {
        is_disabled,
        is_required,
        has_label,
        aria_label,
        aria_labelledby,
        on_focus_change,
        name,
        form,
        validation_behavior,
        ..UseSelectInput::new(state)
    });

    let ctx = SelectCtx {
        state,
        is_invalid,
        is_disabled,
        trigger_element: CapturedElement::new(),
        parts: StoredValue::new(Parts {
            trigger,
            trigger_props,
            value_id: value_props.id,
            hidden_select,
        }),
        listbox: StoredValue::new(listbox),
    };
    let field = FieldContext {
        label: FieldLabelProps::span(UseLabelProps {
            id: label_props.id,
            html_for: None,
        })
        .with_on_click(label_props.on_click),
        description: description_props,
        error_message: error_message_props,
        is_invalid,
        validation_errors,
        validation_details: Signal::derive(move || {
            state.validation.display_validation.get().validation_details
        }),
    };
    let is_open = Signal::derive(move || state.is_open());

    let listbox_parent = super::listbox::ListBoxParent { input: ctx.listbox };

    view! {
        <Provider value=ctx>
            <Provider value=listbox_parent>
                <Provider value=field>
                    <div
                        class=classes
                        style=styles
                        data-open=flag(is_open)
                        data-invalid=flag(is_invalid)
                        data-disabled=flag(is_disabled)
                    >
                        {children()}
                    </div>
                </Provider>
            </Provider>
        </Provider>
    }
}

/// The button opening the select's popover. Exposes `data-open`, `data-invalid`,
/// `data-disabled`, `data-pressed` and (from `use_button`) `data-focus-visible` for styling.
#[component]
pub fn SelectTrigger(
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let ctx = expect_context::<SelectCtx>();
    let (input, trigger_props) = ctx.part(|p| (p.trigger.clone(), p.trigger_props.clone()));
    let button = use_button(input);
    let (attrs, button_styles) = button.props.into_parts();
    let styles = button_styles.merge(styles);
    let state = ctx.state;

    view! {
        <button
            {..attrs}
            {..trigger_props.into_attrs()}
            {..ctx.trigger_element.attr()}
            class=classes
            style=styles
            data-open=flag(Signal::derive(move || state.is_open()))
            data-invalid=flag(ctx.is_invalid)
            data-disabled=flag(ctx.is_disabled)
            data-pressed=flag(button.is_pressed)
        >
            {children()}
        </button>
    }
}

/// The text of the selected option(s), or `placeholder`. Exposes `data-placeholder` while
/// nothing is selected.
#[component]
pub fn SelectValue(
    #[prop(into, optional)] placeholder: Option<String>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
) -> impl IntoView {
    let ctx = expect_context::<SelectCtx>();
    let state = ctx.state;
    let id = ctx.parts.with_value(|p| p.value_id.clone());
    let is_empty = move || state.value().is_empty();
    let text = move || {
        let items = state.selected_items();
        if items.is_empty() {
            placeholder.clone().unwrap_or_default()
        } else {
            items
                .iter()
                .map(|n| n.text_value.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        }
    };

    view! {
        <span
            id=id
            class=classes
            style=styles
            data-placeholder=move || is_empty().then_some("true")
        >
            {text}
        </span>
    }
}

/// The select's popover, positioned at the trigger. Holds the options'
/// [`ListBox`](super::listbox::ListBox); mounted while open. Modal, as react-aria-components'
/// select popover: focus stays inside and the rest of the page is hidden from assistive
/// technology until it closes.
#[component]
#[allow(clippy::needless_pass_by_value)]
pub fn SelectPopover(
    #[prop(into, default = Signal::stored(PlacementX::Left))] placement_x: Signal<PlacementX>,
    #[prop(into, default = Signal::stored(PlacementY::Below))] placement_y: Signal<PlacementY>,
    /// The distance from the trigger, in pixels.
    #[prop(into, optional)]
    offset: Signal<f64>,
    #[prop(into, optional)] cross_offset: Signal<f64>,
    /// The minimum distance from the viewport edges, in pixels.
    #[prop(into, default = Signal::stored(12.0))]
    container_padding: Signal<f64>,
    /// Whether the popover flips above the trigger when there is no room below.
    #[prop(into, default = Signal::stored(true))]
    should_flip: Signal<bool>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: ChildrenFn,
) -> impl IntoView {
    let ctx = expect_context::<SelectCtx>();
    let ctx_labelledby = ctx.listbox_input().aria_labelledby;
    let UsePopoverReturn {
        props,
        resolved_placement_x,
        resolved_placement_y,
        ..
    } = use_popover(UsePopoverInput {
        trigger: ctx.trigger_element,
        placement_x,
        placement_y,
        offset,
        cross_offset,
        container_padding,
        should_flip,
        ..UsePopoverInput::new(ctx.state)
    });
    render_popover(
        ctx.state,
        PopoverParts {
            props,
            resolved_placement_x,
            resolved_placement_y,
        },
        PopoverModality::Modal,
        CapturedElement::new(),
        // A select's popover is a dialog named like its listbox (react-aria-components).
        PopoverDialogLabel {
            aria_label: MaybeProp::default(),
            aria_labelledby: move || ctx_labelledby.clone(),
        },
        classes,
        styles,
        children,
    )
}

/// A visually hidden native form element mirroring the select's value: a `<select>` (for up
/// to 300 options, supporting autofill) or hidden inputs.
#[component]
pub fn HiddenSelect(
    /// The `autocomplete` attribute (autofill hint).
    #[prop(into, optional)]
    auto_complete: Option<String>,
) -> impl IntoView {
    let ctx = expect_context::<SelectCtx>();
    let input = ctx.part(|p| p.hidden_select.clone());
    let has_name = input.name.is_some();
    let UseHiddenSelectReturn {
        container_props,
        use_native_select,
        select_props,
        options,
        input_props,
        input_values,
    } = use_hidden_select(crate::hooks::UseHiddenSelectInput {
        auto_complete,
        trigger: Some(ctx.trigger_element),
        ..input
    });
    let select_props = StoredValue::new(select_props);
    let input_props = StoredValue::new(input_props);

    view! {
        <div {..container_props.into_attrs()}>
            <Show
                when=move || use_native_select.get()
                fallback=move || {
                    has_name
                        .then(|| {
                            input_values
                                .get()
                                .into_iter()
                                .enumerate()
                                .map(|(i, value)| {
                                    let p = input_props.get_value();
                                    view! {
                                        <input
                                            type=p.r#type
                                            style=p.style
                                            autocomplete=p.auto_complete
                                            name=p.name
                                            form=p.form
                                            disabled=p.disabled
                                            required=p.required && i == 0
                                            value=value
                                        />
                                    }
                                })
                                .collect_view()
                        })
                }
            >
                <label>
                    <select {..select_props.get_value().into_attrs()}>
                        <For
                            each=move || options.get()
                            key=|option| (option.value.clone(), option.is_selected)
                            let:option
                        >
                            <option value=option.value.clone() selected=option.is_selected>
                                {if option.value.is_empty() { "\u{A0}".to_owned() } else { option.text }}
                            </option>
                        </For>
                    </select>
                </label>
            </Show>
        </div>
    }
}
