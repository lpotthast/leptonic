// Upstream: react-aria-components/test/TagGroup.test.js @ 99e6102368
use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{
    BrowserTest, async_trait,
    thirtyfour::{By, Key, WebDriver, WebElement},
};
use rootcause::Report;

use crate::pages::{BaseActions, Page};

/// Behavior of the tag group hooks: grid structure, arrow navigation (wrapping, horizontal),
/// removing tags with Delete/Backspace and remove buttons, and focus after removal.
pub struct TagGroupTests {}

#[async_trait]
impl BrowserTest<str> for TagGroupTests {
    fn name(&self) -> Cow<'_, str> {
        "tag_group_tests".into()
    }

    async fn run(&self, driver: &WebDriver, base_url: &str) -> Result<(), Report> {
        let page = Page { driver, base_url };
        page.goto_path("/hooks/tag-group").await?;

        aria_structure(&page).await?;
        keyboard_navigation(&page).await?;
        removing_with_the_keyboard_moves_focus_on(&page).await?;
        remove_button(&page).await?;
        removing_every_tag_focuses_the_group(&page).await?;

        Ok(())
    }
}

async fn tag(page: &Page<'_>, text: &str) -> Result<WebElement, Report> {
    page.by_role_and_text("row", text).await
}

async fn expect_focus(page: &Page<'_>, text: &str) -> Result<(), Report> {
    page.wait_for_focus("row", Some(text)).await
}

async fn aria_structure(page: &Page<'_>) -> Result<(), Report> {
    let grid = page.css("[role=grid]").await?;
    let labelled_by = grid.attr("aria-labelledby").await?.unwrap_or_default();
    assert_that!(
        page.css(&format!("[id='{labelled_by}']"))
            .await?
            .text()
            .await?
    )
    .is_equal_to("Categories".to_owned());
    assert_that!(grid.find_all(By::Css("[role=row]")).await?.len()).is_equal_to(4);
    let remove = page.driver.find(By::Css("[role=row] button")).await?;
    assert_that!(remove.attr("aria-label").await?).is_equal_to(Some("Remove".to_owned()));
    Ok(())
}

async fn keyboard_navigation(page: &Page<'_>) -> Result<(), Report> {
    page.click_element_with_id("test-tg-before").await?;
    page.press_tab().await?;
    expect_focus(page, "News×").await?;
    for (key, expected) in [
        (Key::Right, "Travel×"),
        (Key::Right, "Gaming×"),
        (Key::Right, "Shopping×"),
        // Tag groups wrap around.
        (Key::Right, "News×"),
        (Key::Left, "Shopping×"),
    ] {
        page.send_keys_to_active(key.clone()).await?;
        expect_focus(page, expected)
            .await
            .map_err(|e| e.context(format!("after pressing {key:?}")).into_dynamic())?;
    }
    Ok(())
}

/// Removing the focused tag moves focus to the next one (or the previous one at the end).
async fn removing_with_the_keyboard_moves_focus_on(page: &Page<'_>) -> Result<(), Report> {
    page.send_keys_to_active(Key::Left).await?;
    expect_focus(page, "Gaming×").await?;
    page.send_keys_to_active(Key::Delete).await?;
    page.wait_for_text("test-tg-tags", "News,Travel,Shopping")
        .await?;
    expect_focus(page, "Shopping×").await?;
    page.send_keys_to_active(Key::Backspace).await?;
    page.wait_for_text("test-tg-tags", "News,Travel").await?;
    expect_focus(page, "Travel×").await
}

async fn remove_button(page: &Page<'_>) -> Result<(), Report> {
    let button = tag(page, "News×").await?.find(By::Css("button")).await?;
    button.click().await?;
    page.wait_for_text("test-tg-tags", "Travel").await
}

/// When the last tag is removed, the (now empty) group keeps focus and becomes a plain group.
async fn removing_every_tag_focuses_the_group(page: &Page<'_>) -> Result<(), Report> {
    tag(page, "Travel×").await?.click().await?;
    expect_focus(page, "Travel×").await?;
    page.send_keys_to_active(Key::Delete).await?;
    page.wait_for_text("test-tg-tags", "").await?;
    page.wait_for_selector("[role=group][aria-labelledby]")
        .await?;
    page.wait_for_focus("group", None).await
}
