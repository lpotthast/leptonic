//! Keeps the API tables of the pages in sync with the library: every Input, Return, Fields and Props `ApiTable` names
//! the documented Rust item with `of` and must list exactly the item's public fields (Input, Return and Fields tables,
//! for structs) or props (Props tables, for `#[component]` functions), each with the type of the field or prop as its
//! `ty` (compared by the last path segments, so `Arc` matches `std::sync::Arc`; a type parameter of the item may be
//! described freely: `L: Layout`). `of` is the item's name, qualified with the end of its module path where the name
//! alone is ambiguous (`of="atoms::button::Button"`).

use std::{
    collections::{BTreeMap, BTreeSet, HashMap},
    fs,
    path::{Path, PathBuf},
};

use assertr::prelude::*;
use syn::{
    FnArg, GenericArgument, Item, Pat, Path as TypePath, PathArguments, ReturnType, Token, Type,
    TypeParamBound, Visibility, parse::Parser, punctuated::Punctuated,
};

/// The fields or props of one item: name → type, as `type_name` prints it (`None` for a type parameter of the item,
/// which a page describes in its own words: `L: Layout`, `Fn(T) -> impl IntoView`).
type Members = BTreeMap<String, Option<String>>;

/// The documented items of the library: public struct fields and component props, by module path
/// (`atoms::button::Button`).
#[derive(Default)]
struct LibraryItems {
    structs: HashMap<String, Members>,
    components: HashMap<String, Members>,
}

/// Looks up `of` in `items`: a plain name (`Button`) must be unique, a qualified one (`atoms::button::Button`) matches
/// the end of the module path.
fn lookup<'a>(items: &'a HashMap<String, Members>, of: &str) -> Result<&'a Members, Vec<&'a str>> {
    let candidates: Vec<_> = items
        .iter()
        .filter(|(path, _)| *path == of || path.ends_with(&format!("::{of}")))
        .collect();
    match candidates.as_slice() {
        [(_, item)] => Ok(item),
        _ => Err(candidates.iter().map(|(path, _)| path.as_str()).collect()),
    }
}

impl LibraryItems {
    fn read(dir: &Path) -> Self {
        let mut items = Self::default();
        for file in rust_files(dir) {
            let source = fs::read_to_string(&file).expect("library source is readable");
            let parsed = syn::parse_file(&source)
                .unwrap_or_else(|err| panic!("{} parses: {err}", file.display()));
            // `hooks/button/use_button.rs` is the module `hooks::button::use_button`; `mod.rs` and `lib.rs` are their
            // directory's module.
            let module = file
                .strip_prefix(dir)
                .expect("files are below the library directory")
                .with_extension("")
                .components()
                .map(|part| part.as_os_str().to_string_lossy().into_owned())
                .filter(|part| part != "mod" && part != "lib")
                .collect::<Vec<_>>()
                .join("::");
            items.collect(&module, &parsed.items);
        }
        items
    }

    fn collect(&mut self, module: &str, items: &[Item]) {
        let path = |name: &dyn ToString| {
            if module.is_empty() {
                name.to_string()
            } else {
                format!("{module}::{}", name.to_string())
            }
        };
        for item in items {
            match item {
                Item::Struct(item) if matches!(item.vis, Visibility::Public(_)) => {
                    let generics = type_parameters(&item.generics);
                    let fields =
                        item.fields
                            .iter()
                            .filter(|field| matches!(field.vis, Visibility::Public(_)))
                            .filter_map(|field| {
                                field.ident.as_ref().map(|ident| {
                                    (ident.to_string(), member_type(&field.ty, &generics))
                                })
                            })
                            .collect();
                    self.structs.insert(path(&item.ident), fields);
                }
                Item::Fn(item)
                    if item
                        .attrs
                        .iter()
                        .any(|attr| attr.path().is_ident("component")) =>
                {
                    let generics = type_parameters(&item.sig.generics);
                    let props = item
                        .sig
                        .inputs
                        .iter()
                        .filter_map(|input| match input {
                            FnArg::Typed(arg) => match arg.pat.as_ref() {
                                Pat::Ident(pat) => {
                                    Some((pat.ident.to_string(), member_type(&arg.ty, &generics)))
                                }
                                _ => None,
                            },
                            FnArg::Receiver(_) => None,
                        })
                        .collect();
                    self.components.insert(path(&item.sig.ident), props);
                }
                Item::Mod(item) => {
                    if let Some((_, items)) = &item.content {
                        self.collect(&path(&item.ident), items);
                    }
                }
                _ => {}
            }
        }
    }
}

/// An `ApiTable` as written in a page source.
struct DocumentedTable {
    page: PathBuf,
    kind: String,
    /// The documented item (`of`), if the table names one.
    of: Option<String>,
    /// Row name → the row's `ty`, if it has one, and the name's position in the row (`name="on_focus, on_blur"` with
    /// `ty="Option<Callback<FocusEvent>>"` documents both names with one type, `name="classes, styles"` with
    /// `ty="Classes, Styles"` one type per name).
    rows: BTreeMap<String, Option<(String, usize)>>,
}

/// The API tables of all page sources below `dir`.
fn documented_tables(dir: &Path) -> Vec<DocumentedTable> {
    let mut tables = Vec::new();
    for page in rust_files(dir) {
        let source = fs::read_to_string(&page).expect("page source is readable");
        for table in tags(&source, "ApiTable") {
            let header_end = table.find('>').expect("`<ApiTable` tag is closed");
            let header = &table[..header_end];
            let body = &table[header_end
                ..table
                    .find("</ApiTable>")
                    .expect("`<ApiTable>` has a closing tag")];
            let kind = header
                .split("kind=ApiKind::")
                .nth(1)
                .and_then(|kind| kind.split_whitespace().next())
                .expect("`ApiTable` has a kind")
                .to_owned();
            let rows = tags(body, "ApiRow")
                .filter_map(|row| Some((attribute(row, "name")?, attribute(row, "ty"))))
                .flat_map(|(names, ty)| {
                    names.split(", ").enumerate().map(move |(position, name)| {
                        (name.to_owned(), ty.map(|ty| (ty.to_owned(), position)))
                    })
                })
                .collect();
            tables.push(DocumentedTable {
                page: page.clone(),
                kind,
                of: attribute(header, "of").map(str::to_owned),
                rows,
            });
        }
    }
    tables
}

/// The source of every `<name` element in `source`, each running until the next one (whitespace may follow the name,
/// including line breaks).
fn tags<'a>(source: &'a str, name: &str) -> impl Iterator<Item = &'a str> {
    let open = format!("<{name}");
    let starts: Vec<usize> = source
        .match_indices(&open)
        .map(|(start, _)| start)
        .filter(|start| source[start + open.len()..].starts_with(char::is_whitespace))
        .collect();
    let ends = starts
        .iter()
        .skip(1)
        .copied()
        .chain(std::iter::once(source.len()));
    starts
        .iter()
        .copied()
        .zip(ends)
        .map(move |(start, end)| &source[start..end])
        .collect::<Vec<_>>()
        .into_iter()
}

/// The value of the attribute `name="..."` in `tag`.
fn attribute<'a>(tag: &'a str, name: &str) -> Option<&'a str> {
    let pattern = format!("{name}=\"");
    let start = tag
        .match_indices(&pattern)
        .map(|(start, _)| start)
        .find(|&start| start > 0 && tag[..start].ends_with(char::is_whitespace))?
        + pattern.len();
    let len = tag[start..].find('"')?;
    Some(&tag[start..start + len])
}

fn type_parameters(generics: &syn::Generics) -> Vec<String> {
    generics
        .type_params()
        .map(|param| param.ident.to_string())
        .collect()
}

fn member_type(ty: &Type, type_parameters: &[String]) -> Option<String> {
    let name = type_name(ty);
    (!type_parameters.contains(&name)).then_some(name)
}

/// `ty` with paths shortened to their last segment (`Option<Arc<dyn KeyboardDelegate>>` for
/// `Option<std::sync::Arc<dyn KeyboardDelegate>>`), the form pages write types in. Library inputs and props use only
/// paths, references, tuples, trait objects and `impl Trait`; other types print as `?`.
fn type_name(ty: &Type) -> String {
    let list = |types: Vec<String>| types.join(", ");
    let path = |path: &TypePath| {
        let Some(segment) = path.segments.last() else {
            return String::new();
        };
        let arguments = match &segment.arguments {
            PathArguments::None => String::new(),
            PathArguments::AngleBracketed(arguments) => format!(
                "<{}>",
                list(
                    arguments
                        .args
                        .iter()
                        .filter_map(|argument| match argument {
                            GenericArgument::Type(ty) => Some(type_name(ty)),
                            _ => None,
                        })
                        .collect()
                )
            ),
            PathArguments::Parenthesized(arguments) => format!(
                "({}){}",
                list(
                    arguments
                        .inputs
                        .iter()
                        .map(|arg| type_name(&arg.ty))
                        .collect()
                ),
                match &arguments.output {
                    ReturnType::Default => String::new(),
                    ReturnType::Type(_, ty) => format!(" -> {}", type_name(ty)),
                }
            ),
        };
        format!("{}{arguments}", segment.ident)
    };
    let bounds = |bounds: &Punctuated<TypeParamBound, Token![+]>| {
        bounds
            .iter()
            .filter_map(|bound| match bound {
                TypeParamBound::Trait(bound) => Some(path(&bound.path)),
                _ => None,
            })
            .collect::<Vec<_>>()
            .join(" + ")
    };
    match ty {
        Type::Path(ty) => path(&ty.path),
        Type::Reference(ty) => format!(
            "&{}{}",
            if ty.mutability.is_some() { "mut " } else { "" },
            type_name(&ty.elem)
        ),
        Type::Tuple(ty) => format!("({})", list(ty.elems.iter().map(type_name).collect())),
        Type::TraitObject(ty) => format!("dyn {}", bounds(&ty.bounds)),
        Type::ImplTrait(ty) => format!("impl {}", bounds(&ty.bounds)),
        Type::Paren(ty) => type_name(&ty.elem),
        _ => "?".to_owned(),
    }
}

fn rust_files(dir: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    let mut dirs = vec![dir.to_path_buf()];
    while let Some(dir) = dirs.pop() {
        for entry in
            fs::read_dir(&dir).unwrap_or_else(|err| panic!("{} is readable: {err}", dir.display()))
        {
            let path = entry.expect("directory entry is readable").path();
            if path.is_dir() {
                dirs.push(path);
            } else if path.extension().is_some_and(|ext| ext == "rs") {
                files.push(path);
            }
        }
    }
    files.sort();
    files
}

/// Where the library sources are: next to the book in the repository (override with `LEPTONIC_SRC`).
fn library_dir() -> PathBuf {
    std::env::var_os("LEPTONIC_SRC").map_or_else(
        || Path::new(env!("CARGO_MANIFEST_DIR")).join("../../leptonic/src"),
        PathBuf::from,
    )
}

#[test]
fn api_tables_match_the_library() {
    let library = LibraryItems::read(&library_dir());
    let pages = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/pages");

    let mut problems = Vec::new();
    for table in documented_tables(&pages) {
        let page = table
            .page
            .strip_prefix(&pages)
            .unwrap_or(&table.page)
            .display()
            .to_string();
        let (items, what) = match table.kind.as_str() {
            "Props" => (&library.components, "component"),
            "Input" | "Return" | "Fields" => (&library.structs, "struct"),
            other => {
                if table.of.is_some() {
                    problems.push(format!("{page}: `of` is not supported on {other} tables"));
                }
                continue;
            }
        };
        let Some(of) = &table.of else {
            problems.push(format!("{page}: {} table without `of`", table.kind));
            continue;
        };
        let actual = match lookup(items, of) {
            Ok(actual) => actual,
            Err(candidates) if candidates.is_empty() => {
                problems.push(format!("{page}: no {what} `{of}` in the library"));
                continue;
            }
            Err(candidates) => {
                problems.push(format!(
                    "{page}: `{of}` is ambiguous, qualify it: {candidates:?}"
                ));
                continue;
            }
        };
        let actual: BTreeSet<_> = actual.keys().collect();
        let documented: BTreeSet<_> = table.rows.keys().collect();
        let missing: Vec<_> = actual.difference(&documented).collect();
        let unknown: Vec<_> = documented.difference(&actual).collect();
        if !missing.is_empty() || !unknown.is_empty() {
            problems.push(format!(
                "{page} `{of}`: undocumented {missing:?}, not in the library {unknown:?}"
            ));
        }
    }

    assert_that!(problems)
        .with_detail_message("API tables out of sync with the library")
        .is_empty();
}

/// The `ty` of every row of an Input, Return, Fields or Props table is the type of the field or prop it documents.
#[test]
fn api_types_match_the_library() {
    let library = LibraryItems::read(&library_dir());
    let pages = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/pages");

    let mut problems = Vec::new();
    for table in documented_tables(&pages) {
        let items = match table.kind.as_str() {
            "Props" => &library.components,
            "Input" | "Return" | "Fields" => &library.structs,
            _ => continue,
        };
        let Some(Ok(actual)) = table.of.as_deref().map(|of| lookup(items, of)) else {
            continue; // Reported by `api_tables_match_the_library`.
        };
        let page = table
            .page
            .strip_prefix(&pages)
            .unwrap_or(&table.page)
            .display()
            .to_string();
        let of = table.of.as_deref().unwrap_or_default();
        for (name, documented) in &table.rows {
            let (Some((ty, position)), Some(Some(actual))) = (documented, actual.get(name)) else {
                continue;
            };
            let Ok(types) = Punctuated::<Type, Token![,]>::parse_terminated.parse_str(ty) else {
                problems.push(format!("{page} `{of}`.{name}: `{ty}` is not a Rust type"));
                continue;
            };
            let documented = types
                .iter()
                .nth(if types.len() == 1 { 0 } else { *position })
                .map(type_name);
            if documented.as_ref() != Some(actual) {
                problems.push(format!(
                    "{page} `{of}`.{name}: documented `{ty}`, is `{actual}`"
                ));
            }
        }
    }

    assert_that!(problems)
        .with_detail_message("API table types out of sync with the library")
        .is_empty();
}

/// The `ty` of every `ApiRow` is written on one line with single spaces: a line break or indentation in the string
/// literal would show up in the table.
#[test]
fn api_row_types_have_no_stray_whitespace() {
    let pages = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/pages");
    let mut problems = Vec::new();
    for page in rust_files(&pages) {
        let source = fs::read_to_string(&page).expect("page source is readable");
        for row in tags(&source, "ApiRow") {
            let Some(ty) = attribute(row, "ty") else {
                continue;
            };
            if ty.contains(|c: char| c.is_whitespace() && c != ' ')
                || ty.contains("  ")
                || ty.trim() != ty
            {
                problems.push(format!(
                    "{}: `ty=\"{ty}\"`",
                    page.strip_prefix(&pages).unwrap_or(&page).display()
                ));
            }
        }
    }
    assert_that!(problems)
        .with_detail_message("`ApiRow` types with line breaks, indentation or double spaces")
        .is_empty();
}
