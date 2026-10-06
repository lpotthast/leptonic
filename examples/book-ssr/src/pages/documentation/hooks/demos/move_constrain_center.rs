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

/// A movable element constrained to its area with the given `MoveConstraint`.
#[component]
fn ConstrainedArea(mode: MoveConstraint, label: &'static str) -> impl IntoView {
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
        constraint: Some(mode),
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
        <div {..constraint.container_props.into_attrs()} class="demo-move-area demo-move-area-medium">
            <div
                {..props.into_attrs()}
                tabindex="0"
                class="demo-move-handle"
                class:moving=move || is_moving.get()
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
