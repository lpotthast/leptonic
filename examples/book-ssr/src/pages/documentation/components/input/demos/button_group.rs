use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn ButtonGroupDemo() -> impl IntoView {
    let last_action = RwSignal::new(None::<&str>);

    view! {
        <ButtonGroup>
            <Button on_press=move |_| last_action.set(Some("cut"))>"Cut"</Button>
            <Button on_press=move |_| last_action.set(Some("copy"))>"Copy"</Button>
            <Button on_press=move |_| last_action.set(Some("paste"))>"Paste"</Button>
        </ButtonGroup>
        <p class="demo-status">
            {move || match last_action.get() {
                Some(action) => format!("Last action: {action}."),
                None => "No action yet.".to_owned(),
            }}
        </p>
    }
}
