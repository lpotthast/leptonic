use leptonic::hooks::*;
use leptos::prelude::*;

#[component]
pub fn ConstrainCenterExample() -> impl IntoView {
    let UseMoveReturn {
        props: bounds_props,
        constraint: bounds_constraint,
        ..
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
    let bounds_c = bounds_constraint.unwrap();
    let bounds_pixel_position = bounds_c.pixel_position;
    let bounds_container_attrs = bounds_c.container_props.into_attrs();
    let bounds_movable_attrs = bounds_props.into_attrs();

    let UseMoveReturn {
        props: center_props,
        constraint: center_constraint,
        ..
    } = use_move(UseMoveInput {
        disabled: false.into(),
        axis: Signal::derive(|| None),
        is_rtl: false,
        on_move_start: None,
        on_move: None,
        on_move_end: None,
        on_position_change: None,
        constraint: Some(MoveConstraint::Center),
        allow_container_click: false,
        initial_position: None,
    });
    let center_c = center_constraint.unwrap();
    let center_pixel_position = center_c.pixel_position;
    let center_container_attrs = center_c.container_props.into_attrs();
    let center_movable_attrs = center_props.into_attrs();

    view! {
        <div style="display: flex; gap: 1em; flex-wrap: wrap;">
            <div style="flex: 1; min-width: 200px;">
                <p><strong>"Constrain Bounds (default)"</strong></p>
                <div
                    {..bounds_container_attrs}
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
                        {..bounds_movable_attrs}
                        tabindex="0"
                        style=move || format!("
                            border: 0.15em solid green;
                            padding: 0.5em 1em;
                            position: absolute;
                            cursor: grab;
                            user-select: none;
                            left: {}px;
                            top: {}px;
                        ",
                            bounds_pixel_position.get().0,
                            bounds_pixel_position.get().1
                        )
                    >
                        "Bounds"
                    </div>
                </div>
                <p style="font-size: 0.85em;">"Element stays fully inside"</p>
            </div>

            <div style="flex: 1; min-width: 200px;">
                <p><strong>"Constrain Center"</strong></p>
                <div
                    {..center_container_attrs}
                    style="
                        width: 100%;
                        height: 8em;
                        touch-action: none;
                        border-radius: var(--typography-code-border-radius);
                        background-color: var(--typography-code-background-color);
                        position: relative;
                        overflow: visible;
                    "
                >
                    <div
                        {..center_movable_attrs}
                        tabindex="0"
                        style=move || format!("
                            border: 0.15em solid red;
                            padding: 0.5em 1em;
                            position: absolute;
                            cursor: grab;
                            user-select: none;
                            left: {}px;
                            top: {}px;
                        ",
                            center_pixel_position.get().0,
                            center_pixel_position.get().1
                        )
                    >
                        "Center"
                    </div>
                </div>
                <p style="font-size: 0.85em;">"Element center stays inside"</p>
            </div>
        </div>
    }
}
