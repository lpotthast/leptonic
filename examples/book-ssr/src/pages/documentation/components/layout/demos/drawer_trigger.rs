use leptonic::{
    atoms::dialog::{DialogTitle, DialogTrigger},
    components::prelude::*,
};
use leptos::prelude::*;

/// Filters in a drawer on the right, opened by its `DialogTrigger`: the trigger owns the open state and gives the
/// button `aria-expanded`. Escape or a press outside closes the drawer.
#[component]
pub fn DrawerTriggerDemo() -> impl IntoView {
    let open_issues = RwSignal::new(true);
    let closed_issues = RwSignal::new(false);

    view! {
        <DialogTrigger>
            <Button>"Filters"</Button>
            // Named by its title.
            <Drawer side=DrawerSide::Right classes="demo-drawer">
                <DialogTitle classes="demo-overlay-title">"Filters"</DialogTitle>
                <Checkbox is_selected=open_issues set_selected=open_issues>"Open issues"</Checkbox>
                <Checkbox is_selected=closed_issues set_selected=closed_issues>"Closed issues"</Checkbox>
            </Drawer>
        </DialogTrigger>
        <p class="demo-status">
            {move || match (open_issues.get(), closed_issues.get()) {
                (true, true) => "Showing all issues.",
                (true, false) => "Showing open issues.",
                (false, true) => "Showing closed issues.",
                (false, false) => "Showing no issues.",
            }}
        </p>
    }
}
