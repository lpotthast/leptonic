// Upstream: react-aria-components/src/ListBox.tsx @ 99e6102368
// Upstream: react-aria-components/test/ListBox.test.js @ 99e6102368
// Upstream: react-aria-components/test/ListBox.browser.test.tsx @ 99e6102368
// Upstream: react-aria-components/test/ListBox.ssr.test.js @ 99e6102368

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - The options come from a `collection` built from data (`use_collection`), rendered by
//   `ListBoxItem`s (or `ListBoxItems`) in collection order, instead of a collection built from
//   the rendered children.
// - State (C4): `default_selection` + `on_selection_change`, or `selection` + `set_selection`.
// - `ListBoxSectionHeading` is the section's `Header`; `ListBoxItemLabel` and
//   `ListBoxItemDescription` are the `Text` slots `label` and `description`.
// - Inside a `Select` or `ComboBox`, the listbox takes the parent's configuration from
//   `ListBoxParent` (react-aria-components: `ListBoxContext` and `ListStateContext`), which it
//   hides from listboxes inside its options.
// - A `ListBox` without a `collection` outside of a `Select` or `ComboBox` warns and renders
//   nothing.
//
// ## OMITTED FEATURES
// - Render props and `className`/`style` functions (styling uses the data attributes).
// - Drag and drop on the atoms (`dragAndDropHooks`), `ListBoxLoadMoreItem`.
//
// =============================================================================

use std::collections::HashSet;

use leptos::{context::Provider, prelude::*};
use leptos_classes::Classes;

use super::separator::SeparatorContext;
use crate::{
    CapturedElement, IntoAttrs, Orientation, Out, SlotProps, ValueBinding,
    hooks::{
        collections::{
            AutoFocus, CollectionMemo, CollectionOptions, DisabledBehavior, EscapeKeyBehavior, Key,
            ListLayout, Node, Selection, SelectionBehavior, SelectionMode, SelectionOptions,
            UseListStateInput, use_list_state,
        },
        focus::{UseFocusRingInput, use_focus_ring},
        listbox::{
            ListBoxData, UseListBoxInput, UseListBoxReturn, UseListBoxSectionHeadingProps,
            UseListBoxSectionInput, UseListBoxSectionReturn, UseOptionInput, UseOptionReturn,
            use_listbox, use_listbox_section, use_option,
        },
        separator::SeparatorElementType,
    },
    utils::{
        data_attributes::flag,
        default_class::with_default_class,
        scoped_context::{clear_context, scoped_view, use_clearable_context},
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
pub struct ListBoxItemContext {
    label_props: StoredValue<Option<SlotProps>>,
    description_props: StoredValue<Option<SlotProps>>,
    pub is_selected: Signal<bool>,
    pub is_focused: Signal<bool>,
    pub is_focus_visible: Signal<bool>,
    pub is_disabled: Signal<bool>,
    pub is_pressed: Signal<bool>,
    pub is_hovered: Signal<bool>,
}

/// A headless listbox: a list of options to select one or more from.
///
/// Data attributes (as react-aria-components): `data-empty`, `data-focused`, `data-focus-visible`,
/// `data-layout` (`stack`/`grid`), `data-orientation`.
///
/// The options come from `collection`: render one [`ListBoxItem`] (or [`ListBoxSection`]) per
/// collection entry, in collection order. Inside a [`Select`](super::select::Select) or
/// [`ComboBox`](super::combobox::ComboBox) (see [`ListBoxParent`]), the
/// listbox shows the parent's options with the parent's settings; the props below are then
/// ignored. A [`Separator`](super::separator::Separator) between options renders a
/// `<div role="separator">` (an `<hr>` can't be inside a listbox).
///
/// Without a `collection` outside of a `Select` or `ComboBox`, it warns (debug builds) and
/// renders nothing.
///
/// ```ignore
/// let fruits = use_list_collection(UseListCollectionInput {
///     items: Signal::stored(vec!["Apple", "Banana"]),
///     key: |f| Key::from(*f),
///     text_value: |f| f.to_string(),
/// });
/// view! {
///     <ListBox collection=fruits selection_mode=SelectionMode::Multiple aria_label="Fruits">
///         <ListBoxItem key="Apple">"Apple"</ListBoxItem>
///         <ListBoxItem key="Banana">"Banana"</ListBoxItem>
///     </ListBox>
/// }
/// ```
///
/// Default class: `leptonic-ListBox`.
#[component]
#[allow(
    clippy::too_many_lines,
    clippy::fn_params_excessive_bools,
    clippy::implicit_hasher
)]
pub fn ListBox(
    /// The options. Required outside of a `Select` or `ComboBox` (which provide theirs).
    #[prop(into, optional)]
    collection: Option<CollectionMemo>,
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
    #[prop(into, optional)] aria_labelledby: Option<String>,
    #[prop(into, default = Orientation::Vertical.into())] orientation: Signal<Orientation>,
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
    /// Shown while there are no options (also inside a `Select` or `ComboBox`), in a
    /// `role="option"` element with `display: contents` (react-aria-components).
    #[prop(into, optional)]
    empty_state: Option<ViewFn>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let classes = with_default_class("leptonic-ListBox", classes);
    let (selection, on_selection_change) =
        ValueBinding::from_state_props(selection, set_selection, on_selection_change);
    let element = CapturedElement::new();
    let mut input = if let Some(parent) = use_clearable_context::<ListBoxParent>() {
        parent.input.get_value()
    } else {
        let Some(collection) = collection else {
            crate::utils::dev_warn!(
                "ListBox: a ListBox outside of a Select or ComboBox needs a `collection`"
            );
            return None;
        };
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
            state,
            element,
            id: None,
            layout_delegate: None,
            is_virtualized: false,
            keyboard_delegate: None,
            should_select_on_press_up: false,
            should_focus_on_hover: false,
            on_focus: None,
            on_blur: None,
            on_focus_change: None,
        }
    };

    // Inside a `Virtualizer`: the listbox scrolls, only the visible options render, the
    // focused one stays.
    let state = input.state;
    let root = super::virtualizer::VirtualizerRenderer::root_for(
        super::virtualizer::VirtualizedRootInput {
            collection: state.collection.into(),
            persisted_keys: Signal::derive(move || {
                state.selection.focused_key().into_iter().collect()
            }),
            element: input.element,
        },
    );
    if let Some(root) = root {
        input.layout_delegate = Some(root.layout_delegate.get_value());
        input.is_virtualized = true;
    }

    // As react-aria-components: the layout, orientation, emptiness and focus for styling.
    let data_layout = match input.layout {
        ListLayout::Stack => "stack",
        ListLayout::Grid => "grid",
    };
    let orientation = input.orientation;
    let data_orientation = move || orientation.get().as_str();
    let collection = state.collection;
    let is_empty = Signal::derive(move || collection.with(|c| c.items().next().is_none()));
    let focus_ring = use_focus_ring(UseFocusRingInput::default());
    let empty = move || {
        let empty_state = empty_state.clone()?;
        is_empty.get().then(|| {
            view! {
                <div role="option" style="display: contents">
                    {empty_state.run()}
                </div>
            }
        })
    };

    let UseListBoxReturn { props, data } = use_listbox(input);
    let styles = match root {
        Some(root) => root.scroll_view_styles.get_value().merge(styles),
        None => styles,
    };

    Some(view! {
        <Provider value=data>
            <Provider value=root>
            // Separators between the options are `<div role="separator">`s (react-aria-components).
            <Provider value=SeparatorContext {
                element_type: SeparatorElementType::Div,
            }>
                <div
                    {..props.into_attrs()}
                    {..focus_ring.props.into_attrs()}
                    class=classes
                    style=styles
                    data-empty=flag(is_empty)
                    data-focused=flag(focus_ring.is_focused)
                    data-focus-visible=flag(focus_ring.is_focus_visible)
                    data-layout=data_layout
                    data-orientation=data_orientation
                >
                    // A ListBox inside the options is not the parent's (react-aria-components:
                    // `ListBox` consumes its context).
                    {scoped_view(clear_context::<ListBoxParent>, children)}
                    {empty}
                </div>
            </Provider>
            </Provider>
        </Provider>
    })
}

/// An option of a [`ListBox`], for the collection item `key`.
///
/// Exposes `data-selected`, `data-focused`, `data-focus-visible`, `data-disabled`,
/// `data-pressed` and `data-hovered` (options that can be selected or have an action) for
/// styling.
///
/// Default class: `leptonic-ListBoxItem`.
#[component]
pub fn ListBoxItem(
    /// The item's key in the listbox's collection.
    #[prop(into)]
    key: Key,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let classes = with_default_class("leptonic-ListBoxItem", classes);
    let Some(list) = use_context::<ListBoxData>() else {
        crate::utils::dev_warn!("<ListBoxItem> must be inside a <ListBox>");
        return None;
    };
    // Without a node, the item can't be focused or selected (e.g. a key of another type than the
    // collection's: `"10"` for the item, `10` in the collection).
    if !list
        .state
        .collection
        .with_untracked(|c| c.contains_key(&key))
    {
        crate::utils::dev_warn!(
            "<ListBoxItem key={key:?}>: the listbox's collection has no item with this key"
        );
    }
    let UseOptionReturn {
        props,
        label_props,
        description_props,
        is_selected,
        is_focused,
        is_focus_visible,
        is_disabled,
        is_pressed,
        is_hovered,
        ..
    } = use_option(UseOptionInput {
        // Inside a `ContextMenuTrigger`: its menu opens on this option.
        on_context_menu: super::menu::ContextMenuTargetContext::for_item(&key),
        list,
        key,
    });

    let ctx = ListBoxItemContext {
        label_props: StoredValue::new(Some(label_props)),
        description_props: StoredValue::new(Some(description_props)),
        is_selected,
        is_focused,
        is_focus_visible,
        is_disabled,
        is_pressed,
        is_hovered,
    };
    let (attrs, option_styles) = props.into_parts();
    let styles = option_styles.merge(styles);

    Some(view! {
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
                data-hovered=flag(is_hovered)
            >
                {children()}
            </div>
        </Provider>
    })
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
    let render = std::sync::Arc::new(move |key: Key| {
        let node_key = key.clone();
        let node = Memo::new(move |_| collection.with(|c| c.get(&node_key).cloned()));
        let children = children.clone();
        view! {
            <ListBoxItem key=key classes=classes.clone()>
                {move || node.get().map(|node| children(node))}
            </ListBoxItem>
        }
        .into_any()
    });
    // Inside a `Virtualizer`: the visible options only.
    if let Some(root) = use_context::<Option<super::virtualizer::VirtualizedRoot>>().flatten() {
        return super::virtualizer::render_visible_items(root, render).into_any();
    }
    view! {
        <For
            each=move || collection.with(|c| c.items().map(|node| node.key.clone()).collect::<Vec<_>>())
            key=Clone::clone
            children=move |key| render(key)
        />
    }
    .into_any()
}

/// The main text of a [`ListBoxItem`] (labels the option).
///
/// Default class: `leptonic-ListBoxItemLabel`.
#[component]
pub fn ListBoxItemLabel(
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let classes = with_default_class("leptonic-ListBoxItemLabel", classes);
    let Some(ctx) = use_context::<ListBoxItemContext>() else {
        crate::utils::dev_warn!("<ListBoxItemLabel> must be inside a <ListBoxItem>");
        return None;
    };
    Some(slot(
        ctx.label_props,
        "ListBoxItemLabel",
        classes,
        styles,
        children,
    ))
}

/// Secondary text of a [`ListBoxItem`] (describes the option).
///
/// Default class: `leptonic-ListBoxItemDescription`.
#[component]
pub fn ListBoxItemDescription(
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let classes = with_default_class("leptonic-ListBoxItemDescription", classes);
    let Some(ctx) = use_context::<ListBoxItemContext>() else {
        crate::utils::dev_warn!("<ListBoxItemDescription> must be inside a <ListBoxItem>");
        return None;
    };
    Some(slot(
        ctx.description_props,
        "ListBoxItemDescription",
        classes,
        styles,
        children,
    ))
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

/// What a [`ListBoxSection`] provides to its [`ListBoxSectionHeading`].
#[derive(Clone, Copy)]
struct ListBoxSectionContext {
    heading_props: StoredValue<Option<UseListBoxSectionHeadingProps>>,
    /// The collection's header text.
    heading: Signal<Option<String>>,
}

/// A group of options in a [`ListBox`], for the collection section `key`: one `role="group"`
/// element holding a [`ListBoxSectionHeading`] (when the section has a header in the
/// collection) and the section's options.
///
/// ```ignore
/// <ListBoxSection key="fruit">
///     <ListBoxSectionHeading />
///     <ListBoxItem key="apple">"Apple"</ListBoxItem>
/// </ListBoxSection>
/// ```
///
/// Default class: `leptonic-ListBoxSection`.
#[component]
pub fn ListBoxSection(
    /// The section's key in the listbox's collection.
    #[prop(into)]
    key: Key,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let classes = with_default_class("leptonic-ListBoxSection", classes);
    let Some(list) = use_context::<ListBoxData>() else {
        crate::utils::dev_warn!("<ListBoxSection> must be inside a <ListBox>");
        return None;
    };
    let UseListBoxSectionReturn {
        heading_props,
        group_props,
        heading,
        ..
    } = use_listbox_section(UseListBoxSectionInput { list, key });
    let ctx = ListBoxSectionContext {
        heading_props: StoredValue::new(heading_props),
        heading,
    };

    Some(view! {
        <section {..group_props.into_attrs()} class=classes style=styles>
            <Provider value=ctx>{children()}</Provider>
        </section>
    })
}

/// The heading of a [`ListBoxSection`] (react-aria-components: `Header`), labelling the section.
/// Shows `children`, or else the section's header text from the collection. Hidden from
/// assistive technology as a heading (a listbox can't contain headings): it names the group.
///
/// Default class: `leptonic-ListBoxSectionHeading`.
#[component]
pub fn ListBoxSectionHeading(
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let classes = with_default_class("leptonic-ListBoxSectionHeading", classes);
    let Some(ctx) = use_context::<ListBoxSectionContext>() else {
        crate::utils::dev_warn!("<ListBoxSectionHeading> must be inside a <ListBoxSection>");
        return ().into_any();
    };
    let content = if let Some(children) = children {
        children().into_any()
    } else {
        let heading = ctx.heading;
        (move || heading.get()).into_any()
    };
    if let Some(props) = ctx.heading_props.try_update_value(Option::take).flatten() {
        view! {
            <header {..props.into_attrs()} class=classes style=styles>
                {content}
            </header>
        }
        .into_any()
    } else {
        crate::utils::dev_warn!(
            "ListBoxSectionHeading: one per section, and only for sections with a header in \
                 the collection"
        );
        view! {
            <header class=classes style=styles>
                {content}
            </header>
        }
        .into_any()
    }
}
