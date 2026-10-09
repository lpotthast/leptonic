// Upstream: react-aria-components/test/ListBox.test.js @ 99e6102368
// Upstream: react-aria-components/test/GridList.browser.test.tsx @ 99e6102368
//! A virtualized `ListBox` ("should support virtualizer"): only the visible options (plus
//! overscan) render, each telling its position and the set size; scrolling renders others; End
//! reaches and renders the last option. A log anchored to the end stays at the end when lines are
//! appended, with measured variable heights (rows don't overlap once measured). A list box next to
//! a `Virtualizer` isn't virtualized. The focused option stays rendered while scrolled away;
//! type-ahead reaches options not rendered; pressing a scrolled-to option selects it; the list
//! renders again after `display: none` (react-aria-components' `GridList.browser.test.tsx`).
use assertr::{
    matchers::{all_of, eq, gt, lt},
    prelude::*,
};
use browser_test::{browser_test, thirtyfour::prelude::*};
use leptonic::AriaRole;
use rootcause::Report;

use crate::pages::{ElementActions, Page, role};

const PATH: &str = "/atoms/virtualizer";

/// "Item {i}" for each `i`.
fn items(range: std::ops::Range<u32>) -> Vec<String> {
    range.map(|i| format!("Item {i}")).collect()
}

/// The texts of the rendered options of the 50-item list, read in one call: options come and go
/// while scrolling.
async fn option_texts(page: &Page<'_>) -> Result<Vec<String>, Report> {
    page.low_level()
        .eval(
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

/// The top and bottom of each rendered option in `container`, sorted by top. Read in one call, as
/// rows are measured and re-rendered.
async fn option_extents(
    page: &Page<'_>,
    container: &WebElement,
) -> Result<Vec<(f64, f64)>, Report> {
    page.low_level()
        .eval(
            "return Array.from(arguments[0].querySelectorAll('[role=option]'))
             .map(o => o.getBoundingClientRect()).sort((a, b) => a.top - b.top)
             .map(r => [r.top, r.bottom]);",
            vec![container.to_json()?],
        )
        .await
}

/// Wait until the focused option is fully inside the visible part of `list` (1px tolerance).
async fn wait_for_focused_in_view(page: &Page<'_>, list: &WebElement) -> Result<(), Report> {
    assert_that!(|| focused_margins(page, list))
        .eventually_ok()
        .satisfies(|margins| {
            margins
                .derive(|margins| &margins.top)
                .is_greater_or_equal_to(-1.0);
            margins
                .derive(|margins| &margins.bottom)
                .is_greater_or_equal_to(-1.0);
        })
        .await;
    Ok(())
}

/// Wait until `log` is scrolled to its end (within 2px).
async fn wait_for_the_end(log: &WebElement) -> Result<(), Report> {
    assert_that!(|| async { Ok::<_, Report>(log.scroll_extent().await?.distance_to_end()) })
        .eventually_ok()
        .matches(all_of(matchers![gt(-2.0), lt(2.0)]))
        .await;
    Ok(())
}

/// Wait until `log`'s rendered rows are stacked without overlapping.
async fn wait_for_stacked_rows(page: &Page<'_>, log: &WebElement) -> Result<(), Report> {
    assert_that!(|| option_extents(page, log))
        .eventually_ok()
        .satisfies(|extents| {
            extents.derive_owned(Vec::len).is_greater_or_equal_to(5);
            for (index, pair) in extents.actual().windows(2).enumerate() {
                extents
                    .derive_owned(|extents| extents[index].1)
                    .with_detail_message(format!(
                        "bottom of row {index} must not overlap the next row"
                    ))
                    .is_less_or_equal_to(pair[1].0 + 0.5);
            }
        })
        .await;
    Ok(())
}

/// A virtualized list box renders only its visible options plus overscan, each with its position
/// in the full set ("should support virtualizer").
#[browser_test]
pub async fn renders_the_visible_options(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    assert_that!(|| option_texts(page))
        .eventually_ok()
        .matches(eq(items(0..7)))
        .await;
    let first = page.first_element("#test-virt-list [role=option]").await?;
    assert_that!(first)
        .has_attribute("aria-setsize")
        .await
        .is_equal_to("50");
    assert_that!(first)
        .has_attribute("aria-posinset")
        .await
        .is_equal_to("1");
    Ok(())
}

/// Scrolling a virtualized list box renders the options scrolled into view, with the overscan in
/// the scroll direction ("should support virtualizer").
#[browser_test]
pub async fn scrolling_renders_other_options(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#test-virt-list [role=listbox]")
        .await?
        .scroll_to_top(200.0)
        .await?;
    assert_that!(|| option_texts(page))
        .eventually_ok()
        .matches(eq(items(7..15)))
        .await;
    Ok(())
}

/// PageDown, End and Home scroll the option they focus into view, rendering it first if it wasn't
/// rendered (End reaches the last option).
#[browser_test]
pub async fn focused_option_scrolls_into_view(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let list = page.element("#test-virt-list [role=listbox]").await?;
    page.element("#test-virt-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    assert_that!(|| async { Ok::<_, Report>(page.focused_element().await?.attr("role").await?) })
        .eventually_ok()
        .matches(eq(Some("option".to_owned())))
        .await;
    page.send_keys(Key::PageDown).await?;
    wait_for_focused_in_view(page, &list).await?;

    page.send_keys(Key::End).await?;
    let last = page.element(role(AriaRole::Option).text("Item 49")).await?;
    page.wait_for_focus(&last).await?;
    wait_for_focused_in_view(page, &list).await?;
    assert_that!(last)
        .has_attribute("aria-posinset")
        .await
        .is_equal_to("50");

    page.send_keys(Key::Home).await?;
    page.wait_for_focus(&page.element(role(AriaRole::Option).text("Item 0")).await?)
        .await?;
    wait_for_focused_in_view(page, &list).await?;
    Ok(())
}

/// The log starts at its end and stays there when lines are appended; its rows of variable height
/// don't overlap once measured, at the end and in the middle.
#[browser_test]
pub async fn log_stays_at_its_end(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let log = page.element("#test-virt-log [role=listbox]").await?;
    wait_for_the_end(&log).await?;
    page.element("#test-virt-append").await?.click().await?;
    page.element(role(AriaRole::Option).text("Line 109"))
        .await?;
    wait_for_the_end(&log).await?;
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

/// A list box rendered after a `Virtualizer` is not virtualized: all its options render.
#[browser_test]
pub async fn plain_list_box_is_not_virtualized(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.wait_for_count("#test-virt-plain [role=option]", 30)
        .await?;
    Ok(())
}

/// Focuses the first option of the 50-item list with Tab; returns it.
async fn focus_first_option(page: &Page<'_>) -> Result<WebElement, Report> {
    page.element("#test-virt-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    let first = page.element(role(AriaRole::Option).text("Item 0")).await?;
    page.wait_for_focus(&first).await?;
    Ok(first)
}

/// The focused option stays rendered (with its position in the set) while the list is scrolled
/// away from it, and Tab out of the list and Shift+Tab back focus it again ("should support
/// virtualizer": the focused key is persisted).
#[browser_test]
pub async fn focused_option_stays_rendered(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let first = focus_first_option(page).await?;
    page.element("#test-virt-list [role=listbox]")
        .await?
        .scroll_to_top(500.0)
        .await?;
    page.element(role(AriaRole::Option).text("Item 22")).await?;
    assert_that!(|| option_texts(page))
        .eventually_ok()
        .satisfies(|texts| {
            texts
                .contains("Item 0".to_owned())
                .does_not_contain("Item 1".to_owned());
        })
        .await;
    assert_that!(first)
        .has_attribute("aria-posinset")
        .await
        .is_equal_to("1");

    page.send_keys(Key::Tab).await?;
    page.wait_for_focus(&page.element("#test-virt-after").await?)
        .await?;
    page.send_keys(Key::Shift + Key::Tab).await?;
    page.wait_for_focus(&first).await?;
    Ok(())
}

/// Typing an option's text focuses it, rendering it first if it wasn't rendered, and scrolls it
/// into view (type-ahead with a keyboard delegate that knows every option's position).
#[browser_test]
pub async fn type_ahead_reaches_an_unrendered_option(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let list = page.element("#test-virt-list [role=listbox]").await?;
    focus_first_option(page).await?;
    page.type_text("Item 37").await?;
    let option = page.element(role(AriaRole::Option).text("Item 37")).await?;
    page.wait_for_focus(&option).await?;
    wait_for_focused_in_view(page, &list).await?;
    Ok(())
}

/// Pressing an option rendered after scrolling selects and focuses it (react-aria-components'
/// `GridList.browser.test.tsx` "selects a row via mouse in real browser grid layout").
#[browser_test]
pub async fn pressing_a_scrolled_to_option_selects_it(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    page.element("#test-virt-list [role=listbox]")
        .await?
        .scroll_to_top(500.0)
        .await?;
    let option = page.element(role(AriaRole::Option).text("Item 21")).await?;
    option.click().await?;
    option.wait_for_attr("aria-selected", Some("true")).await?;
    page.wait_for_focus(&option).await?;
    Ok(())
}

/// A virtualized list hidden with `display: none` and shown again renders its options again
/// (react-aria-components' `GridList.browser.test.tsx` "virtualizer renders items after toggling
/// display:none").
#[browser_test]
pub async fn renders_options_after_being_hidden(page: &Page<'_>) -> Result<(), Report> {
    page.goto_path(PATH).await?;
    let hide = page.element("#test-virt-hide").await?;
    assert_that!(|| option_texts(page))
        .eventually_ok()
        .matches(eq(items(0..7)))
        .await;
    for _ in 0..2 {
        hide.click().await?;
        hide.click().await?;
        assert_that!(|| option_texts(page))
            .eventually_ok()
            .matches(eq(items(0..7)))
            .await;
    }
    Ok(())
}
