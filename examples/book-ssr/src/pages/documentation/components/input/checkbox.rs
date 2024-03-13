use indoc::indoc;
use leptonic::{atoms::link::AnchorLink, components::prelude::*};
use leptos::*;

use crate::pages::documentation::{article::Article, toc::Toc};

#[component]
pub fn PageCheckbox() -> impl IntoView {
    let (checked, set_checked) = create_signal(false);
    let (checked_disabled, set_checked_disabled) = create_signal(false);

    view! {
        <Article>
            <H1 id="checkbox" class="anchor">
                "Checkbox"
                <AnchorLink href="#checkbox" description="Direct link to article header"/>
            </H1>

            <P>"Checkbox..."</P>

            <Code>
                {indoc!(r"
                    let (checked, set_checked) = create_signal(false);
                    view! {
                        <Checkbox checked=checked set_checked=set_checked />
                    }
                ")}
            </Code>

            <Checkbox checked=checked set_checked=set_checked />

            <span>"checked: " {move || checked.get()}</span>

            <H2 id="disabled" class="anchor">
                "Disabled"
                <AnchorLink href="#disabled" description="Direct link to section: Disabled"/>
            </H2>

            <P>"Checkboxes support the " <Code inline=true>"disabled"</Code> " property, making them unmodifiable if set true."</P>

            <Code>
                {indoc!(r"
                    <Checkbox disabled=true checked=checked set_checked=set_checked />
                ")}
            </Code>

            <Checkbox disabled=true checked=checked_disabled set_checked=set_checked_disabled />
            <Button variant=ButtonVariant::Flat color=ButtonColor::Secondary size=ButtonSize::Small on_press=move |_| set_checked_disabled.set(!checked_disabled.get_untracked())>"TOGGLE"</Button>

            <H2 id="styling" class="anchor">
                "Styling"
                <AnchorLink href="#styling" description="Direct link to section: Styling"/>
            </H2>

            <P>"You may overwrite any of the following CSS variables to meet your styling needs."</P>

            <Code>
                {indoc!(r"
                    --checkbox-size
                    --checkbox-padding
                    --checkbox-border-radius
                    --checkbox-border
                    --checkbox-border-color
                    --checkbox-color
                    --checkbox-background-color
                    --checkbox-hover-border-color
                    --checkbox-hover-color
                    --checkbox-hover-background-color
                    --checkbox-checked-border-color
                    --checkbox-checked-color
                    --checkbox-checked-background-color
                    --checkbox-checked-hover-border-color
                    --checkbox-checked-hover-color
                    --checkbox-checked-hover-background-color
                ")}
            </Code>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "Checkbox", link: "#checkbox" },
                Toc::Leaf { title: "Disabled", link: "#disabled" },
                Toc::Leaf { title: "Styling", link: "#styling" },
            ]
        }/>
    }
}
