use leptonic::{
    atoms::{field::Label, input::Input, prelude as atoms, text_field::TextField},
    hooks::{
        SubmenuKind,
        collections::{Key, use_collection},
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
        <atoms::MenuTrigger>
            <atoms::Button classes="demo-btn">"File"</atoms::Button>
            <atoms::Popover offset=4.0>
                <atoms::Menu
                    collection=file
                    on_action=move |key: Key| last_action.set(Some(key))
                    classes="demo-menu-list"
                >
                    <atoms::MenuItem key="open" classes="demo-menu-atom-item">"Open"</atoms::MenuItem>
                    // Opens a dialog next to the item. Focus moves into it and stays there; Escape closes it.
                    <atoms::SubmenuTrigger key="rename" kind=SubmenuKind::Dialog>
                        <atoms::MenuItem key="rename" classes="demo-menu-atom-item">"Rename \u{2026}"</atoms::MenuItem>
                        <atoms::Popover offset=-4.0 classes="demo-popover">
                            <atoms::Dialog aria_label="Rename">
                                <TextField value=name set_value=name classes="demo-field">
                                    <Label classes="demo-field-label">"File name"</Label>
                                    <Input classes=["demo-input", "demo-text-input", "demo-atom-input"]/>
                                </TextField>
                            </atoms::Dialog>
                        </atoms::Popover>
                    </atoms::SubmenuTrigger>
                    <atoms::MenuItem key="delete" classes="demo-menu-atom-item">"Delete"</atoms::MenuItem>
                </atoms::Menu>
            </atoms::Popover>
        </atoms::MenuTrigger>
        <p class="demo-status">
            {move || format!("File: {}. ", name.get())}
            {move || match last_action.get() {
                Some(key) => format!("Last action: {key}."),
                None => "No action yet.".to_owned(),
            }}
        </p>
    }
}
