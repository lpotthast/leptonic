use std::collections::HashSet;

use leptos::{context::Provider, prelude::*};

use crate::{
    Out,
    hooks::{
        DisabledBehavior, IntoAttrs, ListBoxData, Orientation, SelectionBehavior, SelectionMode,
        UseListBoxInput, UseListBoxReturn, UseListBoxSectionInput, UseListBoxSectionReturn,
        UseOptionInput, UseOptionReturn,
        collections::{
            AutoFocus, CollectionMemo, CollectionOptions, EscapeKeyBehavior, Key, ListLayout,
            ListState, Node, Selection, SelectionOptions, UseListStateInput, use_list_state,
        },
        use_listbox, use_listbox_section, use_option,
    },
    utils::{
        CapturedElement, SlotProps, ValueBinding, classes::Classes, data_attributes::flag,
        styles::Styles,
    },
};

/// Provided by components that render a [`ListBox`] for their own options (a select's or combo
/// box's popover): the listbox then uses this configuration instead of its props.
#[derive(Clone, Copy)]
pub struct ListBoxParent {
    pub input: StoredValue<UseListBoxInput>,
}

/// Context from [`ListBoxItem`] to [`ListBoxItemLabel`] and [`ListBoxItemDescription`].
#[derive(Debug, Clone)]
pub struct ListBoxItemCtx {
    label_props: StoredValue<Option<SlotProps>>,
    description_props: StoredValue<Option<SlotProps>>,
    pub is_selected: Signal<bool>,
    pub is_focused: Signal<bool>,
    pub is_focus_visible: Signal<bool>,
    pub is_disabled: Signal<bool>,
    pub is_pressed: Signal<bool>,
}

/// A headless listbox: a list of options to select one or more from.
///
/// The options come from `collection`: render one [`ListBoxItem`] (or [`ListBoxSection`]) per
/// collection entry, in collection order. Inside a [`Select`](super::select::Select) or
/// [`ComboBox`](super::combobox::ComboBox) (see [`ListBoxParent`]), the
/// listbox shows the parent's options with the parent's settings; the props below are then
/// ignored.
///
/// ```ignore
/// let fruits = use_list_collection(Signal::stored(vec!["Apple", "Banana"]), |f| Key::from(*f), |f| f.to_string());
/// view! {
///     <ListBox collection=fruits selection_mode=SelectionMode::Multiple aria_label="Fruits">
///         <ListBoxItem key="Apple">"Apple"</ListBoxItem>
///         <ListBoxItem key="Banana">"Banana"</ListBoxItem>
///     </ListBox>
/// }
/// ```
#[component]
#[allow(
    clippy::too_many_lines,
    clippy::fn_params_excessive_bools,
    clippy::implicit_hasher
)]
pub fn ListBox(
    /// The options. Required outside of a select.
    #[prop(into, optional)]
    collection: Option<CollectionMemo>,
    /// Use an existing list state instead of creating one from `collection` and the selection
    /// props.
    #[prop(optional)]
    state: Option<ListState>,
    #[prop(into, optional)] selection_mode: Signal<SelectionMode>,
    #[prop(optional)] selection_behavior: SelectionBehavior,
    /// The initially selected keys.
    #[prop(into, optional)]
    default_selected_keys: Vec<Key>,
    /// The selection (controlled), replacing `default_selected_keys`: a value or any signal.
    #[prop(into, optional)]
    selection: Option<Signal<Selection>>,
    /// Receives the new state: an `RwSignal`, `WriteSignal`, closure, `Callback`, ...
    #[prop(into, optional)]
    set_selection: Option<Out<Selection>>,
    #[prop(into, optional)] on_selection_change: Option<Callback<Selection>>,
    #[prop(into, optional)] disabled_keys: Option<Signal<HashSet<Key>>>,
    #[prop(optional)] disabled_behavior: DisabledBehavior,
    #[prop(optional)] disallow_empty_selection: bool,
    #[prop(into, optional)] aria_label: MaybeProp<String>,
    #[prop(into, optional)] aria_labelledby: Option<String>,
    #[prop(default = Orientation::Vertical)] orientation: Orientation,
    #[prop(optional)] layout: ListLayout,
    /// Arrow keys wrap around at the ends.
    #[prop(optional)]
    should_focus_wrap: bool,
    /// Focus an item when the listbox mounts.
    #[prop(optional)]
    auto_focus: Option<AutoFocus>,
    #[prop(optional)] escape_key_behavior: EscapeKeyBehavior,
    /// Called with the key of an activated option.
    #[prop(into, optional)]
    on_action: Option<Callback<Key>>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let (selection, on_selection_change) =
        ValueBinding::from_state_props(selection, set_selection, on_selection_change);
    let element = CapturedElement::new();
    let input = if let Some(parent) = use_context::<ListBoxParent>() {
        parent.input.get_value()
    } else {
        let state = state.unwrap_or_else(|| {
            let collection = collection.unwrap_or_else(|| {
                crate::utils::dev_warn!("ListBox: no `collection` given (and not inside a Select)");
                Memo::new(|_| std::sync::Arc::default())
            });
            use_list_state(UseListStateInput {
                collection,
                selection: SelectionOptions {
                    selection_mode,
                    selection_behavior,
                    default_selection: Selection::keys(default_selected_keys),
                    selection,
                    on_selection_change,
                    disallow_empty_selection: Signal::stored(disallow_empty_selection),
                    disabled_keys: disabled_keys.unwrap_or_default(),
                    disabled_behavior,
                    ..SelectionOptions::default()
                },
            })
        });
        UseListBoxInput {
            aria_label,
            aria_labelledby: Signal::stored(aria_labelledby),
            orientation,
            layout,
            options: CollectionOptions {
                auto_focus: Signal::stored(auto_focus),
                should_focus_wrap,
                escape_key_behavior,
                ..CollectionOptions::default()
            },
            on_action,
            ..UseListBoxInput::new(state, element)
        }
    };

    let UseListBoxReturn { props, data } = use_listbox(input);

    view! {
        <Provider value=data>
            <div {..props.into_attrs()} class=classes style=styles>
                {children()}
            </div>
        </Provider>
    }
}

/// An option of a [`ListBox`], for the collection item `key`.
///
/// Exposes `data-selected`, `data-focused`, `data-focus-visible`, `data-disabled` and
/// `data-pressed` for styling.
#[component]
pub fn ListBoxItem(
    /// The item's key in the listbox's collection.
    #[prop(into)]
    key: Key,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let list = expect_context::<ListBoxData>();
    let UseOptionReturn {
        props,
        label_props,
        description_props,
        is_selected,
        is_focused,
        is_focus_visible,
        is_disabled,
        is_pressed,
        ..
    } = use_option(UseOptionInput { list, key });

    let ctx = ListBoxItemCtx {
        label_props: StoredValue::new(Some(label_props)),
        description_props: StoredValue::new(Some(description_props)),
        is_selected,
        is_focused,
        is_focus_visible,
        is_disabled,
        is_pressed,
    };
    let (attrs, option_styles) = props.into_parts();
    let styles = option_styles.merge(styles);

    view! {
        <Provider value=ctx>
            <div
                {..attrs}
                class=classes
                style=styles
                data-selected=flag(is_selected)
                data-focused=flag(is_focused)
                data-focus-visible=flag(is_focus_visible)
                data-disabled=flag(is_disabled)
                data-pressed=flag(is_pressed)
            >
                {children()}
            </div>
        </Provider>
    }
}

/// One [`ListBoxItem`] per item of the listbox's collection, as it currently is (e.g. filtered
/// by a combo box's input), rendered by `children`.
///
/// ```ignore
/// <ListBox>
///     <ListBoxItems let:node>{node.text_value.to_string()}</ListBoxItems>
/// </ListBox>
/// ```
#[component]
pub fn ListBoxItems<F, IV>(
    /// Renders an item's content.
    children: F,
    /// CSS classes of each item.
    #[prop(into, optional)]
    classes: Classes,
) -> impl IntoView
where
    F: Fn(Node) -> IV + Send + Sync + 'static,
    IV: IntoView + 'static,
{
    let list = expect_context::<ListBoxData>();
    let collection = list.state.collection;
    let children = std::sync::Arc::new(children);
    view! {
        <For
            each=move || collection.with(|c| c.items().cloned().collect::<Vec<_>>())
            key=|node| node.key.clone()
            let:node
        >
            {
                let children = children.clone();
                let key = node.key.clone();
                view! { <ListBoxItem key=key classes=classes.clone()>{children(node)}</ListBoxItem> }
            }
        </For>
    }
}

/// The main text of a [`ListBoxItem`] (labels the option).
#[component]
pub fn ListBoxItemLabel(
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let ctx = expect_context::<ListBoxItemCtx>();
    slot(
        ctx.label_props,
        "ListBoxItemLabel",
        classes,
        styles,
        children,
    )
}

/// Secondary text of a [`ListBoxItem`] (describes the option).
#[component]
pub fn ListBoxItemDescription(
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let ctx = expect_context::<ListBoxItemCtx>();
    slot(
        ctx.description_props,
        "ListBoxItemDescription",
        classes,
        styles,
        children,
    )
}

/// Renders a label/description slot. The slot's props go to the first such element only.
fn slot(
    props: StoredValue<Option<SlotProps>>,
    component: &str,
    classes: Classes,
    styles: Styles,
    children: Children,
) -> AnyView {
    if let Some(props) = props.try_update_value(Option::take).flatten() {
        view! {
            <span {..props.into_attrs()} class=classes style=styles>
                {children()}
            </span>
        }
        .into_any()
    } else {
        crate::utils::dev_warn!("{component}: only one per ListBoxItem is supported");
        view! {
            <span class=classes style=styles>
                {children()}
            </span>
        }
        .into_any()
    }
}

/// A group of options in a [`ListBox`], for the collection section `key`. Renders the
/// section's header (if the collection has one) followed by the children.
#[component]
pub fn ListBoxSection(
    /// The section's key in the listbox's collection.
    #[prop(into)]
    key: Key,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    #[prop(into, optional)] heading_classes: Classes,
    children: Children,
) -> impl IntoView {
    let list = expect_context::<ListBoxData>();
    let UseListBoxSectionReturn {
        item_props,
        heading_props,
        group_props,
        heading,
    } = use_listbox_section(UseListBoxSectionInput { list, key });

    view! {
        <div {..item_props.into_attrs()}>
            {heading_props
                .map(|props| {
                    view! {
                        <div {..props.into_attrs()} class=heading_classes>
                            {heading}
                        </div>
                    }
                })}
            <div {..group_props.into_attrs()} class=classes style=styles>
                {children()}
            </div>
        </div>
    }
}
