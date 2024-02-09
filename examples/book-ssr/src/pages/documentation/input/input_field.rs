use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::*;

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageInput() -> impl IntoView {
    let (text, set_text) = create_signal("text".to_owned());
    let (password, set_password) = create_signal("secret".to_owned());
    let (number, set_number) = create_signal(4.2);
    let number_string = Signal::derive(move || format!("{:.1}", number.get()));

    let (placeholder_input, set_placeholder_input) = create_signal(String::new());

    view! {
        <H1>"Inputs"</H1>

        <P>"Creating an input is as simple as doing the following"</P>

        <Code>
            {indoc!(r#"
                let (text, set_text) = create_signal("text".to_owned());
                view! {
                    <TextInput get=text set=set_text/>
                }
            "#)}
        </Code>

        <TextInput get=text set=set_text/>
        <P style="color: gray; margin-top: 0; font-style: italic;">"Text is: " {move || text.get()}</P>

        <H2>"Labeled"</H2>

        <P>"Wrap an input and a label to link them together."</P>

        <Code>
            {indoc!(r#"
                <FormControl>
                    <Label>
                        "Label"
                    </Label>
                    <TextInput get=text set=set_text/>
                </FormControl>
            "#)}
        </Code>

        <FormControl>
            <Label>
                "Label"
            </Label>
            <TextInput get=text set=set_text/>
        </FormControl>

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
                let (password, set_password) = create_signal("secret".to_owned());
                view! {
                    <PasswordInput get=password set=set_password/>
                }
            "#)}
        </Code>

        <PasswordInput get=password set=set_password/>
        <P style="color: gray; margin-top: 0; font-style: italic;">"Password is: " {move || password.get()}</P>

        <P>
            "Note that the input is always given back to you as a "<Code inline=true>"String"</Code>" no matter the type."
            " When using a number input, you may want to convert the String like this: "
            <Code inline=true>"str::parse::<f64>(v.as_str()).ok()"</Code>" to receive an "<Code inline=true>"Option<f64>"</Code>
            ", being "<Code inline=true>"None"</Code>" on invalid input."
        </P>

        <Code>
            {indoc!(r#"
                let (number, set_number) = create_signal(Some(42.0));
                let number_string = Signal::derive(move || format!("{:.1}", number.get()));
                view! {
                    <NumberInput min=0.0 max=10.0 step=0.1
                        get=number
                        set=set_number
                    />
                }
            "#)}
        </Code>

        <NumberInput min=0.0 max=10.0 step=0.1
            get=number
            set=set_number
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
            {indoc!(r"
                view! {
                    <Input get=text set=set_text/>
                    <Input get=text set=create_callback(move |v: String| set_text.set(v))/>
                }
            ")}
        </Code>

        <H2>"Placeholder"</H2>

        <P>"You can supply a placeholder to the input. It is shown as when the input is empty."</P>

        <Code>
            {indoc!(r#"
                let (text, set_text) = create_signal(String::new());
                view! {
                    <TextInput get=text set=set_text placeholder="This is a placeholder"/>
                    <Button
                        variant=ButtonVariant::Flat
                        size=ButtonSize::Small
                        on_press=move |_| set_text.set(String::new())>
                        "Clear input"
                    </Button>
                }
            "#)}
        </Code>

        <TextInput get=placeholder_input set=set_placeholder_input placeholder="This is a placeholder"/>
        <Button variant=ButtonVariant::Flat size=ButtonSize::Small on_press=move |_| set_placeholder_input.set(String::new())>"Clear input"</Button>

        <H2>"Styling"</H2>

        <P>"You may overwrite any of the following CSS variables to meet your styling needs."</P>

        <Code>
            {indoc!(r"
                --input-padding
                --input-color
                --input-background-color
                --input-border
                --input-border-bottom
                --input-border-radius
                --input-min-height
                --input-focused-border-color
            ")}
        </Code>
    }
}
