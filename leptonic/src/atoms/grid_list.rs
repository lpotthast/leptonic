use std::collections::HashSet;

use leptos::{context::Provider, prelude::*};

use crate::{
    Out,
    hooks::{
        DisabledBehavior, GridListData, IntoAttrs, KeyboardNavigationBehavior, SelectionBehavior,
        SelectionMode, UseGridListInput, UseGridListItemInput, UseGridListItemReturn,
        UseGridListReturn,
        collections::{
            AutoFocus, CollectionMemo, CollectionOptions, EscapeKeyBehavior, Key, ListLayout,
            ListState, Selection, SelectionOptions, UseListStateInput, use_list_state,
        },
        use_grid_list, use_grid_list_item,
    },
    utils::{
        CapturedElement, ValueBinding, classes::Classes, data_attributes::flag, styles::Styles,
    },
};

/// A headless grid list: a list of interactive rows that can be selected and navigated like a
/// listbox, and may contain buttons, checkboxes or links.
///
/// The rows come from `collection`: render one [`GridListItem`] per collection item, in
/// collection order.
#[component]
#[allow(
    clippy::too_many_lines,
    clippy::fn_params_excessive_bools,
    clippy::implicit_hasher
)]
pub fn GridList(
    /// The rows. Required unless `state` is given.
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
    #[prop(optional)] layout: ListLayout,
    #[prop(optional)] keyboard_navigation_behavior: KeyboardNavigationBehavior,
    /// Arrow keys wrap around at the ends.
    #[prop(optional)]
    should_focus_wrap: bool,
    /// Focus a row when the grid list mounts.
    #[prop(optional)]
    auto_focus: Option<AutoFocus>,
    #[prop(optional)] escape_key_behavior: EscapeKeyBehavior,
    /// Called with the key of an activated row.
    #[prop(into, optional)]
    on_action: Option<Callback<Key>>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let (selection, on_selection_change) =
        ValueBinding::from_state_props(selection, set_selection, on_selection_change);
    let state = state.unwrap_or_else(|| {
        let collection = collection.unwrap_or_else(|| {
            crate::utils::dev_warn!("GridList: neither `collection` nor `state` given");
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

    let UseGridListReturn { props, data } = use_grid_list(UseGridListInput {
        aria_label,
        aria_labelledby: Signal::stored(aria_labelledby),
        layout,
        keyboard_navigation_behavior,
        options: CollectionOptions {
            auto_focus: Signal::stored(auto_focus),
            should_focus_wrap,
            escape_key_behavior,
            ..CollectionOptions::default()
        },
        on_action,
        ..UseGridListInput::new(state, CapturedElement::new())
    });

    view! {
        <Provider value=data>
            <div {..props.into_attrs()} class=classes style=styles>
                {children()}
            </div>
        </Provider>
    }
}

/// A row of a [`GridList`], for the collection item `key`: an outer `role="row"` element with a
/// single `role="gridcell"` holding the children.
///
/// Exposes `data-selected`, `data-focused`, `data-focus-visible`, `data-disabled` and
/// `data-pressed` on the row for styling.
#[component]
pub fn GridListItem(
    /// The row's key in the grid list's collection.
    #[prop(into)]
    key: Key,
    /// CSS classes (of the row element).
    #[prop(into, optional)]
    classes: Classes,
    /// CSS styles (of the row element).
    #[prop(into, optional)]
    styles: Styles,
    children: Children,
) -> impl IntoView {
    let list = expect_context::<GridListData>();
    let UseGridListItemReturn {
        row_props,
        grid_cell_props,
        is_selected,
        is_focused,
        is_focus_visible,
        is_disabled,
        is_pressed,
        ..
    } = use_grid_list_item(UseGridListItemInput::new(list, key));
    let (attrs, row_styles) = row_props.into_parts();
    let styles = row_styles.merge(styles);

    view! {
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
            <div {..grid_cell_props.into_attrs()}>{children()}</div>
        </div>
    }
}
