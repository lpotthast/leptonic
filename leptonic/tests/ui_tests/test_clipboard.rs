// Upstream: react-aria/test/dnd/useClipboard.test.js @ 99e6102368
//! `use_clipboard`: cut, copy and paste while the element has focus, with synthesized
//! `ClipboardEvent`s carrying a `DataTransfer`. Fixture: `/hooks/clipboard`.
use assertr::prelude::*;
use browser_test::thirtyfour::prelude::*;
use rootcause::Report;
use serde::Deserialize;

use crate::{
    pages::{Page, PageActions, role},
    polling::{expect, wait_for},
};

/// What dispatching a clipboard event did.
#[derive(Debug, Deserialize)]
struct Fired {
    /// Whether its default was prevented (the browser's menu item enabled).
    prevented: bool,
    /// The clipboard data's (type, data) pairs afterwards.
    items: Vec<(String, String)>,
}

/// Dispatches the clipboard event `kind` at the "Copy" button, with the page's clipboard data
/// (`window.__clipboard`; `reset` starts with an empty one, `data` is added first). A script: a
/// `ClipboardEvent` carries a `DataTransfer`, which no event init from WebDriver can.
async fn fire(
    page: &Page<'_>,
    kind: &str,
    reset: bool,
    data: &[(&str, &str)],
) -> Result<Fired, Report> {
    let button = page.element(role("button").text("Copy")).await?;
    page.eval(
        "const [button, kind, reset, data] = arguments;
         if (reset || !window.__clipboard) { window.__clipboard = new DataTransfer(); }
         const clipboardData = window.__clipboard;
         for (const [type, value] of data) { clipboardData.items.add(value, type); }
         const event = new ClipboardEvent(kind, {clipboardData, bubbles: true, cancelable: true});
         button.dispatchEvent(event);
         return {
             prevented: event.defaultPrevented,
             items: [...clipboardData.items]
                 .filter(i => i.kind === 'string')
                 .map(i => [i.type, clipboardData.getData(i.type)]),
         };",
        vec![
            button.to_json()?,
            kind.into(),
            reset.into(),
            serde_json::to_value(data)?,
        ],
    )
    .await
}

/// `(type, data)` pairs as owned strings.
fn pairs(expected: &[(&str, &str)]) -> Vec<(String, String)> {
    expected
        .iter()
        .map(|(t, d)| ((*t).to_owned(), (*d).to_owned()))
        .collect()
}

/// Opens the fixture with `query` and, with `focus`, focuses the "Copy" button.
async fn open(page: &Page<'_>, query: &str, focus: bool) -> Result<(), Report> {
    page.goto_path(&format!("/hooks/clipboard{query}")).await?;
    if focus {
        page.element(role("button").text("Before"))
            .await?
            .click()
            .await?;
        page.send_keys(Key::Tab).await?;
        page.wait_for_focus(&page.element(role("button").text("Copy")).await?)
            .await?;
    }
    Ok(())
}

/// Waits until the log is `expected`, and checks that nothing more is logged.
async fn expect_log(page: &Page<'_>, expected: &[&str]) -> Result<(), Report> {
    wait_for("the clipboard log")
        .observing(|| page.inner_texts("#test-clipboard-log li"))
        .to_be_equal_to(expected)
        .await?;
    expect("the clipboard log")
        .observing(|| page.inner_texts("#test-clipboard-log li"))
        .to_stay_equal_to(expected)
        .await?;
    Ok(())
}

/// "should copy items to the clipboard".
pub async fn copies(page: &Page<'_>) -> Result<(), Report> {
    open(page, "", true).await?;
    assert_that!(fire(page, "beforecopy", true, &[]).await?.prevented).is_true();
    let copied = fire(page, "copy", true, &[]).await?;
    assert_that!(copied.items).is_equal_to(pairs(&[("text/plain", "hello world")]));
    expect_log(page, &["copy"]).await?;
    Ok(())
}

/// "should only enable copying when focused".
pub async fn copies_only_when_focused(page: &Page<'_>) -> Result<(), Report> {
    open(page, "", false).await?;
    assert_that!(fire(page, "beforecopy", true, &[]).await?.prevented).is_false();
    assert_that!(fire(page, "copy", true, &[]).await?.items).is_empty();
    expect_log(page, &[]).await?;
    Ok(())
}

/// "should not enable copying when there is no getItems option".
pub async fn no_copy_without_items(page: &Page<'_>) -> Result<(), Report> {
    open(page, "?items=none", true).await?;
    assert_that!(fire(page, "beforecopy", true, &[]).await?.prevented).is_false();
    assert_that!(fire(page, "copy", true, &[]).await?.items).is_empty();
    expect_log(page, &[]).await?;
    Ok(())
}

/// "should cut items to the clipboard".
pub async fn cuts(page: &Page<'_>) -> Result<(), Report> {
    open(page, "?cut", true).await?;
    assert_that!(fire(page, "beforecut", true, &[]).await?.prevented).is_true();
    let cut = fire(page, "cut", true, &[]).await?;
    assert_that!(cut.items).is_equal_to(pairs(&[("text/plain", "hello world")]));
    expect_log(page, &["cut"]).await?;
    Ok(())
}

/// "should only enable cutting when focused".
pub async fn cuts_only_when_focused(page: &Page<'_>) -> Result<(), Report> {
    open(page, "?cut", false).await?;
    assert_that!(fire(page, "beforecut", true, &[]).await?.prevented).is_false();
    assert_that!(fire(page, "cut", true, &[]).await?.items).is_empty();
    expect_log(page, &[]).await?;
    Ok(())
}

/// "should not enable cutting when there is no getItems option".
pub async fn no_cut_without_items(page: &Page<'_>) -> Result<(), Report> {
    open(page, "?cut&items=none", true).await?;
    assert_that!(fire(page, "beforecut", true, &[]).await?.prevented).is_false();
    assert_that!(fire(page, "cut", true, &[]).await?.items).is_empty();
    expect_log(page, &[]).await?;
    Ok(())
}

/// "should not enable cutting when there is no onCut option".
pub async fn no_cut_without_on_cut(page: &Page<'_>) -> Result<(), Report> {
    open(page, "", true).await?;
    assert_that!(fire(page, "beforecut", true, &[]).await?.prevented).is_false();
    assert_that!(fire(page, "cut", true, &[]).await?.items).is_empty();
    expect_log(page, &[]).await?;
    Ok(())
}

/// "should paste items from the clipboard".
pub async fn pastes(page: &Page<'_>) -> Result<(), Report> {
    open(page, "?paste", true).await?;
    assert_that!(fire(page, "beforepaste", true, &[]).await?.prevented).is_true();
    fire(page, "paste", true, &[("text/plain", "hello world")]).await?;
    expect_log(page, &["paste text/plain=hello world"]).await?;
    Ok(())
}

/// "should only enable pasting when focused".
pub async fn pastes_only_when_focused(page: &Page<'_>) -> Result<(), Report> {
    open(page, "?paste", false).await?;
    assert_that!(fire(page, "beforepaste", true, &[]).await?.prevented).is_false();
    fire(page, "paste", true, &[("text/plain", "hello world")]).await?;
    expect_log(page, &[]).await?;
    Ok(())
}

/// "should not enable pasting when there is no onPaste option".
pub async fn no_paste_without_on_paste(page: &Page<'_>) -> Result<(), Report> {
    open(page, "", true).await?;
    assert_that!(fire(page, "beforepaste", true, &[]).await?.prevented).is_false();
    fire(page, "paste", true, &[("text/plain", "hello world")]).await?;
    expect_log(page, &[]).await?;
    Ok(())
}

/// Copies, then pastes the same clipboard data.
async fn copy_and_paste(page: &Page<'_>, query: &str) -> Result<Vec<(String, String)>, Report> {
    open(page, query, true).await?;
    let copied = fire(page, "copy", true, &[]).await?.items;
    fire(page, "paste", false, &[]).await?;
    Ok(copied)
}

/// "data: should work with custom data types".
pub async fn custom_types(page: &Page<'_>) -> Result<(), Report> {
    let copied = copy_and_paste(page, "?items=custom&paste").await?;
    assert_that!(copied).is_equal_to(pairs(&[("test", "test data")]));
    expect_log(page, &["copy", "paste test=test data"]).await?;
    Ok(())
}

/// "should work with multiple items of the same custom type": only the first item's data under
/// its type, all items as JSON.
pub async fn multiple_items_of_a_custom_type(page: &Page<'_>) -> Result<(), Report> {
    let copied = copy_and_paste(page, "?items=multiple&paste").await?;
    assert_that!(copied).is_equal_to(pairs(&[
        ("test", "item 1"),
        (
            "application/vnd.react-aria.items+json",
            r#"[{"test":"item 1"},{"test":"item 2"}]"#,
        ),
    ]));
    expect_log(page, &["copy", "paste test=item 1 | test=item 2"]).await?;
    Ok(())
}

/// "should work with items of multiple types".
pub async fn items_of_multiple_types(page: &Page<'_>) -> Result<(), Report> {
    let copied = copy_and_paste(page, "?items=types&paste").await?;
    assert_that!(copied).is_equal_to(pairs(&[
        ("test", "test data"),
        ("text/plain", "test data"),
        (
            "application/vnd.react-aria.items+json",
            r#"[{"test":"test data","text/plain":"test data"}]"#,
        ),
    ]));
    expect_log(page, &["copy", "paste test=test data text/plain=test data"]).await?;
    Ok(())
}

/// "should work with multiple items of multiple types": native types join the items' data.
pub async fn multiple_items_of_multiple_types(page: &Page<'_>) -> Result<(), Report> {
    let copied = copy_and_paste(page, "?items=multiple-types&paste").await?;
    assert_that!(copied).is_equal_to(pairs(&[
        ("test", "item 1"),
        ("text/plain", "item 1\nitem 2"),
        (
            "application/vnd.react-aria.items+json",
            r#"[{"test":"item 1","text/plain":"item 1"},{"test":"item 2","text/plain":"item 2"}]"#,
        ),
    ]));
    expect_log(
        page,
        &[
            "copy",
            "paste test=item 1 text/plain=item 1 | test=item 2 text/plain=item 2",
        ],
    )
    .await?;
    Ok(())
}

/// "should show the action type of the clipboard event if cutting".
pub async fn the_action_of_a_cut(page: &Page<'_>) -> Result<(), Report> {
    open(page, "?items=action&cut", true).await?;
    let cut = fire(page, "cut", true, &[]).await?;
    assert_that!(cut.items).is_equal_to(pairs(&[("cut", "test data")]));
    expect_log(page, &["cut"]).await?;
    Ok(())
}

/// "should show the action type of the clipboard event if copying".
pub async fn the_action_of_a_copy(page: &Page<'_>) -> Result<(), Report> {
    open(page, "?items=action", true).await?;
    let copied = fire(page, "copy", true, &[]).await?;
    assert_that!(copied.items).is_equal_to(pairs(&[("copy", "test data")]));
    expect_log(page, &["copy"]).await?;
    Ok(())
}
