use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::button_basic::ButtonBasicDemo;
use super::demos::button_colors::ButtonColorsDemo;
use super::demos::button_disabled::ButtonDisabledDemo;
use super::demos::button_group::ButtonGroupDemo;
use super::demos::button_variants::ButtonVariantsDemo;
use crate::pages::documentation::demo_shell::DemoShell;
use crate::pages::documentation::{article::Article, toc::Toc};

#[component]
pub fn PageButton() -> impl IntoView {
    view! {
        <Article>
            <h1 id="button" class="anchor">
                "Button component"
                <AnchorLink href="#button" description="Direct link to article header"/>
            </h1>

            <p>
                "The fully themed Button component with colors, variants, groups, and 60+ CSS variables. "
                "See the "<Link href=crate::routes::doc::Button.materialize()>"Button overview"</Link>" for concept guidance."
            </p>

            <DemoShell source=include_str!("demos/button_basic.rs")>
                <ButtonBasicDemo />
            </DemoShell>

            <h2 id="colors" class="anchor">
                "Colors"
                <AnchorLink href="#colors" description="Direct link to section: Colors"/>
            </h2>

            <p>"Buttons come in different colors. You can overwrite these using theme variables."</p>

            <DemoShell source=include_str!("demos/button_colors.rs")>
                <ButtonColorsDemo />
            </DemoShell>

            <h2 id="variants" class="anchor">
                "Variants"
                <AnchorLink href="#variants" description="Direct link to section: Variants"/>
            </h2>

            <p>
                "Buttons come in three different " <Code inline=true>"ButtonVariant"</Code> "s."
                <Code inline=true>"Flat"</Code> ", "
                <Code inline=true>"Outlined"</Code> " and "
                <Code inline=true>"Filled"</Code> ", with the Filled variant being the default, hence the visual of our simple button above."
            </p>

            <DemoShell source=include_str!("demos/button_variants.rs")>
                <ButtonVariantsDemo />
            </DemoShell>

            <h2 id="groups" class="anchor">
                "Groups"
                <AnchorLink href="#groups" description="Direct link to section: Groups"/>
            </h2>

            <p>"Buttons can be displayed in a group. This lets adjacent buttons snap to each other, creating a seamless row of buttons. It is recommended to only use the Filled button variant when putting buttons inside a group."</p>

            <DemoShell source=include_str!("demos/button_group.rs")>
                <ButtonGroupDemo />
            </DemoShell>

            <h2 id="disabled" class="anchor">
                "Disabled"
                <AnchorLink href="#disabled" description="Direct link to section: Disabled"/>
            </h2>

            <p>"Buttons can be set disabled using a signal."</p>

            <p>
                "Buttons can be disabled using the "
                <Code inline=true>"disabled"</Code>
                " property. You can supply anything evaluating to a boolean, including signals."
            </p>

            <DemoShell source=include_str!("demos/button_disabled.rs")>
                <ButtonDisabledDemo />
            </DemoShell>

            <h2 id="styling" class="anchor">
                "Styling"
                <AnchorLink href="#styling" description="Direct link to section: Styling"/>
            </h2>

            <p>"You may overwrite any of the following CSS variables to meet your styling needs."</p>

            <Code language=Language::Css>
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

            <h2 id="see-also" class="anchor">
                "See Also"
                <AnchorLink href="#see-also" description="Direct link to section: See Also"/>
            </h2>

            <ul>
                <li><Link href=crate::routes::doc::Button.materialize()>"Button overview"</Link></li>
                <li><Link href=crate::routes::doc::button::Hook.materialize()>"Hook deep-dive: use_button"</Link></li>
                <li><Link href=crate::routes::doc::button::Atom.materialize()>"Atom deep-dive: Button atom"</Link></li>
            </ul>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "Button", link: "#button" },
                Toc::Leaf { title: "Colors", link: "#colors" },
                Toc::Leaf { title: "Variants", link: "#variants" },
                Toc::Leaf { title: "Groups", link: "#groups" },
                Toc::Leaf { title: "Disabled", link: "#disabled" },
                Toc::Leaf { title: "Styling", link: "#styling" },
                Toc::Leaf { title: "See Also", link: "#see-also" },
            ]
        }/>
    }
}
