use indoc::indoc;
use leptonic::components::prelude::*;
use leptonic::prelude::*;
use leptos::*;

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageButton() -> impl IntoView {
    let (disabled, set_disabled) = create_signal(false);
    view! {
        <H1>"Buttons"</H1>

        <P>"Buttons are one of the most common input mechanisms with which your users can interact with your software."</P>
        <P>"Buttons only require an action handler and can therefor be created like in this minimal example."</P>

        <Code>
            {indoc!(r#"
                <Button on_press=move |_| {}>
                    "My Button"
                </Button>
            "#)}
        </Code>

        <div>
            <Button on_press=move |_| {}>"My Button"</Button>
        </div>

        <H2>"Colors"</H2>

        <P>"Buttons come in different colors. You can overwrite these using theme variables."</P>

        <Code>
            {indoc!(r#"
                <Button on_press=move |_| {} color=ButtonColor::Primary>"Primary"</Button>
                <Button on_press=move |_| {} color=ButtonColor::Secondary>"Secondary"</Button>
                <Button on_press=move |_| {} color=ButtonColor::Warn>"Warn"</Button>
                <Button on_press=move |_| {} color=ButtonColor::Danger>"Danger"</Button>
                <Button on_press=move |_| {} color=ButtonColor::Info>"Info"</Button>
            "#)}
        </Code>

        <Stack orientation=StackOrientation::Horizontal spacing=Size::Em(0.6) style="justify-content: flex-start;">
            <Button on_press=move |_| {} color=ButtonColor::Primary>"Primary"</Button>
            <Button on_press=move |_| {} color=ButtonColor::Secondary>"Secondary"</Button>
            <Button on_press=move |_| {} color=ButtonColor::Warn>"Warn"</Button>
            <Button on_press=move |_| {} color=ButtonColor::Danger>"Danger"</Button>
            <Button on_press=move |_| {} color=ButtonColor::Info>"Info"</Button>
        </Stack>

        <H2>"Variants"</H2>

        <P>
            "Buttons come in three different " <Code inline=true>"ButtonVariant"</Code> "s."
            <Code inline=true>"Flat"</Code> ", "
            <Code inline=true>"Outlined"</Code> " and "
            <Code inline=true>"Filled"</Code> ", with the Filled variant being the default, hence the visual of our simple button above."
        </P>

        <Code>
            {indoc!(r#"
                <Button on_press=move |_| {} variant=ButtonVariant::Flat>"Flat"</Button>
                <Button on_press=move |_| {} variant=ButtonVariant::Outlined>"Outlined"</Button>
                <Button on_press=move |_| {} variant=ButtonVariant::Filled>"Filled"</Button>
            "#)}
        </Code>

        <Stack orientation=StackOrientation::Horizontal spacing=Size::Em(0.6) style="justify-content: flex-start;">
            <Button on_press=move |_| {} variant=ButtonVariant::Flat>"Flat"</Button>
            <Button on_press=move |_| {} variant=ButtonVariant::Outlined>"Outlined"</Button>
            <Button on_press=move |_| {} variant=ButtonVariant::Filled>"Filled"</Button>
        </Stack>

        <H2>"Button group"</H2>

        <P>"Buttons can be displayed in a group. This lets adjacent buttons snap to each other, creating a seamless row of buttons. It is recommended to only use the Filled button variant when putting buttons inside a group."</P>

        <Code>
            {indoc!(r#"
                <ButtonGroup>
                    <Button on_press=move |_| {}>"Button 1"</Button>
                    <Button on_press=move |_| {}>"Button 2"</Button>
                    <Button on_press=move |_| {}>"Button 3"</Button>
                </ButtonGroup>
            "#)}
        </Code>

        <ButtonGroup>
            <Button on_press=move |_| {}>"Button 1"</Button>
            <Button on_press=move |_| {}>"Button 2"</Button>
            <Button on_press=move |_| {}>"Button 3"</Button>
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
                <Button on_press=move |_| {} disabled=true>"Always Disabled"</Button>
                <Button on_press=move |_| {} disabled=disabled>"Disabled"</Button>
                <Button on_press=move |_| {} disabled=Signal::derive(move || !disabled.get())>"!Disabled"</Button>
            "#)}
        </Code>

        <div>
            "Disable: " <Toggle state=disabled set_state=set_disabled/>
        </div>

        <ButtonWrapper>
            <Button on_press=move |_| {} disabled=true>"Always Disabled"</Button>
            <Button on_press=move |_| {} disabled=disabled>"Disabled"</Button>
            <Button on_press=move |_| {} disabled=Signal::derive(move || !disabled.get())>"!Disabled"</Button>
        </ButtonWrapper>

        <H2>"Styling"</H2>

        <P>"You may overwrite any of the following CSS variables to meet your styling needs."</P>

        <Code>
            {indoc!(r"
                --button-border-size
                --button-border-radius
                --button-box-shadow-opacity

                --button-flat-primary-text-color
                --button-flat-primary-text-color-hover
                --button-flat-primary-background-color-hover

                --button-outlined-primary-text-color
                --button-outlined-primary-text-color-hover
                --button-outlined-primary-border-color
                --button-outlined-primary-border-color-hover
                --button-outlined-primary-box-shadow-color

                --button-filled-primary-text-color
                --button-filled-primary-text-color-hover
                --button-filled-primary-background-color
                --button-filled-primary-background-color-hover
                --button-filled-primary-border-color
                --button-filled-primary-border-color-hover
                --button-filled-primary-box-shadow-color

                --button-flat-secondary-text-color
                --button-flat-secondary-text-color-hover
                --button-flat-secondary-background-color-hover

                --button-outlined-secondary-text-color
                --button-outlined-secondary-text-color-hover
                --button-outlined-secondary-border-color
                --button-outlined-secondary-border-color-hover
                --button-outlined-secondary-box-shadow-color

                --button-filled-secondary-text-color
                --button-filled-secondary-text-color-hover
                --button-filled-secondary-background-color
                --button-filled-secondary-background-color-hover
                --button-filled-secondary-border-color
                --button-filled-secondary-border-color-hover
                --button-filled-secondary-box-shadow-color

                --button-flat-success-text-color
                --button-flat-success-text-color-hover
                --button-flat-success-background-color-hover

                --button-outlined-success-text-color
                --button-outlined-success-text-color-hover
                --button-outlined-success-border-color
                --button-outlined-success-border-color-hover
                --button-outlined-success-box-shadow-color

                --button-filled-success-text-color
                --button-filled-success-text-color-hover
                --button-filled-success-background-color
                --button-filled-success-background-color-hover
                --button-filled-success-border-color
                --button-filled-success-border-color-hover
                --button-filled-success-box-shadow-color

                --button-flat-info-text-color
                --button-flat-info-text-color-hover
                --button-flat-info-background-color-hover

                --button-outlined-info-text-color
                --button-outlined-info-text-color-hover
                --button-outlined-info-border-color
                --button-outlined-info-border-color-hover
                --button-outlined-info-box-shadow-color

                --button-filled-info-text-color
                --button-filled-info-text-color-hover
                --button-filled-info-background-color
                --button-filled-info-background-color-hover
                --button-filled-info-border-color
                --button-filled-info-border-color-hover
                --button-filled-info-box-shadow-color

                --button-flat-warning-text-color
                --button-flat-warning-text-color-hover
                --button-flat-warning-background-color-hover

                --button-outlined-warning-text-color
                --button-outlined-warning-text-color-hover
                --button-outlined-warning-border-color
                --button-outlined-warning-border-color-hover
                --button-outlined-warning-box-shadow-color

                --button-filled-warning-text-color
                --button-filled-warning-text-color-hover
                --button-filled-warning-background-color
                --button-filled-warning-background-color-hover
                --button-filled-warning-border-color
                --button-filled-warning-border-color-hover
                --button-filled-warning-box-shadow-color

                --button-flat-danger-text-color
                --button-flat-danger-text-color-hover
                --button-flat-danger-background-color-hover

                --button-outlined-danger-text-color
                --button-outlined-danger-text-color-hover
                --button-outlined-danger-border-color
                --button-outlined-danger-border-color-hover
                --button-outlined-danger-box-shadow-color

                --button-filled-danger-text-color
                --button-filled-danger-text-color-hover
                --button-filled-danger-background-color
                --button-filled-danger-background-color-hover
                --button-filled-danger-border-color
                --button-filled-danger-border-color-hover
                --button-filled-danger-box-shadow-color
            ")}
        </Code>
    }
}
