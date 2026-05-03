use std::collections::HashSet;

use leptos::{context::Provider, ev, prelude::*};

use crate::{
    hooks::{
        EscapeKeyBehavior, FocusStrategy, IntoAttrs, Keyed, ListBoxOrientation, Selection,
        SelectionBehavior, SelectionKey, SelectionMode, UseListBoxInput, UseListBoxReturn,
        UseOptionInput, UseOptionReturn, UseSelectionStateReturn, use_listbox, use_option,
    },
    utils::{CapturedElement, classes::Classes, styles::Styles},
};

use super::select::SelectCtx;

/// Context shared from [`ListBox`] to [`ListBoxItem`] children.
#[derive(Clone)]
pub struct ListBoxCtx<K: SelectionKey> {
    /// The selection state for wiring individual options.
    pub selection_state: UseSelectionStateReturn<K>,
    /// The currently focused key in the listbox.
    pub focused_key: Signal<Option<K>>,
    /// Whether selection occurs on pointer-up rather than pointer-down.
    pub should_select_on_press_up: bool,
    /// Whether options use virtual focus (aria-activedescendant).
    pub should_use_virtual_focus: bool,
    /// Whether options receive focus on mouse hover.
    pub should_focus_on_hover: bool,
}

// Manual Copy impl: derive(Copy) would add `K: Copy` which is too restrictive.
// All fields are Copy (Signal, bool, UseSelectionStateReturn).
impl<K: SelectionKey> Copy for ListBoxCtx<K> {}

/// Context shared from [`ListBoxItem`] to [`ListBoxItemLabel`] and [`ListBoxItemDescription`].
#[derive(Clone)]
pub struct ListBoxItemCtx {
    /// The ID for the label element.
    pub label_id: String,
    /// The ID for the description element.
    pub description_id: String,
    /// Whether this option is currently selected.
    pub is_selected: Signal<bool>,
    /// Whether this option is currently focused.
    pub is_focused: Signal<bool>,
    /// Whether this option is disabled.
    pub is_disabled: Signal<bool>,
    /// Whether this option is pressed.
    pub is_pressed: Signal<bool>,
    /// Whether the focus ring should be visible.
    pub is_focus_visible: Signal<bool>,
}

/// A headless listbox container atom.
///
/// Wraps `use_listbox` and provides shared state to child [`ListBoxItem`] atoms
/// via context.
///
/// When used inside a [`Select`](super::select::Select) atom, automatically
/// reads [`SelectCtx`] and wires the `menu_config` from the select hook.
/// Explicit props always override the `menu_config` values.
#[component]
#[allow(clippy::too_many_lines, clippy::implicit_hasher)]
pub fn ListBox<V: Keyed + Clone + Send + Sync + 'static>(
    /// Ordered list of all items.
    #[prop(into, optional)]
    items: Option<Signal<Vec<V>>>,
    /// The selection mode.
    #[prop(into, optional)]
    selection_mode: Option<SelectionMode>,
    /// The selection behavior (toggle vs replace).
    #[prop(into, optional)]
    selection_behavior: Option<SelectionBehavior>,
    /// Controlled selected keys.
    #[prop(into, optional)]
    selected_keys: Option<Signal<Selection<V::Key>>>,
    /// Default selected keys (uncontrolled).
    #[prop(into, optional)]
    default_selected_keys: Option<Selection<V::Key>>,
    /// Callback when selection changes.
    #[prop(into, optional)]
    on_selection_change: Option<Callback<Selection<V::Key>>>,
    /// Keys that cannot be selected.
    #[prop(into, optional)]
    disabled_keys: Option<Signal<HashSet<V::Key>>>,
    /// Whether to disallow empty selection.
    #[prop(into, optional)]
    disallow_empty_selection: Option<bool>,
    /// Whether the listbox is disabled.
    #[prop(into, optional)]
    disabled: Option<Signal<bool>>,
    /// Escape key behavior.
    #[prop(into, optional)]
    escape_key_behavior: Option<EscapeKeyBehavior>,
    /// Whether arrow key navigation wraps around.
    #[prop(into, optional)]
    should_focus_wrap: Option<bool>,
    /// Auto-focus strategy.
    #[prop(into, optional)]
    auto_focus: Option<Signal<Option<FocusStrategy>>>,
    /// Whether to select items on focus.
    #[prop(into, optional)]
    select_on_focus: Option<bool>,
    /// An accessible label for the listbox.
    #[prop(into, optional)]
    aria_label: Option<&'static str>,
    /// The ID of an element that labels the listbox.
    #[prop(into, optional)]
    aria_labelledby: Option<String>,
    /// A function to get the text value for type-ahead.
    #[prop(into, optional)]
    get_text_value: Option<Callback<V::Key, String>>,
    /// Whether selection occurs on pointer-up rather than pointer-down.
    #[prop(into, optional)]
    should_select_on_press_up: Option<bool>,
    /// Whether options should receive focus on mouse hover.
    #[prop(into, optional)]
    should_focus_on_hover: Option<bool>,
    /// Whether options use virtual focus (aria-activedescendant).
    #[prop(into, optional)]
    should_use_virtual_focus: Option<bool>,
    /// Callback when the listbox should close (e.g. Escape key).
    #[prop(into, optional)]
    on_close: Option<Callback<()>>,
    /// CSS classes.
    #[prop(into, optional)]
    classes: Classes,
    /// CSS styles.
    #[prop(into, optional)]
    styles: Styles,
    children: Children,
) -> impl IntoView {
    // Try to read SelectCtx for composition mode.
    let select_ctx = use_context::<SelectCtx<V::Key>>();
    let menu_config = select_ctx.map(|ctx| ctx.menu_config());

    // Resolve items: convert V → V::Key via Keyed trait, or use menu_config keys.
    let resolved_items = items
        .map(V::items_to_keys)
        .or_else(|| menu_config.as_ref().map(|mc| mc.items))
        .unwrap_or_else(|| Signal::derive(Vec::new));

    let resolved_selection_mode = selection_mode
        .or_else(|| menu_config.as_ref().map(|mc| mc.selection_mode))
        .unwrap_or(SelectionMode::Single);

    let resolved_selection_behavior = selection_behavior.unwrap_or(SelectionBehavior::Toggle);

    let resolved_selected_keys =
        selected_keys.or_else(|| menu_config.as_ref().map(|mc| mc.selected_keys));

    let resolved_on_selection_change =
        on_selection_change.or_else(|| menu_config.as_ref().map(|mc| mc.on_selection_change));

    let resolved_disabled_keys = disabled_keys
        .or_else(|| menu_config.as_ref().map(|mc| mc.disabled_keys))
        .unwrap_or_else(|| Signal::derive(HashSet::new));

    let resolved_disallow_empty_selection = disallow_empty_selection
        .or_else(|| menu_config.as_ref().map(|mc| mc.disallow_empty_selection))
        .unwrap_or(false);

    let resolved_disabled = disabled.unwrap_or_else(|| Signal::derive(|| false));

    let resolved_escape_key_behavior = escape_key_behavior
        .or_else(|| menu_config.as_ref().map(|mc| mc.escape_key_behavior))
        .unwrap_or_default();

    let resolved_should_focus_wrap = should_focus_wrap
        .or_else(|| menu_config.as_ref().map(|mc| mc.should_focus_wrap))
        .unwrap_or(true);

    let resolved_auto_focus = auto_focus
        .or_else(|| menu_config.as_ref().map(|mc| mc.auto_focus))
        .unwrap_or_else(|| Signal::derive(|| None));

    let resolved_select_on_focus = select_on_focus
        .or_else(|| menu_config.as_ref().map(|mc| mc.select_on_focus))
        .unwrap_or(false);

    let resolved_aria_label = aria_label;
    let resolved_aria_labelledby =
        aria_labelledby.or_else(|| menu_config.as_ref().map(|mc| mc.aria_labelledby.clone()));

    let resolved_get_text_value =
        get_text_value.or_else(|| menu_config.as_ref().and_then(|mc| mc.get_text_value));

    let resolved_on_close = on_close.or_else(|| menu_config.as_ref().and_then(|mc| mc.on_close));

    let resolved_should_select_on_press_up = should_select_on_press_up
        .or_else(|| menu_config.as_ref().map(|mc| mc.should_select_on_press_up))
        .unwrap_or(false);

    let resolved_should_focus_on_hover = should_focus_on_hover
        .or_else(|| menu_config.as_ref().map(|mc| mc.should_focus_on_hover))
        .unwrap_or(false);

    let resolved_should_use_virtual_focus = should_use_virtual_focus
        .or_else(|| menu_config.as_ref().map(|mc| mc.should_use_virtual_focus))
        .unwrap_or(false);

    let collection_ref = CapturedElement::new();

    let UseListBoxReturn {
        listbox_props,
        state,
        id: _,
    } = use_listbox(UseListBoxInput {
        selection_mode: resolved_selection_mode,
        selection_behavior: resolved_selection_behavior,
        is_disabled: resolved_disabled,
        selected_keys: resolved_selected_keys,
        default_selected_keys,
        on_selection_change: resolved_on_selection_change,
        disabled_keys: resolved_disabled_keys,
        disallow_empty_selection: resolved_disallow_empty_selection,
        items: resolved_items,
        should_focus_wrap: resolved_should_focus_wrap,
        auto_focus: resolved_auto_focus,
        select_on_focus: resolved_select_on_focus,
        aria_label: resolved_aria_label,
        aria_labelledby: resolved_aria_labelledby,
        get_text_value: resolved_get_text_value,
        is_virtualized: false,
        orientation: ListBoxOrientation::Vertical,
        collection_ref,
        on_close: resolved_on_close,
        escape_key_behavior: resolved_escape_key_behavior,
    });

    // Extract blur handler and focused key setter before consuming menu_config.
    let on_blur_handler = menu_config.as_ref().map(|mc| mc.on_blur.clone());
    let menu_set_focused_key = menu_config.as_ref().map(|mc| mc.set_focused_key);

    // Wire focused key back to Select when in composition mode.
    if let Some(set_focused_key) = menu_set_focused_key {
        let listbox_focused_key = state.collection.selection_state.focused_key;
        Effect::new(move |_| {
            set_focused_key.set(listbox_focused_key.get());
        });
    }

    // Provide context for ListBoxItem children.
    let ctx = ListBoxCtx {
        selection_state: state.collection.selection_state,
        focused_key: state.collection.selection_state.focused_key,
        should_select_on_press_up: resolved_should_select_on_press_up,
        should_use_virtual_focus: resolved_should_use_virtual_focus,
        should_focus_on_hover: resolved_should_focus_on_hover,
    };

    let listbox_attrs = listbox_props.into_attrs();
    let collection_attr = collection_ref.attr();

    if let Some(blur_handler) = on_blur_handler {
        view! {
            <Provider value=ctx>
                <div
                    {..listbox_attrs}
                    {..collection_attr}
                    {..blur_handler.into_on(ev::focusout)}
                    class=classes
                    style=styles
                >
                    {children()}
                </div>
            </Provider>
        }
        .into_any()
    } else {
        view! {
            <Provider value=ctx>
                <div
                    {..listbox_attrs}
                    {..collection_attr}
                    class=classes
                    style=styles
                >
                    {children()}
                </div>
            </Provider>
        }
        .into_any()
    }
}

/// A headless listbox option atom.
///
/// Wraps `use_option` and provides state to child [`ListBoxItemLabel`] and
/// [`ListBoxItemDescription`] atoms via context.
///
/// Exposes `data-selected`, `data-focused`, `data-disabled`, `data-pressed`
/// attributes for CSS styling.
#[component]
pub fn ListBoxItem<K: SelectionKey>(
    /// The unique key for this option.
    #[prop(into)]
    key: K,
    /// Whether this option is disabled.
    #[prop(into, optional)]
    disabled: Option<Signal<bool>>,
    /// Text value for accessibility (screen reader announcement).
    #[prop(into, optional)]
    text_value: Option<String>,
    /// Callback when the option is pressed/activated.
    #[prop(into, optional)]
    on_press: Option<Callback<()>>,
    /// CSS classes.
    #[prop(into, optional)]
    classes: Classes,
    /// CSS styles.
    #[prop(into, optional)]
    styles: Styles,
    children: Children,
) -> impl IntoView {
    let ctx = expect_context::<ListBoxCtx<K>>();
    let is_disabled = disabled.unwrap_or_else(|| Signal::derive(|| false));

    let UseOptionReturn {
        option_props,
        label_props,
        description_props,
        is_selected,
        is_focused,
        is_disabled: resolved_disabled,
        is_pressed,
        is_focus_visible,
    } = use_option(UseOptionInput {
        key,
        state: ctx.selection_state,
        is_disabled,
        should_select_on_press_up: ctx.should_select_on_press_up,
        should_use_virtual_focus: ctx.should_use_virtual_focus,
        should_focus_on_hover: ctx.should_focus_on_hover,
        on_focus: None,
        on_press,
        text_value,
        focused_key: ctx.focused_key,
    });

    let item_ctx = ListBoxItemCtx {
        label_id: label_props.id,
        description_id: description_props.id,
        is_selected,
        is_focused,
        is_disabled: resolved_disabled,
        is_pressed,
        is_focus_visible,
    };

    let data_selected = Signal::derive(move || is_selected.get().then_some("true"));
    let data_focused = Signal::derive(move || is_focused.get().then_some("true"));
    let data_disabled = Signal::derive(move || resolved_disabled.get().then_some("true"));
    let data_pressed = Signal::derive(move || is_pressed.get().then_some("true"));

    let (option_props, option_styles) = option_props.into_inner();
    let styles = option_styles.merge(styles);

    view! {
        <Provider value=item_ctx>
            <div
                {..option_props.into_attrs()}
                class=classes
                style=styles
                attr:data-selected=data_selected
                attr:data-focused=data_focused
                attr:data-disabled=data_disabled
                attr:data-pressed=data_pressed
            >
                {children()}
            </div>
        </Provider>
    }
}

/// Renders a label element wired to the parent [`ListBoxItem`]'s ARIA label association.
#[component]
pub fn ListBoxItemLabel(
    /// CSS classes.
    #[prop(into, optional)]
    classes: Classes,
    /// CSS styles.
    #[prop(into, optional)]
    styles: Styles,
    children: Children,
) -> impl IntoView {
    let ctx = expect_context::<ListBoxItemCtx>();
    view! {
        <span id=ctx.label_id class=classes style=styles>
            {children()}
        </span>
    }
}

/// Renders a description element wired to the parent [`ListBoxItem`]'s ARIA description.
#[component]
pub fn ListBoxItemDescription(
    /// CSS classes.
    #[prop(into, optional)]
    classes: Classes,
    /// CSS styles.
    #[prop(into, optional)]
    styles: Styles,
    children: Children,
) -> impl IntoView {
    let ctx = expect_context::<ListBoxItemCtx>();
    view! {
        <span id=ctx.description_id class=classes style=styles>
            {children()}
        </span>
    }
}
