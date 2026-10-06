use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn PopoverControlledDemo() -> impl IntoView {
    let is_open = RwSignal::new(false);
    let email = RwSignal::new(true);

    view! {
        <div class="demo-popover-stage">
            // Bound to `is_open`: "Done" closes the popover, and dismissing it sets `is_open` to false.
            <Popover is_open=is_open set_open=is_open>
                <PopoverTrigger slot>
                    <Button>"Notifications"</Button>
                </PopoverTrigger>
                <div class="demo-popover-form">
                    <Switch is_selected=email set_selected=email>"Email me about replies"</Switch>
                    <Button on_press=move |_| is_open.set(false)>"Done"</Button>
                </div>
            </Popover>
        </div>
        <p class="demo-status">
            {move || {
                format!(
                    "Email notifications are {}. The popover is {}.",
                    if email.get() { "on" } else { "off" },
                    if is_open.get() { "open" } else { "closed" },
                )
            }}
        </p>
    }
}
