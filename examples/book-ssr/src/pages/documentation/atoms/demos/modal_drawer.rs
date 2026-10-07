use leptonic::atoms::{
    button::Button,
    dialog::Dialog,
    modal::{ModalBackdrop, ModalContent},
};
use leptos::prelude::*;
use leptos_icons::Icon;

const FOLDERS: [&str; 3] = ["Inbox", "Sent", "Archive"];

/// A drawer: a modal whose panel slides in from the left edge. Choosing a folder, the close button, Escape or a
/// press outside closes it.
#[component]
pub fn ModalDrawerDemo() -> impl IntoView {
    let is_open = RwSignal::new(false);
    let (folder, set_folder) = signal(FOLDERS[0]);

    view! {
        <Button on_press=move |_| is_open.set(true) classes="demo-btn">"Folders"</Button>
        <p class="demo-status">{move || format!("Showing {}.", folder.get())}</p>

        // The backdrop holds the panel at the left edge; the panel slides in and out (see the styles).
        <ModalBackdrop is_open=is_open set_open=is_open is_dismissable=true classes="demo-drawer-backdrop">
            <ModalContent classes="demo-drawer">
                <Dialog aria_label="Folders" classes="demo-drawer-dialog">
                    <Button on_press=move |_| is_open.set(false) aria_label="Close folders" classes="demo-drawer-close">
                        <Icon icon=icondata::BsXLg/>
                    </Button>
                    {FOLDERS
                        .iter()
                        .map(|&name| view! {
                            <Button
                                on_press=move |_| {
                                    set_folder.set(name);
                                    is_open.set(false);
                                }
                                classes="demo-drawer-item"
                            >
                                {name}
                            </Button>
                        })
                        .collect_view()}
                </Dialog>
            </ModalContent>
        </ModalBackdrop>
    }
}
