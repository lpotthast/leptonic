use std::{io::Write, path::PathBuf};

use include_dir::{include_dir, Dir};

static SCSS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/scss");

/// Path must point to a folder which can be deleted and recreated freely!
pub fn generate(path: PathBuf) {
    if path.exists() {
        std::fs::remove_dir_all(&path).unwrap();
    }
    std::fs::create_dir_all(&path).unwrap();

    SCSS_DIR.extract(&path).unwrap();

    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .append(false)
        .truncate(true)
        .open(&path.join("leptonic-themes.scss"))
        .unwrap();

    file.write_all(
        r#"
        @import "./themes/builder";
        @import "./themes/light";
        @import "./themes/dark";
        "#
        .as_bytes(),
    )
    .unwrap();
}
