use leptonic::prelude::*;
use leptos::*;
use indoc::indoc;

#[component]
pub fn PageInstallation(cx: Scope) -> impl IntoView {
    view! { cx,
        <Typography variant=TypographyVariant::H2>"Installation"</Typography>

        <Typography variant=TypographyVariant::Paragraph>
            "Start by adding the leptonic library as a dependency of your app."
        </Typography>

        <Typography variant=TypographyVariant::Paragraph>
            "We will see later why the theme dependency should be optional."
        </Typography>

        <Code>
            {indoc!(r#"
                cargo add leptonic
                cargo add --optional leptonic-theme
            "#)}
        </Code>

        <Typography variant=TypographyVariant::Paragraph>
            "Leptonic comes with styling. In order to build your app with these styles, a build script is required."
        </Typography>

        <Typography variant=TypographyVariant::Paragraph>
            "But: A normal Rust build-script will not suffice. We want the theme to be build by Trunk.  "
        </Typography>

        <Code>
            {indoc!(r#"
                [[bin]]
                name = "force-build"
                path = "build.rs"
                required-features = ["build_deps"]  # only needed for build-dependencies
            "#)}
        </Code>

        <Typography variant=TypographyVariant::Paragraph>
            "With the force-build target in place, we can set up a custom Trunk.toml."
        </Typography>

        <Typography variant=TypographyVariant::Paragraph>
            "The [watch] section is used to ignore changes in the \"./generated\" directory. When omitted, Trunk would recompile our app in an endless loop."
        </Typography>

        <Typography variant=TypographyVariant::Paragraph>
            "We use the [[hooks]] section to tell Trunk that \"force-build\" must be executed BEFORE building the application."
        </Typography>

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
