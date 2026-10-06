use leptonic::{
    atoms::prelude as atoms,
    hooks::{
        Placement,
        collections::{Key, use_collection},
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
        <atoms::MenuTrigger>
            <atoms::Button classes="demo-btn">"File"</atoms::Button>
            <atoms::Popover placement=Placement::BottomLeft offset=4.0>
                <atoms::Menu collection=file on_action=on_action classes="demo-menu-list">
                    <atoms::MenuItem key="new" classes="demo-menu-atom-item">"New"</atoms::MenuItem>
                    // Hover "Share", press it, or press ArrowRight on it.
                    <atoms::SubmenuTrigger key="share">
                        <atoms::MenuItem key="share" classes="demo-menu-atom-item">
                            <atoms::MenuItemLabel>"Share"</atoms::MenuItemLabel>
                            // A decorative arrow, hidden from the item's accessible name.
                            <span class="demo-menu-atom-arrow" aria-hidden="true">"\u{203a}"</span>
                        </atoms::MenuItem>
                        <atoms::Popover offset=-4.0>
                            <atoms::Menu collection=targets on_action=on_action classes="demo-menu-list">
                                <atoms::MenuItems classes="demo-menu-atom-item" let:node>
                                    {node.text_value.to_string()}
                                </atoms::MenuItems>
                            </atoms::Menu>
                        </atoms::Popover>
                    </atoms::SubmenuTrigger>
                    <atoms::MenuItem key="close" classes="demo-menu-atom-item">"Close"</atoms::MenuItem>
                </atoms::Menu>
            </atoms::Popover>
        </atoms::MenuTrigger>
        <p class="demo-status">
            {move || match last_action.get() {

                Some(key) => format!("Last action: {key}."),
                None => "No action yet.".to_owned(),
            }}
        </p>
    }
}
