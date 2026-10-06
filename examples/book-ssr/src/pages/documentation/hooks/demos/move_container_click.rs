use leptonic::{
    hooks::*,
    utils::{
        css::{CssDimension, LengthPercentageAuto, try_px},
        style::{LeftProperty, TopProperty},
        styles::Styles,
    },
};
use leptos::prelude::*;

fn offset(px: f64) -> LengthPercentageAuto {
    LengthPercentageAuto::from(try_px(px).unwrap_or(CssDimension::Zero))
}

#[component]
pub fn ContainerClickExample() -> impl IntoView {
    let UseMoveReturn {
        props,
        is_moving,
        constraint,
    } = use_move(UseMoveInput {
        is_disabled: false.into(),
        axis: None.into(),
        on_move_start: None,
        on_move: None,
        on_move_end: None,
        on_position_change: None,
        constraint: Some(MoveConstraint::Bounds),
        allow_container_click: true,
        initial_position: None,
    });
    let constraint = constraint.expect("`constraint` is set, so the constraint return is present");
    let pixel_position = constraint.pixel_position;

    let handle_styles = Styles::builder()
        .with_reactive(move || LeftProperty.declare(offset(pixel_position.get().0)))
        .with_reactive(move || TopProperty.declare(offset(pixel_position.get().1)))
        .build();

    view! {
        <div {..constraint.container_props.into_attrs()} class="demo-move-area demo-move-area-clickable">
            <div
                {..props.into_attrs()}
                tabindex="0"
                class="demo-move-handle"
                class:moving=move || is_moving.get()
                style=handle_styles
            >
                "Click anywhere!"
            </div>
        </div>

        <p class="demo-caption">"Click anywhere in the area to move the element there, or drag it directly."</p>
    }
}
