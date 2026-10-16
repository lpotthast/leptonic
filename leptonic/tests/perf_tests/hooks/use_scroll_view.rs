// No upstream: performance measurements (`documentation/testing.md`, "Performance measurements").
//! The performance of the hook `use_scroll_view` (`/perf/hooks/use_scroll_view`): a 100 × 100 px
//! scroll view of a 2,500px high content box. Measured (`Page::measure`) and checked against its
//! budget (`tests/budgets/hooks/use_scroll_view.json`): mounting and unmounting it, the user
//! scrolling it and the window (each until the scrolling ended, 300ms after the last scroll
//! event), scrolling it programmatically (to a position and to its end), and changes of its
//! content size, its own size, `allows_window_scrolling` and `scroll_direction`.
use assertr::{matchers::eq, prelude::*};
use browser_test::{TestGroup, browser_test, thirtyfour::WebElement};
use rootcause::Report;

use crate::{
    harness::fixture_test,
    pages::{ElementActions, Page},
};

const PATH: &str = "/perf/hooks/use_scroll_view";
const VIEW: &str = "#perf-scroll-view";

/// Registers the performance tests of `use_scroll_view`.
pub fn register(group: TestGroup<str>) -> TestGroup<str> {
    group
        .with(fixture_test(MountAndUnmount))
        .with(fixture_test(UserScroll))
        .with(fixture_test(WindowScroll))
        .with(fixture_test(ScrollTo))
        .with(fixture_test(ScrollToTheEnd))
        .with(fixture_test(ContentSize))
        .with(fixture_test(ViewSize))
        .with(fixture_test(WindowScrolling))
        .with(fixture_test(ScrollDirection))
}

/// Opens the fixture; returns the view once it reported its size.
async fn open(page: &Page<'_>) -> Result<WebElement, Report> {
    page.goto_path(PATH).await?;
    let view = page.element(VIEW).await?;
    assert_that!(|| page.control("size-changes"))
        .eventually_ok()
        .matches(eq(1.0))
        .await;
    Ok(view)
}

/// Waits until `view` scrolls and until its scrolling ended.
async fn until_the_scrolling_ended(view: &WebElement) -> Result<(), Report> {
    view.wait_for_attr("data-scrolling", Some("true")).await?;
    view.wait_for_attr("data-scrolling", None).await
}

/// The view's scroll position.
async fn scroll_top(view: &WebElement) -> Result<f64, Report> {
    Ok(view.scroll_extent().await?.top)
}

/// Mounting the scroll view and unmounting it.
#[browser_test]
pub async fn mount_and_unmount(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.control("unmount").await?;
    page.wait_for_count(VIEW, 0).await?;
    page.measure("mount", page.control("mount")).await?;
    page.element(VIEW).await?;
    page.measure("unmount", page.control("unmount")).await?;
    page.wait_for_count(VIEW, 0).await?;
    Ok(())
}

/// The user scrolling the view by 200px, until the scrolling ended: the visible rectangle
/// follows.
#[browser_test]
pub async fn user_scroll(page: &Page<'_>) -> Result<(), Report> {
    let view = open(page).await?;
    page.measure("scroll", async {
        view.scroll_to_top(200.0).await?;
        until_the_scrolling_ended(&view).await
    })
    .await?;
    assert_that!(page.control("visible-top").await?).is_equal_to(200.0);
    view.wait_for_attr("data-user-scrolling", None).await?;
    Ok(())
}

/// The window scrolling the view partly out of the viewport, until the scrolling ended: the
/// visible rectangle follows.
#[browser_test]
pub async fn window_scroll(page: &Page<'_>) -> Result<(), Report> {
    let view = open(page).await?;
    let top = view.client_rect().await?.top;
    page.measure("scroll", async {
        page.low_level()
            .eval::<()>(
                "window.scrollTo(0, arguments[0]);",
                vec![(top + 50.0).into()],
            )
            .await?;
        until_the_scrolling_ended(&view).await
    })
    .await?;
    // The view's top lies between pixels.
    let visible_top = page.control("visible-top").await?;
    assert_that!(visible_top).is_greater_than(49.0);
    assert_that!(visible_top).is_less_than(51.0);
    Ok(())
}

/// Scrolling the view programmatically to 500px (`scroll_to`): not the user scrolling.
#[browser_test]
pub async fn scroll_to(page: &Page<'_>) -> Result<(), Report> {
    let view = open(page).await?;
    page.measure("scroll to", page.control("scroll-to")).await?;
    assert_that!(scroll_top(&view).await?).is_equal_to(500.0);
    assert_that!(page.control("visible-top").await?).is_equal_to(500.0);
    view.wait_for_attr("data-scrolling", None).await?;
    Ok(())
}

/// Scrolling the view programmatically to its end (`scroll_to_end`).
#[browser_test]
pub async fn scroll_to_the_end(page: &Page<'_>) -> Result<(), Report> {
    let view = open(page).await?;
    page.measure("scroll to the end", page.control("scroll-to-the-end"))
        .await?;
    assert_that!(view.scroll_extent().await?.distance_to_end()).is_equal_to(0.0);
    Ok(())
}

/// The content growing to 5,000px and shrinking back to 2,500px.
#[browser_test]
pub async fn content_size(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    let content = page.element("#perf-scroll-content").await?;
    page.measure("grow", page.control("grow-content")).await?;
    assert_that!(content.style_property("height").await?).is_equal_to("5000px".to_owned());
    page.measure("shrink", page.control("shrink-content"))
        .await?;
    assert_that!(content.style_property("height").await?).is_equal_to("2500px".to_owned());
    Ok(())
}

/// The view growing to 200px and shrinking back to 100px: each reports its new size.
#[browser_test]
pub async fn view_size(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.measure("grow", page.control("taller-view")).await?;
    assert_that!(page.control("size-changes").await?).is_equal_to(2.0);
    page.measure("shrink", page.control("default-view")).await?;
    assert_that!(page.control("size-changes").await?).is_equal_to(3.0);
    Ok(())
}

/// Turning `allows_window_scrolling` off and on again.
#[browser_test]
pub async fn window_scrolling(page: &Page<'_>) -> Result<(), Report> {
    open(page).await?;
    page.measure("off", page.control("window-scrolling-off"))
        .await?;
    page.measure("on", page.control("window-scrolling-on"))
        .await?;
    assert_that!(page.control("visible-top").await?).is_equal_to(0.0);
    Ok(())
}

/// Changing `scroll_direction` to vertical (no horizontal overflow) and back to both.
#[browser_test]
pub async fn scroll_direction(page: &Page<'_>) -> Result<(), Report> {
    let view = open(page).await?;
    page.measure("vertical", page.control("vertical")).await?;
    assert_that!(view.style_property("overflow-x").await?).is_equal_to("hidden".to_owned());
    page.measure("both", page.control("both")).await?;
    assert_that!(view.style_property("overflow-x").await?).is_equal_to("auto".to_owned());
    Ok(())
}
