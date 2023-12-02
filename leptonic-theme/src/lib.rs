use anyhow::{Context, Result};
use include_dir::{include_dir, Dir};
use indoc::indoc;
use std::{io::Write, path::Path};

static SCSS_DIR: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/scss");

/// Path must point to a folder which can be deleted and recreated freely!
///
/// # Errors
///
/// Will return `Err` if `path` can not be deleted or created or if
/// files cannot be successfully copied or written.
pub fn generate(path: impl AsRef<Path>) -> Result<()> {
    let path = path.as_ref();

    if path.exists() {
        std::fs::remove_dir_all(path)
            .with_context(|| format!("Could not remove path '{path:?}'"))?;
    }
    std::fs::create_dir_all(path).with_context(|| format!("Could not create path '{path:?}'"))?;

    SCSS_DIR
        .extract(path)
        .with_context(|| format!("Could not extract theme into '{path:?}'"))?;

    let themes_file_path = path.join("leptonic-themes.scss");
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .append(false)
        .truncate(true)
        .open(&themes_file_path)
        .context("Could not find leptonic-themes.scss after copying comp-time created SCSS_DIR. This must be a bug.")?;

    file.write_all(
        indoc!(
            r#"
            @import "./themes/builder";
            @import "./themes/light";
            @import "./themes/dark";
            "#
        )
        .as_bytes(),
    )
    .with_context(|| format!("Could not write to '{}'", themes_file_path.display()))?;

    Ok(())
}
