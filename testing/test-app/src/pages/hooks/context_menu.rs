use leptonic::hooks::{ContextMenuEvent, IntoAttrs, UseContextMenuInput, use_context_menu};
use leptos::{prelude::*, web_sys};

/// `use_context_menu` (react-aria's `useContextMenu.test.tsx`): an element requesting context
/// menus and one without a handler, each in a wrapper logging the `contextmenu` events reaching
/// it. Everything goes to `#test-context-menu-log`.
#[component]
pub fn PageHookContextMenu() -> impl IntoView {
    let log = RwSignal::new(Vec::<String>::new());
    let push = move |entry: String| log.update(|l| l.push(entry));
    let with_handler = use_context_menu(UseContextMenuInput {
        on_context_menu: Some(Callback::new(move |e: ContextMenuEvent| {
            push(format!("menu:{}:{}:{}", e.x, e.y, e.target.id()));
        })),
    });
    let without_handler = use_context_menu(UseContextMenuInput::default());
    let wrapper = move |name: &'static str| {
        move |e: web_sys::MouseEvent| {
            let prevented = if e.default_prevented() {
                ":prevented"
            } else {
                ""
            };
            push(format!("{name}-wrapper{prevented}"));
        }
    };

    view! {
        <div id="test-page-hook-context-menu">
            <h1>"use_context_menu"</h1>
            <style>".test-context-menu-target { width: 100px; height: 50px; border: 1px solid }"</style>
            <div on:contextmenu=wrapper("handler")>
                <div
                    id="test-context-menu-handler"
                    class="test-context-menu-target"
                    tabindex="0"
                    {..with_handler.props.into_attrs()}
                >
                    "With handler"
                </div>
            </div>
            <div on:contextmenu=wrapper("none")>
                <div
                    id="test-context-menu-none"
                    class="test-context-menu-target"
                    tabindex="0"
                    {..without_handler.props.into_attrs()}
                >
                    "Without handler"
                </div>
            </div>
            <button id="test-context-menu-reset" on:click=move |_| log.set(Vec::new())>
                "Reset log"
            </button>
            <div>"Log: " <span id="test-context-menu-log">{move || log.get().join(",")}</span></div>
        </div>
    }
}
