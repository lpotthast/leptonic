// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the atom `Virtualizer` (`/perf/atoms/virtualizer`), around a `ListBox` of
//! 10 or 10,000 options in 25px rows, 100px high: what each thing it offers costs, measured
//! (`Page::measure`) and checked against its budget (`tests/budgets/atoms/virtualizer.json`):
//! mounting and unmounting it, scrolling (by a row, a page, to the middle and the end),
//! keyboard navigation to options that aren't rendered (with the focused option kept rendered),
//! and changes of its row size, its collection and its visibility.
//!
//! A scroll keeps the view "scrolling" until 300ms after its last scroll event: the scroll
//! measurements include that end; the measurements of keys that scroll end before it (the test
//! waits for it before the next one).
use assertr::{matchers::eq, prelude::*};
use browser_test::{TestGroup, browser_test, thirtyfour::prelude::*};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page, css},
};

const PATH: &str = "/perf/atoms/virtualizer";
const LISTBOX: &str = "#perf-virtualizer [role=listbox]";
/// The content box inside the list box (the scroll view), around the rendered options.
const CONTENT: &str = "#perf-virtualizer [role=listbox] > [role=presentation]";

/// Registers the performance tests of `Virtualizer`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmountSmall))
        .with(fixture_test(MountAndUnmountLarge))
        .with(fixture_test(ScrollSmall))
        .with(fixture_test(ScrollLarge))
        .with(fixture_test(KeyboardNavigationSmall))
        .with(fixture_test(KeyboardNavigationLarge))
        .with(fixture_test(ScrollAwayFromTheFocusedOption))
        .with(fixture_test(RowSize))
        .with(fixture_test(AppendAndRemove))
        .with(fixture_test(HideAndShow))
}

/// The rendered option `text`.
async fn option(page: &Page<'_>, text: &str) -> Result<WebElement, Report> {
    page.element(css("#perf-virtualizer [role=option]").text(text))
        .await
}

/// Opens the fixture with `count` options (10 or 10,000, built before the list mounts) and
/// waits until its first option is rendered.
async fn open(page: &Page<'_>, count: usize) -> Result<WebElement, Report> {
    page.goto_path(PATH).await?;
    if count != 10 {
        page.control("unmount").await?;
        page.control("large").await?;
        page.wait_for_attr("#perf-count", "data-count", Some(&count.to_string()))
            .await?;
        page.control("mount").await?;
    }
    option(page, "Item 0").await?;
    page.element(LISTBOX).await
}

/// Waits until the view no longer counts as scrolling (its content takes pointer events again).
async fn wait_for_scroll_end(page: &Page<'_>) -> Result<(), Report> {
    let content = page.element(CONTENT).await?;
    assert_that!(|| content.style_property("pointer-events"))
        .eventually_ok()
        .matches(eq("auto".to_owned()))
        .await;
    Ok(())
}

/// Scrolls `list` to `top` and waits in the page, frame by frame, until the scrolling started
/// (the content stops taking pointer events) and ended (300ms after the last scroll event). Seen
/// from the test, polling would miss a scrolling state shorter than a slow round trip.
const SCROLL_UNTIL_IT_ENDS: &str = "const [list, top, content, done] = arguments;
    const start = performance.now();
    let scrolled = false;
    const check = () => {
        const events = getComputedStyle(content).pointerEvents;
        if (events === 'none') scrolled = true;
        if (scrolled && events === 'auto') done(true);
        else if (performance.now() - start > 5000) done(false);
        else requestAnimationFrame(check);
    };
    list.scrollTop = top;
    requestAnimationFrame(check);";

/// Scrolls `list` to `top`, until the scrolling ended (300ms after the scroll event).
async fn scroll_until_it_ends(page: &Page<'_>, list: &WebElement, top: f64) -> Result<(), Report> {
    let content = page.element(CONTENT).await?;
    let ended: bool = page
        .low_level()
        .eval_async(
            SCROLL_UNTIL_IT_ENDS,
            vec![list.to_json()?, top.into(), content.to_json()?],
        )
        .await?;
    if !ended {
        rootcause::bail!("scrolling the list to {top} didn't start and end within 5s");
    }
    Ok(())
}

/// Mounting and unmounting the list (10 options).
#[browser_test]
pub async fn mount_and_unmount_small(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(LISTBOX, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    option(page, "Item 3").await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(LISTBOX, 0).await?;
    Ok(())
}

/// Mounting and unmounting the list (10,000 options): only the visible ones render.
#[browser_test]
pub async fn mount_and_unmount_large(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(LISTBOX, 0).await?;
    page.control("large").await?;
    page.wait_for_attr("#perf-count", "data-count", Some("10000"))
        .await?;
    page.measure("mount", page.control("mount")).await?;
    let first = option(page, "Item 0").await?;
    assert_that!(first)
        .has_attribute("aria-setsize")
        .await
        .is_equal_to("10000");
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(LISTBOX, 0).await?;
    Ok(())
}

/// Scrolling the list of 10 options by a page (to its end).
#[browser_test]
pub async fn scroll_small(page: &Page<'_>) -> Result<(), Report> {
    let list = open(page, 10).await?;
    page.measure("scroll by a page", scroll_until_it_ends(page, &list, 100.0))
        .await?;
    option(page, "Item 9").await?;
    Ok(())
}

/// Scrolling the list of 10,000 options down: by a row, by a page, to the middle and to the
/// end (each until the scrolling ended).
#[browser_test]
pub async fn scroll_large(page: &Page<'_>) -> Result<(), Report> {
    let list = open(page, 10_000).await?;
    page.measure("scroll by a row", scroll_until_it_ends(page, &list, 25.0))
        .await?;
    option(page, "Item 4").await?;
    page.measure("scroll by a page", scroll_until_it_ends(page, &list, 125.0))
        .await?;
    option(page, "Item 8").await?;
    page.measure(
        "scroll to the middle",
        scroll_until_it_ends(page, &list, 125_000.0),
    )
    .await?;
    option(page, "Item 5000").await?;
    page.measure(
        "scroll to the end",
        scroll_until_it_ends(page, &list, 249_900.0),
    )
    .await?;
    option(page, "Item 9999").await?;
    Ok(())
}

/// The focused option's position in the list (`aria-posinset`).
async fn focused_position(page: &Page<'_>) -> Result<u32, Report> {
    let focused = page.focused_element().await?;
    match focused.attr("aria-posinset").await? {
        Some(position) => Ok(position.parse()?),
        None => rootcause::bail!("the focused element is no option"),
    }
}

/// Keyboard navigation in a list of `count` options: Tab focuses the first option, ArrowDown the
/// next, PageDown one a page further, End the last (rendering it), Home the first again. Each
/// step that scrolls is measured until the page is idle; the test waits for the scrolling to end
/// before the next.
async fn keyboard_navigation(page: &Page<'_>, count: usize) -> Result<(), Report> {
    open(page, count).await?;
    page.element("#perf-before").await?.click().await?;
    page.measure("focus", page.send_keys(Key::Tab)).await?;
    page.wait_for_focus(&option(page, "Item 0").await?).await?;
    page.measure("arrow down", page.send_keys(Key::Down))
        .await?;
    page.wait_for_focus(&option(page, "Item 1").await?).await?;
    wait_for_scroll_end(page).await?;
    page.measure("page down", page.send_keys(Key::PageDown))
        .await?;
    assert_that!(focused_position(page).await?).is_greater_than(2);
    wait_for_scroll_end(page).await?;
    page.measure("end", page.send_keys(Key::End)).await?;
    let last = format!("Item {}", count - 1);
    page.wait_for_focus(&option(page, &last).await?).await?;
    wait_for_scroll_end(page).await?;
    page.measure("home", page.send_keys(Key::Home)).await?;
    page.wait_for_focus(&option(page, "Item 0").await?).await?;
    wait_for_scroll_end(page).await?;
    Ok(())
}

/// Keyboard navigation in the list of 10 options.
#[browser_test]
pub async fn keyboard_navigation_small(page: &Page<'_>) -> Result<(), Report> {
    keyboard_navigation(page, 10).await
}

/// Keyboard navigation in the list of 10,000 options: End renders the last option, Home the
/// first again.
#[browser_test]
pub async fn keyboard_navigation_large(page: &Page<'_>) -> Result<(), Report> {
    keyboard_navigation(page, 10_000).await
}

/// Scrolling the list of 10,000 options to its middle while its first option has focus: the
/// focused option stays rendered.
#[browser_test]
pub async fn scroll_away_from_the_focused_option(page: &Page<'_>) -> Result<(), Report> {
    let list = open(page, 10_000).await?;
    page.element("#perf-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    let first = option(page, "Item 0").await?;
    page.wait_for_focus(&first).await?;
    page.measure(
        "scroll to the middle",
        scroll_until_it_ends(page, &list, 125_000.0),
    )
    .await?;
    option(page, "Item 5000").await?;
    page.wait_for_count(css("#perf-virtualizer [role=option]").text("Item 0"), 1)
        .await?;
    Ok(())
}

/// Changing the row size of the list of 10,000 options (`layout_options`: 40px rows) and back.
#[browser_test]
pub async fn row_size(page: &Page<'_>) -> Result<(), Report> {
    open(page, 10_000).await?;
    let content = page.element(CONTENT).await?;
    page.measure("taller rows", page.control("taller-rows"))
        .await?;
    assert_that!(content.style_property("height").await?).is_equal_to("400000px".to_owned());
    page.measure("default rows", page.control("default-rows"))
        .await?;
    assert_that!(content.style_property("height").await?).is_equal_to("250000px".to_owned());
    Ok(())
}

/// Appending 10 options to the list of 10,000 options and removing them again.
#[browser_test]
pub async fn append_and_remove(page: &Page<'_>) -> Result<(), Report> {
    open(page, 10_000).await?;
    page.measure("append", page.control("append")).await?;
    page.wait_for_attr(
        css("#perf-virtualizer [role=option]").text("Item 0"),
        "aria-setsize",
        Some("10010"),
    )
    .await?;
    page.measure("remove", page.control("remove")).await?;
    page.wait_for_attr(
        css("#perf-virtualizer [role=option]").text("Item 0"),
        "aria-setsize",
        Some("10000"),
    )
    .await?;
    Ok(())
}

/// Hiding the list of 10 options (`display: none` on its container) and showing it again: it
/// renders its options again.
#[browser_test]
pub async fn hide_and_show(page: &Page<'_>) -> Result<(), Report> {
    open(page, 10).await?;
    let list = page.element(LISTBOX).await?;
    page.measure("hide", page.control("hide")).await?;
    assert_that!(list.is_displayed().await?).is_false();
    page.measure("show", page.control("show")).await?;
    option(page, "Item 3").await?;
    assert_that!(list.is_displayed().await?).is_true();
    Ok(())
}
