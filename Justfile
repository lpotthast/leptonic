# Run `cargo install just`. Then simply run `just` to get a list of executable recepies.
# See https://github.com/casey/just for furthrt details about Just.

# Lists all available commands.
default:
  just --list

# Perform a one-time setup to get up and running with Rust development.
once:
  just enable-wasm
  just use-stable
  just install-tools

# Enable the WASM target, enabling frontend build.
enable-wasm:
  rustup target add wasm32-unknown-unknown

# Enable the nightly Rust compiler
use-nightly:
  rustup default nightly
  rustup update

# Enable the stable Rust compiler
use-stable:
  rustup default stable
  rustup update

# Install dependencies for building, running examples, profiling and more...
install-tools:
  cargo install just # Tool to execute the recipes of this Justfile.
  cargo install cargo-clean-all # Clean up build artifacts, cluttering up your system.
  cargo install cargo-edit # Make `cargo upgrade` available, upgrading dependencies in your Cargo.toml.
  cargo install cargo-expand # Expand macros, super helpful when debugging procedural macros.
  cargo install cargo-llvm-lines # Count lines of LLVM IR per generic function.
  cargo install cargo-sort # Sort the dependencies section of a Cargo.toml file.
  cargo install cargo-udeps # Find unused dependencies (RustRover has that functionality already built in).
  cargo install cargo-upgrades # Check for upgradable dependencies.
  cargo install cargo-watch # Run a command, watch for filesystem changes and re-run that command automatically.
  cargo install cargo-whatfeatures # Inspect features made available by a specific crate.
  cargo install tokei # Count your code, quickly.
  cargo install trunk # Build, watch, serve WASM frontends.
  cargo install twiggy # Inspect WASM bundles.
  cargo install create-tauri-app # Create new Tauri applications.
  cargo install tauri-cli # Run tauri applications.
  cargo install cargo-leptos # Run leptos applications.
  cargo install wasm-bindgen-cli # WASM bindgen cli.

# Find the minimum supported rust version
msrv:
    cargo install cargo-msrv
    cargo msrv --min "2021" --path leptonic
    cargo msrv --min "2021" --path leptonic-theme

serve-ts:
  tailscale serve https+insecure://localhost:4100

# Serve the Book example (https://127.0.0.1:4100)
serve:
  cd ./examples/book-ssr && cargo leptos serve

# Serve the Book example in release mode
serve-release:
  cd ./examples/book-ssr && cargo leptos serve --release

# Serve the test app for manual inspection (http://127.0.0.1:4200)
serve-test-app:
  cd ./testing/test-app && cargo leptos serve

# Serve the CSR template (http://127.0.0.1:4001)
serve-template-csr:
  cd ./examples/leptonic-template-csr && trunk serve

# Serve the SSR template (http://127.0.0.1:3000)
serve-template-ssr:
  cd ./examples/leptonic-template-ssr && cargo leptos serve

# Serve the SSR Nightly template (http://127.0.0.1:3000)
serve-template-ssr-nightly:
  cd ./examples/leptonic-template-ssr-nightly && cargo +nightly leptos serve

# Serve the Tauri template (http://127.0.0.1:1420)
serve-template-tauri:
  cd ./examples/leptonic-template-tauri && cargo tauri dev

# Fast feedback: unit tests of leptonic (all features) and leptonic-theme. No browser involved.
unit-test:
  cargo test -p leptonic --features full --lib
  cargo test -p leptonic --features full,ssr --lib
  cargo test -p leptonic-theme

# Everything that must pass before work is considered done. Fails on the first problem.
verify:
  cargo fmt --all --check
  cargo clippy -p leptonic --tests
  cargo clippy -p leptonic --features full --tests
  cargo clippy -p leptonic --no-default-features --features atoms,clipboard --tests
  cargo clippy -p leptonic --features full,ssr --tests
  cargo clippy -p leptonic --features full,hydrate --tests
  cargo clippy -p leptonic-theme --tests
  LEPTOS_OUTPUT_NAME=leptonic-test-app cargo clippy --manifest-path ./testing/test-app/Cargo.toml --features ssr
  LEPTOS_OUTPUT_NAME=leptonic-test-app cargo clippy --manifest-path ./testing/test-app/Cargo.toml --lib --no-default-features --features hydrate --target wasm32-unknown-unknown
  # From the book's directory, so that its `.cargo/config.toml` (`LEPTOS_OUTPUT_NAME`, ICU4X data) applies.
  cd ./examples/book-ssr && cargo clippy --features ssr
  cd ./examples/book-ssr && cargo clippy --lib --no-default-features --features hydrate --target wasm32-unknown-unknown
  just unit-test
  just browser-test

# Run browser tests (headless by default)
browser-test:
  cargo test --manifest-path ./leptonic/Cargo.toml --test browser_test -- --nocapture

# Run browser tests with visible browser (for debugging)
browser-test-visible:
  BROWSER_TEST_VISIBLE=1 BROWSER_TEST_PARALLELISM=1 cargo test --manifest-path ./leptonic/Cargo.toml --test browser_test -- --nocapture

# Run the book's browser tests (every documentation page: errors, dark theme, links, phone width, Markdown export).
book-browser-test:
  cd ./examples/book-ssr && cargo test --test browser_test -- --nocapture

# Run the book's browser tests with visible browser (for debugging)
book-browser-test-visible:
  cd ./examples/book-ssr && BROWSER_TEST_VISIBLE=1 cargo test --test browser_test -- --nocapture

# Serve the book on its own port, target directory and site output, next to `just serve` (default port 4300).
# Builds against the live library sources. The reload port is port + 1.
book-serve-isolated port="4300":
  cd ./examples/book-ssr && \
    CARGO_TARGET_DIR=target/isolated-{{port}} \
    LEPTOS_SITE_ROOT=target/isolated-{{port}}/site \
    LEPTOS_SITE_ADDR=127.0.0.1:{{port}} \
    LEPTOS_RELOAD_PORT=$(({{port}} + 1)) \
    cargo leptos serve

# Regenerate the book's ICU4X data (`examples/book-ssr/icu4x-data`): after every ICU4X update (old data breaks the
# build) and for another locale. The book bakes only the data of the locales its demos format with (3.6 MB less wasm).
book-icu-data:
  ./scripts/icu-datagen.sh examples/book-ssr examples/book-ssr/icu4x-data ^en ^en-GB ^de ^ar ^ar-EG ^es ^fr ^hi ^ja ^pt ^pt-BR ^sv

# Check which process is occupying the given port.
# This can help you find out which process to kill if some process has gone rogue.
check-port port:
  lsof -i :{{port}}

# Run `cargo sort` for every crate.
sort:
  cargo sort ./leptonic -w -g
  cargo sort ./leptonic-theme -w -g
  cargo sort ./testing/test-app -w -g
  cargo sort ./examples/book-ssr -w -g
  cargo sort ./examples/leptonic-template-csr -w -g
  cargo sort ./examples/leptonic-template-ssr -w -g
  cargo sort ./examples/leptonic-template-tauri -w -g

# Run `cargo fmt` for every crate.
fmt:
  cargo fmt --all --manifest-path ./leptonic/Cargo.toml
  cargo +nightly fmt --all --manifest-path ./leptonic/Cargo.toml -- --unstable-features --config imports_granularity=Crate,group_imports=StdExternalCrate
  cargo fmt --all --manifest-path ./leptonic-theme/Cargo.toml
  cargo +nightly fmt --all --manifest-path ./leptonic-theme/Cargo.toml -- --unstable-features --config imports_granularity=Crate,group_imports=StdExternalCrate
  cargo fmt --all --manifest-path ./testing/test-app/Cargo.toml
  cargo +nightly fmt --all --manifest-path ./testing/test-app/Cargo.toml -- --unstable-features --config imports_granularity=Crate,group_imports=StdExternalCrate
  cargo fmt --all --manifest-path ./examples/book-ssr/Cargo.toml
  cargo +nightly fmt --all --manifest-path ./examples/book-ssr/Cargo.toml -- --unstable-features --config imports_granularity=Crate,group_imports=StdExternalCrate
  cargo fmt --all --manifest-path ./examples/leptonic-template-csr/Cargo.toml
  cargo +nightly fmt --all --manifest-path ./examples/leptonic-template-csr/Cargo.toml -- --unstable-features --config imports_granularity=Crate,group_imports=StdExternalCrate
  cargo fmt --all --manifest-path ./examples/leptonic-template-ssr/Cargo.toml
  cargo +nightly fmt --all --manifest-path ./examples/leptonic-template-ssr/Cargo.toml -- --unstable-features --config imports_granularity=Crate,group_imports=StdExternalCrate
  cargo fmt --all --manifest-path ./examples/leptonic-template-tauri/Cargo.toml
  cargo +nightly fmt --all --manifest-path ./examples/leptonic-template-tauri/Cargo.toml -- --unstable-features --config imports_granularity=Crate,group_imports=StdExternalCrate

leptosfmt:
  cargo install leptosfmt
  leptosfmt ./leptonic/*
  leptosfmt ./examples/book/*
  leptosfmt ./testing/test-app/*

# Run `cargo update` for every crate, updating the dependencies of all crates to the latest non-breaking version. Rewrites Cargo.lock files.
update:
  cargo update --manifest-path ./leptonic/Cargo.toml
  cargo update --manifest-path ./leptonic-theme/Cargo.toml
  cargo update --manifest-path ./testing/test-app/Cargo.toml
  cargo update --manifest-path ./examples/book-ssr/Cargo.toml
  cargo update --manifest-path ./examples/leptonic-template-csr/Cargo.toml
  cargo update --manifest-path ./examples/leptonic-template-ssr/Cargo.toml
  cargo update --manifest-path ./examples/leptonic-template-tauri/Cargo.toml

# Run `cargo test` for every crate.
test:
  cargo test --manifest-path ./leptonic/Cargo.toml
  cargo test --manifest-path ./leptonic-theme/Cargo.toml
  # From the book's directory, so that its `.cargo/config.toml` (`LEPTOS_OUTPUT_NAME`) applies.
  cd ./examples/book-ssr && cargo test
  cargo test --manifest-path ./examples/leptonic-template-csr/Cargo.toml
  cargo test --manifest-path ./examples/leptonic-template-ssr/Cargo.toml
  cargo test --manifest-path ./examples/leptonic-template-tauri/Cargo.toml

# Run `cargo upgrades` for every crate, checking if new crate versions including potentially breaking changes are available.
upgrades: # "-" prefixes allow for non-zero status codes!
  -cargo upgrades --manifest-path ./leptonic/Cargo.toml
  -cargo upgrades --manifest-path ./leptonic-theme/Cargo.toml
  -cargo upgrades --manifest-path ./testing/test-app/Cargo.toml
  -cargo upgrades --manifest-path ./examples/book-ssr/Cargo.toml
  -cargo upgrades --manifest-path ./examples/leptonic-template-csr/Cargo.toml
  -cargo upgrades --manifest-path ./examples/leptonic-template-ssr/Cargo.toml
  -cargo upgrades --manifest-path ./examples/leptonic-template-tauri/Cargo.toml

# Run `cargo upgrade` for every crate, automatically bumping all dependencies to their latest versions
upgrade: # "-" prefixes allow for non-zero status codes!
  -cargo upgrade --manifest-path ./leptonic/Cargo.toml
  -cargo upgrade --manifest-path ./leptonic-theme/Cargo.toml
  -cargo upgrade --manifest-path ./testing/test-app/Cargo.toml
  -cargo upgrade --manifest-path ./examples/book-ssr/Cargo.toml
  -cargo upgrade --manifest-path ./examples/leptonic-template-csr/Cargo.toml
  -cargo upgrade --manifest-path ./examples/leptonic-template-ssr/Cargo.toml
  -cargo upgrade --manifest-path ./examples/leptonic-template-tauri/Cargo.toml

# Run `cargo clippy --tests` for every crate. Lint levels are configured in each crate's [lints.clippy] section.
# Free disk space: delete the incremental caches of the agents' shared target dirs (they pile up old sessions;
# the next build is slower, nothing else changes).
clean-agent-incremental:
  rm -rf ./target/agents/*/incremental ./testing/test-app/target/agents/*/incremental ./examples/book-ssr/target/agents/*/incremental

# The library's API docs; rustdoc warnings (broken or ambiguous links) fail.
doc:
  RUSTDOCFLAGS="-D warnings" cargo doc --manifest-path ./leptonic/Cargo.toml --features full --no-deps

clippy: # "-" prefixes allow for non-zero status codes!
  -cargo clippy --tests --manifest-path ./leptonic/Cargo.toml
  -cargo clippy --tests --manifest-path ./leptonic/Cargo.toml --features full
  # Release too: some lints depend on type sizes that differ there (`trivially_copy_pass_by_ref`).
  -cargo clippy --tests --release --manifest-path ./leptonic/Cargo.toml --features full
  # The atoms without the components layer: what consumers like agnite dev-ui build (consumers.md).
  -cargo clippy --tests --manifest-path ./leptonic/Cargo.toml --no-default-features --features atoms,clipboard
  -cargo clippy --tests --manifest-path ./leptonic-theme/Cargo.toml
  -cargo clippy --tests --manifest-path ./testing/test-app/Cargo.toml
  -cd ./examples/book-ssr && cargo clippy --tests
  -cargo clippy --tests --manifest-path ./examples/leptonic-template-csr/Cargo.toml
  -cargo clippy --tests --manifest-path ./examples/leptonic-template-ssr/Cargo.toml
  -cargo clippy --tests --manifest-path ./examples/leptonic-template-tauri/Cargo.toml
