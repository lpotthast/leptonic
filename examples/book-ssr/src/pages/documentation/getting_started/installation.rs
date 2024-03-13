use indoc::indoc;
use leptonic::{atoms::link::AnchorLink, components::prelude::*};
use leptos::*;

use crate::pages::documentation::{article::Article, doc_root::DocRoutes, toc::Toc};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageInstallation() -> impl IntoView {
    view! {
        <Article>
            <H1 id="installation" class="anchor">
                "Installation"
                <AnchorLink href="#installation" description="Direct link to an overview of installation instructions."/>
            </H1>

            <P>
                "The easiest way to get started is by cloning either "
                <LinkExt href="https://github.com/lpotthast/leptonic-template-ssr" target=LinkExtTarget::Blank>"template-ssr"</LinkExt> " or "
                <LinkExt href="https://github.com/lpotthast/leptonic-template-csr" target=LinkExtTarget::Blank>"template-csr"</LinkExt> "."
            </P>

            <Code>
                {indoc!(r"
                    git clone https://github.com/lpotthast/leptonic-template-ssr.git
                    git clone https://github.com/lpotthast/leptonic-template-csr.git
                ")}
            </Code>

            <Ul>
                <Li slot>"Prefer server-side-rendering and hydration (SSR) when you want the best performing and feature rich (server functions) solution."</Li>
                <Li slot>"Use client-side-rendering (CSR) when big bundle-sizes and slow initial load times are not an issue for you or when you want a solution with less complexity involved."</Li>
            </Ul>

            <P>"The way you use Leptonic will stay the same either way."</P>

            <H2 id="custom-setup" class="anchor">
                "Custom setup"
                <AnchorLink href="#custom-setup" description="Custom setup"/>
            </H2>

            <P>"Start by adding "<Code inline=true>"leptonic"</Code>" as a dependency of your app. "</P>

            <Code>
                {indoc!(r"
                    cargo add leptonic
                ")}
            </Code>

            <P>
                "Leptonic comes with default styling in form of the "<LinkExt href="https://github.com/lpotthast/leptonic-theme/tree/main" target=LinkExtTarget::Blank>"leptonic-theme"</LinkExt>" crate. "
                "The themes, as well as other static files, are automatically copied to your project root directory when building your application. "
                "You have to tell Leptonic where you want these files to be stored. "
                "We recommend not excluding them from your VCS."
            </P>

            <P>"Add the following to your "<Code inline=true>"Cargo.toml"</Code>". We will assume that the 'style' directory also contains your 'main.scss' file."</P>

            <Code>
                {indoc!(r#"
                    [package.metadata.leptonic]
                    # REQUIRED: Leptonic's build-script will copy the Leptonic themes to this directory.
                    style-dir = "style"

                    # REQUIRED: Leptonic's build-script will copy static JS dependencies to this directory.
                    js-dir = "public/js"
                "#)}
            </Code>

            <P>"To incorporate the Leptonic themes in your app, add the following to your "<Code inline=true>"style/main.scss"</Code>" file."</P>

            <Code>
                {indoc!(r#"
                    @import "./leptonic/leptonic-themes";
                "#)}
            </Code>

            <P>"You can overwrite or add styles for a particular theme using a "<Code inline=true>"[data-theme=\"...\"]"</Code>" selector like so:"</P>

            <Code>
                {indoc!(r#"
                    [data-theme="light"] {
                        --brand-color: #8856e6;
                    }

                    [data-theme="dark"] {
                        --brand-color: #8856e6;
                    }
                "#)}
            </Code>

            <P>
                "Leptonic depends on the "<Code inline=true>"leptos-use"</Code>" crate. Some of the features used require an opt-in."
                " In order for your app to compile properly, add a folder named "<Code inline=true>".cargo"</Code>" besides your "<Code inline=true>"Cargo.toml"</Code>" file."
                " Place a "<Code inline=true>"config.toml"</Code>" file inside it containing the following content:"
            </P>

            <Code>
                {indoc!(r#"
                    [build]
                    # `leptonic` depends on some `leptos-use` functions requiring this opt-in. This may change in the future.
                    rustflags = ["--cfg=web_sys_unstable_apis"]
                "#)}
            </Code>

            <P>"You should now be ready to use leptonic components in your leptos app. Let's set up your first component."</P>


            <P>"Similar to Leptos, this crate comes with a prelude module."</P>

            <P>"Just " <Code inline=true>"use leptonic::prelude::*;"</Code> " and you are ready to use any component mentioned in this book."</P>

            <P>
                "Leptonic provides the "<Code inline=true>"<Root>"</Code>" component. "
                "It is responsible for enabling the "<Link href=DocRoutes::Themes>"Theming"</Link>", "<Link href=DocRoutes::Modal>"Modal"</Link>" and "
                <Link href=DocRoutes::Toast>"Toast"</Link>" functionality of Leptonic as well as providing global event-listening capabilities."
                "You have to include it in your app once, and render all your content inside it."
            </P>

            <P>"Let's implement the famous counter example."</P>

            <Code>
                {indoc!(r#"
                    use leptonic::prelude::*;

                    #[component]
                    pub fn App() -> impl IntoView {
                        let (count, set_count) = create_signal(0);
                        view! {
                            <Root default_theme=LeptonicTheme::default()>
                                <Box style="display: flex; flex-direction: column; align-items: center; padding: 1em; min-height: 100%; min-width: 100%">
                                    <H2>"Welcome to Leptonic"</H2>

                                    <span style="margin-top: 3em;">"Count: " {move || count.get()}</span>
                                    <Button on_click=move|_| set_count.update(|c| *c += 1)>
                                        "Increase"
                                    </Button>
                                </Box>
                            </Root>
                        }
                    }
                "#)}
            </Code>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "Installation", link: "#installation" },
                Toc::Leaf { title: "Custom setup", link: "#custom-setup" },
            ]
        }/>
    }
}
