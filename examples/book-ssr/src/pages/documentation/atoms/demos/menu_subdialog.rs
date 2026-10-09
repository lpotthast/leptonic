use leptonic::{
    atoms::{self, field::Label, input::Input, text_field::TextField},
    hooks::{
        collections::{Key, use_collection},
        menu::SubmenuKind,
    },
};
use leptos::prelude::*;

#[component]
pub fn MenuSubdialogDemo() -> impl IntoView {
    let name = RwSignal::new("report.pdf".to_owned());
    let last_action = RwSignal::new(None::<Key>);
    let file = use_collection(|b| {
        b.item("open", "Open");
        b.item("rename", "Rename");
        b.item("delete", "Delete");
    });

    view! {
        <atoms::menu::MenuTrigger>
            <atoms::button::Button classes="demo-btn">"File"</atoms::button::Button>
            <atoms::popover::Popover offset=4.0>
                <atoms::menu::Menu
                    collection=file
                    on_action=move |key: Key| last_action.set(Some(key))
                    classes="demo-menu-list"
                >
                    <atoms::menu::MenuItem key="open" classes="demo-menu-atom-item">"Open"</atoms::menu::MenuItem>
                    // Opens a dialog next to the item. Focus moves into it and stays there; Escape closes it.
                    <atoms::menu::SubmenuTrigger key="rename" kind=SubmenuKind::Dialog>
                        <atoms::menu::MenuItem key="rename" classes="demo-menu-atom-item">"Rename \u{2026}"</atoms::menu::MenuItem>
                        <atoms::popover::Popover offset=-4.0 classes="demo-popover">
                            <atoms::dialog::Dialog aria_label="Rename">
                                <TextField value=name set_value=name classes="demo-field">
                                    <Label classes="demo-field-label">"File name"</Label>
                                    <Input classes=["demo-input", "demo-text-input", "demo-atom-input"]/>
                                </TextField>
                            </atoms::dialog::Dialog>
                        </atoms::popover::Popover>
                    </atoms::menu::SubmenuTrigger>
                    <atoms::menu::MenuItem key="delete" classes="demo-menu-atom-item">"Delete"</atoms::menu::MenuItem>
                </atoms::menu::Menu>
            </atoms::popover::Popover>
        </atoms::menu::MenuTrigger>
        <p class="demo-status">
            {move || format!("File: {}. ", name.get())}
            {move || match last_action.get() {
                Some(key) => format!("Last action: {key}."),
                None => "No action yet.".to_owned(),
            }}
        </p>
    }
}
