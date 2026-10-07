use leptonic::{components::prelude::*, prelude::icondata};
use leptos::prelude::*;

const FOLDERS: [&str; 3] = ["Inbox", "Sent", "Archive"];

/// A menu sliding in from the left. Choosing a folder, the close button, Escape or a press outside closes it.
#[component]
pub fn DrawerLeftDemo() -> impl IntoView {
    let is_open = RwSignal::new(false);
    let (folder, set_folder) = signal(FOLDERS[0]);

    view! {
        <Button on_press=move |_| is_open.set(true)>"Folders"</Button>
        <p class="demo-status">{move || format!("Showing {}.", folder.get())}</p>

        <Drawer is_open=is_open set_open=is_open aria_label="Folders" classes="demo-drawer">
            <div class="demo-drawer-header">
                <Button on_press=move |_| is_open.set(false) variant=ButtonVariant::Flat attr:aria-label="Close folders">
                    <Icon icon=icondata::BsXLg/>
                </Button>
            </div>
            {FOLDERS
                .iter()
                .map(|&name| view! {
                    <Button on_press=move |_| { set_folder.set(name); is_open.set(false); } variant=ButtonVariant::Flat>
                        {name}
                    </Button>
                })
                .collect_view()}
        </Drawer>
    }
}
