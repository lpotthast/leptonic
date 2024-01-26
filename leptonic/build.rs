use std::{io::Write, path::PathBuf};

const ENABLE_LOGGING: bool = false;

#[allow(clippy::unwrap_used)]
pub fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Cargo.lock");

    let target_dir = get_cargo_target_dir().unwrap();
    let root = search_parent_dir_containing_cargo_toml(target_dir).unwrap();
    log(format!("root is: {root:?}"));

    let generated_dir = root.join("generated");
    let js_dir = root.join("public").join("js");

    leptonic_theme::generate(generated_dir.join("leptonic")).unwrap();
    log("theme written");

    std::fs::create_dir_all(js_dir.clone()).unwrap();
    log("js dir created");

    std::fs::File::create(js_dir.join("tiptap-bundle.min.js"))
        .unwrap()
        .write_all(leptos_tiptap_build::TIPTAP_BUNDLE_MIN_JS.as_bytes())
        .unwrap();
    log("tiptap-bundle.min.js written");

    std::fs::File::create(js_dir.join("tiptap.js"))
        .unwrap()
        .write_all(leptos_tiptap_build::TIPTAP_JS.as_bytes())
        .unwrap();
    log("tiptap.js written");
}

// Credits @ssrlive (source: https://github.com/rust-lang/cargo/issues/9661)
fn get_cargo_target_dir() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(std::env::var("OUT_DIR")?);
    let profile = std::env::var("PROFILE")?;
    let mut target_dir = None;
    let mut sub_path = out_dir.as_path();
    while let Some(parent) = sub_path.parent() {
        if parent.ends_with(&profile) {
            target_dir = Some(parent);
            break;
        }
        sub_path = parent;
    }
    let target_dir = target_dir.ok_or("not found")?;
    Ok(target_dir.to_path_buf())
}

fn search_parent_dir_containing_cargo_toml(
    mut start: PathBuf,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
    loop {
        let mut cargo_toml = start.clone();
        cargo_toml.push("Cargo.toml");

        if cargo_toml.try_exists()? {
            return Ok(start);
        }

        start = start.parent().ok_or_else(|| "No more parents")?.to_owned();
    }
}

fn log(msg: impl AsRef<str>) {
    if ENABLE_LOGGING {
        println!("cargo:warning={}", msg.as_ref());
    }
}
