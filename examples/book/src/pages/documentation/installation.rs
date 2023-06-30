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
            "Start by adding both "<Code inline=true>"leptonic"</Code>", "<Code inline=true>"leptonic-theme"</Code>" and "<Code inline=true>"leptos-tiptap-build"</Code>" as dependencies of your app. "
            "We will see later why the theme and tiptap dependencies should be optional."
        </P>

        <Code>
            {indoc!(r#"
                cargo add leptonic
                cargo add --optional leptonic-theme
                cargo add --optional leptos-tiptap-build
            "#)}
        </Code>

        <P>
            "Leptonic comes with default styling in form of the "<Code inline=true>"leptonic-theme"</Code>" crate. "
            "In order to build your app with these styles, a build script is required. "
            "Currently, Leptonic focuses on integration with client-side-rendering and building with Trunk. "
            "When building our application with Trunk, the build.rs script should executed before the Trunk build runs. "
            "For us to be able to explicitly run the build script, we define it as a [[bin]] target."
        </P>

        <P>
            "Add this to your Cargo.toml:"
        </P>

        <Code>
            {indoc!(r#"
                [[bin]]
                name = "force-build"
                path = "build.rs"
                required-features = ["build_deps"]  # only needed for build-dependencies
            "#)}
        </Code>

        <P>"Our build.rs script needs access to our previously added, optional dependencies. Let's define that at the end of our Cargo.toml:"</P>

        <Code>
            {indoc!(r#"
                [features]
                build_deps = ["leptonic-theme", "leptos-tiptap-build"]
            "#)}
        </Code>

        <P>"Let's create the actual build.rs file"</P>

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
            "With the force-build target and build script in place, we can set up a custom Trunk.toml."<br />
            "The [watch] section is used to ignore changes in the \"./generated\" directory. When omitted, Trunk would recompile our app in an endless loop!"<br />
            "We use the [[hooks]] section to tell Trunk that \"force-build\" must be executed BEFORE building the application."
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
                command_arguments = ["run", "--bin", "force-build", "--release", "--features", "build_deps"]
            "#)}
        </Code>

        <P>"Make sure that you are using a reasonable index.html file like the following"</P>

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
