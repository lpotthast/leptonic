use std::marker::PhantomData;

use leptos::context::Provider;
use leptos::html;
use leptos::portal::Portal;
use leptos::prelude::*;

use crate::hooks::UseOverlayAttrs;
use crate::{
    hooks::{
        use_overlay, use_overlay_position, use_overlay_trigger, PlacementX, PlacementY,
        UseOverlayInput, UseOverlayPositionInput, UseOverlayPositionReturn, UseOverlayReturn,
        UseOverlayTriggerInput, UseOverlayTriggerReturn,
    },
    utils::{aria::AriaHasPopup, locale::WritingDirection},
};

#[derive(Debug, Clone)]
pub struct PopoverContext {
    pub state: ReadSignal<bool>,
    pub set_state: WriteSignal<bool>,

    id: Oco<'static, str>,
    overlay_attrs: UseOverlayAttrs,
    trigger_el: ReadSignal<Option<NodeRef<html::Custom<&'static str>>>>, // TODO: generic el type
    set_trigger_el: WriteSignal<Option<NodeRef<html::Custom<&'static str>>>>, // TODO: generic el type
}

#[component]
pub fn Popover(#[prop(into)] disabled: Signal<bool>, children: Children) -> impl IntoView {
    let UseOverlayReturn {
        props: overlay_props,
        id,
        state,
        set_state,
    } = use_overlay(UseOverlayInput { disabled });

    let (trigger_el, set_trigger_el) = signal(None);
    let overlay_attrs = overlay_props.into_attrs();

    view! {
        <Provider value=PopoverContext {
            state,
            set_state,
            id,
            overlay_attrs,
            trigger_el,
            set_trigger_el,
        }>{children()}</Provider>
    }
}

#[component]
pub fn PopoverTrigger(children: Children) -> impl IntoView {
    let ctx = expect_context::<PopoverContext>();

    let trigger_el: NodeRef<html::Custom<&str>> = NodeRef::new();
    ctx.set_trigger_el.set(Some(trigger_el));

    let UseOverlayTriggerReturn {
        props: trigger_props,
    } = use_overlay_trigger(UseOverlayTriggerInput {
        show: ctx.state.into(),
        overlay_id: ctx.id,
        overlay_type: AriaHasPopup::Menu,
    });

    view! {
        <leptonic-popover-trigger {..trigger_props.into_attrs()} node_ref=trigger_el>
            {children()}
        </leptonic-popover-trigger>
    }
}

#[component]
pub fn PopoverContent(
    #[prop(into)] placement_x: Signal<PlacementX>,
    #[prop(into)] placement_y: Signal<PlacementY>,
    #[prop(into)] writing_direction: Signal<WritingDirection>,
    children: ChildrenFn,
) -> impl IntoView {
    let ctx = expect_context::<PopoverContext>();

    let overlay_el: NodeRef<html::Custom<&str>> = NodeRef::new();

    let UseOverlayPositionReturn {
        props: overlay_pos_props,
    } = use_overlay_position(UseOverlayPositionInput {
        overlay: overlay_el,
        target: ctx.trigger_el.get_untracked().expect("trigger present"),
        placement_x,
        placement_y,
        writing_direction,
        phantom_data: PhantomData,
    });

    view! {
        <Portal>
            {
                let overlay_attrs = ctx.overlay_attrs.clone();
                let overlay_pos_attrs = overlay_pos_props.to_attrs();
                let children = children.clone();
                let overlay_el = overlay_el;
                view! {
                    <Show when=move || ctx.state.get()>
                        <leptonic-popover-content
                            {..overlay_attrs.clone()}
                            {..overlay_pos_attrs.clone()}
                            node_ref=overlay_el
                        >
                            {children()}
                        </leptonic-popover-content>
                    </Show>
                }
            }
        </Portal>
    }
}
