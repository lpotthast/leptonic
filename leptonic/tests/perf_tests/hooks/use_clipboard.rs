// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_clipboard` (`/perf/hooks/use_clipboard`, on a focusable
//! `<div role="button">`): what each thing it offers costs, measured (`Page::measure`) and
//! checked against its budget (`tests/budgets/hooks/use_clipboard.json`): mounting and
//! unmounting it, focus and blur (it listens to the document's clipboard events while focused),
//! copying, cutting and pasting (clipboard events a script dispatches), and disabling it (a copy
//! it ignores).
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{harness::fixture_test, pages::Page};

const PATH: &str = "/perf/hooks/use_clipboard";

/// Registers the performance tests of `use_clipboard`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(FocusAndBlur))
        .with(fixture_test(CopyCutAndPaste))
        .with(fixture_test(DisableAndEnable))
}
const CLIPBOARD: &str = "#perf-clipboard";

/// Dispatches the clipboard event `kind` (`copy`, `cut`, `paste`) on the element, with a
/// clipboard holding `pasted` (as `text/plain`, if any); returns the text it holds afterwards.
async fn fire(page: &Page<'_>, kind: &str, pasted: Option<&str>) -> Result<String, Report> {
    let element = page.element(CLIPBOARD).await?;
    page.low_level()
        .eval(
            "const [element, kind, pasted] = arguments;
             const clipboardData = new DataTransfer();
             if (pasted !== null) clipboardData.items.add(pasted, 'text/plain');
             element.dispatchEvent(
                 new ClipboardEvent(kind, {clipboardData, bubbles: true, cancelable: true}));
             return clipboardData.getData('text/plain');",
            vec![element.to_json()?, kind.into(), pasted.into()],
        )
        .await
}

/// The counts of copies, cuts and pastes.
async fn counts(page: &Page<'_>) -> Result<(f64, f64, f64), Report> {
    Ok((
        page.control("copies").await?,
        page.control("cuts").await?,
        page.control("pastes").await?,
    ))
}

/// Focuses the element with the keyboard (Tab from the plain button before it).
async fn focus_with_tab(page: &Page<'_>) -> Result<(), Report> {
    page.element("#perf-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&page.element(CLIPBOARD).await?).await?;
    Ok(())
}

/// Mounting the element with its clipboard handling and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(CLIPBOARD, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(CLIPBOARD).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(CLIPBOARD, 0).await?;
    Ok(())
}

/// Tab moving focus to the element (its listeners start) and back (they stop).
#[browser_test]
pub async fn focus_and_blur(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#perf-before").await?.click().await?;
    let clipboard = page.element(CLIPBOARD).await?;
    page.measure("focus", page.send_keys(Key::Tab)).await?;
    page.wait_for_focus(&clipboard).await?;
    page.measure("blur", page.send_keys(Key::Shift + Key::Tab))
        .await?;
    page.wait_for_focus(&page.element("#perf-before").await?)
        .await?;
    Ok(())
}

/// A copy and a cut (each puts the item on the clipboard) and a paste, the element focused.
#[browser_test]
pub async fn copy_cut_and_paste(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    focus_with_tab(page).await?;
    let copied = page.measure("copy", fire(page, "copy", None)).await?;
    assert_that!(copied).is_equal_to("hello world".to_owned());
    let cut = page.measure("cut", fire(page, "cut", None)).await?;
    assert_that!(cut).is_equal_to("hello world".to_owned());
    page.measure("paste", fire(page, "paste", Some("pasted")))
        .await?;
    assert_that!(counts(page).await?).is_equal_to((1.0, 1.0, 1.0));
    Ok(())
}

/// Disabling the clipboard handling, a copy it ignores, and enabling it again.
#[browser_test]
pub async fn disable_and_enable(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    focus_with_tab(page).await?;
    page.measure("disable", page.control("disable")).await?;
    let copied = page
        .measure("ignored copy", fire(page, "copy", None))
        .await?;
    assert_that!(copied).is_equal_to(String::new());
    page.measure("enable", page.control("enable")).await?;
    fire(page, "copy", None).await?;
    assert_that!(counts(page).await?).is_equal_to((1.0, 0.0, 0.0));
    Ok(())
}
