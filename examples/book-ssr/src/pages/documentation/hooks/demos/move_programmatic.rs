use leptonic::{
    components::prelude::*,
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

const PRESETS: [(&str, f64, f64); 6] = [
    ("Top left", 0.0, 0.0),
    ("Top center", 0.5, 0.0),
    ("Top right", 1.0, 0.0),
    ("Center", 0.5, 0.5),
    ("Bottom left", 0.0, 1.0),
    ("Bottom right", 1.0, 1.0),
];

#[component]
pub fn ProgrammaticExample() -> impl IntoView {
    let UseConstrainedMoveReturn {
        props,
        is_moving,
        container_props,
        normalized_position,
        pixel_position,
        set_position,
    } = use_constrained_move(
        UseMoveInput::default(),
        MoveConstraintOptions::new(MoveConstraint::Bounds),
    );

    let handle_styles = Styles::builder()
        .with_reactive(move || LeftProperty.declare(offset(pixel_position.get().x)))
        .with_reactive(move || TopProperty.declare(offset(pixel_position.get().y)))
        .build();

    view! {
        <div {..container_props.into_attrs()} class="demo-move-area">
            <div
                {..props.into_attrs()}
                tabindex="0"
                class="demo-move-handle"
                data-moving=flag(is_moving)
                style=handle_styles
            >
                "Programmable"
            </div>
        </div>

        <div class="demo-inline-controls demo-move-presets">
            {PRESETS
                .into_iter()
                .map(|(label, x, y)| {
                    view! {
                        <Button on_press=move |_| set_position.run(NormalizedPosition { x, y })>{label}</Button>
                    }
                })
                .collect_view()}
        </div>

        <p class="demo-status">
            {move || {
                let position = normalized_position.get();
                format!("Position ({:.2}, {:.2}).", position.x, position.y)
            }}
        </p>
    }
}
