use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn CardDemo() -> impl IntoView {
    let subscribed = RwSignal::new(false);

    view! {
        <Card>
            // The card's heading: one level below the heading of the section the card is in.
            <h2 class="demo-card-title">"Weekly digest"</h2>
            <p>"The most read articles of the week, every Monday morning."</p>
            <Button on_press=move |_| subscribed.update(|subscribed| *subscribed = !*subscribed)>
                {move || if subscribed.get() { "Unsubscribe" } else { "Subscribe" }}
            </Button>
            <p class="demo-status">
                {move || if subscribed.get() { "Subscribed." } else { "Not subscribed." }}
            </p>
        </Card>
    }
}
