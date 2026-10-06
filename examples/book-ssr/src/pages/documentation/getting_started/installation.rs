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
                <Link href="https://github.com/lpotthast/leptonic-template-ssr" target=LinkTarget::Blank>"leptonic-template-ssr"</Link>
                " or "
                <Link href="https://github.com/lpotthast/leptonic-template-csr" target=LinkTarget::Blank>"leptonic-template-csr"</Link>
                ". To add leptonic to an existing app, follow the "<AnchorLink href="#custom-setup">"custom setup"</AnchorLink>"."
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

            <Section title="Custom Setup">
                <Section title="Dependency and Features">
                    <p>
                        "Add leptonic with the features you need. "<Code inline=true>"full"</Code>" enables every layer and "
                        "every extra:"
                    </p>

                    <Code language=Language::Shell>
                        {indoc!(r"
                            cargo add leptonic --features full
                        ")}
                    </Code>

                    <p>
                        "For SSR, forward your app\u{2019}s "<Code inline=true>"ssr"</Code>" and "<Code inline=true>"hydrate"</Code>
                        " features to leptonic\u{2019}s features of the same name (see "
                        <Link href=routes::doc::Ssr.materialize()>"Server-Side Rendering"</Link>"):"
                    </p>

                    <Code language=Language::Toml>
                        {indoc!(r#"
                            [features]
                            hydrate = ["leptos/hydrate", "leptonic/hydrate"]
                            ssr = ["leptos/ssr", "leptonic/ssr"]
                        "#)}
                    </Code>

                    <Section title="Feature Flags">
                        <p>
                            "Each layer (see "<Link href=routes::doc::Architecture.materialize()>"Hooks, Atoms & Components"</Link>
                            ") is a feature that includes the layers below it; the extras include the components."
                        </p>
                        <DocTable headers=&["Feature", "Enables"]>
                            <TableRow>
                                <TableCell><Code inline=true>"hooks"</Code></TableCell>
                                <TableCell>"The hooks and utilities. The only default feature."</TableCell>
                            </TableRow>
                            <TableRow>
                                <TableCell><Code inline=true>"atoms"</Code></TableCell>
                                <TableCell>"The atoms, and "<Code inline=true>"hooks"</Code>"."</TableCell>
                            </TableRow>
                            <TableRow>
                                <TableCell><Code inline=true>"components"</Code></TableCell>
                                <TableCell>"The themed components, and "<Code inline=true>"atoms"</Code>"."</TableCell>
                            </TableRow>
                            <TableRow>
                                <TableCell><Code inline=true>"clipboard"</Code></TableCell>
                                <TableCell>
                                    "Copying to the clipboard: the copy button of code blocks ("
                                    <Link href=routes::doc::Typography.materialize()>"Code"</Link>") and "
                                    <Code inline=true>"leptonic::utils::clipboard::write_text"</Code>"."
                                </TableCell>
                            </TableRow>
                            <TableRow>
                                <TableCell><Code inline=true>"syntax-highlight"</Code></TableCell>
                                <TableCell>"Syntax highlighting of code blocks, with syntect."</TableCell>
                            </TableRow>
                            <TableRow>
                                <TableCell><Code inline=true>"sanitize"</Code></TableCell>
                                <TableCell>
                                    <Link href=routes::doc::SanitizedHtml.materialize()>"Sanitized HTML"</Link>
                                    ", with ammonia."
                                </TableCell>
                            </TableRow>
                            <TableRow>
                                <TableCell><Code inline=true>"tiptap"</Code></TableCell>
                                <TableCell>
                                    "The "<Link href=routes::doc::RichTextEditor.materialize()>"Rich Text Editor"</Link>
                                    ". The build script copies its JavaScript into your app."
                                </TableCell>
                            </TableRow>
                            <TableRow>
                                <TableCell><Code inline=true>"full"</Code></TableCell>
                                <TableCell>
                                    "All of the above: "<Code inline=true>"hooks"</Code>", "<Code inline=true>"atoms"</Code>", "
                                    <Code inline=true>"components"</Code>", "<Code inline=true>"clipboard"</Code>", "
                                    <Code inline=true>"syntax-highlight"</Code>", "<Code inline=true>"sanitize"</Code>" and "
                                    <Code inline=true>"tiptap"</Code>"."
                                </TableCell>
                            </TableRow>
                            <TableRow>
                                <TableCell><Code inline=true>"ssr"</Code>", "<Code inline=true>"hydrate"</Code></TableCell>
                                <TableCell>"Server-side rendering and hydration. Not part of "<Code inline=true>"full"</Code>"."</TableCell>
                            </TableRow>
                            <TableRow>
                                <TableCell><Code inline=true>"nightly"</Code></TableCell>
                                <TableCell>"Leptos\u{2019} "<Code inline=true>"nightly"</Code>" feature. Not part of "<Code inline=true>"full"</Code>"."</TableCell>
                            </TableRow>
                        </DocTable>
                    </Section>
                </Section>

                <Section title="Build Configuration">
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

                <Section title="Styles and Static Files">
                    <p>
                        "The default styling comes from the "
                        <Link href="https://github.com/lpotthast/leptonic/tree/main/leptonic-theme" target=LinkTarget::Blank>
                            "leptonic-theme"
                        </Link>
                        " crate. Leptonic\u{2019}s build script copies the themes into your project, so you "
                        "have to tell it where to put them. Add this to your "<Code inline=true>"Cargo.toml"</Code>
                        " (we assume your "<Code inline=true>"main.scss"</Code>" lives in "<Code inline=true>"style/"</Code>"):"
                    </p>

                    <Code language=Language::Toml>
                        {indoc!(r#"
                            [package.metadata.leptonic]
                            # Where the build script copies the leptonic themes to.
                            style-dir = "style"
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

                <Section title="The Root Component">
                    <p>
                        "Render your app inside leptonic\u{2019}s "<Code inline=true>"<Root>"</Code>
                        " component, exactly once (a second one logs a warning). It provides:"
                    </p>
                    <ul>
                        <li>
                            "The theme: a "<Code inline=true>"ThemeProvider"</Code>" starting with "
                            <Code inline=true>"default_theme"</Code>", which remembers the user\u{2019}s choice in local "
                            "storage and sets "<Code inline=true>"data-theme"</Code>" on "<Code inline=true>"<html>"</Code>
                            " (see "<Link href=routes::doc::Themes.materialize()>"Themes"</Link>")."
                        </li>
                        <li>"The "<Link href=routes::doc::Toast.materialize()>"toasts"</Link>" area ("<Code inline=true>"ToastRoot"</Code>")."</li>
                        <li>
                            "The "<Code inline=true>"Leptonic"</Code>" context, with "<Code inline=true>"is_mobile_device"</Code>
                            " and "<Code inline=true>"is_desktop_device"</Code>" signals read from the user agent."
                        </li>
                        <li>
                            "The "<Code inline=true>"--leptonic-vh"</Code>" CSS variable on "<Code inline=true>"<html>"</Code>
                            ": the window\u{2019}s inner height, updated when it is resized."
                        </li>
                    </ul>
                    <p>
                        "Modals, popovers and tooltips need nothing from "<Code inline=true>"<Root>"</Code>": they render "
                        "into "<Code inline=true>"<body>"</Code>" through Leptos\u{2019} "<Code inline=true>"Portal"</Code>"."
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
                                        <h1>"Welcome to Leptonic"</h1>
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
                        "Import leptonic through its preludes: "<Code inline=true>"leptonic::components::prelude"</Code>" for "
                        "the components, "<Code inline=true>"leptonic::atoms::prelude"</Code>" for the atoms (usually as "
                        <Code inline=true>"use leptonic::atoms::prelude as atoms;"</Code>", as their names overlap with the "
                        "components\u{2019}), "<Code inline=true>"leptonic::hooks"</Code>" for the hooks, and "
                        <Code inline=true>"leptonic::prelude"</Code>" for the shared types ("<Code inline=true>"Out"</Code>", "
                        <Code inline=true>"ValueBinding"</Code>", "<Code inline=true>"icondata"</Code>", \u{2026})."
                    </p>
                </Section>
            </Section>
        </DocPage>
    }
}
