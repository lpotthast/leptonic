use leptonic::prelude::*;
use leptos::*;
use indoc::indoc;

#[component]
pub fn PageInstallation(cx: Scope) -> impl IntoView {
    view! { cx,
        <h2>"Installation"</h2>

        <Code>
            "cargo add leptonic"
        </Code>
        <Code>
            {indoc!(r#"
                [[bin]]
                name = "force-build"
                path = "build.rs"
                required-features = ["build_deps"]  # only needed for build-dependencies
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
