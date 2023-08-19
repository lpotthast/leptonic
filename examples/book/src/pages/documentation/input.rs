use indoc::indoc;
use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn PageInput(cx: Scope) -> impl IntoView {
    let (text, set_text) = create_signal(cx, "text".to_owned());
    let (password, set_password) = create_signal(cx, "secret".to_owned());
    let (number, set_number) = create_signal(cx, 4.2);
    let number_string = Signal::derive(cx, move || format!("{:.1}", number.get()));

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

        <Input get=text set=set_text/>
        <P style="color: gray; margin-top: 0; font-style: italic;">"Text is: " {move || text.get()}</P>

        <H2>"Types"</H2>

        <P>
            "You can use the "<Code inline=true>"InputType"</Code>" enum, to either create a "
            <Code inline=true>"Text"</Code>
            ", "<Code inline=true>"Password"</Code>
            " or "<Code inline=true>"Number"</Code>
            " input, "<Code inline=true>"Text"</Code>" being the default type when unspecified."
        </P>

        <Code>
            {indoc!(r#"
                let (password, set_password) = create_signal(cx, "secret".to_owned());
                view! { cx,
                    <Input ty=InputType::Password get=password set=set_password/>
                }
            "#)}
        </Code>

        <Input get=password set=set_password ty=InputType::Password/>
        <P style="color: gray; margin-top: 0; font-style: italic;">"Password is: " {move || password.get()}</P>

        <P>
            "Note that the input is always given back to you as a "<Code inline=true>"String"</Code>" no matter the type."
            " When using a number input, you may want to convert the String like this: "
            <Code inline=true>"str::parse::<f64>(v.as_str()).ok()"</Code>" to receive an "<Code inline=true>"Option<f64>"</Code>
            ", being "<Code inline=true>"None"</Code>" on invalid input."
        </P>

        <Code>
            {indoc!(r#"
                let (number, set_number) = create_signal(cx, Some(42.0));
                let number_string = Signal::derive(cx, move || format!("{:.1}", number.get()));
                view! { cx,
                    <Input ty=InputType::Number { min: Some(0.0), max: Some(10.0), step: Some(0.1) }
                        get=number_string
                        set=create_callback(cx, move |v: String| {
                            if let Some(v) = str::parse::<f64>(v.as_str()).ok() { set_number.set(v) }
                        })
                    />
                }
            "#)}
        </Code>

        <Input ty=InputType::Number { min: Some(0.0), max: Some(10.0), step: Some(0.1) }
            get=number_string
            set=create_callback(cx, move |v: String| {
                if let Some(v) = str::parse::<f64>(v.as_str()).ok() { set_number.set(v) }
            })
        />
        <P style="color: gray; margin-top: 0; font-style: italic;">"Number is: " {move || number_string.get()}</P>

        <H2>"Value updates"</H2>

        <P>
            "An input can be used without providing the "<Code inline=true>"set"</Code>" prop. "
            "You will not be notified about changes to the input value. "
            "This can be useful when you know that the input will always be "<Code inline=true>"disabled"</Code>" and you never expect changes."
        </P>

        <P>
            "The "<Code inline=true>"set"</Code>" prop, providing you with new values whenever the inputs content changes"
            ", accepts an "<Code inline=true>"Out<String>"</Code>
            ", allowing you to either provide a "<Code inline=true>"WriteSignal"</Code>" whose value is set when the input changes"
            " or a custom "<Code inline=true>"Callback"</Code>" called whenever the input changes"
            ", allowing you to handle values by yourself if required."
        </P>

        <P>
            "You can define the "<Code inline=true>"set"</Code>" prop in one of the following ways."
        </P>

        <Code>
            {indoc!(r#"
                view! { cx,
                    <Input get=text set=set_text/>
                    <Input get=text set=create_callback(cx, move |v: String| set_text.set(v))/>
                }
            "#)}
        </Code>

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

        <Input get=label_input set=set_label_input label="This is my label"/>
        <Button variant=ButtonVariant::Flat size=ButtonSize::Small on_click=move |_| set_label_input.set("".to_owned())>"Clear input"</Button>

        <H2>"Styling"</H2>

        <P>"You may overwrite any of the following CSS variables to meet your styling needs."</P>

        <Code>
            {indoc!(r#"
                --input-padding
                --input-color
                --input-background-color
                --input-border:
                --input-border-bottom
                --input-border-radius
                --input-min-height
                --input-focused-border-color
            "#)}
        </Code>
    }
}
