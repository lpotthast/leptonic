use anyhow::{Context, Result};
use cargo_toml::{Manifest, Value};
use lazy_static::lazy_static;
use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

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

#[derive(Debug)]
struct LeptonicMetadata {
    relative_style_dir: String,
    relative_js_dir: String,
}

#[allow(clippy::unwrap_used)]
pub fn main() -> Result<()> {
    let out_dir = get_out_dir().context("Could not find 'out_dir'.")?;
    let target_dir = get_cargo_target_dir(&out_dir).context("Could not find 'target_dir'.")?;
    let root_dir = target_dir
        .parent()
        .context("Expected 'target_dir' to have a parent.")?
        .to_owned();

    log(Level::Debug, format!("root_dir is: {root_dir:?}"));

    let cargo_lock_path = root_dir.join("Cargo.lock");
    let cargo_toml_path = root_dir.join("Cargo.toml");
    assert!(
        cargo_toml_path.exists(),
        //.expect("Can't check existence of file Cargo.toml"),
        "Unable to find '{}'",
        cargo_toml_path.display()
    );

    let metadata = match read_leptonic_metadata(&cargo_toml_path)? {
        Some(metadata) => metadata,
        None => return Ok(()),
    };

    println!("cargo:rerun-if-changed={}", cargo_lock_path.display());
    println!("cargo:rerun-if-changed={}", cargo_toml_path.display());

    let style_dir = root_dir.join(&metadata.relative_style_dir);
    #[allow(unused_variables)]
    let js_dir = root_dir.join(&metadata.relative_js_dir);

    let theme_dir = style_dir.join("leptonic");
    leptonic_theme::generate(&theme_dir).unwrap();
    log(
        Level::Info,
        format!("theme written to {}", theme_dir.display()),
    );

    #[cfg(feature = "tiptap")]
    copy_tiptap_files(&js_dir);

    Ok(())
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

/// Parse the Cargo.toml file! Abort if the Cargo.toml has no config.
fn read_leptonic_metadata(cargo_toml_path: &PathBuf) -> Result<Option<LeptonicMetadata>> {
    let cargo_toml: Manifest<Value> =
        cargo_toml::Manifest::from_path_with_metadata(&cargo_toml_path).with_context(|| {
            format!(
                "Could not parse Cargo.toml at '{}'",
                cargo_toml_path.display()
            )
        })?;

    log(
        Level::Debug,
        format!("Processing '{}'", cargo_toml_path.display()),
    );

    let leptonic_metadata = cargo_toml
        .package
        .as_ref()
        .and_then(|pkg| pkg.metadata.as_ref())
        .or_else(|| cargo_toml.workspace.as_ref()?.metadata.as_ref())
        .and_then(|metadata| metadata.get("leptonic"));

    let meta = match leptonic_metadata {
        Some(metadata) => {
            // Found "leptonic" in either package or workspace metadata, proceed
            log(
                Level::Info,
                format!(
                    "Found 'leptonic' in metadata of package or workspace: {:?}",
                    metadata
                ),
            );
            metadata
        }
        None => {
            log(Level::Debug, "Aborting. Cargo.toml in root dir does not contain a package or workspace or is missing the necessary metadata.");
            return Ok(None);
        }
    };

    let table = meta
        .as_table()
        .context("Leptonic metadata was not of type 'table'.")?;

    let relative_style_dir = table
        .get("style-dir")
        .context("Leptonic's 'style-dir' metadata was not declared.")?
        .as_str()
        .context("Leptonic's 'style-dir' metadata was not of type 'string'.")?
        .to_owned();

    let relative_js_dir = table
        .get("js-dir")
        .context("Leptonic's 'js-dir' metadata was not declared.")?
        .as_str()
        .context("Leptonic's 'js-dir' metadata was not of type 'string'.")?
        .to_owned();

    log(
        Level::Debug,
        format!("relative_style_dir is: {relative_style_dir:?}"),
    );
    log(
        Level::Debug,
        format!("relative_js_dir is: {relative_js_dir:?}"),
    );

    Ok(Some(LeptonicMetadata {
        relative_style_dir,
        relative_js_dir,
    }))
}

fn get_out_dir() -> Result<PathBuf> {
    let out_dir = PathBuf::from(std::env::var("OUT_DIR")?);
    log(Level::Debug, format!("out_dir is: {out_dir:?}"));
    Ok(out_dir)
}

// Credits @ssrlive (source: https://github.com/rust-lang/cargo/issues/9661)
fn get_cargo_target_dir(out_dir: impl AsRef<Path>) -> Result<PathBuf> {
    let mut target_dir = None;
    let mut sub_path = out_dir.as_ref();
    while let Some(parent) = sub_path.parent() {
        if parent.ends_with("target") {
            target_dir = Some(parent);
            break;
        }
        sub_path = parent;
    }
    let target_dir = target_dir.with_context(|| {
        format!(
            "Could not find `target` dir in parents of {}",
            out_dir.as_ref().display()
        )
    })?;
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
