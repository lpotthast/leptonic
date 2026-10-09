// Upstream: react-aria-components/src/GridList.tsx @ 99e6102368
// Upstream: react-aria-components/test/GridList.test.js @ 99e6102368
// Upstream: react-aria-components/test/GridList.browser.test.tsx @ 99e6102368
use std::collections::HashSet;

use leptos::{context::Provider, prelude::*};
use leptos_classes::Classes;

use crate::{
    CapturedElement, IntoAttrs, Out, SlotProps, ValueBinding,
    hooks::{
        collections::{
            AutoFocus, CollectionMemo, CollectionOptions, DisabledBehavior, EscapeKeyBehavior, Key,
            ListLayout, Selection, SelectionBehavior, SelectionMode, SelectionOptions,
            UseListStateInput, use_list_state,
        },
        focus::{UseFocusRingInput, use_focus_ring},
        gridlist::{
            FocusMode, GridListData, KeyboardNavigationBehavior, UseGridListInput,
            UseGridListItemInput, UseGridListItemReturn, UseGridListReturn,
            UseGridListSectionInput, UseGridListSectionReturn, UseGridListSectionRowHeaderProps,
            UseGridListSectionRowProps, use_grid_list, use_grid_list_item, use_grid_list_section,
        },
        interactions::{UseHoverInput, use_hover},
    },
    utils::{data_attributes::flag, default_class::with_default_class, styles::Styles},
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - The rows come from a `collection` (`CollectionMemo`); render one `GridListItem` per item, in
//   collection order (react-aria-components builds the collection from the children).
// - Section labels come from the collection (the section's header and `aria_label`).
//
// ## OMITTED FEATURES
// - Virtualization, drag and drop, links rendered as `<a>`, `renderEmptyState`, load more.
//
// =============================================================================

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
    /// How pressing an item changes the selection; a change applies right away.
    #[prop(into, optional)]
    selection_behavior: Signal<SelectionBehavior>,
    /// The initially selected keys.
    #[prop(into, optional)]
    default_selection: Selection,
    /// The selection (controlled), replacing `default_selection`: a value or any signal.
    #[prop(into, optional)]
    selection: Option<Signal<Selection>>,
    /// Receives the new state: an `RwSignal`, `WriteSignal`, closure, `Callback`, ...
    #[prop(into, optional)]
    set_selection: Option<Out<Selection>>,
    #[prop(into, optional)] on_selection_change: Option<Callback<Selection>>,
    #[prop(into, optional)] disabled_keys: Option<Signal<HashSet<Key>>>,
    #[prop(optional)] disabled_behavior: DisabledBehavior,
    #[prop(into, optional)] disallow_empty_selection: Signal<bool>,
    #[prop(into, optional)] aria_label: MaybeProp<String>,
    /// Ids of elements labelling it.
    #[prop(into, optional)]
    aria_labelledby: MaybeProp<String>,
    #[prop(optional)] layout: ListLayout,
    /// How the keyboard reaches the rows' interactive children. A grid layout always uses
    /// `KeyboardNavigationBehavior::Tab` (the arrow keys move between rows in two dimensions).
    #[prop(optional)]
    keyboard_navigation_behavior: KeyboardNavigationBehavior,
    /// Select rows when a press ends instead of when it starts (e.g. for draggable rows).
    #[prop(optional)]
    should_select_on_press_up: bool,
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
                default_selection,
                selection,
                on_selection_change,
                disallow_empty_selection,
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
        aria_labelledby: Signal::derive(move || aria_labelledby.get()),
        layout,
        // As react-aria-components: the arrow keys move between the rows of a grid layout.
        keyboard_navigation_behavior: match layout {
            ListLayout::Grid => KeyboardNavigationBehavior::Tab,
            ListLayout::Stack => keyboard_navigation_behavior,
        },
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
        should_select_on_press_up,
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
/// Exposes `data-selected`, `data-focused`, `data-focus-visible`, `data-disabled`,
/// `data-pressed` and `data-hovered` on the row for styling. A [`GridListItemDescription`] inside describes the row.
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
    /// What gets focus when the row is focused: the row, or its first focusable child.
    #[prop(optional)]
    focus_mode: FocusMode,
    /// Let ArrowUp/ArrowDown move between rows while a child has focus, also with
    /// `KeyboardNavigationBehavior::Tab`.
    #[prop(optional)]
    allows_arrow_navigation: bool,
    children: Children,
) -> impl IntoView {
    let classes = with_default_class("leptonic-GridListItem", classes);
    let Some(list) = use_context::<GridListData>() else {
        crate::utils::dev_warn!("a <GridListItem> belongs in a <GridList>");
        return ().into_any();
    };
    let UseGridListItemReturn {
        row_props,
        grid_cell_props,
        description_props,
        is_selected,
        is_focused,
        is_focus_visible,
        is_disabled,
        is_pressed,
        allows_selection,
        has_action,
        ..
    } = use_grid_list_item(UseGridListItemInput {
        // Inside a `ContextMenuTrigger`: its menu opens on this row.
        on_context_menu: super::menu::ContextMenuTargetContext::for_item(&key),
        list,
        key,
        focus_mode,
        allows_arrow_navigation,
    });
    let (attrs, row_styles) = row_props.into_parts();
    let styles = row_styles.merge(styles);
    let item = GridListItemContext {
        description_props: StoredValue::new(Some(description_props)),
    };
    // Interactive rows show hover (react-aria-components' `GridListItem`).
    let hover = use_hover(UseHoverInput {
        is_disabled: Signal::derive(move || !allows_selection.get() && !has_action.get()),
        ..UseHoverInput::default()
    });

    view! {
        <div
            {..attrs}
            {..hover.props.into_attrs()}
            class=classes
            style=styles
            data-selected=flag(is_selected)
            data-focused=flag(is_focused)
            data-focus-visible=flag(is_focus_visible)
            data-disabled=flag(is_disabled)
            data-pressed=flag(is_pressed)
            data-hovered=flag(hover.is_hovered)
        >
            <div {..grid_cell_props.into_attrs()} style="display: contents">
                <Provider value=item>{children()}</Provider>
            </div>
        </div>
    }
    .into_any()
}

/// What a [`GridListItem`] provides to its [`GridListItemDescription`].
#[derive(Clone, Copy)]
struct GridListItemContext {
    description_props: StoredValue<Option<SlotProps>>,
}

/// Secondary text of a [`GridListItem`] (react-aria-components: `Text slot="description"`): the
/// row is labelled by its text and described by this.
///
/// Default class: `leptonic-GridListItemDescription`.
#[component]
pub fn GridListItemDescription(
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let classes = with_default_class("leptonic-GridListItemDescription", classes);
    let Some(ctx) = use_context::<GridListItemContext>() else {
        crate::utils::dev_warn!("a <GridListItemDescription> belongs in a <GridListItem>");
        return ().into_any();
    };
    if let Some(props) = ctx
        .description_props
        .try_update_value(Option::take)
        .flatten()
    {
        view! {
            <span {..props.into_attrs()} class=classes style=styles>
                {children()}
            </span>
        }
        .into_any()
    } else {
        crate::utils::dev_warn!("GridListItemDescription: only one per GridListItem");
        view! {
            <span class=classes style=styles>
                {children()}
            </span>
        }
        .into_any()
    }
}

/// What a [`GridListSection`] provides to its [`GridListHeader`].
#[derive(Clone, Copy)]
struct GridListSectionContext {
    header: StoredValue<Option<(UseGridListSectionRowProps, UseGridListSectionRowHeaderProps)>>,
    /// The collection's header text.
    heading: Signal<Option<String>>,
}

/// A group of rows in a [`GridList`], for the collection section `key`: one `role="rowgroup"`
/// element holding an optional [`GridListHeader`] and the section's rows. It is labelled by the
/// rendered header and the section's `aria_label` in the collection.
///
/// Default class: `leptonic-GridListSection`.
#[component]
pub fn GridListSection(
    /// The section's key in the grid list's collection.
    #[prop(into)]
    key: Key,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let classes = with_default_class("leptonic-GridListSection", classes);
    let Some(list) = use_context::<GridListData>() else {
        crate::utils::dev_warn!("a <GridListSection> belongs in a <GridList>");
        return ().into_any();
    };
    let UseGridListSectionReturn {
        row_props,
        row_header_props,
        row_group_props,
        heading,
    } = use_grid_list_section(UseGridListSectionInput { list, key });
    let ctx = GridListSectionContext {
        header: StoredValue::new(Some((row_props, row_header_props))),
        heading,
    };

    view! {
        <div {..row_group_props.into_attrs()} class=classes style=styles>
            <Provider value=ctx>{children()}</Provider>
        </div>
    }
    .into_any()
}

/// The header row of a [`GridListSection`], labelling it: a `role="row"` element with a
/// `role="rowheader"` cell (`display: contents`). Shows `children`, or else the section's header
/// text from the collection.
///
/// Default class: `leptonic-GridListHeader`.
#[component]
pub fn GridListHeader(
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let classes = with_default_class("leptonic-GridListHeader", classes);
    let Some(ctx) = use_context::<GridListSectionContext>() else {
        crate::utils::dev_warn!("a <GridListHeader> belongs in a <GridListSection>");
        return ().into_any();
    };
    let heading = ctx.heading;
    let content = match children {
        Some(children) => children().into_any(),
        None => (move || heading.get()).into_any(),
    };
    if let Some((row_props, row_header_props)) = ctx.header.try_update_value(Option::take).flatten()
    {
        view! {
            <div {..row_props.into_attrs()} class=classes style=styles>
                <div {..row_header_props.into_attrs()} style="display: contents">
                    {content}
                </div>
            </div>
        }
        .into_any()
    } else {
        crate::utils::dev_warn!("GridListHeader: only one per GridListSection");
        view! { <div class=classes style=styles>{content}</div> }.into_any()
    }
}
