use leptonic::{
    components::prelude::*,
    hooks::{Key, Orientation},
};
use leptos::prelude::*;

#[component]
pub fn CheckboxGroupDemo() -> impl IntoView {
    let (channels, set_channels) = signal(vec![Key::from("email")]);

    view! {
        <CheckboxGroup
            label="Notify me by"
            description="Choose any number of channels."
            orientation=Orientation::Horizontal
            default_value=vec![Key::from("email")]
            on_change=move |value| set_channels.set(value)
        >
            <Checkbox value="email">"Email"</Checkbox>
            <Checkbox value="sms">"SMS"</Checkbox>
            <Checkbox value="push">"Push notification"</Checkbox>
        </CheckboxGroup>
        <p class="demo-status">
            {move || {
                let channels = channels.get().iter().map(ToString::to_string).collect::<Vec<_>>();
                if channels.is_empty() { "No notifications".to_owned() } else { format!("Channels: {}", channels.join(", ")) }
            }}
        </p>
    }
}
