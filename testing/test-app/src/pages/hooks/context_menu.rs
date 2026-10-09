use leptonic::{
    IntoAttrs,
    hooks::interactions::{
        ContextMenuEvent, UseContextMenuInput, UsePressInput, use_context_menu, use_press,
    },
};
use leptos::{prelude::*, web_sys};

/// `use_context_menu` (react-aria's `useContextMenu.test.tsx`): an element requesting context
/// menus, one without a handler, each in a wrapper logging the `contextmenu` events reaching it,
/// and one whose own `use_press` gets the long press requesting the menu on iOS (as `use_button`
/// wires it). Everything goes to `#test-context-menu-log`.
#[component]
pub fn PageHookContextMenu() -> impl IntoView {
    let log = RwSignal::new(Vec::<String>::new());
    let push = move |entry: String| log.update(|l| l.push(entry));
    let request = move || {
        Some(Callback::new(move |e: ContextMenuEvent| {
            push(format!(
                "menu:{}:{}:{}",
                e.point.x,
                e.point.y,
                e.target.id()
            ));
        }))
    };
    let with_handler = use_context_menu(UseContextMenuInput {
        on_context_menu: request(),
    });
    let without_handler = use_context_menu(UseContextMenuInput::default());
    let with_long_press = use_context_menu(UseContextMenuInput {
        on_context_menu: request(),
    });
    let long_press = use_press(UsePressInput {
        long_press: with_long_press.long_press,
        ..UsePressInput::default()
    });
    let (long_press_attrs, long_press_styles) = long_press.props.into_parts();
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
            <style>
                ".test-context-menu-target { width: 100px; height: 50px; border: 1px solid }"
            </style>
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
            <div on:contextmenu=wrapper("long-press")>
                <div
                    id="test-context-menu-long-press"
                    class="test-context-menu-target"
                    tabindex="0"
                    {..with_long_press.props.into_attrs()}
                    {..long_press_attrs}
                    style=long_press_styles
                >
                    "Long press"
                </div>
            </div>
            <button id="test-context-menu-reset" on:click=move |_| log.set(Vec::new())>
                "Reset log"
            </button>
            <div>"Log: " <span id="test-context-menu-log">{move || log.get().join(",")}</span></div>
        </div>
    }
}
