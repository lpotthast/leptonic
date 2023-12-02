use std::io::Write;

#[allow(clippy::unwrap_used)]
pub fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Cargo.lock");

    let root: std::path::PathBuf = std::env::var("CARGO_MANIFEST_DIR").unwrap().into();
    let generated_dir = root.join("generated");
    let js_dir = generated_dir.join("js");

    leptonic_theme::generate(generated_dir.join("leptonic")).unwrap();
    println!("cargo:warning=theme written");

    std::fs::create_dir_all(js_dir.clone()).unwrap();
    println!("cargo:warning=js dir created");

    std::fs::File::create(js_dir.join("tiptap-bundle.min.js"))
        .unwrap()
        .write_all(leptos_tiptap_build::TIPTAP_BUNDLE_MIN_JS.as_bytes())
        .unwrap();
    println!("cargo:warning=tiptap-bundle.min.js written");

    std::fs::File::create(js_dir.join("tiptap.js"))
        .unwrap()
        .write_all(leptos_tiptap_build::TIPTAP_JS.as_bytes())
        .unwrap();
    println!("cargo:warning=tiptap.js written");
}
