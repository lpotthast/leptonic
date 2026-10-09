use std::sync::Arc;

use leptonic::{
    CapturedElement, IntoAttrs,
    atoms::{
        checkbox::{CheckboxButton, CheckboxField},
        focus_scope::FocusScope,
    },
    hooks::{
        button::use_button,
        collections::{
            AutoFocus, CollectionBuilder, CollectionOptions, Key, NodeKind, Selection,
            SelectionMode, SelectionOptions, UseListStateInput, use_collection, use_list_state,
        },
        menu::{
            MenuData, MenuTriggerType, UseMenuInput, UseMenuItemInput, UseMenuItemReturn,
            UseMenuReturn, UseMenuSectionInput, UseMenuSectionReturn, UseMenuTriggerInput,
            UseMenuTriggerStateInput, use_menu, use_menu_item, use_menu_section, use_menu_trigger,
            use_menu_trigger_state,
        },
        overlay::{
            OverlayPositionOptions, OverlayTriggerType, Placement, PopoverModality,
            UsePopoverInput, UsePopoverReturn, use_popover,
        },
    },
};
use leptos::prelude::*;

/// Describes the menu's items. Called by `use_collection` to build the collection.
type MenuContents = Arc<dyn Fn(&mut CollectionBuilder) + Send + Sync>;

#[component]
pub fn MenuDemo() -> impl IntoView {
    // App state: the last action, and the checked items of the View menu. The menu is created anew on every
    // opening, so its selection lives here, bound with `selection`, and survives closing the menu.
    let last_action = RwSignal::new(None::<Key>);
    let view_selection = RwSignal::new(Selection::keys([Key::from("sidebar")]));
    let disabled = RwSignal::new(false);

    let status = move || {
        let action = last_action
            .get()
            .map_or_else(|| "none".to_owned(), |key| key.to_string());
        let view = view_selection.with(|selection| match selection {
            Selection::All => "all".to_owned(),
            Selection::Keys(keys) if keys.is_empty() => "none".to_owned(),
            Selection::Keys(keys) => {
                let mut keys: Vec<String> = keys.iter().map(ToString::to_string).collect();
                keys.sort();
                keys.join(", ")
            }
        });
        format!("Last action: {action}. View: {view}.")
    };

    view! {
        <div class="demo-flex-center-row">
            // An action menu: items trigger actions, nothing stays selected.
            <MenuButton
                label="Actions"
                contents=Arc::new(|b: &mut CollectionBuilder| {
                    b.item("edit", "Edit");
                    b.item("duplicate", "Duplicate");
                    b.item("archive", "Archive").disabled(true);
                    b.item("delete", "Delete");
                })
                on_action=Callback::new(move |key| last_action.set(Some(key)))
                is_disabled=disabled
            />
            // A menu with sections whose items can be checked.
            <MenuButton
                label="View"
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
                selection=view_selection
                is_disabled=disabled
            />
        </div>
        <p class="demo-status">{status}</p>
        <div class="demo-controls">
            <CheckboxField is_selected=disabled set_selected=disabled>
                <CheckboxButton classes="demo-check">
                    <span class="demo-check-box" aria-hidden="true"></span>
                    "Disabled"
                </CheckboxButton>
            </CheckboxField>
        </div>
    }
}

/// A button opening a menu in a popover. `use_menu_trigger` configures the button and tells the menu which element
/// labels it and where focus goes when it opens.
#[component]
fn MenuButton(
    label: &'static str,
    contents: MenuContents,
    #[prop(optional)] on_action: Option<Callback<Key>>,
    /// The checked items; without them, the menu is a menu of actions.
    #[prop(optional)]
    selection: Option<RwSignal<Selection>>,
    #[prop(into)] is_disabled: Signal<bool>,
) -> impl IntoView {
    let state = use_menu_trigger_state(UseMenuTriggerStateInput::default());
    let menu_trigger = use_menu_trigger(UseMenuTriggerInput {
        menu_type: OverlayTriggerType::Menu,
        is_disabled,
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
        position: OverlayPositionOptions {
            placement: Signal::stored(Placement::BottomStart),
            offset: Signal::stored(4.0),
            ..OverlayPositionOptions::default()
        },
        state: state.overlay,
        trigger: CapturedElement::new(),
        target_rect: Signal::stored(None),
        modality: PopoverModality::Modal,
        is_keyboard_dismiss_disabled: Signal::stored(false),
        should_close_on_interact_outside: None,
        group: None,
        is_submenu: false,
        scroll: None,
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
            <span class="demo-menu-arrow" aria-hidden="true">"\u{25bc}"</span>
        </button>
        <Show when=move || state.is_open()>
            {
                let popover_attrs = popover_attrs.clone();
                let popover_styles = popover_styles.clone();
                let menu_id = menu_props.id.clone();
                view! {
                    <div {..popover_attrs} style=popover_styles>
                        <FocusScope contain=true restore_focus=true>
                            <Menu
                                id=menu_id
                                labelled_by=menu_props.aria_labelledby
                                auto_focus=menu_props.auto_focus
                                contents=contents.get_value()
                                on_action=on_action
                                selection=selection
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
    id: String,
    labelled_by: Signal<String>,
    auto_focus: Signal<Option<AutoFocus>>,
    contents: MenuContents,
    on_action: Option<Callback<Key>>,
    selection: Option<RwSignal<Selection>>,
    on_close: Callback<()>,
) -> impl IntoView {
    let collection = use_collection(move |b| contents(b));
    let state = use_list_state(UseListStateInput {
        collection,
        selection: SelectionOptions {
            selection_mode: Signal::stored(if selection.is_some() {
                SelectionMode::Multiple
            } else {
                SelectionMode::None
            }),
            selection: selection.map(Into::into),
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
        on_action,
        on_close: Some(on_close),
        state,
        element: CapturedElement::new(),
        aria_label: MaybeProp::default(),
        keyboard_delegate: None,
        submenu: None,
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
        <ul {..props.into_attrs()} class="demo-menu-list">
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
        <li {..item_props.into_attrs()} class="demo-menu-section">
            {
                let heading_attrs = heading_props.into_attrs();
                move || heading.get().map(|text| view! {
                    <span {..heading_attrs.clone()} class="demo-menu-heading">{text}</span>
                })
            }
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
        ..
    } = use_menu_item(UseMenuItemInput {
        menu,
        key,
        should_close_on_select: leptonic::hooks::collections::CloseOnSelect::Auto,
        submenu_trigger: None,
    });
    let (attrs, styles) = props.into_parts();

    view! {
        <li
            {..attrs}
            data-focused=move || is_focused.get().then_some("")
            data-focus-visible=move || is_focus_visible.get().then_some("")
            class="demo-menu-item"
            style=styles
        >
            {selectable.then(|| view! {
                <span class="demo-menu-check" aria-hidden="true">
                    {move || if is_selected.get() { "\u{2713}" } else { "" }}
                </span>
            })}
            <span {..label_props.into_attrs()}>{text}</span>
        </li>
    }
}
