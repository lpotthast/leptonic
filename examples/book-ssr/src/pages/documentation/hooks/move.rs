use leptonic::{components::prelude::*, hooks::*};
use leptos::{html, prelude::*};
use leptos_use::use_element_bounding;
use ringbuf::{
    traits::{Consumer, Observer, RingBuffer},
    HeapRb,
};

use crate::pages::documentation::{article::Article, toc::Toc};

#[component]
pub fn PageUseMove() -> impl IntoView {
    view! {
        <Article>
            <h1 id="use_move" class="anchor">
                "use_move"
                <AnchorLink href="#use_move" description="Direct link to article header"/>
            </h1>

            <p>"Track pointer and keyboard movement. Supports arrow key navigation, text selection management, and optional area-constrained movement via " <code>"MoveConstraint"</code> "."</p>

            <h2 id="basic-movement" class="anchor">
                "Basic Movement"
                <AnchorLink href="#basic-movement" description="Direct link to section"/>
            </h2>

            <p>"Unconstrained drag and keyboard movement with event logging."</p>

            <BasicMovementExample/>

            <h2 id="constrained" class="anchor">
                "Constrained Movement"
                <AnchorLink href="#constrained" description="Direct link to section"/>
            </h2>

            <p>"Constrain element movement within a container boundary using a " <code>"MoveConstraint"</code> " configuration."</p>

            <ConstrainedBasicExample/>

            <h2 id="axis-constraint" class="anchor">
                "Axis Constraint"
                <AnchorLink href="#axis-constraint" description="Direct link to section"/>
            </h2>

            <p>"Constrain movement to horizontal or vertical axis only."</p>

            <AxisExample/>

            <h2 id="container-click" class="anchor">
                "Container Click"
                <AnchorLink href="#container-click" description="Direct link to section"/>
            </h2>

            <p>"When enabled, clicking the container moves the element to that position."</p>

            <ContainerClickExample/>

            <h2 id="constrain-center" class="anchor">
                "Constrain Center"
                <AnchorLink href="#constrain-center" description="Direct link to section"/>
            </h2>

            <p>"Compare constraining element bounds vs element center."</p>

            <ConstrainCenterExample/>

            <h2 id="programmatic" class="anchor">
                "Programmatic Control"
                <AnchorLink href="#programmatic" description="Direct link to section"/>
            </h2>

            <p>"Use set_position to programmatically move the element."</p>

            <ProgrammaticExample/>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "use_move", link: "#use_move" },
                Toc::Leaf { title: "Basic Movement", link: "#basic-movement" },
                Toc::Leaf { title: "Constrained Movement", link: "#constrained" },
                Toc::Leaf { title: "Axis Constraint", link: "#axis-constraint" },
                Toc::Leaf { title: "Container Click", link: "#container-click" },
                Toc::Leaf { title: "Constrain Center", link: "#constrain-center" },
                Toc::Leaf { title: "Programmatic Control", link: "#programmatic" },
            ]
        }/>
    }
}

#[component]
fn BasicMovementExample() -> impl IntoView {
    let (events, set_events) = signal(HeapRb::<Oco<'static, str>>::new(50));
    let (left, set_left) = signal(0.0);
    let (top, set_top) = signal(0.0);

    let container: NodeRef<html::Div> = NodeRef::new();
    let container_bounding = use_element_bounding(container);

    let draggable: NodeRef<html::Div> = NodeRef::new();
    let draggable_bounding = use_element_bounding(draggable);

    let UseMoveReturn { props, .. } = use_move(UseMoveInput {
        disabled: false.into(),
        axis: None.into(),
        on_move_start: Some(Callback::new(move |e: MoveStartEvent| {
            set_events.update(move |events| {
                events.push_overwrite(Oco::Owned(format!(
                    "MoveStart {{ pointer: {}, page: ({}, {}) }}",
                    e.pointer_type, e.page_x, e.page_y
                )));
            });
        })),
        on_move: Some(Callback::new(move |e: MoveEvent| {
            set_left.update(move |l| *l += e.delta_x);
            set_top.update(move |l| *l += e.delta_y);
            set_events.update(move |events| {
                events.push_overwrite(Oco::Owned(format!(
                    "Move {{ dx: {}, dy: {}, pointer: {} }}",
                    e.delta_x, e.delta_y, e.pointer_type
                )));
            });
        })),
        on_move_end: Some(Callback::new(move |e: MoveEndEvent| {
            set_left.update(move |l| {
                *l = (*l).clamp(
                    0.0,
                    container_bounding.width.get_untracked()
                        - draggable_bounding.width.get_untracked(),
                )
            });
            set_top.update(move |t| {
                *t = (*t).clamp(
                    0.0,
                    container_bounding.height.get_untracked()
                        - draggable_bounding.height.get_untracked(),
                )
            });
            set_events.update(move |events| {
                events.push_overwrite(Oco::Owned(format!(
                    "MoveEnd {{ pointer: {} }}",
                    e.pointer_type
                )));
            });
        })),
        constraint: None,
    });

    let string = Memo::new(move |_| {
        events.with(|events| {
            let mut result = String::new();
            for e in events.iter().rev() {
                result.push_str(e.as_str());
                result.push('\n');
            }
            result
        })
    });

    view! {
        <Code>
            "..."
        </Code>

        // The `touch-action: none` is important. Browsers would otherwise interrupt touchmove events after a small delay!
        <div node_ref=container style="
            width: 100%;
            height: 10em;
            touch-action: none;
            border: none;
            border-radius: var(--typography-code-border-radius);
            background-color: var(--typography-code-background-color);
            color: var(--typography-code-color);
        ">
            <div
                {..props.into_attrs()}
                node_ref=draggable
                tabindex="0"
                style=move || format!("
                    border: 0.1em solid green;
                    padding: 0.5em 1em;
                    transition: none;
                    position: relative;
                    width: fit-content;
                    cursor: pointer;
                ")
                style:left=move || format!("{}px", left.get().clamp(
                    0.0,
                    container_bounding.width.get_untracked()
                        - draggable_bounding.width.get_untracked(),
                ))
                style:top=move || format!("{}px", top.get().clamp(
                    0.0,
                    container_bounding.height.get_untracked()
                        - draggable_bounding.height.get_untracked(),
                ))
            >
                "Drag me (or use arrow keys)"
            </div>
        </div>

        <p>"Last " { move || events.with(|events| events.occupied_len()) } " events: "</p>

        <pre style="
            width: 100%;
            height: 15em;
            overflow: auto;
            padding: var(--typography-code-padding);
            border: none;
            border-radius: var(--typography-code-border-radius);
            background-color: var(--typography-code-background-color);
            color: var(--typography-code-color);
        ">
            { move || string.get() }
        </pre>
    }
}

#[component]
fn ConstrainedBasicExample() -> impl IntoView {
    let (events, set_events) = signal(HeapRb::<Oco<'static, str>>::new(20));

    let UseMoveReturn {
        props,
        is_moving,
        constraint,
    } = use_move(UseMoveInput {
        disabled: false.into(),
        axis: Signal::derive(|| None),
        on_move_start: Some(Callback::new(move |e: MoveStartEvent| {
            set_events.update(move |events| {
                events.push_overwrite(Oco::Owned(format!("Start: pointer={}", e.pointer_type)));
            });
        })),
        on_move: Some(Callback::new(move |e: MoveEvent| {
            set_events.update(move |events| {
                events.push_overwrite(Oco::Owned(format!(
                    "Move: delta=({:.1}, {:.1})",
                    e.delta_x, e.delta_y
                )));
            });
        })),
        on_move_end: Some(Callback::new(move |e: MoveEndEvent| {
            set_events.update(move |events| {
                events.push_overwrite(Oco::Owned(format!("End: pointer={}", e.pointer_type)));
            });
        })),
        constraint: Some(MoveConstraint {
            is_rtl: false,
            constrain_center: false,
            allow_container_click: false,
            initial_position: None,
        }),
    });
    let c = constraint.unwrap();
    let normalized_position = c.normalized_position;
    let pixel_position = c.pixel_position;
    let container_attrs = c.container_props.into_attrs();
    let movable_attrs = props.into_attrs();

    let event_string = Memo::new(move |_| {
        events.with(|events| {
            let mut result = String::new();
            for e in events.iter().rev() {
                result.push_str(e.as_str());
                result.push('\n');
            }
            result
        })
    });

    view! {
        <div
            {..container_attrs}
            style="
                width: 100%;
                height: 12em;
                touch-action: none;
                border: none;
                border-radius: var(--typography-code-border-radius);
                background-color: var(--typography-code-background-color);
                color: var(--typography-code-color);
                position: relative;
            "
        >
            <div
                {..movable_attrs}
                tabindex="0"
                style=move || format!("
                    border: 0.15em solid {};
                    padding: 0.5em 1em;
                    position: absolute;
                    width: fit-content;
                    cursor: grab;
                    user-select: none;
                    left: {}px;
                    top: {}px;
                ",
                    if is_moving.get() { "var(--brand-color)" } else { "green" },
                    pixel_position.get().0,
                    pixel_position.get().1
                )
            >
                "Drag me"
            </div>
        </div>

        <p style="font-size: 0.9em;">
            "Position: ("
            { move || format!("{:.2}", normalized_position.get().0) }
            ", "
            { move || format!("{:.2}", normalized_position.get().1) }
            ") | Moving: "
            { move || if is_moving.get() { "Yes" } else { "No" } }
        </p>

        <pre style="
            width: 100%;
            height: 8em;
            overflow: auto;
            padding: var(--typography-code-padding);
            border: none;
            border-radius: var(--typography-code-border-radius);
            background-color: var(--typography-code-background-color);
            color: var(--typography-code-color);
        ">
            { move || event_string.get() }
        </pre>
    }
}

#[component]
fn AxisExample() -> impl IntoView {
    let UseMoveReturn {
        props: h_props,
        constraint: h_constraint,
        ..
    } = use_move(UseMoveInput {
        disabled: false.into(),
        axis: Signal::derive(|| Some(MoveAxis::Horizontal)),
        on_move_start: None,
        on_move: None,
        on_move_end: None,
        constraint: Some(MoveConstraint {
            is_rtl: false,
            constrain_center: false,
            allow_container_click: false,
            initial_position: None,
        }),
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
        on_move_start: None,
        on_move: None,
        on_move_end: None,
        constraint: Some(MoveConstraint {
            is_rtl: false,
            constrain_center: false,
            allow_container_click: false,
            initial_position: None,
        }),
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

#[component]
fn ContainerClickExample() -> impl IntoView {
    let UseMoveReturn {
        props, constraint, ..
    } = use_move(UseMoveInput {
        disabled: false.into(),
        axis: Signal::derive(|| None),
        on_move_start: None,
        on_move: None,
        on_move_end: None,
        constraint: Some(MoveConstraint {
            is_rtl: false,
            constrain_center: false,
            allow_container_click: true,
            initial_position: None,
        }),
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

#[component]
fn ConstrainCenterExample() -> impl IntoView {
    let UseMoveReturn {
        props: bounds_props,
        constraint: bounds_constraint,
        ..
    } = use_move(UseMoveInput {
        disabled: false.into(),
        axis: Signal::derive(|| None),
        on_move_start: None,
        on_move: None,
        on_move_end: None,
        constraint: Some(MoveConstraint {
            is_rtl: false,
            constrain_center: false,
            allow_container_click: false,
            initial_position: None,
        }),
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
        on_move_start: None,
        on_move: None,
        on_move_end: None,
        constraint: Some(MoveConstraint {
            is_rtl: false,
            constrain_center: true,
            allow_container_click: false,
            initial_position: None,
        }),
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

#[component]
fn ProgrammaticExample() -> impl IntoView {
    let UseMoveReturn {
        props, constraint, ..
    } = use_move(UseMoveInput {
        disabled: false.into(),
        axis: Signal::derive(|| None),
        on_move_start: None,
        on_move: None,
        on_move_end: None,
        constraint: Some(MoveConstraint {
            is_rtl: false,
            constrain_center: false,
            allow_container_click: false,
            initial_position: None,
        }),
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
                on:click=move |_| set_position.run((0.0, 0.0))
                style="padding: 0.3em 0.6em; cursor: pointer;"
            >
                "Top-Left"
            </button>
            <button
                on:click=move |_| set_position.run((0.5, 0.0))
                style="padding: 0.3em 0.6em; cursor: pointer;"
            >
                "Top-Center"
            </button>
            <button
                on:click=move |_| set_position.run((1.0, 0.0))
                style="padding: 0.3em 0.6em; cursor: pointer;"
            >
                "Top-Right"
            </button>
            <button
                on:click=move |_| set_position.run((0.5, 0.5))
                style="padding: 0.3em 0.6em; cursor: pointer;"
            >
                "Center"
            </button>
            <button
                on:click=move |_| set_position.run((0.0, 1.0))
                style="padding: 0.3em 0.6em; cursor: pointer;"
            >
                "Bottom-Left"
            </button>
            <button
                on:click=move |_| set_position.run((1.0, 1.0))
                style="padding: 0.3em 0.6em; cursor: pointer;"
            >
                "Bottom-Right"
            </button>
        </div>

        <p style="font-size: 0.9em;">
            "Normalized: ("
            { move || format!("{:.2}", normalized_position.get().0) }
            ", "
            { move || format!("{:.2}", normalized_position.get().1) }
            ")"
        </p>
    }
}
