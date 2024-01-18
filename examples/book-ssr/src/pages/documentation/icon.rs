use indoc::indoc;
use leptonic::prelude::*;
use leptos::*;
use leptos_icons::BsIcon;

#[component]
pub fn PageIcon() -> impl IntoView {
    view! {
        <H1>"Icons"</H1>

        <P>"Icons are supported through the "<LinkExt target=LinkExtTarget::Blank href="https://github.com/Carlosted/leptos-icons">"https://github.com/Carlosted/leptos-icons"</LinkExt> " crate."</P>
        <P>"Feel free to check out the associated icon index site at "<LinkExt target=LinkExtTarget::Blank href="https://carlosted.github.io/icondata/">"https://carlosted.github.io/icondata/"</LinkExt> "."</P>

        <P>"With the dependency set up, adding our required icons as features."</P>

        <Code>
            {indoc!(r#"
                leptos_icons = { version = "0.0.15", features = [
                    "BsFolderFill",
                    "BsFolder",
                ] }
            "#)}
        </Code>

        <P>"We can simply include these icons like this, using the "<Code inline=true>"<Icon>"</Code> " component provided by this library."</P>

        <Code>
            {indoc!(r"
                <Icon icon=BsIcon::BsFolderFill/>
                <Icon icon=BsIcon::BsFolder/>
            ")}
        </Code>

        <Icon icon=BsIcon::BsFolderFill style="font-size: 6em;"/>
        <Icon icon=BsIcon::BsFolder style="font-size: 6em;"/>

        <H2>"Styling"</H2>

        <P>"There are currently no CSS variables exposed, targeting the "<Code inline=true>"<Icon>"</Code> " component."</P>

        <P>"Notes:"</P>
        <ul>
            <li>"Scale icons by setting the css "<Code inline=true>"font-size"</Code> " attribute."</li>
            <li>"Color icons by setting the css "<Code inline=true>"color"</Code> " attribute."</li>
            <li>"Some icons may also render different when altering the css "<Code inline=true>"background-color"</Code> " attribute."</li>
        </ul>
    }
}
