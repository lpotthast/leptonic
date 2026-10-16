// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_virtualizer_state` (`/perf/hooks/use_virtualizer_state`): the
//! state of a collection of 10 or 10,000 items in rows estimated at 25px, driven through its
//! methods (no scroll view): mounting and unmounting it, the first layout (the view's size),
//! moving the visible rectangle (a row, a page, to the middle and the end), starting and ending
//! to scroll, an item's measured size, its queries (an item's layout info, the item at a point),
//! persisted keys, new layout options and collection changes. Checked against its budget
//! (`tests/budgets/hooks/use_virtualizer_state.json`).
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test};
use rootcause::Report;

use crate::{harness::fixture_test, pages::Page};

const PATH: &str = "/perf/hooks/use_virtualizer_state";
const STATE: &str = "#perf-state";

/// Registers the performance tests of `use_virtualizer_state`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmountSmall))
        .with(fixture_test(MountAndUnmountLarge))
        .with(fixture_test(LayOutAndScrollSmall))
        .with(fixture_test(LayOutAndScrollLarge))
        .with(fixture_test(StartAndEndScrolling))
        .with(fixture_test(ItemMeasured))
        .with(fixture_test(LayoutInfoOfTheLastItem))
        .with(fixture_test(KeyAtAPoint))
        .with(fixture_test(PersistedKeys))
        .with(fixture_test(LayoutOptions))
        .with(fixture_test(AppendAndRemove))
}

/// Opens the fixture with the state of `count` items (10 or 10,000, built before it mounts).
async fn open(page: &Page<'_>, count: u32) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    if count != 10 {
        page.control("unmount").await?;
        page.control("large").await?;
        page.wait_for_attr("#perf-count", "data-count", Some(&count.to_string()))
            .await?;
        page.control("mount").await?;
    }
    page.element(STATE).await?;
    Ok(())
}

/// Opens the fixture with `count` items, laid out in a 100 × 100 px view at the top.
async fn open_laid_out(page: &Page<'_>, count: u32) -> Result<(), Report> {
    open(page, count).await?;
    page.control("set-size").await?;
    wait_for_height(page, count * 25).await
}

/// Waits until the content is `height` px high.
async fn wait_for_height(page: &Page<'_>, height: u32) -> Result<(), Report> {
    page.wait_for_attr(STATE, "data-height", Some(&height.to_string()))
        .await
}

/// How many layout infos the state renders.
async fn visible(page: &Page<'_>) -> Result<u32, Report> {
    Ok(page
        .element(STATE)
        .await?
        .attr("data-visible")
        .await?
        .unwrap_or_default()
        .parse()?)
}

/// Mounting and unmounting `count` items (10 or 10,000).
async fn mount_and_unmount(page: &Page<'_>, count: u32) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(STATE, 0).await?;
    if count != 10 {
        page.control("large").await?;
        page.wait_for_attr("#perf-count", "data-count", Some(&count.to_string()))
            .await?;
    }
    page.measure("mount", page.control("mount")).await?;
    page.wait_for_attr(STATE, "data-visible", Some("0")).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(STATE, 0).await?;
    Ok(())
}

/// Mounting and unmounting the state of 10 items (no size yet: nothing visible).
#[browser_test]
pub async fn mount_and_unmount_small(page: &Page<'_>) -> Result<(), Report> {
    mount_and_unmount(page, 10).await
}

/// Mounting and unmounting the state of 10,000 items (no size yet: nothing visible).
#[browser_test]
pub async fn mount_and_unmount_large(page: &Page<'_>) -> Result<(), Report> {
    mount_and_unmount(page, 10_000).await
}

/// The first layout (a 100 × 100 px view), then the visible rectangle moving down by a row, by
/// a page, to the middle (10,000 items) and to the end of `count` items.
async fn lay_out_and_scroll(page: &Page<'_>, count: u32) -> Result<(), Report> {
    open(page, count).await?;
    page.measure("set size", page.control("set-size")).await?;
    wait_for_height(page, count * 25).await?;
    assert_that!(visible(page).await?).is_greater_than(0);
    page.measure("scroll by a row", page.control("scroll-by-a-row"))
        .await?;
    assert_that!(page.control("top").await?).is_equal_to(25.0);
    page.measure("scroll by a page", page.control("scroll-by-a-page"))
        .await?;
    assert_that!(page.control("top").await?).is_equal_to(125.0);
    // 10 items: the middle is where the page ended.
    if count > 10 {
        page.measure("scroll to the middle", page.control("scroll-to-the-middle"))
            .await?;
        assert_that!(page.control("top").await?).is_equal_to(f64::from(count * 25 / 2));
    }
    page.measure("scroll to the end", page.control("scroll-to-the-end"))
        .await?;
    assert_that!(page.control("top").await?).is_equal_to(f64::from(count * 25 - 100));
    assert_that!(visible(page).await?).is_greater_than(0);
    Ok(())
}

/// The first layout and scrolling down, 10 items.
#[browser_test]
pub async fn lay_out_and_scroll_small(page: &Page<'_>) -> Result<(), Report> {
    lay_out_and_scroll(page, 10).await
}

/// The first layout and scrolling down, 10,000 items.
#[browser_test]
pub async fn lay_out_and_scroll_large(page: &Page<'_>) -> Result<(), Report> {
    lay_out_and_scroll(page, 10_000).await
}

/// Scrolling starting and ending (10,000 items laid out): nothing to lay out again.
#[browser_test]
pub async fn start_and_end_scrolling(page: &Page<'_>) -> Result<(), Report> {
    open_laid_out(page, 10_000).await?;
    let before = visible(page).await?;
    page.measure("start", page.control("start-scrolling"))
        .await?;
    page.measure("end", page.control("end-scrolling")).await?;
    assert_that!(visible(page).await?).is_equal_to(before);
    Ok(())
}

/// The first of 10,000 items measured at 40px (estimated at 25px), and at 25px again.
#[browser_test]
pub async fn item_measured(page: &Page<'_>) -> Result<(), Report> {
    open_laid_out(page, 10_000).await?;
    page.measure("measured", page.control("measure-first-item"))
        .await?;
    wait_for_height(page, 250_015).await?;
    page.measure("measured again", page.control("measure-first-item-again"))
        .await?;
    wait_for_height(page, 250_000).await?;
    Ok(())
}

/// The layout info of the last of 10,000 items (laid out up to it).
#[browser_test]
pub async fn layout_info_of_the_last_item(page: &Page<'_>) -> Result<(), Report> {
    open_laid_out(page, 10_000).await?;
    let top = page
        .measure(
            "layout info of the last",
            page.control("layout-info-of-the-last"),
        )
        .await?;
    assert_that!(top).is_equal_to(249_975.0);
    Ok(())
}

/// The item at a point: in the middle of 10,000 items.
#[browser_test]
pub async fn key_at_a_point(page: &Page<'_>) -> Result<(), Report> {
    open_laid_out(page, 10_000).await?;
    let index = page
        .measure("key at the middle", page.control("key-at-the-middle"))
        .await?;
    assert_that!(index).is_equal_to(5_000.0);
    Ok(())
}

/// Persisting the last of 10,000 items (rendered while not visible), and no item again.
#[browser_test]
pub async fn persisted_keys(page: &Page<'_>) -> Result<(), Report> {
    open_laid_out(page, 10_000).await?;
    let before = visible(page).await?;
    page.measure("persist the last", page.control("persist-last"))
        .await?;
    assert_that!(visible(page).await?).is_equal_to(before + 1);
    page.measure("persist none", page.control("persist-none"))
        .await?;
    assert_that!(visible(page).await?).is_equal_to(before);
    Ok(())
}

/// The content height (px).
async fn height(page: &Page<'_>) -> Result<u32, Report> {
    Ok(page
        .element(STATE)
        .await?
        .attr("data-height")
        .await?
        .unwrap_or_default()
        .parse()?)
}

/// New layout options (rows estimated at 40px) for 10,000 items, and the first ones again. The
/// rows laid out before keep their earlier estimates (25px): the content grows to nearly
/// 400,000px.
#[browser_test]
pub async fn layout_options(page: &Page<'_>) -> Result<(), Report> {
    open_laid_out(page, 10_000).await?;
    page.measure("taller rows", page.control("taller-rows"))
        .await?;
    assert_that!(height(page).await?).is_greater_than(399_000);
    page.measure("default rows", page.control("default-rows"))
        .await?;
    assert_that!(height(page).await?).is_less_than(251_000);
    Ok(())
}

/// Appending 10 items to 10,000 and removing them again.
#[browser_test]
pub async fn append_and_remove(page: &Page<'_>) -> Result<(), Report> {
    open_laid_out(page, 10_000).await?;
    page.measure("append", page.control("append")).await?;
    wait_for_height(page, 250_250).await?;
    page.measure("remove", page.control("remove")).await?;
    wait_for_height(page, 250_000).await?;
    Ok(())
}
