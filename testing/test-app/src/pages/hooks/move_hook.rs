use leptonic::hooks::{IntoAttrs, MoveEndEvent, MoveEvent, MoveStartEvent, UseMoveInput, use_move};
use leptos::prelude::*;

/// `use_move` (react-aria's `useMove.test.js`): a movable element, one nested in another movable
/// one, inside a wrapper logging other key presses. Events are appended to `#test-move-log` as
/// `<element>:start:<pointer type>`, `<element>:move:<pointer type>:<dx>:<dy>`, `<element>:end:..`.
#[component]
pub fn PageHookMove() -> impl IntoView {
    let log = RwSignal::new(Vec::<String>::new());

    view! {
        <div id="test-page-hook-move">
            <h1>"use_move"</h1>
            <div on:keydown=move |e| log.update(|l| l.push(format!("keydown:{}", e.key())))>
                <Movable log name="single" />
            </div>
            <Movable log name="parent">
                <Movable log name="child" />
            </Movable>
            <button id="test-move-reset" on:click=move |_| log.set(Vec::new())>"Reset log"</button>
            <div>"Log: " <span id="test-move-log">{move || log.get().join(",")}</span></div>
        </div>
    }
}

#[component]
fn Movable(
    log: RwSignal<Vec<String>>,
    name: &'static str,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let push = move |entry: String| log.update(|l| l.push(format!("{name}:{entry}")));
    let movement = use_move(UseMoveInput {
        on_move_start: Some(Callback::new(move |e: MoveStartEvent| {
            push(format!("start:{}", e.pointer_type));
        })),
        on_move: Some(Callback::new(move |e: MoveEvent| {
            push(format!(
                "move:{}:{}:{}",
                e.pointer_type, e.delta_x, e.delta_y
            ));
        })),
        on_move_end: Some(Callback::new(move |e: MoveEndEvent| {
            push(format!("end:{}", e.pointer_type));
        })),
        ..UseMoveInput::default()
    });
    view! {
        <div
            id=format!("test-move-{name}")
            tabindex="0"
            style="display: inline-block; padding: 16px; border: 1px solid; touch-action: none"
            {..movement.props.into_attrs()}
        >
            {name}
            {children.map(|children| children())}
        </div>
    }
}
