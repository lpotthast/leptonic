// Upstream: react-aria-components/test/TagGroup.test.js @ 99e6102368
use std::borrow::Cow;

use assertr::prelude::*;
use browser_test::{BrowserTest, async_trait, thirtyfour::prelude::*};
use rootcause::{Report, prelude::ResultExt};

use crate::pages::{ElementActions, Page, PageActions, role};

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

        cases!(
            aria_structure(&page),
            keyboard_navigation(&page),
            removing_with_the_keyboard_moves_focus_on(&page),
            remove_button(&page),
            removing_every_tag_focuses_the_group(&page),
        );

        Ok(())
    }
}

async fn tag(page: &Page<'_>, text: &str) -> Result<WebElement, Report> {
    page.element(role("row").text(text)).await
}

async fn expect_focus(page: &Page<'_>, text: &str) -> Result<(), Report> {
    page.wait_for_focus(&tag(page, text).await?).await?;
    Ok(())
}

async fn aria_structure(page: &Page<'_>) -> Result<(), Report> {
    let grid = page.element("[role=grid]").await?;
    assert_that!(grid.referenced_text("aria-labelledby").await?).is_equal_to("Categories");
    assert_that!(grid.elements("[role=row]").await?).has_length(4);
    let remove = page.element("[role=row] button").await?;
    assert_that!(remove.attr("aria-label").await?)
        .get_some()
        .is_equal_to("Remove");
    Ok(())
}

async fn keyboard_navigation(page: &Page<'_>) -> Result<(), Report> {
    page.element("#test-tg-before").await?.click().await?;
    page.send_keys(Key::Tab).await?;
    expect_focus(page, "News×").await?;
    for (key, expected) in [
        (Key::Right, "Travel×"),
        (Key::Right, "Gaming×"),
        (Key::Right, "Shopping×"),
        // Tag groups wrap around.
        (Key::Right, "News×"),
        (Key::Left, "Shopping×"),
    ] {
        page.send_keys(key.clone()).await?;
        expect_focus(page, expected)
            .await
            .context_with(|| format!("after pressing {key:?}"))?;
    }
    Ok(())
}

/// Removing the focused tag moves focus to the next one (or the previous one at the end).
async fn removing_with_the_keyboard_moves_focus_on(page: &Page<'_>) -> Result<(), Report> {
    page.send_keys(Key::Left).await?;
    expect_focus(page, "Gaming×").await?;
    let tags = page.element("#test-tg-tags").await?;
    page.send_keys(Key::Delete).await?;
    tags.wait_for_inner_text("News,Travel,Shopping").await?;
    expect_focus(page, "Shopping×").await?;
    page.send_keys(Key::Backspace).await?;
    tags.wait_for_inner_text("News,Travel").await?;
    expect_focus(page, "Travel×").await?;
    Ok(())
}

async fn remove_button(page: &Page<'_>) -> Result<(), Report> {
    let button = tag(page, "News×").await?.element("button").await?;
    button.click().await?;
    page.element("#test-tg-tags")
        .await?
        .wait_for_inner_text("Travel")
        .await?;
    Ok(())
}

/// When the last tag is removed, the (now empty) group keeps focus and becomes a plain group.
async fn removing_every_tag_focuses_the_group(page: &Page<'_>) -> Result<(), Report> {
    tag(page, "Travel×").await?.click().await?;
    expect_focus(page, "Travel×").await?;
    page.send_keys(Key::Delete).await?;
    page.element("#test-tg-tags")
        .await?
        .wait_for_inner_text("")
        .await?;
    let group = page.element("[role=group][aria-labelledby]").await?;
    page.wait_for_focus(&group).await?;
    Ok(())
}
