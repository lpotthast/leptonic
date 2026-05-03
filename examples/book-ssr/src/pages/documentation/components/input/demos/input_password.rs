use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn InputPasswordDemo() -> impl IntoView {
    let (password, set_password) = signal("secret".to_owned());
    let (number, set_number) = signal(4.2);
    let (unrestricted_number, set_unrestricted_number) = signal(4.2);
    let number_string = Signal::derive(move || format!("{:.1}", number.get()));

    view! {
        <PasswordInput get=password set=set_password/>
        <p style="color: gray; margin-top: 0; font-style: italic;">"Password is: " {move || password.get()}</p>

        <NumberInput
            get=unrestricted_number
            set=set_unrestricted_number
        />

        <NumberInput min=0.0 max=10.0 step=0.1
            get=number
            set=set_number
        />
        <p style="color: gray; margin-top: 0; font-style: italic;">"Number is: " {move || number_string.get()}</p>
    }
}
