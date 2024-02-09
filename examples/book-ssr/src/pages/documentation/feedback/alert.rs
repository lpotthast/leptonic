use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::*;

#[component]
pub fn PageAlert() -> impl IntoView {
    view! {
        <H1>"Alerts"</H1>

        <Code>
            {indoc!(r#"
                <Alert variant=AlertVariant::Success>
                    <AlertTitle slot>"Success"</AlertTitle>
                    <AlertContent slot>"Action completed"</AlertContent>
                </Alert>
            "#)}
        </Code>

        <Alert variant=AlertVariant::Success>
            <AlertTitle slot>"Success"</AlertTitle>
            <AlertContent slot>"Action completed."</AlertContent>
        </Alert>

        <Alert variant=AlertVariant::Info>
            <AlertTitle slot>"Info"</AlertTitle>
            <AlertContent slot>"This concept is based on [...]"</AlertContent>
        </Alert>

        <Alert variant=AlertVariant::Warn>
            <AlertTitle slot>"Warn"</AlertTitle>
            <AlertContent slot>"This is not plausible."</AlertContent>
        </Alert>

        <Alert variant=AlertVariant::Danger>
            <AlertTitle slot>"Warn"</AlertTitle>
            <AlertContent slot>"There was an error!"</AlertContent>
        </Alert>

        <H2>"Customization"</H2>

        <Ul>
            <Li slot>"The "<Code inline=true>"default_icon_slot"</Code> " can be specified to change the default position of the icon. Slot `None` will lead to no icon being rendered."</Li>
            <Li slot>"Both " <Code inline=true>"AlertTitle"</Code> "and" <Code inline=true>"AlertContent"</Code> " can be omitted."</Li>
            <Li slot>"The " <Code inline=true>"AlertPrepend"</Code> "and" <Code inline=true>"AlertAppend"</Code> " slot can be overridden."</Li>
            <Li slot>"Custom ids, classes and styles can be applied to all slots."</Li>
        </Ul>

        <Code>
            {indoc!(r##"
                <Alert variant=AlertVariant::Success default_icon_slot=AlertIconSlot::None>
                    <AlertPrepend slot style="align-items: center; font-size: 1.8em; margin: 0;">"🎉"</AlertPrepend>
                    <AlertTitle slot style=r#"
                        align-items: center;
                        justify-content: center;
                        height: 100%;
                        text-transform: uppercase;
                        font-size: 1.3em;
                    "#>
                        "Success"
                    </AlertTitle>
                    <AlertAppend slot style="align-items: center; font-size: 1.8em; margin: 0;">"🎉"</AlertAppend>
                </Alert>
            "##)}
        </Code>

        <Alert variant=AlertVariant::Success default_icon_slot=AlertIconSlot::None>
            <AlertPrepend slot style="align-items: center; font-size: 1.8em; margin: 0;">"🎉"</AlertPrepend>
            <AlertTitle slot style=r#"
                align-items: center;
                justify-content: center;
                height: 100%;
                text-transform: uppercase;
                font-size: 1.3em;
            "#>
                "Success"
            </AlertTitle>
            <AlertAppend slot style="align-items: center; font-size: 1.8em; margin: 0;">"🎉"</AlertAppend>
        </Alert>

        <Code>
            {indoc!(r#"
                <Alert variant=AlertVariant::Warn default_icon_slot=AlertIconSlot::None>
                    <AlertTitle slot style="text-transform: uppercase; font-size: 1.3em;">
                        "Warning"
                        <AlertIcon variant=AlertVariant::Warn style="margin-left: 0.5em;" />
                    </AlertTitle>
                    <AlertContent slot>"This is dangerous!"</AlertContent>
                </Alert>
            "#)}
        </Code>

        <Alert variant=AlertVariant::Warn default_icon_slot=AlertIconSlot::None>
            <AlertTitle slot style="text-transform: uppercase; font-size: 1.3em;">
                "Warning"
                <AlertIcon variant=AlertVariant::Warn style="margin-left: 0.5em;" />
            </AlertTitle>
            <AlertContent slot>"This is dangerous!"</AlertContent>
        </Alert>

        <H2>"Styling"</H2>

        <P>"You may overwrite any of the following CSS variables to meet your styling needs."</P>

        <Code>
            {indoc!(r"
                --alert-margin
                --alert-padding
                --alert-border
                --alert-border-radius
                --alert-primary-background-color
                --alert-primary-color
                --alert-info-background-color
                --alert-info-color
                --alert-success-background-color
                --alert-success-color
                --alert-warn-background-color
                --alert-warn-color
                --alert-danger-background-color
                --alert-danger-color
            ")}
        </Code>
    }
}
