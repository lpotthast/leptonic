// Upstream: react-aria-components/test/ListBox.test.js @ 99e6102368
use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{
    BrowserTest, async_trait,
    thirtyfour::{Key, WebDriver},
};
use rootcause::Report;

use crate::pages::{BaseActions, Page};

/// A virtualized `ListBox` ("should support virtualizer"): only the visible options (plus
/// overscan) render, each telling its position and the set size; scrolling renders others; End
/// reaches and renders the last option. A log anchored to the end stays at the end when lines are
/// appended, with measured variable heights (rows don't overlap once measured). A list box next to
/// a `Virtualizer` isn't virtualized.
pub struct VirtualizerTests {}

#[async_trait]
impl BrowserTest<str> for VirtualizerTests {
    fn name(&self) -> Cow<'_, str> {
        "virtualizer_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/atoms/virtualizer").await?;

        // 100px of 25px rows, a third of overscan, snapped to rows: Items 0 to 6.
        let texts = wait_for_options(&page, "Item 0").await?;
        assert_that!(texts).is_equal_to((0..7).map(|i| format!("Item {i}")).collect::<Vec<_>>());
        let first = page.css("#test-virt-list [role=option]").await?;
        assert_that!(first.attr("aria-setsize").await?).is_equal_to(Some("50".to_owned()));
        assert_that!(first.attr("aria-posinset").await?).is_equal_to(Some("1".to_owned()));

        // Scrolled to 200px: Items 7 to 14 (the scroll moves on, so overscan goes down).
        page.driver
            .execute(
                "document.querySelector('#test-virt-list [role=listbox]').scrollTop = 200",
                vec![],
            )
            .await?;
        let texts = wait_for_options(&page, "Item 7").await?;
        assert_that!(texts).is_equal_to((7..15).map(|i| format!("Item {i}")).collect::<Vec<_>>());

        // End reaches the last option: rendered (persisted as the focused key).
        page.click_element_with_id("test-virt-before").await?;
        page.press_tab().await?;
        page.wait_for_focus("option", None).await?;
        // The focused option scrolls into view, also one that wasn't rendered when it got focus.
        let focused_in_view = "(() => { const list = document.querySelector('#test-virt-list [role=listbox]').getBoundingClientRect(); \
                               const option = document.activeElement.getBoundingClientRect(); \
                               return option.top >= list.top - 1 && option.bottom <= list.bottom + 1; })()";
        page.send_keys_to_active(Key::PageDown).await?;
        wait_until(&page, focused_in_view).await?;
        page.send_keys_to_active(Key::End).await?;
        page.wait_for_focus("option", Some("Item 49")).await?;
        wait_until(&page, focused_in_view).await?;
        let last = page.by_role_and_text("option", "Item 49").await?;
        assert_that!(last.attr("aria-posinset").await?).is_equal_to(Some("50".to_owned()));
        page.send_keys_to_active(Key::Home).await?;
        page.wait_for_focus("option", Some("Item 0")).await?;
        wait_until(&page, focused_in_view).await?;

        // The log starts at its end and stays there when lines are appended.
        let at_end = "(() => { const el = document.querySelector('#test-virt-log [role=listbox]'); \
                      return Math.abs(el.scrollHeight - el.clientHeight - el.scrollTop) < 2; })()";
        wait_until(&page, at_end).await?;
        page.wait_for_selector("#test-virt-log [role=option]")
            .await?;
        page.click_element_with_id("test-virt-append").await?;
        wait_until(&page, "!!Array.from(document.querySelectorAll('#test-virt-log [role=option]')).find(o => o.textContent === 'Line 109')").await?;
        wait_until(&page, at_end).await?;
        // Measured: the rows of variable height don't overlap, at the end and in the middle.
        wait_until(&page, &no_overlap("#test-virt-log")).await?;
        page.driver
            .execute(
                "const el = document.querySelector('#test-virt-log [role=listbox]'); el.scrollTop = el.scrollHeight / 2;",
                vec![],
            )
            .await?;
        wait_until(&page, &no_overlap("#test-virt-log")).await?;

        // The `Virtualizer`s' context doesn't reach the list box after them: all its options
        // render.
        page.wait_for_count("#test-virt-plain [role=option]", 30)
            .await?;
        page.expect_no_page_errors().await
    }
}

/// Whether the rendered options in `container` are stacked without overlapping (each starts at
/// or after the end of the one before), with at least 5 of them.
fn no_overlap(container: &str) -> String {
    format!(
        "(() => {{ const rects = Array.from(document.querySelectorAll('{container} [role=option]')) \
         .map(o => o.getBoundingClientRect()).sort((a, b) => a.top - b.top); \
         return rects.length >= 5 && rects.every((r, i) => i === 0 || rects[i - 1].bottom <= r.top + 0.5); }})()"
    )
}

/// The texts of the rendered options of the 50-item list, once the first one is `first`.
async fn wait_for_options(page: &Page<'_>, first: &str) -> Result<Vec<String>, Report> {
    for _ in 0..100 {
        // In one call: options come and go while scrolling.
        let texts = page
            .driver
            .execute(
                "return Array.from(document.querySelectorAll('#test-virt-list [role=option]')).map(o => o.textContent);",
                vec![],
            )
            .await?
            .convert::<Vec<String>>()?;
        if texts.first().map(String::as_str) == Some(first) {
            return Ok(texts);
        }
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    }
    rootcause::bail!("the first option never became {first}")
}

/// Waits until the JavaScript expression `condition` is true.
async fn wait_until(page: &Page<'_>, condition: &str) -> Result<(), Report> {
    for _ in 0..100 {
        let value = page
            .driver
            .execute(&format!("return {condition};"), vec![])
            .await?
            .convert::<bool>()?;
        if value {
            return Ok(());
        }
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    }
    rootcause::bail!("never true: {condition}")
}
