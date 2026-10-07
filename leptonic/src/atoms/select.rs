use std::collections::HashSet;

use leptos::{context::Provider, ev, prelude::*};

use super::{
    field::{FieldContext, LabelContext},
    form::use_validation_behavior,
    popover::{PopoverDialogLabel, PopoverParts, render_popover},
};
use crate::{
    Out,
    atoms::field::LabelPresence,
    hooks::{
        IntoAttrs, Placement, PopoverModality, SelectMode, SelectState, UseFocusRingInput,
        UseHiddenSelectReturn, UseLabelProps, UseListBoxInput, UsePopoverInput, UsePopoverReturn,
        UseSelectInput, UseSelectReturn, UseSelectStateInput, UseSelectTriggerProps, ValidateFn,
        ValidationBehavior,
        collections::{CloseOnSelect, CollectionMemo, Key},
        use_button, use_focus_ring, use_hidden_select, use_popover, use_select, use_select_state,
    },
    utils::{
        CapturedElement, ValueBinding, classes::Classes, data_attributes::flag,
        default_class::with_default_class, styles::Styles,
    },
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
///
/// Data attributes: `data-focused`, `data-focus-visible` (focus anywhere inside), `data-open`,
/// `data-disabled`, `data-invalid`, `data-required`.
///
/// Default class: `leptonic-Select`.
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
    /// The selected keys (controlled): a value or any signal.
    #[prop(into, optional)]
    value: Option<Signal<Vec<Key>>>,
    /// Receives the new state: an `RwSignal`, `WriteSignal`, closure, `Callback`, ...
    #[prop(into, optional)]
    set_value: Option<Out<Vec<Key>>>,
    /// Called when the selected keys change.
    #[prop(into, optional)]
    on_change: Option<Callback<Vec<Key>>>,
    #[prop(into, optional)] disabled_keys: Option<Signal<HashSet<Key>>>,
    #[prop(into, optional)] is_disabled: Signal<bool>,
    #[prop(into, optional)] is_required: Signal<bool>,
    /// Whether the popover starts open. Ignored with `is_open`.
    #[prop(optional)]
    default_open: bool,
    /// Whether the popover is open (controlled): a value or any signal.
    #[prop(into, optional)]
    is_open: Option<Signal<bool>>,
    /// Receives the open state: an `RwSignal`, `WriteSignal`, closure, `Callback`, ...
    #[prop(into, optional)]
    set_open: Option<Out<bool>>,
    #[prop(into, optional)] on_open_change: Option<Callback<bool>>,
    #[prop(optional)] allows_empty_collection: bool,
    /// Close the popover when an option is selected. Default: in `Single` mode.
    #[prop(into, optional)]
    should_close_on_select: CloseOnSelect,
    /// Labels the select when there is no `Label` inside.
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
    let classes = with_default_class("leptonic-Select", classes);
    let (value, on_change) = ValueBinding::from_state_props(value, set_value, on_change);
    let (is_open, on_open_change) =
        ValueBinding::from_state_props(is_open, set_open, on_open_change);
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
        is_open,
        on_open_change,
        is_invalid,
        validate,
        validation_behavior,
        name,
        collection,
    });

    // As in react-aria-components: a visible label is expected unless an ARIA label is given.
    let label_presence = LabelPresence::new(aria_label, aria_labelledby.as_ref());
    let has_label = label_presence.has_label;
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
        form,
        state,
        id: None,
        aria_describedby: None,
        keyboard_delegate: None,
        on_focus: None,
        on_blur: None,
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
    let label = LabelContext::span(UseLabelProps {
        id: label_props.id,
        html_for: None,
    })
    .with_on_click(label_props.on_click)
    .with_presence(label_presence);
    let field = FieldContext {
        description: description_props,
        error_message: error_message_props,
        is_invalid,
        validation_errors,
        validation_details: Signal::derive(move || {
            state.validation.display_validation.get().validation_details
        }),
    };
    let is_open = Signal::derive(move || state.is_open());
    let is_focused = Signal::derive(move || state.is_focused());
    let focus_ring = use_focus_ring(UseFocusRingInput {
        within: true,
        ..UseFocusRingInput::default()
    });
    let focus_within = (
        focus_ring.props.on_focusin.into_on(ev::focusin),
        focus_ring.props.on_focusout.into_on(ev::focusout),
    );

    let listbox_parent = super::listbox::ListBoxParent { input: ctx.listbox };

    view! {
        <Provider value=ctx>
            <Provider value=listbox_parent>
                <Provider value=label><Provider value=field>
                    <div
                        {..focus_within}
                        class=classes
                        style=styles
                        data-focused=flag(is_focused)
                        data-focus-visible=flag(focus_ring.is_focus_visible)
                        data-open=flag(is_open)
                        data-invalid=flag(is_invalid)
                        data-disabled=flag(is_disabled)
                        data-required=flag(is_required)
                    >
                        {children()}
                    </div>
                </Provider></Provider>
            </Provider>
        </Provider>
    }
}

/// The button opening the select's popover. Exposes `data-open`, `data-invalid`,
/// `data-disabled`, `data-pressed`, `data-hovered` and (from `use_button`)
/// `data-focus-visible` for styling.
///
/// Default class: `leptonic-SelectTrigger`.
#[component]
pub fn SelectTrigger(
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let classes = with_default_class("leptonic-SelectTrigger", classes);
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
            data-hovered=flag(button.is_hovered)
        >
            {children()}
        </button>
    }
}

/// The text of the selected option(s), or `placeholder`. Several selected options are listed in
/// the locale's way ("Cat, Dog, and Kangaroo"). Exposes `data-placeholder` while nothing is
/// selected.
///
/// Default class: `leptonic-SelectValue`.
#[component]
pub fn SelectValue(
    /// Shown while nothing is selected. Default: "Select an item" (react-aria-components; not
    /// localized yet).
    #[prop(into, optional)]
    placeholder: MaybeProp<String>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
) -> impl IntoView {
    let classes = with_default_class("leptonic-SelectValue", classes);
    let ctx = expect_context::<SelectCtx>();
    let state = ctx.state;
    let id = ctx.parts.with_value(|p| p.value_id.clone());
    let is_empty = move || state.value().is_empty();
    let locale = crate::utils::i18n::use_locale();
    let text = move || {
        let items = state.selected_items();
        if items.is_empty() {
            placeholder
                .get()
                .unwrap_or_else(|| "Select an item".to_owned())
        } else {
            let texts: Vec<&str> = items.iter().map(|n| &*n.text_value).collect();
            locale.with(|locale| {
                crate::utils::list_formatter::ListFormatter::new(
                    locale,
                    &crate::utils::list_formatter::ListFormatOptions::default(),
                )
                .format(&texts)
            })
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
///
/// Default class: `leptonic-SelectPopover`.
#[component]
#[allow(clippy::needless_pass_by_value)]
pub fn SelectPopover(
    /// Where the popover goes relative to the trigger.
    #[prop(into, default = Signal::stored(Placement::BottomStart))]
    placement: Signal<Placement>,
    /// The popover's maximum height. Default: the room available.
    #[prop(into, optional)]
    max_height: Signal<Option<f64>>,
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
    let classes = with_default_class("leptonic-SelectPopover", classes);
    let ctx = expect_context::<SelectCtx>();
    let ctx_labelledby = ctx.listbox_input().aria_labelledby;
    let UsePopoverReturn {
        props,
        arrow_props,
        placement: resolved_placement,
        trigger_anchor_point,
        ..
    } = use_popover(UsePopoverInput {
        trigger: ctx.trigger_element,
        placement,
        max_height,
        offset,
        cross_offset,
        container_padding,
        should_flip,
        state: ctx.state,
        arrow_size: Signal::stored(None),
        arrow_boundary_offset: Signal::stored(0.0),
        boundary: None,
        target_rect: Signal::stored(None),
        modality: PopoverModality::Modal,
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
            trigger: ctx.trigger_element,
            trigger_name: Some("Select"),
        },
        PopoverModality::Modal,
        CapturedElement::new(),
        super::popover::PopoverGroup::Root(CapturedElement::new()),
        // A select's popover is a dialog named like its listbox (react-aria-components).
        PopoverDialogLabel {
            aria_label: MaybeProp::default(),
            aria_labelledby: move || ctx_labelledby.get(),
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
    /// The text of the hidden `<label>` (browsers identify fields for autofill by their
    /// labels). Default: the select's `aria_label`.
    #[prop(into, optional)]
    label: MaybeProp<String>,
) -> impl IntoView {
    let ctx = expect_context::<SelectCtx>();
    let input = ctx.part(|p| p.hidden_select.clone());
    let has_name = ctx.state.name().is_some();
    let default_label = input.label;
    let UseHiddenSelectReturn {
        container_props,
        use_native_select,
        select_props,
        options,
        label,
        input_props,
        first_input_capture,
        input_values,
    } = use_hidden_select(crate::hooks::UseHiddenSelectInput {
        auto_complete,
        trigger: Some(ctx.trigger_element),
        label: MaybeProp::derive(move || label.get().or_else(|| default_label.get())),
        ..input
    });
    let select_props = StoredValue::new(select_props);
    let input_props = StoredValue::new(input_props);
    let first_input_capture = StoredValue::new(first_input_capture);

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
                                    let input = view! {
                                        <input
                                            type=p.r#type
                                            style=p.style
                                            autocomplete=p.auto_complete
                                            name=p.name
                                            form=p.form
                                            disabled=p.disabled
                                            required=move || i == 0 && p.required.get()
                                            value=value
                                        />
                                    };
                                    // The first input stands for the field (form reset,
                                    // native validation).
                                    if i == 0 {
                                        input
                                            .add_any_attr(first_input_capture.get_value())
                                            .into_any()
                                    } else {
                                        input.into_any()
                                    }
                                })
                                .collect_view()
                        })
                }
            >
                <label>
                    {move || label.get()}
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
