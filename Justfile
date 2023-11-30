# See: https://github.com/casey/just

# Lists all available commands.
default:
  just --list

# Find the minimum supported rust version
msrv:
    cargo install cargo-msrv
    cargo msrv --min "2021" --path leptonic
    cargo msrv --min "2021" --path leptonic-theme

# Enable the nightly compiler
setup-rust-nightly:
  rustup default nightly
  rustup target add wasm32-unknown-unknown
  rustup update

# Enable the stable compiler
setup-rust-stable:
  rustup default stable
  rustup target add wasm32-unknown-unknown
  rustup update

# Install dependencies for building, running examples, profiling and more...
install-tools:
  cargo install --git https://github.com/ctron/trunk --branch trunk-ng trunk-ng
  cargo install twiggy
  cargo install cargo-expand
  cargo install cargo-whatfeatures
  cargo install cargo-upgrades
  cargo install cargo-edit
  cargo install cargo-clean-all

# Serve the Book example
serve:
  cd ./examples/book && trunk-ng serve

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
