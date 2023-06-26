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

        <Stack orientation=StackOrientation::Horizontal spacing=6 style="justify-content: flex-start;">
            <Button on_click=move |_| {} variant=ButtonVariant::Flat>"Flat"</Button>
            <Button on_click=move |_| {} variant=ButtonVariant::Outlined>"Outlined"</Button>
            <Button on_click=move |_| {} variant=ButtonVariant::Filled>"Filled"</Button>
        </Stack>

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
            "Disable: " <Toggle state=disabled on_toggle=move |s| set_disabled.set(s)/>
        </div>

        <ButtonWrapper>
            <Button on_click=move |_| {} disabled=true>"Disabled"</Button>
            <Button on_click=move |_| {} disabled=disabled>"Disabled"</Button>
            <Button on_click=move |_| {} disabled=Signal::derive(cx, move || !disabled.get())>"Disabled"</Button>
        </ButtonWrapper>

        <H2>"Variations"</H2>

        <Button on_click=move |_| {} variations=view!{cx, <Button on_click=move |_| {}>"Secondary action"</Button>}.into_view(cx)>
            "MainAction"
        </Button>

        <H2>"Styling"</H2>

        <P>"You may overwrite any of the following CSS variables to meet your styling needs."</P>

        <Code>
            {indoc!(r#"
                --button-border-size
                --button-border-radius
                --button-box-shadow-opacity

                --button-primary-text-color
                --button-primary-text-color-hover
                --button-primary-background-color
                --button-primary-background-color-hover
                --button-primary-border-color
                --button-primary-border-color-hover
                --button-primary-box-shadow-color

                --button-secondary-text-color
                --button-secondary-text-color-hover
                --button-secondary-background-color
                --button-secondary-background-color-hover
                --button-secondary-border-color
                --button-secondary-border-color-hover
                --button-secondary-box-shadow-color

                --button-success-text-color
                --button-success-text-color-hover
                --button-success-background-color
                --button-success-background-color-hover
                --button-success-border-color
                --button-success-border-color-hover
                --button-success-box-shadow-color

                --button-info-text-color
                --button-info-text-color-hover
                --button-info-background-color
                --button-info-background-color-hover
                --button-info-border-color
                --button-info-border-color-hover
                --button-info-box-shadow-color

                --button-warning-text-color
                --button-warning-text-color-hover
                --button-warning-background-color
                --button-warning-background-color-hover
                --button-warning-border-color
                --button-warning-border-color-hover
                --button-warning-box-shadow-color

                --button-danger-text-color
                --button-danger-text-color-hover
                --button-danger-background-color
                --button-danger-background-color-hover
                --button-danger-border-color
                --button-danger-border-color-hover
                --button-danger-box-shadow-color
            "#)}
        </Code>
    }
}
