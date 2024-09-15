use crate::{
    hooks::*,
    state::overlay::OverlayTriggerState,
    utils::{aria::AriaHasPopup, locale::WritingDirection},
    Transparent,
};
use leptos::*;

#[derive(Debug, Clone)]
pub struct PopoverContext {
    pub state: ReadSignal<bool>,
    pub set_state: WriteSignal<bool>,

    pub(crate) id: Oco<'static, str>,
    pub(crate) overlay_props: UseOverlayProps, // TODO: Rc?
    pub(crate) trigger_el: ReadSignal<Option<NodeRef<html::Custom>>>,
    pub(crate) set_trigger_el: WriteSignal<Option<NodeRef<html::Custom>>>,

    pub(crate) disabled: MaybeSignal<bool>,
}

#[component]
pub fn Popover(#[prop(into)] disabled: MaybeSignal<bool>, children: Children) -> impl IntoView {
    let UseOverlayReturn {
        props: overlay_props,
        id,
        state,
        set_state,
    } = use_overlay(UseOverlayInput { disabled });

    let (trigger_el, set_trigger_el) = create_signal(None);

    view! {
        <Provider value=PopoverContext { id, overlay_props, state, set_state, trigger_el, set_trigger_el, disabled }>
            { children() }
        </Provider>
    }
}

#[component]
pub fn PopoverTrigger(children: Children) -> impl IntoView {
    let ctx = expect_context::<PopoverContext>();

    let trigger_el: NodeRef<html::Custom> = create_node_ref();
    ctx.set_trigger_el.set(Some(trigger_el));

    let state = OverlayTriggerState {
        show: ctx.state,
        set_show: ctx.set_state,
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

    let UseOverlayTriggerReturn {
        props: trigger_props,
    } = use_overlay_trigger(
        state,
        Some(press_responder),
        UseOverlayTriggerInput {
            overlay_id: ctx.id,
            overlay_type: AriaHasPopup::Menu,
        },
    );

    let attrs = press_props.attrs.merge(trigger_props.attrs);
    let handlers = press_props.handlers;

    view! {
        <Transparent>
            <leptonic-popover-trigger {..attrs} {..handlers} node_ref=trigger_el>
                { children() }
            </leptonic-popover-trigger>
        </Transparent>
    }
}

#[component]
pub fn PopoverContent(
    #[prop(into)] placement_x: MaybeSignal<PlacementX>,
    #[prop(into)] placement_y: MaybeSignal<PlacementY>,
    #[prop(into)] writing_direction: MaybeSignal<WritingDirection>,
    children: ChildrenFn,
) -> impl IntoView {
    let ctx = expect_context::<PopoverContext>();

    let overlay_el: NodeRef<html::Custom> = create_node_ref();

    let UseOverlayPositionReturn {
        props: overlay_pos_props,
    } = use_overlay_position(
        UseOverlayPositionInput::builder()
            .overlay_ref(overlay_el)
            .target_ref(ctx.trigger_el.get_untracked().expect("trigger present"))
            .container_ref(Option::<NodeRef<html::Custom>>::None)
            .placement_x(placement_x)
            .placement_y(placement_y)
            .writing_direction(writing_direction)
            .build(),
    );

    view! {
        <Portal>
        {
            let overlay_props_attrs = ctx.overlay_props.attrs.clone();
            let overlay_pos_props_attrs = overlay_pos_props.attrs.clone();
            let children = children.clone();
            view! {
                <Show when=move || ctx.state.get()>
                    <leptonic-popover-content
                        {..overlay_props_attrs.clone()}
                        {..overlay_pos_props_attrs.clone()}
                        node_ref=overlay_el
                    >
                        { children() }
                    </leptonic-popover-content>
                </Show>
            }
        }
        </Portal>
    }
}
