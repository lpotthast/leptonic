use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn ButtonWrapperDemo() -> impl IntoView {
    let last_action = RwSignal::new(None::<&str>);

    view! {
        <ButtonWrapper>
            <Button variant=ButtonVariant::Flat on_press=move |_| last_action.set(Some("discard"))>"Discard"</Button>
            <Button variant=ButtonVariant::Outlined on_press=move |_| last_action.set(Some("save draft"))>"Save draft"</Button>
            <Button on_press=move |_| last_action.set(Some("publish"))>"Publish"</Button>
        </ButtonWrapper>
        <p class="demo-status">
            {move || match last_action.get() {
                Some(action) => format!("Last action: {action}."),
                None => "No action yet.".to_owned(),
            }}
        </p>
    }
}
