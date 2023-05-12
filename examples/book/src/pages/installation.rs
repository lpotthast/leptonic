use indoc::indoc;
use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn PageInstallation(cx: Scope) -> impl IntoView {
    view! { cx,
        <Typography variant=TypographyVariant::H2 id="#installation">
            "Installation"
            <Anchor href="#installation" title="Direct link to an overview of installation instructions." />

            //<a href="#adding-the-menu-to-all-windows"
            //class="hash-link"
            //aria-label="Direct link to Adding the menu to all windows"
            // title="Direct link to Adding the menu to all windows">&ZeroWidthSpace;</a>
        </Typography>

        <P>
            "Start by adding the leptonic library as a dependency of your app."
        </P>

        <P>
            "We will see later why the theme dependency should be optional."
        </P>

        <Code>
            {indoc!(r#"
                cargo add leptonic
                cargo add --optional leptonic-theme
            "#)}
        </Code>

        <P>
            "Leptonic comes with styling. In order to build your app with these styles, a build script is required."
        </P>

        <P>
            "But: A normal Rust build-script will not suffice. We want the theme to be build by Trunk.  "
        </P>

        <Code>
            {indoc!(r#"
                [[bin]]
                name = "force-build"
                path = "build.rs"
                required-features = ["build_deps"]  # only needed for build-dependencies
            "#)}
        </Code>

        <P>
            "With the force-build target in place, we can set up a custom Trunk.toml."
        </P>

        <P>
            "The [watch] section is used to ignore changes in the \"./generated\" directory. When omitted, Trunk would recompile our app in an endless loop."
        </P>

        <P>
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






        <Code>
            {indoc!(r#"
                leptonic = { path = "../.." }
                leptonic-theme = { path = "../../../leptonic-theme", optional = true }
            "#)}
        </Code>
        <Code>
            {indoc!(r#"
                [features]
                build_deps = ["leptonic-theme"]
            "#)}
        </Code>
        <Code>
            {indoc!(r#"
                pub fn main() {
                    let root: std::path::PathBuf = std::env::var("CARGO_MANIFEST_DIR").unwrap().into();
                    leptonic_theme::generate(root.join("generated").join("leptonic"));
                    println!("cargo:warning=theme written")
                }
            "#)}
        </Code>
    }
}
