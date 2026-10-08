// Upstream: react-aria/test/dnd/useClipboard.test.js @ 99e6102368
use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{BrowserTest, async_trait, thirtyfour::WebDriver};
use rootcause::Report;
use serde_json::Value;

use crate::pages::{BaseActions, Page};

const LOG: &str = "test-clipboard-log";

/// `use_clipboard`: cut, copy and paste while the element has focus, with synthesized
/// `ClipboardEvent`s carrying a `DataTransfer`. Fixture: `/hooks/clipboard`.
pub struct ClipboardTests {}

#[async_trait]
impl BrowserTest<str> for ClipboardTests {
    fn name(&self) -> Cow<'_, str> {
        "clipboard_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        copies(&page).await?;
        copies_only_when_focused(&page).await?;
        no_copy_without_items(&page).await?;
        cuts(&page).await?;
        cuts_only_when_focused(&page).await?;
        no_cut_without_items(&page).await?;
        no_cut_without_on_cut(&page).await?;
        pastes(&page).await?;
        pastes_only_when_focused(&page).await?;
        no_paste_without_on_paste(&page).await?;
        custom_types(&page).await?;
        multiple_items_of_a_custom_type(&page).await?;
        items_of_multiple_types(&page).await?;
        multiple_items_of_multiple_types(&page).await?;
        the_action_of_a_cut(&page).await?;
        the_action_of_a_copy(&page).await?;
        page.expect_no_page_errors().await
    }
}

/// What dispatching a clipboard event did.
#[derive(Debug)]
struct Fired {
    /// Whether its default was prevented (the browser's menu item enabled).
    prevented: bool,
    /// The clipboard data's (type, data) pairs afterwards.
    items: Vec<(String, String)>,
}

/// Dispatches the clipboard event `kind` at the "Copy" button, with the page's clipboard data
/// (`window.__clipboard`; `reset` starts with an empty one, `data` is added first).
async fn fire(
    page: &Page<'_>,
    kind: &str,
    reset: bool,
    data: &[(&str, &str)],
) -> Result<Fired, Report> {
    let data: Vec<Value> = data
        .iter()
        .map(|(t, d)| Value::Array(vec![(*t).into(), (*d).into()]))
        .collect();
    let result = page
        .driver
        .execute(
            "const [kind, reset, data] = arguments;
             if (reset || !window.__clipboard) { window.__clipboard = new DataTransfer(); }
             const clipboardData = window.__clipboard;
             for (const [type, value] of data) { clipboardData.items.add(value, type); }
             const button = [...document.querySelectorAll('[role=button]')].find(b => b.textContent.trim() === 'Copy');
             const event = new ClipboardEvent(kind, {clipboardData, bubbles: true, cancelable: true});
             button.dispatchEvent(event);
             return {
                 prevented: event.defaultPrevented,
                 items: [...clipboardData.items].filter(i => i.kind === 'string').map(i => [i.type, clipboardData.getData(i.type)]),
             };",
            vec![kind.into(), reset.into(), Value::Array(data)],
        )
        .await?;
    let json = result.json();
    Ok(Fired {
        prevented: json["prevented"].as_bool().unwrap_or(false),
        items: json["items"]
            .as_array()
            .map(|items| {
                items
                    .iter()
                    .map(|i| {
                        (
                            i[0].as_str().unwrap_or_default().to_owned(),
                            i[1].as_str().unwrap_or_default().to_owned(),
                        )
                    })
                    .collect()
            })
            .unwrap_or_default(),
    })
}

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
        page.by_role_and_text("button", "Before")
            .await?
            .click()
            .await?;
        page.press_tab().await?;
        page.wait_for_active_text("Copy").await?;
    }
    Ok(())
}

async fn expect_log(page: &Page<'_>, expected: &[&str]) -> Result<(), Report> {
    let expected: Vec<String> = expected.iter().map(|e| (*e).to_owned()).collect();
    wait_for!("the log", expected.clone(), log(page).await?);
    // Nothing more is logged.
    stays!("the log", expected, log(page).await?);
    Ok(())
}

async fn log(page: &Page<'_>) -> Result<Vec<String>, Report> {
    let entries = page
        .driver
        .execute(
            "return [...document.getElementById(arguments[0]).querySelectorAll('li')].map(li => li.textContent);",
            vec![LOG.into()],
        )
        .await?;
    Ok(entries
        .json()
        .as_array()
        .map(|e| {
            e.iter()
                .map(|e| e.as_str().unwrap_or_default().to_owned())
                .collect()
        })
        .unwrap_or_default())
}

/// "should copy items to the clipboard".
async fn copies(page: &Page<'_>) -> Result<(), Report> {
    open(page, "", true).await?;
    assert_that!(fire(page, "beforecopy", true, &[]).await?.prevented).is_true();
    let copied = fire(page, "copy", true, &[]).await?;
    assert_that!(copied.items).is_equal_to(pairs(&[("text/plain", "hello world")]));
    expect_log(page, &["copy"]).await
}

/// "should only enable copying when focused".
async fn copies_only_when_focused(page: &Page<'_>) -> Result<(), Report> {
    open(page, "", false).await?;
    assert_that!(fire(page, "beforecopy", true, &[]).await?.prevented).is_false();
    assert_that!(fire(page, "copy", true, &[]).await?.items).is_empty();
    expect_log(page, &[]).await
}

/// "should not enable copying when there is no getItems option".
async fn no_copy_without_items(page: &Page<'_>) -> Result<(), Report> {
    open(page, "?items=none", true).await?;
    assert_that!(fire(page, "beforecopy", true, &[]).await?.prevented).is_false();
    assert_that!(fire(page, "copy", true, &[]).await?.items).is_empty();
    expect_log(page, &[]).await
}

/// "should cut items to the clipboard".
async fn cuts(page: &Page<'_>) -> Result<(), Report> {
    open(page, "?cut", true).await?;
    assert_that!(fire(page, "beforecut", true, &[]).await?.prevented).is_true();
    let cut = fire(page, "cut", true, &[]).await?;
    assert_that!(cut.items).is_equal_to(pairs(&[("text/plain", "hello world")]));
    expect_log(page, &["cut"]).await
}

/// "should only enable cutting when focused".
async fn cuts_only_when_focused(page: &Page<'_>) -> Result<(), Report> {
    open(page, "?cut", false).await?;
    assert_that!(fire(page, "beforecut", true, &[]).await?.prevented).is_false();
    assert_that!(fire(page, "cut", true, &[]).await?.items).is_empty();
    expect_log(page, &[]).await
}

/// "should not enable cutting when there is no getItems option".
async fn no_cut_without_items(page: &Page<'_>) -> Result<(), Report> {
    open(page, "?cut&items=none", true).await?;
    assert_that!(fire(page, "beforecut", true, &[]).await?.prevented).is_false();
    assert_that!(fire(page, "cut", true, &[]).await?.items).is_empty();
    expect_log(page, &[]).await
}

/// "should not enable cutting when there is no onCut option".
async fn no_cut_without_on_cut(page: &Page<'_>) -> Result<(), Report> {
    open(page, "", true).await?;
    assert_that!(fire(page, "beforecut", true, &[]).await?.prevented).is_false();
    assert_that!(fire(page, "cut", true, &[]).await?.items).is_empty();
    expect_log(page, &[]).await
}

/// "should paste items from the clipboard".
async fn pastes(page: &Page<'_>) -> Result<(), Report> {
    open(page, "?paste", true).await?;
    assert_that!(fire(page, "beforepaste", true, &[]).await?.prevented).is_true();
    fire(page, "paste", true, &[("text/plain", "hello world")]).await?;
    expect_log(page, &["paste text/plain=hello world"]).await
}

/// "should only enable pasting when focused".
async fn pastes_only_when_focused(page: &Page<'_>) -> Result<(), Report> {
    open(page, "?paste", false).await?;
    assert_that!(fire(page, "beforepaste", true, &[]).await?.prevented).is_false();
    fire(page, "paste", true, &[("text/plain", "hello world")]).await?;
    expect_log(page, &[]).await
}

/// "should not enable pasting when there is no onPaste option".
async fn no_paste_without_on_paste(page: &Page<'_>) -> Result<(), Report> {
    open(page, "", true).await?;
    assert_that!(fire(page, "beforepaste", true, &[]).await?.prevented).is_false();
    fire(page, "paste", true, &[("text/plain", "hello world")]).await?;
    expect_log(page, &[]).await
}

/// Copies, then pastes the same clipboard data.
async fn copy_and_paste(page: &Page<'_>, query: &str) -> Result<Vec<(String, String)>, Report> {
    open(page, query, true).await?;
    let copied = fire(page, "copy", true, &[]).await?.items;
    fire(page, "paste", false, &[]).await?;
    Ok(copied)
}

/// "data: should work with custom data types".
async fn custom_types(page: &Page<'_>) -> Result<(), Report> {
    let copied = copy_and_paste(page, "?items=custom&paste").await?;
    assert_that!(copied).is_equal_to(pairs(&[("test", "test data")]));
    expect_log(page, &["copy", "paste test=test data"]).await
}

/// "should work with multiple items of the same custom type": only the first item's data under
/// its type, all items as JSON.
async fn multiple_items_of_a_custom_type(page: &Page<'_>) -> Result<(), Report> {
    let copied = copy_and_paste(page, "?items=multiple&paste").await?;
    assert_that!(copied).is_equal_to(pairs(&[
        ("test", "item 1"),
        (
            "application/vnd.react-aria.items+json",
            r#"[{"test":"item 1"},{"test":"item 2"}]"#,
        ),
    ]));
    expect_log(page, &["copy", "paste test=item 1 | test=item 2"]).await
}

/// "should work with items of multiple types".
async fn items_of_multiple_types(page: &Page<'_>) -> Result<(), Report> {
    let copied = copy_and_paste(page, "?items=types&paste").await?;
    assert_that!(copied).is_equal_to(pairs(&[
        ("test", "test data"),
        ("text/plain", "test data"),
        (
            "application/vnd.react-aria.items+json",
            r#"[{"test":"test data","text/plain":"test data"}]"#,
        ),
    ]));
    expect_log(page, &["copy", "paste test=test data text/plain=test data"]).await
}

/// "should work with multiple items of multiple types": native types join the items' data.
async fn multiple_items_of_multiple_types(page: &Page<'_>) -> Result<(), Report> {
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
    .await
}

/// "should show the action type of the clipboard event if cutting".
async fn the_action_of_a_cut(page: &Page<'_>) -> Result<(), Report> {
    open(page, "?items=action&cut", true).await?;
    let cut = fire(page, "cut", true, &[]).await?;
    assert_that!(cut.items).is_equal_to(pairs(&[("cut", "test data")]));
    expect_log(page, &["cut"]).await
}

/// "should show the action type of the clipboard event if copying".
async fn the_action_of_a_copy(page: &Page<'_>) -> Result<(), Report> {
    open(page, "?items=action", true).await?;
    let copied = fire(page, "copy", true, &[]).await?;
    assert_that!(copied.items).is_equal_to(pairs(&[("copy", "test data")]));
    expect_log(page, &["copy"]).await
}
