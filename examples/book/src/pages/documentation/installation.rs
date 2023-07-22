use indoc::indoc;
use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn PageInstallation(cx: Scope) -> impl IntoView {
    view! { cx,
        <H1 id="#installation">
            "Installation"
            <Anchor href="#installation" title="Direct link to an overview of installation instructions."/>
        </H1>

        <P>
            "We assume that you already have an app depending on "<Code inline=true>"leptos"</Code>" in version "<Code inline=true>"0.4.3"</Code>" or higher."
        </P>
        
        <P>
            "Start by adding "<Code inline=true>"leptonic"</Code>", "<Code inline=true>"leptonic-theme"</Code>" and "<Code inline=true>"leptos-tiptap-build"</Code>" as dependencies of your app. "
            "The later ones are "<Code inline=true>"[build-dependencies]"</Code>" as they will only be used in a "<Code inline=true>"build.rs"</Code>" script which we define later."
        </P>

        <Code>
            {indoc!(r#"
                cargo add leptonic
                cargo add --build leptonic-theme
                cargo add --build leptos-tiptap-build
            "#)}
        </Code>

        <P>
            "Leptonic comes with default styling in form of the "<Code inline=true>"leptonic-theme"</Code>" crate. "
            "In order to build your app with these styles, a build script is required. "
        </P>

        <P>"Let's create our "<Code inline=true>"build.rs"</Code>" file, generating our theme and copying required JS files."</P>

        <Code>
            {indoc!(r#"
                use std::io::Write;

                pub fn main() {
                    let root_dir: std::path::PathBuf = std::env::var("CARGO_MANIFEST_DIR").unwrap().into();
                    let generated_dir = root_dir.join("generated");
                    let js_dir = generated_dir.join("js");

                    leptonic_theme::generate(generated_dir.join("leptonic"));
                    println!("cargo:warning=theme written");

                    std::fs::create_dir_all(js_dir.clone()).unwrap();
                    println!("cargo:warning=js dir created");

                    std::fs::File::create(js_dir.join("tiptap-bundle.min.js"))
                        .unwrap()
                        .write(leptos_tiptap_build::TIPTAP_BUNDLE_MIN_JS.as_bytes())
                        .unwrap();
                    println!("cargo:warning=tiptap-bundle.min.js written");

                    std::fs::File::create(js_dir.join("tiptap.js"))
                        .unwrap()
                        .write(leptos_tiptap_build::TIPTAP_JS.as_bytes())
                        .unwrap();
                    println!("cargo:warning=tiptap.js written");
                }
            "#)}
        </Code>

        <P>
            "Currently, Leptonic focuses on integration with client-side-rendering and building with Trunk. "
            "When building our application with Trunk, the build.rs script should be executed before the Trunk build runs. "
            "Let's set up a custom "<Code inline=true>"Trunk.toml"</Code>" file:"
        </P>

        <P>
            "The "<Code inline=true>"[watch]"</Code>" section is used to ignore changes in the \"./generated\" directory (which our build script writes to). When omitted, Trunk would recompile our app in an endless loop."<br />
            "We use the "<Code inline=true>"[[hooks]]"</Code>" section to tell Trunk that a \"cargo check\" must be executed BEFORE building the application, as a check run will execute our build.rs file and thereby ensure that all required files for building the application are present."
        </P>

        <Code>
            {indoc!(r#"
                [watch]
                # Paths to watch. The `build.target`'s parent folder is watched by default.
                ignore = [
                    # These files are generated from our build.rs script, not excluding them would result in an endless restart-cycle!
                    # Keep this list in sync with what the build script generates.
                    "./generated",
                ]

                [serve]
                address = "127.0.0.1"
                port = 4001
                open = false
                no_autoreload = false

                [[hooks]]
                stage = "pre_build"
                command = "cargo"
                command_arguments = ["check"]
            "#)}
        </Code>

        <P>"The styling of our app must include the leptonic themes. Let's ensure that by adding the following line to our "<Code inline=true>"scss/style.scss"</Code>" file. This is the default location for a Trunk-based project. Create the file if you do not have it already."</P>

        <Code>
            {indoc!(r##"
                @import "../generated/leptonic/leptonic-themes";
            "##)}
        </Code>

        <P>"You can overwrite or add styles for a particular theme using a "<Code inline=true>"[theme=\"...\"]"</Code>" selector like so:"</P>

        <Code>
            {indoc!(r##"
                [theme="light"] {
                    --brand-color: #e66956;
                }
            "##)}
        </Code>

        <P>"Make sure that you are using a reasonable index.html file like the following. This should work out of the box when you followed the previous instructions."</P>

        <Code>
            {indoc!(r##"
                <!DOCTYPE html>
                <html lang="en">

                <head>
                    <meta charset="UTF-8" />

                    <meta name="description" content="Leptonic" />
                    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                    <meta name="theme-color" content="#e66956" />

                    <title>Leptonic</title>

                    <script type="module" src="/js/tiptap-bundle.min.js"></script>
                    <script type="module" src="/js/tiptap.js"></script>

                    <!-- <link rel="icon" href="/res/icon/leptonic_x64.png" /> -->

                    <link data-trunk rel="rust" data-wasm-opt="z" />
                    <link data-trunk rel="scss" href="scss/style.scss" />
                    <link data-trunk rel="copy-dir" href="generated/js/" />
                    <link rel="stylesheet" href="https://fonts.googleapis.com/css?family=Roboto&display=swap">
                </head>

                <body></body>

                </html>
            "##)}
        </Code>
    }
}
