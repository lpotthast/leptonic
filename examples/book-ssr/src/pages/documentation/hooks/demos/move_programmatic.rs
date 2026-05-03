use leptonic::hooks::*;
use leptos::prelude::*;

#[component]
pub fn ProgrammaticExample() -> impl IntoView {
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
        allow_container_click: false,
        initial_position: None,
    });
    let c = constraint.unwrap();
    let normalized_position = c.normalized_position;
    let pixel_position = c.pixel_position;
    let set_position = c.set_position;
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
            "
        >
            <div
                {..movable_attrs}
                tabindex="0"
                style=move || format!("
                    border: 0.15em solid yellow;
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
                "Programmable"
            </div>
        </div>

        <div style="display: flex; gap: 0.5em; flex-wrap: wrap; margin-top: 0.5em;">
            <button
                on:click=move |_| set_position.run(NormalizedPosition { x: 0.0, y: 0.0 })
                style="padding: 0.3em 0.6em; cursor: pointer;"
            >
                "Top-Left"
            </button>
            <button
                on:click=move |_| set_position.run(NormalizedPosition { x: 0.5, y: 0.0 })
                style="padding: 0.3em 0.6em; cursor: pointer;"
            >
                "Top-Center"
            </button>
            <button
                on:click=move |_| set_position.run(NormalizedPosition { x: 1.0, y: 0.0 })
                style="padding: 0.3em 0.6em; cursor: pointer;"
            >
                "Top-Right"
            </button>
            <button
                on:click=move |_| set_position.run(NormalizedPosition { x: 0.5, y: 0.5 })
                style="padding: 0.3em 0.6em; cursor: pointer;"
            >
                "Center"
            </button>
            <button
                on:click=move |_| set_position.run(NormalizedPosition { x: 0.0, y: 1.0 })
                style="padding: 0.3em 0.6em; cursor: pointer;"
            >
                "Bottom-Left"
            </button>
            <button
                on:click=move |_| set_position.run(NormalizedPosition { x: 1.0, y: 1.0 })
                style="padding: 0.3em 0.6em; cursor: pointer;"
            >
                "Bottom-Right"
            </button>
        </div>

        <p style="font-size: 0.9em;">
            "Normalized: ("
            { move || format!("{:.2}", normalized_position.get().x) }
            ", "
            { move || format!("{:.2}", normalized_position.get().y) }
            ")"
        </p>
    }
}
