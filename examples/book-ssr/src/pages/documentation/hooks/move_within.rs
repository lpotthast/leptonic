use leptonic::components::prelude::*;
use leptonic::hooks::*;
use leptos::prelude::*;
use ringbuf::traits::{Consumer, RingBuffer};
use ringbuf::HeapRb;

use crate::pages::documentation::article::Article;
use crate::pages::documentation::toc::Toc;

#[component]
pub fn PageUseMoveWithin() -> impl IntoView {
    view! {
        <Article>
            <h1 id="use_move_within" class="anchor">
                "use_move_within"
                <AnchorLink href="#use_move_within" description="Direct link to article header"/>
            </h1>

            <p>"Constrain element movement within a container boundary."</p>

            <h2 id="basic" class="anchor">
                "Basic Usage"
                <AnchorLink href="#basic" description="Direct link to section"/>
            </h2>

            <p>"The simplest usage: drag an element freely within a container."</p>

            <BasicExample/>

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
                Toc::Leaf { title: "use_move_within", link: "#use_move_within" },
                Toc::Leaf { title: "Basic Usage", link: "#basic" },
                Toc::Leaf { title: "Axis Constraint", link: "#axis-constraint" },
                Toc::Leaf { title: "Container Click", link: "#container-click" },
                Toc::Leaf { title: "Constrain Center", link: "#constrain-center" },
                Toc::Leaf { title: "Programmatic Control", link: "#programmatic" },
            ]
        }/>
    }
}

#[component]
fn BasicExample() -> impl IntoView {
    let (events, set_events) = signal(HeapRb::<Oco<'static, str>>::new(20));

    let UseMoveWithinReturn {
        container_props,
        movable_props,
        pixel_position,
        normalized_position,
        is_moving,
        ..
    } = use_move_within(UseMoveWithinInput {
        axis: MoveAxis::Both.into(),
        disabled: false.into(),
        on_move_start: Some(Callback::new(move |e: MoveWithinStartEvent| {
            set_events.update(move |events| {
                events.push_overwrite(Oco::Owned(format!(
                    "Start: norm=({:.2}, {:.2})",
                    e.normalized_x, e.normalized_y
                )));
            });
        })),
        on_move: Some(Callback::new(move |e: MoveWithinEvent| {
            set_events.update(move |events| {
                events.push_overwrite(Oco::Owned(format!(
                    "Move: pixel=({:.0}, {:.0}) delta=({:.1}, {:.1})",
                    e.pixel_x, e.pixel_y, e.delta_x, e.delta_y
                )));
            });
        })),
        on_move_end: Some(Callback::new(move |e: MoveWithinEndEvent| {
            set_events.update(move |events| {
                events.push_overwrite(Oco::Owned(format!(
                    "End: norm=({:.2}, {:.2})",
                    e.normalized_x, e.normalized_y
                )));
            });
        })),
        ..Default::default()
    });
    let container_attrs = container_props.into_attrs();
    let movable_attrs = movable_props.into_attrs();

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
    let UseMoveWithinReturn {
        container_props: h_container_props,
        movable_props: h_movable_props,
        pixel_position: h_pixel_position,
        ..
    } = use_move_within(UseMoveWithinInput {
        axis: MoveAxis::Horizontal.into(),
        disabled: false.into(),
        ..Default::default()
    });
    let h_container_attrs = h_container_props.into_attrs();
    let h_movable_attrs = h_movable_props.into_attrs();

    let UseMoveWithinReturn {
        container_props: v_container_props,
        movable_props: v_movable_props,
        pixel_position: v_pixel_position,
        ..
    } = use_move_within(UseMoveWithinInput {
        axis: MoveAxis::Vertical.into(),
        disabled: false.into(),
        ..Default::default()
    });
    let v_container_attrs = v_container_props.into_attrs();
    let v_movable_attrs = v_movable_props.into_attrs();

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
    let UseMoveWithinReturn {
        container_props,
        movable_props,
        pixel_position,
        ..
    } = use_move_within(UseMoveWithinInput {
        axis: MoveAxis::Both.into(),
        disabled: false.into(),
        allow_container_click: true,
        ..Default::default()
    });
    let container_attrs = container_props.into_attrs();
    let movable_attrs = movable_props.into_attrs();

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
    let UseMoveWithinReturn {
        container_props: bounds_container_props,
        movable_props: bounds_movable_props,
        pixel_position: bounds_pixel_position,
        ..
    } = use_move_within(UseMoveWithinInput {
        axis: MoveAxis::Both.into(),
        disabled: false.into(),
        constrain_center: false,
        ..Default::default()
    });
    let bounds_container_attrs = bounds_container_props.into_attrs();
    let bounds_movable_attrs = bounds_movable_props.into_attrs();

    let UseMoveWithinReturn {
        container_props: center_container_props,
        movable_props: center_movable_props,
        pixel_position: center_pixel_position,
        ..
    } = use_move_within(UseMoveWithinInput {
        axis: MoveAxis::Both.into(),
        disabled: false.into(),
        constrain_center: true,
        ..Default::default()
    });
    let center_container_attrs = center_container_props.into_attrs();
    let center_movable_attrs = center_movable_props.into_attrs();

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
    let UseMoveWithinReturn {
        container_props,
        movable_props,
        pixel_position,
        normalized_position,
        set_position,
        ..
    } = use_move_within(UseMoveWithinInput {
        axis: MoveAxis::Both.into(),
        disabled: false.into(),
        ..Default::default()
    });
    let container_attrs = container_props.into_attrs();
    let movable_attrs = movable_props.into_attrs();

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
