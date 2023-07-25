# Find the minimum supported rust version
msrv:
    cargo install cargo-msrv
    cargo msrv --min "2021" --path leptonic
    cargo msrv --min "2021" --path leptonic-theme
