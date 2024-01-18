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
  cargo install cargo-clean-all # Clean up build artifact cluttering up your system.
  cargo install cargo-edit # Make `cargo upgrade` available, upgrading dependencies in your Cargo.toml.
  cargo install cargo-expand # Expand macros, super helpful ehn debugging procedural macros.
  cargo install cargo-llvm-lines # Count lines of LLVM IR per generic function.
  cargo install cargo-sort # Sort the dependencies section of a Cargo.toml file.
  cargo install cargo-udeps # Find unused dependencies.
  cargo install cargo-upgrades # Check for upgradable dependencies.
  cargo install cargo-watch # Run a command and watch for filesystem changes.
  cargo install cargo-whatfeatures # Inspect features made available by a specific crate.
  cargo install just # Tool to execute the recepies of this Justfile.
  cargo install tokei # Count your code, line counts, quickly.
  cargo install trunk # Build, watch, server WASM frontends.
  cargo install twiggy # Inspect WASM bundles.

# Find the minimum supported rust version
msrv:
    cargo install cargo-msrv
    cargo msrv --min "2021" --path leptonic
    cargo msrv --min "2021" --path leptonic-theme

# Serve the Book example
serve:
  cd ./examples/book && trunk serve

# Check which process is occupying the given port.
# This can help you find out which process to kill if some process has gone rogue.
check-port port:
  lsof -i :{{port}}

# Run `cargo sort` for every crate.
sort:
  cargo sort ./leptonic -w -g
  cargo sort ./leptonic-theme -w -g
  cargo sort ./examples/book -w -g

# Run `cargo fmt` for every crate.
fmt:
  cargo fmt --all --manifest-path ./leptonic/Cargo.toml
  cargo fmt --all --manifest-path ./leptonic-theme/Cargo.toml
  cargo fmt --all --manifest-path ./examples/book/Cargo.toml

leptosfmt:
  cargo install leptosfmt
  leptosfmt ./leptonic/*
  leptosfmt ./examples/book/*

# Run `cargo update` for every crate, updating the dependencies of all crates to the latest non-breaking version. Rewrites Cargo.lock files.
update:
  cargo update --manifest-path ./leptonic/Cargo.toml
  cargo update --manifest-path ./leptonic-theme/Cargo.toml
  cargo update --manifest-path ./examples/book/Cargo.toml

# Run `cargo test` for every crate.
test:
  cargo test --manifest-path ./leptonic/Cargo.toml
  cargo test --manifest-path ./leptonic-theme/Cargo.toml
  cargo test --manifest-path ./examples/book/Cargo.toml

# Run `cargo upgrades` for every crate, checking if new crate versions including potentially breaking changes are available.
upgrades: # "-" prefixes allow for non-zero status codes!
  -cargo upgrades --manifest-path ./leptonic/Cargo.toml
  -cargo upgrades --manifest-path ./leptonic-theme/Cargo.toml
  -cargo upgrades --manifest-path ./examples/book/Cargo.toml

# Run `cargo upgrade` for every crate, automatically bumping all dependencies to their latest versions
upgrade: # "-" prefixes allow for non-zero status codes!
  -cargo upgrade --manifest-path ./leptonic/Cargo.toml
  -cargo upgrade --manifest-path ./leptonic-theme/Cargo.toml
  -cargo upgrade --manifest-path ./examples/book/Cargo.toml

# Run `cargo clippy --tests -- -Dclippy::all -Dclippy::pedantic` for every crate.
clippy: # "-" prefixes allow for non-zero status codes!
  -cargo clippy --tests --manifest-path ./leptonic/Cargo.toml -- -Dclippy::all -Dclippy::pedantic
  -cargo clippy --tests --manifest-path ./leptonic-theme/Cargo.toml -- -Dclippy::all -Dclippy::pedantic
  -cargo clippy --tests --manifest-path ./examples/book/Cargo.toml -- -Dclippy::all -Dclippy::pedantic
