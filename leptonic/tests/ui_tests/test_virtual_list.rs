use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{BrowserTest, async_trait, thirtyfour::WebDriver};
use rootcause::Report;

use crate::pages::{BaseActions, Page};

/// A virtualized log (`VirtualList`, no react-aria equivalent): it renders a slice of its 2,000
/// lines and follows its end while lines are appended; scrolling away stops following, toggling
/// it on scrolls back to the end; rows holding the text selection stay rendered; the rows' text
/// is selectable.
pub struct VirtualListTests {}

const LOG: &str = "document.getElementById('test-vl-log')";

#[async_trait]
impl BrowserTest<str> for VirtualListTests {
    fn name(&self) -> Cow<'_, str> {
        "virtual_list_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/atoms/virtual_list").await?;

        // Following: at the end, with the last line rendered, and only a slice of all lines.
        wait_until(&page, &at_end()).await?;
        wait_until(&page, &rendered("Line 1999")).await?;
        let count = eval_number(&page, &format!("{LOG}.querySelectorAll('.line').length")).await?;
        assert_that!(count).is_less_than(100.0);
        let user_select = eval_string(
            &page,
            &format!("getComputedStyle({LOG}.querySelector('.line')).userSelect"),
        )
        .await?;
        assert_that!(user_select).is_not_equal_to("none".to_owned());

        // Appended lines come into view, in visual order in the DOM.
        page.click_element_with_id("test-vl-append").await?;
        wait_until(&page, &rendered("Line 2049")).await?;
        wait_until(&page, &at_end()).await?;
        assert_that!(eval_bool(&page, &in_visual_order()).await?).is_true();

        // After scroll jumps too; a selection from one visible row to another holds exactly the
        // rows between.
        for top in [10_000, 5_000, 20_000] {
            page.driver
                .execute(&format!("{LOG}.scrollTop = {top};"), vec![])
                .await?;
            tokio::time::sleep(std::time::Duration::from_millis(400)).await;
            assert_that!(eval_bool(&page, &in_visual_order()).await?).is_true();
        }
        let selected = eval_string(
            &page,
            &format!(
                "(() => {{ const lines = Array.from({LOG}.querySelectorAll('.line')) \
                 .filter(l => /^Line \\d+$/.test(l.textContent)) \
                 .sort((a, b) => parseFloat(a.parentElement.style.top) - parseFloat(b.parentElement.style.top)); \
                 const range = document.createRange(); range.setStartBefore(lines[0]); \
                 range.setEndAfter(lines[2]); return range.toString().replace(/\\s+/g, ' ').trim() + '|' + \
                 [lines[0], lines[2]].map(l => l.textContent).join(','); }})()"
            ),
        )
        .await?;
        let (text, ends) = selected.split_once('|').unwrap_or_default();
        let (first, last) = ends.split_once(',').unwrap_or_default();
        assert_that!(text.starts_with(first) && text.ends_with(last)).is_true();
        // Only the rows between (no long line: the rows were picked from short ones in a run).
        assert_that!(text.matches("Line ").count()).is_less_than(5);

        // Scrolling away stops following; appended lines don't move the view.
        page.driver
            .execute(&format!("{LOG}.scrollTop = 0;"), vec![])
            .await?;
        page.wait_for_text("test-vl-follow", "not following")
            .await?;
        page.click_element_with_id("test-vl-append").await?;
        wait_until(
            &page,
            &format!("{LOG}.scrollHeight > 0 && {}", rendered("Line 1")),
        )
        .await?;
        let top = eval_number(&page, &format!("{LOG}.scrollTop")).await?;
        assert_that!(top).is_equal_to(0.0);

        // A row holding the text selection stays rendered while scrolled away.
        page.driver
            .execute(
                &format!(
                    "const line = Array.from({LOG}.querySelectorAll('.line')).find(l => l.textContent === 'Line 1');
                     const range = document.createRange();
                     range.selectNodeContents(line);
                     const selection = window.getSelection();
                     selection.removeAllRanges();
                     selection.addRange(range);"
                ),
                vec![],
            )
            .await?;
        // Scrolling back to the end follows again; the selected row stays rendered.
        wait_until(
            &page,
            &format!(
                "({LOG}.scrollTop = {LOG}.scrollHeight, \
                 document.getElementById('test-vl-follow').textContent === 'following')"
            ),
        )
        .await?;
        wait_until(&page, &rendered("Line 2099")).await?;
        assert_that!(eval_bool(&page, &rendered("Line 1")).await?).is_true();
        assert_that!(eval_string(&page, "window.getSelection().toString()").await?)
            .is_equal_to("Line 1".to_owned());
        // Without the selection, the row goes.
        page.driver
            .execute("window.getSelection().removeAllRanges();", vec![])
            .await?;
        wait_until(&page, &format!("!{}", rendered("Line 1"))).await?;

        // Turning following on scrolls to the end.
        page.driver
            .execute(&format!("{LOG}.scrollTop = 0;"), vec![])
            .await?;
        page.wait_for_text("test-vl-follow", "not following")
            .await?;
        page.click_element_with_id("test-vl-follow").await?;
        page.wait_for_text("test-vl-follow", "following").await?;
        wait_until(&page, &rendered("Line 2099")).await?;
        wait_until(&page, &at_end()).await?;

        page.expect_no_page_errors().await
    }
}

/// Whether the rows' wrappers are in visual order in the DOM (a text selection follows the DOM).
fn in_visual_order() -> String {
    format!(
        "(() => {{ const tops = Array.from({LOG}.querySelectorAll('.line')).map(l => \
         parseFloat(l.parentElement.style.top)); return tops.every((top, i) => i === 0 || \
         tops[i - 1] <= top); }})()"
    )
}

/// Whether the log shows its end.
fn at_end() -> String {
    format!("Math.abs({LOG}.scrollHeight - {LOG}.clientHeight - {LOG}.scrollTop) < 2")
}

/// Whether a line with exactly this text is rendered.
fn rendered(text: &str) -> String {
    format!("Array.from({LOG}.querySelectorAll('.line')).some(l => l.textContent === '{text}')")
}

async fn eval_bool(page: &Page<'_>, expression: &str) -> Result<bool, Report> {
    Ok(page
        .driver
        .execute(&format!("return {expression};"), vec![])
        .await?
        .convert::<bool>()?)
}

async fn eval_number(page: &Page<'_>, expression: &str) -> Result<f64, Report> {
    Ok(page
        .driver
        .execute(&format!("return {expression};"), vec![])
        .await?
        .convert::<f64>()?)
}

async fn eval_string(page: &Page<'_>, expression: &str) -> Result<String, Report> {
    Ok(page
        .driver
        .execute(&format!("return {expression};"), vec![])
        .await?
        .convert::<String>()?)
}

/// Waits until the JavaScript expression `condition` is true.
async fn wait_until(page: &Page<'_>, condition: &str) -> Result<(), Report> {
    for _ in 0..100 {
        if eval_bool(page, condition).await? {
            return Ok(());
        }
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    }
    rootcause::bail!("never true: {condition}")
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
        wait_until(
            &page,
            "!!document.querySelector('#test-vl-rebuilt-list .line')",
        )
        .await?;
        for round in 1..=3 {
            page.click_element_with_id("test-vl-source").await?;
            page.wait_for_text("test-vl-rebuilt-plain", &format!("Plain {round}"))
                .await?;
            page.click_element_with_id("test-vl-rebuilt-list").await?;
            page.click_element_with_id("test-vl-rebuilt-plain").await?;
            page.press_tab().await?;
        }
        page.expect_no_page_errors().await
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
        page.click_element_with_id("test-vl-source").await?;
        page.wait_for_text("test-vl-rebuilt-plain", "Plain 1")
            .await?;
        page.click_element_with_id("test-vl-rebuilt-wrapper")
            .await?;
        page.expect_no_page_errors().await
    }
}
