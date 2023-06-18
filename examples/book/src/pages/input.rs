use indoc::indoc;
use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn PageInput(cx: Scope) -> impl IntoView {
    let (text, set_text) = create_signal(cx, "text".to_owned());
    let (password, set_password) = create_signal(cx, "secret".to_owned());
    let (number, set_number) = create_signal(cx, "number".to_owned());
    let (label_input, set_label_input) = create_signal(cx, "".to_owned());

    view! { cx,
        <H1>"Inputs"</H1>

        <P>"Creating an input is as simple as doing the following"</P>
        <Code>
            {indoc!(r#"
                let (text, set_text) = create_signal(cx, "text".to_owned());
                view! { cx,
                    <Input get=text set=set_text/>
                }
            "#)}
        </Code>

        <Input get=text set=move |v| set_text.set(v)/>

        <H2>"Types"</H2>

        <P>"You can use the " <Code inline=true>"InputType"</Code> " enum, to either create a text, password or number input."</P>

        <Code>
            {indoc!(r#"
                let (password, set_password) = create_signal(cx, "secret".to_owned());
                view! { cx,
                    <Input ty=InputType::Password get=text set=set_text/>
                }
            "#)}
        </Code>

        <Input get=password set=move |v| set_password.set(v) ty=InputType::Password/>

        <Input get=number set=move |v| set_number.set(v) ty=InputType::Number/>

        <H2>"Label"</H2>

        <P>"You can supply a label to the input. It is shown as the inputs placeholder when the input is empty."</P>

        <Code>
            {indoc!(r#"
                let (text, set_text) = create_signal(cx, "".to_owned());
                view! { cx,
                    <Input get=text set=set_text label="This is my label"/>
                    <Button
                        variant=ButtonVariant::Flat
                        size=ButtonSize::Small
                        on_click=move |_| set_text.set("".to_owned())>
                        "Clear input"
                    </Button>
                }
            "#)}
        </Code>

        <Input get=label_input set=move |v| set_label_input.set(v) label="This is my label"/>
        <Button variant=ButtonVariant::Flat size=ButtonSize::Small on_click=move |_| set_label_input.set("".to_owned())>"Clear input"</Button>
    }
}
