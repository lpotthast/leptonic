use leptonic::hooks::*;
use leptos::prelude::*;

#[component]
pub fn AxisExample() -> impl IntoView {
    let UseMoveReturn {
        props: h_props,
        constraint: h_constraint,
        ..
    } = use_move(UseMoveInput {
        disabled: false.into(),
        axis: Signal::derive(|| Some(MoveAxis::Horizontal)),
        is_rtl: false,
        on_move_start: None,
        on_move: None,
        on_move_end: None,
        on_position_change: None,
        constraint: Some(MoveConstraint::Bounds),
        allow_container_click: false,
        initial_position: None,
    });
    let h_c = h_constraint.unwrap();
    let h_pixel_position = h_c.pixel_position;
    let h_container_attrs = h_c.container_props.into_attrs();
    let h_movable_attrs = h_props.into_attrs();

    let UseMoveReturn {
        props: v_props,
        constraint: v_constraint,
        ..
    } = use_move(UseMoveInput {
        disabled: false.into(),
        axis: Signal::derive(|| Some(MoveAxis::Vertical)),
        is_rtl: false,
        on_move_start: None,
        on_move: None,
        on_move_end: None,
        on_position_change: None,
        constraint: Some(MoveConstraint::Bounds),
        allow_container_click: false,
        initial_position: None,
    });
    let v_c = v_constraint.unwrap();
    let v_pixel_position = v_c.pixel_position;
    let v_container_attrs = v_c.container_props.into_attrs();
    let v_movable_attrs = v_props.into_attrs();

    view! {
        <div style="display: flex; gap: 1em; flex-wrap: wrap;">
            <div style="flex: 1; min-width: 200px;">
                <p><strong>"Horizontal Only"</strong></p>
                <div
                    {..h_container_attrs}
                    style="
                        width: 100%;
                        height: 4em;
                        touch-action: none;
                        border-radius: var(--typography-code-border-radius);
                        background-color: var(--typography-code-background-color);
                        position: relative;
                    "
                >
                    <div
                        {..h_movable_attrs}
                        tabindex="0"
                        style=move || format!("
                            border: 0.15em solid orange;
                            padding: 0.3em 0.6em;
                            position: absolute;
                            cursor: ew-resize;
                            user-select: none;
                            left: {}px;
                            top: {}px;
                        ",
                            h_pixel_position.get().0,
                            h_pixel_position.get().1
                        )
                    >
                        "H"
                    </div>
                </div>
            </div>

            <div style="flex: 1; min-width: 200px;">
                <p><strong>"Vertical Only"</strong></p>
                <div
                    {..v_container_attrs}
                    style="
                        width: 100%;
                        height: 8em;
                        touch-action: none;
                        border-radius: var(--typography-code-border-radius);
                        background-color: var(--typography-code-background-color);
                        position: relative;
                    "
                >
                    <div
                        {..v_movable_attrs}
                        tabindex="0"
                        style=move || format!("
                            border: 0.15em solid purple;
                            padding: 0.3em 0.6em;
                            position: absolute;
                            cursor: ns-resize;
                            user-select: none;
                            left: {}px;
                            top: {}px;
                        ",
                            v_pixel_position.get().0,
                            v_pixel_position.get().1
                        )
                    >
                        "V"
                    </div>
                </div>
            </div>
        </div>
    }
}
