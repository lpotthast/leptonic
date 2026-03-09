use leptos::{context::Provider, portal::Portal, prelude::*};

use crate::{
    hooks::{UseOverlayAttrs, *},
    utils::{locale::WritingDirection, CapturedElement},
};

#[derive(Debug, Clone)]
pub struct PopoverContext {
    pub state: ReadSignal<bool>,
    pub set_state: WriteSignal<bool>,

    id: Oco<'static, str>,
    overlay_attrs: UseOverlayAttrs,
    trigger_element: CapturedElement,
}

#[component]
pub fn Popover(children: Children) -> impl IntoView {
    let (state, set_state) = signal(false);

    let UseOverlayReturn {
        props: overlay_props,
        underlay_props: _,
        id,
    } = use_overlay(UseOverlayInput {
        is_open: state.into(),
        on_close: Callback::new(move |()| set_state.set(false)),
        is_dismissable: true,
        should_close_on_blur: false,
        is_keyboard_dismiss_disabled: false,
        should_close_on_interact_outside: None,
    });

    let trigger_element = CapturedElement::new();
    let overlay_attrs = overlay_props.into_attrs();

    view! {
        <Provider value=PopoverContext {
            state,
            set_state,
            id,
            overlay_attrs,
            trigger_element,
        }>{children()}</Provider>
    }
}

#[component]
pub fn PopoverTrigger(children: Children) -> impl IntoView {
    let ctx = expect_context::<PopoverContext>();

    let UseOverlayTriggerReturn {
        props: trigger_props,
    } = use_overlay_trigger(UseOverlayTriggerInput {
        show: ctx.state.into(),
        overlay_id: ctx.id,
        overlay_type: OverlayTriggerType::Dialog,
    });

    let trigger_capture = ctx.trigger_element.attr();

    view! {
        <div class="leptonic-popover-trigger" {..trigger_props.into_attrs()} {..trigger_capture}>
            {children()}
        </div>
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

    let UseOverlayPositionReturn {
        props: overlay_pos_props,
        resolved_placement_x: _,
        resolved_placement_y: _,
    } = use_overlay_position(UseOverlayPositionInput {
        target: ctx.trigger_element,
        placement_x,
        placement_y,
        writing_direction,
        offset: 0.0.into(),
        cross_offset: 0.0.into(),
        container_padding: 12.0.into(),
        should_flip: true.into(),
        max_height: None,
        is_open: ctx.state.into(),
    });

    let overlay_pos_attrs = overlay_pos_props.into_attrs();

    view! {
        <Portal>
            {
                let overlay_attrs = ctx.overlay_attrs.clone();
                let overlay_pos_attrs = overlay_pos_attrs.clone();
                let children = children.clone();
                view! {
                    <Show when=move || ctx.state.get()>
                        <div
                            class="leptonic-popover-content"
                            {..overlay_attrs.clone()}
                            {..overlay_pos_attrs.clone()}
                        >
                            {children()}
                        </div>
                    </Show>
                }
            }
        </Portal>
    }
}
