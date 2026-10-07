use std::str::FromStr;

use leptonic::{atoms::kbd::Keys as KeyCaps, utils::key::KeyboardKey};
use leptos::{context::Provider, prelude::*};
use leptos_classes::Classes;

use super::Code;

/// A plain documentation table with the given column headers. Rows are [`TableRow`]s of [`TableCell`]s.
///
/// A static `<table>`: leptonic's `Table` atom is an interactive grid (keyboard navigation, selection), which reference
/// tables don't need.
#[component]
pub fn DocTable(headers: &'static [&'static str], children: Children) -> impl IntoView {
    view! {
        <div class="doc-table-container">
            <table class="doc-table">
                <thead>
                    <tr>{headers.iter().map(|header| view! { <th>{*header}</th> }).collect_view()}</tr>
                </thead>
                <tbody>{children()}</tbody>
            </table>
        </div>
    }
}

/// A row of a [`DocTable`].
#[component]
pub fn TableRow(children: Children) -> impl IntoView {
    view! { <tr>{children()}</tr> }
}

/// A cell of a [`TableRow`].
#[component]
pub fn TableCell(#[prop(into, optional)] classes: Classes, children: Children) -> impl IntoView {
    view! { <td class=classes>{children()}</td> }
}

/// What an [`ApiTable`] documents. Decides the columns.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApiKind {
    /// Fields of a hook's input struct: Field, Type, Default, Description.
    Input,
    /// Fields of a hook's return struct: Field, Type, Description.
    Return,
    /// Fields of any other struct, e.g. an event or a context: Field, Type, Description.
    Fields,
    /// Props of an atom or component: Prop, Type, Default, Description.
    Props,
    /// `data-*` attributes an atom renders: Attribute, Values, Description.
    DataAttributes,
}

impl ApiKind {
    const fn headers(self) -> &'static [&'static str] {
        match self {
            Self::Input => &["Field", "Type", "Default", "Description"],
            Self::Return | Self::Fields => &["Field", "Type", "Description"],
            Self::Props => &["Prop", "Type", "Default", "Description"],
            Self::DataAttributes => &["Attribute", "Values", "Description"],
        }
    }

    const fn has_default(self) -> bool {
        matches!(self, Self::Input | Self::Props)
    }
}

/// API reference table. Rows are [`ApiRow`]s.
///
/// `of` names the documented Rust item: the struct of an Input, Return or Fields table, the component of a Props table.
/// It is shown above the table, and a unit test checks that the rows list exactly the item's public fields or props.
#[component]
pub fn ApiTable(
    kind: ApiKind,
    #[prop(optional)] of: Option<&'static str>,
    children: Children,
) -> impl IntoView {
    view! {
        {of.map(|of| view! { <p class="doc-api-of"><Code inline=true>{of}</Code></p> })}
        <DocTable headers=kind.headers()>
            <Provider value=kind>{children()}</Provider>
        </DocTable>
    }
}

/// One entry of an [`ApiTable`]. The children describe it.
///
/// `name` may list several entries sharing a row, separated by `", "` (`"on_focus, on_blur"`). `ty` is the type (or
/// the possible values of a data attribute), written on one line with single spaces (a unit test checks it). `default` is shown in tables with a default column; rows without one show
/// a dash.
#[component]
pub fn ApiRow(
    name: &'static str,
    #[prop(optional)] ty: Option<&'static str>,
    #[prop(optional)] default: Option<&'static str>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let kind = expect_context::<ApiKind>();
    let names = name
        .split(", ")
        .enumerate()
        .map(|(i, name)| {
            view! {
                {(i > 0).then_some(", ")}
                <Code inline=true>{name}</Code>
            }
        })
        .collect_view();

    // Column labels, shown next to the values when narrow screens stack the cells (see `_article.scss`).
    let label = move |column: usize| kind.headers()[column];
    let description_column = kind.headers().len() - 1;

    view! {
        <TableRow>
            <TableCell classes="doc-table-name">{names}</TableCell>
            <TableCell attr:data-label=label(1)><TypeOrDash ty/></TableCell>
            {kind.has_default().then(|| view! { <TableCell attr:data-label=label(2)><CodeOrDash code=default/></TableCell> })}
            <TableCell attr:data-label=label(description_column)>{children.map(|children| children())}</TableCell>
        </TableRow>
    }
}

/// A Rust type, which may wrap after `<`, `, ` and `::` so that long types don't push the description out of view.
#[component]
fn TypeOrDash(ty: Option<&'static str>) -> impl IntoView {
    let Some(ty) = ty else {
        return "\u{2014}".into_any();
    };
    let parts = split_type(ty);
    let last = parts.len() - 1;
    view! {
        <code class="doc-type">
            {parts
                .into_iter()
                .enumerate()
                .map(|(i, part)| view! { {part}{(i < last).then(|| view! { <wbr/> })} })
                .collect_view()}
        </code>
    }
    .into_any()
}

/// Splits a type after each run of `<`, `,`, `:` and spaces: `Option<Callback<Key>>` becomes `Option<`,
/// `Callback<`, `Key>>`.
fn split_type(ty: &str) -> Vec<&str> {
    let mut parts = Vec::new();
    let mut rest = ty;
    while let Some(at) = rest.find(['<', ',', ':']) {
        let end = rest[at..]
            .find(|c: char| !matches!(c, '<' | ',' | ':' | ' '))
            .map_or(rest.len(), |offset| at + offset);
        parts.push(&rest[..end]);
        rest = &rest[end..];
    }
    if !rest.is_empty() || parts.is_empty() {
        parts.push(rest);
    }
    parts
}

#[component]
fn CodeOrDash(code: Option<&'static str>) -> impl IntoView {
    match code {
        Some(code) => view! { <Code inline=true>{code}</Code> }.into_any(),
        None => "\u{2014}".into_any(),
    }
}

/// Keyboard interaction table. Rows are [`KeyRow`]s.
#[component]
pub fn KeyboardTable(children: Children) -> impl IntoView {
    view! {
        <DocTable headers=&["Key", "Action"]>{children()}</DocTable>
    }
}

/// One keyboard interaction. The children describe what happens.
///
/// `keys` is written like [`Keys`]: alternatives separated by `" / "`, for example `"Enter / Space"` or
/// `"Shift + Tab"`.
#[component]
pub fn KeyRow(keys: &'static str, children: Children) -> impl IntoView {
    view! {
        <TableRow>
            <TableCell classes="doc-table-name"><Keys keys/></TableCell>
            <TableCell attr:data-label="Action">{children()}</TableCell>
        </TableRow>
    }
}

/// Keys in prose or tables, rendered as key caps by leptonic's `Keys` atom (`<kbd>`s, styled by `.doc-keys` in
/// `_article.scss`).
///
/// `keys` is written as displayed: alternatives separated by `" / "`, key combinations joined with `" + "`. Keys are
/// [`KeyboardKey`] names (`"ArrowDown"`, `"PageUp"`, `"Control"`, `"Command"`); a few descriptions of key groups
/// (`KEY_DESCRIPTIONS`) are allowed too, as text when alone. A unit test checks the names of all pages.
///
/// The keys are shown as written, on every platform: leptonic's `ShortcutKeys` atom shows a shortcut in the keys of
/// the reader's platform (Command on Apple devices for the primary modifier), while these tables document specific
/// keys ("Control + X / Command + X").
#[component]
pub fn Keys(keys: &'static str) -> impl IntoView {
    keys.split(" / ")
        .enumerate()
        .map(|(i, alternative)| {
            view! {
                {(i > 0).then_some(" / ")}
                <KeyCombination combination=alternative/>
            }
        })
        .collect_view()
}

/// Descriptions of key groups allowed in [`Keys`] next to key names.
#[cfg(test)]
const KEY_DESCRIPTIONS: &[&str] = &[
    "Any character",
    "Letter keys",
    "Arrow keys",
    "0\u{2013}9",
    "A\u{2013}Z",
];

/// A key combination like `"Shift + Tab"`: its keys as key caps in one `<kbd class="doc-keys">`, joined by `+`
/// (leptonic's `Keys` atom). A description of a key group (`"Arrow keys"`) alone is text; in a combination
/// (`"Shift + Arrow keys"`) it is a key cap too.
#[component]
fn KeyCombination(combination: &'static str) -> impl IntoView {
    let keys: Vec<KeyboardKey> = combination
        .split(" + ")
        .map(|name| {
            let Ok(key) = KeyboardKey::from_str(name);
            key
        })
        .collect();
    if let [KeyboardKey::Other(_)] = keys.as_slice() {
        return combination.into_any();
    }
    view! { <KeyCaps keys classes="doc-keys"/> }.into_any()
}

#[cfg(test)]
mod tests {
    use std::{fs, path::Path, str::FromStr};

    use assertr::prelude::*;
    use leptonic::utils::key::KeyboardKey;

    use super::{KEY_DESCRIPTIONS, split_type};

    /// The `keys` of every `KeyRow` in `dir`, with `\u{..}` escapes resolved.
    fn key_rows(dir: &Path, rows: &mut Vec<String>) {
        for entry in fs::read_dir(dir).expect("pages are readable").flatten() {
            let path = entry.path();
            if path.is_dir() {
                key_rows(&path, rows);
            } else if path.extension().is_some_and(|ext| ext == "rs") {
                let source = fs::read_to_string(&path).expect("page is readable");
                for (start, _) in source.match_indices(" keys=\"") {
                    if !source[..start].ends_with("<KeyRow") && !source[..start].ends_with("<Keys")
                    {
                        continue;
                    }
                    let rest = &source[start + " keys=\"".len()..];
                    let keys = &rest[..rest.find('"').expect("keys are a string literal")];
                    rows.push(keys.replace("\\u{2013}", "\u{2013}"));
                }
            }
        }
    }

    #[test]
    fn keys_are_keyboard_key_names() {
        let mut rows = Vec::new();
        key_rows(
            &Path::new(env!("CARGO_MANIFEST_DIR")).join("src/pages"),
            &mut rows,
        );
        assert_that!(rows.len()).is_greater_than(100);

        let unknown: Vec<&str> = rows
            .iter()
            .flat_map(|keys| {
                keys.split(" / ")
                    .flat_map(|combination| combination.split(" + "))
            })
            .filter(|key| {
                let Ok(parsed) = KeyboardKey::from_str(key);
                matches!(parsed, KeyboardKey::Other(_)) && !KEY_DESCRIPTIONS.contains(key)
            })
            .collect();
        assert_that!(unknown).is_empty();
    }

    #[test]
    fn splits_types_after_opening_brackets_commas_and_paths() {
        assert_that!(split_type("Option<Callback<PressEvent>>")).is_equal_to(vec![
            "Option<",
            "Callback<",
            "PressEvent>>",
        ]);
        assert_that!(split_type("Option<Oco<'static, str>>")).is_equal_to(vec![
            "Option<",
            "Oco<",
            "'static, ",
            "str>>",
        ]);
        assert_that!(split_type("hooks::Key")).is_equal_to(vec!["hooks::", "Key"]);
        assert_that!(split_type("bool")).is_equal_to(vec!["bool"]);
    }
}
