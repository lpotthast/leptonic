use cargo_toml::{Manifest, Value};
use lazy_static::lazy_static;
use std::{path::PathBuf, str::FromStr};

lazy_static! {
    static ref ENABLE_LOGGING: bool = {
        option_env!("LEPTONIC_BUILD_ENABLE_LOGGING")
            .and_then(|v| str::parse::<bool>(v).ok())
            .unwrap_or(false)
    };
    static ref MIN_LOG_LEVEL: Level = {
        option_env!("LEPTONIC_BUILD_MIN_LOG_LEVEL")
            .and_then(|v| str::parse::<Level>(v).ok())
            .unwrap_or(Level::Debug)
    };
}

#[allow(clippy::unwrap_used)]
pub fn main() {
    let target_dir = get_cargo_target_dir().unwrap();
    let root_dir = target_dir.parent().unwrap().to_owned();
    log(Level::Debug, format!("root_dir is: {root_dir:?}"));

    let cargo_lock_path = root_dir.join("Cargo.lock");
    let cargo_toml_path = root_dir.join("Cargo.toml");
    assert!(
        cargo_toml_path.exists(),
        //.expect("Can't check existence of file Cargo.toml"),
        "Unable to find '{}'",
        cargo_toml_path.display()
    );

    // Read config from Cargo.toml file! Abort if the Cargo.toml has no config.
    let cargo_toml: Manifest<Value> =
        cargo_toml::Manifest::from_path_with_metadata(&cargo_toml_path)
            .expect("Deserializable Cargo.toml");

    if cargo_toml.package.is_none() {
        log(
            Level::Debug,
            "aborting. Root dir does not contain a package.",
        );
        return;
    }

    let meta = cargo_toml
        .package()
        .metadata
        .as_ref()
        .and_then(|m| m.get("leptonic"));

    if meta.is_none() {
        log(
            Level::Debug,
            "aborting. Root dir is a package without specifying leptonic metadata.",
        );
        return;
    }

    let meta = meta.unwrap();
    let table = meta.as_table().unwrap();

    println!("cargo:rerun-if-changed={}", cargo_toml_path.display());
    println!("cargo:rerun-if-changed={}", cargo_lock_path.display());

    let relative_style_dir = table
        .get("style-dir")
        .expect("'style-dir' being defined")
        .as_str()
        .expect("str");
    let relative_js_dir = table
        .get("js-dir")
        .expect("'js-dir' being defined")
        .as_str()
        .expect("str");

    log(
        Level::Debug,
        format!("relative_style_dir is: {relative_style_dir:?}"),
    );
    log(
        Level::Debug,
        format!("relative_js_dir is: {relative_js_dir:?}"),
    );

    let style_dir = root_dir.join(relative_style_dir);
    #[allow(unused_variables)]
    let js_dir = root_dir.join(relative_js_dir);

    let theme_dir = style_dir.join("leptonic");
    leptonic_theme::generate(&theme_dir).unwrap();
    log(
        Level::Info,
        format!("theme written to {}", theme_dir.display()),
    );

    #[cfg(feature = "tiptap")]
    copy_tiptap_files(&js_dir);
}

#[cfg(feature = "tiptap")]
fn copy_tiptap_files(js_dir: &PathBuf) {
    use std::io::Write;

    std::fs::create_dir_all(js_dir).unwrap();

    std::fs::File::create(js_dir.join("tiptap-bundle.min.js"))
        .unwrap()
        .write_all(leptos_tiptap_build::TIPTAP_BUNDLE_MIN_JS.as_bytes())
        .unwrap();
    log(
        Level::Info,
        format!("tiptap-bundle.min.js written to {}", js_dir.display()),
    );

    std::fs::File::create(js_dir.join("tiptap.js"))
        .unwrap()
        .write_all(leptos_tiptap_build::TIPTAP_JS.as_bytes())
        .unwrap();
    log(
        Level::Info,
        format!("tiptap.js written to {}", js_dir.display()),
    );
}

// Credits @ssrlive (source: https://github.com/rust-lang/cargo/issues/9661)
fn get_cargo_target_dir() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(std::env::var("OUT_DIR")?);
    log(Level::Debug, format!("out_dir is: {out_dir:?}"));
    let mut target_dir = None;
    let mut sub_path = out_dir.as_path();
    while let Some(parent) = sub_path.parent() {
        if parent.ends_with("target") {
            target_dir = Some(parent);
            break;
        }
        sub_path = parent;
    }
    let target_dir = target_dir
        .ok_or_else(|| format!("Could not find `target` dir in parents of {out_dir:?}"))?;
    Ok(target_dir.to_path_buf())
}

fn log(level: Level, msg: impl AsRef<str>) {
    let msg = msg.as_ref();
    if *ENABLE_LOGGING && level >= *MIN_LOG_LEVEL {
        println!("cargo:warning=[{level}] {msg}");
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[allow(dead_code)]
enum Level {
    Debug = 0,
    Info = 1,
    Warn = 2,
    Error = 3,
}

impl FromStr for Level {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "debug" | "Debug" | "DEBUG" => Ok(Self::Debug),
            "info" | "Info" | "INFO" => Ok(Self::Info),
            "warn" | "Warn" | "WARN" => Ok(Self::Warn),
            "error" | "Error" | "ERROR" => Ok(Self::Error),
            _ => Err(format!("'{s}' is not a valid LogLevel.")),
        }
    }
}

impl std::fmt::Display for Level {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Debug => "DEBUG",
            Self::Info => "INFO",
            Self::Warn => "WARN",
            Self::Error => "ERROR",
        })
    }
}
