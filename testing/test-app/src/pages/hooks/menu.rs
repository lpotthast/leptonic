use std::sync::Arc;

use leptonic::{
    atoms::focus_scope::FocusScope,
    hooks::{
        IntoAttrs, MenuData, MenuTriggerType, OverlayTriggerType, Placement, PopoverModality,
        SelectionMode, UseMenuInput, UseMenuItemInput, UseMenuItemReturn, UseMenuReturn,
        UseMenuSectionInput, UseMenuSectionReturn, UseMenuTriggerInput, UseMenuTriggerStateInput,
        UsePopoverInput, UsePopoverReturn,
        collections::{
            AutoFocus, CollectionBuilder, CollectionOptions, Key, NodeKind, Selection,
            SelectionOptions, UseListStateInput, use_collection, use_list_state,
        },
        use_button, use_menu, use_menu_item, use_menu_section, use_menu_trigger,
        use_menu_trigger_state, use_popover,
    },
    utils::CapturedElement,
};
use leptos::prelude::*;

use crate::pages::atoms::listbox::describe_selection;

/// Two menus built from the menu hooks (there is no menu atom), each in a modal popover:
/// - "Actions": an action menu (Copy, Cut, Paste (disabled), Delete). Every action is appended
///   to `#test-menu-actions`.
/// - "View": a menu with multiple selection and two sections ("Panels": Sidebar, Toolbar;
///   "Zoom": Fit). The selection is shown in `#test-menu-view-selection`.
#[component]
pub fn PageHookMenu() -> impl IntoView {
    let actions = RwSignal::new(Vec::<String>::new());
    let view_selection = RwSignal::new(String::new());

    view! {
        <div id="test-page-hook-menu">
            <h1>"Menu"</h1>
            <button id="test-menu-before">"Before"</button>
            <MenuButton
                label="Actions"
                selection_mode=SelectionMode::None
                contents=Arc::new(|b: &mut CollectionBuilder| {
                    for action in ["Copy", "Cut", "Paste", "Delete"] {
                        b.item(action, action).disabled(action == "Paste");
                    }
                })
                on_action=Callback::new(move |key: Key| actions.update(|a| a.push(key.to_string())))
                on_selection_change=Callback::new(|_| {})
            />
            <MenuButton
                label="View"
                selection_mode=SelectionMode::Multiple
                contents=Arc::new(|b: &mut CollectionBuilder| {
                    b.section("panels", |s| {
                        s.header("panels-header", "Panels");
                        s.item("Sidebar", "Sidebar");
                        s.item("Toolbar", "Toolbar");
                    });
                    b.section("zoom", |s| {
                        s.header("zoom-header", "Zoom");
                        s.item("Fit", "Fit");
                    });
                })
                on_action=Callback::new(|_| {})
                on_selection_change=Callback::new(move |selection: Selection| {
                    view_selection.set(describe_selection(&selection));
                })
            />
            <button id="test-menu-after">"After"</button>
            <div>"Actions: " <span id="test-menu-actions">{move || actions.get().join(",")}</span></div>
            <div>"View: " <span id="test-menu-view-selection">{view_selection}</span></div>
        </div>
    }
}

type Build = Arc<dyn Fn(&mut CollectionBuilder) + Send + Sync>;

/// A trigger button opening a menu in a modal popover.
#[component]
fn MenuButton(
    label: &'static str,
    selection_mode: SelectionMode,
    contents: Build,
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
    let menu_id = StoredValue::new(menu_props.id.clone());
    let (labelled_by, auto_focus) = (menu_props.aria_labelledby, menu_props.auto_focus);
    let (trigger_attrs, trigger_styles) = use_button(menu_trigger.button).props.into_parts();
    let UsePopoverReturn {
        props: popover_props,
        trigger_props: popover_trigger_props,
        ..
    } = use_popover(UsePopoverInput {
        placement: Signal::stored(Placement::BottomLeft),
        offset: Signal::stored(4.0),
        cross_offset: Signal::stored(0.0),
        container_padding: Signal::stored(12.0),
        should_flip: Signal::stored(true),
        modality: PopoverModality::Modal,
        is_keyboard_dismiss_disabled: false.into(),
        should_close_on_interact_outside: None,
        state: state.overlay,
        trigger: CapturedElement::new(),
        max_height: Signal::stored(None),
        arrow_size: Signal::stored(None),
        arrow_boundary_offset: Signal::stored(0.0),
        boundary: None,
        target_rect: Signal::stored(None),
        group: None,
        is_submenu: false,
    });
    let (popover_attrs, popover_styles) = popover_props.into_parts();
    let contents = StoredValue::new(contents);

    view! {
        <button {..trigger_attrs} {..popover_trigger_props.into_attrs()} style=trigger_styles>
            {label}
        </button>
        <Show when=move || state.is_open()>
            {
                let popover_attrs = popover_attrs.clone();
                let popover_styles = popover_styles.clone();
                view! {
                    <div {..popover_attrs} style=popover_styles>
                        <FocusScope contain=true restore_focus=true>
                            <Menu
                                id=menu_id.get_value()
                                labelled_by=labelled_by
                                auto_focus=auto_focus
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

/// The menu itself. Like a react-aria `<Menu>` inside a `<Popover>`, it only exists while open,
/// so its auto focus applies on every opening.
#[component]
fn Menu(
    id: String,
    labelled_by: Signal<String>,
    auto_focus: Signal<Option<AutoFocus>>,
    selection_mode: SelectionMode,
    contents: Build,
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
        id: Some(id),
        aria_labelledby: labelled_by.into(),
        options: CollectionOptions {
            auto_focus,
            should_focus_wrap: true,
            ..CollectionOptions::default()
        },
        on_action: Some(on_action),
        on_close: Some(on_close),
        state,
        element: CapturedElement::new(),
        aria_label: MaybeProp::default(),
        keyboard_delegate: None,
        submenu: None,
    });

    // Render the collection: items, and sections with their items.
    let entries = collection.with_untracked(|c| {
        c.iter()
            .map(|node| {
                let children: Vec<Key> = c
                    .children(&node.key)
                    .filter(|n| n.kind == NodeKind::Item)
                    .map(|n| n.key.clone())
                    .collect();
                (node.key.clone(), node.kind, children)
            })
            .collect::<Vec<_>>()
    });

    view! {
        <ul {..props.into_attrs()}>
            {entries
                .into_iter()
                .map(|(key, kind, children)| match kind {
                    NodeKind::Section => {
                        view! { <MenuSection menu=data.clone() key=key items=children /> }
                            .into_any()
                    }
                    _ => view! { <MenuItem menu=data.clone() key=key /> }.into_any(),
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
        <li {..item_props.into_attrs()}>
            {heading_props.map(|props| view! { <span {..props.into_attrs()}>{heading}</span> })}
            <ul {..group_props.into_attrs()}>
                {items
                    .into_iter()
                    .map(|key| view! { <MenuItem menu=menu.clone() key=key /> })
                    .collect_view()}
            </ul>
        </li>
    }
}

#[component]
fn MenuItem(menu: MenuData, key: Key) -> impl IntoView {
    let text = key.to_string();
    let UseMenuItemReturn {
        props, label_props, ..
    } = use_menu_item(UseMenuItemInput {
        menu,
        key,
        should_close_on_select: leptonic::hooks::collections::CloseOnSelect::Auto,
        submenu_trigger: None,
    });
    let (attrs, styles) = props.into_parts();

    view! {
        <li {..attrs} style=styles>
            <span {..label_props.into_attrs()}>{text}</span>
        </li>
    }
}
