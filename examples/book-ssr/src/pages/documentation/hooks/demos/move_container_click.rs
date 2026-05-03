use leptonic::hooks::*;
use leptos::prelude::*;

#[component]
pub fn ContainerClickExample() -> impl IntoView {
    let UseMoveReturn {
        props, constraint, ..
    } = use_move(UseMoveInput {
        disabled: false.into(),
        axis: Signal::derive(|| None),
        is_rtl: false,
        on_move_start: None,
        on_move: None,
        on_move_end: None,
        on_position_change: None,
        constraint: Some(MoveConstraint::Bounds),
        allow_container_click: true,
        initial_position: None,
    });
    let c = constraint.unwrap();
    let pixel_position = c.pixel_position;
    let container_attrs = c.container_props.into_attrs();
    let movable_attrs = props.into_attrs();

    view! {
        <div
            {..container_attrs}
            style="
                width: 100%;
                height: 10em;
                touch-action: none;
                border-radius: var(--typography-code-border-radius);
                background-color: var(--typography-code-background-color);
                position: relative;
                cursor: crosshair;
            "
        >
            <div
                {..movable_attrs}
                tabindex="0"
                style=move || format!("
                    border: 0.15em solid cyan;
                    padding: 0.5em 1em;
                    position: absolute;
                    cursor: grab;
                    user-select: none;
                    left: {}px;
                    top: {}px;
                ",
                    pixel_position.get().0,
                    pixel_position.get().1
                )
            >
                "Click anywhere!"
            </div>
        </div>

        <p style="font-size: 0.9em; font-style: italic;">
            "Click anywhere in the container to move the element there, or drag it directly."
        </p>
    }
}
