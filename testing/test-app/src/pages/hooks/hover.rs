use leptonic::{
    IntoAttrs,
    atoms::hoverable::Hoverable,
    hooks::interactions::{HoverEndEvent, HoverStartEvent, UseHoverInput, use_hover},
};
use leptos::prelude::*;

/// `use_hover` (react-aria's `useHover.test.js`): a target with an inner element, which can be
/// disabled while hovered and whose inner button removes itself. Hover events are appended to
/// `#test-hover-log` as `start:<pointer type>:<target id>`, `change:<bool>`, `end:...`.
#[component]
pub fn PageHookHover() -> impl IntoView {
    let log = RwSignal::new(Vec::<String>::new());
    let disabled = RwSignal::new(false);
    let inner_shown = RwSignal::new(true);
    let hover = use_hover(UseHoverInput {
        is_disabled: disabled.into(),
        on_hover_start: Some(Callback::new(move |e: HoverStartEvent| {
            log.update(|l| {
                l.push(format!("start:{}:{}", e.pointer_type, e.target.id()));
            });
        })),
        on_hover_end: Some(Callback::new(move |e: HoverEndEvent| {
            log.update(|l| {
                l.push(format!("end:{}:{}", e.pointer_type, e.target.id()));
            });
        })),
        on_hover_change: Some(Callback::new(move |hovered: bool| {
            log.update(|l| l.push(format!("change:{hovered}")));
        })),
    });
    let is_hovered = hover.is_hovered;

    view! {
        <div id="test-page-hook-hover">
            <h1>"use_hover"</h1>
            <p id="test-hover-away">"Away from the target"</p>
            // The `Hoverable` atom marks its child.
            <Hoverable>
                <span id="test-hoverable">"Hoverable"</span>
            </Hoverable>
            <div
                id="test-hover-target"
                style="display: inline-block; padding: 4px; border: 1px solid"
                data-hovered=move || is_hovered.get().then_some("true")
                {..hover.props.into_attrs()}
            >
                <span id="test-hover-inner">"Inner"</span>
                <Show when=move || inner_shown.get()>
                    <button id="test-hover-remove" on:click=move |_| inner_shown.set(false)>
                        "Remove me"
                    </button>
                </Show>
            </div>
            <p>
                <button id="test-hover-disable" on:click=move |_| disabled.update(|d| *d = !*d)>
                    "Toggle disabled"
                </button>
            </p>
            <div>"Log: " <span id="test-hover-log">{move || log.get().join(",")}</span></div>
        </div>
    }
}
