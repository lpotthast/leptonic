use indoc::indoc;
use leptonic::{components::prelude::*, hooks::LinkTarget};
use leptos::prelude::*;

use crate::{kit::*, routes};

#[component]
pub fn PageInstallation() -> impl IntoView {
    view! {
        <DocPage title="Installation">
            <p>
                "The quickest start is one of the templates, "
                <LinkExt href="https://github.com/lpotthast/leptonic-template-ssr" target=LinkTarget::_Blank>"leptonic-template-ssr"</LinkExt>
                " or "
                <LinkExt href="https://github.com/lpotthast/leptonic-template-csr" target=LinkTarget::_Blank>"leptonic-template-csr"</LinkExt>
                ". To add leptonic to an existing app, follow the "<a href="#custom-setup">"custom setup"</a>"."
            </p>

            <Section title="Templates">
                <Code language=Language::Shell>
                    {indoc!(r"
                        git clone https://github.com/lpotthast/leptonic-template-ssr.git
                        git clone https://github.com/lpotthast/leptonic-template-csr.git
                    ")}
                </Code>

                <ul>
                    <li>
                        "Server-side rendering with hydration (SSR) gives you the fastest initial load and server functions."
                    </li>
                    <li>
                        "Client-side rendering (CSR) is simpler to set up and deploy, at the cost of a bigger bundle and a slower "
                        "first load."
                    </li>
                </ul>

                <p>"You use leptonic the same way in both."</p>
            </Section>

            <Section title="Custom setup">
                <Section title="Dependency and features">
                    <p>
                        "Leptonic is split into three layers, each behind a feature (see "
                        <Link href=routes::doc::Architecture.materialize()>"Hooks, Atoms & Components"</Link>
                        "). Only "<Code inline=true>"hooks"</Code>" is enabled by default. Enable "<Code inline=true>"components"</Code>
                        " (which includes the atoms) to use the pre-built components, or "<Code inline=true>"full"</Code>
                        " for everything, including the rich text editor, syntax highlighting and clipboard support."
                    </p>

                    <Code language=Language::Shell>
                        {indoc!(r"
                            cargo add leptonic --features full
                        ")}
                    </Code>

                    <p>
                        "For SSR, forward your app\u{2019}s "<Code inline=true>"ssr"</Code>" and "<Code inline=true>"hydrate"</Code>
                        " features to leptonic\u{2019}s features of the same name:"
                    </p>

                    <Code language=Language::Toml>
                        {indoc!(r#"
                            [features]
                            hydrate = ["leptos/hydrate", "leptonic/hydrate"]
                            ssr = ["leptos/ssr", "leptonic/ssr"]
                        "#)}
                    </Code>
                </Section>

                <Section title="Build configuration">
                    <p>
                        "Leptonic is written against web-sys\u{2019} unstable API signatures, so your app has to opt in. Add a "
                        <Code inline=true>".cargo/config.toml"</Code>" next to your "<Code inline=true>"Cargo.toml"</Code>":"
                    </p>

                    <Code language=Language::Toml>
                        {indoc!(r#"
                            [build]
                            rustflags = ["--cfg=web_sys_unstable_apis"]
                        "#)}
                    </Code>

                    <p>
                        "If you also set target-specific "<Code inline=true>"rustflags"</Code>" (e.g. for a custom linker), repeat the flag "
                        "there: Cargo does not merge "<Code inline=true>"[build].rustflags"</Code>" with "
                        <Code inline=true>"[target.<triple>].rustflags"</Code>", the target-specific entry replaces them."
                    </p>

                    <Code language=Language::Toml>
                        {indoc!(r#"
                            [target.aarch64-apple-darwin]
                            rustflags = ["--cfg=web_sys_unstable_apis", "-C", "link-arg=-fuse-ld=lld"]
                        "#)}
                    </Code>
                </Section>

                <Section title="Styles and static files">
                    <p>
                        "The default styling comes from the "
                        <LinkExt href="https://github.com/lpotthast/leptonic/tree/main/leptonic-theme" target=LinkTarget::_Blank>
                            "leptonic-theme"
                        </LinkExt>
                        " crate. Leptonic\u{2019}s build script copies the themes and other static files into your project, so you "
                        "have to tell it where to put them. Add this to your "<Code inline=true>"Cargo.toml"</Code>
                        " (we assume your "<Code inline=true>"main.scss"</Code>" lives in "<Code inline=true>"style/"</Code>"):"
                    </p>

                    <Code language=Language::Toml>
                        {indoc!(r#"
                            [package.metadata.leptonic]
                            # Where the build script copies the leptonic themes to.
                            style-dir = "style"

                            # Where the build script copies static JS dependencies (of the rich text editor) to.
                            js-dir = "public/js"
                        "#)}
                    </Code>

                    <p>
                        "The build script finds this "<Code inline=true>"Cargo.toml"</Code>" by searching upwards from Cargo\u{2019}s "
                        "target directory. If your target directory lies outside your project (a "
                        <Code inline=true>"CARGO_TARGET_DIR"</Code>" elsewhere), point "
                        <Code inline=true>"LEPTONIC_APP_DIR"</Code>" at your project, for example in its "
                        <Code inline=true>".cargo/config.toml"</Code>":"
                    </p>

                    <Code language=Language::Toml>
                        {indoc!(r#"
                            [env]
                            LEPTONIC_APP_DIR = { value = ".", relative = true }
                        "#)}
                    </Code>

                    <p>"We recommend committing the copied files. Then include the themes in your "<Code inline=true>"style/main.scss"</Code>":"</p>

                    <Code language=Language::Css>
                        {indoc!(r#"
                            @use "./leptonic/leptonic-themes";
                        "#)}
                    </Code>

                    <p>
                        "Override theme variables with a "<Code inline=true>"[data-theme=\"...\"]"</Code>" selector, see "
                        <Link href=routes::doc::Themes.materialize()>"Themes"</Link>":"
                    </p>

                    <Code language=Language::Css>
                        {indoc!(r#"
                            [data-theme="light"],
                            [data-theme="dark"] {
                                --brand-color: #8856e6;
                            }
                        "#)}
                    </Code>
                </Section>

                <Section title="The Root component">
                    <p>
                        "Render your app inside leptonic\u{2019}s "<Code inline=true>"<Root>"</Code>
                        " component, exactly once. It provides theming, the infrastructure of "
                        <Link href=routes::doc::Modal.materialize()>"modals"</Link>" and "
                        <Link href=routes::doc::components::Toast.materialize()>"toasts"</Link>", and global event listeners."
                    </p>

                    <Code language=Language::Rust>
                        {indoc!(r#"
                            use leptonic::components::prelude::*;
                            use leptos::prelude::*;

                            #[component]
                            pub fn App() -> impl IntoView {
                                let (count, set_count) = signal(0);

                                view! {
                                    <Root default_theme=LeptonicTheme::default()>
                                        <h2>"Welcome to Leptonic"</h2>
                                        <p>"Count: " {move || count.get()}</p>
                                        <Button on_press=move |_| set_count.update(|c| *c += 1)>
                                            "Increase"
                                        </Button>
                                    </Root>
                                }
                            }
                        "#)}
                    </Code>

                    <p>
                        "Components, atoms and hooks have their own preludes: "<Code inline=true>"leptonic::components::prelude"</Code>", "
                        <Code inline=true>"leptonic::atoms::prelude"</Code>" and "<Code inline=true>"leptonic::hooks"</Code>"."
                    </p>
                </Section>
            </Section>
        </DocPage>
    }
}
