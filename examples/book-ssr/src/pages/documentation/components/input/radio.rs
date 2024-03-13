use indoc::indoc;
use leptonic::{atoms::link::AnchorLink, components::prelude::*};
use leptos::*;

use crate::pages::documentation::{article::Article, toc::Toc};

#[component]
pub fn PageRadio() -> impl IntoView {
    let (checked, set_checked) = create_signal(false);
    let (checked2, set_checked2) = create_signal(false);
    let (checked3, set_checked3) = create_signal(false);
    let (checked_disabled, set_checked_disabled) = create_signal(false);

    view! {
        <Article>
            <H1 id="radio" class="anchor">
                "Radio"
                <AnchorLink href="#radio" description="Direct link to article header"/>
            </H1>

            <P>"Radio..."</P>

            <Code>
                {indoc!(r"
                    let (checked, set_checked) = create_signal(false);
                    view! {
                        <Radio checked=checked set_checked=set_checked />
                    }
                ")}
            </Code>

            <Radio checked=checked set_checked=set_checked />

            <span>"checked: " {move || checked.get()}</span>

            <H2 id="radio-groups" class="anchor">
                "Radio groups"
                 <AnchorLink href="#radio-groups" description="Direct link to section: Radio groups"/>
            </H2>

            <Code>
                {indoc!(r"
                    <RadioGroup>
                        <Radio checked=checked2 set_checked=set_checked2 />
                        <Radio checked=checked3 set_checked=set_checked3 />
                    </RadioGroup>
                ")}
            </Code>

            <RadioGroup>
                <Radio checked=checked2 set_checked=set_checked2 />
                <Radio checked=checked3 set_checked=set_checked3 />
            </RadioGroup>

            <H2 id="labeled" class="anchor">
                "Labeled"
                <AnchorLink href="#labeled" description="Direct link to section: Labeled"/>
            </H2>

            <P>"Wrap an input and a label to link them together."</P>

            <Code>
                {indoc!(r#"
                    <FormControl>
                        <Radio checked=checked set_checked=set_checked />
                        <Label>"Label"</Label>
                    </FormControl>
                "#)}
            </Code>

            <FormControl>
                <Radio checked=checked set_checked=set_checked />
                <Label>
                    "Label"
                </Label>
            </FormControl>

            <H2 id="disabled" class="anchor">
                "Disabled"
                <AnchorLink href="#disabled" description="Direct link to section: Disabled"/>
            </H2>

            <P>"Radio buttons support the " <Code inline=true>"disabled"</Code> " property, making them unmodifiable if set true."</P>

            <Code>
                {indoc!(r"
                    <Radio disabled=true checked=checked set_checked=set_checked />
                ")}
            </Code>

            <Radio disabled=true checked=checked_disabled set_checked=set_checked_disabled />
            <Button variant=ButtonVariant::Flat color=ButtonColor::Secondary size=ButtonSize::Small on_press=move |_| set_checked_disabled.set(!checked_disabled.get_untracked())>"TOGGLE"</Button>

            <H2 id="styling" class="anchor">
                "Styling"
                <AnchorLink href="#styling" description="Direct link to section: Styling"/>
            </H2>

            <P>"You may overwrite any of the following CSS variables to meet your styling needs."</P>

            <Code>
                {indoc!(r"
                    --radio-size
                    --radio-fill-size
                    --radio-border
                    --radio-border-radius
                    --radio-border-color
                    --radio-hover-border-color
                    --radio-checked-border-color
                    --radio-background-color
                    --radio-checked-fill-background-color
                    --radio-disabled-filter
                ")}
            </Code>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "Radio", link: "#radio" },
                Toc::Leaf { title: "Groups", link: "#radio-groups" },
                Toc::Leaf { title: "Labeled", link: "#labeled" },
                Toc::Leaf { title: "Disabled", link: "#disabled" },
                Toc::Leaf { title: "Styling", link: "#styling" },
            ]
        }/>
    }
}
