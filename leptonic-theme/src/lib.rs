use std::{
    collections::HashSet,
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
};

use anyhow::{Context, Result};
use include_dir::{Dir, DirEntry, include_dir};

static SCSS_DIR: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/scss");

/// Write the leptonic theme (SCSS sources) into `path`.
///
/// `path` must be a directory owned by leptonic: files in it that are not part of the theme are
/// removed.
///
/// This is safe to call concurrently for the same `path`, which happens when cargo-leptos builds
/// the server and the WASM client in parallel and both run leptonic's build script. Every file is
/// written to a temporary sibling and renamed into place, and files that already have the right
/// content are left untouched (so their modification time doesn't trigger needless rebuilds).
///
/// # Errors
///
/// Will return `Err` if `path` can not be created or if files cannot be written.
pub fn generate(path: impl AsRef<Path>) -> Result<()> {
    let path = path.as_ref();

    let mut files: Vec<(PathBuf, &[u8])> = Vec::new();
    collect_files(&SCSS_DIR, &mut files);

    for (relative, content) in &files {
        write_if_changed(&path.join(relative), content)?;
    }

    let expected: HashSet<PathBuf> = files.into_iter().map(|(rel, _)| path.join(rel)).collect();
    remove_stale_files(path, &expected)?;

    Ok(())
}

fn collect_files(dir: &'static Dir<'static>, out: &mut Vec<(PathBuf, &'static [u8])>) {
    for entry in dir.entries() {
        match entry {
            DirEntry::Dir(dir) => collect_files(dir, out),
            DirEntry::File(file) => out.push((file.path().to_path_buf(), file.contents())),
        }
    }
}

fn write_if_changed(target: &Path, content: &[u8]) -> Result<()> {
    if std::fs::read(target).is_ok_and(|existing| existing == content) {
        return Ok(());
    }

    let parent = target
        .parent()
        .with_context(|| format!("'{}' has no parent directory", target.display()))?;
    std::fs::create_dir_all(parent)
        .with_context(|| format!("Could not create path '{}'", parent.display()))?;

    // A unique temporary name per writer keeps concurrent writers from clobbering each other's
    // partially written files. The rename is atomic, so readers always see a complete file.
    let file_name = target
        .file_name()
        .with_context(|| format!("'{}' has no file name", target.display()))?
        .to_string_lossy();
    let tmp = parent.join(format!(
        ".{file_name}.{}.{}.tmp",
        std::process::id(),
        unique_suffix()
    ));
    std::fs::write(&tmp, content)
        .with_context(|| format!("Could not write to '{}'", tmp.display()))?;
    std::fs::rename(&tmp, target).with_context(|| {
        let _ = std::fs::remove_file(&tmp);
        format!(
            "Could not move '{}' to '{}'",
            tmp.display(),
            target.display()
        )
    })?;
    Ok(())
}

/// Unique within this process. Combined with the process id, this gives every writer its own
/// temporary file name.
fn unique_suffix() -> u64 {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    COUNTER.fetch_add(1, Ordering::Relaxed)
}

/// Remove regular files below `dir` that are not part of the generated theme. Temporary files of
/// concurrent writers are left alone.
fn remove_stale_files(dir: &Path, expected: &HashSet<PathBuf>) -> Result<()> {
    let entries = match std::fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(()),
        Err(err) => {
            return Err(err).with_context(|| format!("Could not read '{}'", dir.display()));
        }
    };
    for entry in entries {
        let entry = entry.with_context(|| format!("Could not read '{}'", dir.display()))?;
        let path = entry.path();
        let file_type = entry
            .file_type()
            .with_context(|| format!("Could not stat '{}'", path.display()))?;
        if file_type.is_dir() {
            remove_stale_files(&path, expected)?;
        } else if !expected.contains(&path) && !is_temp_file(&path) {
            match std::fs::remove_file(&path) {
                Ok(()) => {}
                Err(err) if err.kind() == std::io::ErrorKind::NotFound => {}
                Err(err) => {
                    return Err(err)
                        .with_context(|| format!("Could not remove '{}'", path.display()));
                }
            }
        }
    }
    Ok(())
}

fn is_temp_file(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| {
            name.starts_with('.')
                && Path::new(name)
                    .extension()
                    .is_some_and(|ext| ext.eq_ignore_ascii_case("tmp"))
        })
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::*;

    /// The atom theme's entry file.
    const ENTRY_FILE_NAME: &str = "leptonic-atoms.scss";

    fn temp_dir(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "leptonic-theme-test-{name}-{}-{}",
            std::process::id(),
            unique_suffix()
        ));
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn generates_all_files_and_removes_stale_ones() {
        let dir = temp_dir("stale");
        let stale = dir.join("components").join("removed-component.scss");
        std::fs::create_dir_all(stale.parent().unwrap()).unwrap();
        std::fs::write(&stale, "stale").unwrap();

        generate(&dir).unwrap();

        assert_that!(dir.join(ENTRY_FILE_NAME).exists()).is_true();
        assert_that!(stale.exists()).is_false();
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn concurrent_generation_succeeds() {
        let dir = temp_dir("concurrent");
        std::thread::scope(|scope| {
            let handles: Vec<_> = (0..8).map(|_| scope.spawn(|| generate(&dir))).collect();
            for handle in handles {
                assert_that!(handle.join().unwrap()).is_ok();
            }
        });
        assert_that!(std::fs::read_to_string(dir.join(ENTRY_FILE_NAME)).unwrap()).is_equal_to(
            SCSS_DIR
                .get_file(ENTRY_FILE_NAME)
                .unwrap()
                .contents_utf8()
                .unwrap()
                .to_string(),
        );
        std::fs::remove_dir_all(&dir).unwrap();
    }
}
