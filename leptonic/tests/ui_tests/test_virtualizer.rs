// Upstream: react-aria-components/test/ListBox.test.js @ 99e6102368
//! A virtualized `ListBox` ("should support virtualizer"): only the visible options (plus
//! overscan) render, each telling its position and the set size; scrolling renders others; End
//! reaches and renders the last option. A log anchored to the end stays at the end when lines are
//! appended, with measured variable heights (rows don't overlap once measured). A list box next to
//! a `Virtualizer` isn't virtualized.
use assertr::prelude::*;
use browser_test::thirtyfour::prelude::*;
use rootcause::Report;

use crate::{
    pages::{ElementActions, Page, PageActions, role},
    polling::wait_for,
};

const PATH: &str = "/atoms/virtualizer";

/// "Item {i}" for each `i`.
fn items(range: std::ops::Range<u32>) -> Vec<String> {
    range.map(|i| format!("Item {i}")).collect()
}

/// The texts of the rendered options of the 50-item list, read in one call: options come and go
/// while scrolling.
async fn option_texts(page: &Page<'_>) -> Result<Vec<String>, Report> {
    page.eval(
        "return Array.from(document.querySelectorAll('#test-virt-list [role=option]'))
             .map(o => o.textContent);",
        vec![],
    )
    .await
}

/// How far the focused element lies inside the visible part of a list: its distances from the
/// list's top and bottom edges (negative: cut off).
#[derive(Debug)]
struct Margins {
    top: f64,
    bottom: f64,
}

/// The [`Margins`] of the focused element inside `list`.
async fn focused_margins(page: &Page<'_>, list: &WebElement) -> Result<Margins, Report> {
    let list = list.client_rect().await?;
    let option = page.focused_element().await?.client_rect().await?;
    Ok(Margins {
        top: option.top - list.top,
        bottom: list.bottom - option.bottom,
    })
}

/// How far `element` is scrolled from its end, in pixels.
async fn distance_to_end(page: &Page<'_>, element: &WebElement) -> Result<f64, Report> {
    page.eval(
        "const el = arguments[0]; return el.scrollHeight - el.clientHeight - el.scrollTop;",
        vec![element.to_json()?],
    )
    .await
}

/// The top and bottom of each rendered option in `container`, sorted by top. Read in one call, as
/// rows are measured and re-rendered.
async fn option_extents(
    page: &Page<'_>,
    container: &WebElement,
) -> Result<Vec<(f64, f64)>, Report> {
    page.eval(
        "return Array.from(arguments[0].querySelectorAll('[role=option]'))
             .map(o => o.getBoundingClientRect()).sort((a, b) => a.top - b.top)
             .map(r => [r.top, r.bottom]);",
        vec![container.to_json()?],
    )
    .await
}

/// Whether `extents` are at least 5 rows, each starting at or after the end of the one before.
fn stacked(extents: &[(f64, f64)]) -> bool {
    extents.len() >= 5 && extents.windows(2).all(|pair| pair[0].1 <= pair[1].0 + 0.5)
}

/// Wait until the focused option is fully inside the visible part of `list` (1px tolerance).
async fn wait_for_focused_in_view(page: &Page<'_>, list: &WebElement) -> Result<(), Report> {
    wait_for("the focused option's margins inside the list")
        .observing(|| focused_margins(page, list))
        .to_be("at least -1px each", |margins| {
            margins.top >= -1.0 && margins.bottom >= -1.0
        })
        .await
}

/// Wait until `log` is scrolled to its end (within 2px).
async fn wait_for_the_end(page: &Page<'_>, log: &WebElement) -> Result<(), Report> {
    wait_for("the log's distance from its end")
        .observing(|| distance_to_end(page, log))
        .to_be("less than 2px", |distance| distance.abs() < 2.0)
        .await
}

/// Wait until `log`'s rendered rows are stacked without overlapping.
async fn wait_for_stacked_rows(page: &Page<'_>, log: &WebElement) -> Result<(), Report> {
    wait_for("the log's rendered rows (top, bottom)")
        .observing(|| option_extents(page, log))
        .to_be("at least 5, not overlapping", |extents| stacked(extents))
        .await
}

/// 100px of 25px rows, a third of overscan, snapped to rows: Items 0 to 6, each with its position
/// in the set of 50.
pub async fn renders_the_visible_options(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    wait_for("the rendered options")
        .observing(|| option_texts(page))
        .to_be_equal_to(items(0..7))
        .await?;
    let first = page.element("#test-virt-list [role=option]").await?;
    assert_that!(first.attr("aria-setsize").await?)
        .get_some()
        .is_equal_to("50");
    assert_that!(first.attr("aria-posinset").await?)
        .get_some()
        .is_equal_to("1");
    Ok(())
}

/// Scrolled to 200px: Items 7 to 14 (the scroll moves on, so overscan goes down).
pub async fn scrolling_renders_other_options(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#test-virt-list [role=listbox]")
        .await?
        .scroll_to_top(200.0)
        .await?;
    wait_for("the rendered options")
        .observing(|| option_texts(page))
        .to_be_equal_to(items(7..15))
        .await?;
    Ok(())
}

/// The focused option scrolls into view, also one that wasn't rendered when it got focus; End
/// reaches and renders the last option (persisted as the focused key).
pub async fn focused_option_scrolls_into_view(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let list = page.element("#test-virt-list [role=listbox]").await?;
    page.element("#test-virt-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    wait_for("the role of the focused element")
        .observing(|| async { Ok(page.focused_element().await?.attr("role").await?) })
        .to_be_equal_to(Some("option".to_owned()))
        .await?;
    page.send_keys(Key::PageDown).await?;
    wait_for_focused_in_view(page, &list).await?;

    page.send_keys(Key::End).await?;
    let last = page.element(role("option").text("Item 49")).await?;
    page.wait_for_focus(&last).await?;
    wait_for_focused_in_view(page, &list).await?;
    assert_that!(last.attr("aria-posinset").await?)
        .get_some()
        .is_equal_to("50");

    page.send_keys(Key::Home).await?;
    page.wait_for_focus(&page.element(role("option").text("Item 0")).await?)
        .await?;
    wait_for_focused_in_view(page, &list).await?;
    Ok(())
}

/// The log starts at its end and stays there when lines are appended; its rows of variable height
/// don't overlap once measured, at the end and in the middle.
pub async fn log_stays_at_its_end(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let log = page.element("#test-virt-log [role=listbox]").await?;
    wait_for_the_end(page, &log).await?;
    page.element("#test-virt-append").await?.click().await?;
    page.element(role("option").text("Line 109")).await?;
    wait_for_the_end(page, &log).await?;
    wait_for_stacked_rows(page, &log).await?;

    let height: f64 = log
        .prop("scrollHeight")
        .await?
        .unwrap_or_default()
        .parse()?;
    log.scroll_to_top(height / 2.0).await?;
    wait_for_stacked_rows(page, &log).await?;
    Ok(())
}

/// The `Virtualizer`s' context doesn't reach the list box after them: all its options render.
pub async fn plain_list_box_is_not_virtualized(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.wait_for_count("#test-virt-plain [role=option]", 30)
        .await?;
    Ok(())
}
