use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::input_basic::InputBasicDemo;
use super::demos::input_labeled::InputLabeledDemo;
use super::demos::input_password::InputPasswordDemo;
use crate::pages::documentation::demo_shell::DemoShell;
use crate::pages::documentation::{article::Article, toc::Toc};

#[component]
pub fn PageInput() -> impl IntoView {
    let (placeholder_input, set_placeholder_input) = signal(String::new());

    view! {
        <Article>
            <h1 id="input" class="anchor">
                "Input"
                <AnchorLink href="#input" description="Direct link to article header"/>
            </h1>

            <p>"Creating an input is as simple as doing the following"</p>

            <DemoShell source=include_str!("demos/input_basic.rs")>
                <InputBasicDemo />
            </DemoShell>

            <h2 id="labeled" class="anchor">
                "Labeled"
                <AnchorLink href="#labeled" description="Direct link to section: Labeled"/>
            </h2>

            <p>"Wrap an input and a label to link them together."</p>

            <DemoShell source=include_str!("demos/input_labeled.rs")>
                <InputLabeledDemo />
            </DemoShell>

            <h2 id="types" class="anchor">
                "Types"
                <AnchorLink href="#types" description="Direct link to section: Types"/>
            </h2>

            <p>
                "You can use the "<Code inline=true>"InputType"</Code>" enum, to either create a "
                <Code inline=true>"Text"</Code>
                ", "<Code inline=true>"Password"</Code>
                " or "<Code inline=true>"Number"</Code>
                " input, "<Code inline=true>"Text"</Code>" being the default type when unspecified."
            </p>

            <DemoShell source=include_str!("demos/input_password.rs")>
                <InputPasswordDemo />
            </DemoShell>

            <h2 id="value-updates" class="anchor">
                "Value updates"
                <AnchorLink href="#value-updates" description="Direct link to section: Value updates"/>
            </h2>

            <p>
                "An input can be used without providing the "<Code inline=true>"set"</Code>" prop. "
                "You will not be notified about changes to the input value. "
                "This can be useful when you know that the input will always be "<Code inline=true>"disabled"</Code>" and you never expect changes."
            </p>

            <p>
                "The "<Code inline=true>"set"</Code>" prop, providing you with new values whenever the inputs content changes"
                ", accepts an "<Code inline=true>"Out<String>"</Code>
                ", allowing you to either provide a "<Code inline=true>"WriteSignal"</Code>" whose value is set when the input changes"
                " or a custom "<Code inline=true>"Callback"</Code>" called whenever the input changes"
                ", allowing you to handle values by yourself if required."
            </p>

            <p>
                "You can define the "<Code inline=true>"set"</Code>" prop in one of the following ways."
            </p>

            <Code language=Language::Rust>
                {indoc!(r"
                    view! {
                        <Input get=text set=set_text/>
                        <Input get=text set=create_callback(move |v: String| set_text.set(v))/>
                    }
                ")}
            </Code>

            <h2 id="placeholder" class="anchor">
                "Placeholder"
                <AnchorLink href="#placeholder" description="Direct link to section: Placeholder"/>
            </h2>

            <p>"You can supply a placeholder to the input. It is shown as when the input is empty."</p>

            <Code language=Language::Rust>
                {indoc!(r#"
                    let (text, set_text) = signal(String::new());
                    view! {
                        <TextInput get=text set=set_text placeholder=Oco::Borrowed("This is a placeholder")/>
                        <Button
                            variant=ButtonVariant::Flat
                            size=ButtonSize::Small
                            on_press=move |_| set_text.set(String::new())>
                            "Clear input"
                        </Button>
                    }
                "#)}
            </Code>

            <TextInput
                get=placeholder_input
                set=set_placeholder_input
                placeholder=Oco::Borrowed("This is a placeholder")
            />
            <Button
                variant=ButtonVariant::Flat
                size=ButtonSize::Small
                on_press=move |_| set_placeholder_input.set(String::new())
            >
                "Clear input"
            </Button>

            <h2 id="styling" class="anchor">
                "Styling"
                <AnchorLink href="#styling" description="Direct link to section: Styling"/>
            </h2>

            <p>"You may overwrite any of the following CSS variables to meet your styling needs."</p>

            <Code language=Language::Rust>
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
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "Input", link: "#input" },
                Toc::Leaf { title: "Labeled", link: "#labeled" },
                Toc::Leaf { title: "Types", link: "#types" },
                Toc::Leaf { title: "Value updates", link: "#value-updates" },
                Toc::Leaf { title: "Placeholder", link: "#placeholder" },
                Toc::Leaf { title: "Styling", link: "#styling" },
            ]
        }/>
    }
}
