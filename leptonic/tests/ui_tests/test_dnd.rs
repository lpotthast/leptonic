// Upstream: react-aria/test/dnd/dnd.test.js @ 99e6102368
use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{
    BrowserTest, async_trait,
    thirtyfour::{By, Key, WebDriver, WebElement},
};
use rootcause::Report;

use crate::pages::{BaseActions, Page};

/// Keyboard drag and drop (native drags can't be driven through WebDriver): `use_drag` /
/// `use_drop` between single elements, and reordering a collection with the collection hooks.
/// Spec: react-aria `dnd.test.js` ("keyboard") and the droppable collection behavior.
pub struct DndTests {}

#[async_trait]
impl BrowserTest<str> for DndTests {
    fn name(&self) -> Cow<'_, str> {
        "dnd_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/hooks/dnd").await?;

        basic_drag_and_drop(&page).await?;
        escape_cancels(&page).await?;
        reorder_a_list(&page).await?;

        Ok(())
    }
}

async fn attr(element: &WebElement, name: &str) -> Result<Option<String>, Report> {
    Ok(element.attr(name).await?)
}

/// The text of the element `aria-describedby` of `element` refers to.
async fn description(page: &Page<'_>, element: &WebElement) -> Result<String, Report> {
    let ids = attr(element, "aria-describedby").await?.unwrap_or_default();
    let text = page
        .driver
        .execute(
            "return arguments[0].split(' ').map(id => document.getElementById(id)?.textContent ?? '').join(' ');",
            vec![serde_json::Value::String(ids)],
        )
        .await?;
    Ok(text.json().as_str().unwrap_or_default().to_owned())
}

async fn log(page: &Page<'_>) -> Result<Vec<String>, Report> {
    let mut entries = Vec::new();
    for li in page
        .css("#test-dnd-log")
        .await?
        .find_all(By::Css("li"))
        .await?
    {
        entries.push(li.text().await?);
    }
    Ok(entries)
}

async fn expect_log(page: &Page<'_>, expected: &[&str]) -> Result<(), Report> {
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
    loop {
        let entries = log(page).await?;
        if entries == expected {
            return Ok(());
        }
        if std::time::Instant::now() > deadline {
            leptos_browser_test::bail!("expected log {expected:?}, got {entries:?}");
        }
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    }
}

async fn droppable(page: &Page<'_>, label: &str) -> Result<WebElement, Report> {
    page.by_role_and_text("button", label).await
}

async fn clear_log(page: &Page<'_>) -> Result<(), Report> {
    // Reloading resets the log and all state.
    page.goto_path("/hooks/dnd").await
}

async fn basic_drag_and_drop(page: &Page<'_>) -> Result<(), Report> {
    let draggable = page.element("test-dnd-draggable").await?;
    let target_1 = droppable(page, "Drop here").await?;
    let target_2 = droppable(page, "Drop here 2").await?;
    assert_that!(attr(&draggable, "data-dragging").await?).is_equal_to(Some("false".to_owned()));
    assert_that!(attr(&draggable, "draggable").await?).is_equal_to(Some("true".to_owned()));

    page.click_element_with_id("test-dnd-before").await?;
    page.press_tab().await?;
    page.wait_for_focus_on(&draggable, "the draggable").await?;
    assert_that!(description(page, &draggable).await?)
        .is_equal_to("Press Enter to start dragging.".to_owned());

    page.send_keys_to_active(Key::Enter).await?;
    // The drag moves focus to the nearest drop target.
    page.wait_for_focus_on(&target_1, "the first drop target")
        .await?;
    assert_that!(attr(&draggable, "data-dragging").await?).is_equal_to(Some("true".to_owned()));
    assert_that!(attr(&target_1, "data-droptarget").await?).is_equal_to(Some("true".to_owned()));
    assert_that!(description(page, &target_1).await?)
        .is_equal_to("Press Enter to drop. Press Escape to cancel drag.".to_owned());
    expect_log(page, &["dragstart", "dropenter 1"]).await?;

    page.press_tab().await?;
    page.wait_for_focus_on(&target_2, "the second drop target")
        .await?;
    assert_that!(attr(&target_1, "data-droptarget").await?).is_equal_to(Some("false".to_owned()));
    assert_that!(attr(&target_2, "data-droptarget").await?).is_equal_to(Some("true".to_owned()));
    expect_log(
        page,
        &["dragstart", "dropenter 1", "dropexit 1", "dropenter 2"],
    )
    .await?;

    page.send_keys_to_active(Key::Enter).await?;
    expect_log(
        page,
        &[
            "dragstart",
            "dropenter 1",
            "dropexit 1",
            "dropenter 2",
            "drop 2 hello world Move",
            // The drag ends before the drop target is left (react-aria's `DragSession::drop`).
            "dragend Move",
            "dropexit 2",
        ],
    )
    .await?;
    page.wait_for_focus_on(&target_2, "the second drop target")
        .await?;
    assert_that!(attr(&draggable, "data-dragging").await?).is_equal_to(Some("false".to_owned()));
    assert_that!(attr(&target_2, "data-droptarget").await?).is_equal_to(Some("false".to_owned()));
    // Drop targets are only described during drags.
    assert_that!(attr(&target_2, "aria-describedby").await?).is_none();
    Ok(())
}

async fn escape_cancels(page: &Page<'_>) -> Result<(), Report> {
    clear_log(page).await?;
    let draggable = page.element("test-dnd-draggable").await?;
    page.click_element_with_id("test-dnd-before").await?;
    page.press_tab().await?;
    page.send_keys_to_active(Key::Enter).await?;
    let target_1 = droppable(page, "Drop here").await?;
    page.wait_for_focus_on(&target_1, "the first drop target")
        .await?;
    page.send_keys_to_active(Key::Escape).await?;
    page.wait_for_focus_on(&draggable, "the draggable").await?;
    expect_log(
        page,
        &["dragstart", "dropenter 1", "dropexit 1", "dragend Cancel"],
    )
    .await
}

async fn row(page: &Page<'_>, letter: &str) -> Result<WebElement, Report> {
    page.css("[role=grid][aria-label='Letters']")
        .await?
        .find(By::XPath(format!(
            ".//*[@role='row'][.//*[@role='gridcell'][normalize-space(.)='{letter}']]"
        )))
        .await
        .map_err(Into::into)
}

async fn expect_focused_indicator(page: &Page<'_>, label: &str) -> Result<(), Report> {
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
    loop {
        let active = page.driver.active_element().await?;
        let active_label = attr(&active, "aria-label").await?;
        if active_label.as_deref() == Some(label)
            && attr(&active, "aria-roledescription").await?.as_deref() == Some("drop indicator")
        {
            return Ok(());
        }
        if std::time::Instant::now() > deadline {
            let actual = page.describe_active_element().await?;
            leptos_browser_test::bail!(
                "expected the drop indicator {label:?} to have focus, it is on {actual} (label {active_label:?})"
            );
        }
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    }
}

/// Reordering with the keyboard: the drop target starts after the dragged row, arrow keys move
/// between the valid positions (rows can't be dropped on), Enter drops.
async fn reorder_a_list(page: &Page<'_>) -> Result<(), Report> {
    page.click_element_with_id("test-dnd-before-list").await?;
    page.press_tab().await?;
    page.wait_for_focus_on(&row(page, "A").await?, "row A")
        .await?;
    page.send_keys_to_active(Key::Down).await?;
    let b = row(page, "B").await?;
    page.wait_for_focus_on(&b, "row B").await?;

    page.send_keys_to_active(Key::Enter).await?;
    expect_focused_indicator(page, "Insert between B and C").await?;
    assert_that!(attr(&b, "data-dragging").await?).is_some();
    // Rows can't be dropped on: they are hidden from assistive technology during the drag.
    assert_that!(attr(&row(page, "A").await?, "aria-hidden").await?)
        .is_equal_to(Some("true".to_owned()));

    page.send_keys_to_active(Key::Up).await?;
    expect_focused_indicator(page, "Insert between A and B").await?;
    page.send_keys_to_active(Key::Up).await?;
    expect_focused_indicator(page, "Insert before A").await?;

    page.send_keys_to_active(Key::Enter).await?;
    page.wait_for_text("test-dnd-order", "BACD").await?;
    let b = row(page, "B").await?;
    page.wait_for_focus_on(&b, "row B").await?;
    assert_that!(attr(&b, "aria-hidden").await?).is_none();
    Ok(())
}
