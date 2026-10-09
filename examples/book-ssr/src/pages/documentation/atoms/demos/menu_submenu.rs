use leptonic::{
    atoms,
    hooks::{
        collections::{Key, use_collection},
        overlay::Placement,
    },
};
use leptos::prelude::*;

#[component]
pub fn MenuSubmenuDemo() -> impl IntoView {
    let last_action = RwSignal::new(None::<Key>);
    let on_action = move |key: Key| last_action.set(Some(key));
    let file = use_collection(|b| {
        b.item("new", "New");
        b.item("share", "Share");
        b.item("close", "Close");
    });
    let targets = use_collection(|b| {
        b.item("email", "Email");
        b.item("link", "Copy link");
    });

    view! {
        <atoms::menu::MenuTrigger>
            <atoms::button::Button classes="demo-btn">"File"</atoms::button::Button>
            <atoms::popover::Popover placement=Placement::BottomLeft offset=4.0>
                <atoms::menu::Menu collection=file on_action=on_action classes="demo-menu-list">
                    <atoms::menu::MenuItem key="new" classes="demo-menu-atom-item">"New"</atoms::menu::MenuItem>
                    // Hover "Share", press it, or press ArrowRight on it.
                    <atoms::menu::SubmenuTrigger key="share">
                        <atoms::menu::MenuItem key="share" classes="demo-menu-atom-item">
                            <atoms::menu::MenuItemLabel>"Share"</atoms::menu::MenuItemLabel>
                            // A decorative arrow, hidden from the item's accessible name.
                            <span class="demo-menu-atom-arrow" aria-hidden="true">"\u{203a}"</span>
                        </atoms::menu::MenuItem>
                        <atoms::popover::Popover offset=-4.0>
                            <atoms::menu::Menu collection=targets on_action=on_action classes="demo-menu-list">
                                <atoms::menu::MenuItems classes="demo-menu-atom-item" let:node>
                                    {node.text_value.to_string()}
                                </atoms::menu::MenuItems>
                            </atoms::menu::Menu>
                        </atoms::popover::Popover>
                    </atoms::menu::SubmenuTrigger>
                    <atoms::menu::MenuItem key="close" classes="demo-menu-atom-item">"Close"</atoms::menu::MenuItem>
                </atoms::menu::Menu>
            </atoms::popover::Popover>
        </atoms::menu::MenuTrigger>
        <p class="demo-status">
            {move || match last_action.get() {

                Some(key) => format!("Last action: {key}."),
                None => "No action yet.".to_owned(),
            }}
        </p>
    }
}
