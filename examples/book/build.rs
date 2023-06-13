use std::io::Write;
use std::path::{Path, PathBuf};

const JS_DIR: &str = "./generated/js";

pub fn main() {
    let root: std::path::PathBuf = std::env::var("CARGO_MANIFEST_DIR").unwrap().into();
    leptonic_theme::generate(root.join("generated").join("leptonic"));
    println!("cargo:warning=theme written");

    make_dir(Path::new(JS_DIR));
    println!("cargo:warning=js dir created");

    write_str(
        leptos_tiptap_build::TIPTAP_BUNDLE_MIN_JS,
        path(JS_DIR, "tiptap-bundle.min.js"),
    );
    println!("cargo:warning=tiptap-bundle.min.js written");

    write_str(leptos_tiptap_build::TIPTAP_JS, path(JS_DIR, "tiptap.js"));
    println!("cargo:warning=tiptap.js written");
}

fn path<'a>(root: &'static str, path: &'a str) -> PathBuf {
    PathBuf::from(format!("{root}/{path}"))
}

fn make_dir(path: &Path) {
    println!("Creating directory: {}", path.display());
    std::fs::create_dir_all(path).unwrap();
}

fn write_str(src: impl AsRef<str>, path: impl AsRef<Path>) {
    let src = src.as_ref();
    let path = path.as_ref();
    println!(
        "Writing {path} (len = {len})",
        len = src.len(),
        path = path.display(),
    );
    let mut file = std::fs::File::create(path).unwrap();
    file.write(src.as_bytes()).unwrap();
}
