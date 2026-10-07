use leptonic::{components::prelude::*, prelude::*, utils::css::em};
use leptos::prelude::*;

#[component]
pub fn AppBarDemo() -> impl IntoView {
    let (last_action, set_last_action) = signal(None::<&'static str>);

    view! {
        // A scrolling frame standing in for the page: the bar sticks to its top.
        <div class="demo-app-bar-frame" role="region" aria-label="Scrolling page" tabindex="0">
            // A second banner on the page: its name tells it apart from the book's own app bar.
            <AppBar height=em(3.0) aria_label="My App" classes="demo-app-bar">
                <span class="demo-app-bar-title">"My App"</span>
                <div class="demo-app-bar-actions">
                    // Icon-only buttons are named with `aria-label`; the icons themselves are decorative.
                    <Button
                        variant=ButtonVariant::Flat
                        attr:aria-label="Notifications"
                        on_press=move |_| set_last_action.set(Some("Notifications"))
                    >
                        <Icon icon=icondata::BsBell/>
                    </Button>
                    <Button
                        variant=ButtonVariant::Flat
                        attr:aria-label="Log out"
                        on_press=move |_| set_last_action.set(Some("Log out"))
                    >
                        <Icon icon=icondata::BsBoxArrowRight/>
                    </Button>
                </div>
            </AppBar>

            <div class="demo-app-bar-content">
                {(0..8).map(|_| view! { <Skeleton animated=false height=em(3.0)/> }).collect_view()}
            </div>
        </div>
        <p class="demo-status">
            {move || match last_action.get() {
                Some(action) => format!("Last action: {action}."),
                None => "No action yet.".to_owned(),
            }}
        </p>
    }
}
