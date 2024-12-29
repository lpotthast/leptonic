use indoc::indoc;
use leptonic::{atoms::link::AnchorLink, components::prelude::*, prelude::*};
use leptos::prelude::*;

use crate::pages::documentation::{article::Article, toc::Toc};

#[component]
pub fn PageSkeleton() -> impl IntoView {
    view! {
        <Article>
            <h1 id="skeleton" class="anchor">
                "Skeleton"
                <AnchorLink href="#skeleton" description="Direct link to article header"/>
            </h1>

            <p>
                "A skeleton is a placeholder element of a specific shape and size which can be displayed in place of some actual content, "
                "whenever, for example, this content cannot be displayed because required data is still being fetched from a network resource. "
                "This reduces layout shifts and prepares the user for where content will be visible when available."
            </p>

            <Code>
                {indoc!(r"
                    <Skeleton height=Size::Em(5.0)/>
                ")}
            </Code>

            <p>
                "An explicit "<Code inline=true>"width"</Code>" property of "<Code inline=true>"Size::Percent(100.0)"</Code>" can be omitting as this is the default width of any skeleton."<br />
                "The "<Code inline=true>"height"</Code>" property defaults to "<Code inline=true>"Size::Auto"</Code>", so setting an explicit height is always advised when not embedding children in the skeleton."
            </p>

            <p>"The skeleton will render as:"</p>

            <Skeleton height=Size::Em(5.0)/>

            <h2 id="animation">
                "Animation"
                <AnchorLink href="#animation" description="Direct link to section: Animation"/>
            </h2>

            <p>
                "By default, skeleton components contain an animation, suggesting that something is waiting to replace this component shortly. "
                "If for any reason, this animation is not desired, it can be disabled using the "<Code inline=true>"animated"</Code>" property."
            </p>

            <Code>
                {indoc!(r"
                    <Skeleton animated=false height=Size::Em(5.0)/>
                ")}
            </Code>

            <p>"The skeleton will render as:"</p>

            <Skeleton animated=false height=Size::Em(5.0)/>

            <p>
                "Albeit used quite often these days, I would like to remind you that this concept is only tries to mitigate the problem of slowly loading resources. "
                "All that might just not be required, if resources are preloaded, if services providing data do that in a few milliseconds, and so on and so forth... Try avoiding overly aggressive use of the skeleton component."
                "But, even if your services respond quickly, keep in mind that the (uncontrollable) user-network-speeds may still result in slow resources."
            </p>

            <h2 id="children">
                "Children"
                <AnchorLink href="#children" description="Direct link to section: Children"/>
            </h2>

            <p>
                "As already mentioned briefly, the skeleton component optionally accepts children, so that content con be rendered if desired. "
                "In this case, a specific "<Code inline=true>"height"</Code>" property may not be specified."
            </p>

            <Code>
                {indoc!(r#"
                    <Skeleton animated=false>
                        "I am a skeleton!"
                    </Skeleton>
                "#)}
            </Code>

            <p>"The skeleton will render as:"</p>

            <Skeleton animated=false>
                "I am a skeleton!"
            </Skeleton>

            <p>"We will encounter these skeletons on other pages of this layout chapter."</p>

            <h2 id="styling">
                "Styling"
                <AnchorLink href="#styling" description="Direct link to section: Styling"/>
            </h2>

            <p>"You may overwrite any of the following CSS variables to meet your styling needs."</p>

            <Code>
                {indoc!(r"
                    --skeleton-background-color
                    --skeleton-animation-highlight-color
                    --skeleton-border-radius
                    --skeleton-padding
                    --skeleton-cursor
                ")}
            </Code>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "Skeleton", link: "#skeleton" },
                Toc::Leaf { title: "Animation", link: "#animation" },
                Toc::Leaf { title: "Children", link: "#children" },
                Toc::Leaf { title: "Styling", link: "#styling" },
            ]
        }/>
    }
}
