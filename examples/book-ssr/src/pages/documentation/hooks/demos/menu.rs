use std::sync::Arc;

use leptonic::{
    atoms::focus_scope::FocusScope,
    hooks::{
        IntoAttrs, MenuData, MenuTriggerType, OverlayTriggerType, PlacementX, PlacementY,
        PopoverModality, SelectionMode, UseMenuInput, UseMenuItemInput, UseMenuItemReturn,
        UseMenuReturn, UseMenuSectionInput, UseMenuSectionReturn, UseMenuTriggerInput,
        UseMenuTriggerStateInput, UsePopoverInput, UsePopoverReturn,
        collections::{
            AutoFocus, CollectionBuilder, CollectionOptions, Key, NodeKind, Selection,
            SelectionOptions, UseListStateInput, use_collection, use_list_state,
        },
        use_button, use_menu, use_menu_item, use_menu_section, use_menu_trigger,
        use_menu_trigger_state, use_popover,
    },
    utils::{CapturedElement, classes::Classes},
};
use leptos::prelude::*;

/// Describes the menu's items. Called by `use_collection` to build the collection.
type MenuContents = Arc<dyn Fn(&mut CollectionBuilder) + Send + Sync>;

#[component]
pub fn MenuDemo() -> impl IntoView {
    let last_action = RwSignal::new(None::<Key>);
    let view_selection = RwSignal::new(String::from("none"));

    view! {
        <div class="demo-flex-center-row">
            // An action menu: items trigger actions, nothing stays selected.
            <MenuButton
                label="Actions"
                selection_mode=SelectionMode::None
                contents=Arc::new(|b: &mut CollectionBuilder| {
                    b.item("edit", "Edit");
                    b.item("duplicate", "Duplicate");
                    b.item("archive", "Archive").disabled(true);
                    b.item("delete", "Delete");
                })
                on_action=Callback::new(move |key| last_action.set(Some(key)))
                on_selection_change=Callback::new(|_| {})
            />
            // A menu with sections whose items can be checked.
            <MenuButton
                label="View"
                selection_mode=SelectionMode::Multiple
                contents=Arc::new(|b: &mut CollectionBuilder| {
                    b.section("panels", |s| {
                        s.header("panels-header", "Panels");
                        s.item("sidebar", "Sidebar");
                        s.item("toolbar", "Toolbar");
                    });
                    b.section("zoom", |s| {
                        s.header("zoom-header", "Zoom");
                        s.item("fit", "Fit to window");
                    });
                })
                on_action=Callback::new(|_| {})
                on_selection_change=Callback::new(move |selection| {
                    view_selection.set(describe(&selection));
                })
            />
        </div>
        <p>
            "Last action: "
            <strong>{move || last_action.get().map_or_else(|| "none".to_owned(), |key| key.to_string())}</strong>
            ". View: "<strong>{view_selection}</strong>"."
        </p>
    }
}

fn describe(selection: &Selection) -> String {
    match selection {
        Selection::All => "all".to_owned(),
        Selection::Keys(keys) if keys.is_empty() => "none".to_owned(),
        Selection::Keys(keys) => {
            let mut keys: Vec<String> = keys.iter().map(ToString::to_string).collect();
            keys.sort();
            keys.join(", ")
        }
    }
}

/// A button opening a menu in a popover. `use_menu_trigger` configures the button and tells the menu which element
/// labels it and where focus goes when it opens.
#[component]
fn MenuButton(
    label: &'static str,
    selection_mode: SelectionMode,
    contents: MenuContents,
    on_action: Callback<Key>,
    on_selection_change: Callback<Selection>,
) -> impl IntoView {
    let state = use_menu_trigger_state(UseMenuTriggerStateInput::default());
    let menu_trigger = use_menu_trigger(UseMenuTriggerInput {
        menu_type: OverlayTriggerType::Menu,
        is_disabled: false.into(),
        trigger: MenuTriggerType::Press,
        state,
    });
    let menu_props = menu_trigger.menu_props;
    let (trigger_attrs, trigger_styles) = use_button(menu_trigger.button).props.into_parts();

    let UsePopoverReturn {
        props: popover_props,
        trigger_props: popover_trigger_props,
        ..
    } = use_popover(UsePopoverInput {
        placement_x: Signal::stored(PlacementX::Start),
        placement_y: Signal::stored(PlacementY::Below),
        offset: Signal::stored(4.0),
        cross_offset: Signal::stored(0.0),
        container_padding: Signal::stored(12.0),
        should_flip: Signal::stored(true),
        modality: PopoverModality::Modal,
        is_keyboard_dismiss_disabled: false,
        should_close_on_interact_outside: None,
        ..UsePopoverInput::new(state.overlay)
    });
    let (popover_attrs, popover_styles) = popover_props.into_parts();
    let contents = StoredValue::new(contents);

    view! {
        <button
            {..trigger_attrs}
            {..popover_trigger_props.into_attrs()}
            class="demo-btn-primary demo-menu-trigger"
            style=trigger_styles
        >
            {label}
            <span class=Classes::from("demo-disclosure-arrow").add_reactive("open", state.overlay.is_open)>"\u{25bc}"</span>
        </button>
        <Show when=move || state.is_open()>
            {
                let popover_attrs = popover_attrs.clone();
                let popover_styles = popover_styles.clone();
                view! {
                    <div {..popover_attrs} style=popover_styles>
                        <FocusScope contain=true restore_focus=true>
                            <Menu
                                id=menu_props.id
                                labelled_by=menu_props.aria_labelledby
                                auto_focus=menu_props.auto_focus
                                selection_mode=selection_mode
                                contents=contents.get_value()
                                on_action=on_action
                                on_selection_change=on_selection_change
                                on_close=Callback::new(move |()| state.close())
                            />
                        </FocusScope>
                    </div>
                }
            }
        </Show>
    }
}

/// The menu only exists while it is open, so it is focused anew on every opening.
#[component]
fn Menu(
    id: Signal<String>,
    labelled_by: Signal<String>,
    auto_focus: Signal<Option<AutoFocus>>,
    selection_mode: SelectionMode,
    contents: MenuContents,
    on_action: Callback<Key>,
    on_selection_change: Callback<Selection>,
    on_close: Callback<()>,
) -> impl IntoView {
    let collection = use_collection(move |b| contents(b));
    let state = use_list_state(UseListStateInput {
        collection,
        selection: SelectionOptions {
            selection_mode: Signal::stored(selection_mode),
            on_selection_change: Some(on_selection_change),
            ..SelectionOptions::default()
        },
    });
    let UseMenuReturn { props, data } = use_menu(UseMenuInput {
        id: Some(id.get_untracked()),
        aria_labelledby: labelled_by.into(),
        options: CollectionOptions {
            auto_focus,
            should_focus_wrap: true,
            ..CollectionOptions::default()
        },
        on_action: Some(on_action),
        on_close: Some(on_close),
        ..UseMenuInput::new(state, CapturedElement::new())
    });

    // Top-level nodes are items or sections; a section lists the keys of its items.
    let nodes = collection.with_untracked(|c| {
        c.iter()
            .map(|node| {
                let items: Vec<Key> = c
                    .children(&node.key)
                    .filter(|child| child.kind == NodeKind::Item)
                    .map(|child| child.key.clone())
                    .collect();
                (node.key.clone(), node.kind, items)
            })
            .collect::<Vec<_>>()
    });

    view! {
        <ul {..props.into_attrs()} class="demo-overlays-menu-list">
            {nodes
                .into_iter()
                .map(|(key, kind, items)| match kind {
                    NodeKind::Section => view! { <MenuSection menu=data.clone() key items/> }.into_any(),
                    _ => view! { <MenuItem menu=data.clone() key/> }.into_any(),
                })
                .collect_view()}
        </ul>
    }
}

#[component]
fn MenuSection(menu: MenuData, key: Key, items: Vec<Key>) -> impl IntoView {
    let UseMenuSectionReturn {
        item_props,
        heading_props,
        group_props,
        heading,
    } = use_menu_section(UseMenuSectionInput {
        menu: menu.clone(),
        key,
    });

    view! {
        <li {..item_props.into_attrs()} class="demo-overlays-menu-section">
            {heading_props.map(|props| view! {
                <span {..props.into_attrs()} class="demo-overlays-menu-heading">{heading}</span>
            })}
            <ul {..group_props.into_attrs()}>
                {items.into_iter().map(|key| view! { <MenuItem menu=menu.clone() key/> }).collect_view()}
            </ul>
        </li>
    }
}

#[component]
fn MenuItem(menu: MenuData, key: Key) -> impl IntoView {
    let text = menu
        .state
        .collection
        .with_untracked(|c| c.get(&key).map(|node| node.text_value.to_string()))
        .unwrap_or_default();
    let selectable = menu.state.selection.selection_mode() != SelectionMode::None;
    let UseMenuItemReturn {
        props,
        label_props,
        is_focused,
        is_focus_visible,
        is_selected,
        is_disabled,
        ..
    } = use_menu_item(UseMenuItemInput {
        menu,
        key,
        should_close_on_select: None,
    });
    let (attrs, styles) = props.into_parts();

    view! {
        <li
            {..attrs}
            class=Classes::from("demo-overlays-menu-item")
                .add_reactive("focused", is_focused)
                .add_reactive("focus-visible", is_focus_visible)
                .add_reactive("disabled", is_disabled)
            style=styles
        >
            {selectable.then(|| view! {
                <span class="demo-overlays-menu-check" aria-hidden="true">
                    {move || if is_selected.get() { "\u{2713}" } else { "" }}
                </span>
            })}
            <span {..label_props.into_attrs()}>{text}</span>
        </li>
    }
}
