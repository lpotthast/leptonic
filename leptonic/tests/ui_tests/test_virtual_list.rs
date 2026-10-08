// No upstream: `VirtualList` is a leptonic addition (react-aria has no virtualized plain list).
use std::{borrow::Cow, time::Duration};

use assertr::prelude::*;
use browser_test::{BrowserTest, async_trait, thirtyfour::prelude::*};
use rootcause::{Report, prelude::ResultExt};
use serde::{Deserialize, Serialize, de::DeserializeOwned};

use crate::{
    pages::{ElementActions, Page, PageActions},
    polling::{expect, wait_for},
};

/// A virtualized log (`VirtualList`, no react-aria equivalent): it renders a slice of its 2,000
/// lines and follows its end while lines are appended; scrolling away stops following, toggling
/// it on scrolls back to the end; rows holding the text selection stay rendered; the rows' text
/// is selectable.
pub struct VirtualListTests {}

#[async_trait]
impl BrowserTest<str> for VirtualListTests {
    fn name(&self) -> Cow<'_, str> {
        "virtual_list_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/atoms/virtual_list").await?;

        cases!(
            follows_its_end(&page),
            appended_lines_come_into_view(&page),
            scroll_jumps_render_rows_in_order(&page),
            scrolling_away_stops_following(&page),
            selected_row_stays_rendered(&page),
            turning_following_on_scrolls_to_the_end(&page),
        );
        Ok(())
    }
}

/// A view rebuilt in place (the same type, a new owner: tachys reuses the DOM) whose elements carry
/// a hook's props: the old owner's handlers must be gone (agnite dev-ui, 2026-10-07: a click
/// panicked on disposed signals).
pub struct VirtualListRebuildTests {}

#[async_trait]
impl BrowserTest<str> for VirtualListRebuildTests {
    fn name(&self) -> Cow<'_, str> {
        "virtual_list_rebuild_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/atoms/virtual_list").await?;

        cases!(rebuilt_views_drop_the_old_handlers(&page));
        Ok(())
    }
}

/// Attributes spread onto a component (`<Comp {..props.into_attrs()} />`, with
/// `--cfg=erase_components`: a `Vec<AnyAttribute>`) whose view is rebuilt in place: Leptos'
/// `Vec<AnyAttribute>::rebuild` removes the old attributes by key but not the old event
/// listeners, so the disposed owner's handlers still run (and panic). A bug in tachys (0.2.19 and
/// its main branch, 2026-03-14), not in leptonic.
pub struct ComponentSpreadRebuildKnownIssues {}

#[async_trait]
impl BrowserTest<str> for ComponentSpreadRebuildKnownIssues {
    fn name(&self) -> Cow<'_, str> {
        "component_spread_rebuild_known_issues".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/atoms/virtual_list").await?;

        cases!(rebuilt_component_spread_drops_the_old_handlers(&page));
        Ok(())
    }
}

/// Following turns off when the user scrolls away from the end: measured row sizes must survive
/// that (the layout options change, but only `anchor_to`), else the content jumps and every row is
/// measured again. A behavior guard: the original bug didn't reproduce here (its timing); the
/// regression test is the unit test `anchoring_changes_keep_measured_sizes`.
pub struct VirtualListFollowToggleTests {}

#[async_trait]
impl BrowserTest<str> for VirtualListFollowToggleTests {
    fn name(&self) -> Cow<'_, str> {
        "virtual_list_follow_toggle_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/atoms/virtual_list").await?;

        cases!(follow_toggle_keeps_measured_sizes(&page));
        Ok(())
    }
}

/// Runs `script` with `log` bound to the log element and `arg` to `arg`; `script` returns its
/// result (`return ...`) if any.
async fn on_log<T: DeserializeOwned>(
    page: &Page<'_>,
    script: &str,
    arg: impl Serialize,
) -> Result<T, Report> {
    page.eval(
        &format!(
            "const log = document.getElementById('test-vl-log'); const arg = arguments[0]; {script}"
        ),
        vec![serde_json::to_value(arg)?],
    )
    .await
}

/// How far the log is scrolled from its end, in pixels.
const DISTANCE_TO_END: &str = "return log.scrollHeight - log.clientHeight - log.scrollTop;";

/// The texts of the rendered lines, in DOM order.
const RENDERED_LINES: &str =
    "return Array.from(log.querySelectorAll('.line')).map(l => l.textContent);";

/// The `top` of each rendered row, in DOM order (a text selection follows the DOM).
const ROW_TOPS: &str = "return Array.from(log.querySelectorAll('.line')).map(l => parseFloat(l.parentElement.style.top));";

/// The extent of the rendered rows and of the log's viewport, in content pixels.
const COVERAGE: &str = "
    const rows = Array.from(log.querySelectorAll('.line')).map(l => l.parentElement);
    return {
        rows_top: rows.length ? Math.min(...rows.map(r => parseFloat(r.style.top))) : null,
        rows_bottom: rows.length
            ? Math.max(...rows.map(r => parseFloat(r.style.top) + parseFloat(r.style.height)))
            : null,
        view_top: log.scrollTop,
        view_bottom: log.scrollTop + log.clientHeight,
    };";

/// The extent of the rendered rows and of the viewport ([`COVERAGE`]).
#[derive(Debug, Deserialize)]
struct Coverage {
    rows_top: Option<f64>,
    rows_bottom: Option<f64>,
    view_top: f64,
    view_bottom: f64,
}

impl Coverage {
    fn covers_the_view(&self) -> bool {
        matches!(
            (self.rows_top, self.rows_bottom),
            (Some(top), Some(bottom)) if top <= self.view_top && bottom >= self.view_bottom
        )
    }
}

/// Wait until the log is scrolled to its end (within 2px).
async fn wait_for_the_end(page: &Page<'_>) -> Result<(), Report> {
    wait_for("the log's distance from its end")
        .observing(|| on_log::<f64>(page, DISTANCE_TO_END, ()))
        .to_be("less than 2px", |distance| distance.abs() < 2.0)
        .await
}

/// Wait until the line with the text `line` is rendered.
async fn wait_for_rendered(page: &Page<'_>, line: &str) -> Result<(), Report> {
    wait_for("the rendered lines")
        .observing(|| on_log::<Vec<String>>(page, RENDERED_LINES, ()))
        .to_be(&format!("including {line:?}"), |lines| {
            lines.iter().any(|rendered| rendered == line)
        })
        .await
}

/// The rendered rows' tops are in visual order in the DOM.
async fn assert_rows_in_visual_order(page: &Page<'_>) -> Result<(), Report> {
    let tops = on_log::<Vec<f64>>(page, ROW_TOPS, ()).await?;
    let mut sorted = tops.clone();
    sorted.sort_by(f64::total_cmp);
    assert_that!(tops).is_equal_to(sorted);
    Ok(())
}

/// Selects from the first to the third short line ("Line <n>") in visual order; returns the
/// selection's text (whitespace collapsed) and the texts of its first and last line.
const SELECT_THREE_SHORT_ROWS: &str = "
    const lines = Array.from(log.querySelectorAll('.line'))
        .filter(l => /^Line \\d+$/.test(l.textContent))
        .sort((a, b) => parseFloat(a.parentElement.style.top) - parseFloat(b.parentElement.style.top));
    const range = document.createRange();
    range.setStartBefore(lines[0]);
    range.setEndAfter(lines[2]);
    return [range.toString().replace(/\\s+/g, ' ').trim(), [lines[0].textContent, lines[2].textContent]];";

/// The first row in the log's view and its offset from the view's top, e.g. `Line 1234@-3`.
const ANCHOR: &str = "
    const row = Array.from(log.querySelectorAll('.line')).map(l => l.parentElement)
        .find(r => parseFloat(r.style.top) >= log.scrollTop);
    return row.textContent.slice(0, 12) + '@' + (parseFloat(row.style.top) - log.scrollTop);";

/// Following: at the end, with the last line rendered, and only a slice of all lines; the lines'
/// text is selectable.
async fn follows_its_end(page: &Page<'_>) -> Result<(), Report> {
    wait_for_the_end(page).await?;
    wait_for_rendered(page, "Line 1999").await?;
    assert_that!(page.count("#test-vl-log .line").await?).is_less_than(100);
    let line = page.element("#test-vl-log .line").await?;
    assert_that!(line.css_value("user-select").await?).is_not_equal_to("none");
    Ok(())
}

/// Appended lines come into view, in visual order in the DOM.
async fn appended_lines_come_into_view(page: &Page<'_>) -> Result<(), Report> {
    page.element("#test-vl-append").await?.click().await?;
    wait_for_rendered(page, "Line 2049").await?;
    wait_for_the_end(page).await?;
    assert_rows_in_visual_order(page).await?;
    Ok(())
}

/// After scroll jumps too, the rows are in visual order; a selection from one visible row to
/// another holds exactly the rows between.
async fn scroll_jumps_render_rows_in_order(page: &Page<'_>) -> Result<(), Report> {
    for top in [10_000, 5_000, 20_000] {
        on_log::<()>(page, "log.scrollTop = arg;", top).await?;
        wait_for(format!("the rendered rows, scrolled to {top}"))
            .observing(|| on_log::<Coverage>(page, COVERAGE, ()))
            .to_be("covering the viewport", Coverage::covers_the_view)
            .await?;
        assert_rows_in_visual_order(page)
            .await
            .context_with(|| format!("scrolled to {top}"))?;
    }
    let (text, [first, last]) =
        on_log::<(String, [String; 2])>(page, SELECT_THREE_SHORT_ROWS, ()).await?;
    assert_that!(text.as_str())
        .starts_with(&first)
        .ends_with(&last);
    // Only the rows between (no long line: the rows were picked from short ones in a run).
    assert_that!(text.matches("Line ").count()).is_less_than(5);
    Ok(())
}

/// Scrolling away stops following; appended lines don't move the view.
async fn scrolling_away_stops_following(page: &Page<'_>) -> Result<(), Report> {
    on_log::<()>(page, "log.scrollTop = arg;", 0).await?;
    page.element("#test-vl-follow")
        .await?
        .wait_for_inner_text("not following")
        .await?;
    page.element("#test-vl-append").await?.click().await?;
    wait_for_rendered(page, "Line 1").await?;
    assert_that!(on_log::<f64>(page, "return log.scrollTop;", ()).await?).is_equal_to(0.0);
    Ok(())
}

/// A row holding the text selection stays rendered while scrolled away; without the selection,
/// it goes.
async fn selected_row_stays_rendered(page: &Page<'_>) -> Result<(), Report> {
    on_log::<()>(
        page,
        "const line = Array.from(log.querySelectorAll('.line')).find(l => l.textContent === arg);
         window.selectionChanges = 0;
         document.addEventListener('selectionchange', () => window.selectionChanges += 1);
         const range = document.createRange();
         range.selectNodeContents(line);
         const selection = window.getSelection();
         selection.removeAllRanges();
         selection.addRange(range);",
        "Line 1",
    )
    .await?;
    // `selectionchange` is dispatched later; the list's own listener (registered at mount) has
    // run once this one did. A user doesn't select and scroll within one task.
    wait_for("the number of selection changes")
        .observing(|| on_log::<u32>(page, "return window.selectionChanges;", ()))
        .to_be("at least 1", |changes| *changes >= 1)
        .await?;
    // Scrolling back to the end follows again; the selected row stays rendered.
    let follow = page.element("#test-vl-follow").await?;
    wait_for("the following state, scrolled to the end")
        .observing(|| async {
            on_log::<()>(page, "log.scrollTop = log.scrollHeight;", ()).await?;
            follow.inner_text().await
        })
        .to_be_equal_to("following")
        .await?;
    wait_for_rendered(page, "Line 2099").await?;
    assert_that!(on_log::<Vec<String>>(page, RENDERED_LINES, ()).await?)
        .contains("Line 1".to_owned());
    assert_that!(on_log::<String>(page, "return window.getSelection().toString();", ()).await?)
        .is_equal_to("Line 1");

    on_log::<()>(page, "window.getSelection().removeAllRanges();", ()).await?;
    wait_for("the rendered lines")
        .observing(|| on_log::<Vec<String>>(page, RENDERED_LINES, ()))
        .to_be("without \"Line 1\"", |lines| {
            lines.iter().all(|line| line != "Line 1")
        })
        .await?;
    Ok(())
}

/// Turning following on scrolls to the end.
async fn turning_following_on_scrolls_to_the_end(page: &Page<'_>) -> Result<(), Report> {
    let follow = page.element("#test-vl-follow").await?;
    on_log::<()>(page, "log.scrollTop = arg;", 0).await?;
    follow.wait_for_inner_text("not following").await?;
    follow.click().await?;
    follow.wait_for_inner_text("following").await?;
    wait_for_rendered(page, "Line 2099").await?;
    wait_for_the_end(page).await?;
    Ok(())
}

/// Rebuilding the view three times: clicks on the rebuilt list and the plain element, and Tab, hit
/// only live handlers (a disposed owner's handler panics, which fails the test).
async fn rebuilt_views_drop_the_old_handlers(page: &Page<'_>) -> Result<(), Report> {
    page.element("#test-vl-rebuilt-list .line").await?;
    for round in 1..=3 {
        page.element("#test-vl-source").await?.click().await?;
        page.element("#test-vl-rebuilt-plain")
            .await?
            .wait_for_inner_text(&format!("Plain {round}"))
            .await?;
        page.element("#test-vl-rebuilt-list").await?.click().await?;
        page.element("#test-vl-rebuilt-plain")
            .await?
            .click()
            .await?;
        page.send_keys(Key::Tab).await?;
    }
    Ok(())
}

/// A click on the rebuilt component with spread attributes hits only live handlers.
async fn rebuilt_component_spread_drops_the_old_handlers(page: &Page<'_>) -> Result<(), Report> {
    page.element("#test-vl-source").await?.click().await?;
    page.element("#test-vl-rebuilt-plain")
        .await?
        .wait_for_inner_text("Plain 1")
        .await?;
    page.element("#test-vl-rebuilt-wrapper")
        .await?
        .click()
        .await?;
    Ok(())
}

/// A user scroll up from the end and down again (so the rows above the stop were rendered and
/// measured) turns following off; the content doesn't move, and no measured row height goes back
/// to the estimate.
async fn follow_toggle_keeps_measured_sizes(page: &Page<'_>) -> Result<(), Report> {
    wait_for_the_end(page).await?;
    wait_for_rendered(page, "Line 1999").await?;

    // Records every row wrapper whose measured height goes back to the 20px estimate.
    on_log::<()>(
        page,
        "window.__vlReestimated = [];
         new MutationObserver(records => {
             for (const record of records) {
                 const wrapper = record.target;
                 if (!wrapper.firstElementChild?.classList.contains('line')) continue;
                 const old = /height: ([0-9.]+)px/.exec(record.oldValue ?? '')?.[1];
                 if (old !== undefined && old !== '20' && wrapper.style.height === '20px') {
                     window.__vlReestimated.push(wrapper.textContent.slice(0, 12));
                 }
             }
         }).observe(log, { subtree: true, attributes: true, attributeFilter: ['style'], attributeOldValue: true });",
        (),
    )
    .await?;

    // The user scroll: one step every 50ms (real timers pacing it, so the scroll doesn't end in
    // between), 30 up, 12 down. Then the first row in view and its offset, before the scroll ends
    // (300ms later) and turns following off.
    for step in std::iter::repeat_n(-150, 30).chain(std::iter::repeat_n(150, 12)) {
        on_log::<()>(page, "log.scrollTop += arg;", step).await?;
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
    tokio::time::sleep(Duration::from_millis(70)).await;
    let before = on_log::<String>(page, ANCHOR, ()).await?;
    page.element("#test-vl-follow")
        .await?
        .wait_for_inner_text("not following")
        .await?;
    // Settle (a re-layout runs in effects and frames), then check: the content didn't move.
    expect("the anchor row and its offset")
        .observing(|| on_log::<String>(page, ANCHOR, ()))
        .for_at_least(Duration::from_millis(500))
        .to_stay_equal_to(before)
        .await?;
    assert_that!(on_log::<Vec<String>>(page, "return window.__vlReestimated;", ()).await?)
        .is_empty();
    Ok(())
}
