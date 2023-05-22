use indoc::indoc;
use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn PageButton(cx: Scope) -> impl IntoView {
    let (disabled, set_disabled) = create_signal(cx, false);
    view! { cx,
        <H1>"Buttons"</H1>

        <P>"Buttons are one of the most common input mechanisms with which your users can interact with your software."</P>
        <P>"Buttons only require an action handler and can therefor be created like in this minimal example."</P>

        <Code>
            {indoc!(r#"
                <Button on_click=move |_| {}>
                    "My Button"
                </Button>
            "#)}
        </Code>

        <div>
            <Button on_click=move |_| {}>"My Button"</Button>
        </div>

        <h2>"Variants"</h2>

        <P>
            "Buttons come in three different " <Code inline=true>"ButtonVariant"</Code> "s."
            <Code inline=true>"Flat"</Code> ", "
            <Code inline=true>"Outlined"</Code> " and "
            <Code inline=true>"Filled"</Code> ", with the Filled variant being the default, hence the visual of our simple button above."
        </P>

        <Code>
            {indoc!(r#"
                <Button on_click=move |_| {} variant=ButtonVariant::Flat>"Flat"</Button>
                <Button on_click=move |_| {} variant=ButtonVariant::Outlined>"Outlined"</Button>
                <Button on_click=move |_| {} variant=ButtonVariant::Filled>"Filled"</Button>
            "#)}
        </Code>

        <div>
            <Button on_click=move |_| {} variant=ButtonVariant::Flat>"Flat"</Button>
            <Button on_click=move |_| {} variant=ButtonVariant::Outlined>"Outlined"</Button>
            <Button on_click=move |_| {} variant=ButtonVariant::Filled>"Filled"</Button>
        </div>

        <h2>"Button group"</h2>

        <P>"Buttons can be displayed in a group. This lets adjacent buttons snap to each other, creating a seamless row of buttons. It is recommended to only use the Filled button variant when putting buttons inside a group."</P>

        <Code>
            {indoc!(r#"
                <ButtonGroup>
                    <Button on_click=move |_| {}>"Button 1"</Button>
                    <Button on_click=move |_| {}>"Button 2"</Button>
                    <Button on_click=move |_| {}>"Button 3"</Button>
                </ButtonGroup>
            "#)}
        </Code>

        <ButtonGroup>
            <Button on_click=move |_| {}>"Button 1"</Button>
            <Button on_click=move |_| {}>"Button 2"</Button>
            <Button on_click=move |_| {}>"Button 3"</Button>
        </ButtonGroup>

        <H2>"Disabled"</H2>

        <P>"Buttons can be set disabled using a signal."</P>

        <P>
            "Buttons can be disabled using the "
            <Code inline=true>"disabled"</Code>
            " property. You can supply anything evaluating to a boolean, including signals."
        </P>

        <Code>
            {indoc!(r#"
                <Button on_click=move |_| {} disabled=true>"Disabled"</Button>
                <Button on_click=move |_| {} disabled=disabled>"Disabled"</Button>
                <Button on_click=move |_| {} disabled=Signal::derive(cx, move || !disabled.get())>"Disabled"</Button>
            "#)}
        </Code>

        <div>
            "Disable: " <Toggle state=disabled on_toggle=set_disabled/>
        </div>

        <Button on_click=move |_| {} disabled=true>"Disabled"</Button>
        <Button on_click=move |_| {} disabled=disabled>"Disabled"</Button>
        <Button on_click=move |_| {} disabled=Signal::derive(cx, move || !disabled.get())>"Disabled"</Button>

        <H2>"Variations"</H2>

        <Button on_click=move |_| {} variations=view!{cx, <Button on_click=move |_| {}>"Secondary action"</Button>}.into_view(cx)>
            "MainAction"
        </Button>
    }
}
