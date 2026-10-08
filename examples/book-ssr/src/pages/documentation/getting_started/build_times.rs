use indoc::indoc;
use leptos::prelude::*;

use crate::{kit::*, routes};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageBuildTimes() -> impl IntoView {
    view! {
        <DocPage title="Build Times & Bundle Size">
            <p>
                "A Leptos app compiles twice, for the server and for the browser, and ships its views, their types and its "
                "data as WebAssembly. This guide collects the settings that keep your edit-and-reload loop short and your "
                "bundle small. The numbers come from this book: about 200 pages, built with leptonic\u{2019}s "
                <Code inline=true>"full"</Code>" feature."
            </p>

            <Section title="Development Builds">
                <p>
                    "Keep the dev profile unoptimized, also for leptonic: optimizing a crate costs time on every build that "
                    "touches it, and leptonic is generic code your app instantiates. If runtime speed in development matters, "
                    "optimize only dependencies that rarely change. A "<Code inline=true>"\"*\""</Code>" pattern also matches "
                    "path dependencies outside your workspace, so exempt leptonic explicitly:"
                </p>
                <Code language=Language::Toml>
                    {indoc!(r#"
                        [profile.dev.package."*"]
                        opt-level = 1
                        # No debug info for dependencies: smaller target directories, faster linking.
                        debug = false

                        [profile.dev.package.leptonic]
                        opt-level = 0

                        # Your app: file and line in backtraces, without the full (and huge) debug info.
                        [profile.dev.package.my-app]
                        debug = "line-tables-only"
                    "#)}
                </Code>
                <p>
                    "Use Leptos\u{2019} type-erased views ("<Code inline=true>"erase_components"</Code>") for every build, not "
                    "only for cargo-leptos\u{2019} dev builds: set the flag in your "<Code inline=true>".cargo/config.toml"</Code>
                    " and tell cargo-leptos not to add it itself, so that the server, the browser bundle, "
                    <Code inline=true>"cargo check"</Code>" and clippy share one set of flags and reuse each other\u{2019}s "
                    "artifacts."
                </p>
                <Code language=Language::Toml>
                    {indoc!(r#"
                        # .cargo/config.toml
                        [build]
                        rustflags = ["--cfg=web_sys_unstable_apis", "--cfg=erase_components"]

                        # Cargo.toml
                        [package.metadata.leptos]
                        disable-erase-components = true
                        lib-profile-dev = "wasm-dev"

                        [profile.wasm-dev]
                        inherits = "dev"
                        debug = false
                        strip = "symbols"
                    "#)}
                </Code>
                <p>
                    "The "<Code inline=true>"wasm-dev"</Code>" profile drops the function names of the development bundle: in "
                    "this book they were 117 of its 167 MB, so the browser downloads and compiles a fraction as much on every "
                    "reload."
                </p>
            </Section>

            <Section title="Release Bundle">
                <p>
                    "Optimize the release bundle for size: one codegen unit with link-time optimization lets the compiler "
                    "drop unused code across crates. cargo-leptos runs wasm-opt on release builds."
                </p>
                <Code language=Language::Toml>
                    {indoc!(r#"
                        [package.metadata.leptos]
                        lib-profile-release = "wasm-release"

                        [profile.wasm-release]
                        inherits = "release"
                        opt-level = "z"
                        lto = true
                        codegen-units = 1
                        panic = "abort"
                    "#)}
                </Code>
                <p>
                    "Serve the bundle compressed. This book\u{2019}s release bundle is 18.2 MB, 6.2 MB with gzip and 4.3 MB with "
                    "brotli. With axum, tower-http\u{2019}s "<Code inline=true>"CompressionLayer"</Code>" (features "
                    <Code inline=true>"compression-br"</Code>" and "<Code inline=true>"compression-gzip"</Code>") compresses "
                    "responses for the browsers that accept it."
                </p>
            </Section>

            <Section title="Locale Data">
                <p>
                    "Leptonic formats numbers and dates and compares text with ICU4X (see "
                    <Link href=format!("{}#internationalization", routes::doc::Ssr.materialize())>"Server-Side Rendering"</Link>
                    "), which compiles its locale data into your app: by default the data of every locale. Bake only the "
                    "locales your app supports. In this book, that made the bundle 20% smaller. Generate the data with "
                    "leptonic\u{2019}s "<Code inline=true>"scripts/icu-datagen.sh"</Code>" (it needs "
                    <Code inline=true>"icu4x-datagen"</Code>" of your ICU4X version, and network access), and build with it:"
                </p>
                <Code language=Language::Shell>
                    {indoc!(r"
                        scripts/icu-datagen.sh path/to/my-app path/to/my-app/icu4x-data ^en ^de ^fr
                    ")}
                </Code>
                <Code language=Language::Toml>
                    {indoc!(r#"
                        # .cargo/config.toml of your app
                        [build]
                        rustflags = ["--cfg=icu4x_custom_data"]

                        [env]
                        # Server and browser both use it: hydration needs both to format alike.
                        ICU4X_DATA_DIR = { value = "icu4x-data", relative = true }
                    "#)}
                </Code>
                <p>
                    "Regenerate the data after every ICU4X update: data of another version doesn\u{2019}t compile. "
                    <Code inline=true>"^de"</Code>" stands for German without its regional variants ("<Code inline=true>"de-AT"</Code>
                    ", "<Code inline=true>"de-CH"</Code>", \u{2026})."
                </p>
            </Section>

            <Section title="Features">
                <p>
                    "Enable only the leptonic features you use (see "
                    <Link href=format!("{}#feature-flags", routes::doc::Installation.materialize())>"Feature Flags"</Link>
                    "). "<Code inline=true>"syntax-highlight"</Code>" is the heaviest: syntect and its regex engine cost this "
                    "book 68 s of CPU time per fresh build and 1.35 MB of bundle. Features can differ per side: enable it in "
                    "your app\u{2019}s "<Code inline=true>"ssr"</Code>" feature only and highlight on the server, so the browser "
                    "bundle doesn\u{2019}t carry it."
                </p>
                <Code language=Language::Toml>
                    {indoc!(r#"
                        [features]
                        ssr = ["leptos/ssr", "leptonic/ssr", "leptonic/syntax-highlight"]
                        hydrate = ["leptos/hydrate", "leptonic/hydrate"]
                    "#)}
                </Code>
                <p>
                    "The same holds for your other dependencies: declare them with "<Code inline=true>"default-features = false"</Code>
                    " and only the features your code uses. Two server examples from this book: "
                    <Code inline=true>"axum-server"</Code>"\u{2019}s "<Code inline=true>"tls-rustls"</Code>" feature builds the "
                    <Code inline=true>"aws-lc-rs"</Code>" crypto provider (27 s of CPU time; the book uses "
                    <Code inline=true>"tls-rustls-no-provider"</Code>" and installs rustls\u{2019} "<Code inline=true>"ring"</Code>
                    " provider instead), and tower-http\u{2019}s "<Code inline=true>"full"</Code>" feature builds zstd (11 s)."
                </p>
            </Section>

            <Section title="Large Apps">
                <p>
                    "Every edit of a crate expands all of its "<Code inline=true>"view!"</Code>" macros again (3.4 s for this "
                    "book\u{2019}s 62,000 lines) and type-checks its view types, which dominates a rebuild. Split a large app "
                    "into several crates, for example one per area of pages: an edit then recompiles only its own crate."
                </p>
            </Section>
        </DocPage>
    }
}
