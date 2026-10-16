// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_auto_scroll` (`/perf/hooks/use_auto_scroll`): scrolling a
//! drop target while a native drag nears its edges. Only desktop Safari scrolls (other browsers
//! scroll during native drags themselves), so the tests measure its handle in Chrome (where it
//! ignores the drag's position) and in Chrome posing as desktop Safari (a user agent override:
//! `Page::emulate_platform` has no desktop WebKit). Measured (`Page::measure`) and checked
//! against its budget (`tests/budgets/hooks/use_auto_scroll.json`): mounting and unmounting it,
//! positions it ignores, a position away from the edges, and scrolling for 5 frames near the
//! bottom edge until stopped.
use assertr::prelude::*;
use browser_test::{TestGroup, browser_test};
use rootcause::{Report, prelude::ResultExt};

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_auto_scroll";
const SCROLLING: &str = "#perf-auto-scroll";

/// Registers the performance tests of `use_auto_scroll`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(IgnoredOutsideSafari))
        .with(fixture_test(AwayFromTheEdgesInSafari))
        .with(fixture_test(ScrollNearAnEdgeInSafari))
}

/// Makes the browser report desktop Safari on a Mac from the next page load on (user agent and
/// `navigator.userAgentData`, without Chromium's brands).
async fn pose_as_safari(page: &Page<'_>) -> Result<(), Report> {
    page.low_level()
        .driver()
        .cdp()
        .send_raw(
            "Emulation.setUserAgentOverride",
            serde_json::json!({
                "userAgent": "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.4 Safari/605.1.15",
                "platform": "MacIntel",
                "userAgentMetadata": {
                    "platform": "macOS",
                    "platformVersion": "15.0.0",
                    "architecture": "arm",
                    "model": "",
                    "mobile": false,
                    "brands": [],
                },
            }),
        )
        .await
        .context("failed to pose as Safari")?;
    Ok(())
}

/// Mounting the hook and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(SCROLLING, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(SCROLLING).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(SCROLLING, 0).await?;
    Ok(())
}

/// In Chrome: a position near the bottom edge (ignored: Chrome scrolls itself) and stopping.
#[browser_test]
pub async fn ignored_outside_safari(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let scrolling = page.element(SCROLLING).await?;
    page.measure("near the bottom", page.control("move-near-the-bottom"))
        .await?;
    page.measure("stop", page.control("stop")).await?;
    assert_that!(scrolling.scroll_extent().await?.top).is_equal_to(0.0);
    Ok(())
}

/// In desktop Safari: a position away from the edges (no scrolling).
#[browser_test]
pub async fn away_from_the_edges_in_safari(page: &Page<'_>) -> Result<(), Report> {
    pose_as_safari(page).await?;
    page.goto_path(PATH).await?;
    let scrolling = page.element(SCROLLING).await?;
    page.measure("in the middle", page.control("move-to-the-middle"))
        .await?;
    assert_that!(scrolling.scroll_extent().await?.top).is_equal_to(0.0);
    Ok(())
}

/// In desktop Safari: scrolling for 5 frames near the bottom edge (15px per frame), then stopped.
#[browser_test]
pub async fn scroll_near_an_edge_in_safari(page: &Page<'_>) -> Result<(), Report> {
    pose_as_safari(page).await?;
    page.goto_path(PATH).await?;
    let scrolling = page.element(SCROLLING).await?;
    page.measure("scroll for 5 frames", page.control("scroll-for-frames"))
        .await?;
    assert_that!(scrolling.scroll_extent().await?.top).is_equal_to(75.0);
    Ok(())
}
