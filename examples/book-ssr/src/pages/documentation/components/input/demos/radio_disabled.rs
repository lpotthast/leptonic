use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn RadioDisabledDemo() -> impl IntoView {
    let (checked_disabled, set_checked_disabled) = signal(false);

    view! {
        <Radio disabled=true checked=checked_disabled set_checked=set_checked_disabled />
        <Button variant=ButtonVariant::Flat color=ButtonColor::Secondary size=ButtonSize::Small on_press=move |_| set_checked_disabled.set(!checked_disabled.get_untracked())>"TOGGLE"</Button>
    }
}
