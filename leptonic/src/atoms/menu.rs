use crate::{
    hooks::*,
    state::{menu::MenuTriggerState, tree::TreeState, CollectionFocusStrategy, Key},
    utils::{
        aria::{AriaAccessibleName, AriaHasPopup},
        pointer_type::PointerType,
    },
    OptMaybeSignal, Transparent,
};
use leptos::*;

#[derive(Clone)]
pub struct MenuContext {
    pub state: ReadSignal<bool>,
    pub set_state: WriteSignal<bool>,

    pub(crate) trigger_el: NodeRef<html::Custom>,
    pub(crate) disabled: MaybeSignal<bool>,
}

/*
MenuTrigger establishes context? Is that right? Simpler for users?

MenuItem registers itself in context..

*/

#[component]
pub fn MenuTrigger(#[prop(into)] disabled: MaybeSignal<bool>, children: Children) -> impl IntoView {
    let trigger_el: NodeRef<html::Custom> = create_node_ref();

    let ctx = MenuContext {
        trigger_el,
        disabled,
        state: todo!(),
        set_state: todo!(),
    };

    let state = MenuTriggerState {
        open: ctx.state,
        set_open: ctx.set_state,
    };

    let UsePressReturn {
        props: press_props,
        is_pressed: _,
        press_responder,
    } = use_press(
        UsePressInput::builder()
            .disabled(ctx.disabled)
            .on_press(|_| {})
            .build(),
    );

    let UseMenuTriggerReturn {
        props: trigger_props,
    } = use_menu_trigger(
        trigger_el,
        press_responder,
        state,
        UseMenuTriggerInput {
            disabled: ctx.disabled,
            menu_trigger_id: todo!(), // ctx.id,
            menu_id: todo!(),         // ctx.overlay_id,
            overlay_type: AriaHasPopup::Menu,
        },
    );

    let attrs = press_props.attrs.merge(trigger_props.attrs);
    let handlers = press_props.handlers;

    view! {
        <Provider value=ctx>
            <Transparent>
                <leptonic-menu-trigger {..attrs} {..handlers} node_ref=trigger_el>
                    { children() }
                </leptonic-menu-trigger>
            </Transparent>
        </Provider>
    }
}

#[component]
pub fn Menu(
    /// An accessible name for this menu.
    #[prop(into)]
    accessible_name: AriaAccessibleName,

    /// Whether the collection or one of its items should be automatically focused upon render. Defaults to None.
    #[prop(into, default = None)]
    auto_focus: Option<CollectionFocusStrategy>,

    /// Whether focus should wrap around when the end/start is reached / keyboard navigation is circular. Defaults to true.
    #[prop(into, default = true)]
    should_focus_wrap: bool,

    /// Handler that is called when an item is selected. // TODO: Is `selected` the right word? Name it `clicked` or `activated` instead?
    #[prop(into)]
    on_action: Callback<Key>,

    /// Handler that is called when the menu should close after selecting an item. Defaults to None.
    #[prop(into, default = None)]
    on_close: Option<Callback<()>>,

    children: Children,
) -> impl IntoView {
    let state = TreeState {};

    let UseMenuReturn { attrs, handlers } = use_menu(
        UseMenuInput {
            accessible_name,
            auto_focus,
            should_focus_wrap,
            on_action,
            on_close,
        },
        state,
    );

    view! {
        <Transparent>
            <leptonic-menu {..attrs} {..handlers}>
                { children() }
            </leptonic-menu>
        </Transparent>
    }
}

#[component]
pub fn MenuItem(
    #[prop(into)] key: Key,

    /// Whether this item should be disabled.
    #[prop(into, default = false.into())]
    disabled: MaybeSignal<bool>,

    /// Whether the menu should close when this item is selected.
    /// If None, a default logic determines wether the menu closes (e.g. in singe select mode).
    #[prop(into, default = OptMaybeSignal(None))]
    close_on_select: OptMaybeSignal<bool>,

    #[prop(into, default = AriaHasPopup::False)] aria_haspopup: AriaHasPopup,

    /// Contents of this menu item. Could be a simple text node.
    children: Children,
) -> impl IntoView {
    let UsePressReturn {
        props: press_props,
        is_pressed,
        press_responder,
    } = use_press(UsePressInput {
        disabled,
        force_prevent_default: false,
        on_press: None,
        on_press_up: None,
        on_press_start: None,
        on_press_end: None,
    });

    let UseHoverReturn {
        props: hover_props,
        is_hovered,
        hover_responder,
    } = use_hover(UseHoverInput {
        disabled,
        on_hover_start: None,
        on_hover_end: None,
    });

    let UseMenuItemReturn {
        attrs: menu_attrs,
        handlers: menu_handlers,
    } = use_menu_item(
        UseMenuItemInput {
            key,
            disabled,
            close_on_select,
            aria_haspopup,
            on_focus: None,
            on_blur: None,
            on_focus_change: None,
        },
        press_responder,
        hover_responder,
    );

    let attrs = press_props.attrs.merge(hover_props.attrs).merge(menu_attrs);

    let handlers = press_props
        .handlers
        .merge(hover_props.handlers)
        .merge(menu_handlers);

    view! {
        <Transparent>
            <leptonic-menu-item {..attrs} {..handlers}>
                { children() }
            </leptonic-menu-item>
        </Transparent>
    }
}
