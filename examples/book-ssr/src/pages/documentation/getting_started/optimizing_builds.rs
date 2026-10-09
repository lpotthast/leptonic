use indoc::indoc;
use leptos::prelude::*;

use crate::{kit::*, routes};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageOptimizingBuilds() -> impl IntoView {
    view! {
        <DocPage title="Optimizing Compile Times & Binary Sizes">
            <p>
                "A Leptos app compiles twice, for the server and for the browser, and ships its views, their types and its "
                "data as WebAssembly. This guide collects configuration you can copy into your app to keep rebuilds short, "
                "pages fast and the browser bundle small, each setting with what it buys and what it costs."
            </p>
            <p>
                "The numbers come from two apps: this book (about 200 pages with 62,000 lines of views, built on an Apple "
                "M4 Max) and leptonic\u{2019}s test app (a Leptos SSR app with a 13 MB release bundle). Your app\u{2019}s "
                "numbers will differ: measure before and after each change."
            </p>

            <Section title="At a Glance">
                <DocTable headers=&["Setting", "Buys", "Costs"]>
                    <TableRow>
                        <TableCell><AnchorLink href="#features">"Only the features you use"</AnchorLink></TableCell>
                        <TableCell>"Less to compile, a smaller bundle"</TableCell>
                        <TableCell>"Nothing"</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><AnchorLink href="#debug-info">"No debug info for dependencies"</AnchorLink></TableCell>
                        <TableCell>"Smaller target directories, faster linking"</TableCell>
                        <TableCell>"A debugger shows no variables of dependencies"</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><AnchorLink href="#type-erased-views">"Type-erased views for every build"</AnchorLink></TableCell>
                        <TableCell>"cargo-leptos, check, clippy and tests share compiled dependencies"</TableCell>
                        <TableCell>"Release builds use type-erased views too"</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><AnchorLink href="#development-bundle">"A small development bundle"</AnchorLink></TableCell>
                        <TableCell>"19.7 MB instead of 43.8 MB (book), faster reloads"</TableCell>
                        <TableCell>"No function names in browser stack traces"</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><AnchorLink href="#server-rendering-speed">"An optimized development server"</AnchorLink></TableCell>
                        <TableCell>"Pages render about 6\u{d7} faster (test app)"</TableCell>
                        <TableCell>"No slower rebuilds measured"</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><AnchorLink href="#release-profile">"A release bundle without LTO"</AnchorLink></TableCell>
                        <TableCell>"Rebuilds as fast as dev, pages load in 180 ms instead of 450 ms (test app)"</TableCell>
                        <TableCell>"About 1 MB more than with LTO"</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><AnchorLink href="#production-builds">"LTO for production"</AnchorLink></TableCell>
                        <TableCell>"The last MB"</TableCell>
                        <TableCell>"84 s instead of 20 s to rebuild after a library change (test app)"</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><AnchorLink href="#compression">"A precompressed bundle"</AnchorLink></TableCell>
                        <TableCell>"2.94 MB to download instead of 13 MB (book, brotli)"</TableCell>
                        <TableCell>"About 20 s per release build"</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><AnchorLink href="#caching">"Hashed file names, cached for good"</AnchorLink></TableCell>
                        <TableCell>"Browsers keep the bundle and its compiled code between visits"</TableCell>
                        <TableCell>"A middleware for the header"</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><AnchorLink href="#locale-data">"Only your locales\u{2019} data"</AnchorLink></TableCell>
                        <TableCell>"A 20% smaller bundle (book)"</TableCell>
                        <TableCell>"Regenerating the data after every ICU4X update"</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><AnchorLink href="#linking">"A faster linker"</AnchorLink></TableCell>
                        <TableCell>"0.75 s instead of 1.2 s per server link (test app)"</TableCell>
                        <TableCell>"An extra tool to install"</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><AnchorLink href="#large-apps">"Several crates"</AnchorLink></TableCell>
                        <TableCell>"An edit recompiles only its own crate"</TableCell>
                        <TableCell>"Restructuring your app"</TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Features">
                <p>
                    "Leptonic enables "<Code inline=true>"intl-strings"</Code>" by default; hooks are always available. "
                    "Most apps also want the "<Code inline=true>"atoms"</Code>" feature (see "
                    <Link href=format!("{}#feature-flags", routes::doc::Installation.materialize())>"Feature Flags"</Link>
                    "). Declare leptonic without its default features and name the ones you use:"
                </p>
                <Code language=Language::Shell>
                    {indoc!(r"
                        cargo add leptonic --no-default-features --features atoms,intl-strings
                    ")}
                </Code>
                <p>
                    <Code inline=true>"intl-strings"</Code>" carries the hooks\u{2019} messages (labels, descriptions, "
                    "announcements) in 34 languages. An app in English only can leave it out: the messages are then English "
                    "in every locale, and the bundle is smaller."
                </p>
                <p>
                    <Code inline=true>"syntax-highlight"</Code>" is the heaviest feature: syntect and its regex engine cost "
                    "this book 68 s of CPU time per fresh build and 1.35 MB of bundle. Features can differ per side: enable it "
                    "in your app\u{2019}s "<Code inline=true>"ssr"</Code>" feature only and highlight on the server, so the "
                    "browser bundle doesn\u{2019}t carry it."
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

            <Section title="Development Builds">
                <p>
                    "cargo-leptos builds with the dev profile unless you pass "<Code inline=true>"--release"</Code>". The "
                    "settings below go into your app\u{2019}s "<Code inline=true>"Cargo.toml"</Code>"; "
                    <Code inline=true>"my-app"</Code>" stands for your package, and the profile names are yours to choose."
                </p>

                <Section title="Debug Info">
                    <Code language=Language::Toml>
                        {indoc!(r#"
                            [profile.dev.package."*"]
                            # No debug info for dependencies: smaller target directories, faster linking.
                            debug = false

                            # Your app: file and line in backtraces, without the full (and huge) debug info.
                            [profile.dev.package.my-app]
                            debug = "line-tables-only"
                        "#)}
                    </Code>
                    <p>
                        "Debug info fills target directories and the linker has to process it: the full debug info of this "
                        "book outgrew the 4 GiB that 32-bit DWARF can address. Costs: a debugger shows no "
                        "variables of your dependencies, nor of your app; backtraces keep file and line."
                    </p>
                    <p>
                        "Leave "<Code inline=true>"opt-level"</Code>" out of "<Code inline=true>"[profile.dev.package.\"*\"]"</Code>
                        ": the profiles below inherit dev\u{2019}s package overrides, and an "<Code inline=true>"opt-level"</Code>
                        " there would replace theirs for every dependency. If you use leptonic as a path dependency and edit "
                        "it, keep it unoptimized: in this book, leptonic at "<Code inline=true>"opt-level = 3"</Code>" made a "
                        "rebuild after a one-line change in leptonic take 36\u{2013}46 s instead of 18\u{2013}20 s."
                    </p>
                </Section>

                <Section title="Type-Erased Views">
                    <p>
                        "cargo-leptos adds Leptos\u{2019} "<Code inline=true>"erase_components"</Code>" flag to its dev builds "
                        "only, so plain cargo commands ("<Code inline=true>"cargo check"</Code>", clippy, tests) run with other "
                        "flags and can\u{2019}t reuse what it compiled. Set the flag in your "<Code inline=true>".cargo/config.toml"</Code>" and tell "
                        "cargo-leptos not to add it itself: all builds then share one set of flags and reuse each other\u{2019}s "
                        "compiled dependencies."
                    </p>
                    <Code language=Language::Toml>
                        {indoc!(r#"
                            # .cargo/config.toml
                            [build]
                            rustflags = ["--cfg=web_sys_unstable_apis", "--cfg=erase_components"]

                            # Cargo.toml
                            [package.metadata.leptos]
                            disable-erase-components = true
                        "#)}
                    </Code>
                    <p>
                        "Costs: release builds use type-erased views too. This book does, and its release numbers on this "
                        "page were measured with them."
                    </p>
                </Section>

                <Section title="Development Bundle">
                    <p>
                        "Build the browser bundle with a profile of its own, without function names and optimized for size:"
                    </p>
                    <Code language=Language::Toml>
                        {indoc!(r#"
                            [package.metadata.leptos]
                            lib-profile-dev = "wasm-dev"

                            [profile.wasm-dev]
                            inherits = "dev"
                            opt-level = "z"
                            debug = false
                            strip = "symbols"

                            # A package with an override of its own doesn't get the profile-wide `strip`: repeat it.
                            [profile.wasm-dev.package.my-app]
                            debug = false
                            strip = "symbols"
                        "#)}
                    </Code>
                    <p>
                        "In this book, the function names were 117 MB of a 167 MB development bundle. With them gone, "
                        <Code inline=true>"opt-level = \"z\""</Code>" shrank the bundle from 43.8 MB to 19.7 MB: the browser "
                        "loads and compiles less on every reload, and a rebuild after a page edit took 12\u{2013}14 s instead "
                        "of 17\u{2013}18 s (wasm-bindgen handles the smaller file in 1.2 s instead of 3.7 s). A rebuild after a "
                        "change in leptonic took about 28 s either way."
                    </p>
                    <p>
                        "Costs: browser stack traces show "<Code inline=true>"wasm-function[1234]"</Code>" instead of function "
                        "names; panic messages keep their file and line. The server keeps its symbols."
                    </p>
                </Section>

                <Section title="Server Rendering Speed">
                    <p>"Build the server of your development builds with a little optimization:"</p>
                    <Code language=Language::Toml>
                        {indoc!(r#"
                            [package.metadata.leptos]
                            bin-profile-dev = "server-dev"

                            [profile.server-dev]
                            inherits = "dev"
                            opt-level = 1
                        "#)}
                    </Code>
                    <p>
                        "In leptonic\u{2019}s test app, the server rendered pages about 6\u{d7} faster than at "
                        <Code inline=true>"opt-level = 0"</Code>" (a page full of calendars in 87 ms instead of 550 ms), and a "
                        "rebuild after a change in leptonic took 24 s instead of 26 s: no slower. The profile keeps dev\u{2019}s "
                        "debug assertions, so it suits development and tests, not production."
                    </p>
                </Section>
            </Section>

            <Section title="Release Builds">
                <p>
                    <Code inline=true>"cargo leptos build --release"</Code>" and "<Code inline=true>"serve --release"</Code>
                    " use the release profiles. You run them while you develop too: to judge your app\u{2019}s speed, or "
                    "because tests run faster against an optimized bundle. Optimize for size without link-time optimization "
                    "there, and add it for the build you deploy."
                </p>

                <Section title="Release Profile">
                    <Code language=Language::Toml>
                        {indoc!(r#"
                            [package.metadata.leptos]
                            lib-profile-release = "wasm-release"

                            [profile.wasm-release]
                            inherits = "release"
                            opt-level = "z"
                            # Reuse what didn't change since the last release build.
                            incremental = true
                        "#)}
                    </Code>
                    <p>
                        "In leptonic\u{2019}s test app, this bundle is 13 MB (dev: 33 MB), its pages load in about 180 ms "
                        "instead of 450 ms with the dev bundle, and a rebuild after a change in leptonic takes about 20 s, as fast as a dev build. "
                        "The "<Code inline=true>"opt-level"</Code>"s 1, 2, 3 and "<Code inline=true>"\"s\""</Code>" loaded "
                        "pages about as fast; "<Code inline=true>"\"z\""</Code>" gave the smallest bundle. In this book, "
                        <Code inline=true>"\"s\""</Code>" made the bundle 11% larger than "<Code inline=true>"\"z\""</Code>
                        ". cargo-leptos runs wasm-opt on every release bundle."
                    </p>
                    <p>
                        "If your tests rely on debug assertions, add "<Code inline=true>"debug-assertions = true"</Code>" and "
                        <Code inline=true>"overflow-checks = true"</Code>" to this profile, as the test app does. Its server "
                        "uses the "<AnchorLink href="#server-rendering-speed">"optimized development profile"</AnchorLink>
                        " in release builds too ("<Code inline=true>"bin-profile-release"</Code>")."
                    </p>
                </Section>

                <Section title="Production Builds">
                    <p>
                        "Link-time optimization with one codegen unit lets the compiler drop unused code across crates. In the "
                        "test app it saved only 1 MB of 13 MB, but made a rebuild after a change in leptonic take 84 s instead "
                        "of 20 s. That is worth paying for the build you deploy, not for every build. Cargo reads profile "
                        "settings from the environment, so turn it on in your deployment\u{2019}s build command and keep the "
                        "profile fast:"
                    </p>
                    <Code language=Language::Shell>
                        {indoc!(r"
                            CARGO_PROFILE_WASM_RELEASE_LTO=true \
                            CARGO_PROFILE_WASM_RELEASE_CODEGEN_UNITS=1 \
                            cargo leptos build --release --precompress
                        ")}
                    </Code>
                </Section>
            </Section>

            <Section title="Serving the Bundle">
                <Section title="Compression">
                    <p>
                        "Serve the bundle compressed. This book\u{2019}s release bundle is 13.01 MB, 4.36 MB with gzip and "
                        "2.94 MB with brotli. "<Code inline=true>"--precompress"</Code>" (release builds only) writes gzip and "
                        "brotli files next to the bundle at build time, and leptos_axum\u{2019}s file handler "
                        "("<Code inline=true>"file_and_error_handler"</Code>", "<Code inline=true>"site_pkg_dir_service"</Code>
                        ") serves them to the browsers that accept them. Costs: about 20 s per build for a 13 MB bundle "
                        "(brotli at quality 11), which is why it belongs in the production build."
                    </p>
                    <p>
                        "Development builds benefit from compression too. cargo-leptos only runs "
                        <Code inline=true>"--precompress"</Code>" with "<Code inline=true>"--release"</Code>
                        ". This book prepares missing or stale compressed WASM files before its server accepts requests, "
                        "including with "<Code inline=true>"just serve"</Code>" and its development profiles: gzip at level 6 "
                        "and brotli at quality 4. Fresh files are reused, including the production build\u{2019}s more tightly "
                        "compressed files. Each bundle is compressed once, so reloads do not repeat that work."
                    </p>
                    <p>
                        "Compress the rest (pages, server function responses) per request with tower-http\u{2019}s "
                        <Code inline=true>"CompressionLayer"</Code>" (features "<Code inline=true>"compression-br"</Code>
                        " and "<Code inline=true>"compression-gzip"</Code>"), but not the bundle: compressing this book\u{2019}s "
                        "43 MB development bundle took 3.4 s of CPU time on every request. Precompressed files already carry "
                        "their encoding, so the layer leaves them alone."
                    </p>
                    <Code language=Language::Rust>
                        {indoc!(r#"
                            use tower_http::compression::{
                                CompressionLayer,
                                predicate::{DefaultPredicate, NotForContentType, Predicate},
                            };

                            let app = app.layer(
                                CompressionLayer::new().compress_when(
                                    DefaultPredicate::new().and(NotForContentType::const_new("application/wasm")),
                                ),
                            );
                        "#)}
                    </Code>
                </Section>

                <Section title="Caching">
                    <p>
                        "With "<Code inline=true>"hash-files"</Code>", cargo-leptos puts a hash of their content into the names "
                        "of the bundle, its JavaScript and your stylesheet, so a changed file gets a new name. Browsers can then "
                        "keep them for good, the bundle together with the code they compiled from it, so a returning "
                        "visitor\u{2019}s page loads without downloading the bundle again."
                    </p>
                    <Code language=Language::Toml>
                        {indoc!(r"
                            [package.metadata.leptos]
                            hash-files = true
                        ")}
                    </Code>
                    <p>
                        "In your shell, "<Code inline=true>"HydrationScripts"</Code>" finds the hashed names itself; replace "
                        "your "<Code inline=true>"Stylesheet"</Code>" with "<Code inline=true>"HashedStylesheet"</Code>" (from "
                        <Code inline=true>"leptos_meta"</Code>"). Then tell browsers to cache the files under "
                        <Code inline=true>"/pkg/"</Code>" (cargo-leptos\u{2019} "<Code inline=true>"site-pkg-dir"</Code>
                        "), whose names carry the hash:"
                    </p>
                    <Code language=Language::Rust>
                        {indoc!(r#"
                            // In the shell's <head>:
                            <HashedStylesheet options=options.clone() id="leptos"/>
                            <HydrationScripts options/>
                        "#)}
                    </Code>
                    <Code language=Language::Rust>
                        {indoc!(r#"
                            use axum::{
                                extract::Request,
                                http::{HeaderValue, header::CACHE_CONTROL},
                                middleware::{self, Next},
                                response::Response,
                            };

                            /// Lets browsers keep the files under `/pkg/` for good: their names carry their content's hash.
                            async fn cache_hashed_files(request: Request, next: Next) -> Response {
                                let hashed = request.uri().path().starts_with("/pkg/");
                                let mut response = next.run(request).await;
                                if hashed && response.status().is_success() {
                                    response.headers_mut().insert(
                                        CACHE_CONTROL,
                                        HeaderValue::from_static("public, max-age=31536000, immutable"),
                                    );
                                }
                                response
                            }

                            // In `main`, after the routes:
                            let app = app.layer(middleware::from_fn(cache_hashed_files));
                        "#)}
                    </Code>
                    <p>
                        "Files of your "<Code inline=true>"assets-dir"</Code>" keep their names, so they don\u{2019}t get "
                        "this header."
                    </p>
                </Section>
            </Section>

            <Section title="Locale Data">
                <p>
                    "Leptonic formats numbers and dates and compares text with ICU4X (see "
                    <Link href=format!("{}#internationalization", routes::doc::Ssr.materialize())>"Server-Side Rendering"</Link>
                    "), which compiles its locale data into your app: by default the data of every locale. Bake only the "
                    "locales your app supports. In this book, that made the release bundle 20% smaller (18.18 MB to 14.42 MB; "
                    "brotli 4.28 MB to 3.53 MB). Generate the data with "<Code inline=true>"icu4x-datagen"</Code>" of the "
                    "same minor version as your "<Code inline=true>"icu_*"</Code>" crates (leptonic\u{2019}s "
                    <Code inline=true>"scripts/icu-datagen.sh"</Code>" checks it for you):"
                </p>
                <Code language=Language::Shell>
                    {indoc!(r"
                        icu4x-datagen --format baked --markers all --segmenter-models none \
                            --locales ^en ^de ^fr --out icu4x-data --overwrite
                    ")}
                </Code>
                <Code language=Language::Toml>
                    {indoc!(r#"
                        # .cargo/config.toml of your app
                        [build]
                        rustflags = ["--cfg=web_sys_unstable_apis", "--cfg=erase_components", "--cfg=icu4x_custom_data"]

                        [env]
                        # Server and browser both use it: hydration needs both to format alike.
                        ICU4X_DATA_DIR = { value = "icu4x-data", relative = true }
                    "#)}
                </Code>
                <p>
                    <Code inline=true>"^de"</Code>" stands for German without its regional variants ("
                    <Code inline=true>"de-AT"</Code>", "<Code inline=true>"de-CH"</Code>", \u{2026}); other locales fall back "
                    "to their parent or the root locale. "<Code inline=true>"--markers all"</Code>" is required (the linker "
                    "drops the data your app doesn\u{2019}t use), and without segmenter models, which leptonic doesn\u{2019}t "
                    "use, the directory holds 6.2 MB instead of 17\u{2013}18 MB of sources. Costs: regenerate the data after every ICU4X "
                    "update, as data of another minor version doesn\u{2019}t compile."
                </p>
            </Section>

            <Section title="Linking">
                <p>
                    "Every rebuild links the server binary and the bundle again. The bundle is always linked "
                    "with rust-lld, and since Rust 1.90, so is a server on x86_64 Linux, which is fast already. mold is "
                    "faster still: it linked the test app\u{2019}s server in 0.75 s instead of 1.2 s. That is rarely worth "
                    "the extra setup, unless linking makes up much of your rebuilds:"
                </p>
                <Code language=Language::Toml>
                    {indoc!(r#"
                        # .cargo/config.toml
                        [target.x86_64-unknown-linux-gnu]
                        linker = "clang"
                        rustflags = ["--cfg=web_sys_unstable_apis", "--cfg=erase_components", "-C", "link-arg=-fuse-ld=mold"]
                    "#)}
                </Code>
                <p>
                    "Target-specific "<Code inline=true>"rustflags"</Code>" replace "<Code inline=true>"[build].rustflags"</Code>
                    " instead of adding to them, so repeat your "<Code inline=true>"--cfg"</Code>" flags there (see "
                    <Link href=format!("{}#build-configuration", routes::doc::Installation.materialize())>"Build Configuration"</Link>
                    ")."
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
