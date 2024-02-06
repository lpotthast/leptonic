use indoc::indoc;
use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn PageSkeleton() -> impl IntoView {
    view! {
        <H1>"Skeletons"</H1>

        <P>
            "A skeleton is a placeholder element of a specific shape and size which can be displayed in place of some actual content, "
            "whenever, for example, this content cannot be displayed because required data is still being fetched from a network resource. "
            "This reduces layout shifts and prepares the user for where content will be visible when available."
        </P>

        <Code>
            {indoc!(r"
                <Skeleton height=Size::Em(5.0)/>
            ")}
        </Code>

        <P>
            "An explicit "<Code inline=true>"width"</Code>" property of "<Code inline=true>"Size::Percent(100.0)"</Code>" can be omitting as this is the default width of any skeleton."<br />
            "The "<Code inline=true>"height"</Code>" property defaults to "<Code inline=true>"Size::Auto"</Code>", so setting an explicit height is always advised when not embedding children in the skeleton."
        </P>

        <P>"The skeleton will render as:"</P>

        <Skeleton height=Size::Em(5.0)/>

        <H2>"Animation"</H2>

        <P>
            "By default, skeleton components contain an animation, suggesting that something is waiting to replace this component shortly. "
            "If for any reason, this animation is not desired, it can be disabled using the "<Code inline=true>"animated"</Code>" property."
        </P>

        <Code>
            {indoc!(r"
                <Skeleton animated=false height=Size::Em(5.0)/>
            ")}
        </Code>

        <P>"The skeleton will render as:"</P>

        <Skeleton animated=false height=Size::Em(5.0)/>

        <P>
            "Albeit used quite often these days, I would like to remind you that this concept is only tries to mitigate the problem of slowly loading resources. "
            "All that might just not be required, if resources are preloaded, if services providing data do that in a few milliseconds, and so on and so forth... Try avoiding overly aggressive use of the skeleton component."
            "But, even if your services respond quickly, keep in mind that the (uncontrollable) user-network-speeds may still result in slow resources."
        </P>

        <H2>"Children"</H2>

        <P>
            "As already mentioned briefly, the skeleton component optionally accepts children, so that content con be rendered if desired. "
            "In this case, a specific "<Code inline=true>"height"</Code>" property may not be specified."
        </P>

        <Code>
            {indoc!(r#"
                <Skeleton animated=false>
                    "I am a skeleton!"
                </Skeleton>
            "#)}
        </Code>

        <P>"The skeleton will render as:"</P>

        <Skeleton animated=false>
            "I am a skeleton!"
        </Skeleton>

        <P>"We will encounter these skeletons on other pages of this layout chapter."</P>

        <H2>"Styling"</H2>

        <P>"You may overwrite any of the following CSS variables to meet your styling needs."</P>

        <Code>
            {indoc!(r"
                --skeleton-background-color
                --skeleton-animation-highlight-color
                --skeleton-border-radius
                --skeleton-padding
                --skeleton-cursor
            ")}
        </Code>
    }
}
