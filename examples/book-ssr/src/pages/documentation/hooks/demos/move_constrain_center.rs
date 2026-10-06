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

/// A movable element constrained to its area with the given `MoveConstraint`.
#[component]
fn ConstrainedArea(mode: MoveConstraint, label: &'static str) -> impl IntoView {
    let UseConstrainedMoveReturn {
        props,
        is_moving,
        container_props,
        pixel_position,
        ..
    } = use_constrained_move(UseMoveInput::default(), MoveConstraintOptions::new(mode));

    let handle_styles = Styles::builder()
        .with_reactive(move || LeftProperty.declare(offset(pixel_position.get().x)))
        .with_reactive(move || TopProperty.declare(offset(pixel_position.get().y)))
        .build();

    view! {
        <div {..container_props.into_attrs()} class="demo-move-area demo-move-area-medium">
            <div
                {..props.into_attrs()}
                tabindex="0"
                class="demo-move-handle"
                data-moving=flag(is_moving)
                style=handle_styles
            >
                {label}
            </div>
        </div>
    }
}

#[component]
pub fn ConstrainCenterExample() -> impl IntoView {
    view! {
        <div class="demo-move-columns">
            <div class="demo-move-column">
                <p><strong><code>"MoveConstraint::Bounds"</code></strong></p>
                <ConstrainedArea mode=MoveConstraint::Bounds label="Bounds"/>
                <p class="demo-caption">"The element stays fully inside."</p>
            </div>
            <div class="demo-move-column">
                <p><strong><code>"MoveConstraint::Center"</code></strong></p>
                <ConstrainedArea mode=MoveConstraint::Center label="Center"/>
                <p class="demo-caption">"The element\u{2019}s center stays inside."</p>
            </div>
        </div>
    }
}
