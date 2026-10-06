use std::{
    path::{Path, PathBuf},
    str::FromStr,
    sync::LazyLock,
};

use anyhow::{Context, Result};
use cargo_toml::{Manifest, Value};

static ENABLE_LOGGING: LazyLock<bool> = LazyLock::new(|| {
    option_env!("LEPTONIC_BUILD_ENABLE_LOGGING")
        .and_then(|v| str::parse::<bool>(v).ok())
        .unwrap_or(false)
});

static MIN_LOG_LEVEL: LazyLock<Level> = LazyLock::new(|| {
    option_env!("LEPTONIC_BUILD_MIN_LOG_LEVEL")
        .and_then(|v| str::parse::<Level>(v).ok())
        .unwrap_or(Level::Debug)
});

#[derive(Debug)]
struct LeptonicMetadata {
    relative_style_dir: String,
}

#[allow(clippy::unwrap_used)]
pub fn main() -> Result<()> {
    // Do nothing when building documentation.
    if cfg!(doc) {
        return Ok(());
    }

    println!("cargo:rerun-if-env-changed={APP_DIR_ENV}");
    let Some((root_dir, metadata)) = find_app()? else {
        return Ok(());
    };

    log(Level::Debug, format!("root_dir is: {}", root_dir.display()));

    let cargo_lock_path = root_dir.join("Cargo.lock");
    let cargo_toml_path = root_dir.join("Cargo.toml");

    println!("cargo:rerun-if-changed={}", cargo_lock_path.display());
    println!("cargo:rerun-if-changed={}", cargo_toml_path.display());

    let style_dir = root_dir.join(&metadata.relative_style_dir);

    let theme_dir = style_dir.join("leptonic");
    leptonic_theme::generate(&theme_dir).unwrap();
    log(
        Level::Info,
        format!("theme written to {}", theme_dir.display()),
    );

    Ok(())
}

/// Parse the Cargo.toml file! Abort if the Cargo.toml has no config.
fn read_leptonic_metadata(cargo_toml_path: &Path) -> Result<Option<LeptonicMetadata>> {
    let cargo_toml: Manifest<Value> = Manifest::from_path_with_metadata(cargo_toml_path)
        .with_context(|| {
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

    let meta = if let Some(metadata) = leptonic_metadata {
        // Found "leptonic" in either package or workspace metadata, proceed
        log(
            Level::Info,
            format!("Found 'leptonic' in metadata of package or workspace: {metadata:?}"),
        );
        metadata
    } else {
        log(
            Level::Debug,
            "Aborting. Cargo.toml in root dir does not contain a package or workspace or is missing the necessary metadata.",
        );
        return Ok(None);
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

    log(
        Level::Debug,
        format!("relative_style_dir is: {relative_style_dir:?}"),
    );

    Ok(Some(LeptonicMetadata { relative_style_dir }))
}

/// Environment variable naming the consuming app's root directory (the one whose `Cargo.toml` declares
/// `[package.metadata.leptonic]` or `[workspace.metadata.leptonic]`). Only needed when the target directory lies
/// outside the app, e.g. with a `CARGO_TARGET_DIR` elsewhere.
const APP_DIR_ENV: &str = "LEPTONIC_APP_DIR";

/// Finds the consuming app and its leptonic metadata: the directory given by `LEPTONIC_APP_DIR`, else the nearest
/// ancestor of `OUT_DIR` whose `Cargo.toml` declares leptonic metadata. `None` when no app declares any (e.g. when
/// building leptonic itself), in which case nothing is generated.
fn find_app() -> Result<Option<(PathBuf, LeptonicMetadata)>> {
    if let Some(app_dir) = std::env::var_os(APP_DIR_ENV) {
        let app_dir = PathBuf::from(app_dir);
        let cargo_toml_path = app_dir.join("Cargo.toml");
        let metadata = read_leptonic_metadata(&cargo_toml_path)?.with_context(|| {
            format!(
                "{APP_DIR_ENV} points to '{}', whose Cargo.toml declares no leptonic metadata.",
                app_dir.display()
            )
        })?;
        return Ok(Some((app_dir, metadata)));
    }

    let out_dir = PathBuf::from(std::env::var("OUT_DIR").context("Could not read 'OUT_DIR'.")?);
    log(Level::Debug, format!("out_dir is: {}", out_dir.display()));
    for dir in out_dir.ancestors().skip(1) {
        let cargo_toml_path = dir.join("Cargo.toml");
        if !cargo_toml_path.is_file() {
            continue;
        }
        if let Some(metadata) = read_leptonic_metadata(&cargo_toml_path)? {
            return Ok(Some((dir.to_path_buf(), metadata)));
        }
    }
    log(
        Level::Debug,
        format!(
            "No Cargo.toml with leptonic metadata above {}.",
            out_dir.display()
        ),
    );
    Ok(None)
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
