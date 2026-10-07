use std::collections::HashSet;

use leptos::{context::Provider, prelude::*};

use crate::{
    Out,
    hooks::{
        DisabledBehavior, FocusMode, GridListData, IntoAttrs, KeyboardNavigationBehavior,
        SelectionBehavior, SelectionMode, UseFocusRingInput, UseGridListInput,
        UseGridListItemInput, UseGridListItemReturn, UseGridListReturn,
        collections::{
            AutoFocus, CollectionMemo, CollectionOptions, EscapeKeyBehavior, Key, ListLayout,
            Selection, SelectionOptions, UseListStateInput, use_list_state,
        },
        use_focus_ring, use_grid_list, use_grid_list_item,
    },
    utils::{
        CapturedElement, ValueBinding, classes::Classes, data_attributes::flag,
        default_class::with_default_class, styles::Styles,
    },
};

/// A headless grid list: a list of interactive rows that can be selected and navigated like a
/// listbox, and may contain buttons, checkboxes or links.
///
/// The rows come from `collection`: render one [`GridListItem`] per collection item, in
/// collection order.
///
/// Data attributes (as react-aria-components): `data-empty`, `data-focused`, `data-focus-visible`,
/// `data-layout` (`stack`/`grid`).
///
/// Default class: `leptonic-GridList`.
#[component]
#[allow(
    clippy::too_many_lines,
    clippy::fn_params_excessive_bools,
    clippy::implicit_hasher
)]
pub fn GridList(
    /// The rows.
    #[prop(into)]
    collection: CollectionMemo,
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
    let classes = with_default_class("leptonic-GridList", classes);
    let (selection, on_selection_change) =
        ValueBinding::from_state_props(selection, set_selection, on_selection_change);
    let state = {
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
    };

    // As react-aria-components: the layout, emptiness and focus for styling.
    let data_layout = match layout {
        ListLayout::Stack => "stack",
        ListLayout::Grid => "grid",
    };
    let collection = state.collection;
    let is_empty = Signal::derive(move || collection.with(|c| c.items().next().is_none()));
    let focus_ring = use_focus_ring(UseFocusRingInput::default());

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
        state,
        element: CapturedElement::new(),
        id: None,
        keyboard_delegate: None,
        should_select_on_press_up: false,
        tree: None,
    });

    view! {
        <Provider value=data>
            <div
                {..props.into_attrs()}
                {..focus_ring.props.into_attrs()}
                class=classes
                style=styles
                data-empty=flag(is_empty)
                data-focused=flag(focus_ring.is_focused)
                data-focus-visible=flag(focus_ring.is_focus_visible)
                data-layout=data_layout
            >
                {children()}
            </div>
        </Provider>
    }
}

/// A row of a [`GridList`], for the collection item `key`: an outer `role="row"` element with a
/// single `role="gridcell"` (`display: contents`, so the row lays out the children) holding the
/// children.
///
/// Exposes `data-selected`, `data-focused`, `data-focus-visible`, `data-disabled` and
/// `data-pressed` on the row for styling.
///
/// Default class: `leptonic-GridListItem`.
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
    let classes = with_default_class("leptonic-GridListItem", classes);
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
    } = use_grid_list_item(UseGridListItemInput {
        // Inside a `ContextMenuTrigger`: its menu opens on this row.
        on_context_menu: super::menu::ContextMenuTargetContext::for_item(&key),
        list,
        key,
        focus_mode: FocusMode::Row,
        allows_arrow_navigation: false,
    });
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
            <div {..grid_cell_props.into_attrs()} style="display: contents">
                {children()}
            </div>
        </div>
    }
}
