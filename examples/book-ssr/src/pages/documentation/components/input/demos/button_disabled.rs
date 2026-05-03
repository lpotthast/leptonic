use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn ButtonDisabledDemo() -> impl IntoView {
    let (disabled, set_disabled) = signal(false);

    view! {
        <div>
            "Disable: " <Toggle state=disabled set_state=set_disabled/>
        </div>

        <ButtonWrapper>
            <Button on_press=move |_| {} disabled=true>"Always Disabled"</Button>
            <Button on_press=move |_| {} disabled=disabled>"Disabled"</Button>
            <Button on_press=move |_| {} disabled=Signal::derive(move || !disabled.get())>"!Disabled"</Button>
        </ButtonWrapper>
    }
}
