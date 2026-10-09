use leptonic::{
    IntoAttrs,
    hooks::interactions::{
        ContextMenuEvent, UseContextMenuInput, UseContextMenuReturn, use_context_menu,
    },
};
use leptos::prelude::*;

#[component]
pub fn ContextMenuDemo() -> impl IntoView {
    let last = RwSignal::new(None::<(f64, f64)>);
    let count = RwSignal::new(0_u32);

    let UseContextMenuReturn { props, .. } = use_context_menu(UseContextMenuInput {
        on_context_menu: Some(Callback::new(move |e: ContextMenuEvent| {
            last.set(Some((e.point.x, e.point.y)));
            count.update(|count| *count += 1);
        })),
    });

    view! {
        // Right click the area, or focus it and press Shift+F10 (or the context menu key).
        <div class="demo-context-target" tabindex="0" {..props.into_attrs()}>
            "Request a context menu here"
        </div>
        <p class="demo-status">
            {move || match last.get() {
                Some((x, y)) => {
                    let requests = match count.get() {
                        1 => "1 request".to_owned(),
                        n => format!("{n} requests"),
                    };
                    format!("{requests}, the last at {x:.0}, {y:.0} (from the area\u{2019}s top left corner).")
                }
                None => "No request yet.".to_owned(),
            }}
        </p>
    }
}
