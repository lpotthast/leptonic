use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn ButtonStyledConceptDemo() -> impl IntoView {
    view! {
        <Button on_press=move |_| {} color=ButtonColor::Primary variant=ButtonVariant::Filled>
            "Save"
        </Button>
    }
}
