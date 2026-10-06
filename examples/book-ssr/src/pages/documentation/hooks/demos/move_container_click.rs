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

#[component]
pub fn ContainerClickExample() -> impl IntoView {
    let UseConstrainedMoveReturn {
        props,
        is_moving,
        container_props,
        pixel_position,
        ..
    } = use_constrained_move(
        UseMoveInput::default(),
        MoveConstraintOptions {
            allow_container_click: true,
            ..MoveConstraintOptions::new(MoveConstraint::Bounds)
        },
    );

    let handle_styles = Styles::builder()
        .with_reactive(move || LeftProperty.declare(offset(pixel_position.get().x)))
        .with_reactive(move || TopProperty.declare(offset(pixel_position.get().y)))
        .build();

    view! {
        <div {..container_props.into_attrs()} class="demo-move-area demo-move-area-clickable">
            <div
                {..props.into_attrs()}
                tabindex="0"
                class="demo-move-handle"
                data-moving=flag(is_moving)
                style=handle_styles
            >
                "Click anywhere!"
            </div>
        </div>

        <p class="demo-caption">"Click anywhere in the area to move the element there, or drag it directly."</p>
    }
}
