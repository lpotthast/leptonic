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

/// A constrained movable element that only moves along `axis`.
#[component]
fn AxisArea(
    axis: MoveAxis,
    area_class: &'static str,
    handle_class: &'static str,
    label: &'static str,
) -> impl IntoView {
    let UseMoveReturn {
        props,
        is_moving,
        constraint,
    } = use_move(UseMoveInput {
        is_disabled: false.into(),
        axis: Signal::stored(Some(axis)),
        on_move_start: None,
        on_move: None,
        on_move_end: None,
        on_position_change: None,
        constraint: Some(MoveConstraint::Bounds),
        allow_container_click: false,
        initial_position: None,
    });
    let constraint = constraint.expect("`constraint` is set, so the constraint return is present");
    let pixel_position = constraint.pixel_position;

    let handle_styles = Styles::builder()
        .with_reactive(move || LeftProperty.declare(offset(pixel_position.get().0)))
        .with_reactive(move || TopProperty.declare(offset(pixel_position.get().1)))
        .build();

    view! {
        <div {..constraint.container_props.into_attrs()} class=area_class>
            <div
                {..props.into_attrs()}
                tabindex="0"
                class=handle_class
                class:moving=move || is_moving.get()
                style=handle_styles
            >
                {label}
            </div>
        </div>
    }
}

#[component]
pub fn AxisExample() -> impl IntoView {
    view! {
        <div class="demo-move-columns">
            <div class="demo-move-column">
                <p><strong>"Horizontal only"</strong></p>
                <AxisArea
                    axis=MoveAxis::Horizontal
                    area_class="demo-move-area demo-move-area-short"
                    handle_class="demo-move-handle demo-move-handle-small demo-move-handle-horizontal"
                    label="H"
                />
            </div>
            <div class="demo-move-column">
                <p><strong>"Vertical only"</strong></p>
                <AxisArea
                    axis=MoveAxis::Vertical
                    area_class="demo-move-area demo-move-area-medium"
                    handle_class="demo-move-handle demo-move-handle-small demo-move-handle-vertical"
                    label="V"
                />
            </div>
        </div>
    }
}
