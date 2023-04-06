pub fn main() {
    let root: std::path::PathBuf = std::env::var("CARGO_MANIFEST_DIR").unwrap().into();
    leptonic_theme::generate(root.join("generated").join("leptonic"));
    println!("cargo:warning=theme written")
}
