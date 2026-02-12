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

# Serve the Book example (https://127.0.0.1:4100)
serve:
  cd ./examples/book-ssr && cargo leptos serve

# Serve the Book example in release mode
serve-release:
  cd ./examples/book-ssr && cargo leptos serve --release

# Serve the test app (for manual inspection)
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

# Run browser tests (headless by default)
browser-test:
  cargo test --manifest-path ./leptonic/Cargo.toml --test browser_test -- --nocapture

# Run browser tests with visible browser (for debugging)
browser-test-visible:
  BROWSER_TEST_VISIBLE=1 cargo test --manifest-path ./leptonic/Cargo.toml --test browser_test -- --nocapture

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
  cargo test --manifest-path ./examples/book-ssr/Cargo.toml
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

# Run `cargo clippy --tests -- -Dclippy::all -Dclippy::pedantic` for every crate.
clippy: # "-" prefixes allow for non-zero status codes!
  -cargo clippy --tests --manifest-path ./leptonic/Cargo.toml -- -Dclippy::all -Dclippy::pedantic
  -cargo clippy --tests --manifest-path ./leptonic-theme/Cargo.toml -- -Dclippy::all -Dclippy::pedantic
  -cargo clippy --tests --manifest-path ./testing/test-app/Cargo.toml -- -Dclippy::all -Dclippy::pedantic
  -cargo clippy --tests --manifest-path ./examples/book-ssr/Cargo.toml -- -Dclippy::all -Dclippy::pedantic
  -cargo clippy --tests --manifest-path ./examples/leptonic-template-csr/Cargo.toml -- -Dclippy::all -Dclippy::pedantic
  -cargo clippy --tests --manifest-path ./examples/leptonic-template-ssr/Cargo.toml -- -Dclippy::all -Dclippy::pedantic
  -cargo clippy --tests --manifest-path ./examples/leptonic-template-tauri/Cargo.toml -- -Dclippy::all -Dclippy::pedantic
