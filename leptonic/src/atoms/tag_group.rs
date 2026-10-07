//! Headless tag groups: a focusable list of tags (labels, categories, filters) to navigate,
//! select and remove.
// Upstream: react-aria-components/src/TagGroup.tsx @ 99e6102368
use std::{collections::HashSet, sync::Arc};

use leptos::{context::Provider, prelude::*};

use crate::{
    Out,
    hooks::{
        DisabledBehavior, IntoAttrs, SelectionBehavior, SelectionMode, TagGroupData,
        UseButtonInput, UseFocusRingInput, UseHoverInput, UseTagGroupInput, UseTagGroupProps,
        UseTagGroupReturn, UseTagInput, UseTagReturn,
        collections::{
            CollectionMemo, Key, Node, Selection, SelectionOptions, UseListStateInput,
            use_list_state,
        },
        use_button, use_focus_ring, use_hover, use_tag, use_tag_group,
    },
    utils::{
        ValueBinding, classes::Classes, data_attributes::flag, default_class::with_default_class,
        styles::Styles,
    },
};

use super::field::{FieldContext, LabelContext, LabelPresence};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - The tags come from `collection` (as all collection atoms): render one [`Tag`] per item, or
//   [`TagItems`] (react-aria-components: the collection is built from the rendered `Tag`s).
// - The remove button is the [`TagRemoveButton`] atom (react-aria-components: a `Button` with
//   `slot="remove"`). Its accessible name is "Remove" plus the tag's.
// - `TagList`'s `renderEmptyState` is `empty_state`, shown while the collection is empty.
//
// ## OMITTED FEATURES
// - Filtering through an `Autocomplete` (`SelectableCollectionContext`), shared element
//   transitions, virtualization (`TagItems` renders every tag).
// - Per-tag props: `isDisabled` (use the group's `disabled_keys`), `textValue` (the collection's
//   text value), `onAction` (the group's `on_action`), links (`href`), hover events.
// - The group's `escapeKeyBehavior` and `shouldSelectOnPressUp`.
//
// =============================================================================

/// What a [`TagGroup`] provides to its [`TagList`].
#[derive(Clone, Copy)]
struct TagListCtx {
    props: StoredValue<Option<UseTagGroupProps>>,
    data: StoredValue<TagGroupData>,
}

/// What a [`Tag`] provides to its [`TagRemoveButton`].
#[derive(Clone)]
struct TagRemoveCtx {
    button: StoredValue<Option<UseButtonInput>>,
}

/// A headless tag group: a [`Label`](super::field::Label), a [`TagList`] with the tags, and
/// optionally a [`Description`](super::field::Description).
///
/// With `on_remove`, tags can be removed with Delete/Backspace (all selected ones if the focused
/// tag is selected) or their [`TagRemoveButton`]; remove the keys from your data.
///
/// ```ignore
/// let tags = RwSignal::new(vec!["News", "Travel", "Gaming"]);
/// let collection = use_list_collection(tags.into(), |t| Key::from(*t), |t| t.to_string());
/// view! {
///     <TagGroup collection=collection on_remove=move |keys: HashSet<Key>| tags.update(|t| t.retain(|t| !keys.contains(&Key::from(*t))))>
///         <Label>"Categories"</Label>
///         <TagList>
///             <TagItems let:node>
///                 {node.text_value.to_string()}
///                 <TagRemoveButton>"×"</TagRemoveButton>
///             </TagItems>
///         </TagList>
///     </TagGroup>
/// }
/// ```
///
/// Default class: `leptonic-TagGroup`.
#[component]
#[allow(clippy::too_many_lines, clippy::implicit_hasher)]
pub fn TagGroup(
    /// The tags.
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
    #[prop(into, optional)] aria_describedby: Option<String>,
    /// Called with the keys to remove; enables removing tags.
    #[prop(into, optional)]
    on_remove: Option<Callback<HashSet<Key>>>,
    /// Called with the key of an activated tag.
    #[prop(into, optional)]
    on_action: Option<Callback<Key>>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let classes = with_default_class("leptonic-TagGroup", classes);
    let (selection, on_selection_change) =
        ValueBinding::from_state_props(selection, set_selection, on_selection_change);
    let state = use_list_state(UseListStateInput {
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
    });
    // As in react-aria-components: a visible label is expected unless an ARIA label is given.
    let label_presence = LabelPresence::new(aria_label, aria_labelledby.as_ref());
    let UseTagGroupReturn {
        grid_props,
        label_props,
        description_props,
        error_message_props,
        data,
    } = use_tag_group(UseTagGroupInput {
        state,
        element: crate::utils::CapturedElement::new(),
        id: None,
        has_label: label_presence.has_label,
        aria_label,
        aria_labelledby,
        aria_describedby,
        keyboard_delegate: None,
        on_remove,
        on_action,
    });

    provide_context(LabelContext::span(label_props).with_presence(label_presence));
    provide_context(FieldContext {
        description: description_props,
        error_message: error_message_props,
        is_invalid: Signal::stored(false),
        validation_errors: Signal::stored(Vec::new()),
        validation_details: Signal::stored(crate::hooks::ValidityStateSnapshot::default()),
    });
    let list = TagListCtx {
        props: StoredValue::new(Some(grid_props)),
        data: StoredValue::new(data),
    };

    view! {
        <Provider value=list>
            <div class=classes style=styles>
                {children()}
            </div>
        </Provider>
    }
}

/// The list of tags of a [`TagGroup`] (the `role="grid"` element).
///
/// Data attributes: `data-empty`, `data-focused`, `data-focus-visible`.
///
/// Default class: `leptonic-TagList`.
#[component]
pub fn TagList(
    /// Shown while the group has no tags.
    #[prop(into, optional)]
    empty_state: Option<ViewFn>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let classes = with_default_class("leptonic-TagList", classes);
    let Some(ctx) = use_context::<TagListCtx>() else {
        crate::utils::dev_warn!("A <TagList> must be inside a <TagGroup>.");
        return None;
    };
    let Some(props) = ctx.props.try_update_value(Option::take).flatten() else {
        crate::utils::dev_warn!("A <TagGroup> has one <TagList>.");
        return None;
    };
    let data = ctx.data.get_value();
    let collection = data.list.state.collection;
    let is_empty = Signal::derive(move || collection.with(|c| c.items().next().is_none()));
    let focus_ring = use_focus_ring(UseFocusRingInput::default());
    let empty = move || {
        is_empty
            .get()
            .then(|| empty_state.as_ref().map(ViewFn::run))
            .flatten()
    };

    Some(view! {
        <Provider value=data>
            <div
                {..props.into_attrs()}
                {..focus_ring.props.into_attrs()}
                class=classes
                style=styles
                data-empty=flag(is_empty)
                data-focused=flag(focus_ring.is_focused)
                data-focus-visible=flag(focus_ring.is_focus_visible)
            >
                {children()}
                {empty}
            </div>
        </Provider>
    })
}

/// A tag of a [`TagGroup`], for the collection item `key`: a `role="row"` element with a single
/// `role="gridcell"` (`display: contents`) holding the children.
///
/// Data attributes: `data-selected`, `data-disabled`, `data-hovered`, `data-focused`,
/// `data-focus-visible`, `data-pressed`, `data-allows-removing`, `data-selection-mode`
/// (`single`/`multiple`, absent without selection).
///
/// Default class: `leptonic-Tag`.
#[component]
pub fn Tag(
    /// The tag's key in the group's collection.
    #[prop(into)]
    key: Key,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let classes = with_default_class("leptonic-Tag", classes);
    let group = expect_context::<TagGroupData>();
    let selection = group.list.state.selection;
    let has_action = group.list.on_action.is_some();
    let UseTagReturn {
        row_props,
        grid_cell_props,
        remove_button,
        is_selected,
        is_focused,
        is_focus_visible,
        is_disabled,
        is_pressed,
        allows_selection,
        allows_removing,
    } = use_tag(UseTagInput { group, key });
    let hover = use_hover(UseHoverInput {
        is_disabled: Signal::derive(move || !allows_selection.get() && !has_action),
        ..UseHoverInput::default()
    });
    let (attrs, row_styles) = row_props.into_parts();
    let styles = row_styles.merge(styles);
    let selection_mode = move || match selection.selection_mode() {
        SelectionMode::None => None,
        SelectionMode::Single => Some("single"),
        SelectionMode::Multiple => Some("multiple"),
    };
    let remove = TagRemoveCtx {
        button: StoredValue::new(remove_button),
    };

    view! {
        <div
            {..attrs}
            {..hover.props.into_attrs()}
            class=classes
            style=styles
            data-selected=flag(is_selected)
            data-disabled=flag(is_disabled)
            data-hovered=flag(hover.is_hovered)
            data-focused=flag(is_focused)
            data-focus-visible=flag(is_focus_visible)
            data-pressed=flag(is_pressed)
            data-allows-removing=allows_removing.then_some("true")
            data-selection-mode=selection_mode
        >
            <div {..grid_cell_props.into_attrs()} style="display: contents">
                <Provider value=remove>{children()}</Provider>
            </div>
        </div>
    }
}

/// One [`Tag`] per item of the group's collection, rendered by `children`.
///
/// ```ignore
/// <TagList>
///     <TagItems let:node>{node.text_value.to_string()}</TagItems>
/// </TagList>
/// ```
#[component]
pub fn TagItems<F, IV>(
    /// Renders a tag's content.
    children: F,
    /// CSS classes of each tag.
    #[prop(into, optional)]
    classes: Classes,
) -> impl IntoView
where
    F: Fn(Node) -> IV + Send + Sync + 'static,
    IV: IntoView + 'static,
{
    let group = expect_context::<TagGroupData>();
    let collection = group.list.state.collection;
    let children = Arc::new(children);
    view! {
        <For
            each=move || collection.with(|c| c.items().cloned().collect::<Vec<_>>())
            key=|node| node.key.clone()
            let:node
        >
            {
                let children = children.clone();
                let key = node.key.clone();
                view! { <Tag key=key classes=classes.clone()>{children(node)}</Tag> }
            }
        </For>
    }
}

/// The button removing its [`Tag`] (rendered only when the group has `on_remove`). Keyboard
/// users can also press Delete or Backspace on the tag.
///
/// Data attributes: `data-hovered`, `data-pressed`, `data-focus-visible`.
///
/// Default class: `leptonic-TagRemoveButton`.
#[component]
pub fn TagRemoveButton(
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    /// The button's content, e.g. "×" or an icon (its accessible name is "Remove" and the tag's).
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    let classes = with_default_class("leptonic-TagRemoveButton", classes);
    let ctx = expect_context::<TagRemoveCtx>();
    let input = ctx.button.get_value()?;
    let button = use_button(input);
    let (attrs, button_styles) = button.props.into_parts();
    let styles = button_styles.merge(styles);
    Some(view! {
        <button
            {..attrs}
            class=classes
            style=styles
            data-hovered=flag(button.is_hovered)
            data-pressed=flag(button.is_pressed)
        >
            {children.map(|children| children())}
        </button>
    })
}
