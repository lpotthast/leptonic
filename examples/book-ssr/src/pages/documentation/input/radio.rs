use indoc::indoc;
use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn PageRadio() -> impl IntoView {
    let (checked, set_checked) = create_signal(false);
    let (checked2, set_checked2) = create_signal(false);
    let (checked3, set_checked3) = create_signal(false);
    let (checked_disabled, set_checked_disabled) = create_signal(false);

    view! {
        <H1>"Radio"</H1>

        <P>"Radio..."</P>

        <Code>
            {indoc!(r"
                let (checked, set_checked) = create_signal(false);
                view! {
                    <Radio checked=checked set_checked=set_checked />
                }
            ")}
        </Code>

        <Radio checked=checked set_checked=set_checked />

        <span>"checked: " {move || checked.get()}</span>

        <H2>"Radio groups"</H2>

        <Code>
            {indoc!(r"
                <RadioGroup>
                    <Radio checked=checked2 set_checked=set_checked2 />
                    <Radio checked=checked3 set_checked=set_checked3 />
                </RadioGroup>
            ")}
        </Code>

        <RadioGroup>
            <Radio checked=checked2 set_checked=set_checked2 />
            <Radio checked=checked3 set_checked=set_checked3 />
        </RadioGroup>

        <H2>"Disabled"</H2>

        <P>"Radio buttons support the " <Code inline=true>"disabled"</Code> " property, making them unmodifiable if set true."</P>

        <Code>
            {indoc!(r"
                <Radio disabled=true checked=checked set_checked=set_checked />
            ")}
        </Code>

        <Radio disabled=true checked=checked_disabled set_checked=set_checked_disabled />
        <Button variant=ButtonVariant::Flat color=ButtonColor::Secondary size=ButtonSize::Small on_click=move |_| set_checked_disabled.set(!checked_disabled.get_untracked())>"TOGGLE"</Button>

        <H2>"Styling"</H2>

        <P>"You may overwrite any of the following CSS variables to meet your styling needs."</P>

        <Code>
            {indoc!(r"
                --radio-size
                --radio-fill-size
                --radio-border
                --radio-border-radius
                --radio-border-color
                --radio-hover-border-color
                --radio-checked-border-color
                --radio-background-color
                --radio-checked-fill-background-color
                --radio-disabled-filter
            ")}
        </Code>
    }
}
