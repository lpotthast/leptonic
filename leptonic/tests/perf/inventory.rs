//! What leptonic provides, the subjects of the performance tests: every public hook (a
//! `pub fn use_*` in `src/hooks/`) and every atom (a `#[component] pub fn` in `src/atoms/`), read
//! from the sources. A subject's id is `<kind>/<name>`, its name as `leptonic::metrics` reports
//! it: a hook's (`hooks/use_press`), an atom's in snake case (`atoms/button`,
//! `atoms/virtual_list`). The performance test of a subject is `perf_tests/<id>.rs`.

use std::{
    collections::BTreeSet,
    path::{Path, PathBuf},
    sync::OnceLock,
};

/// The hooks' and atoms' ids (`hooks/use_press`, `atoms/button`).
pub fn subjects() -> &'static BTreeSet<String> {
    static SUBJECTS: OnceLock<BTreeSet<String>> = OnceLock::new();
    SUBJECTS.get_or_init(|| {
        let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        let mut subjects = BTreeSet::new();
        for file in rust_files(&src.join("hooks")) {
            subjects.extend(
                hooks(&read(&file))
                    .iter()
                    .map(|hook| format!("hooks/{hook}")),
            );
        }
        for file in rust_files(&src.join("atoms")) {
            subjects.extend(
                atoms(&read(&file))
                    .iter()
                    .map(|atom| format!("atoms/{}", snake_case(atom))),
            );
        }
        subjects
    })
}

fn read(file: &Path) -> String {
    std::fs::read_to_string(file)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", file.display()))
}

/// The `.rs` files below `dir`.
fn rust_files(dir: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    let entries = std::fs::read_dir(dir)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", dir.display()));
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            files.extend(rust_files(&path));
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            files.push(path);
        }
    }
    files
}

/// The name of the function `line` declares as `pub fn <name>` (not `pub(crate)`).
fn public_fn(line: &str) -> Option<&str> {
    let rest = line.trim_start().strip_prefix("pub fn ")?;
    let end = rest
        .find(|c: char| !(c.is_ascii_alphanumeric() || c == '_'))
        .unwrap_or(rest.len());
    Some(&rest[..end])
}

/// The hooks `source` declares: its `pub fn use_*`.
fn hooks(source: &str) -> Vec<String> {
    source
        .lines()
        .filter_map(public_fn)
        .filter(|name| name.starts_with("use_"))
        .map(str::to_owned)
        .collect()
}

/// The atoms `source` declares: the first `pub fn` after each `#[component]` (attributes and doc
/// comments may come between).
fn atoms(source: &str) -> Vec<String> {
    let mut atoms = Vec::new();
    let mut in_component = false;
    for line in source.lines() {
        if line.trim() == "#[component]" {
            in_component = true;
        } else if in_component && let Some(name) = public_fn(line) {
            atoms.push(name.to_owned());
            in_component = false;
        } else if in_component && line.trim_start().starts_with("fn ") {
            // A private component.
            in_component = false;
        }
    }
    atoms
}

/// `name` in snake case, as `#[component]` names an atom's body function (`VirtualList`:
/// `virtual_list`).
fn snake_case(name: &str) -> String {
    let mut snake = String::with_capacity(name.len() + 4);
    for (i, c) in name.chars().enumerate() {
        if c.is_ascii_uppercase() {
            if i > 0 {
                snake.push('_');
            }
            snake.push(c.to_ascii_lowercase());
        } else {
            snake.push(c);
        }
    }
    snake
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::*;

    #[test]
    fn hooks_are_public_use_functions() {
        let source = "pub fn use_press() {}\npub(crate) fn use_internal() {}\npub fn helper() {}\n    pub fn use_nested<T>(x: T) {}";
        assert_that!(hooks(source))
            .is_equal_to(vec!["use_press".to_owned(), "use_nested".to_owned()]);
    }

    #[test]
    fn atoms_are_public_components() {
        let source = "#[component]\n/// Docs.\n#[allow(unused)]\npub fn Button() {}\n#[component]\nfn Private() {}\npub fn NotAComponent() {}";
        assert_that!(atoms(source)).is_equal_to(vec!["Button".to_owned()]);
    }

    #[test]
    fn atoms_are_snake_cased() {
        assert_that!(snake_case("VirtualList")).is_equal_to("virtual_list".to_owned());
        assert_that!(snake_case("Button")).is_equal_to("button".to_owned());
    }

    #[test]
    fn the_sources_have_hooks_and_atoms() {
        assert_that!(subjects().contains("hooks/use_press")).is_true();
        assert_that!(subjects().contains("atoms/button")).is_true();
        assert_that!(subjects().contains("atoms/virtual_list")).is_true();
    }
}
