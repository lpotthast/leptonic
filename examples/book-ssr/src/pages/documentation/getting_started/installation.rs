use indoc::indoc;
use leptonic::hooks::link::LinkTarget;
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
                        "Add leptonic with the features you need. Most apps want the "<Code inline=true>"atoms"</Code>
                        " feature. Hooks are always available:"
                    </p>

                    <Code language=Language::Shell>
                        {indoc!(r"
                            cargo add leptonic --features atoms
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
                            "Hooks and shared utilities are always available. Enable "<Code inline=true>"atoms"</Code>
                            " for unstyled elements built on those hooks. The other features add optional capabilities."
                        </p>
                        <DocTable headers=&["Feature", "Enables"]>
                            <TableRow>
                                <TableCell><Code inline=true>"intl-strings"</Code></TableCell>
                                <TableCell>"Localized hook messages in 34 languages. Enabled by default."</TableCell>
                            </TableRow>
                            <TableRow>
                                <TableCell><Code inline=true>"atoms"</Code></TableCell>
                                <TableCell>"The unstyled atoms."</TableCell>
                            </TableRow>
                            <TableRow>
                                <TableCell><Code inline=true>"clipboard"</Code></TableCell>
                                <TableCell>
                                    "Copying to the clipboard with "<Code inline=true>"leptonic::write_text"</Code>
                                    "."
                                </TableCell>
                            </TableRow>
                            <TableRow>
                                <TableCell><Code inline=true>"syntax-highlight"</Code></TableCell>
                                <TableCell>
                                    "Syntax highlighting of source code with syntect: "
                                    <Code inline=true>"leptonic::highlight_to_classed_html"</Code>
                                    " returns HTML with "<Code inline=true>"syn-*"</Code>" classes for your stylesheet to color. "
                                    "Works on the server too."
                                </TableCell>
                            </TableRow>
                            <TableRow>
                                <TableCell><Code inline=true>"full"</Code></TableCell>
                                <TableCell>"All of the above."</TableCell>
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

                <Section title="Styles">
                    <p>
                        "Hooks and atoms bring no styles, so there is nothing to install for them: style the atoms with your "
                        "own CSS, selecting their default classes ("<Code inline=true>"leptonic-<AtomName>"</Code>") and the "
                        "data attributes of their state (see "
                        <Link href=format!("{}#styling-atoms", routes::doc::Architecture.materialize())>"Styling Atoms"</Link>
                        "). Every atom page has a \u{201c}Styling\u{201d} section with an example."
                    </p>

                    <Section title="The Optional Atom Theme">
                        <p>
                            "If you\u{2019}d rather start from a finished look, leptonic ships an optional theme for its atoms "
                            "in the "
                            <Link href="https://github.com/lpotthast/leptonic/tree/main/leptonic-theme" target=LinkTarget::Blank>
                                "leptonic-theme"
                            </Link>
                            " crate. Leptonic\u{2019}s build script copies its SCSS sources into your project, into a "
                            <Code inline=true>"leptonic"</Code>" folder of the style directory you name in your "
                            <Code inline=true>"Cargo.toml"</Code>" (we assume your "<Code inline=true>"main.scss"</Code>
                            " lives in "<Code inline=true>"style/"</Code>"):"
                        </p>

                        <Code language=Language::Toml>
                            {indoc!(r#"
                                [package.metadata.leptonic]
                                # Where the build script copies leptonic's stylesheets to.
                                style-dir = "style"
                            "#)}
                        </Code>

                        <p>
                            "The build script finds this "<Code inline=true>"Cargo.toml"</Code>" by searching upwards from "
                            "Cargo\u{2019}s target directory. If your target directory lies outside your project (a "
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

                        <p>"We recommend committing the copied files. Then include the theme in your "<Code inline=true>"style/main.scss"</Code>":"</p>

                        <Code language=Language::Css>
                            {indoc!(r#"
                                @use "./leptonic/leptonic-atoms";
                            "#)}
                        </Code>

                        <p>
                            "Without "<Code inline=true>"style-dir"</Code>", the build script copies nothing. "
                            <Link href=format!("{}#the-atom-theme", routes::doc::Themes.materialize())>"Themes"</Link>
                            " explains how to adapt the theme."
                        </p>
                    </Section>
                </Section>

                <Section title="App Setup">
                    <p>
                        "Leptonic needs no root component. Two pieces belong at the top of most apps:"
                    </p>
                    <ul>
                        <li>
                            "A "<Code inline=true>"ThemeProvider"</Code>", which sets the "<Code inline=true>"data-theme"</Code>
                            " attribute your styles select on (on "<Code inline=true>"<html>"</Code>" too) and lets theme "
                            "controls switch it. Control it with "<Code inline=true>"signal_ls"</Code>" to remember the "
                            "user\u{2019}s choice in local storage (see "<Link href=routes::doc::Themes.materialize()>"Themes"</Link>")."
                        </li>
                        <li>
                            "A "<Code inline=true>"ToastRegion"</Code>" showing the "<Link href=routes::doc::Toast.materialize()>"toasts"</Link>
                            " of a "<Code inline=true>"ToastQueue"</Code>", if your app shows toasts. Provide the queue as a "
                            "context, so that any part of the app can add toasts."
                        </li>
                    </ul>
                    <p>
                        "Modals, popovers and tooltips need nothing at the top: they render into "<Code inline=true>"<body>"</Code>
                        " through Leptos\u{2019} "<Code inline=true>"Portal"</Code>". For layouts as tall as the window, use "
                        "the "<Code inline=true>"dvh"</Code>" unit ("<Code inline=true>"min-height: 100dvh"</Code>"), which "
                        "follows mobile browsers\u{2019} toolbars."
                    </p>

                    <Code language=Language::Rust>
                        {indoc!(r#"
                            use leptonic::{
                                atoms::{
                                    button::Button,
                                    theme::{LeptonicTheme, ThemeProvider},
                                    toast::{
                                        Toast,
                                        ToastCloseButton,
                                        ToastContent,
                                        ToastRegion,
                                        ToastTitle,
                                    },
                                },
                                hooks::toast::{ToastOptions, ToastQueue},
                                signal_ls,
                            };
                            use leptos::prelude::*;

                            #[component]
                            pub fn App() -> impl IntoView {
                                // The user's theme, remembered in local storage.
                                let (theme, set_theme) = signal_ls("theme", LeptonicTheme::default());
                                // The app's toasts, here a title each.
                                let toasts = ToastQueue::<String>::new(None);
                                provide_context(toasts);

                                view! {
                                    <ThemeProvider theme set_theme>
                                        <h1>"Welcome to Leptonic"</h1>
                                        <Button on_press=move |_| {
                                            toasts.add("Saved".to_owned(), ToastOptions::default());
                                        }>
                                            "Save"
                                        </Button>

                                        <ToastRegion queue=toasts let:toast>
                                            <Toast toast=toast.clone()>
                                                <ToastContent>
                                                    <ToastTitle>{toast.content.clone()}</ToastTitle>
                                                </ToastContent>
                                                <ToastCloseButton>"\u{00d7}"</ToastCloseButton>
                                            </Toast>
                                        </ToastRegion>
                                    </ThemeProvider>
                                }
                            }
                        "#)}
                    </Code>

                    <p>
                        "Import atoms from "<Code inline=true>"leptonic::atoms::<family>"</Code>" and hooks from "
                        <Code inline=true>"leptonic::hooks::<family>"</Code>". Shared types and utilities are exported "
                        "from the crate root, for example "<Code inline=true>"leptonic::Out"</Code>" and "
                        <Code inline=true>"leptonic::ValueBinding"</Code>"."
                    </p>
                </Section>
            </Section>
        </DocPage>
    }
}
