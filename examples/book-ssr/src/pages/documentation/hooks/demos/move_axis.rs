use leptonic::{
    hooks::*,
    utils::{
        css::{LengthPercentageAuto, computed_px},
        data_attributes::flag,
        style::{LeftProperty, TopProperty},
        styles::Styles,
    },
};
use leptos::prelude::*;

/// A pixel offset. Measured sizes are `NaN` before the first layout, which renders as `0px`.
fn offset(px: f64) -> LengthPercentageAuto {
    LengthPercentageAuto::from(computed_px(px))
}

/// A constrained movable element that only moves along `axis`.
#[component]
fn AxisArea(
    axis: MoveAxis,
    area_class: &'static str,
    handle_class: &'static str,
    label: &'static str,
) -> impl IntoView {
    let UseConstrainedMoveReturn {
        props,
        is_moving,
        container_props,
        pixel_position,
        ..
    } = use_constrained_move(
        UseMoveInput {
            axis: Signal::stored(axis),
            ..UseMoveInput::default()
        },
        MoveConstraintOptions::new(MoveConstraint::Bounds),
    );

    let handle_styles = Styles::builder()
        .with_reactive(move || LeftProperty.declare(offset(pixel_position.get().x)))
        .with_reactive(move || TopProperty.declare(offset(pixel_position.get().y)))
        .build();

    view! {
        <div {..container_props.into_attrs()} class=area_class>
            <div
                {..props.into_attrs()}
                tabindex="0"
                class=handle_class
                data-moving=flag(is_moving)
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
                    label="Horizontal"
                />
            </div>
            <div class="demo-move-column">
                <p><strong>"Vertical only"</strong></p>
                <AxisArea
                    axis=MoveAxis::Vertical
                    area_class="demo-move-area demo-move-area-medium"
                    handle_class="demo-move-handle demo-move-handle-small demo-move-handle-vertical"
                    label="Vertical"
                />
            </div>
        </div>
    }
}
