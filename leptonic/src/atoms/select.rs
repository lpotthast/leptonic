// Upstream: react-aria-components/src/Select.tsx @ 99e6102368
// Upstream: react-aria-components/test/Select.test.js @ 99e6102368
// Upstream: react-aria-components/test/Select.ssr.test.js @ 99e6102368
use std::collections::HashSet;

use leptos::{context::Provider, ev, prelude::*};
use leptos_classes::Classes;

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - The value is typed and its type is the selection mode (`S: SelectedValues`: `Option<V>`
//   selects one value, `HashSet<V>` several; react-aria: `selectionMode` with `Key | null` or
//   `Key[]`); the collection's keys are the values'.
// - State (C4): `default_value` + `on_change`, or `value` + `set_value`; the popover's
//   `default_open`, or `is_open` + `set_open`.
// - The parts are atoms reading `SelectContext` (`SelectTrigger`, `SelectValue`, `SelectPopover`,
//   `HiddenSelect`; react-aria-components: contexts consumed by `Button`, `Popover`, ...).
//
// =============================================================================
pub use super::typed_values::SelectedValues;
use super::{
    field::{FieldContext, LabelContext},
    form::use_validation_behavior,
    popover::{PopoverDialogLabel, PopoverParts, render_popover},
    typed_values::{KeyedStateProps, selected_state_props},
};
use crate::{
    CapturedElement, IntoAttrs, Out, ValueBinding,
    atoms::field::LabelPresence,
    hooks::{
        button::use_button,
        collections::{CloseOnSelect, CollectionMemo, Key},
        focus::{FocusRingTarget, UseFocusRingInput, use_focus_ring},
        form::{UseLabelProps, ValidateFn, ValidationBehavior},
        listbox::UseListBoxInput,
        overlay::{
            OverlayPositionOptions, Placement, PopoverModality, UsePopoverInput, UsePopoverReturn,
            use_popover,
        },
        select::{
            SelectState, UseHiddenSelectReturn, UseSelectInput, UseSelectReturn,
            UseSelectStateInput, UseSelectTriggerProps, use_hidden_select, use_select,
            use_select_state,
        },
    },
    utils::{
        data_attributes::flag,
        default_class::with_default_class,
        intl_strings::{AtomStrings, use_localized_strings},
        scoped_context::{ClearContexts, clear_context},
        styles::Styles,
    },
};

/// Context from [`Select`] to its parts.
#[derive(Clone)]
pub struct SelectContext {
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
    trigger: crate::hooks::button::UseButtonInput,
    trigger_props: UseSelectTriggerProps,
    value_id: String,
    hidden_select: crate::hooks::select::UseHiddenSelectInput,
}

impl SelectContext {
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
pub fn Select<S: SelectedValues>(
    /// The options; their keys are the values' (`SelectionValue::to_key`).
    #[prop(into)]
    collection: CollectionMemo,
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
    /// Called when the selected value(s) change.
    #[prop(into, optional)]
    on_change: Option<Callback<S>>,
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
    /// Close the popover when an option is selected. Default: when selecting one value.
    #[prop(into, optional)]
    should_close_on_select: CloseOnSelect,
    /// Labels the select when there is no `Label` inside.
    #[prop(into, optional)]
    aria_label: MaybeProp<String>,
    #[prop(into, optional)] aria_labelledby: Option<String>,
    #[prop(into, optional)] is_invalid: Signal<bool>,
    #[prop(optional)] validate: Option<ValidateFn<S>>,
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
    let KeyedStateProps {
        default_value,
        value,
        set_value,
        on_change,
        validate,
    } = selected_state_props(Some(default_value), value, set_value, on_change, validate);
    let default_value = default_value.unwrap_or_default();
    let (value, on_change) = ValueBinding::from_state_props(value, set_value, on_change);
    let (is_open, on_open_change) =
        ValueBinding::from_state_props(is_open, set_open, on_open_change);
    let validation_behavior = use_validation_behavior(validation_behavior);
    let state = use_select_state(UseSelectStateInput {
        selection_mode: S::MODE,
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

    let ctx = SelectContext {
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
        target: FocusRingTarget::Within,
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
/// Outside a [`Select`], renders nothing and warns in debug builds.
///
/// Default class: `leptonic-SelectTrigger`.
#[component]
pub fn SelectTrigger(
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let Some(ctx) = use_context::<SelectContext>() else {
        crate::utils::dev_warn!("SelectTrigger: not inside a Select");
        return None;
    };
    let classes = with_default_class("leptonic-SelectTrigger", classes);
    let (input, trigger_props) = ctx.part(|p| (p.trigger.clone(), p.trigger_props.clone()));
    let button = use_button(input);
    let (attrs, button_styles) = button.props.into_parts();
    let styles = button_styles.merge(styles);
    let state = ctx.state;

    Some(view! {
        <button
            {..attrs}
            {..trigger_props.into_attrs()}
            {..ctx.trigger_element.attr()}
            class=classes
            style=styles
            data-open=flag(Signal::derive(move || state.is_open()))
            data-invalid=flag(ctx.is_invalid)
            data-disabled=flag(ctx.is_disabled)
            data-pressed=flag(Signal::derive(move || button.is_pressed.get() || state.is_open()))
            data-hovered=flag(button.is_hovered)
        >
            {children()}
        </button>
    })
}

/// The text of the selected option(s), or `placeholder`. Several selected options are listed in
/// the locale's way ("Cat, Dog, and Kangaroo"). Exposes `data-placeholder` while nothing is
/// selected.
///
/// Outside a [`Select`], renders nothing and warns in debug builds.
///
/// Default class: `leptonic-SelectValue`.
#[component]
pub fn SelectValue(
    /// Shown while nothing is selected. Default: "Select an item" (localized).
    #[prop(into, optional)]
    placeholder: MaybeProp<String>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
) -> impl IntoView {
    let Some(ctx) = use_context::<SelectContext>() else {
        crate::utils::dev_warn!("SelectValue: not inside a Select");
        return None;
    };
    let classes = with_default_class("leptonic-SelectValue", classes);
    let state = ctx.state;
    let id = ctx.parts.with_value(|p| p.value_id.clone());
    let selected_items = Memo::new(move |_| state.selected_items());
    let is_empty = move || selected_items.with(Vec::is_empty);
    let locale = crate::utils::i18n::use_locale();
    let strings = use_localized_strings::<AtomStrings>();
    let text = move || {
        let items = selected_items.get();
        if items.is_empty() {
            placeholder
                .get()
                .unwrap_or_else(|| strings.read().select_placeholder())
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

    Some(view! {
        <span
            id=id
            class=classes
            style=styles
            data-placeholder=move || is_empty().then_some("true")
        >
            {text}
        </span>
    })
}

/// The select's popover, positioned at the trigger. Holds the options'
/// [`ListBox`](super::listbox::ListBox); mounted while open. Modal, as react-aria-components'
/// select popover: focus stays inside and the rest of the page is hidden from assistive
/// technology until it closes.
///
/// Outside a [`Select`], renders nothing and warns in debug builds.
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
    let Some(ctx) = use_context::<SelectContext>() else {
        crate::utils::dev_warn!("SelectPopover: not inside a Select");
        return None;
    };
    let classes = with_default_class("leptonic-SelectPopover", classes);
    let ctx_labelledby = ctx.listbox_input().aria_labelledby;
    let UsePopoverReturn {
        props,
        arrow_props,
        placement: resolved_placement,
        trigger_anchor_point,
        ..
    } = use_popover(UsePopoverInput {
        trigger: ctx.trigger_element,
        position: OverlayPositionOptions {
            placement,
            offset,
            cross_offset,
            container_padding,
            should_flip,
            max_height,
            ..OverlayPositionOptions::default()
        },
        state: ctx.state,
        target_rect: Signal::stored(None),
        // The focused option keeps its place when the popover moves (react-aria-components).
        scroll: Some(ctx.listbox_input().element),
        modality: PopoverModality::Modal,
        is_keyboard_dismiss_disabled: Signal::stored(false),
        should_close_on_interact_outside: None,
        group: None,
        is_submenu: false,
    });
    Some(render_popover(
        ctx.state,
        PopoverParts {
            props,
            arrow_props,
            placement: resolved_placement,
            trigger_anchor_point,
            trigger: ctx.trigger_element,
            trigger_name: Some("Select"),
            on_enter: None,
            on_exit: None,
            width_with: None,
            clear_contexts: ClearContexts(&[
                clear_context::<LabelContext>,
                clear_context::<FieldContext>,
            ]),
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
    ))
}

/// A visually hidden native form element mirroring the select's value: a `<select>` (for up
/// to 300 options, supporting autofill) or hidden inputs.
///
/// Outside a [`Select`], renders nothing and warns in debug builds.
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
    let Some(ctx) = use_context::<SelectContext>() else {
        crate::utils::dev_warn!("HiddenSelect: not inside a Select");
        return None;
    };
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
    } = use_hidden_select(crate::hooks::select::UseHiddenSelectInput {
        auto_complete,
        trigger: Some(ctx.trigger_element),
        label: MaybeProp::derive(move || label.get().or_else(|| default_label.get())),
        ..input
    });
    let select_props = StoredValue::new(select_props);
    let input_props = StoredValue::new(input_props);
    let first_input_capture = StoredValue::new(first_input_capture);

    Some(view! {
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
                            key=|option| option.key.clone()
                            let:option
                        >
                            {
                                let state = ctx.state;
                                let key = option.key.clone();
                                let text = Memo::new(move |_| {
                                    key.as_ref().map_or_else(String::new, |key| {
                                        state.list.collection.with(|c| {
                                            c.get(key).map_or_else(|| key.to_string(), |node| node.text_value.to_string())
                                        })
                                    })
                                });
                                let is_empty = option.key.is_none();
                                let value = option.value.clone();
                                let is_selected = move || option.is_selected(&state);
                                view! {
                                    <option
                                        value=value
                                        label=is_empty.then_some("\u{A0}")
                                        selected=is_selected.clone()
                                        prop:selected=is_selected
                                    >
                                        {move || text.get()}
                                    </option>
                                }
                            }
                        </For>
                    </select>
                </label>
            </Show>
        </div>
    })
}

#[cfg(all(test, feature = "ssr"))]
mod tests {
    use assertr::prelude::*;

    use super::*;
    use crate::testing::with_owner;

    // No upstream: misplaced composition parts render nothing in Leptonic.
    #[test]
    fn select_trigger_without_parent_renders_nothing() {
        with_owner(|| {
            let html = view! { <SelectTrigger>"Misplaced trigger"</SelectTrigger> }.to_html();
            assert_that!(html).is_equal_to(().to_html());
        });
    }

    #[test]
    fn select_value_without_parent_renders_nothing() {
        with_owner(|| {
            let html = view! { <SelectValue placeholder="Misplaced value" /> }.to_html();
            assert_that!(html).is_equal_to(().to_html());
        });
    }

    #[test]
    fn select_popover_without_parent_renders_nothing() {
        with_owner(|| {
            let html = view! { <SelectPopover>"Misplaced popover"</SelectPopover> }.to_html();
            assert_that!(html).is_equal_to(().to_html());
        });
    }

    #[test]
    fn hidden_select_without_parent_renders_nothing() {
        with_owner(|| {
            let html = view! { <HiddenSelect /> }.to_html();
            assert_that!(html).is_equal_to(().to_html());
        });
    }
}
