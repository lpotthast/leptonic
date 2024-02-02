use indoc::indoc;
use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn PageCheckbox() -> impl IntoView {
    let (checked, set_checked) = create_signal(false);

    view! {
        <H1>"Checkbox"</H1>

        <P>"Checkbox..."</P>

        <Code>
            {indoc!(r"
                <Checkbox checked=checked set_checked=set_checked />
            ")}
        </Code>

        <Checkbox checked=checked set_checked=set_checked />

        <span>"checked: " {move || checked.get()}</span>

        <H2>"Disabled"</H2>


        <Checkbox disabled=true checked=checked set_checked=set_checked />
    }
}
