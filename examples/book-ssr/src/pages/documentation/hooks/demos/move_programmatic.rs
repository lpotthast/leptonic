use leptonic::{
    components::prelude::*,
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
        allow_container_click: false,
        initial_position: None,
    });
    let constraint = constraint.expect("`constraint` is set, so the constraint return is present");
    let normalized_position = constraint.normalized_position;
    let pixel_position = constraint.pixel_position;
    let set_position = constraint.set_position;

    let handle_styles = Styles::builder()
        .with_reactive(move || LeftProperty.declare(offset(pixel_position.get().0)))
        .with_reactive(move || TopProperty.declare(offset(pixel_position.get().1)))
        .build();

    view! {
        <div {..constraint.container_props.into_attrs()} class="demo-move-area">
            <div
                {..props.into_attrs()}
                tabindex="0"
                class="demo-move-handle"
                class:moving=move || is_moving.get()
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

        <p>
            "Normalized: ("
            {move || format!("{:.2}", normalized_position.get().x)}
            ", "
            {move || format!("{:.2}", normalized_position.get().y)}
            ")"
        </p>
    }
}
